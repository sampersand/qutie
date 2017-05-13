use parsing::stream::Stream;
use parsing::frame::Frame;
use parsing::identifier::Identifier;
use parsing::token::Token;
use parsing::expression::Expression;
use parsing::operator::Operator;
use obj::traits::ToRc;
use std::rc::Rc;
use obj::objects::number::Number;
use obj::objects::text::Text;
use obj::objects::object::{Object, ObjType};
use obj::objects::block::{LParen, Block};
use obj::objects::function::Function;


pub fn parse<'a>(mut stream: Stream<'a>) {
   let ref mut frame = Frame::new(None);
   while !stream.is_empty() {
      exec_expr(stream.next_expr(), frame);
   }
}

pub fn exec_exprs(exprs: Vec<Expression>, frame: &mut Frame) {
   for expr in exprs {
      exec_expr(expr, frame);
   }
}

fn handle_identifier(id: Identifier, expr: &mut Expression, frame: &mut Frame) {
   use obj::constants;
   use obj::control_statements;
   if let Some(constant) = constants::get_constant(&id) {
      frame.push(constant);
      return
   }
   if control_statements::handle_control(&id, expr, frame) {
      /* do nothing, was already handeled */
      return
   }
   match frame.get(&id) {
      None => panic!("unknown identifier: {:?}", id),
      Some(ref val) => 
         if val.is_a(ObjType::Function) && 
               !expr.is_empty() &&
               does_match!(expr.peek_front().unwrap(), &Token::Block((_, _), _)) {
            let args = expr.next_block().unwrap().pop_single_expr().expect("only one expr for args");
            let res = cast_as!(val, Function).qt_call(args, frame);
            frame.push(res);
         } else {
            frame.push(val.clone())
         }
   }
}

fn handle_assignment(mut expr: Expression, frame: &mut Frame) {
   assert!(2 < expr.len(), "need at least 3 operands for assignment!");
   let identifier = 
      match expr.pop_front().unwrap() {
         Token::Identifier(identifier) => identifier,
         other @ _ => panic!("can only assign to identifiers not {:?}", other)
      };
   let assign_type = 
      match expr.pop_front().unwrap() {
         Token::Assignment(assign_type) => assign_type,
         other @ _ => unreachable!("The second thing should always be an assignment value, not {:?}!", other)
      };

   let was_endl = strip_exec_expr(expr, frame);
   let val = frame.pop().expect("cant set a key to nothing!");
   if !was_endl { frame.push(val.clone()); }
   frame.set(identifier, val);
}

pub fn strip_exec_expr(mut expr: Expression, frame: &mut Frame) -> bool {
   let is_endl = expr.is_endl;
   if is_endl {
      expr.is_endl = false;
   }
   exec_expr(expr, frame);
   exor.is_endl = is_endl;
   is_endl
}

pub fn exec_expr(mut expr: Expression, frame: &mut Frame) {
   if expr.is_empty() { return }

   let is_assignment = 
      2 < expr.len() && 
      match expr.get(1).unwrap() {
         &Token::Assignment(_) => true,
         _ => false
      };
   if is_assignment {
      handle_assignment(expr, frame);
      return
   }
   let mut oper_stack = Vec::<Operator>::new();
   while !expr.is_empty() {
      let token = expr.pop_front();
      match token {
         Token::Identifier(id)        => handle_identifier(id, &mut expr, frame),
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
         Token::LineTerminator        => unreachable!(),
            {
               while let Some(oper) = oper_stack.pop() {
                  oper.exec(frame);
               }
               frame.pop();
               assert!(expr.is_empty(), "ended without empty expr: {:?}", expr);
               return;
            },
         Token::Separator => { /* do nothing with separators by default */ }
      }
   };
   while let Some(oper) = oper_stack.pop() {
      oper.exec(frame);
   }
   assert!(expr.is_empty(), "ended without empty expr: {:?}", expr)
}






