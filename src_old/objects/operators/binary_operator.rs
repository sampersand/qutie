use parsing::frame::Frame;

use objects::object::RcObject;
use objects::result::{ObjResult, BoolResult};

pub enum FuncType {
   Obj(fn(RcObject, RcObject, &mut Frame) -> ObjResult),
   Bool(fn(RcObject, RcObject, &mut Frame) -> BoolResult)
}


pub struct BinaryOperator {
   sigil: &'static str,
   priority: u8,
   is_left_assoc: bool,
   func: FuncType
}


mod opers {
   use objects::operators::binary_operator::BinaryOperator;
   use objects::object::RcObject;
   use objects::result::{ObjResult, BoolResult};
   use parsing::frame::Frame;
   use objects::operators::binary_operator::FuncType;

   macro_rules! new_oper {
      ($oper_name:ident, $sigil:expr, $priority:expr, $is_left:ident, $func_name:ident) => {
         fn $func_name(lhs: RcObject, rhs: RcObject, frame: &mut Frame) -> ObjResult {
            lhs.$func_name(rhs, frame)
         }
         pub const $oper_name: BinaryOperator = BinaryOperator {
            sigil: $sigil,
            func: FuncType::Obj($func_name),
            priority: $priority,
            is_left_assoc: $is_left
         };
      };
      (BOOL; $oper_name:ident, $sigil:expr, $priority:expr, $is_left:ident, $func_name:ident) => {
         fn $func_name(lhs: RcObject, rhs: RcObject, frame: &mut Frame) -> BoolResult {
            lhs.$func_name(rhs, frame)
         }
         pub const $oper_name: BinaryOperator = BinaryOperator {
            sigil: $sigil,
            func: FuncType::Bool($func_name),
            priority: $priority,
            is_left_assoc: $is_left
         };
      }

   }
   new_oper!(ADD, "+", 12, false, oper_add);
   new_oper!(SUB, "-", 12, false, oper_sub);
   new_oper!(MUL, "*", 11, false, oper_mul);
   new_oper!(DIV, "/", 11, false, oper_div);
   new_oper!(MOD, "%", 11, false, oper_mod);
   new_oper!(POW, "**", 10, true, oper_pow);

   new_oper!(BOOL; EQL, "==", 15, true, oper_eql);
   new_oper!(BOOL; NEQ, "!=", 15, true, oper_neq);
   new_oper!(BOOL; LTH, "<",  15, true, oper_lth);
   new_oper!(BOOL; LEQ, "<=", 15, true, oper_leq);
   new_oper!(BOOL; GTH, ">",  15, true, oper_gth);
   new_oper!(BOOL; GEQ, ">=", 15, true, oper_geq);
}

use std;
derive_impl!(Display; BinaryOperator, sigil);
derive_impl!(Debug; BinaryOperator, "Ob");

use objects::operators::Operator;
impl Operator for BinaryOperator {
   fn should_exec(&self, other: &Operator) -> bool {
      (other.is_left_assoc() && other.priority() >= self.priority()) ||
      (!other.is_left_assoc() && other.priority() > self.priority())
   }

   fn exec(&self, frame: &mut Frame) {
      let rhs = frame.pop().expect("bad rhs for operator");
      let lhs = frame.pop().expect("bad lhs for operator");
      let res = 
         match self.func {
            FuncType::Obj(func) => (func)(lhs, rhs, frame).expect("problem with exec of function"),
            FuncType::Bool(func) => (func)(lhs, rhs, frame).expect("problem with exec of function"),
         };
      frame.push(res);
   }
   fn priority(&self) -> u8 { self.priority }

}

use traits::misc::TryFrom;
impl <'a> TryFrom<&'a str> for BinaryOperator {
   fn try_from(inp: &'a str) -> Option<BinaryOperator> {
      match inp {
         "+" => Some(opers::ADD),
         "-" => Some(opers::SUB),
         "*" => Some(opers::MUL),
         "/" => Some(opers::DIV),
         "%" => Some(opers::MOD),
         "**" => Some(opers::POW),
         "==" => Some(opers::EQL),
         "!=" => Some(opers::NEQ),
         "<"  => Some(opers::LTH),
         "<=" => Some(opers::LEQ),
         ">"  => Some(opers::GTH),
         ">=" => Some(opers::GEQ),
         _ => None
      }
   }
}

















