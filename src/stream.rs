use std::collections::LinkedList;

type SourceT = LinkedList<char>;

#[derive(Debug)]
pub struct Stream {
   source: SourceT
}

use std::fmt::{Formatter, Error, Display};

impl Display for Stream {
   fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
      write!(f, "{:?}", self.source)
   }
}

impl Stream {
   pub fn from(inp: String) -> Stream {
      let mut list = SourceT::new();
      for chr in inp.chars() {
         list.push_back(chr);
      }
      Stream{ source: list }
   }
}

