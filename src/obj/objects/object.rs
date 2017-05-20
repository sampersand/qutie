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
pub trait Object: Debug +
                  QtAdd + QtSub + QtMul + QtDiv + QtMod + QtPow +
                  QtEql + QtNeq + QtLth + QtGth + QtLeq + QtGeq +
                  ToBoolean + ToText {
   fn obj_type(&self) -> ObjType; /* should be a static method, but then object cant be a type */
   fn is_a(&self, ty: ObjType) -> bool {
      self.obj_type() == ty
   }
}
#[derive(Debug, PartialEq)]
pub enum ObjType {
   Number,
   Identifier, /* maybe path ? */
   Text,
   Block,
   Boolean,
   List,
   Function
}
