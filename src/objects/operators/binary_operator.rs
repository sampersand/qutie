use parsing::frame::Frame;

use objects::result::ObjResult;
use objects::rc_object::RcObject;

pub struct BinaryOperator {
   sigil: &'static str,
   priority: u32,
   is_left_assoc: bool,
   func: fn(RcObject, RcObject, &mut Frame) -> ObjResult
}

use std;
derive_impl!(Display; BinaryOperator, sigil);
derive_impl!(Debug; BinaryOperator, "Ob");

impl BinaryOperator {
   pub fn should_exec(&self, other: &BinaryOperator) -> bool {
      (other.is_left_assoc && other.priority <= self.priority) ||
      (!other.is_left_assoc && other.priority < self.priority)
   }

   pub fn exec(&self, frame: &mut Frame) {
      let rhs = frame.pop().expect("bad args for operator");
      let lhs = frame.pop().expect("bad args for operator");
      let res = ((self.func)(lhs, rhs, frame)).expect("problem with exec of function");
      frame.push(res);
   }
}

use objects::traits::misc::TryFrom;
impl TryFrom for BinaryOperator {
   fn try_from(inp: &str) -> Option<BinaryOperator> {
      match inp {
         "+" => Some(ADD),
         "*" => Some(MUL),
         _ => None
      }
   }
}


macro_rules! new_oper {
   ($oper_name:ident, $sigil:expr, $priority:expr, $is_left:ident, $func_name:ident) => {
      fn $func_name(lhs: RcObject, rhs: RcObject, frame: &mut Frame) -> ObjResult {
         lhs.$func_name(rhs, frame)
      }
      const $oper_name: BinaryOperator = BinaryOperator{
         sigil: $sigil,
         func: $func_name,
         priority: $priority,
         is_left_assoc: $is_left
      };
   }
}

new_oper!(ADD, "+", 10, false, qt_add);
new_oper!(MUL, "*", 11, false, qt_mul);



// impl Object for BinaryOperator {
//    fn hash(&self) -> u8 {
//       todo!("hash for operator");
//    }
//    fn _eql(&self, other: RcObject) -> bool {
//       todo!("_eql for operator")
//    }
// }










