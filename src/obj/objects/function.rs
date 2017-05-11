use obj::objects::object::{Object, ObjType};
use obj::objects::block::Block;
use std::rc::Rc;
pub struct Function {
   file: String, /* todo: update this */
   line: usize,
   body: Rc<Block>
}

impl Function {
   pub fn new(file: String, line: usize, body: Rc<Block>) -> Function {
      Function{ file: file, line: line, body: body }
   }
   pub fn to_string(&self) -> String {
      concat_all!("<", self.file, ">")
   }
}

use std;
impl_defaults!(Debug; Function, "F");
impl_defaults!(Display; to_string; Function);


use obj::result::BoolResult;

impl_defaults!(ToRc; Function);
impl_defaults!(Object; Function);
impl_defaults!(ToBoolean; Function);

impl_defaults!(QtAdd; Function);
impl_defaults!(QtSub; Function);
impl_defaults!(QtMul; Function);
impl_defaults!(QtDiv; Function);
impl_defaults!(QtMod; Function);
impl_defaults!(QtPow; Function);
impl_defaults!(QtEql; Function);
impl_defaults!(QtNeq; Function);
impl_defaults!(QtLth; Function);
impl_defaults!(QtGth; Function);
impl_defaults!(QtLeq; Function);
impl_defaults!(QtGeq; Function);



























