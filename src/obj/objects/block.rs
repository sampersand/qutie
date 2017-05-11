#[derive(Debug, PartialEq, Eq)]
pub enum LParen {
   Round, Square, Curly
}
#[derive(Debug, PartialEq, Eq)]
pub enum RParen {
   Round, Square, Curly
} 

impl From<char> for LParen {
   fn from(inp: char) -> LParen {
      match inp {
         '(' => LParen::Round,
         '[' => LParen::Square,
         '{' => LParen::Curly,
         _ => unreachable!("bad lparen: {:?}", inp)
      }
   }
}

impl <'a> From<&'a LParen> for char {
   fn from(lparen: &'a LParen) -> char {
      match *lparen {
         LParen::Round => '(',
         LParen::Square => '[',
         LParen::Curly => '{',
      }
   }
}

impl LParen {
   pub fn get_rparen(&self) -> RParen {
      match *self {
         LParen::Round => RParen::Round,
         LParen::Square => RParen::Square,
         LParen::Curly => RParen::Curly,
      }
   }
}
impl From<char> for RParen {
   fn from(inp: char) -> RParen {
      match inp {
         ')' => RParen::Round,
         ']' => RParen::Square,
         '}' => RParen::Curly,
         _ => unreachable!("bad rparen: {:?}", inp)
      }
   }
}

impl <'a> From<&'a RParen> for char {
   fn from(rparen: &'a RParen) -> char {
      match *rparen {
         RParen::Round => ')',
         RParen::Square => ']',
         RParen::Curly => '}',
      }
   }
}
use std;
impl_defaults!(TO_STRING; char; LParen);
impl_defaults!(TO_STRING; char; RParen);
impl_defaults!(DISPLAY; LParen);
impl_defaults!(DISPLAY; RParen);






