use object::Object;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::rc::Rc;
use ::token::Token;
use std::ops::Deref;

pub type StackFrame = LinkedList<Rc<Object>>;
pub type NamePool = HashMap<String, Rc<Object>>;

/// Mimir Parse Engine
#[derive(Default, Debug)]
pub struct Mimir {
    virtual_stack_frames: Vec<StackFrame>,
    names: Vec<NamePool>,
}
impl Mimir {
    pub fn new() -> Mimir { 
        let mut mimir = Mimir::default();
        let mut stdlib = HashMap::new();
        stdlib.insert("print".to_string(), Rc::new(Object::NativeFunction(::libstd::print)));
        mimir.names.push(stdlib);
        mimir
    }
    pub fn parse(&mut self, tokens: &[Token]) {
        {
            if self.virtual_stack_frames.last().is_none() {
                self.virtual_stack_frames.push(StackFrame::new());
            }
            if self.names.last().is_none() {
                self.names.push(HashMap::new());
            }
            let active_frame = self.virtual_stack_frames.last_mut().unwrap();
            for token in tokens {
                match token {
                    &Token::Symbol(ref sym) => {
                        let mut symdup = None;
                        for pool in self.names.iter().rev() {
                            symdup = pool.get(sym);
                            if symdup.is_some() {
                                break;
                            }
                        }
                        if let Some(symdup) = symdup {
                            active_frame.push_back(symdup.clone());
                        } else {
                            active_frame.push_back(Rc::new(Object::Symbol(sym.clone())));
                        }
                    },
                    &Token::Text(ref text) => {
                        active_frame.push_back(Rc::new(Object::Text(text.clone())));
                    },
                    &Token::Float(float) => {
                        active_frame.push_back(Rc::new(Object::Float(float)));
                    },
                    &Token::Number(ref number) => {
                        active_frame.push_back(Rc::new(Object::Number(number.clone())));
                    },
                    &Token::Bind => {
                        if let Some(val) = active_frame.pop_back() {
                            if let Some(bind_name) = active_frame.pop_back() {
                                if let &Object::Symbol(ref sym) = bind_name.deref() {
                                    self.names.last_mut().expect("Unexepected stack error").insert(sym.clone(), val);
                                } else {
                                    panic!("Cannot bind to non symbol");
                                }
                            } else {
                                panic!("Unable to bind symbol");
                            }
                        } else {
                            panic!("Unable to bind value");
                        }
                    }
                    _ => {}
                }
            }
        }
        let mut this_frame = self.virtual_stack_frames.pop();
        let mut re_add = false;
        {
            let mut last_frame = self.virtual_stack_frames.last_mut();
            if let &mut Some(ref mut this_frame) = &mut this_frame {
                if let &mut Some(ref mut last_frame) = &mut last_frame {
                    last_frame.append(this_frame)
                }
            }
            re_add = last_frame.is_none();
        }
        if re_add {
            if let Some(this_frame) = this_frame {
                self.virtual_stack_frames.push(this_frame);
            }
        }
        
    }
}

