use objects::object::{Object, RcObject};
use objects::result::ObjResult;
use parsing::frame::Frame;

pub struct Boolean {
   val: bool
}

use std;
derive_impl!(Display; Boolean, val);
derive_impl!(Debug; Boolean, "B");
derive_impl!(ToRc; Boolean);
derive_impl!(Castable; Boolean);
derive_impl!(Opers; Boolean);
derive_impl!(Types; Boolean);
derive_impl!(ToText; Boolean, val);
derive_impl!(ToNumber; Boolean);
derive_impl!(ToBool; Boolean);
impl Boolean {
   pub fn from(inp: bool) -> Boolean {
      Boolean{val: inp}
   }
}

use traits::misc::TryFrom;
impl TryFrom for Boolean {
   fn try_from(inp: &str) -> Option<Boolean> {
      match inp {
         "true" => Some(Boolean{val: true}),
         "false" => Some(Boolean{val: false}),
         _ => None
      }
   }
}
use std::rc::Rc;


impl Object for Boolean {
   fn hash(&self) -> u8 {
      self.val as u8 // maybe update this?
   }
   fn _eql(&self, other: RcObject) -> bool {
      is_a!(other, boolean) && self.val == cast_as!(other, Boolean).val
   }
}

derive_impl!(+; Boolean);
derive_impl!(-; Boolean);
derive_impl!(*; Boolean);
derive_impl!(/; Boolean);
derive_impl!(%; Boolean);
derive_impl!(**; Boolean);
derive_impl!(==; Boolean);
derive_impl!(!=; Boolean);
derive_impl!( <; Boolean);
derive_impl!(<=; Boolean);
derive_impl!( >; Boolean);
derive_impl!(>=; Boolean);

























