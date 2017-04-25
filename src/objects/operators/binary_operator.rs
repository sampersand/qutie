use parsing::frame::Frame;

use objects::object::Object;
use objects::result::ObjResult;
use objects::rc_object::RcObject;

pub struct BinaryOperator {
   sigil: &'static str,
   func: fn(RcObject, RcObject, &mut Frame) -> ObjResult
}

use std;
derive_impl!(Display; BinaryOperator, sigil);
derive_impl!(Debug; BinaryOperator, "Ob");
derive_impl!(ToRc; BinaryOperator);
derive_impl!(OPER: +; BinaryOperator);


use objects::traits::misc::TryFrom;
impl TryFrom for BinaryOperator {
   fn try_from(inp: &str) -> Option<BinaryOperator> {
      match inp {
         "+" => Some(ADD),
         _ => None
      }
   }
}

macro_rules! new_oper {
   ($oper_name:ident, $sigil:expr, $func_name:ident) => {
      fn $func_name(lhs: RcObject, rhs: RcObject, frame: &mut Frame) -> ObjResult {
         lhs.$func_name(rhs, frame)
      }
      const $oper_name: BinaryOperator = BinaryOperator{ sigil: $sigil, func: $func_name };
   }
}

new_oper!(ADD, "+", qt_add);



impl Object for BinaryOperator {
   fn hash(&self) -> u8 {
      todo!("hash for operator");
   }
   fn _eql(&self, other: RcObject) -> bool {
      todo!("_eql for operator")
   }
}










