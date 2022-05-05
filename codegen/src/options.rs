use serde::{Deserialize,Serialize};
//use serde_derive::{Serialize, Deserialize};
use clap::Parser;


#[derive(Debug, Serialize, Deserialize)]
//#[serde(untagged)]
pub enum Time {
    One(usize),
    Two(usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub val: u16,          //opcode in int
    pub operator: String,  //opcode in String 
    pub operands: Vec<String>, //operands list
    pub instr_size: usize, //0/8/16
    pub instr_operand_size: usize,
    pub time: Time,
    pub z: String,
    pub n: String,
    pub h: String,
    pub c: String,
}

#[derive(Parser, Debug)]
#[clap(name    = "CodeGenner")]
#[clap(version = "0.1.0")]
#[clap(author  = "Mycroft92 <madhukar DOT yerraguntla AT gmail.com>")]
#[clap(about   = "OpCode fetch and Code generation for gameboy emulation")]
pub struct CMDArgs {
    #[clap(long, short, default_value_t = String::from("https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html"))]
    pub url: String,
    #[clap(long, short, default_value_t = String::from("instruction_list.json"))]
    pub out: String,
    #[clap(long, short, default_value_t = String::from("output.log"))]
    pub log: String,
    #[clap(long, short)]
    pub debug: bool
}

//Builder API but I like derive API more
// pub fn parse_opts () {
//     let matches = App::new("CodeGenner")
//         .version("0.1.0")
//         .author("Mycroft92 <madhukar.yerraguntla@gmail.com>")
//         .about()
//         .arg();

// }