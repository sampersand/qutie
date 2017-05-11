use std::iter::{Iterator, Peekable};
use std::str::Chars;
use parsing::token::{Token, Assignments, LineTerminators};
use obj::objects::block;
use parsing::identifier;

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
      todo!("peek not implemented");
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

macro_rules! is_path_seperator { ($c:expr) => ( $c == '.' ) }
macro_rules! is_assignment { ($c:expr) => ( $c == '=' ) }
macro_rules! is_terminator { ($c:expr) => ( $c == ';' ) }
macro_rules! is_comment { ($c:expr) => ( $c == '#' ) }
macro_rules! is_alpha { ($c:expr) => ( $c.is_alphabetic() || $c == '_' ) }
macro_rules! is_numeric { ($c:expr) => ( $c.is_numeric() || $c == '_' ) }
macro_rules! is_aplhanumeric { ($c:expr) => ( is_alpha!($c) || is_numeric!($c) ) }
macro_rules! is_quote { ($c:expr) => ( vec!['`', '\'', '"'].contains(&$c) ) }
macro_rules! is_block_start { ($c:expr) => ( vec!['(', '[', '{'].contains(&$c) ) }
macro_rules! is_block_end { ($c:expr) => ( vec![')', ']', '}'].contains(&$c) ) }
macro_rules! is_oper_start { ($c:expr) => ( vec!['+', '-', '*', '/'].contains(&$c) ) }
macro_rules! is_symbol { ($c:expr) => (
   vec!['+', '-', '*', '/', '%', '<', '>', '=', '&', '|', '^', '~'].contains(&$c)
) }

impl <'a> Stream <'a> {
   fn next_identifier(&mut self) -> Token {
      let mut acc = String::new();
      let mut is_path = false;
      loop {
         match self.source.peek() {
            Some(c) if is_aplhanumeric!(*c) => {},
            Some(c) if is_path_seperator!(*c) => { is_path = true; },
            _ => break
         }
         acc.push(self.source.next().unwrap());
      }
      assert!(!acc.is_empty());
      if is_path {
         if acc.chars().last().unwrap() == '.' { panic!("bad identifier: {:?}", acc) }
         Token::Path(acc)
      } else {
         Token::Identifier(identifier::Identifier::from(acc))
      }
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
      assert!(!acc.is_empty());
      Token::Number(acc)
   }

   fn next_oper(&mut self) -> Token {
      use parsing::operator::Operator;
      let mut acc = String::new();
      loop {
         match self.source.peek() {
            Some(c) if is_symbol!(*c) => {},
            _ => break
         }
         acc.push(self.source.next().unwrap());
      }
      assert!(!acc.is_empty());
      Token::Operator(Operator::from(acc.as_str()))
   }

   fn next_text(&mut self) -> Token {
      let quote = self.source.next().unwrap();
      let mut acc = String::new();
      while let Some(c) = self.source.next() {
         acc.push(
            match c {
               '\\' => 
                  match self.source.next() {
                     None => panic!("`\\` has nothing following it!"),
                     Some(e) => match e {
                        '\'' => '\'',
                        '"' => '"',
                        '`' => '`',
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        _ =>  panic!("Unknown escape character: {:?}", e)
                     }
                  },
               _ if c == quote => break,
               _ => c
            }
         )
      };
      Token::Text(quote, acc)
   }

   fn next_block(&mut self) -> Token {
      let lparen = block::LParen::from(self.source.next().unwrap());
      let rparen = lparen.get_rparen();
      let mut ret = vec![];
      loop {
         match self.next() {
            None => panic!("no rhs found for lparen: {:?}", lparen),
            Some(token) => 
               match token {
                  Token::RParen(p) => 
                     if p == rparen {
                        break
                     } else {
                        panic!("bad rparen {:?} for lparen {:?}", p, lparen)
                     },
                  _ => ret.push(token)
               }
         }
      }
      Token::Block((lparen, rparen), ret)
   }
   fn handle_comment(&mut self) -> Option<Token> {
      assert!(is_comment!(self.source.next().unwrap()));
      loop {
         match self.source.peek() {
            Some(c) if *c == '\n' => break,
            None => break,
            _ => {}
         }
         self.source.next();
      }
      self.next()
   }

   pub fn next(&mut self) -> Option<Token> {
      macro_rules! next_chr { () => (self.source.next().unwrap()) }
      self.strip_whitespace();
      if self.is_empty() {
         return None
      }
      let c = *self.source.peek().unwrap();

      match c {
         _ if is_assignment!(c)  => Some(Token::Assignment(Assignments::from(next_chr!()))),
         _ if is_terminator!(c)  => Some(Token::LineTerminator(LineTerminators::from(next_chr!()))),
         _ if is_comment!(c)     =>      self.handle_comment() /* will be some or none*/,
         _ if is_alpha!(c)       => Some(self.next_identifier()),
         _ if is_numeric!(c)     => Some(self.next_number()),
         _ if is_quote!(c)       => Some(self.next_text()),
         _ if is_block_start!(c) => Some(self.next_block()),
         _ if is_block_end!(c)   => Some(Token::RParen(block::RParen::from(next_chr!()))),
         _ if is_oper_start!(c)  => Some(self.next_oper()),
         _                       => Some(Token::Unknown(next_chr!()))
      }
   }
}



















