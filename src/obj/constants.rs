use parsing::identifier::Identifier;
use obj::objects::object::Object;
use obj::objects::boolean;
use obj::traits::ToRc;
use std::rc::Rc;

pub fn get_constant(inp: &Identifier) -> Option<Rc<Object>> {
   match &**inp {
      "true" => Some(boolean::TRUE.to_rc()),
      "false" => Some(boolean::FALSE.to_rc()),
      _ => None,
   }
}