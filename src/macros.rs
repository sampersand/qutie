macro_rules! into_mutable {
   ($obj:ident) => {{
      use std::mem::transmute;
      #[allow(mutable_transmutes)]
      let a: &mut Object = match $obj.obj_type() {
         ObjType::Number => {{
            use obj::objects::number::Number;
            *transmute::<&&Object, &mut &mut Number>(&&*$obj)
         }},
         ObjType::Text => {{
            use obj::objects::text::Text;
            *transmute::<&&Object, &mut &mut Text>(&&*$obj)
         }},
         ObjType::Block => {{
            use obj::objects::block::Block;
            *transmute::<&&Object, &mut &mut Block>(&&*$obj)
         }},
         ObjType::Boolean => {{
            use obj::objects::boolean::Boolean;
            *transmute::<&&Object, &mut &mut Boolean>(&&*$obj)
         }},
         ObjType::List => {{
            use obj::objects::list::List;
            *transmute::<&&Object, &mut &mut List>(&&*$obj)
         }},
         ObjType::Function => {{
            use obj::objects::function::Function;
            *transmute::<&&Object, &mut &mut Function>(&&*$obj)
         }},
         ObjType::Null => {{
            use obj::objects::null::Null;
            *transmute::<&&Object, &mut &mut Null>(&&*$obj)
         }},
      };
      a
   }}
}
macro_rules! assert_match {
   ($lhs:expr, $rhs:pat) => (assert_match!($lhs, $rhs, "Explicit assert error"));
   ($lhs:expr, $rhs:pat, $msg:expr) => ( assert!(does_match!($lhs, $rhs), $msg) )
}

macro_rules! does_match {
   ($lhs:expr, $rhs:pat) => (match $lhs { $rhs => true, _ => false })
}

macro_rules! concat_all {
   ($start:expr $(, $child:expr)* ) => {{
      let mut ret = String::new();
      ret.push_str($start.to_string().as_str());
      $(ret.push_str($child.to_string().as_str());)*
      ret
   }}
}

macro_rules! cast_as {
   ($obj:expr, $ty:ident) => {{
      let obj = $obj;
      assert!(obj.is_a(ObjType::$ty));
      use std::mem::transmute;
      use std::rc::Rc;
      unsafe {
         transmute::<&Rc<Object>, &Rc<$ty>>(obj)
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
}
macro_rules! impl_traits {
   (conv=$_trait:ident, $obj:ident) => {
      use obj::traits::conversion::$_trait;
      impl $_trait for $obj {}
   };

   (data=$_trait:ident, $obj:ident) => {
      use obj::traits::data::$_trait;
       impl $_trait for $obj {}
   };
   
   (oper=$_trait:ident, $obj:ident) => {
      use obj::traits::operators::$_trait; 
      impl $_trait for $obj {}
   };
}











