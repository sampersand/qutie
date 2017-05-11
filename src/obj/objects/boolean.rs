use obj::objects::object::{Object, ObjType};
use obj::traits::ToRc;

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

impl ToRc for Boolean {}
impl_defaults!(Object; Boolean);

use std::rc::Rc;
use obj::result::{ObjResult, ObjError};
impl_defaults!(QtAdd; Boolean);
impl_defaults!(QtSub; Boolean);
impl_defaults!(QtMul; Boolean);
impl_defaults!(QtDiv; Boolean);
impl_defaults!(QtMod; Boolean);
impl_defaults!(QtPow; Boolean);




























