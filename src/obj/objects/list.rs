use obj::objects::object::{Object, ObjType};
use std::rc::Rc;
use parsing::frame::Frame;
use obj::objects::null::Null;
use obj::traits::Castable;

pub struct List {
   contents: Vec<Rc<Object>>
}

impl List {
   pub fn new(contents: Vec<Rc<Object>>) -> List {
      List { contents: contents }
   }
   pub fn to_string(&self) -> String {
      let mut text_contents: Vec<String> = vec![];
      for item in &self.contents {
         text_contents.push(item.as_text_string());
      }
      let mut ret = String::from("[");
      ret.push_str(&text_contents.join(", "));
      ret.push(']');
      ret
   } 
   pub fn get(&self, pos: usize) -> Option<&Rc<Object>>  {
      self.contents.get(pos)
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

use obj::traits::data::{GetItem, SetItem};
use obj::objects::number::Number;
impl GetItem for List {
   fn get_item(&self, item: Rc<Object>, frame: &mut Frame) -> ObjResult {
      if let Some(item_num) = item.cast() {
         let num = (item_num as Rc<Number>).num;
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
      } else {
         panic!("Idk what to do with type: {:?}", item.obj_type())
      }
   }
}
impl SetItem for List {
   fn set_item(&mut self, item: Rc<Object>, val: Rc<Object>, frame: &mut Frame) -> Result<(), ObjError> {
      if let Some(item_num) = item.cast() {
         let num = (item_num as Rc<Number>).num;
         if num < 0 && self.contents.len() as i32 + num < 0 {
            return Err(ObjError::InvalidKey(item))
         }
         let num = 
            if num < 0 {
               self.contents.len() - (-num) as usize
            } else {
               num as usize
            };
         while self.contents.len() <= num {
            self.contents.push(Null::get().to_rc())
         }
         self.contents[num] = val;
         Ok(())
      } else {
         panic!("Idk what to do with type: {:?}", item.obj_type())
      }
   }
}
impl_traits!(data=DelItem, List);

impl_defaults!(ToRc; List);
impl_defaults!(Object; List);



impl_traits!(operators=QtAdd, List);
impl_traits!(operators=QtSub, List);
impl_traits!(operators=QtMul, List);
impl_traits!(operators=QtDiv, List);
impl_traits!(operators=QtMod, List);
impl_traits!(operators=QtPow, List);
impl_traits!(operators=QtEql, List);
impl_traits!(operators=QtNeq, List);
impl_traits!(operators=QtLth, List);
impl_traits!(operators=QtGth, List);
impl_traits!(operators=QtLeq, List);
impl_traits!(operators=QtGeq, List);
impl_traits!(misc=QtCall, List);



























