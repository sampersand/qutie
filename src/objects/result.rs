use objects::rc_object::RcObject;

#[derive(Debug)]
pub enum ObjError {
   EndOfFile,
   NotImplemented,
   NoResultDontFail, /* only for endline */
   NoSuchKey(RcObject),
}
pub type ObjResult = Result<RcObject, ObjError>;