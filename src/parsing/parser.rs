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



// fn handle_identifier(id: Identifier, expr: &mut Expression, frame: &mut Frame) {
//    use obj::constants;
//    use obj::control_statements;
//    if let Some(constant) = constants::get_constant(&id) {
//       frame.push(constant);
//       return
//    }
//    if control_statements::handle_control(&id, expr, frame) {
//       /* do nothing, was already handeled */
//       return
//    }
//    match frame.get(&id) {
//       None => panic!("unknown identifier: {:?}", id),
//       Some(ref val) => 
//          if val.is_a(ObjType::Function) && 
//                !expr.is_empty() &&
//                does_match!(expr.peek_front().unwrap(), &Token::Block((_, _), _)) {
//             let args = expr.next_block().unwrap().pop_single_expr().expect("only one expr for args");
//             let res = cast_as!(val, Function).qt_call(args, frame);
//             frame.push(res);
//          } else {
//             frame.push(val.clone())
//          }
//    }
// }

// fn handle_assignment(mut expr: Expression, frame: &mut Frame) {
//    assert!(2 < expr.len(), "need at least 3 operands for assignment!");
//    let identifier = 
//       match expr.pop_front().unwrap() {
//          Token::Identifier(identifier) => identifier,
//          other @ _ => panic!("can only assign to identifiers not {:?}", other)
//       };
//    let assign_type = 
//       match expr.pop_front().unwrap() {
//          Token::Assignment(assign_type) => assign_type,
//          other @ _ => unreachable!("The second thing should always be an assignment value, not {:?}!", other)
//       };

//    let was_endl = expr.strip_exec_expr(frame);
//    let val = frame.pop().expect("cant set a key to nothing!");
//    if !was_endl { frame.push(val.clone()); }
//    frame.set(identifier, val);
// }

// pub fn strip_exec_expr(mut expr: Expression, frame: &mut Frame) -> bool {
//    let is_endl = expr.is_endl;
//    if is_endl {
//       expr.is_endl = false;
//    }
//    expr.exec_expr(frame);
//    expr.is_endl = is_endl;
//    is_endl
// }




