use parsing::stream::Stream;
use parsing::frame::Frame;
use objects::number::Number;
use objects::operators::binary_operator::BinaryOperator;
use objects::traits::misc::{TryFrom, ToRc};
use objects::rc_object::RcObject;

enum Token {
   Paren(String),

}

fn next_token(stream: &mut Stream) -> Option<Token> {
   panic!()
}

pub fn exec_frame(frame: &mut Frame){
   let mut token_stack = vec![];
   let mut oper_stack = vec![];
   while let Some(token) = next_token(frame.stream) {
      match token {
         Paren(paren) => panic!(),
         Text
         Number(num) => token_stack.push(token)
         // "{" | "}" | "(" | ")" | "[" | "]" => todo!("parenthesis"),
         // "+" => todo!("addition"),
         _ => token_stack.push(token)
      };
   }
   // while let Some(token) = frame.stream.next_token() {
   //       ($object:ident, $func:ident) => {
   //          if let Some(obj) = $object::try_from(token.as_str()) {
   //             frame.push_stack($func(obj.to_rc(), frame));
   //             continue;
   //          }
   //       }
   //    }

   //    try_object!(Number);
   //    try_object!(BinaryOperator, handle_binary);

   //    exception!(SYNTAX; "Unexpected token: `{}`", token);
   // }
}



















