use obj::objects::object::{Object, ObjType};
use std::rc::Rc;

pub struct List {
   pub contents: Vec<Rc<Object>>
}

impl List {
   pub fn new(contents: Vec<Rc<Object>>) -> List {
      List { contents: contents }
   }
   pub fn to_string(&self) -> String {
      let string_iter = 
         self.contents.iter().map(|item: &Rc<Object>|
            match item.to_text() { 
               Ok(text) => text.to_string(),
               Err(err) => concat_all!("<to_string err: ", err, ">")
            }
         );
      /* TODO: This; see if there's a `join` method or something */
      let mut ret = String::from("[");
      for mut item in string_iter {
         item.push_str(", ");
         ret.push_str(item.as_str())
      }
      ret.push(']');
      ret
   }
}
use std;
impl_defaults!(Debug; List, "L");
impl_defaults!(Display; to_string; List);

use obj::result::ObjError;
use obj::objects::boolean::Boolean;
use obj::traits::conversion::{ToBoolean, ToText};
use obj::objects::text::Text;
impl ToText for List {
   fn to_text(&self) -> Result<Rc<Text>, ObjError> {
      Ok(Text::from(self.to_string()).to_rc())
   }
}

impl ToBoolean for List {
   fn to_boolean(&self) -> Result<Rc<Boolean>, ObjError> {
      Ok(Boolean::get(self.contents.is_empty()).to_rc())
   }
}

impl_defaults!(ToRc; List);
impl_defaults!(Object; List);

impl_defaults!(QtAdd; List);
impl_defaults!(QtSub; List);
impl_defaults!(QtMul; List);
impl_defaults!(QtDiv; List);
impl_defaults!(QtMod; List);
impl_defaults!(QtPow; List);
impl_defaults!(QtEql; List);
impl_defaults!(QtNeq; List);
impl_defaults!(QtLth; List);
impl_defaults!(QtGth; List);
impl_defaults!(QtLeq; List);
impl_defaults!(QtGeq; List);



























