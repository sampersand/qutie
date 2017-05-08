use parsing::stream::Stream;
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;
use objects::result::{ObjResult, ObjError};

use objects::object::{RcObject, RcObjWrapper};
pub type StackType = Vec<RcObject>;
pub type LocalsType = HashMap<RcObjWrapper, RcObject>;


#[derive(Debug)]
pub enum Parenthesis {
   Round,
   Square,
   Curly,
}

use traits::misc::TryFrom;
impl TryFrom<char> for Parenthesis {
   fn try_from(inp: char) -> Option<Parenthesis> {
      match inp {
         '(' | ')' => Some(Parenthesis::Round),
         '[' | ']' => Some(Parenthesis::Square),
         '{' | '}' => Some(Parenthesis::Curly),
         _ => None
      }
   }
}
impl Parenthesis {
   fn into_char(&self, is_rhs: bool) -> char {
      use self::Parenthesis::*;
      match *self {
         Round =>  if is_rhs { ')' } else { '(' },
         Square => if is_rhs { ']' } else { '[' },
         Curly =>  if is_rhs { '}' } else { '{' },
      }
   }
}

pub struct Frame<'a> {
   pub stream: Rc<RefCell<Stream>>,
   pub parens: [Parenthesis; 2],
   parent: Option<&'a Frame<'a>>,
   stack: StackType,
   locals: Rc<RefCell<LocalsType>>,
}

impl <'a> Frame<'a> {
   pub fn new(stream: Stream) -> Frame<'a> {
      Frame{ stream: Rc::new(RefCell::new(stream)),
             parens: [Parenthesis::Curly, Parenthesis::Curly],
             stack: StackType::new(),
             locals: Rc::new(RefCell::new(LocalsType::new())),
             parent: None
      }
   }
   // pub fn fork(&'a self) -> Frame<'a> {
   //    Frame{ stream: self.stream.clone(),
   //           parens: self.parens.clone(),
   //           stack: StackType::new(),
   //           locals: Rc::new(RefCell::new(LocalsType::new())),
   //           parent: Some(self)
   //    }
   // }

   fn fork_stream(&'a self, parens: [Parenthesis; 2], stream: Stream) -> Frame<'a> {
      Frame{ stream: Rc::new(RefCell::new(stream)),
             parens: parens,
             stack: StackType::new(),
             locals: self.locals.clone(),
             parent: Some(self)
      }
   }

   pub fn exec(&mut self) {
      use parsing::parser;
      parser::exec_frame(self);
   }
   pub fn eval(&mut self) -> Option<RcObject> {
      self.exec();
      self.stack.pop()
   }

   pub fn push(&mut self, new_obj: RcObject) {
      self.stack.push(new_obj);
   }

   pub fn pop(&mut self) -> Option<RcObject> {
      self.stack.pop()
   }
   pub fn assign(&mut self, key: RcObject, val: RcObject) -> ObjResult {
      self.locals.borrow_mut().insert(RcObjWrapper(key), val.clone());
      Ok(val)
   }
   pub fn retrieve(&mut self, key: RcObject) -> ObjResult {
      if let Some(val) = self.locals.borrow().get(&RcObjWrapper(key.clone())) {
         Ok(val.clone())
      } else {
         Err(ObjError::NoSuchKey(key))
      }
   }
}

impl <'a> Frame<'a> {
   pub fn try_from(&'a self, inp: &str) -> Option<Frame<'a>> {
      if inp.len() < 2 { return None }
      let mut chars = inp.chars();
      if let Some(start_paren) = Parenthesis::try_from(chars.nth(0).unwrap()) {
         if let Some(end_paren) = Parenthesis::try_from(chars.last().unwrap()) {
            let mut inp = inp.to_string();
            assert!(inp.pop().is_some());
            inp.remove(0);
            return Some(self.fork_stream([start_paren, end_paren], Stream::from(inp.as_str())))
         }
      }
      None
   }
}

use std;
impl <'a> std::fmt::Debug for Frame<'a> {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "Frame{}{:?} | {:?}{}",
             self.parens[0].into_char(false), self.stack, self.locals,
             self.parens[1].into_char(true)
      )
   }
}












