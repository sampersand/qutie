use std::rc::Rc;
pub trait ToRc : Sized {
   fn to_rc(self) -> Rc<Self> {
      Rc::new(self)
   }
}
pub trait TryFrom : Sized {
   fn try_from(inp: &str) -> Option<Self>;
}