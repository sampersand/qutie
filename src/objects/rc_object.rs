use std::hash::{Hash, Hasher};
use std::rc::Rc;
use objects::object::Object;


pub type RcObject = Rc<Object>;

#[derive(Clone, Debug)]
pub struct RcObjWrapper(pub RcObject);
impl PartialEq for RcObjWrapper {
   fn eq(&self, other: &RcObjWrapper) -> bool {
      (self.0).equals((other.0).clone())
   }
}

impl Eq for RcObjWrapper{}

impl Hash for RcObjWrapper {
   fn hash<T: Hasher>(&self, hasher: &mut T){
      hasher.write(&[(self.0).hash()]);
   }
}

