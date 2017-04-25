use parsing::frame::Frame;
use objects::number::Number;
use objects::operators::binary_operator::BinaryOperator;
use objects::traits::misc::{TryFrom, ToRc};
use objects::rc_object::RcObject;
fn handle_binary(obj: RcObject, frame: &mut Frame){
   
}

pub fn exec(frame: &mut Frame){
   while let Some(token) = frame.stream.next_token() {
      macro_rules! try_object {
         ($object:ident) => {
            if let Some(obj) = $object::try_from(token.as_str()) {
               frame.push_stack(obj.to_rc());
               continue;
            }
         }
         ($object:ident, $func:ident) => {
            if let Some(obj) = $object::try_from(token.as_str()) {
               frame.push_stack($func(obj.to_rc(), frame));
               continue;
            }
         }
      }

      try_object!(Number);
      try_object!(BinaryOperator, handle_binary);

      exception!(SYNTAX; "Unexpected token: `{}`", token);
   }
}



















