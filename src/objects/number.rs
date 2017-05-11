use objects::object::Object;

pub struct Number {
   num: i32
}

use std;
impl std::fmt::Debug for Number {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "N({})", self)
   }
}

impl std::fmt::Display for Number {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "{}", self.num)
   }
}

use objects::traits::ToRc;

impl From<String> for Number {
   fn from(inp: String) -> Number {
      match inp.parse::<i32>() {
         Ok(num) => Number{num: num},
         Err(_) => panic!("can't convert {} to a number", inp)
      }
   }
}
impl ToRc for Number {}
impl Object for Number {}

use std::rc::Rc;
use objects::result::ObjResult;
macro_rules! num_operator {
   ($oper:ident, $func:ident) => {
      use objects::traits::operators::$oper;
      impl $oper for Number {
         fn $func(&self, _: Rc<Object>) -> ObjResult {
            panic!("todo: oper")
         }
      }
   }
}

num_operator!(QtAdd, qt_add);
num_operator!(QtSub, qt_sub);
num_operator!(QtMul, qt_mul);
num_operator!(QtDiv, qt_div);
num_operator!(QtMod, qt_mod);
num_operator!(QtPow, qt_pow);




























