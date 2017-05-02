use objects::object::{Object, RcObject};
use objects::result::{ObjResult, ObjError};
use parsing::frame::Frame;

pub struct Number {
   num: i32
}

use std;
derive_impl!(Display; Number, num);
derive_impl!(Debug; Number, "N");
derive_impl!(ToRc; Number);
derive_impl!(Castable; Number);

use traits::misc::TryFrom;
impl TryFrom for Number {
   fn try_from(inp: &str) -> Option<Number> {
      match inp.parse::<i32>() {
         Ok(num) => Some(Number{num: num}),
         Err(_) => None
      }
   }
}
use traits::types::ToNumber;
use std::rc::Rc;
impl ToNumber for Number {
   fn to_number(&self) -> Result<Rc<Number>, ObjError> {
      Ok(Number{num: self.num}.to_rc())
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
      use traits::operator::$_trait;
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
impl_num_oper!(OperAdd, oper_add, +);
impl_num_oper!(OperSub, oper_sub, -);
impl_num_oper!(OperMul, oper_mul, *);
impl_num_oper!(OperDiv, oper_div, /);
impl_num_oper!(OperMod, oper_mod, %);
impl_num_oper!(OperPow, oper_pow, &);

derive_impl!(Opers; Number);
derive_impl!(Types; Number);
derive_impl!(ToText; Number, num);
derive_impl!(ToBool; Number, num);












