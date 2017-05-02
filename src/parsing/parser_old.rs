use parsing::stream::Stream;
use parsing::frame::Frame;
use objects::number::Number;
use objects::operators::binary_operator::BinaryOperator;
use traits::misc::{TryFrom, ToRc};
use objects::rc_object::RcObject;

enum Token {
   LParen(char),
   RParen(char),
   Symbol(String), // ie variable name
   Number(String),
   Operator(String),
}

fn next_token(stream: &mut Stream) -> Option<Token> {
   macro_rules! is_digit {
      ($chr:expr) => ( $chr.is_digit(10) )
   }
   macro_rules! is_alphabetic {
      ($chr:expr) => ( $chr.is_alphabetic() || $chr == '_' )
   }
   macro_rules! is_alphanumeric {
      ($chr:expr) => ( is_alphabetic!($chr) || is_digit!($chr) )
   }
   macro_rules! is_whitespace {
      ($chr:expr) => ( $chr.is_whitespace() )
   }

   /* trim whitespace */
   while let Some(ref mut wrapper) = stream.peek() {
      if !is_whitespace!(wrapper.chr) { break }
      wrapper.take();
   }

   enum TokenType{ Symbol, Number, Operator }

   let mut acc = String::new();

   let token_type = 
      match stream.peek() {
         None => return None,
         Some(ref mut wrapper) => 
            match wrapper.chr {
               /* preemptively return for parens */
               '(' | '[' | '{'  => return Some(Token::LParen(wrapper.take())),
               ')' | ']' | '}'  => return Some(Token::RParen(wrapper.take())),
               /* otherwise, check if alphanumeric */
               chr @ _ => {
                  acc.push(wrapper.take());
                  if is_alphabetic!(chr) { TokenType::Symbol } 
                  else if is_digit!(chr) { TokenType::Number }
                  else { TokenType::Operator }
               }
            }
      };

   while let Some(ref mut wrapper) = stream.peek() {
      match token_type {
         TokenType::Symbol if !is_alphanumeric!(wrapper.chr) => break,
         TokenType::Number if !is_digit!(wrapper.chr) => break,
         TokenType::Operator if is_alphanumeric!(wrapper.chr) || is_whitespace!(wrapper.chr) => break,
         _ => acc.push(wrapper.take())
      }
   };
   Some(match token_type {
      TokenType::Symbol => Token::Symbol(acc),
      TokenType::Number => Token::Number(acc),
      TokenType::Operator => Token::Operator(acc),
   })
}

/* based off shunting-yard algorithm (And the explanation on wikipedia) */
pub fn exec_frame(frame: &mut Frame){
   let mut output_stack = vec![]
   let mut oper_stack = vec![];
   while let Some(token) = next_token(frame.stream) {
      match token {
         Token::LParen(rparen) => todo!("lparen"),
         Token::RParen(lparen) => todo!("rparen"),
         Token::Symbol(sym) => todo!("sym"),
         Token::Number(num) => 
            output_stack.push(Number::try_from(&num).expect("cant unwrap number!").to_rc()),
         Token::Operator(oper) =>
            oper_stack.push(BinaryOperator::try_from(&oper).expect("can't unwrap oper!").to_rc())
      };
   }
   
}



















