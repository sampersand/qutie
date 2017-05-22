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

pub mod misc {
   use obj::result::{ObjResult, ObjError};
   use parsing::expression::Expression;
   use parsing::frame::Frame;
   pub trait QtCall {
      fn qt_call(&self, args: Expression, frame: &mut Frame) -> ObjResult {
         Err(ObjError::NotImplemented)
      }
      fn is_callable(&self) -> bool {
         false
      }
   }
}






use obj::objects::object::{Object, ObjType};

pub trait Castable<T: Object> {
   fn cast(&self) -> Option<Rc<T>>;
   fn force_cast(&self) -> Rc<T> {
      if let Some(rc) = self.cast(){
         rc
      } else {
         panic!("Cannot cast self to type <??>")
      }
   }
}

unsafe fn __force_cast<T: Object>(inp: &Rc<Object>) -> Rc<T> {
   use std::mem::transmute;
   use std::rc::Rc;
   transmute::<&Rc<Object>, &Rc<T>>(inp).clone()
}

macro_rules! impl_castable {
   ($obj_ty:ty, $first:ident::$obj_ident:ident) => {
      use obj::objects::$first::$obj_ident;
      impl Castable<$obj_ty> for Rc<Object> {
         fn cast(&self) -> Option<Rc<$obj_ty>> {
            if self.is_a(ObjType::$obj_ident) {
               Some(unsafe{ __force_cast::<$obj_ty>(self) })
            } else {
               None
            }
         }
      }
   }
}
impl_castable!(Number, number::Number);
impl_castable!(Block, block::Block);
impl_castable!(Boolean, boolean::Boolean);
impl_castable!(Function, function::Function);
impl_castable!(List, list::List);
impl_castable!(Null, null::Null);
impl_castable!(Text, text::Text);


















