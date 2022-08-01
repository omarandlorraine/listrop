use std::fs;

extern crate clap;
use clap::{Arg, ArgAction, Command};

extern crate rust_lisp;

mod lisp;
mod arch;

fn main() {
    let matches = Command::new("listrop")
        .version("0.1.0")
        .author("Sam M W <sam.magnus.wilson@gmail.com>")
        .about("Stochastically generates machine code")
        .arg(Arg::new("filename").required(true).help("the file to parse"))
        .subcommand(
            Command::new("kr580vm1").about("Generates code for the KR580VM1 or Intel 8080")
            .arg(Arg::new("only80").long("only80").action(ArgAction::SetTrue).help("do not permit KR580VM1 specific instructions (i.e., the generated program will be compatible with the Intel 8080)")))
        .subcommand(
            Command::new("stm8").about("Generates code for the STM8"))
        .subcommand(
            Command::new("mos6502").about("Generates code for the MOS 6502")
            .arg(Arg::new("rorbug").long("rorbug").action(ArgAction::SetTrue).help("avoid the bug in the ROR instruction of very early chips"))
            .arg(Arg::new("cmos").long("cmos").action(ArgAction::SetTrue).help("allow CMOS instructions (including phx, stz)"))
            .arg(Arg::new("illegal").long("illegal").action(ArgAction::SetTrue).help("allow illegal instructions (including lax, dcp, anc)"))
        )
        .get_matches();

    let filename = matches.get_one::<String>("filename").expect("argument parsing failed");
    let contents = fs::read_to_string(filename.as_str())
        .expect("Something went wrong reading the file");

    lisp::load_file(&contents);
}
