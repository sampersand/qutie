use std::rc::Rc;
pub trait ToRc : Sized {
   fn to_rc(self) -> Rc<Self> {
      Rc::new(self)
   }
}

/************* operators *************/
pub mod operators {
   use std::rc::Rc;
   use obj::objects::object::Object;
   use obj::result::{ObjResult, BoolResult, ObjError};
   macro_rules! def_binary_oper {
      ($name:ident, $func:ident, $ret_ty:ty) => {
         pub trait $name {
            fn $func(&self, other: &Rc<Object>) -> $ret_ty {
               Err(ObjError::NotImplemented)
            }
         }
      }
   }
   def_binary_oper!(QtAdd, qt_add, ObjResult);
   def_binary_oper!(QtSub, qt_sub, ObjResult);
   def_binary_oper!(QtMul, qt_mul, ObjResult);
   def_binary_oper!(QtDiv, qt_div, ObjResult);
   def_binary_oper!(QtMod, qt_mod, ObjResult);
   def_binary_oper!(QtPow, qt_pow, ObjResult);
   def_binary_oper!(QtEql, qt_eql, BoolResult);
   def_binary_oper!(QtNeq, qt_neq, BoolResult);
   def_binary_oper!(QtGth, qt_gth, BoolResult);
   def_binary_oper!(QtLth, qt_lth, BoolResult);
   def_binary_oper!(QtGeq, qt_geq, BoolResult);
   def_binary_oper!(QtLeq, qt_leq, BoolResult);

}













