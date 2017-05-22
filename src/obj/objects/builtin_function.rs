use obj::objects::object::{Object, ObjType};
use obj::objects::block::Block;
use obj::objects::null::Null;
use obj::result::ObjResult;

use parsing::identifier::Identifier;
use parsing::frame::Frame;
use parsing::token::Token;
use parsing::expression::Expression;
use std::rc::Rc;

pub struct BuiltinFunction {
   pub name: &'static str,
   pub func: fn(Vec<Rc<Object>>, &mut Frame) -> ObjResult,
}

impl BuiltinFunction {
   pub fn new(name: &'static str,
              func: fn(Vec<Rc<Object>>, &mut Frame) -> ObjResult ) -> BuiltinFunction {
      BuiltinFunction{ name: name, func: func }
   }
   pub fn to_string(&self) -> String {
      concat_all!("<builtin function '", self.name, "'>")
   }
}
use obj::traits::misc::QtCall;
impl QtCall for BuiltinFunction {
   fn qt_call(&self, args: Expression, frame: &mut Frame) -> ObjResult {
      /* this is kinda hacky way to do things */
      let orig_length = frame.stack_len();
      args.exec(frame);
      let mut acc = vec![];
      while orig_length < frame.stack_len()  {
         acc.insert(0, frame.pop().unwrap());
      }
      let mut new_frame = frame.spawn_child();
      // for i in 0..(self.args.len()-1) {
      //    new_frame.set(self.args[i].clone(), acc[i].clone());
      // }
      (self.func)(acc, &mut new_frame)
   }
   fn is_callable(&self) -> bool { true }
}

use std;
impl_defaults!(Debug; BuiltinFunction, "F");
impl_defaults!(Display; to_string; BuiltinFunction);


use obj::result::BoolResult;

impl_defaults!(ToRc; BuiltinFunction);
impl_defaults!(Object; BuiltinFunction);

impl_traits!(conversion=ToBoolean, BuiltinFunction);
impl_traits!(conversion=ToText, BuiltinFunction);

impl_traits!(data=GetItem, BuiltinFunction);
impl_traits!(data=SetItem, BuiltinFunction);
impl_traits!(data=DelItem, BuiltinFunction);

impl_traits!(operators=QtAdd, BuiltinFunction);
impl_traits!(operators=QtSub, BuiltinFunction);
impl_traits!(operators=QtMul, BuiltinFunction);
impl_traits!(operators=QtDiv, BuiltinFunction);
impl_traits!(operators=QtMod, BuiltinFunction);
impl_traits!(operators=QtPow, BuiltinFunction);
impl_traits!(operators=QtEql, BuiltinFunction);
impl_traits!(operators=QtNeq, BuiltinFunction);
impl_traits!(operators=QtLth, BuiltinFunction);
impl_traits!(operators=QtGth, BuiltinFunction);
impl_traits!(operators=QtLeq, BuiltinFunction);
impl_traits!(operators=QtGeq, BuiltinFunction);



























