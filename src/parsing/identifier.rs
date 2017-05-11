#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
   pub id: String
}
impl Identifier {
   pub fn from(inp: String) -> Identifier {
      Identifier{ id: inp}
   }
}
use std;
impl_defaults!(Display; to_string; Identifier);
