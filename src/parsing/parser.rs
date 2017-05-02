use std::rc::Rc;
use parsing::frame::Frame;
use parsing::stream::Stream;
use objects::number::Number;
use objects::identifier::Identifier;
use objects::operators::binary_operator::BinaryOperator;
use traits::misc::{TryFrom, ToRc};
use objects::object::RcObject;

fn next_token(frame: &mut Frame) -> Option<String> {
   let mut acc = String::new();
   let mut acc_isalphanum = false;
   while !frame.stream.borrow().is_empty() {
      let c = frame.stream.borrow_mut().next().expect("we just checked for this");
      match c {
         '(' => 
            if let Some(obj) = frame.fork().exec().pop() {
               frame.push(obj);
               /* what todo here */
               return None
            },
         ')' => break,
         _ if c.is_whitespace() && acc.is_empty() => {},
         _ if c.is_whitespace()                   => break,
         _ if acc.is_empty() => { acc_isalphanum = is_char!(alphanumeric; c); acc.push(c) },
         _ if acc_isalphanum != is_char!(alphanumeric; c) => { frame.stream.borrow_mut().feed(c); break },
         _ if acc_isalphanum == is_char!(alphanumeric; c) => acc.push(c),
         _ => exception!(SYNTAX; "Unexpected character: {:?}", c)
      }
   } /* end while */
   if acc.is_empty() {
      None
   } else {
      Some(acc)
   }
}

fn try_obj_from(token: &str) -> Option<RcObject> {
   if let Some(ident) = Identifier::try_from(&token) {
      Some(ident.to_rc())
   } else if let Some(num) = Number::try_from(&token) {
      Some(num.to_rc())
   } else {
      None
   }
}
fn assign(key: Rc<Identifier>, frame: &mut Frame) {
   let next_token = 
      if let Some(next_token) = next_token(frame) {
         next_token 
      } else {
         exception!(ASSIGNMENT; "Can't find rhs of equals sign");
      };
   frame.assign(key, 
      if let Some(next_obj) = try_obj_from(&next_token) {
         next_obj
      } else {
         exception!(ASSIGNMENT; "can't turn rhs of equals sign into an object!")
      });
}
fn process_token(token: String, oper_stack: &mut Vec<BinaryOperator>, frame: &mut Frame) {
   macro_rules! retrieve {
      ($obj:ident) => {
         if let Ok(val) = frame.retrieve($obj.clone()) {
            val
         } else {
            exception!(RETRIEVAL; "can't retrieve key of {:?}", $obj)
         }
      }
   }
   if token == ";" {
      frame.pop(); // and do nothing
   } else if token == "," {
      // do nothing
   } else if try_handle_control_function(&token, frame) {
      // do nothing, was already handled
   } else if let Some(obj) = try_obj_from(&token) {
      if is_a!(obj, identifier) {
         match next_token(frame) {
            Some(next_token) => 
               if next_token == "=" {
                  assign(cast_as!(obj, Identifier), frame)
               } else {
                  let to_push = retrieve!(obj);
                  frame.push(to_push);
                  process_token(next_token, oper_stack, frame);
               },
            None => 
               {
                  let to_push = retrieve!(obj);
                  frame.push(to_push);
               },
         }
      } else {
         frame.push(obj);
      };
   } else if let Some(oper) = BinaryOperator::try_from(&token) {
      while let Some(oper2) = oper_stack.pop() {
         if oper2.should_exec(&oper) { oper2.exec(frame); }
         else { oper_stack.push(oper2); break }
      }
      oper_stack.push(oper);
   } else {
      exception!(SYNTAX; "bad token: `{}`", token)
   }
}
pub fn exec_frame(frame: &mut Frame){
   let mut oper_stack: Vec<BinaryOperator> = vec![];
   while let Some(token) = next_token(frame) {
      process_token(token, &mut oper_stack, frame);
   }

   while let Some(oper) = oper_stack.pop() {
      oper.exec(frame);
   }
}



fn if_fn(frame: &mut Frame) {

}

fn try_handle_control_function(func: &str, frame: &mut Frame) -> bool {
   match func {
      "if" => if_fn(frame),
      _ => return false
   };
   true
}











