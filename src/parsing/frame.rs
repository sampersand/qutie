use parsing::stream::Stream;
use parsing::parser;
use std::collections::HashMap;
use objects::rc_object::{RcObject, RcObjWrapper};
pub type StackType = Vec<RcObject>;
pub type LocalsType = HashMap<RcObjWrapper, RcObject>;

#[derive(Debug)]
pub struct Frame<'a> {
   pub stream: &'a mut Stream,
   parent: Option<&'a Frame<'a>>,
   stack: StackType,
   locals: LocalsType,
}

pub fn new_root<'a>(inp: &'a mut Stream) -> Frame<'a> {
   Frame::_new(inp, StackType::new(), LocalsType::new(), None)
}

impl <'a> Frame<'a> {
   fn _new(stream: &'a mut Stream,
           stack: StackType,
           locals: LocalsType,
           parent: Option<&'a Frame<'a>>) -> Frame<'a> {
      Frame{ stream: stream, stack: stack, locals: locals, parent: parent }
   }
   pub fn new(inp: &'a mut Stream, parent: &'a Frame<'a>) -> Frame<'a> {
      Frame::_new(inp, StackType::new(), LocalsType::new(), Some(parent))
   }

   pub fn exec(&mut self) {
      parser::exec_frame(self);
   }
   pub fn push_stack(&mut self, new_obj: RcObject) {
      self.stack.push(new_obj);
   }
}
















