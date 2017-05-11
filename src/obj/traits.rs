use std::rc::Rc;

/************* operators *************/
pub mod operators {
   use std::rc::Rc;
   use obj::objects::object::Object;
   use obj::result::{ObjResult, ObjError};
   macro_rules! def_binary_oper {
      ($name:ident, $func:ident) => {
         pub trait $name {
            fn $func(&self, other: Rc<Object>) -> ObjResult{
               Err(ObjError::NotImplemented)
            }
         }
      }
   }
   def_binary_oper!(QtAdd, qt_add);
   def_binary_oper!(QtSub, qt_sub);
   def_binary_oper!(QtMul, qt_mul);
   def_binary_oper!(QtDiv, qt_div);
   def_binary_oper!(QtMod, qt_mod);
   def_binary_oper!(QtPow, qt_pow);
}






