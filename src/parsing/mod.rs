#[derive(Debug)]
pub enum Token {
   Identifier(String),
   Path(String),
   Assignment(String),
   LineTerminator(String),
   Number(String),
   Operator(String),
   Unknown(String),
   Block((char, char), Vec<Token>),
   Text(char, String)
}

pub mod stream;
pub mod parser;
pub mod frame;
