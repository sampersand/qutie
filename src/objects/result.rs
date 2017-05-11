use std::rc::Rc;
use objects::object::Object;
#[derive(Debug)]
pub enum ObjError {
   NotImplemented,
   PlaceHolderForFutureErrors
}

pub type ObjResult = Result<Rc<Object>, ObjError>;