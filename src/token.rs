use num::BigInt;

#[derive(Debug, Clone)]
pub enum Token {
    Symbol(String),
    Text(String),
    Float(f64),
    Number(BigInt),
    WhiteSpace,
    BeginList,
    EndList,
    BeginFrame,
    EndFrame,
    Void,
    Bind,
}

#[derive(Eq,PartialEq)]
enum Mode {
    Normal,
    StringBuilder,
    FloatBuilder,
    NumberBuilder,
}
impl Mode {
    fn is_normal(&self) -> bool { self == &Mode::Normal }   
    fn is_string_builder(&self) -> bool { self == &Mode::StringBuilder }
    fn is_float_builder(&self) -> bool { self == &Mode::FloatBuilder }
    fn is_number_builder(&self) -> bool { self == &Mode::NumberBuilder }
}

impl Token {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut input = Box::new(input.chars()) as Box<Iterator<Item=char>>;

        let mut mode = Mode::Normal;
        
        loop {
            let step = input.next();

            if step.is_none() {
                break;
            }
            let character = step.unwrap();

            if mode.is_normal() {
                match character {
                    '"' => mode = Mode::StringBuilder,
                    '%' => mode = Mode::FloatBuilder,
                    '&' => mode = Mode::NumberBuilder,
                    '{' => tokens.push(Token::BeginFrame),
                    '}' => tokens.push(Token::EndFrame),
                    '[' => tokens.push(Token::BeginList),
                    ']' => tokens.push(Token::EndList),
                    '@' => tokens.push(Token::Bind),
                    '_' => tokens.push(Token::Void),
                    c if c.is_whitespace() => tokens.push(Token::WhiteSpace),
                    c => {
                        let mut res = None; // Should be overwritten
                        tokens.push(Token::Symbol(Some(c).into_iter().chain((&mut input).take_while(|n| {
                            let p = is_reserved_symbol(n);
                            if p {
                                res = Some(*n)
                            }
                            !p
                        })).collect::<String>()));
                        input = Box::new(res.into_iter().chain(input)) as Box<Iterator<Item=char>>;
                    }
                }
            }

            if mode.is_string_builder() {
                let mut escaped = false;
                tokens.push(Token::Text((&mut input).take_while(|&n| {
                    if !escaped {
                        return !(n == '"');
                    } else {
                        escaped = n == '\\';
                        return true;
                    }
                }).collect::<String>()));
            }

            if mode.is_float_builder() {
                let mut res = None;
                let mut dot = false;
                tokens.push(Token::Float((&mut input).take_while(|&n| {
                    let p = if !dot {
                        if n == '.' {
                            dot = true;
                            true
                        } else {
                            n.is_numeric()
                        }
                    } else {
                        n.is_numeric()
                    };
                    if !p {
                        res = Some(n);
                    }
                    p
                }).collect::<String>().parse::<f64>().expect("unexpected float parse error")));
                input = Box::new(res.into_iter().chain(input)) as Box<Iterator<Item=char>>;
            }

            if mode.is_number_builder() {
                let mut res = None;
                tokens.push(Token::Number((&mut input).enumerate().take_while(|&(i, n)| {
                    let p = if i == 0 {
                        if n == '-' {
                            true
                        } else {
                            n.is_numeric()
                        }
                    } else {
                        n.is_numeric()
                    };
                    if !p {
                        res = Some(n);
                    }
                    p
                }).map(|n| n.1).collect::<String>().parse::<BigInt>().expect("unexpected integer parse error")));
                input = Box::new(res.into_iter().chain(input)) as Box<Iterator<Item=char>>;
            }


            mode = Mode::Normal;
        }

        tokens
    }
}

fn is_reserved_symbol(c: &char) -> bool {
    let c = *c;
    c == '"' || c.is_whitespace() || c == '[' || c == ']' || c == '{' || c == '}' || c == '@'
}
