use parsing::identifier::Identifier;
use parsing::token::Token;
use parsing::frame::Frame;
use parsing::expression::Expression;
use obj::objects::object::{Object, ObjType};
use obj::objects::boolean;
use obj::objects::null::Null;
use obj::objects::function::Function;
use obj::traits::{ToRc, Castable};
use std::rc::Rc;
use obj::objects::block::Block;

macro_rules! next_obj { 
   ($expr:expr, $frame:expr) => {{
      $expr.exec($frame);
      $frame.pop().expect("can't find next arg")
   }};

}
macro_rules! next_arg {
   ($expr:expr, $frame:expr, $err:expr) => {
      if $expr.is_empty() {
         panic!($err)
      } else {
         next_obj!(Expression::new(vec![$expr.pop_front().unwrap()], false), $frame)
      }
   }
}
fn handle_debug(expr: &mut Expression, frame: &mut Frame) {
   let args = next_arg!(expr, frame, "no debug arg");
   println!("debug: {:?}", args);
   // println!("debug: {:?} | {:?}", args, frame);
}

fn handle_if(expr: &mut Expression, frame: &mut Frame) {
   let cond = next_arg!(expr, frame, "no condition"); /* could go til we get a squiggly block */
   let if_true = next_arg!(expr, frame, "no if true");
   let has_false = 
      match expr.peek_front() {
         None => false,
         Some(e) =>
            match e {
               &Token::Identifier(ref o) => &**o == "else",
               _ => false
            }
      };
   let if_false = 
      if has_false {
         expr.pop_front(); /* else */
         next_arg!(expr, frame, "no false condition")
      } else {
         Null::get().to_rc()
      };
   if cond.to_boolean().expect("can't convert condition to boolean").val {
      if let Some(block) = if_true.clone().cast(){
         (block as Rc<Block>).exec_no_pop(frame);
      } else {
         frame.push(if_true)
      }
   } else {
      if let Some(if_false_block) = if_false.cast(){
         (if_false_block as Rc<Block>).exec_no_pop(frame);
      } else {
         frame.push(if_false)
      }
   }
}

fn handle_while(expr: &mut Expression, frame: &mut Frame) {
   let cond = expr.next_block().expect("no cond found for while loop");
   let body = expr.next_block().expect("no body found for while loop");
// {Expression::exec_exprs(cond.clone(), frame); frame.pop().unwrap() }
   while cond.clone().
              exec(frame).
              expect("no result found from condition").
              to_boolean().
              expect("can't convert condition to boolean").
              val {
      body.clone().exec_no_pop(frame);
   }
}

fn handle_func(expr: &mut Expression, frame: &mut Frame) {
   let mut args = expr.next_block().
                         expect("no function args found").
                         pop_single_expr().
                         expect("need single expression for arg");
   let mut ident_args = vec![];
   while !args.is_empty() {
      match args.pop_front().unwrap() {
         Token::Identifier(ident) => ident_args.push(ident),
         Token::Separator => { /* do nothing cause its a separator */ }
         arg @ _ => panic!("unexpected non-ident token type: {:?}", arg)
      }   
   }
   let body = expr.next_block().expect("no body found");
   let file = frame.file.clone();
   let lineno = frame.lineno;
   frame.push(Function::new(file, lineno, ident_args, body.clone()).to_rc());
}
fn handle_return(expr: &mut Expression, frame: &mut Frame) {
   // expr.clone().exec(frame);
   // expr.clear();
   let val = frame.pop().expect("cant set a key to nothing!");
   println!("ret: {:?}", val);
   panic!("<return>")
}

pub fn handle_control(inp: &Identifier, expr: &mut Expression, frame: &mut Frame) -> bool {
   match &**inp {
      "__debug" => handle_debug(expr, frame),
      "if" => handle_if(expr, frame),
      "while" => handle_while(expr, frame),
      "func" => handle_func(expr, frame),
      "return" => handle_return(expr, frame),
      _ => return false
   } ;
   true
}










