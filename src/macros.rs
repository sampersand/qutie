macro_rules! todo {
   ($msg:expr $(,$arg:expr)*) => {{
      print!("TODO:\n\t");
      println!($msg $(, $arg)* );
      use std::process::exit;
      exit(1);
   }}
}
macro_rules! impl_defaults {
   (DISPLAY; $obj:ident) => {
      impl std::fmt::Display for $obj {
         fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.to_string())
         }
      }
   };
   (TO_STRING; char; $obj:ident) => {
      impl $obj {
         pub fn to_string(&self) -> String {
            char::from(self).to_string()
         }
      }
   };
   (TO_STRING; func=$func:ident; $obj:ident) => {
      impl $obj {
         pub fn to_string(&self) -> String {
            self.$func().to_string()
         }
      }
   };

}