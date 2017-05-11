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

add = func(a, b){
   a + b
};
1 + add(2, 3)

";
   let mut stream = parsing::stream::Stream::new(inp);
   let res = parsing::parser::parse(&mut stream);
   // println!("{:?}\n", res);
   println!("\n--[done]--");
}



















