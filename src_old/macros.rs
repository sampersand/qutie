macro_rules! exception {
   (SYNTAX; $frame:expr, $msg:expr $(,$args:expr)*) => { panic!($msg $(,$args)*) };
   (ASSIGNMENT; $frame:expr, $msg:expr $(,$args:expr)*) => { panic!($msg $(,$args)*) };
   (RETRIEVAL; $frame:expr, $msg:expr $(,$args:expr)*) => { panic!($msg $(,$args)*) }
}

macro_rules! is_char {
   (number; $c:ident) => ( $c.is_digit(10) );
   (alphabetic; $c:ident) => ($c.is_alphabetic() || $c == '_');
   (alphanumeric; $c:ident) => (is_char!(alphabetic; $c) || is_char!(number; $c))
}

macro_rules! todo {
   ($msg:expr) => ( panic!("TODO: {}", $msg) );
   () => ( todo!("this") )
}

macro_rules! derive_impl {
   (Display; $obj:ident, $item:ident) => {
      impl std::fmt::Display for $obj {
         fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.$item)
         }
      }
   };

   (Debug; $obj:ident, $prefix:expr) => {
      impl std::fmt::Debug for $obj {
         fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{}({})", $prefix, self)
         }
      }
   };

   (Castable; $obj:ident) => {
      pub static mut __TYPE_ID: u8 = 0;
      use objects::traits::misc::Castable;
      impl Castable for $obj {
         fn type_id(&self) -> u8 {
            unsafe{
            use globals::CURRENT_TYPE_ID;
               if __TYPE_ID == 0 {
                  CURRENT_TYPE_ID += 1;
                  __TYPE_ID = CURRENT_TYPE_ID;
               }
               __TYPE_ID
            }
         }
      }
   };

   (ToRc; $obj:ident) => { use objects::traits::misc::ToRc; impl ToRc for $obj {} };
   (Opers; $obj:ident) => { use objects::traits::operator::Opers; impl Opers for $obj {} };
   (Types; $obj:ident) => { use objects::traits::types::Types; impl Types for $obj {} };
   (ToText; $obj:ident, $item:ident) => {
      use objects::traits::types::ToText;
      impl ToText for $obj {}
   };
   (ToBoolean; $obj:ident) => { use objects::traits::types::ToBoolean; impl ToBoolean for $obj {} };
   (ToNumber; $obj:ident) => { use objects::traits::types::ToNumber; impl ToNumber for $obj {} };

   (+; $obj:ident)  => { use objects::traits::operator::OperAdd; impl OperAdd for $obj {} };
   (-; $obj:ident)  => { use objects::traits::operator::OperSub; impl OperSub for $obj {} };
   (*; $obj:ident)  => { use objects::traits::operator::OperMul; impl OperMul for $obj {} };
   (/; $obj:ident)  => { use objects::traits::operator::OperDiv; impl OperDiv for $obj {} };
   (%; $obj:ident)  => { use objects::traits::operator::OperMod; impl OperMod for $obj {} };
   (**; $obj:ident) => { use objects::traits::operator::OperPow; impl OperPow for $obj {} };
   (==; $obj:ident) => { use objects::traits::operator::OperEql; impl OperEql for $obj {} };
   (==/!=; $obj:ident) => {
      use objects::traits::operator::{OperEql, OperNeq};
      impl OperEql for $obj {
         fn oper_eql(&self, other: RcObject, _: &mut Frame) -> BoolResult {
            use objects::boolean;
            Ok(boolean::Boolean::from(self._eql(other)).to_rc())
         }
      }
      impl OperNeq for $obj {
         fn oper_neq(&self, other: RcObject, frame: &mut Frame) -> BoolResult {
            match self.oper_eql(other, frame) {
               Ok(bool_) => bool_.oper_not(frame),
               Err(err) => Err(err)
            }
         }
      }
   };
   (!=; $obj:ident) => { use objects::traits::operator::OperNeq; impl OperNeq for $obj {} };
   (<; $obj:ident)  => { use objects::traits::operator::OperLth; impl OperLth for $obj {} };
   (<=; $obj:ident) => { use objects::traits::operator::OperLeq; impl OperLeq for $obj {} };
   (>; $obj:ident)  => { use objects::traits::operator::OperGth; impl OperGth for $obj {} };
   (>=; $obj:ident) => { use objects::traits::operator::OperGeq; impl OperGeq for $obj {} };
   (!; $obj:ident)  => {
      use objects::traits::operator::OperNot;
      impl OperNot for $obj {
         fn oper_not(&self, frame: &mut Frame) -> BoolResult {
            match self.to_boolean(){
               Ok(bool_) => Ok(bool_._not().to_rc()),
               Err(err) => Err(err)
            }
         }
      }
   };
}

macro_rules! is_a {
   ($obj:ident, $module:ident) => {{
      use objects::$module;
      $obj.type_id() == unsafe{ $module::__TYPE_ID }
   }}
}

macro_rules! cast_as {
   ($obj:ident, $into:ident) => {{
      // use objects::$module;
      // if is_a!($obj, $module) { 
      //    panic!("cannot cast `{}` to `{}`", $obj.type_id(), unsafe{$module::__TYPE_ID} );
      // }
      use std::mem;
      // use obejcts::object::RcObject;
      // assert_eq!(mem::size_of::<Self>(), mem::size_of::<T>(), "bad types!");
      unsafe { mem::transmute::<&RcObject, &Rc<$into>>(&$obj).clone() }
   }}
}

/*


   fn cls_type_id() -> u8;
   fn is_a<T: Castable>(inp: &T) -> bool {
      Self::cls_type_id() == inp.self_cls_type_id()
   }
   fn cast_to<'a, T: Castable>(inp: &'a T) -> &'a Self {
      if !Self::is_a(inp) { 
         panic!("cannot cast `{}` to `{}`", Self::cls_type_id(), T::cls_type_id());
      }
      use std::mem;
      assert_eq!(mem::size_of::<Self>(), mem::size_of::<T>(), "bad types!");
      unsafe { mem::transmute(inp) }
   }
}
*/







