use rust_lisp::model::ForeignValue;
use rust_lisp::model::Env;
use rust_lisp::model::Value;
use rust_lisp::model::RuntimeError;
use rust_lisp::utils::require_typed_arg;

use std::rc::Rc;
use std::cell::RefCell;

use strop::machine::mos6502::Instruction6502;
use strop::machine::mos6502::Mos6502;
use strop::search::BasicBlock;

pub struct BasicBlock6502 {
    bb: BasicBlock<Instruction6502>,
}

pub struct State6502 {
    state: Mos6502,
}
