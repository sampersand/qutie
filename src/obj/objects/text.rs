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

impl_defaults!(QtAdd; Text);
impl_defaults!(QtSub; Text);
impl_defaults!(QtMul; Text);
impl_defaults!(QtDiv; Text);
impl_defaults!(QtMod; Text);
impl_defaults!(QtPow; Text);
impl_defaults!(QtEql; Text);
impl_defaults!(QtNeq; Text);
impl_defaults!(QtLth; Text);
impl_defaults!(QtGth; Text);
impl_defaults!(QtLeq; Text);
impl_defaults!(QtGeq; Text);





