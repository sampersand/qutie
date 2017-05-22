use obj::objects::object::{Object, ObjType};
use obj::traits::Castable;

pub struct Number {
   pub num: i32
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
            if let Some(other_num) = other.cast() {
               Ok(Number::new(self.num $oper (other_num as Rc<Number>).num).to_rc())
            } else {
               return Err(ObjError::NotImplemented)
            }
         }
      }
   };
   ($oper_trait:ident, $func:ident, $oper:tt, BoolResult) => {
      use obj::traits::operators::$oper_trait;
      impl $oper_trait for Number {
         fn $func(&self, other: &Rc<Object>) -> BoolResult {
            if let Some(other_num) = other.cast() {
               Ok(Boolean::get(self.num $oper (other_num as Rc<Number>).num).to_rc())
            } else {
               return Err(ObjError::NotImplemented)
            }
         }
      }
   }
}

use obj::traits::conversion::{ToBoolean, ToText};
impl ToBoolean for Number {
   fn to_boolean(&self) -> Result<Rc<Boolean>, ObjError> {
      Ok(Boolean::get(self.num != 0).to_rc())
   }
}
use obj::objects::text::Text;
impl ToText for Number {
   fn to_text(&self) -> Result<Rc<Text>, ObjError> {
      Ok(Text::from(self.num.to_string()).to_rc())
   }
}

impl_traits!(data=GetItem, Number);
impl_traits!(data=SetItem, Number);
impl_traits!(data=DelItem, Number);
impl_traits!(misc=QtCall, Number);

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




























