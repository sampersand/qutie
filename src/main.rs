#![allow(dead_code, unused)]

#[macro_use]
extern crate lazy_static;

/*
Car = class(){
   __init = func(self, maker, wheels, doors){
      self.maker = maker;
      self.wheels = wheels;
      self.doors = doors;
   }
   __text = func(self){
      "I'm a " + self.maker + " with " + self.wheels + " wheels and " + self.doors + " doors."
   }
}
*/

#[macro_use]
mod macros;
mod parsing;
mod obj;

fn main() {
   let inp = "
# foo = 3;
# bar = 4;
# if ( false ) {
#    __debug foo 
# } else { 
#    __debug bar 
# };
i = 0;
a = i * 100;
i = i + 1;
while {
   i < 10
} {
   (__debug(i));
   i = i + 1;
}

";
   let mut stream = parsing::stream::Stream::new(inp);
   let res = parsing::parser::parse(&mut stream);
   // println!("{:?}\n", res);
   println!("\n--[done]--");
}



















