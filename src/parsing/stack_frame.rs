use parsing::stream::Stream;
use std::collections::HashMap;
use objects::object::{RcObject, RcObjWrapper};
pub type StackType = Vec<RcObject>;
pub type LocalsType = HashMap<RcObjWrapper, RcObject>;

#[derive(Debug)]
pub struct StackFrame<'a> {
   stream: Stream,
   parent: Option<&'a StackFrame<'a>>,
   stack: StackType,
   locals: LocalsType,
}

impl <'a> StackFrame<'a> {
   fn _new(stream: Stream,
           stack: StackType,
           locals: LocalsType,
           parent: Option<&'a StackFrame<'a>>) -> StackFrame<'a> {
      StackFrame{ stream: stream, stack: stack, locals: locals, parent: parent }
   }

   pub fn new(inp: &str, parent: &'a StackFrame<'a>) -> StackFrame<'a> {
      StackFrame::_new(Stream::from(inp), StackType::new(), LocalsType::new(), Some(parent))
   }

   pub fn new_root(inp: &str) -> StackFrame<'a> {
      StackFrame::_new(Stream::from(inp), StackType::new(), LocalsType::new(), None)
   }
}