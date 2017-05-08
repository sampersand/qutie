use std::rc::Rc;
pub trait ToRc : Sized {
   fn to_rc(self) -> Rc<Self> {
      Rc::new(self)
   }
}

pub trait TryFrom<T> : Sized {
   fn try_from(inp: T) -> Option<Self>;
}

pub trait Castable {
   fn type_id(&self) -> u8;
}