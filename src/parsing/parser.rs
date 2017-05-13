use parsing::stream::Stream;
use parsing::frame::Frame;
use parsing::identifier::Identifier;
use parsing::token::Token;
use parsing::Expression;
use parsing::operator::Operator;
use obj::traits::ToRc;
use std::rc::Rc;
use obj::objects::number::Number;
use obj::objects::text::Text;
use obj::objects::object::{Object, ObjType};
use obj::objects::block::{LParen, Block};
use obj::objects::function::Function;


pub fn parse<'a>(stream: &'a mut Stream<'a>) {
   let ref mut frame = Frame::new(None);
   let exprs = get_exprs(stream);
   exec_exprs(exprs, frame);
}

pub fn exec_exprs(exprs: Vec<Expression>, frame: &mut Frame) {
   for expr in exprs {
      exec_expr(expr, frame);
   }
}

fn next_expr(stream: &mut Stream) -> Expression {
   let mut expr = vec![];
   while let Some(token) = stream.next() {
      match token {
         Token::LineTerminator => { expr.push(Token::LineTerminator); break },
         Token::Unknown(chr) => panic!("Unknown character: {:?}", chr),
         token @ _ => expr.push(token)
      }
   }
   expr
}

fn get_exprs(stream: &mut Stream) -> Vec<Expression> {
   let mut ret = vec![];
   while !stream.is_empty() {
      ret.push(next_expr(stream));
   }
   ret
}

fn handle_identifier(id: Identifier, tokens: &mut Expression, frame: &mut Frame) {
   use obj::constants;
   use obj::control_statements;
   if let Some(constant) = constants::get_constant(&id) {
      frame.push(constant);
      return
   }
   if control_statements::handle_control(&id, tokens, frame) {
      /* do nothing, was already handeled */
      return
   }
   if let Some(ref val) = frame.get(&id) {
      if val.is_a(ObjType::Function) {
         let args = next_expr!(tokens);
         let res = cast_as!(val, Function).qt_call(args, frame);
         frame.push(res);
      } else {
         frame.push(val.clone())
      }
      return
   }
   panic!("unknown identifier: {:?}", id);
}

fn handle_assignment(mut tokens: Expression, frame: &mut Frame) {
   assert!(2 < tokens.len(), "need at least 3 operands for assignment!");
   let identifier = 
      match tokens.remove(0) {
         Token::Identifier(identifier) => identifier,
         other @ _ => panic!("can only assign to identifiers not {:?}", other)
      };
   let assign_type = 
      match tokens.remove(0) {
         Token::Assignment(assign_type) => assign_type,
         other @ _ => unreachable!("The second thing should always be an assignment value, not {:?}!", other)
      };
   let is_endl = does_match!(*tokens.last().unwrap(), Token::LineTerminator);
   if is_endl {
      assert_match!(tokens.pop(), Some(Token::LineTerminator));
   }
      exec_expr(tokens, frame);
   let val = frame.pop().expect("cant set a key to nothing!");
   if !is_endl {
      frame.push(val.clone());
   }
   frame.set(identifier, val);
}

pub fn exec_expr(mut tokens: Expression, frame: &mut Frame) {
   if tokens.is_empty() { return }

   let is_assignment = 
      2 < tokens.len() && 
      match tokens.get(1).unwrap() {
         &Token::Assignment(_) => true,
         _ => false
      };
   if is_assignment {
      handle_assignment(tokens, frame);
      return
   }
   let mut oper_stack = Vec::<Operator>::new();
   while !tokens.is_empty() {
      let token = tokens.remove(0);
      match token {
         Token::Identifier(id)        => handle_identifier(id, &mut tokens, frame),
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
               LParen::Round => exec_exprs(body, frame),
               LParen::Square => panic!("what to do with square?"),
               LParen::Curly => frame.push(Block::new((lp, rp), body).to_rc()),
            },
         Token::Unknown(_)            => unreachable!(),
         Token::Assignment(_)         => unreachable!(),
         Token::RParen(_)             => unreachable!(), 
         Token::LineTerminator        =>
            {
               while let Some(oper) = oper_stack.pop() {
                  oper.exec(frame);
               }
               frame.pop();
               assert!(tokens.is_empty(), "ended without empty tokens: {:?}", tokens);
               return;
            },
         Token::Separator => { /* do nothing with separators by default */ }
      }
   };
   while let Some(oper) = oper_stack.pop() {
      oper.exec(frame);
   }
   assert!(tokens.is_empty(), "ended without empty tokens: {:?}", tokens)
}






