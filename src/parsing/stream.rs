use std::collections::LinkedList;
use std::iter::Iterator;
use std::iter::IntoIterator;
use std::fmt::{Formatter, Error, Display};

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

   pub fn peek(&self) -> Option<char> {
      match self.source.front() {
         Some(chr) => Some(chr.clone()),
         None => None
      }
   }
   pub fn take(&mut self, expected: char) -> char {
      assert_eq!(Some(expected), self.source.pop_front());
      expected
   }

   pub fn is_empty(&self) -> bool {
      self.source.is_empty()
   }
}












