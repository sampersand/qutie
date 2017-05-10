use objects::object::Object;
use std::rc::Rc;

#[derive(Debug)]
pub struct Frame<'a> {
   parent: Option<&'a Frame<'a>>,
   stack: Vec<Rc<Object>>
}

impl <'a> Frame<'a> {
   pub fn new() -> Frame<'a> {
      Frame {
         parent: None,
         stack: vec![],
      }
   }
   pub fn push(&mut self, obj: Rc<Object>) {
      self.stack.push(obj);
   }
}