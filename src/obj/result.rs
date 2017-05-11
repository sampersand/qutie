use std::rc::Rc;
use obj::objects::object::Object;
#[derive(Debug)]
pub enum ObjError {
   NotImplemented,
   PlaceHolderForFutureErrors
}

pub type ObjResult = Result<Rc<Object>, ObjError>;