#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LParen {
   Round, Square, Curly
}
#[derive(Debug, PartialEq, Eq, Clone)]
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
impl_defaults!(to_string; char; LParen);
impl_defaults!(to_string; char; RParen);
impl_defaults!(Display; to_string; LParen);
impl_defaults!(Display; to_string; RParen);

use parsing::token::Token;
pub struct Block {
   parens: (LParen, RParen),
   pub body: Vec<Token>,
   // frame: /* TODO: FRAME so we can have a frame of reference when it's executed */
}
impl Block {
   pub fn new(parens: (LParen, RParen), body: Vec<Token>) -> Block {
      Block{ parens: parens, body: body }
   }
   pub fn to_string(&self) -> String {
      let mut ret = String::new();
      ret.push_str(self.parens.0.to_string().as_str());
      // ret.push_str(self.body.to_string().as_str());
      ret.push_str("...");
      ret.push_str(self.parens.1.to_string().as_str());
      return ret
   }
}

use obj::objects::object::{Object, ObjType};
impl_defaults!(Display; to_string; Block);
impl_defaults!(Debug; Block, 'B');
impl_defaults!(ToRc; Block);
impl_defaults!(Object; Block);
impl_defaults!(ToBoolean; Block);

impl_defaults!(QtAdd; Block);
impl_defaults!(QtSub; Block);
impl_defaults!(QtMul; Block);
impl_defaults!(QtDiv; Block);
impl_defaults!(QtMod; Block);
impl_defaults!(QtPow; Block);
impl_defaults!(QtEql; Block);
impl_defaults!(QtNeq; Block);
impl_defaults!(QtLth; Block);
impl_defaults!(QtGth; Block);
impl_defaults!(QtLeq; Block);
impl_defaults!(QtGeq; Block);








































