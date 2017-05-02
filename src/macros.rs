macro_rules! is_char {
   (number; $c:ident) => ( $c.is_digit(10) );
   (alphabetic; $c:ident) => ($c.is_alphabetic() || $c == '_');
   (alphanumeric; $c:ident) => (is_char!(alphabetic; $c) || is_char!(number; $c))
}

macro_rules! exception {
   (SYNTAX; $msg:expr, $($args:expr),*) => {
      panic!($msg, $($args,)*)
   }
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
      static mut __TYPE_ID: u8 = 0;
      use traits::misc::Castable;
      impl Castable for $obj {
         fn type_id() -> u8 {
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

   (ToRc; $obj:ident) => { use traits::misc::ToRc; impl ToRc for $obj {} };
   (Opers; $obj:ident) => { use traits::operator::Opers; impl Opers for $obj {} };
   (Types; $obj:ident) => { use traits::types::Types; impl Types for $obj {} };
   (ToText; $obj:ident, $item:ident) => {
      use traits::types::ToText;
      impl ToText for $obj {}
   };
   (ToBool; $obj:ident, $item:ident) => {
      use traits::types::ToBool;
      impl ToBool for $obj {}
   };
   (ToNumber; $obj:ident) => { use traits::types::ToNumber; impl ToNumber for $obj {} };

   (+; $obj:ident) => { use traits::operator::OperAdd; impl OperAdd for $obj {} };
      
   (*; $obj:ident) => {
      use traits::operator::OperMul;
      impl OperMul for $obj {}
   };
}













