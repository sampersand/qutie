use obj::objects::object::{Object, ObjType};
use obj::objects::block::Block;
use parsing::identifier::Identifier;
use parsing::frame::Frame;
use parsing::token::Token;
use parsing::parser;
use std::rc::Rc;
pub struct Function {
   file: String, /* todo: update this */
   line: usize,
   args: Vec<Identifier>,
   body: Vec<Token>
}

impl Function {
   pub fn new(file: String, line: usize, args: Vec<Identifier>, body: Vec<Token>) -> Function {
      Function{ file: file, line: line, args: args, body: body }
   }
   pub fn to_string(&self) -> String {
      concat_all!("<", self.file, ">")
   }
   pub fn qt_call(&self, args: Vec<Token>, frame: &mut Frame) -> Rc<Object> {
      /* this is kinda hacky way to do things */
      let orig_length = frame.stack_len();
      parser::handle(args, frame);
      let mut self_args = self.args.clone();
      let mut i = 0;
      let mut acc = vec![];
      while orig_length < frame.stack_len()  {
         acc.insert(0, frame.pop().unwrap());
      }
      let mut new_frame = frame.spawn_child();
      while !acc.is_empty() {
         new_frame.set(self_args.pop().unwrap(), acc.pop().unwrap());
      }
      parser::handle(self.body.clone(), &mut new_frame);
      if let Some(ret) = new_frame.pop() {
         ret
      } else {
         panic!("todo: None")
      }
   }
}

use std;
impl_defaults!(Debug; Function, "F");
impl_defaults!(Display; to_string; Function);


use obj::result::BoolResult;

impl_defaults!(ToRc; Function);
impl_defaults!(Object; Function);
impl_defaults!(ToBoolean; Function);
impl_defaults!(ToText; Function);

impl_defaults!(QtAdd; Function);
impl_defaults!(QtSub; Function);
impl_defaults!(QtMul; Function);
impl_defaults!(QtDiv; Function);
impl_defaults!(QtMod; Function);
impl_defaults!(QtPow; Function);
impl_defaults!(QtEql; Function);
impl_defaults!(QtNeq; Function);
impl_defaults!(QtLth; Function);
impl_defaults!(QtGth; Function);
impl_defaults!(QtLeq; Function);
impl_defaults!(QtGeq; Function);



























