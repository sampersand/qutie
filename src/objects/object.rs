use std::fmt::Debug;
use objects::rc_object::RcObject;
use traits::operator::{OperAdd, OperMul};
use traits::types::ToNumber;

pub trait Object: Debug + OperAdd + ToNumber + OperMul {
   fn _eql(&self, other: RcObject) -> bool;
   fn hash(&self) -> u8;
   fn equals(&self, other: RcObject) -> bool {
      self.hash() == other.hash() && self._eql(other)
   }
}




