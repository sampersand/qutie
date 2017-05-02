// use objects::object::{Object, RcObject};
// use objects::result::ObjResult;
// use parsing::frame::Frame;

// pub struct Symbol {
//    num: i32
// }

// use std;
// derive_impl!(Display; Symbol, num);
// derive_impl!(Debug; Symbol, "N");
// derive_impl!(ToRc; Symbol);
// derive_impl!(NEW; Symbol, num, i32);
// derive_impl!(Castable; Symbol);

// use traits::misc::TryFrom;
// impl TryFrom for Symbol {
//    fn try_from(inp: &str) -> Option<Symbol> {
//       match inp.parse::<i32>() {
//          Ok(num) => Some(Symbol::new(num)),
//          Err(_) => None
//       }
//    }
// }
// use traits::types::ToNumber;
// use std::rc::Rc;
// impl ToNumber for Symbol {
//    fn to_number(&self) -> Rc<Symbol> {
//       Symbol::new(self.num).to_rc()
//    }
// }


// impl Object for Symbol {
//    fn hash(&self) -> u8 {
//       self.num as u8
//    }
//    fn _eql(&self, other: RcObject) -> bool {
//       todo!("_eql for number")
//    }
// }

// macro_rules! impl_num_oper {
//    ($_trait:ident, $func:ident, $oper:tt) => {
//       use traits::operator::$_trait;
//       impl $_trait for Symbol {
//          fn $func(&self, other: RcObject, _: &mut Frame) -> ObjResult {
//             Ok(Symbol::new(self.num $oper other.to_number().num).to_rc())
//          }
//       }
//    }
// }
// impl_num_oper!(OperAdd, oper_add, +);
// impl_num_oper!(OperSub, oper_sub, -);
// impl_num_oper!(OperMul, oper_mul, *);
// impl_num_oper!(OperDiv, oper_div, /);
// impl_num_oper!(OperMod, oper_mod, %);
// impl_num_oper!(OperPow, oper_pow, &);

// derive_impl!(Opers; Symbol);
// derive_impl!(Types; Symbol);
// derive_impl!(ToText; Symbol, num);
// derive_impl!(ToBool; Symbol, num);












