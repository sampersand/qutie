use objects::object::RcObject;

#[derive(Debug)]
pub enum ObjError {
   EndOfFile,
   NotImplemented,
   NoResultDontFail, /* only for endline */
   NoSuchKey(RcObject),
}
pub type ObjResult = Result<RcObject, ObjError>;

use std::rc::Rc;
use objects::boolean::Boolean;

pub type BoolResult = Result<Rc<Boolean>, ObjError>;