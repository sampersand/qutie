use parsing::stream::Stream;
use parsing::frame::Frame;
use traits::ToRc;
use std::rc::Rc;
use objects::number::Number;
use objects::object::Object;

pub fn parse<'a>(stream: &'a mut Stream<'a>) {
   let ref mut frame = Frame::new();
   while let Some(_) = stream.peek() {
      handle(stream, frame);
   }
   print!("frame: {:?}", frame);
}

macro_rules! is_whitespace { ($c:expr) => ( $c.is_whitespace() ) }
macro_rules! is_operator_start { ($c:expr) => ( vec!['+', '-', '*', '/'].contains(&$c) ) }
macro_rules! is_numeric { ($c:expr) => ( $c.is_numeric() ) }
macro_rules! is_numeric_start { ($c:expr) => ( is_numeric!($c) || $c == '_' ) }
macro_rules! strip_whitespace {
   ($stream:expr) => {
      loop {
         match $stream.peek() {
            Some(c) if is_whitespace!(c) => {},
            _ => break
         }
         $stream.next();
      }
   }
}



fn next_number(stream: &mut Stream, frame: &mut Frame) -> String {
   let mut acc = String::new();
   loop {
      match stream.peek() {
         Some(n) if n.is_numeric() || *n == '_' => {},
         _ => break
      }
      acc.push(stream.next().unwrap());
   }
   acc
}

fn handle_number(stream: &mut Stream, frame: &mut Frame) -> Rc<Number> {
   Number::from(next_number(stream, frame).as_str()).to_rc()
}

fn next_oper(stream: &mut Stream, frame: &mut Frame) -> String {
   let c = stream.next().unwrap();
   match c {
      '+' => '+'.to_string(),
      '-' => '-'.to_string(),
      '/' => '/'.to_string(),
      '*' =>
         {
            {
               let n = stream.peek();
               if n.is_none() || *n.unwrap() != '*' { return '*'.to_string() }
            }
            assert_eq!(stream.next().unwrap(), '*');
            "**".to_string() 
         },
      '%' => c.to_string(),
      _ => panic!("bad oper start: {:?}", c)
   }
}

fn priority(oper: &str) -> u8 {
   match oper {
      "+" | "-"       => 12,
      "*" | "/" | "%" => 11,
      "**"            => 10,
      _ => unreachable!("bad oper: {:?}", oper)
   }
}

fn handle_operator(stream: &mut Stream, frame: &mut Frame) -> Rc<Object> {
   let oper = next_oper(stream, frame);
   strip_whitespace!(stream);
   if is_operator_start!(stream.peek().unwrap())
   print!("{:?}", oper);
   panic!();
}


fn handle(stream: &mut Stream, frame: &mut Frame) -> Rc<Object> {
   strip_whitespace!(stream);
   let c = stream.peek().unwrap().clone();
   match c {
      _ if is_operator_start!(c) => handle_operator(stream, frame),
      _ if is_numeric_start!(c)  => handle_number(stream, frame),
      _ => panic!("Unknown character: {}", c)
   }
}















