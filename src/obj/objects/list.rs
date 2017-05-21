use obj::objects::object::{Object, ObjType};
use std::rc::Rc;
use parsing::frame::Frame;

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

use obj::result::{ObjError, ObjResult};
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

// use obj::traits::data::{GetItem, SetItem, DelItem};
use obj::traits::data::{GetItem};
use obj::objects::number::Number;
impl GetItem for List {
   fn get_item(&self, item: Rc<Object>, frame: &mut Frame) -> ObjResult {
      match item.obj_type() {
         ObjType::Number => 
            {
               let num = cast_as!(&item, Number).num;
               if num < 0 && self.contents.len() as i32 + num < 0 {
                  return Err(ObjError::InvalidKey(item))
               }
               let num = 
                  if num < 0 {
                     self.contents.len() - (-num) as usize
                  } else {
                     num as usize
                  };
               if let Some(res) = self.contents.get(num) {
                  Ok(res.clone())
               } else {
                  Err(ObjError::InvalidKey(item))
               }
            },
         o @ _ => panic!("Idk what to do with type: {:?}", o)
      }
   }
}
impl_traits!(data=SetItem, List);
impl_traits!(data=DelItem, List);

impl_defaults!(ToRc; List);
impl_defaults!(Object; List);



impl_traits!(oper=QtAdd, List);
impl_traits!(oper=QtSub, List);
impl_traits!(oper=QtMul, List);
impl_traits!(oper=QtDiv, List);
impl_traits!(oper=QtMod, List);
impl_traits!(oper=QtPow, List);
impl_traits!(oper=QtEql, List);
impl_traits!(oper=QtNeq, List);
impl_traits!(oper=QtLth, List);
impl_traits!(oper=QtGth, List);
impl_traits!(oper=QtLeq, List);
impl_traits!(oper=QtGeq, List);



























