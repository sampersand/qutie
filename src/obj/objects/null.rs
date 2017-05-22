use obj::objects::object::{Object, ObjType};
use obj::objects::boolean::{Boolean, FALSE};
pub struct Null {}

pub const NULL: Null = Null{};


impl Null {
   #[inline]
   pub fn get() -> Null {
      NULL
   }
   pub fn to_string(&self) -> String {
      "null".to_string()
   }
}

use std;
impl_defaults!(Debug; Null, "Null");
impl_defaults!(Display; to_string; Null);

use obj::traits::operators::QtEql;
use std::rc::Rc;
use obj::result::{BoolResult, ObjError};
impl QtEql for Null {
   fn qt_eql(&self, other: &Rc<Object>) -> BoolResult {
      Ok(Boolean::get(other.is_a(ObjType::Null)).to_rc())
   }
}
use obj::traits::conversion::{ToBoolean, ToText};
use obj::objects::text::Text;
impl ToText for Null {
   fn to_text(&self) -> Result<Rc<Text>, ObjError> {
      Ok(Text::from(self.to_string()).to_rc())
   }
}

impl ToBoolean for Null {
   fn to_boolean(&self) -> Result<Rc<Boolean>, ObjError> {
      Ok(FALSE.to_rc())
   }
}

impl_defaults!(ToRc; Null);
impl_defaults!(Object; Null);

impl_traits!(data=GetItem, Null);
impl_traits!(data=SetItem, Null);
impl_traits!(data=DelItem, Null);

impl_traits!(operators=QtAdd, Null);
impl_traits!(operators=QtSub, Null);
impl_traits!(operators=QtMul, Null);
impl_traits!(operators=QtDiv, Null);
impl_traits!(operators=QtMod, Null);
impl_traits!(operators=QtPow, Null);
impl_traits!(operators=QtNeq, Null);
impl_traits!(operators=QtLth, Null);
impl_traits!(operators=QtGth, Null);
impl_traits!(operators=QtLeq, Null);
impl_traits!(operators=QtGeq, Null);
impl_traits!(misc=QtCall, Null);



























