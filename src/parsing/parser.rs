use parsing::stream::Stream;
use parsing::frame::Frame;
use parsing::Token;
use traits::ToRc;
use std::rc::Rc;
use objects::number::Number;
use objects::object::Object;

pub fn parse<'a>(stream: &'a mut Stream<'a>) {
   let ref mut frame = Frame::new();
   while !stream.is_empty() {
      parse_expr(next_expr(stream), frame);
   }
   print!("frame: {:?}", frame);
}

fn parse_expr(expr: Vec<Token>, frame: &mut Frame) {

   println!("next expr: {:?}", expr);
}

fn next_expr(stream: &mut Stream) -> Vec<Token> {
   use parsing::Token::*;
   let mut ret = vec![];
   while let Some(token) = stream.next() {
      match token {
         LineTerminator(_) =>  break,
         Unknown(chr) => panic!("Unknown character: {:?}", chr),
         token @ _ => ret.push(token)
      }
   }
   ret
}