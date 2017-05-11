use objects::object::{Object, RcObject};
use objects::result::{ObjResult, ObjError, BoolResult};
use parsing::frame::Frame;
use objects::boolean::Boolean;

pub struct Number {
   num: i32
}

use std;
derive_impl!(Display; Number, num);
derive_impl!(Debug; Number, "N");
derive_impl!(ToRc; Number);
derive_impl!(Castable; Number);
derive_impl!(Opers; Number);
derive_impl!(Types; Number);
derive_impl!(ToText; Number, num);

use objects::traits::misc::TryFrom;
impl <'a> TryFrom<&'a str> for Number {
   fn try_from(inp: &'a str) -> Option<Number> {
      match inp.parse::<i32>() {
         Ok(num) => Some(Number{num: num}),
         Err(_) => None
      }
   }
}
use objects::traits::types::ToNumber;
use std::rc::Rc;
impl ToNumber for Number {
   fn to_number(&self) -> Result<Rc<Number>, ObjError> {
      Ok(Number{num: self.num}.to_rc())
   }
}
use objects::traits::types::ToBoolean;
impl ToBoolean for Number {
   fn to_boolean(&self) -> Result<Rc<Boolean>, ObjError> {
      Ok(Boolean::from(self.num != 0).to_rc())
   }
}


impl Object for Number {
   fn hash(&self) -> u8 {
      self.num as u8
   }
   fn _eql(&self, other: RcObject) -> bool {
      is_a!(other, number) && self.num == cast_as!(other, Number).num
   }
}

macro_rules! impl_num_oper {
   ($_trait:ident, $func:ident, $oper:tt) => {
      use objects::traits::operator::$_trait;
      impl $_trait for Number {
         fn $func(&self, other: RcObject, _: &mut Frame) -> ObjResult {
            if let Ok(other_num) = other.to_number() {
               Ok(Number{num: self.num $oper other_num.num}.to_rc())
            } else {
               Err(ObjError::NotImplemented)
            }
         }
      }
   }
}
macro_rules! impl_bool_oper {
   ($_trait:ident, $func:ident, $oper:tt) => {
      use objects::traits::operator::$_trait;
      impl $_trait for Number {
         fn $func(&self, other: RcObject, _: &mut Frame) -> BoolResult {
            if let Ok(other_num) = other.to_number() {
               Ok(Boolean::from(self.num $oper other_num.num).to_rc())
            } else {
               Err(ObjError::NotImplemented)
            }
         }
      }
   }
}

impl_num_oper!(OperAdd, oper_add, +);
impl_num_oper!(OperSub, oper_sub, -);
impl_num_oper!(OperMul, oper_mul, *);
impl_num_oper!(OperDiv, oper_div, /);
impl_num_oper!(OperMod, oper_mod, %);
impl_num_oper!(OperPow, oper_pow, &);
impl_bool_oper!(OperEql, oper_eql, ==);
impl_bool_oper!(OperNeq, oper_neq, !=);
impl_bool_oper!(OperLth, oper_lth,  <);
impl_bool_oper!(OperLeq, oper_leq, <=);
impl_bool_oper!(OperGth, oper_gth,  >);
impl_bool_oper!(OperGeq, oper_geq, >=);
derive_impl!(!; Number);












