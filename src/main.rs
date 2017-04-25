#![allow(dead_code, unused)]
mod stream;

fn main() {
   let s = stream::Stream::from("1 + 2".to_string());
   println!("{}", s);
}