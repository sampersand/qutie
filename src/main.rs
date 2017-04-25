#![allow(dead_code, unused)]
mod parsing;
mod objects;

fn main() {
   let frame = parsing::parser::parse("1 + 2");
   println!("frame: {:?}", frame);
}