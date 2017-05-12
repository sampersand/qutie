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

factorial = func(inp) {
   if ( inp <= 1) {
      1
   } else {
      inp * factorial(inp - 1)
   }
   __debug inp
};

__debug(factorial(5))

";
   let mut stream = parsing::stream::Stream::new(inp);
   let res = parsing::parser::parse(&mut stream);
   // println!("{:?}\n", res);
   println!("\n--[done]--");
}
























