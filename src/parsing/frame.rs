use obj::objects::object::Object;
use std::rc::Rc;
use parsing::identifier::Identifier;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Frame<'a> {
   pub file: String,
   pub lineno: usize,
   parent: Option<&'a Frame<'a>>,
   stack: Vec<Rc<Object>>,
   locals: HashMap<Identifier, Rc<Object>>,
   globals: Rc<HashMap<Identifier, Rc<Object>>>
}

impl <'a> Frame<'a> {
   pub fn new(parent: Option<&'a Frame>) -> Frame<'a> {
      Frame {
         file: "<todo: file>".to_string(),
         lineno: 0,
         parent: parent,
         stack: Vec::new(),
         locals: HashMap::new(),
         globals: 
            if let Some(parent) = parent {
               parent.globals.clone()
            } else {
               Rc::new(HashMap::new())
            }
      }
   }
   pub fn spawn_child<'b>(&self) -> Frame<'b> {
      Frame::new(None) // TODO: parent = this one 
   }

   pub fn is_empty(&self) -> bool {
      self.stack.is_empty()
   }

   pub fn is_root(&self) -> bool {
      self.parent.is_none()
   }

   pub fn push(&mut self, obj: Rc<Object>) {
      self.stack.push(obj);
   }

   pub fn pop(&mut self) -> Option<Rc<Object>> {
      self.stack.pop()
   }

   pub fn stack_len(&mut self) -> usize {
      self.stack.len()
   }

   pub fn get(&self, key: &Identifier) -> Option<Rc<Object>> {
      match self.locals.get(key) {
         None => 
            match self.globals.get(key) {
               Some(val) => Some(val.clone()),
               None => None,
            },
         Some(val) => Some(val.clone()) /* so its not a reference */
      }
   }

   pub fn set(&mut self, key: Identifier, val: Rc<Object>) {
      if self.is_root() {
         unsafe {
            use std::mem::transmute;
            #[allow(mutable_transmutes)]
            transmute::<&HashMap<Identifier, Rc<Object>>, &mut HashMap<Identifier, Rc<Object>>>(&*self.globals)
         }.insert(key.clone(), val.clone());
      }
      self.locals.insert(key, val);
   }
}
















