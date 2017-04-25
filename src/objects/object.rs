use std::fmt::{Debug, Display};
use std::rc::Rc;

use std::hash::{Hash, Hasher};

pub type RcObj = Rc<Object>;
#[derive(Clone, Debug)]
pub struct RcObjWrapper(pub RcObj);
impl PartialEq for RcObjWrapper {
   fn eq(&self, other: &RcObjWrapper) -> bool {
      (self.0)._eql((other.0).clone())
   }
}
impl Eq for RcObjWrapper{}
impl Hash for RcObjWrapper{
   fn hash<T: Hasher>(&self, hasher: &mut T){
      hasher.write(&[(self.0)._hash()]);
   }
}


pub trait Object: Debug + Display {
   fn _eql(&self, other: RcObj) -> bool { false }
   fn _hash(&self) -> u8;
}