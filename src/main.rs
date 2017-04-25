#![allow(dead_code, unused)]
mod parsing;

fn main() {
   let s = parsing::stream::Stream::from("1 + 2".to_string());
   println!("{}", s);
}