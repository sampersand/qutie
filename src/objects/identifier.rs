use objects::object::Object;

pub struct Identifier<'a> {
   id: &'a str
}

use std;
impl <'a> std::fmt::Debug for Identifier<'a> {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "I({})", self)
   }
}

impl <'a> std::fmt::Display for Identifier<'a> {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "{}", self.id)
   }
}

use traits::ToRc;

impl <'a> From<&'a str> for Identifier<'a> {
   fn from(inp: &'a str) -> Identifier<'a> {
      Identifier{ id: inp }
   }
}
impl <'a> ToRc for Identifier<'a> {}

impl <'a> Object for Identifier<'a> {}




