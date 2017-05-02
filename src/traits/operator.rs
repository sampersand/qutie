use objects::rc_object::RcObject;
use objects::result::{ObjResult, ObjError};
use parsing::frame::Frame;

pub trait OperAdd {
   fn add(&self, _: RcObject, _: &mut Frame) -> ObjResult {
      Err(ObjError::NotImplemented)
   }
}
pub trait OperMul {
   fn mul(&self, _: RcObject, _: &mut Frame) -> ObjResult {
      Err(ObjError::NotImplemented)
   }
}