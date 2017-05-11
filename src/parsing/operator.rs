use parsing::token::Token;
use parsing::frame::Frame;
use std::rc::Rc;
use obj::objects::object::Object;
use obj::result::{ObjResult, ObjError};

#[derive(Debug)]
pub enum Operator {
   Add, Sub, Mul, Div, Mod, Pow,
   Eql, Neq, Gth, Lth, Geq, Leq,
}

impl Operator {
   pub fn sigil(&self) -> &'static str {
      use self::Operator::*;
      match *self {
         Add => "+",
         Sub => "-",
         Mul => "*",
         Div => "/",
         Mod => "%",
         Pow => "**",
         Eql => "==",
         Neq => "!=",
         Gth => ">",
         Lth => "<",
         Geq => ">=",
         Leq => "<=",
      }
   }
   fn priority(&self) -> u8 {
      use self::Operator::*;
      match *self {
         Eql | Neq | Gth | Lth | Geq | Leq => 4,
         Add | Sub => 3,
         Mul | Div | Mod => 2,
         Pow => 1,
      }
   }
   pub fn should_exec(&self, other: &Operator) -> bool {
      /* ignoring left/right associative for now */
      self.priority() < other.priority()
   }
   fn operands(&self) -> u8 {
      2
   }

   fn exec_binary(&self, stack: &mut Frame) -> ObjResult {
      let rhs = stack.pop().expect("Stack has too few operands!");
      let lhs = stack.pop().expect("Stack has too few operands!");
      use self::Operator::*;
      use obj::traits::operators::*;
      match *self {
         Add => lhs.qt_add(&rhs),
         Sub => lhs.qt_sub(&rhs),
         Mul => lhs.qt_mul(&rhs),
         Div => lhs.qt_div(&rhs),
         Mod => lhs.qt_mod(&rhs),
         Pow => lhs.qt_pow(&rhs),
         Eql => match lhs.qt_eql(&rhs) { Ok(o) => Ok(o), Err(e) => Err(e) }, // so it casts
         Neq => match lhs.qt_neq(&rhs) { Ok(o) => Ok(o), Err(e) => Err(e) }, // so it casts
         Gth => match lhs.qt_gth(&rhs) { Ok(o) => Ok(o), Err(e) => Err(e) }, // so it casts
         Lth => match lhs.qt_lth(&rhs) { Ok(o) => Ok(o), Err(e) => Err(e) }, // so it casts
         Geq => match lhs.qt_geq(&rhs) { Ok(o) => Ok(o), Err(e) => Err(e) }, // so it casts
         Leq => match lhs.qt_leq(&rhs) { Ok(o) => Ok(o), Err(e) => Err(e) }, // so it casts
      }
   }

   pub fn exec(&self, stack: &mut Frame) {
      if self.operands() == 2 {
         match self.exec_binary(stack) {
            Ok(obj) => stack.push(obj),
            Err(err) => panic!("err with operator `{}`: {:?}", self, err)
         }
      } else {
         todo!("other amounts of operands for exec");
      }
   }
}

impl <'a> From<&'a str> for Operator {
   fn from(inp: &'a str) -> Operator {
      use self::Operator::*;
      match inp {
         "+" => Add,
         "-" => Sub,
         "*" => Mul,
         "/" => Div,
         "%" => Mod,
         "**" => Pow,
         "==" => Eql,
         "!=" => Neq,
         ">"  => Gth,
         "<"  => Lth,
         ">=" => Geq,
         "<=" => Leq,
         _ => unreachable!("bad operator: {:?}", inp)
      }
   }
}

use std;
impl_defaults!(Display; to_string; Operator);
impl_defaults!(to_string; func=sigil; Operator);


