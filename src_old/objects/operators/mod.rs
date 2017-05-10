use parsing::frame::Frame;
pub trait Operator {
   fn should_exec(&self, other: &Operator) -> bool;
   fn exec(&self, frame: &mut Frame);
   fn priority(&self) -> u8;
   fn is_left_assoc(&self) -> bool { false }
}
pub mod binary_operator;
pub mod unary_operator;