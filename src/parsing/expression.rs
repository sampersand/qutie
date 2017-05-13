use parsing::token::Token;
use obj::objects::block::Block;

#[derive(Debug, Clone)]
pub struct Expression {
   body: Vec<Token>,
   is_endl: bool,
}

impl Expression {
   pub fn new(body: Vec<Token>) -> Expression {
      Expression{ body: body }
   }

   pub fn new_empty() -> Expression {
      Expression::new(vec![])
   }

   pub fn len(&self) -> usize {
      self.body.len()
   }

   pub fn push(&mut self, token: Token) {
      self.body.push(token);
   }

   pub fn peek_front(&self) -> Option<&Token> {
      self.body.first()
   }
   pub fn pop_front(&mut self) -> Option<Token> {
      if self.is_empty() {
         None
      } else {
         Some(self.body.remove(0))
      }
   }

   pub fn is_empty(&self) -> bool {
      self.body.is_empty()
   }

   pub fn next_block(&mut self) -> Option<Block> {
      match self.pop_front() {
         None => None,
         Some(token) =>
            match token {
               Token::Block((l, r), body) => Some(Block::new((l, r), body)),
               _ => None,
            }
      }
   }
   pub fn exec(mut self, frame: &mut Frame) {

   }

}








