use objects::number::Number;
use std::rc::Rc;

pub trait ToNumber {
   fn to_number(&self) -> Rc<Number>;
}
