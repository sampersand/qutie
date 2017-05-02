#![allow(dead_code, unused)]

#[macro_use]
mod globals {
   pub static mut CURRENT_TYPE_ID: u8 = 0;
}
#[macro_use]
mod macros;
mod parsing;
mod objects;
mod traits;

fn main() {

   let inp = "2 * (3 + 4)";
   let mut stream = parsing::stream::Stream::from(inp);
   let mut frame = parsing::frame::Frame::new(stream);
   frame = frame.exec();
   println!("frame: {:?}", frame);
}








