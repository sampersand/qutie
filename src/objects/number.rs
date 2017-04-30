use objects::object::Object;
use objects::rc_object::RcObject;
use objects::result::ObjResult;
use parsing::frame::Frame;

pub struct Number {
   num: i32
}

use std;
derive_impl!(Display; Number, num);
derive_impl!(Debug; Number, "N");
derive_impl!(ToRc; Number);
derive_impl!(NEW; Number, num, i32);


use objects::traits::misc::TryFrom;
impl TryFrom for Number {
   fn try_from(inp: &str) -> Option<Number> {
      match inp.parse::<i32>() {
         Ok(num) => Some(Number::new(num)),
         Err(_) => None
      }
   }
}

impl Object for Number {
   fn hash(&self) -> u8 {
      self.num as u8
   }
   fn _eql(&self, other: RcObject) -> bool {
      todo!("_eql for number")
   }
}
macro_rules! impl_num_oper {
   ($_trait:ident, $func:ident, $oper:tt) => {
      use objects::traits::operator::$_trait;
      impl $_trait for Number {
         fn $func(&self, other: RcObject, _: &mut Frame) -> ObjResult {
            
            todo!("oper for number");
         }
      }
   }
}
impl_num_oper!(QtAdd, qt_add, +);













