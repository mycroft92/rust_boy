//! We are going to generate the opcode handling from the yaml stuff
//! Uses tera for template generation
//! Read from instruction_list.yaml and generate the code in the following way:
//! 

//! fn opcode_XX(&mut self, arg: u16, &mut MMU) -> (time, number of bytes consumed)
use crate::inst_parser::{Instruction, Time};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use serde_yaml;
use tera::Tera;
use tera::{to_value, Context, Value};
use log::{info, debug, error};


pub fn generate(inst: &str, out: &str) -> tera::Result<()> {
    let file        = File::open(&inst).map_err(|e| {error!("Unable to open {}: {}",inst, e);e})?;
    let insts: Vec<Instruction> = serde_yaml::from_reader(file).expect("Unpack error");
    let mut context = Context::new();
    context.insert("insts", &insts);

    let tera = Tera::new("src/templates/**/*.rs").map_err(|e| {error!("Parsing error: {}",e);e})?;
    
    let output = match tera.render("inst.rs", &context) {
        Ok(output) => output,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };

    let mut o = File::create(out).expect(& format!("Cannot open {} for writing!", out));
    write!(o, "{}", & output);

    let assembler = match tera.render("assembler.rs", &context) {
        Ok(output) => output,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };
    
    let mut o = File::create("assembler.rs").expect("Cannot open assembler.rs for writing!");
    write!(o, "{}", & assembler);


    Ok(())
}