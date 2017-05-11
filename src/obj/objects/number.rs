use obj::objects::object::{Object, ObjType};

pub struct Number {
   num: i32
}

impl Number {
   #[inline]
   pub fn new(inp: i32) -> Number {
      Number{num: inp}
   }
}
impl <'a> From<&'a str> for Number {
   fn from(inp: &'a str) -> Number {
      match inp.parse::<i32>() {
         Ok(num) => Number::new(num),
         Err(_) => panic!("can't convert {} to a number", inp)
      }
   }
}

use std;
impl_defaults!(Debug; Number, 'N');
impl_defaults!(Display; Number, num);
impl_defaults!(ToRc; Number);
impl_defaults!(Object; Number);

use std::rc::Rc;
use obj::result::{ObjResult, ObjError, BoolResult};
use obj::objects::boolean::Boolean;
macro_rules! num_operator {
   ($oper_trait:ident, $func:ident, $oper:tt, ObjResult) => {
      use obj::traits::operators::$oper_trait;
      impl $oper_trait for Number {
         fn $func(&self, other: &Rc<Object>) -> ObjResult {
            if !other.is_a(ObjType::Number) {
               return Err(ObjError::NotImplemented)
            }
            Ok(Number::new(self.num $oper cast_as!(other, Number).num).to_rc())
         }
      }
   };
   ($oper_trait:ident, $func:ident, $oper:tt, BoolResult) => {
      use obj::traits::operators::$oper_trait;
      impl $oper_trait for Number {
         fn $func(&self, other: &Rc<Object>) -> BoolResult {
            if !other.is_a(ObjType::Number) {
               return Err(ObjError::NotImplemented)
            }
            Ok(Boolean::get(self.num $oper cast_as!(other, Number).num).to_rc())
         }
      }
   }
}

use obj::traits::conversion::ToBoolean;
impl ToBoolean for Number {
   fn to_boolean(&self) -> BoolResult {
      Ok(Boolean::get(self.num != 0).to_rc())
   }
}

num_operator!(QtAdd, qt_add, +, ObjResult);
num_operator!(QtSub, qt_sub, -, ObjResult);
num_operator!(QtMul, qt_mul, *, ObjResult);
num_operator!(QtDiv, qt_div, /, ObjResult);
num_operator!(QtMod, qt_mod, %, ObjResult);
num_operator!(QtPow, qt_pow, &, ObjResult); /* todo: pow */
num_operator!(QtEql, qt_eql, ==, BoolResult);
num_operator!(QtNeq, qt_neq, !=, BoolResult);
num_operator!(QtLth, qt_lth, <,  BoolResult);
num_operator!(QtGth, qt_gth, >,  BoolResult);
num_operator!(QtLeq, qt_leq, <=, BoolResult);
num_operator!(QtGeq, qt_geq, >=, BoolResult); /* todo: pow */




























