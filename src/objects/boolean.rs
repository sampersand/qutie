use objects::object::{Object, RcObject};
use objects::result::{ObjResult, BoolResult};
use parsing::frame::Frame;

pub struct Boolean {
   bool_val: bool
}

use std;
derive_impl!(Display; Boolean, bool_val);
derive_impl!(Debug; Boolean, "B");
derive_impl!(ToRc; Boolean);
derive_impl!(Castable; Boolean);
derive_impl!(Opers; Boolean);
derive_impl!(Types; Boolean);
derive_impl!(ToText; Boolean, bool_val);
derive_impl!(ToNumber; Boolean);
derive_impl!(ToBoolean; Boolean);
impl Boolean {
   pub fn from(inp: bool) -> Boolean {
      Boolean{bool_val: inp}
   }
   pub fn to_bool(&self) -> bool {
      self.bool_val
   }
   pub fn _not(&self) -> Boolean {
      Boolean{ bool_val: !self.bool_val }
   }
}

use traits::misc::TryFrom;
impl <'a> TryFrom<&'a str> for Boolean {
   fn try_from(inp: &'a str) -> Option<Boolean> {
      match inp {
         "true" => Some(Boolean{bool_val: true}),
         "false" => Some(Boolean{bool_val: false}),
         _ => None
      }
   }
}
use std::rc::Rc;


impl Object for Boolean {
   fn hash(&self) -> u8 {
      self.bool_val as u8 // maybe update this?
   }
   fn _eql(&self, other: RcObject) -> bool {
      is_a!(other, boolean) && self.bool_val == cast_as!(other, Boolean).bool_val
   }
}

derive_impl!(+; Boolean);
derive_impl!(-; Boolean);
derive_impl!(*; Boolean);
derive_impl!(/; Boolean);
derive_impl!(%; Boolean);
derive_impl!(**; Boolean);
derive_impl!(==/!=; Boolean);
derive_impl!( <; Boolean);
derive_impl!(<=; Boolean);
derive_impl!( >; Boolean);
derive_impl!(>=; Boolean);
derive_impl!(!; Boolean);
























