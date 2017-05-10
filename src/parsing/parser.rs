use parsing::stream::Stream;
use parsing::frame::Frame;
use parsing::Token;
use traits::ToRc;
use std::rc::Rc;
use objects::number::Number;
use objects::object::Object;

pub fn parse<'a>(stream: &'a mut Stream<'a>) {
   let ref mut frame = Frame::new();
   // while !stream.is_empty() {
      parse_expr(stream, frame);
   // }
   print!("frame: {:?}", frame);
}

fn parse_expr(stream: &mut Stream, frame: &mut Frame) {
   if stream.is_empty() { return }
   println!("next expr: {:?}", next_expr(stream));
}

fn next_expr(stream: &mut Stream) -> Vec<Token> {
   use parsing::Token::*;
   let mut ret = vec![];
   let mut i = 0;
   while !stream.is_empty() {
      i += 1;
      if i >200 { panic!()}
      match stream.next() {
         LineTerminator(_) =>  break,
         Unknown(chr) => panic!("Unknown character: {:?}", chr),
         token @ _ => ret.push(token)
      }
      println!("ret: {:?}", ret);
   }
   ret
}