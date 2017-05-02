use std::fmt::Debug;
use objects::rc_object::RcObject;
use traits::operator::Opers;
use traits::types::Types;

pub trait Object: Debug + Opers + Types {
   fn _eql(&self, other: RcObject) -> bool;
   fn hash(&self) -> u8;
   fn equals(&self, other: RcObject) -> bool {
      self.hash() == other.hash() && self._eql(other)
   }
}




