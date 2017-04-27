use parsing::stream::Stream;
use parsing::frame::Frame;
use objects::number::Number;
use objects::operators::binary_operator::BinaryOperator;
use objects::traits::misc::{TryFrom, ToRc};
use objects::rc_object::RcObject;

enum Token {
   LParen(char),
   RParen(char),
   Symbol(String), // ie variable name
   Constant(String), // ie string, number, etc /* find better name for this */
   Operator(String),
   Unknown(char),
}

fn next_text(stream: &mut Stream) -> Token {
   todo!("next_text")
}
fn next_symbol(stream: &mut Stream) -> Token {
   todo!("next_symbol")
}
fn next_number(stream: &mut Stream) -> Token {
   todo!("next_number")
}
fn next_operator(stream: &mut Stream) -> Token {
   todo!("next_operator")
}

fn is_lparen(chr: char) -> bool {
   match chr {
      '(' | '[' | '{' => true,
      _ => false
   }
}
fn is_rparen(chr: char) -> bool {
   match chr {
      ')' | ']' | '}' => true,
      _ => false
   }
}
fn is_quote(chr: char)  -> bool {
   match chr {
      '\\' | '"' | '`' => true,
      _ => false
   }
}
fn is_symbol_start(chr: char) -> bool { chr.is_alphabetic() }
fn is_number_start(chr: char) -> bool { chr.is_digit(10) }
fn is_operator_start(chr: char) -> bool { false }

fn next_token(stream: &mut Stream) -> Option<Token> {
   /* trim whitespace */
   while let Some(ref mut chr) = stream.peek() {
      if !chr.is_whitespace() { break }
      chr.take();
   }
   /* check if empty and otherwise assign chr */
   let chr = 
      if let Some(chr) = stream.peek() {
         chr.chr.clone()
      } else {
         return None
      };

   Some(
      match chr {
         _ if is_lparen(chr)         => Token::LParen(chr),
         _ if is_rparen(chr)         => Token::RParen(chr),
         _ if is_quote(chr)          => next_text(stream),
         _ if is_symbol_start(chr)   => next_symbol(stream),
         _ if is_number_start(chr)   => next_number(stream),
         _ if is_operator_start(chr) => next_operator(stream),
         _ => Unknown(chr)
      }
   )
}

pub fn exec_frame(frame: &mut Frame){
   // let mut token_stack = vec![];
   // let mut oper_stack = vec![];
   while let Some(token) = next_token(frame.stream) {
      match token {
         Token::LParen(rparen) => todo!("lparen"),
         Token::RParen(lparen) => todo!("rparen"),
         Token::Symbol(sym) => todo!("sym"),
         Token::Constant(cnst) => todo!("Constant"),
         Token::Operator(oper) => todo!("oper"),
         Token::Unknown(err) => exception!(SYNTAX; "Unexpected token: `{}`", err),
      };
   }
   
}



















