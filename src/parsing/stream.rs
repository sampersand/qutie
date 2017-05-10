use std::iter::{Iterator, Peekable};
use std::str::Chars;
use parsing::Token;

pub struct Stream<'a> {
   source: Peekable<Chars<'a>>
}

impl <'a> Stream<'a> {
   pub fn new<'b: 'a>(inp: &'b str) -> Stream<'a> {
      Stream{ source: inp.chars().peekable() }
   }
   pub fn is_empty(&mut self) -> bool {
      self.source.peek().is_none()
   }
   pub fn peek(&self) -> Token {
      panic!();
   }

   fn strip_whitespace(&mut self) {
      loop {
         match self.source.peek() {
            Some(c) if c.is_whitespace() => {},
            _ => break
         }
         self.source.next();
      }
   }
}

macro_rules! is_terminator { ($c:expr) => ( $c == ';' ) }
macro_rules! is_comment { ($c:expr) => ( $c == '#' ) }
macro_rules! is_alpha { ($c:expr) => ( $c.is_alphabetic() || $c == '_' ) }
macro_rules! is_numeric { ($c:expr) => ( $c.is_numeric() || $c == '_' ) }
macro_rules! is_aplhanumeric { ($c:expr) => ( is_alpha!($c) || is_numeric!($c) ) }
macro_rules! is_quote { ($c:expr) => ( vec!['`', '\'', '"'].contains(&$c) ) }
macro_rules! is_block_start { ($c:expr) => ( vec!['(', '[', '{'].contains(&$c) ) }
macro_rules! is_oper_start { ($c:expr) => ( vec!['+', '-', '*', '/'].contains(&$c) ) }

impl <'a> Stream <'a> {
   fn next_identifier(&mut self) -> Token {
      let mut acc = String::new();
      loop {
         match self.source.peek() {
            Some(c) if is_aplhanumeric!(*c) => {},
            _ => break
         }
         acc.push(self.source.next().unwrap());
      } 
      Token::Identifier(acc)
   }
   fn next_number(&mut self) -> Token {
      let mut acc = String::new();
      loop {
         match self.source.peek() {
            Some(c) if is_numeric!(*c) => {},
            _ => break
         }
         acc.push(self.source.next().unwrap());
      } 
      Token::Number(acc)
   }
   fn next_text(&mut self) -> Token {
      panic!("unimplemented: next_text")
   }
   fn next_block(&mut self) -> Token {
      panic!("unimplemented: next_block")
   }
   fn next_oper(&mut self) -> Token {
      panic!("unimplemented: next_oper")
   }
   fn handle_comment(&mut self) -> Token {
      loop {
         match self.source.peek() {
            Some(c) if *c == '\n' => break,
            None => break,
            _ => {}
         }
         self.source.next();
      } 
      Token::LineTerminator('#')
   }

   pub fn next(&mut self) -> Token {
      self.strip_whitespace();
      if self.is_empty() {
         panic!("called next on an empty stream")
      }
      let c = *self.source.peek().unwrap();

      match c {
         _ if is_terminator!(c) => Token::LineTerminator(c),
         _ if is_comment!(c) => self.handle_comment(),
         _ if is_alpha!(c) => self.next_identifier(),
         _ if is_numeric!(c) => self.next_number(),
         _ if is_quote!(c) => self.next_text(),
         _ if is_block_start!(c) => self.next_block(),
         _ if is_oper_start!(c) => self.next_oper(),
         _ => Token::Unknown(self.source.next().unwrap())
      }
   }
}



















