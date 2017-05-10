use parsing::frame::Frame;

use objects::object::RcObject;
use objects::result::{ObjResult, BoolResult};

pub enum FuncType {
   Obj(fn(RcObject, &mut Frame) -> ObjResult),
   Bool(fn(RcObject, &mut Frame) -> BoolResult)
}

pub struct UnaryOperator {
   sigil: &'static str,
   priority: u8,
   is_on_lhs: bool,
   func: FuncType
}

mod opers {
   use objects::operators::unary_operator::UnaryOperator;
   use objects::object::RcObject;
   use objects::result::{ObjResult, BoolResult};
   use parsing::frame::Frame;
   use objects::operators::unary_operator::FuncType;

   macro_rules! new_oper {
      ($oper_name:ident, $sigil:expr, $priority:expr, $is_on_lhs:ident, $func_name:ident) => {
         fn $func_name(arg: RcObject, frame: &mut Frame) -> ObjResult {
            arg.$func_name(frame)
         }
         pub const $oper_name: UnaryOperator = UnaryOperator {
            sigil: $sigil,
            func: FuncType::Obj($func_name),
            priority: $priority,
            is_on_lhs: $is_on_lhs
         };
      };
      (BOOL; $oper_name:ident, $sigil:expr, $priority:expr, $is_on_lhs:ident, $func_name:ident) => {
         fn $func_name(arg: RcObject, frame: &mut Frame) -> BoolResult {
            arg.$func_name(frame)
         }
         pub const $oper_name: UnaryOperator = UnaryOperator {
            sigil: $sigil,
            func: FuncType::Bool($func_name),
            priority: $priority,
            is_on_lhs: $is_on_lhs
         };
      }

   }
   // new_oper!(NOT, "+", 12, false, oper_add);
   // new_oper!(SUB, "-", 12, false, oper_sub);
   // new_oper!(MUL, "*", 11, false, oper_mul);
   // new_oper!(DIV, "/", 11, false, oper_div);
   // new_oper!(MOD, "%", 11, false, oper_mod);
   // new_oper!(POW, "**", 10, true, oper_pow);

   new_oper!(BOOL; NOT, "!", 5, true, oper_not);
}

use std;
derive_impl!(Display; UnaryOperator, sigil);
derive_impl!(Debug; UnaryOperator, "Ou");

use objects::operators::Operator;
impl Operator for UnaryOperator {
   fn should_exec(&self, other: &Operator) -> bool {
      other.priority() >= self.priority()
   }
   fn exec(&self, frame: &mut Frame) {
      let arg = frame.pop().expect("bad lhs for operator");
      let res = 
         match self.func {
            FuncType::Obj(func) => (func)(arg, frame).expect("problem with exec of function"),
            FuncType::Bool(func) => (func)(arg, frame).expect("problem with exec of function"),
         };
      frame.push(res);
   }
   fn priority(&self) -> u8 { self.priority }
}

use traits::misc::TryFrom;
impl <'a> TryFrom<&'a str> for UnaryOperator {
   fn try_from(inp: &'a str) -> Option<UnaryOperator> {
      match inp {
         "!" => Some(opers::NOT),
         _ => None
      }
   }
}

















