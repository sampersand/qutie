use parsing::operator::Operator;
use obj::objects::{block, text};
use parsing::identifier;
use parsing::expression::Expression;
use std;

#[derive(Debug, Clone, Copy)]
pub enum Assignments {
   EqualsSign
}

impl From<char> for Assignments {
   fn from(inp: char) -> Assignments {
      match inp {
         '=' => Assignments::EqualsSign,
         _ => unreachable!("Bad assignment char: {:?}", inp)
      }
   }
}
impl <'a> From<&'a Assignments> for char {
   fn from(assign: &'a Assignments) -> char {
      match *assign {
         Assignments::EqualsSign => '=',
      }
   }
}

impl_defaults!(to_string; char; Assignments);
impl_defaults!(Display; to_string; Assignments);

/************/

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Separator {
   Comma,
   Colon
}

impl From<char> for Separator {
   fn from(inp: char) -> Separator {
      match inp {
         ':' => Separator::Colon,
         ',' => Separator::Comma,
         _ => unreachable!("Bad assignment char: {:?}", inp)
      }
   }
}
impl <'a> From<&'a Separator> for char {
   fn from(assign: &'a Separator) -> char {
      match *assign {
         Separator::Comma => ',',
         Separator::Colon => ':',
      }
   }
}

impl_defaults!(to_string; char; Separator);
impl_defaults!(Display; to_string; Separator);

/*****************/

#[derive(Clone)]
pub enum Token {
   Identifier(identifier::Identifier),
   Assignment(Assignments),
   Separator(Separator), // , and :
   LineTerminator, // ; 
   Number(String),
   Operator(Operator),
   Unknown(char),
   Text(text::Quote, String),
   Block((block::LParen, block::RParen), Vec<Expression>),
   RParen(block::RParen),
}

impl std::fmt::Debug for Token {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      use self::Token::*;
      match self {
         &Identifier(ref s) => write!(f, "I({})", s),
         &Assignment(ref s) => write!(f, "A({})", s),
         &LineTerminator => write!(f, "Endl(;)"),
         &Separator(ref s) => write!(f, "Sep({})", s),
         &Number(ref s) => write!(f, "N({})", s),
         &Operator(ref s) => write!(f, "O({})", s),
         &Unknown(ref s) => write!(f, "U({})", s),
         &Text(ref q, ref s) => write!(f, "T({}{}{})", q, s, q),
         &Block((ref l, ref r), ref b) => write!(f, "B{}{:?}{}", l, b, r),
         &RParen(ref c) => panic!("RParen shouldn't be printed out!")
      }
   }
}

impl std::fmt::Display for Token {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      use self::Token::*;
      match self {
         &Identifier(ref s) => write!(f, "I({})", s),
         &Assignment(ref s) => write!(f, "A({})", s),
         &LineTerminator => write!(f, "Endl(;)"),
         &Separator(ref s) => write!(f, "Sep({})", s),
         &Number(ref s) => write!(f, "N({})", s),
         &Operator(ref s) => write!(f, "O({})", s),
         &Unknown(ref s) => write!(f, "U({})", s),
         &Text(ref q, ref s) => write!(f, "T({}{}{})", q, s, q),
         &Block((ref l, ref r), ref b) => write!(f, "B{}{:?}{}", l, b, r),
         &RParen(ref c) => panic!("RParen shouldn't be printed out!")
      }
   
   }
}


