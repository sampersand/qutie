use parsing::frame::Frame;
pub fn try_handle_control_function(func: &str, frame: &mut Frame) -> bool {
   match func {
      _ => return false
   }
   true
}