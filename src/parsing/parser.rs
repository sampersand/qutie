use std::rc::Rc;
use parsing::frame::Frame;
use parsing::stream::Stream;
use objects::number::Number;
use objects::operators::binary_operator::BinaryOperator;
use objects::traits::misc::{TryFrom, ToRc};
use objects::rc_object::RcObject;

fn next_token(frame: &mut Frame) -> Option<String> {
   let mut acc = String::new();
   let mut is_alphanumeric = false;
   while !frame.stream.borrow().is_empty() {
      let c = frame.stream.borrow_mut().next().expect("we just checked for this");
      match c {
         '(' =>
            if let Some(obj) = frame.fork().exec().pop() {
               frame.push(obj);
               /* what todo here */
               return None
            },
         ')' => return None,
         _ if c.is_whitespace() && acc.is_empty() => {},
         _ if c.is_whitespace()                   => break,
         _ if acc.is_empty() => { is_alphanumeric = c.is_alphanumeric(); acc.push(c) },
         _ if is_alphanumeric != c.is_alphanumeric() => { frame.stream.borrow_mut().feed(c); break },
         _ if is_alphanumeric == c.is_alphanumeric() => acc.push(c),
         _ => panic!("Unexpected character: {:?}", c)
      }
   } /* end while */
   if acc.is_empty() {
      None
   } else {
      Some(acc)
   }
}

pub fn exec_frame(frame: &mut Frame){
   let mut oper_stack: Vec<BinaryOperator> = vec![];
   while let Some(token) = next_token(frame) {
      if let Some(num) = Number::try_from(&token) {
         frame.push(num.to_rc());
      } else if let Some(oper) = BinaryOperator::try_from(&token) {
         while let Some(oper2) = oper_stack.pop() {
            if oper2.should_exec(&oper) { oper2.exec(frame); }
            else { oper_stack.push(oper2); break }
         }
         oper_stack.push(oper);
      } else {
         panic!("bad token: `{}`", token)
      }
   }

   while let Some(oper) = oper_stack.pop() {
      oper.exec(frame);
   }
}



















