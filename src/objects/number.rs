use objects::object::Object;

pub struct Number {
   num: i32
}

use std;
impl std::fmt::Debug for Number {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "N({})", self)
   }
}

impl std::fmt::Display for Number {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "{}", self.num)
   }
}

use traits::ToRc;

impl <'a> From<&'a str> for Number {
   fn from(inp: &'a str) -> Number {
      match inp.parse::<i32>() {
         Ok(num) => Number{num: num},
         Err(_) => panic!("can't convert {} to a number", inp)
      }
   }
}
impl ToRc for Number {}

impl Object for Number {}




