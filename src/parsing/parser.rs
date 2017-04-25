use parsing::frame::Frame;
use objects::number::Number;
use objects::operators::binary_operator::BinaryOperator;
use objects::traits::misc::TryFrom;

fn is_numeric(token: &str) -> bool {
   for chr in token.chars(){
      if !chr.is_numeric() {
         return false;
      }
   }
   return true;
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
      }

      try_object!(Number);
      try_object!(BinaryOperator);
      exception!(SYNTAX; "Unexpected token: `{}`", token);
   }
}



















