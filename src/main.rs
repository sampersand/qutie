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

   let inp = "
if(1){
 a = 1;
}
a == 1";
   let mut stream = parsing::stream::Stream::from(inp);
   let mut frame = parsing::frame::Frame::new(stream);
   frame.exec();
   println!("frame: {:?}", frame);
}








