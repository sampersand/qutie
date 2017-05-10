#[derive(Debug)]
pub enum Token {
   Identifier(String),
   LineTerminator(char),
   Number(String),
   Operator(String),
   Unknown(char),
}

pub mod stream;
pub mod parser;
pub mod frame;
