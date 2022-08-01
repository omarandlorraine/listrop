use std::rc::Rc;
use std::cell::RefCell;

extern crate rust_lisp;
use rust_lisp::model::RuntimeError;

pub fn load_file(contents: &str) -> Result<(), RuntimeError> {
    use rust_lisp::parser::parse;
    use rust_lisp::interpreter::eval_block;

    let env = rust_lisp::default_env();

    let mut ast = Vec::new();
    for item in parse(&contents) {
        ast.push(item.unwrap());
    }
    eval_block(Rc::new(RefCell::new(env)), ast.into_iter()).unwrap();

    Ok(())
}
