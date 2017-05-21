use std::rc::Rc;
pub trait ToRc : Sized {
   fn to_rc(self) -> Rc<Self> {
      Rc::new(self)
   }
}

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

pub mod conversion {
   use std::rc::Rc;
   use obj::objects::object::Object;
   use obj::result::{ObjResult, ObjError};
   use obj::objects::boolean::Boolean;
   use obj::objects::text::Text;
   pub trait ToBoolean {
      fn to_boolean(&self) -> Result<Rc<Boolean>, ObjError> {
         Err(ObjError::NotImplemented)
      }
   }
   pub trait ToText {
      fn to_text(&self) -> Result<Rc<Text>, ObjError> {
         Err(ObjError::NotImplemented)
      }
   }
}


pub mod data { /* this is also pseudo-operator */
   use parsing::identifier::Identifier;
   use parsing::frame::Frame;
   use obj::result::{ObjResult, ObjError};
   use obj::objects::object::Object;
   use std::rc::Rc;

   pub trait GetItem {
      fn get_item(&self, item: Rc<Object>, frame: &mut Frame) -> ObjResult {
         Err(ObjError::NotImplemented)
      }
   }
   pub trait SetItem {
      fn set_item(&mut self, item: Rc<Object>, value: Rc<Object>, frame: &mut Frame) -> Result<(), ObjError> {
         Err(ObjError::NotImplemented)
      }
   }
   pub trait DelItem {
      fn del_item(&mut self, item: Rc<Object>, frame: &mut Frame) -> Result<(), ObjError> {
         Err(ObjError::NotImplemented)
      }
   }
}











