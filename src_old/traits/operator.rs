use objects::object::RcObject;
use objects::result::{ObjResult, ObjError, BoolResult};
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
macro_rules! bool_oper_trait {
   ($trait_name:ident, $fn_name:ident) => {
      pub trait $trait_name {
         fn $fn_name(&self, _: RcObject, _: &mut Frame) -> BoolResult {
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

bool_oper_trait!(OperEql, oper_eql);
bool_oper_trait!(OperNeq, oper_neq);
bool_oper_trait!(OperLth, oper_lth);
bool_oper_trait!(OperLeq, oper_leq);
bool_oper_trait!(OperGth, oper_gth);
bool_oper_trait!(OperGeq, oper_geq);
pub trait OperNot {
   fn oper_not(&self, _: &mut Frame) -> BoolResult {
      Err(ObjError::NotImplemented)
   }
}

pub trait Opers : OperAdd + OperSub +
                  OperMul + OperDiv + OperMod +
                  OperPow + 
                  OperEql + OperNeq + OperLth + OperLeq + OperGth + OperGeq +
                  OperNot
                  {}