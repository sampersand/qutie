use std::collections::LinkedList;
use std::iter::Iterator;
use std::iter::IntoIterator;
use std::fmt::{Formatter, Error, Display};

#[derive(Debug)]
pub struct CharWrapper<'a>{
   pub chr: char,
   stream: &'a mut Stream
}


impl <'a> CharWrapper<'a> {
   pub fn take(&mut self) -> char {
      #[cfg(test)]
      {
         assert_eq!(self.chr, self.stream.source.pop_front().unwrap());
      };
      #[cfg(not(test))]
      {
         self.stream.source.pop_front();
      }
      self.chr
   }
}


impl <'a> Display for CharWrapper<'a> {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{}", self.chr)
   }
}

use std::ops::Deref;
impl <'a> Deref for CharWrapper<'a> {
   type Target = char;
   fn deref(&self) -> &char {
      &self.chr
   }
}



#[derive(Debug)]
pub struct Stream {
   source: LinkedList<char>
}

impl Display for Stream {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{:?}", self.source)
   }
}

impl Stream {
   pub fn from(inp: &str) -> Stream {
      let mut list = LinkedList::<char>::new();
      for chr in inp.chars() {
         list.push_back(chr);
      }
      Stream{ source: list }
   }

   pub fn peek_chr(&self) -> char {
      self.source.front().expect("attempted to peek_chr at end of file").clone()
   }

   pub fn peek(&mut self) -> Option<CharWrapper> {
      let chr = match self.source.front() {
         Some(chr) => chr.clone(),
         None => return None
      };
      Some(CharWrapper{ chr: chr, stream: self })
   }

   pub fn is_empty(&self) -> bool {
      self.source.is_empty()
   }
}












