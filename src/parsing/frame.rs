use parsing::stream::Stream;
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use objects::object::{RcObject, RcObjWrapper};
pub type StackType = Vec<RcObject>;
pub type LocalsType = HashMap<RcObjWrapper, RcObject>;

#[derive(Debug)]
pub struct Frame<'a> {
   pub stream: Rc<RefCell<Stream>>,
   parent: Option<&'a Frame<'a>>,
   stack: StackType,
   locals: LocalsType,
}

impl <'a> Frame<'a> {
   pub fn new(stream: Stream) -> Frame<'a> {
      Frame{ stream: Rc::new(RefCell::new(stream)),
             stack: StackType::new(),
             locals: LocalsType::new(),
             parent: None
      }
   }
   pub fn fork(&'a self) -> Frame<'a> {
      Frame{ stream: self.stream.clone(),
             stack: StackType::new(),
             locals: LocalsType::new(),
             parent: Some(self)
      }
   }

   pub fn exec(mut self) -> Frame<'a> {
      use parsing::parser;
      parser::exec_frame(&mut self);
      self
   }

   pub fn push(&mut self, new_obj: RcObject) {
      self.stack.push(new_obj);
   }
   pub fn pop(&mut self) -> Option<RcObject> {
      self.stack.pop()
   }
   pub fn assign(&mut self, key: RcObject, val: RcObject) {
      self.locals.insert(RcObjWrapper(key), val);
   }
}
















