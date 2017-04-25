use parsing::stream::Stream;
use parsing::stack_frame::StackFrame;

pub fn parse(inp: &str) -> StackFrame {
   let frame = StackFrame::new_root(inp);
   
   frame
}