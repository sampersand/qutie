pub enum Token {
   Identifier(String),
   Path(String),
   Assignment(String),
   LineTerminator(String),
   Number(String),
   Operator(String),
   Unknown(String),
   Text(char, String),
   Block((char, char), Vec<Token>),
   RParen(char)
}

use std;
impl std::fmt::Debug for Token {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      use self::Token::*;
      match self {
         &Identifier(ref s) => write!(f, "I({})", s),
         &Path(ref s) => write!(f, "P({})", s),
         &Assignment(ref s) => write!(f, "A({})", s),
         &LineTerminator(ref s) => write!(f, "L({})", s),
         &Number(ref s) => write!(f, "N({})", s),
         &Operator(ref s) => write!(f, "O({})", s),
         &Unknown(ref s) => write!(f, "U({})", s),
         &Text(ref q, ref s) => write!(f, "T({}{}{})", q, s, q),
         &Block((ref l, ref r), ref b) => write!(f, "B{}{:?}{}", l, b, r),
         &RParen(ref c) => panic!("RParen shouldn't be printed out!")
      }
   }
}


pub mod stream;
pub mod parser;
pub mod frame;




