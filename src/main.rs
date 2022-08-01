use std::fs;

extern crate clap;
use clap::{Arg, Command};

use rust_lisp::default_env;
extern crate rust_lisp;
use rust_lisp::model::Symbol;
use std::rc::Rc;
use std::cell::RefCell;
use rust_lisp::model::Value;
mod lisp;
mod arch;

use rust_lisp::model::RuntimeError;


use rust_lisp::model::Env;
fn stochastic_search_6502(env: Rc<RefCell<Env>>, argl: &[Value]) -> Result<Value, RuntimeError> {
    return Ok(Value::NIL);
}

fn main() {
    // create a base environment
    let env = Rc::new(RefCell::new(default_env()));

    let matches = Command::new("listrop")
        .version("0.1.0")
        .author("Sam M W <sam.magnus.wilson@gmail.com>")
        .about("Stochastically generates machine code")
        .arg(Arg::new("architecture").required(true).help("the file to parse"))
        .arg(Arg::new("filename").required(true).help("the file to parse"))
        .get_matches();

    let arch = matches.get_one::<String>("architecture").expect("argument parsing failed");
    match arch.as_str() {
        "6502" => {
            env.borrow_mut().define(
                Symbol::from("stochastic-search"),
                Value::NativeFunc(stochastic_search_6502)
            );
        }
        _ => {
            panic!("Gah! Unknown architecture!");
        }
    }

    let filename = matches.get_one::<String>("filename").expect("argument parsing failed");
    let contents = fs::read_to_string(filename.as_str())
        .expect("Something went wrong reading the file");

    lisp::load_file(env, &contents);
}
