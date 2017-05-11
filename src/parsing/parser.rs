use parsing::stream::Stream;
use parsing::frame::Frame;
use parsing::token::Token;
use parsing::operator::Operator;
use obj::traits::ToRc;
use std::rc::Rc;
use obj::objects::number::Number;
use obj::objects::object::Object;
use obj::objects::identifier::Identifier;

pub fn parse<'a>(stream: &'a mut Stream<'a>) {
   let ref mut frame = Frame::new();
   while !stream.is_empty() {
      parse_expr(next_expr(stream), frame);
   }
   println!("frame: {:?}", frame);
}

fn handle_assignment(tokens: Vec<Token>, frame: &mut Frame) {
   todo!("todo: assignment");
}

fn parse_expr(tokens: Vec<Token>, frame: &mut Frame) {
   if tokens.is_empty() { return }
   let mut stack = Vec::<Rc<Object>>::new();
   let mut oper_stack = Vec::<Operator>::new();

   let is_assignment = 
      match tokens.get(1).unwrap() {
         &Token::Assignment(_) => true,
         _ => false
      };
   if is_assignment { handle_assignment(tokens, frame); return }
   for token in tokens {
      println!("token: {:?}", token);
      match token {
         Token::Identifier(id)        => stack.push(Identifier::from(id).to_rc()),
         Token::Number(num)           => stack.push(Number::from(num).to_rc()),
         Token::Operator(oper)        => 
         {
            while !oper_stack.is_empty() {
               if !oper_stack.last().unwrap().should_exec(&oper) {
                  break
               }
               oper_stack.pop().unwrap().exec(&mut stack);
            }
            oper_stack.push(oper)
         }
         Token::Text(quote, body)     => unimplemented!(),
         Token::Path(path)            => unimplemented!(),
         Token::Block((lp, rp), body) => unimplemented!(),

         Token::Unknown(s)        => unreachable!(),
         Token::LineTerminator(s) => unreachable!(),
         Token::Assignment(s)     => unreachable!(),
         Token::RParen(c)         => unreachable!(), 
      }
   };
   while let Some(oper) = oper_stack.pop() {
      oper.exec(&mut stack);
   }

   println!("opers: {:?}", oper_stack);
   println!("stack: {:?}", stack);
}

fn next_expr(stream: &mut Stream) -> Vec<Token> {
   let mut ret = vec![];
   while let Some(token) = stream.next() {
      match token {
         Token::LineTerminator(_) =>  break,
         Token::Unknown(chr) => panic!("Unknown character: {:?}", chr),
         token @ _ => ret.push(token)
      }
   }
   ret
}