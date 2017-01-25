use num::BigInt;
use ::mimir::StackFrame;
use std::fmt;

type NativeFunction = fn(Vec<StackFrame>) -> Vec<StackFrame>;

#[derive(Debug, Clone)]
pub enum Object {
    Text(String),
    Symbol(String),
    Float(f64),
    Number(BigInt),
    Type(Type),
    Void(Type),
    List(Vec<Object>),
    LocalFunction(StackFrame),
    NativeFunction(NativeFunction),
}
impl Object {
    fn to_type(&self) -> Type {
        match self {
            &Object::Text(_) => Type::Text,
            &Object::Float(_) => Type::Float,
            &Object::Number(_) => Type::Number,
            &Object::Type(_) => Type::Type,
            &Object::Void(t) => t,
            &Object::List(_) => Type::List,
            &Object::LocalFunction(_) => Type::Function,
            &Object::NativeFunction(_) => Type::Function,
            &Object::Symbol(_) => Type::Symbol,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Text,
    Float,
    Number,
    Type,
    List,
    Function,
    Symbol,
}
