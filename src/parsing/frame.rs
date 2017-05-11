use obj::objects::object::Object;
use std::rc::Rc;
use parsing::identifier::Identifier;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Frame<'a> {
   parent: Option<&'a Frame<'a>>,
   stack: Vec<Rc<Object>>,
   knowns: HashMap<Identifier, Rc<Object>>
}

impl <'a> Frame<'a> {
   pub fn new() -> Frame<'a> {
      Frame {
         parent: None,
         stack: Vec::new(),
         knowns: HashMap::new()
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
   pub fn get(&self, key: &Identifier) -> Option<Rc<Object>> {
      match self.knowns.get(key) {
         None => None,
         Some(val) => Some(val.clone()) /* so its not a reference */
      }
   }
   pub fn set(&mut self, key: Identifier, val: Rc<Object>) {
      self.knowns.insert(key, val);
   }
}
















