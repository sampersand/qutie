use parsing::identifier::Identifier;
use obj::objects::object::Object;
use obj::objects::{boolean, null};
use obj::traits::ToRc;
use std::rc::Rc;

pub fn get_constant(inp: &Identifier) -> Option<Rc<Object>> {
   match &**inp {
      "true" => Some(boolean::TRUE.to_rc()),
      "false" => Some(boolean::FALSE.to_rc()),
      "null" => Some(null::NULL.to_rc()),
      "nil" => Some(null::NULL.to_rc()),
      "none" => Some(null::NULL.to_rc()),
      _ => None,
   }
}

pub fn get_function(inp: &Identifier) -> Option<Rc<Object>> {
   match &**inp {
      "disp" => Some(DISP.to_rc()),
      _ => None,
   }
}


use obj::objects::builtin_function::BuiltinFunction;
use obj::objects::null::Null;
use obj::result::ObjResult;
use parsing::frame::Frame;

fn disp_fn(args: Vec<Rc<Object>>, frame: &mut Frame) -> ObjResult {
   let sep_const: Identifier = Identifier{ id: String::from("sep") };
   let end_const: Identifier = Identifier{ id: String::from("end") };
   let default_sep: String = String::from(" ");
   let default_end: String = String::from("\n");

   let mut text_args: Vec<String> = vec![];
   for arg in args {
      text_args.push(arg.as_text_string());
   }
   let sep = 
      if let Some(sep) = frame.get(&sep_const) {
         sep.as_text_string()
      } else {
         default_sep
      };
   let end = 
      if let Some(end) = frame.get(&end_const) {
         end.as_text_string()
      } else {
         default_end
      };

   print!("{}{}", text_args.join(sep.as_str()), end);
   Ok(Null::get().to_rc())
}

const DISP: BuiltinFunction = BuiltinFunction{ name: "disp", func: disp_fn};
















