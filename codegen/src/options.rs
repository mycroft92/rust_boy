
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name    = "CodeGenner")]
#[clap(version = "0.1.0")]
#[clap(author  = "Mycroft92 <madhukar DOT yerraguntla AT gmail.com>")]
#[clap(about   = "OpCode fetch and Code generation for gameboy emulation")]
pub struct CMDArgs {
    #[clap(long, short, default_value_t = String::from("https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html"))]
    pub url: String,
    #[clap(long, short, default_value_t = String::from("instruction_list.yaml"))]
    pub out: String,
    #[clap(long, short, default_value_t = String::from("output.log"))]
    pub log: String,
    #[clap(long, short)]
    pub debug: bool
}
