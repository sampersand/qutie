use objects::object::{Object, RcObject};
use objects::result::ObjResult;
use parsing::frame::Frame;

pub struct Identifier {
   id: String // todo: update this
}

use std;
derive_impl!(Display; Identifier, id);
derive_impl!(Debug; Identifier, "I");
derive_impl!(ToRc; Identifier);
derive_impl!(Castable; Identifier);

use traits::misc::TryFrom;
impl TryFrom for Identifier {
   fn try_from(inp: &str) -> Option<Identifier> {
      match inp.chars().get(0) {
         Some(c) => {}
      }
      for c in inp.chars() {
         if !c.is_alphanumeric() {
            return None
         }
      }
      return Some(Identifier{id: inp})
   }
}
use std::rc::Rc;


impl Object for Identifier {
   fn hash(&self) -> u8 {
      self.sym.get(0).unwrap()
   }
   fn _eql(&self, other: RcObject) -> bool {
      todo!("_eql for number")
   }
}

macro_rules! impl_num_oper {
   ($_trait:ident, $func:ident, $oper:tt) => {
      use traits::operator::$_trait;
      impl $_trait for Identifier {
         fn $func(&self, other: RcObject, _: &mut Frame) -> ObjResult {
            Ok(Identifier::new(self.num $oper other.to_number().num).to_rc())
         }
      }
   }
}
impl_num_oper!(OperAdd, oper_add, +);
impl_num_oper!(OperSub, oper_sub, -);
impl_num_oper!(OperMul, oper_mul, *);
impl_num_oper!(OperDiv, oper_div, /);
impl_num_oper!(OperMod, oper_mod, %);
impl_num_oper!(OperPow, oper_pow, &);

derive_impl!(Opers; Identifier);
derive_impl!(Types; Identifier);
derive_impl!(ToText; Identifier, num);
derive_impl!(ToBool; Identifier, num);












