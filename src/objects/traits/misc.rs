use std::rc::Rc;
pub trait ToRc : Sized {
   fn to_rc(self) -> Rc<Self> {
      Rc::new(self)
   }
}
pub trait TryFrom : Sized {
   fn try_from(inp: &str) -> Option<Self>;
}

pub trait Castable {
   fn is_a<T: Castable>(&self) -> bool { self.cast_as::<T>().is_some() }
   fn cast_as<T: Castable>(&self) -> Option<T>;
}

struct Foo {}
impl Castable for Foo {
   fn cast_as<T: Castable>(&self) -> Option<T> { panic!() }
}

struct Bar {}
impl Castable for Bar {
   fn cast_as<T: Castable>(&self) -> Option<T> { panic!() }
}


fn foobar(){
   let foo = Foo{};
   let bar: Bar = foo.cast_as::<Bar>().unwrap();
}