#![allow(dead_code, unused)]

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
(2 + 3) * 4
";
   let mut stream = parsing::stream::Stream::new(inp);
   println!("{:?}\n", parsing::parser::parse(&mut stream));
}









