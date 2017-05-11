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
foo = 3;
bar = 4;

__debug (if ( false ) { __debug 10 } else { __debug 11 }

";
   let mut stream = parsing::stream::Stream::new(inp);
   let res = parsing::parser::parse(&mut stream);
   // println!("{:?}\n", res);
   println!("\n--[done]--");
}



















