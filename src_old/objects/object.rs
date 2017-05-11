use std::fmt::Debug;

use objects::traits::operator::Opers;
use objects::traits::types::Types;
use objects::traits::misc::Castable;

pub trait Object: Debug + Opers + Types + Castable {
   fn _eql(&self, other: RcObject) -> bool;
   fn hash(&self) -> u8;
   fn equals(&self, other: RcObject) -> bool {
      self.hash() == other.hash() && self._eql(other)
   }
}


use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub type RcObject = Rc<Object>;

#[derive(Clone)]
pub struct RcObjWrapper(pub RcObject);
use std;
impl std::fmt::Debug for RcObjWrapper {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "{:?}", self.0)
   }
}

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




