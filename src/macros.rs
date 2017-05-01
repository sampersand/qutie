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
      use objects::traits::misc::Castable;
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

   (NEW; $obj:ident, $item:ident, $item_type:ty) => {
      impl $obj {
         pub fn new($item: $item_type) -> $obj {
            $obj{ $item: $item }
         }
      }
   };

   (ToRc; $obj:ident) => {
      use objects::traits::misc::ToRc;
      impl ToRc for $obj {}
   };

   (OPER: +; $obj:ident) => {
      use objects::traits::operator::QtAdd;
      impl QtAdd for $obj {}
   };
}












