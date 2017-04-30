use std::rc::Rc;
use parsing::frame::Frame;
use parsing::stream::Stream;
use objects::number::Number;
use objects::operators::binary_operator::BinaryOperator;
use objects::traits::misc::{TryFrom, ToRc};
use objects::rc_object::RcObject;

#[derive(PartialEq, Debug, Clone)]
enum Token {
   Identifier(String),
   Symbol(String),
   Operator(String),
   LParen(char),
   RParen(char),
   Unknown(char),
}
// fn next_number(stream: &mut Stream) -> Token {
//    let mut acc = String::new();
//    while let Some(c) = stream.peek() {
//       match c {
//          c @ _ if c.is_digit(10) => acc.push(stream.take(c)),
//          _ => break
//       };
//    }
//    Token::Number(acc)
// }

// fn next_token(stream: &mut Stream) -> Token {
//    let c = stream.peek().unwrap();
//    match c {
//       '(' | '[' | '{' => Token::LParen(stream.take(c)),
//       ')' | ']' | '}' => Token::RParen(stream.take(c)),
//       _ if c.is_digit(10) => next_number(stream),
//       _ => Token::Unknown(stream.take(c))
//    }
// }

// fn get_lparen(rparen: char) -> char {
//    match rparen {
//       ')' => '(',
//       ']' => '[',
//       '}' => '{',
//       _ => unreachable!("bad rparen: {:?}", rparen)
//    }
// }

pub fn exec_frame(frame: &mut Frame){
   let mut acc = String::new();
   loop {
      let c =
         if let Some(c) = frame.stream.borrow_mut().next() {
            c
         } else {
            break
         };
      match c {
         _ if c.is_whitespace() => { continue },
         '(' => 
            if let Some(obj) = frame.fork().exec().pop() {
               frame.push(obj);
            } else {
               todo!("add null object");
            },
         ')'  => break,
         _ => {}
      };
   }
   println!("foo");
   
}



















