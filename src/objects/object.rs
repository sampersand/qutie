use std::fmt::Debug;
use std::rc::Rc;
use objects::result::{ObjResult, ObjError};

pub trait Object: Debug {}

macro_rules! default_oper {
   ($oper:ident, $func:ident) => {
      use objects::traits::operators::$oper;
      impl $oper for Object {
         fn $func(&self, _: Rc<Object>) -> ObjResult {
            Err(ObjError::NotImplemented)
         }
      }
   }
}
default_oper!(QtAdd, qt_add);
default_oper!(QtSub, qt_sub);
default_oper!(QtMul, qt_mul);
default_oper!(QtDiv, qt_div);
default_oper!(QtMod, qt_mod);
default_oper!(QtPow, qt_pow);