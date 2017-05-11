use parsing::identifier::Identifier;
use parsing::token::Token;
use parsing::frame::Frame;
use obj::objects::object::{Object, ObjType};
use obj::objects::boolean;
use obj::traits::ToRc;
use std::rc::Rc;
use parsing::parser::parse_expr;
use obj::objects::block::Block;

macro_rules! next_arg {
   ($tokens:expr, $frame:expr, $err:expr) => {
      if $tokens.is_empty() {
         panic!($err)
      } else {
         parse_expr(vec![$tokens.remove(0)], $frame);
         $frame.pop().unwrap()
      }
   }
}
fn handle_debug(tokens: &mut Vec<Token>, frame: &mut Frame) {
   let args = next_arg!(tokens, frame, "no debug arg");
   println!("debug: {:?}", args);
}

fn handle_if(tokens: &mut Vec<Token>, frame: &mut Frame) {
   let cond = next_arg!(tokens, frame, "no condition"); /* could go til we get a squiggly block */
   let if_true = next_arg!(tokens, frame, "no if true");
   let has_false = 
      match tokens.first() {
         None => false,
         Some(e) =>
            match e {
               &Token::Identifier(ref o) => &**o == "else",
               _ => false
            }
      };
   let if_false = 
      if has_false {
         tokens.remove(0); /* else */
         Some(next_arg!(tokens, frame, "no false condition"))
      } else {
         None
      };
   if cond.to_boolean().expect("can't convert condition to boolean").val {
      if if_true.is_a(ObjType::Block) {
         parse_expr(cast_as!(&if_true, Block).body.clone(), frame);
      } else {
         frame.push(if_true)
      }
   } else {
      if let Some(if_false) = if_false {
         if if_false.is_a(ObjType::Block) {
            parse_expr(cast_as!(&if_false, Block).body.clone(), frame);
         } else {
            frame.push(if_false)
         }
      }
   }
}


pub fn handle_control(inp: &Identifier, tokens: &mut Vec<Token>, frame: &mut Frame) -> bool {
   match &**inp {
      "__debug" => handle_debug(tokens, frame),
      "if" => handle_if(tokens, frame),
      _ => return false
   } ;
   true
}







