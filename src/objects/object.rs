use std::fmt::Debug;
use traits::operator::Opers;
use traits::types::Types;

pub trait Object: Debug + Opers + Types {
   fn _eql(&self, other: RcObject) -> bool;
   fn hash(&self) -> u8;
   fn equals(&self, other: RcObject) -> bool {
      self.hash() == other.hash() && self._eql(other)
   }
}


use std::hash::{Hash, Hasher};
use std::rc::Rc;

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




