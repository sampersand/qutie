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

use obj::traits::misc::{
   QtCall
};

pub trait Object: Debug +
                  QtAdd + QtSub + QtMul + QtDiv + QtMod + QtPow +
                  QtEql + QtNeq + QtLth + QtGth + QtLeq + QtGeq +
                  ToBoolean + ToText + 
                  GetItem + SetItem + DelItem +
                  QtCall {
   fn obj_type(&self) -> ObjType; /* should be a static method, but then object cant be a type */
   fn is_a(&self, ty: ObjType) -> bool { self.obj_type() == ty }
   fn as_text_string(&self) -> String {
      self.to_text().expect("Can't convert to text").to_string()
   } 
   fn _eql(&self, other: &Object) -> bool {
      if let Ok(obj) = self.qt_eql(other) {
         obj.bool_val
      } else {
         false
      }
   }
}

#[derive(Debug, PartialEq)]
pub enum ObjType {
   Number,
   Text,
   Block,
   Boolean,
   Null,
   List,
   Map,
   Function,
   BuiltinFunction,
}






