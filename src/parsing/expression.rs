use parsing::frame::Frame;
use parsing::operator::Operator;
use parsing::token::Token;
use parsing::identifier::Identifier;
use obj::objects::list::List;
use obj::objects::block::Block;
use obj::objects::object::{ObjType, Object};
use obj::objects::function::Function;
use obj::traits::{ToRc, Castable};
use std::rc::Rc;
use obj::traits::data::SetItem;

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
            // if val.is_a(ObjType::Function) && 
            //       !self.is_empty() &&
            //       does_match!(self.peek_front().unwrap(), &Token::Block((_, _), _)) {
            //    let args = self.next_block().unwrap().pop_single_expr().expect("only one expr for args");
            //    let res = force_cast!(val, Function).qt_call(args, frame);
            //    frame.push(res);
            // } else {
               frame.push(val.clone())
            // }
      }
   }

   fn handle_assignment(mut self, frame: &mut Frame) {
      let mut keys = vec![];
      let mut assign = None;
      while let Some(token) = self.pop_front() {
         if does_match!(token, Token::Assignment(_)) {
            assign = Some(token);
            break
         } else {
            keys.push(token)
         }
      }
      // assign is unused
      let assign = assign.expect("We should have checked for the presence of an assignment");

      let was_endl = self.is_endl;
      self.is_endl = false;
      let val = self.exec_result(frame).expect("cant set a key to nothing!");
      if !was_endl {
         frame.push(val.clone());
      }
      /* this could be improved lol */
      if keys.len() == 1 {
         match keys.remove(0) {
            Token::Identifier(ident) => frame.set(ident, val),
            other @ _ => panic!("Can only set to Identifier, not {:?}", other)
         }
      } else if keys.is_empty() {
         panic!("cant set to nothing!")
      } else {
         if keys.len() != 2 {
            panic!("TODO: stringed assignment");
         }
         /* TODO: THIS */
         let key = 
            match keys.get(0).unwrap() {
               &Token::Identifier(ref ident) => frame.get(ident).expect("Can't find key"),
               o @ _ => panic!("Cant set to {:?}", o)
            };
         let item = Expression::new(vec![keys.pop().unwrap()], false).exec_result(frame).expect("Can't find val");
         if !item.is_a(ObjType::List) {
            panic!("Can only assign into list indicies right now!")
         }
         let item: Rc<List> = item.force_cast();
         unsafe {
            into_mutable!(key)
         }.set_item(item.contents.get(0).unwrap().clone(), val, frame).expect("Can't set item");

      }
   }

   fn is_assignment(&self) -> bool {
      for token in &self.body {
         if does_match!(token, &Token::Assignment(_)) {
            return true
         }
      }
      false
   }

   fn exec_result(mut self, frame: &mut Frame) -> Option<Rc<Object>> {
      self.exec(frame);
      frame.pop()
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
            Token::Block((lp, rp), mut body) => 
               match lp {
                  LParen::Round =>
                     {
                        /* todo: remove this peek */
                        if let Some(obj) = frame.pop() {
                           if let Some(func) = obj.cast() {
                              assert_eq!(body.len(), 1, "only one expression for function args!");
                              let args = body.pop().unwrap();
                              println!("frame: {:?}|args:{:?}, body:{:?}", frame, args, body);
                              /* check ehreto see if we can refactor this around */
                              let res = (func as Rc<Function>).qt_call(args, frame);
                              frame.push(res);
                              continue
                           } else {
                              frame.push(obj)
                           }
                        }
                        for expr in body {
                           expr.exec(frame);
                        }
                     },
                  LParen::Square => {
                     if !frame.is_empty() {
                        assert_eq!(body.len(), 1, "only one expression for function args!");
                        let item = body.pop().unwrap().exec_result(frame).expect("No item to find passed!");
                        let res = frame.pop().unwrap().get_item(item, frame).expect("Can't access item!");
                        frame.push(res);
                     } else {
                        let stack = 
                           {
                              let mut internal_frame = frame.spawn_child();
                              for expr in body {
                                 expr.exec(&mut internal_frame)
                              }
                              internal_frame.take_stack()
                           };
                        frame.push(List::new(stack).to_rc());
                     }
                  },
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








