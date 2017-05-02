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

pub trait ToBool {
   // fn to_number(&self) -> Rc<Bool>;
}

pub trait Types : ToNumber + ToText + ToBool {}
