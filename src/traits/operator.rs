use objects::rc_object::RcObject;
use objects::result::{ObjResult, ObjError};
use parsing::frame::Frame;

macro_rules! oper_trait {
   ($trait_name:ident, $fn_name:ident) => {
      pub trait $trait_name {
         fn $fn_name(&self, _: RcObject, _: &mut Frame) -> ObjResult {
            Err(ObjError::NotImplemented)
         }
      }
   }
}
oper_trait!(OperAdd, oper_add);
oper_trait!(OperSub, oper_sub);
oper_trait!(OperMul, oper_mul);
oper_trait!(OperDiv, oper_div);
oper_trait!(OperMod, oper_mod);
oper_trait!(OperPow, oper_pow);
pub trait Opers : OperAdd + OperSub + OperMul + OperDiv + OperMod + OperPow {}