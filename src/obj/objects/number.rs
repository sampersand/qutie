use obj::objects::object::{Object, ObjType};
use obj::traits::ToRc;

pub struct Number {
   num: i32
}

use std;
impl_defaults!(Debug; Number, 'N');
impl_defaults!(Display; Number, num);

impl Number {
   #[inline]
   pub fn new(inp: i32) -> Number {
      Number{num: inp}
   }
}
impl From<String> for Number {
   fn from(inp: String) -> Number {
      match inp.parse::<i32>() {
         Ok(num) => Number::new(num),
         Err(_) => panic!("can't convert {} to a number", inp)
      }
   }
}

impl ToRc for Number {}
impl_defaults!(Object; Number);

use std::rc::Rc;
use obj::result::{ObjResult, ObjError};
macro_rules! num_operator {
   ($oper_trait:ident, $func:ident, $oper:tt) => {
      use obj::traits::operators::$oper_trait;
      impl $oper_trait for Number {
         fn $func(&self, other: &Rc<Object>) -> ObjResult {
            if !other.is_a(ObjType::Number) {
               return Err(ObjError::NotImplemented)
            }
            Ok(Number::new(self.num $oper cast_as!(other, Number).num).to_rc())
         }
      }
   }
}

num_operator!(QtAdd, qt_add, +);
num_operator!(QtSub, qt_sub, -);
num_operator!(QtMul, qt_mul, *);
num_operator!(QtDiv, qt_div, /);
num_operator!(QtMod, qt_mod, %);
num_operator!(QtPow, qt_pow, &); /* todo: pow */




























