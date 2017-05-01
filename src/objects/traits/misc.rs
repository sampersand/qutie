use std::rc::Rc;
pub trait ToRc : Sized {
   fn to_rc(self) -> Rc<Self> {
      Rc::new(self)
   }
}
pub trait TryFrom : Sized {
   fn try_from(inp: &str) -> Option<Self>;
}

pub trait Castable : Sized {
   fn type_id() -> u8;
   fn is_a<T: Castable>(&self) -> bool { Self::type_id() == T::type_id() }
   fn cast<'a, T: Castable>(&'a self) -> &'a T {
      if !self.is_a::<T>() { 
         panic!("cannot cast `{}` to `{}`", Self::type_id(), T::type_id());
      }
      use std::mem;
      assert_eq!(mem::size_of::<Self>(), mem::size_of::<T>(), "bad types!");
      unsafe { mem::transmute(self) }
   }
}