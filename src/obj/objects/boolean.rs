use obj::objects::object::{Object, ObjType};
use obj::traits::Castable;

pub struct Boolean {
   pub val: bool
}

pub const TRUE: Boolean = Boolean{val: true};
pub const FALSE: Boolean = Boolean{val: false};


impl Boolean {
   #[inline]
   pub fn get(inp: bool) -> Boolean {
      if inp { TRUE } else { FALSE }
   }
}
impl From<Boolean> for bool {
   fn from(inp: Boolean) -> bool {
      inp.val
   }
}

use std;
impl_defaults!(Debug; Boolean, "Bool");
impl_defaults!(Display; Boolean, val);

use obj::traits::operators::QtEql;
use std::rc::Rc;
use obj::result::{BoolResult, ObjError};
impl QtEql for Boolean {
   fn qt_eql(&self, other: &Rc<Object>) -> BoolResult {
      Ok(Boolean::get(
            if let Some(other_bool) = other.cast() {
               (other_bool as Rc<Boolean>).val == self.val
            } else {
               false
            }
         ).to_rc()
      )
   }
}
use obj::traits::conversion::{ToBoolean, ToText};
use obj::objects::text::Text;
impl ToText for Boolean {
   fn to_text(&self) -> Result<Rc<Text>, ObjError> {
      Ok(Text::from(self.val.to_string()).to_rc())
   }
}

impl ToBoolean for Boolean {
   fn to_boolean(&self) -> Result<Rc<Boolean>, ObjError> {
      Ok(Boolean::get(self.val).to_rc())
   }
}

impl_defaults!(ToRc; Boolean);
impl_defaults!(Object; Boolean);

impl_traits!(data=GetItem, Boolean);
impl_traits!(data=SetItem, Boolean);
impl_traits!(data=DelItem, Boolean);

impl_traits!(operators=QtAdd, Boolean);
impl_traits!(operators=QtSub, Boolean);
impl_traits!(operators=QtMul, Boolean);
impl_traits!(operators=QtDiv, Boolean);
impl_traits!(operators=QtMod, Boolean);
impl_traits!(operators=QtPow, Boolean);
impl_traits!(operators=QtNeq, Boolean);
impl_traits!(operators=QtLth, Boolean);
impl_traits!(operators=QtGth, Boolean);
impl_traits!(operators=QtLeq, Boolean);
impl_traits!(operators=QtGeq, Boolean);
impl_traits!(misc=QtCall, Boolean);



























