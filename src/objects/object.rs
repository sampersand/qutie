use std::fmt::Debug;
use objects::rc_object::RcObject;
use objects::traits::operator::QtAdd;

pub trait Object: Debug + QtAdd {
   // fn is_a(&self, other: )
   fn _eql(&self, other: RcObject) -> bool;
   fn hash(&self) -> u8;
   fn equals(&self, other: RcObject) -> bool {
      self.hash() == other.hash() && self._eql(other)
   }
}




