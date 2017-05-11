macro_rules! next_block {
    ($tokens:expr) => {
      match $tokens.remove(0) {
         Token::Block((_, _), body) => body,
         o @ _ => panic!("expected a block, not {:?}", o)
      }
    }
}

macro_rules! concat_all {
   ($start:expr $(, $child:expr)+ ) => {{
      let mut ret = String::new();
      ret.push_str($start.to_string().as_str());
      $(ret.push_str($child.to_string().as_str());)+
      ret
   }}
}
macro_rules! cast_as {
   ($obj:expr, $ty:ident) => {{
      assert!($obj.is_a(ObjType::$ty));
      use std::mem::transmute;
      use std::rc::Rc;
      unsafe {
         transmute::<&Rc<Object>, &Rc<$ty>>($obj)
      }
   }};
}
macro_rules! todo {
   ($msg:expr $(,$arg:expr)*) => {{
      print!("TODO:\n\t");
      println!($msg $(, $arg)* );
      use std::process::exit;
      exit(1);
   }}
}
macro_rules! impl_defaults {
   (ToRc; $obj:ident) => {
      use obj::traits::ToRc;
      impl ToRc for $obj {}
   };
   (Object; $obj:ident) => {
      impl Object for $obj {
         fn obj_type(&self) -> ObjType {
            ObjType::$obj
         }
      }
   };
   (Display; to_string; $obj:ident) => {
      impl std::fmt::Display for $obj {
         fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.to_string())
         }
      }
   };
   (Display; $obj:ident, $item:ident) => {
      impl std::fmt::Display for $obj {
         fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.$item)
         }
      }
   };
   (Debug; $obj:ident, $c:expr) => {
      impl std::fmt::Debug for $obj {
         fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{}({})", $c, self)
         }
      }
   };
   (to_string; char; $obj:ident) => {
      impl $obj {
         pub fn to_string(&self) -> String {
            char::from(self).to_string()
         }
      }
   };
   (to_string; func=$func:ident; $obj:ident) => {
      impl $obj {
         pub fn to_string(&self) -> String {
            self.$func().to_string()
         }
      }
   };
   (ToBoolean; $obj:ident) => { use obj::traits::conversion::ToBoolean; impl ToBoolean for $obj {} };
   (QtAdd; $obj:ident) => { use obj::traits::operators::QtAdd; impl QtAdd for $obj {} };
   (QtSub; $obj:ident) => { use obj::traits::operators::QtSub; impl QtSub for $obj {} };
   (QtMul; $obj:ident) => { use obj::traits::operators::QtMul; impl QtMul for $obj {} };
   (QtDiv; $obj:ident) => { use obj::traits::operators::QtDiv; impl QtDiv for $obj {} };
   (QtMod; $obj:ident) => { use obj::traits::operators::QtMod; impl QtMod for $obj {} };
   (QtPow; $obj:ident) => { use obj::traits::operators::QtPow; impl QtPow for $obj {} };
   (QtEql; $obj:ident) => { use obj::traits::operators::QtEql; impl QtEql for $obj {} };
   (QtNeq; $obj:ident) => { use obj::traits::operators::QtNeq; impl QtNeq for $obj {} };
   (QtLth; $obj:ident) => { use obj::traits::operators::QtLth; impl QtLth for $obj {} };
   (QtGth; $obj:ident) => { use obj::traits::operators::QtGth; impl QtGth for $obj {} };
   (QtLeq; $obj:ident) => { use obj::traits::operators::QtLeq; impl QtLeq for $obj {} };
   (QtGeq; $obj:ident) => { use obj::traits::operators::QtGeq; impl QtGeq for $obj {} };
}











