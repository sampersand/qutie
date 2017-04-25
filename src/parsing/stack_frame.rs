use parsing::stream::Stream;
use std::collections::HashMap;
use objects::object::{RcObj, RcObjWrapper};
pub type LocalsType = HashMap<RcObjWrapper, RcObj>;

#[derive(Debug)]
pub struct StackFrame<'a> {
   stream: Stream,
   parent: Option<&'a StackFrame<'a>>,
   locals: LocalsType,
}

impl <'a> StackFrame<'a> {
   pub fn new(inp: &str, parent: &'a StackFrame<'a>) -> StackFrame<'a> {
      StackFrame{ stream: Stream::from(inp),
                  parent: Some(parent),
                  locals: LocalsType::new()
                }
   }

   pub fn new_root(inp: &str) -> StackFrame<'a> {
      StackFrame{ stream: Stream::from(inp),
                  parent: None,
                  locals: LocalsType::new()
                }
   }
}