use std::rc::Rc;
use parsing::frame::Frame;
use parsing::stream::Stream;
use objects::number::Number;
use objects::identifier::Identifier;
use objects::operators::binary_operator::BinaryOperator;
use traits::misc::{TryFrom, ToRc};
use objects::object::RcObject;

macro_rules! expect {
   ($expr:expr; $err:expr) => (
      if let Some(obj) = $expr {
         obj
      } else {
         $err
      }
   );
   (Ok; $expr:expr; $err:expr) => (
      if let Ok(obj) = $expr {
         obj
      } else {
         $err
      }
   )
}

fn next_token(frame: &mut Frame) -> Option<String> {
   let mut acc = String::new();
   let mut acc_isalphanum = false;
   'outer_while:
   while !frame.stream.borrow().is_empty() {
      let c = frame.stream.borrow_mut().next().unwrap();
      match c {
         // '(' => 
         //    if let Some(obj) = frame.fork().exec().pop() {
         //       frame.push(obj);
         //       /* what todo here */
         //       return None
         //    },
         '{' | '(' | '[' if !acc.is_empty() => 
            {
               frame.stream.borrow_mut().feed(c);
               break;
            },
         '{' | '(' | '[' =>
            {
               let r_paren =
                  match c {
                     '(' => ')',
                     '[' => ']',
                     '{' => '}',
                     _ => unreachable!("bad paren: {:?}", c)
                  };
               acc.push(c);
               while let Some(next) = next_token(frame) {
                  acc.push_str(next.as_str());

                  if let Some(last) = next.chars().last() {
                     if last == r_paren {
                        break 'outer_while
                     }
                  } else {
                     unreachable!("an empty string was returned from next_token as some");
                  }
               }
            },
         // ')' => break,
         '}' | ')' | ']' => { 
            acc.push(c);
            // assert_eq!(acc.chars().nth(0).unwrap(),
            //            match c {
            //             ')' => '(',
            //             ']' => '[',
            //             '}' => '{',
            //             _ => unreachable!("bad paren: {:?}", c)
            //            }, "unmatched parens!");
            break
         },
         _ if c.is_whitespace() && acc.is_empty() => {},
         _ if c.is_whitespace()                   => break,
         _ if acc.is_empty() => { acc_isalphanum = is_char!(alphanumeric; c); acc.push(c) },
         _ if acc_isalphanum != is_char!(alphanumeric; c) => { frame.stream.borrow_mut().feed(c); break },
         _ if acc_isalphanum == is_char!(alphanumeric; c) => acc.push(c),
         _ => exception!(SYNTAX; frame, "Unexpected character: {:?}", c)
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

fn process_token(token: String, oper_stack: &mut Vec<BinaryOperator>, frame: &mut Frame) {
   macro_rules! retrieve {
      ($obj:ident) => {
         expect!(Ok; frame.retrieve($obj.clone()); 
                 exception!(RETRIEVAL; frame, "can't retrieve key of {:?}", $obj))
      }
   }
   if token == ";" {
      frame.pop(); // and do nothing
   } else if token == "," {
      // do nothing
   } else if token == "$" {
      panic!("found $ ({:?})", frame.pop());
   } else if try_handle_control_function(&token, frame) {
      // do nothing, was already handled
   } else if let Some(obj) = try_obj_from(&token) {
      if is_a!(obj, identifier) {
         match next_token(frame) {
            Some(next) => 
               if next == "=" {
                  let next = expect!(next_token(frame);
                                     exception!(ASSIGNMENT; frame, "Can't find rhs of equals sign"));
                  frame.assign(cast_as!(obj, Identifier),
                               expect!(try_obj_from(&next); 
                                       exception!(ASSIGNMENT; frame, "can't turn rhs of equals sign into an object!")));
               } else {
                  let to_push = retrieve!(obj);
                  frame.push(to_push);
                  process_token(next, oper_stack, frame);
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
   } else if let Some(mut new_frame) = frame.try_from(&token) {
      new_frame.exec();
   } else {
      exception!(SYNTAX; frame, "bad token: `{}`", token)
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
   let cond_token = expect!(next_token(frame); exception!(SYNTAX; frame, "can't find condition"));
   let true_token = expect!(next_token(frame); exception!(SYNTAX; frame, "can't find true branch"));
   let else_token = expect!(next_token(frame); String::new());
   let has_else = else_token == "else";
   let false_token = 
      if has_else {
         expect!(next_token(frame); exception!(SYNTAX; frame, "can't find false branch"))
      } else {
         String::new()
      };
   if !has_else {
      for c in else_token.chars().rev() {
         frame.stream.borrow_mut().feed(c);
      }
   }
   let cond = expect!(Ok; expect!(expect!(frame.try_from(&cond_token);
                                          exception!(SYNTAX; frame, "frames are expected after an if statement!")).eval();
                                 exception!(SYNTAX; frame, "condition is empty!")).to_boolean();
                         exception!(SYNTAX; frame, "can't transform condition into a boolean")).to_bool();

   if cond {
      if let Some(mut new_frame) = frame.try_from(&true_token) {
         new_frame.exec();
         return
      }
      if let Some(obj) = try_obj_from(&true_token) {
         frame.push(obj);
      }
   } else {
      if let Some(mut new_frame) = frame.try_from(&false_token) {
         new_frame.exec();
         return
      }
      if let Some(obj) = try_obj_from(&false_token) {
         frame.push(obj);
      }
   }

}

fn try_handle_control_function(func: &str, frame: &mut Frame) -> bool {
   match func {
      "if" => if_fn(frame),
      _ => return false
   };
   true
}











