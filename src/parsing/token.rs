use parsing::operator::Operator;
use obj::objects::{block, text};
use parsing::identifier;
use parsing::Expression;
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

/*****************/

#[derive(Clone)]
pub enum Token {
   Identifier(identifier::Identifier),
   Path(String), /* todo: this */
   Assignment(Assignments),
   Separator, /* ie , */
   LineTerminator, /* ie ; */
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
         &Path(ref s) => write!(f, "P({})", s),
         &Assignment(ref s) => write!(f, "A({})", s),
         &LineTerminator => write!(f, "Endl(;)"),
         &Separator => write!(f, "Sep(,)"),
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
         &Path(ref s) => write!(f, "P({})", s),
         &Assignment(ref s) => write!(f, "A({})", s),
         &LineTerminator => write!(f, "Endl(;)"),
         &Separator => write!(f, "Sep(,)"),
         &Number(ref s) => write!(f, "N({})", s),
         &Operator(ref s) => write!(f, "O({})", s),
         &Unknown(ref s) => write!(f, "U({})", s),
         &Text(ref q, ref s) => write!(f, "T({}{}{})", q, s, q),
         &Block((ref l, ref r), ref b) => write!(f, "B{}{:?}{}", l, b, r),
         &RParen(ref c) => panic!("RParen shouldn't be printed out!")
      }
   
   }
}


