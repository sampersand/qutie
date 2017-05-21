use parsing::identifier::Identifier;
use obj::objects::object::Object;
use obj::objects::{boolean, null};
use obj::traits::ToRc;
use std::rc::Rc;

pub fn get_constant(inp: &Identifier) -> Option<Rc<Object>> {
   match &**inp {
      "true" => Some(boolean::TRUE.to_rc()),
      "false" => Some(boolean::FALSE.to_rc()),
      "null" => Some(null::NULL.to_rc()),
      "nil" => Some(null::NULL.to_rc()),
      "none" => Some(null::NULL.to_rc()),
      _ => None,
   }
}