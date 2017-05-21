use obj::objects::object::{Object, ObjType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Quote {
   Single, Double, Grave
}
impl_defaults!(to_string; char; Quote);
impl_defaults!(Display; to_string; Quote);


impl From<char> for Quote {
   fn from(inp: char) -> Quote {
      match inp {
         '\'' => Quote::Single,
         '"' => Quote::Double,
         '`' => Quote::Grave,
         _ => unreachable!("bad quote: {:?}", inp)
      }
   }
}
impl <'a> From<&'a Quote> for char {
   fn from(inp: &'a Quote) -> char {
      match *inp {
         Quote::Single => '\'',
         Quote::Double => '"',
         Quote::Grave => '`',
      }
   }
}


pub struct Text {
   quote: Quote,
   text: String
}

impl Text {
   #[inline]
   pub fn new(quote: Quote, body: String) -> Text {
      Text{quote: quote, text: body}
   }
}

impl From<String> for Text {
   fn from(body: String) -> Text {
      Text::new(Quote::Single, body.to_string())
   }
}


use std;
impl_defaults!(Debug; Text, 'T');
impl_defaults!(Display; Text, text);

use std::rc::Rc;
use obj::result::{ObjResult, ObjError, BoolResult};
use obj::objects::boolean::Boolean;
use obj::traits::conversion::{ToBoolean, ToText};

impl ToBoolean for Text {
   fn to_boolean(&self) -> BoolResult {
      Ok(Boolean::get(!self.text.is_empty()).to_rc())
   }
}
impl ToText for Text {
   fn to_text(&self) -> Result<Rc<Text>, ObjError> {
      Ok(Text::from(self.text.clone()).to_rc())
   }
}

impl_defaults!(ToRc; Text);
impl_defaults!(Object; Text);

impl_traits!(data=GetItem, Text);
impl_traits!(data=SetItem, Text);
impl_traits!(data=DelItem, Text);

impl_traits!(oper=QtAdd, Text);
impl_traits!(oper=QtSub, Text);
impl_traits!(oper=QtMul, Text);
impl_traits!(oper=QtDiv, Text);
impl_traits!(oper=QtMod, Text);
impl_traits!(oper=QtPow, Text);
impl_traits!(oper=QtEql, Text);
impl_traits!(oper=QtNeq, Text);
impl_traits!(oper=QtLth, Text);
impl_traits!(oper=QtGth, Text);
impl_traits!(oper=QtLeq, Text);
impl_traits!(oper=QtGeq, Text);





