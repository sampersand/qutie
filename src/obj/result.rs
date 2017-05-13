use std::rc::Rc;
use obj::objects::object::Object;
use obj::objects::boolean::Boolean;
#[derive(Debug)]
pub enum ObjError {
   NotImplemented,
   Return(Rc<Object>),
}

pub type ObjResult = Result<Rc<Object>, ObjError>;
pub type BoolResult = Result<Rc<Boolean>, ObjError>;