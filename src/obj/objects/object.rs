use std::fmt::Debug;
use std::rc::Rc;
use obj::result::{ObjResult, ObjError};

use obj::traits::operators::{
   QtAdd, QtSub, QtMul, QtDiv, QtMod, QtPow,
   QtEql, QtNeq, QtLth, QtGth, QtLeq, QtGeq
};

use obj::traits::conversion::{
   ToBoolean, ToText
};

use obj::traits::data::{
   GetItem, SetItem, DelItem
};

pub trait Object: Debug +
                  QtAdd + QtSub + QtMul + QtDiv + QtMod + QtPow +
                  QtEql + QtNeq + QtLth + QtGth + QtLeq + QtGeq +
                  ToBoolean + ToText + 
                  GetItem + SetItem + DelItem {
   fn obj_type(&self) -> ObjType; /* should be a static method, but then object cant be a type */
   fn is_a(&self, ty: ObjType) -> bool {
      self.obj_type() == ty
   }
}
#[derive(Debug, PartialEq)]
pub enum ObjType {
   Number,
   // Identifier, /* maybe path ? */
   Text,
   Block,
   Boolean,
   List,
   Function,
   Null
}

trait Castable<T: Object> {
   fn cast_as(&self) -> Option<Rc<T>>;
}

unsafe fn __cast_as<T: Object>(inp: &Rc<Object>) -> Rc<T> {
   use std::mem::transmute;
   use std::rc::Rc;
   transmute::<&Rc<Object>, &Rc<T>>(inp).clone()
}

macro_rules! impl_castable {
   ($obj_ty:ty, $first:ident::$obj_ident:ident) => {
      use obj::objects::$first::$obj_ident;
      impl Castable<$obj_ty> for Rc<Object> {
         fn cast_as(&self) -> Option<Rc<$obj_ty>> {
            if self.is_a(ObjType::$obj_ident) {
               Some(unsafe{ __cast_as::<$obj_ty>(self) })
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








