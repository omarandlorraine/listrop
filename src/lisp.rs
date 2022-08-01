use std::rc::Rc;
use std::cell::RefCell;

extern crate rust_lisp;
use rust_lisp::model::RuntimeError;

use rust_lisp::model::Env;
pub fn load_file(env: Rc<RefCell<Env>>, contents: &str) -> Result<(), RuntimeError> {
    use rust_lisp::parser::parse;
    use rust_lisp::interpreter::eval_block;

    let mut ast = Vec::new();
    for item in parse(contents) {
        ast.push(item.unwrap());
    }
    eval_block(env, ast.into_iter()).unwrap();

    Ok(())
}
