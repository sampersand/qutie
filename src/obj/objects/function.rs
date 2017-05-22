use obj::objects::object::{Object, ObjType};
use obj::objects::block::Block;
use obj::objects::null::Null;
use obj::result::ObjResult;

use parsing::identifier::Identifier;
use parsing::frame::Frame;
use parsing::token::Token;
use parsing::expression::Expression;
use std::rc::Rc;

pub struct Function {
   file: String, /* todo: update this */
   line: usize,  /* and this */
   args: Vec<Identifier>,
   body: Block,
}

impl Function {
   pub fn new(file: String, line: usize, args: Vec<Identifier>, body: Block) -> Function {
      Function{ file: file, line: line, args: args, body: body }
   }
   pub fn to_string(&self) -> String {
      concat_all!("<", self.file, ">")
   }
}
use obj::traits::misc::QtCall;
impl QtCall for Function {
   fn qt_call(&self, args: Expression, frame: &mut Frame) -> ObjResult {
      /* this is kinda hacky way to do things */
      let orig_length = frame.stack_len();
      args.exec(frame);
      let mut self_args = self.args.clone();
      let mut acc = vec![];
      while orig_length < frame.stack_len()  {
         acc.insert(0, frame.pop().unwrap());
      }
      let mut new_frame = frame.spawn_child();
      while !acc.is_empty() {
         new_frame.set(self_args.pop().unwrap(), acc.pop().unwrap());
      }
      if let Some(ret) = self.body.clone().exec(&mut new_frame) {
         Ok(ret)
      } else {
         Ok(Null::get().to_rc())
      }
   }
   fn is_callable(&self) -> bool {
      true
   }
}

use std;
impl_defaults!(Debug; Function, "F");
impl_defaults!(Display; to_string; Function);


use obj::result::BoolResult;

impl_defaults!(ToRc; Function);
impl_defaults!(Object; Function);

impl_traits!(conversion=ToBoolean, Function);
impl_traits!(conversion=ToText, Function);

impl_traits!(data=GetItem, Function);
impl_traits!(data=SetItem, Function);
impl_traits!(data=DelItem, Function);

impl_traits!(operators=QtAdd, Function);
impl_traits!(operators=QtSub, Function);
impl_traits!(operators=QtMul, Function);
impl_traits!(operators=QtDiv, Function);
impl_traits!(operators=QtMod, Function);
impl_traits!(operators=QtPow, Function);
impl_traits!(operators=QtEql, Function);
impl_traits!(operators=QtNeq, Function);
impl_traits!(operators=QtLth, Function);
impl_traits!(operators=QtGth, Function);
impl_traits!(operators=QtLeq, Function);
impl_traits!(operators=QtGeq, Function);



























