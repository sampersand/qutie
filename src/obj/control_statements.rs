use parsing::identifier::Identifier;
use parsing::token::Token;
use parsing::frame::Frame;
use obj::objects::object::{Object, ObjType};
use obj::objects::boolean;
use obj::objects::function::Function;
use obj::traits::ToRc;
use std::rc::Rc;
use parsing::parser;
use obj::objects::block::Block;

macro_rules! exec {
   ($tokens:expr, $frame:expr) => {{
      handle_expressions($tokens, $frame);
      $frame.pop().unwrap()
   }}
}
macro_rules! next_arg {
   ($tokens:expr, $frame:expr, $err:expr) => {
      if $tokens.is_empty() {
         panic!($err)
      } else {
         exec!(vec![$tokens.remove(0)], $frame)
      }
   }
}
fn handle_debug(tokens: &mut Vec<Token>, frame: &mut Frame) {
   let args = next_arg!(tokens, frame, "no debug arg");
   println!("debug: {:?} | {:?}", args, frame);
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
         parser::exec_exprs(cast_as!(&if_true, Block).body.clone(), frame);
      } else {
         frame.push(if_true)
      }
   } else {
      if let Some(if_false) = if_false {
         if if_false.is_a(ObjType::Block) {
            parser::exec_exprs(cast_as!(&if_false, Block).body.clone(), frame);
         } else {
            frame.push(if_false)
         }
      }
   }
}

fn handle_while(tokens: &mut Vec<Token>, frame: &mut Frame) {
   let cond = next_block!(tokens);
   let body = next_block!(tokens);
   while exec!(cond.clone(), frame).
            to_boolean().
            expect("can't convert condition to boolean").
            val {
      parser::exec_exprs(body.clone(), frame);
   }
}
fn handle_func(tokens: &mut Vec<Token>, frame: &mut Frame) {
   let mut args = next_block!(tokens);
   let mut ident_args = vec![];
   while !args.is_empty() {
      match args.remove(0) {
         Token::Identifier(ident) => ident_args.push(ident),
         Token::Separator => { /* do nothing cause its a separator */ }
         arg @ _ => panic!("unexpected non-ident token type: {:?}", arg)
      }   
   }
   let body = next_block!(tokens);
   let file = frame.file.clone();
   let lineno = frame.lineno;
   frame.push(Function::new(file, lineno, ident_args, body.clone()).to_rc());
}

pub fn handle_control(inp: &Identifier, tokens: &mut Vec<Token>, frame: &mut Frame) -> bool {
   match &**inp {
      "__debug" => handle_debug(tokens, frame),
      "if" => handle_if(tokens, frame),
      "while" => handle_while(tokens, frame),
      "func" => handle_func(tokens, frame),
      _ => return false
   } ;
   true
}










