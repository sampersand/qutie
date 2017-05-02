use objects::object::{Object, RcObject};
use objects::result::ObjResult;
use parsing::frame::Frame;

pub struct Identifier {
   id: String // todo: update this
}

use std;
derive_impl!(Display; Identifier, id);
derive_impl!(Debug; Identifier, "I");
derive_impl!(ToRc; Identifier);
derive_impl!(Castable; Identifier);
derive_impl!(Opers; Identifier);
derive_impl!(Types; Identifier);
derive_impl!(ToText; Identifier, id);
derive_impl!(ToNumber; Identifier);
derive_impl!(ToBool; Identifier);


use traits::misc::TryFrom;
impl TryFrom for Identifier {
   fn try_from(inp: &str) -> Option<Identifier> {

      match inp.chars().nth(0) {
         Some(c) if is_char!(alphabetic; c) => {},
         _ => return None
      };

      for c in inp.chars() {
         if !is_char!(alphanumeric; c) {
            return None
         }
      }
      Some(Identifier{id: inp.to_string()})
   }
}
use std::rc::Rc;


impl Object for Identifier {
   fn hash(&self) -> u8 {
      self.id.chars().nth(0).unwrap() as u8
   }
   fn _eql(&self, other: RcObject) -> bool {
      todo!("_eql for number")
   }
}

derive_impl!(+; Identifier);
derive_impl!(-; Identifier);
derive_impl!(*; Identifier);
derive_impl!(/; Identifier);
derive_impl!(%; Identifier);
derive_impl!(**; Identifier);












