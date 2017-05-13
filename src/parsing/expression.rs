use parsing::frame::Frame;
use parsing::operator::Operator;
use parsing::token::Token;
use parsing::identifier::Identifier;
use obj::objects::block::Block;
use obj::objects::object::{ObjType, Object};
use obj::objects::function::Function;
use obj::traits::ToRc;

#[derive(Debug, Clone)]
pub struct Expression {
   body: Vec<Token>,
   is_endl: bool,
}

impl Expression {
   pub fn new(body: Vec<Token>, is_endl: bool) -> Expression {
      Expression{ body: body, is_endl: is_endl }
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

   fn handle_identifier(&mut self, id: Identifier, frame: &mut Frame){
      use obj::constants;
      use obj::control_statements;
      if let Some(constant) = constants::get_constant(&id) {
         frame.push(constant);
         return
      }
      if control_statements::handle_control(&id, self, frame) {
         /* do nothing, was already handeled */
         return
      }
      match frame.get(&id) {
         None => panic!("unknown identifier: {:?}", id),
         Some(ref val) => 
            if val.is_a(ObjType::Function) && 
                  !self.is_empty() &&
                  does_match!(self.peek_front().unwrap(), &Token::Block((_, _), _)) {
               let args = self.next_block().unwrap().pop_single_expr().expect("only one expr for args");
               let res = cast_as!(val, Function).qt_call(args, frame);
               frame.push(res);
            } else {
               frame.push(val.clone())
            }
      }
   }

   fn handle_assignment(mut self, frame: &mut Frame) {
      assert!(2 < self.len(), "need at least 3 operands for assignment!");
      let identifier = 
         match self.pop_front().unwrap() {
            Token::Identifier(identifier) => identifier,
            other @ _ => panic!("can only assign to identifiers not {:?}", other)
         };

      let assign_type = 
         match self.pop_front().unwrap() {
            Token::Assignment(assign_type) => assign_type,
            other @ _ => unreachable!("The second thing should always be an assignment value, not {:?}!", other)
         };

      let was_endl = self.is_endl;
      self.is_endl = false;
      self.exec(frame);
      let val = frame.pop().expect("cant set a key to nothing!");
      if !was_endl {
         frame.push(val.clone());
      }
      frame.set(identifier, val);
   }

   fn is_assignment(&self) -> bool {
      2 < self.len() && does_match!(self.body.get(1).unwrap(), &Token::Assignment(_))
   }

   pub fn exec(mut self, frame: &mut Frame) {
      use obj::objects::number::Number;
      use obj::objects::text::Text;
      use obj::objects::block::LParen;
      if self.is_empty() {
         return
      } else if self.is_assignment() {
         self.handle_assignment(frame);
         return
      }

      let mut oper_stack = Vec::<Operator>::new();

      while !self.is_empty() {
         let token = self.pop_front().unwrap();
         match token {
            Token::Identifier(id)        => self.handle_identifier(id, frame),
            Token::Number(num)           => frame.push(Number::from(num.as_str()).to_rc()),
            Token::Operator(oper)        => 
               {
                  while !oper_stack.is_empty() {
                     if !oper_stack.last().unwrap().should_exec(&oper) {
                        break
                     }
                     oper_stack.pop().unwrap().exec(frame);
                  }
                  oper_stack.push(oper)
               },
            Token::Text(quote, body)     => frame.push(Text::new(quote, body).to_rc()),
            Token::Path(path)            => unimplemented!(),
            Token::Block((lp, rp), body) => 
               match lp {
                  LParen::Round  => 
                     for expr in body {
                        expr.exec(frame);
                     },
                  LParen::Square => panic!("what to do with square?"),
                  LParen::Curly  => { frame.push(Block::new((lp, rp), body).to_rc()); },
               },
            Token::Unknown(_)            => unreachable!(),
            Token::Assignment(_)         => unreachable!(),
            Token::RParen(_)             => unreachable!(), 
            Token::LineTerminator        => unreachable!(),
            Token::Separator => { /* do nothing with separators by default */ }
         }
      };

      while let Some(oper) = oper_stack.pop() {
         oper.exec(frame);
      }

      if self.is_endl {
         frame.pop();
      }
      assert!(self.is_empty(), "ended without empty self: {:?}", self)

   }
}








