use parsing::token::Token;
use std::rc::Rc;
use obj::objects::object::Object;
use obj::result::{ObjResult, ObjError};

#[derive(Debug)]
pub enum Operator {
   Add, Sub, Mul, Div, Mod, Pow
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
      }
   }
   pub fn should_exec(&self, other: &Operator) -> bool {
      todo!("should_exec for operator");
      false
   }
   fn operands(&self) -> u8 {
      2
   }

   fn exec_binary(&self, stack: &mut Vec<Rc<Object>>) -> ObjResult {
      let rhs = stack.pop().expect("Stack has too few operands!");
      let lhs = stack.pop().expect("Stack has too few operands!");
      use self::Operator::*;
      use obj::traits::operators::*;
      match *self {
         Add => lhs.qt_add(rhs),
         Sub => lhs.qt_sub(rhs),
         Mul => lhs.qt_mul(rhs),
         Div => lhs.qt_div(rhs),
         Mod => lhs.qt_mod(rhs),
         Pow => lhs.qt_pow(rhs),
      }
   }

   pub fn exec(&self, stack: &mut Vec<Rc<Object>>) {
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
         _ => unreachable!("bad operator: {:?}", inp)
      }
   }
}

use std;
impl_defaults!(DISPLAY; Operator);
impl_defaults!(TO_STRING; func=sigil; Operator);


