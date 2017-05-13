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


mod execute {
   use std::fs::File;
   use std::io::Read;
   pub fn read_file(path: &str) -> String {
      let mut text = String::new();
      match File::open(path) {
         Ok(mut file) => 
            if let Err(err) = file.read_to_string(&mut text){
               panic!("Cannot read file {:?} to string: {:?}", path, err)
            },
         Err(err) => panic!("Cannot open file {:?} for reading: {:?}", path, err)
      };
      text
   }
}



fn main() {
   let path = "/Users/westerhack/code/rust/qutie/examples/example.qt";
   let inp = execute::read_file(path);
   let mut stream = parsing::stream::Stream::new(&inp);
   let res = parsing::parser::parse(stream);
   // println!("{:?}\n", res);
}
























