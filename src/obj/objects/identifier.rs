use obj::objects::object::{Object, ObjType};
use obj::traits::ToRc;

pub struct Identifier {
   id: String
}

use std;
impl_defaults!(Display; Identifier, id);
impl_defaults!(Debug; Identifier, 'I');


impl From<String> for Identifier {
   fn from(inp: String) -> Identifier {
      Identifier{ id: inp }
   }
}
impl ToRc for Identifier {}

impl_defaults!(Object; Identifier);

impl_defaults!(QtAdd; Identifier);
impl_defaults!(QtSub; Identifier);
impl_defaults!(QtMul; Identifier);
impl_defaults!(QtDiv; Identifier);
impl_defaults!(QtMod; Identifier);
impl_defaults!(QtPow; Identifier);


