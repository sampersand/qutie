use parsing::token::Token;
use parsing::expression::Expression;
use parsing::frame::Frame;
use std::rc::Rc;
use obj::objects::object::{Object, ObjType};


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

#[derive(Clone)]
pub struct Block {
   parens: (LParen, RParen),
   pub body: Vec<Expression>,
   // frame: /* TODO: FRAME so we can have a frame of reference when it's executed */
}
impl Block {
   pub fn new(parens: (LParen, RParen), body: Vec<Expression>) -> Block {
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
   pub fn is_single(&self) -> bool {
      self.body.len() == 1
   }
   pub fn pop_single_expr(&mut self) -> Option<Expression> {
      if !self.is_single() {
         None
      } else {
         Some(self.body.remove(0))
      }
   }
   pub fn exec(self, frame: &mut Frame) -> Option<Rc<Object>> {
      self.exec_no_pop(frame);
      frame.pop()
   }
   pub fn exec_no_pop(self, frame: &mut Frame) {
      for expr in self.body {
         expr.exec(frame);
      }
   }
}


impl_defaults!(Display; to_string; Block);
impl_defaults!(Debug; Block, 'B');
impl_defaults!(ToRc; Block);
impl_defaults!(Object; Block);

impl_traits!(data=GetItem, Block);
impl_traits!(data=SetItem, Block);
impl_traits!(data=DelItem, Block);

impl_traits!(conv=ToBoolean, Block);
impl_traits!(conv=ToText, Block);

impl_traits!(oper=QtAdd, Block);
impl_traits!(oper=QtSub, Block);
impl_traits!(oper=QtMul, Block);
impl_traits!(oper=QtDiv, Block);
impl_traits!(oper=QtMod, Block);
impl_traits!(oper=QtPow, Block);
impl_traits!(oper=QtEql, Block);
impl_traits!(oper=QtNeq, Block);
impl_traits!(oper=QtLth, Block);
impl_traits!(oper=QtGth, Block);
impl_traits!(oper=QtLeq, Block);
impl_traits!(oper=QtGeq, Block);








































