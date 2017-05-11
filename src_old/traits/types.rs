use objects::result::ObjError;
use std::rc::Rc;

use objects::number::Number;
pub trait ToNumber {
   fn to_number(&self) -> Result<Rc<Number>, ObjError> {
      Err(ObjError::NotImplemented)
   }
}

pub trait ToText {
   // fn to_text(&self) -> Rc<Text>;
}

use objects::boolean::Boolean;
use objects::traits::misc::ToRc;

pub trait ToBoolean {
   fn to_boolean(&self) -> Result<Rc<Boolean>, ObjError> {
      Ok(Boolean::from(true).to_rc())
   }
}

pub trait Types : ToNumber + ToText + ToBoolean {}
