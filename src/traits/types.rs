use objects::number::Number;
use std::rc::Rc;

pub trait ToNumber {
   fn to_number(&self) -> Rc<Number>;
}

pub trait ToText {
   // fn to_text(&self) -> Rc<Text>;
}

pub trait ToBool {
   // fn to_number(&self) -> Rc<Bool>;
}

pub trait Types : ToNumber + ToText + ToBool {}
