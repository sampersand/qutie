use std::rc::Rc;
use parsing::frame::Frame;
use parsing::stream::Stream;
use objects::number::Number;
use objects::operators::binary_operator::BinaryOperator;
use objects::traits::misc::{TryFrom, ToRc};
use objects::rc_object::RcObject;

#[derive(PartialEq, Debug, Clone)]
enum Token {
   Number(String),
   Operator(String),
   LParen(char),
   RParen(char),
   Function(String),
   Unknown(char),
}
fn next_number(stream: &mut Stream) -> Token {
   let mut acc = String::new();
   while let Some(c) = stream.peek() {
      match c {
         c @ _ if c.is_digit(10) => acc.push(stream.take(c)),
         _ => break
      };
   }
   Token::Number(acc)
}

fn next_token(stream: &mut Stream) -> Token {
   let c = stream.peek().unwrap();
   match c {
      '(' | '[' | '{' => Token::LParen(stream.take(c)),
      ')' | ']' | '}' => Token::RParen(stream.take(c)),
      _ if c.is_digit(10) => next_number(stream),
      _ => Token::Unknown(stream.take(c))
   }
}

fn get_lparen(rparen: char) -> char {
   match rparen {
      ')' => '(',
      ']' => '[',
      '}' => '{',
      _ => unreachable!("bad rparen: {:?}", rparen)
   }
}

/* based off shunting-yard algorithm (And the explanation on wikipedia) */
pub fn exec_frame(frame: &mut Frame){
   let mut stack = vec![];
   let mut opers = vec![];
   let ref mut stream = frame.stream;
   while !stream.is_empty() {
      use self::Token::*;
      match next_token(stream) {
         Number(num) => stack.push(Number(num)),
         Operator(oper) => opers.push(Operator(oper)),
         LParen(lparen) => opers.push(LParen(lparen)),
         RParen(rparen) => {
            let matching_lparen = LParen(get_lparen(rparen));
            loop {
               match opers.pop() {
                  Some(oper) => 
                     if oper == matching_lparen { break }
                     else { stack.push(oper); },
                  None => exception!(SYNTAX; "unmatched paren `{:?}`", rparen)
               }
            }
            if let Some(oper) = opers.last() {
               match oper.clone() {
                  Function(func) => stack.push(Function(func)),
                  _ => {}
               }
            }
         },
         Unknown(num) => exception!(SYNTAX; "unexpected character: {:?}", stream.peek().unwrap()),
         _ => todo!("thisfunction")
      }
   }
   println!("stack: {:?}", stack);
   println!("opers: {:?}", opers);
   // while let Some(token) =3
   
}



















