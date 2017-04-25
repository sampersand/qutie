use objects::rc_object::RcObject;
use objects::result::{ObjResult, ObjError};
use parsing::frame::Frame;

pub trait QtAdd {
   fn qt_add(&self, _: RcObject, _: &mut Frame) -> ObjResult {
      Err(ObjError::NotImplemented)
   }
}