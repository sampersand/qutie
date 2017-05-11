use obj::objects::object::{Object, ObjType};

pub struct Boolean {
   val: bool
}

use std;
impl_defaults!(Debug; Boolean, 'B');
impl_defaults!(Display; Boolean, val);

impl Boolean {
   #[inline]
   pub fn new(inp: bool) -> Boolean {
      Boolean{val: inp}
   }
}
impl <'a> From<&'a str> for Boolean {
   fn from(inp: &'a str) -> Boolean {
      match inp {
         "true" => Boolean::new(true),
         "false" => Boolean::new(false),
         _ => unreachable!("Bad boolean input: {:?}", inp)
      }
   }
}
use obj::traits::operators::QtEql;
use std::rc::Rc;
use obj::result::BoolResult;
impl QtEql for Boolean {
   fn qt_eql(&self, other: &Rc<Object>) -> BoolResult {
      Ok(Boolean::new(other.is_a(ObjType::Boolean) &&
                      cast_as!(other, Boolean).val == self.val).to_rc())
   }
}

impl_defaults!(ToRc; Boolean);
impl_defaults!(Object; Boolean);

impl_defaults!(QtAdd; Boolean);
impl_defaults!(QtSub; Boolean);
impl_defaults!(QtMul; Boolean);
impl_defaults!(QtDiv; Boolean);
impl_defaults!(QtMod; Boolean);
impl_defaults!(QtPow; Boolean);
impl_defaults!(QtNeq; Boolean);
impl_defaults!(QtLth; Boolean);
impl_defaults!(QtGth; Boolean);
impl_defaults!(QtLeq; Boolean);
impl_defaults!(QtGeq; Boolean);



























