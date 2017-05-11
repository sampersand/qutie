use obj::objects::object::Object;
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
         stack: Vec::<Rc<Object>>::new(),
      }
   }

   pub fn push(&mut self, obj: Rc<Object>) {
      self.stack.push(obj);
   }

   pub fn is_empty(&self) -> bool {
      self.stack.is_empty()
   }

   pub fn pop(&mut self) -> Option<Rc<Object>> {
      self.stack.pop()
   }

}