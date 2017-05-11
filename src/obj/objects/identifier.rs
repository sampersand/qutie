use obj::objects::object::Object;

pub struct Identifier {
   id: String
}

use std;
impl std::fmt::Debug for Identifier {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "I({})", self)
   }
}

impl std::fmt::Display for Identifier {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "{}", self.id)
   }
}

use obj::traits::ToRc;

impl From<String> for Identifier {
   fn from(inp: String) -> Identifier {
      Identifier{ id: inp }
   }
}
impl ToRc for Identifier {}

impl Object for Identifier {}




