use obj::objects::object::{Object, ObjType};
use std::rc::Rc;
use parsing::frame::Frame;
use parsing::expression::Expression;
use obj::objects::null::Null;
use obj::traits::Castable;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
struct ObjWrapper(Rc<Object>);
impl PartialEq for ObjWrapper {
   fn eq(&self, other: &ObjWrapper) -> bool {
      (self.0)._eql( other.clone().0)
   }
}

impl Eq for ObjWrapper {}
impl Hash for ObjWrapper{
   fn hash<T: Hasher>(&self, hasher: &mut T) {
      /* todo: hash */
      hasher.write(&[1]);
   }
}



pub struct Map {
   contents: HashMap<ObjWrapper, Rc<Object>>
}

impl Map {
   pub fn from(inp: Vec<Expression>) -> Map {
      assert_eq!(inp.len(), 1, "Only one expression should be in the body of maps!");
      let inp = inp.pop().unwrap();
      use parsing::token::Separator;
      let mut map = HashMap::new();
      while !inp.is_empty() {
         let val = inp.pop().expect("Can't find a value!");
         assert_match!(inp.pop().expect("Cant find colon!"), Separator::Colon(_));
      }
      Map { contents: map }
   }

   pub fn to_string(&self) -> String {
      let mut text_contents: Vec<String> = vec![];
      for (key, val) in self.contents.iter() {
         let mut to_insert = key.0.as_text_string();
         to_insert.push_str(": ");
         to_insert.push_str(&val.as_text_string());
         text_contents.push(to_insert);
      }
      let mut ret = String::from("{");
      ret.push_str(&text_contents.join(", "));
      ret.push('}');
      ret
   } 
}

use std;
impl_defaults!(Debug; Map, "M");
impl_defaults!(Display; to_string; Map);

use obj::result::{ObjError, ObjResult};
use obj::objects::boolean::Boolean;
use obj::traits::conversion::{ToBoolean, ToText};
use obj::objects::text::Text;

impl ToText for Map {
   fn to_text(&self) -> Result<Rc<Text>, ObjError> {
      Ok(Text::from(self.to_string()).to_rc())
   }
}

impl ToBoolean for Map {
   fn to_boolean(&self) -> Result<Rc<Boolean>, ObjError> {
      Ok(Boolean::get(self.contents.is_empty()).to_rc())
   }
}

use obj::traits::data::{GetItem, SetItem};
use obj::objects::number::Number;
impl GetItem for Map {
   fn get_item(&self, item: Rc<Object>, frame: &mut Frame) -> ObjResult {
      let key = ObjWrapper(item.clone());
      if let Some(val) = self.contents.get(&key) {
         Ok(val.clone())
      } else {
         Err(ObjError::InvalidKey(item))
      }
   }
}
impl SetItem for Map {
   fn set_item(&mut self, item: Rc<Object>, val: Rc<Object>, frame: &mut Frame) -> Result<(), ObjError> {
      let key = ObjWrapper(item);
      self.contents.insert(key, val);
      Ok(())
   }
}
impl_traits!(data=DelItem, Map);

impl_defaults!(ToRc; Map);
impl_defaults!(Object; Map);



impl_traits!(operators=QtAdd, Map);
impl_traits!(operators=QtSub, Map);
impl_traits!(operators=QtMul, Map);
impl_traits!(operators=QtDiv, Map);
impl_traits!(operators=QtMod, Map);
impl_traits!(operators=QtPow, Map);
impl_traits!(operators=QtEql, Map);
impl_traits!(operators=QtNeq, Map);
impl_traits!(operators=QtLth, Map);
impl_traits!(operators=QtGth, Map);
impl_traits!(operators=QtLeq, Map);
impl_traits!(operators=QtGeq, Map);
impl_traits!(misc=QtCall, Map);



























