use parsing::operator::Operator;
use obj::objects::block;
use parsing::identifier;
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

#[derive(Debug, Clone, Copy)]
pub enum LineTerminators {
   Semicolon
   /* this allows me to add in things like \n ending a line in the future */
}

impl From<char> for LineTerminators {
   fn from(inp: char) -> LineTerminators {
      match inp {
         ';' => LineTerminators::Semicolon,
         _ => unreachable!("Bad line terminator char: {:?}", inp)
      }
   }
}
impl <'a> From<&'a LineTerminators> for char {
   fn from(assign: &'a LineTerminators) -> char {
      match *assign {
         LineTerminators::Semicolon => ';',
      }
   }
}

impl_defaults!(to_string; char; LineTerminators);
impl_defaults!(Display; to_string; LineTerminators);

/*****************/

#[derive(Clone)]
pub enum Token {
   Identifier(identifier::Identifier),
   Path(String), /* todo: this */
   Assignment(Assignments),
   LineTerminator(LineTerminators),
   Number(String),
   Operator(Operator),
   Unknown(char),
   Text(char, String),
   Block((block::LParen, block::RParen), Vec<Token>),
   RParen(block::RParen)
}

impl std::fmt::Debug for Token {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      use self::Token::*;
      match self {
         &Identifier(ref s) => write!(f, "I({})", s),
         &Path(ref s) => write!(f, "P({})", s),
         &Assignment(ref s) => write!(f, "A({})", s),
         &LineTerminator(ref s) => write!(f, "L({})", s),
         &Number(ref s) => write!(f, "N({})", s),
         &Operator(ref s) => write!(f, "O({})", s),
         &Unknown(ref s) => write!(f, "U({})", s),
         &Text(ref q, ref s) => write!(f, "T({}{}{})", q, s, q),
         &Block((ref l, ref r), ref b) => write!(f, "B{}{:?}{}", l, b, r),
         &RParen(ref c) => panic!("RParen shouldn't be printed out!")
      }
   }
}




