#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
   pub id: String
}
impl Identifier {
   pub fn from(inp: String) -> Identifier {
      Identifier{ id: inp}
   }
   pub fn to_string(&self) -> String {
      return self.id.to_string()
   }
}
use std;
impl_defaults!(Display; to_string; Identifier);
