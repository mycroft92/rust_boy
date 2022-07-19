//! We are going to generate the opcode handling from the yaml stuff
//! Uses tera for template generation
//! Read from instruction_list.yaml and generate the code in the following way:

//! fn opcode_XX(&mut self, arg: u16, &mut MMU) -> (time, number of bytes consumed)
use crate::inst_parser::{Instruction};
use log::{debug, error};
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use tera::Tera;
use tera::{to_value, try_get_value, Context, Value};
use std::process::{Command};
use std::env;
use std::path::Path;

pub fn hex(val: & Value, _: & HashMap<String, Value>) -> tera::Result<Value> {
    let val: u16 = try_get_value!("hex", "value", u16, val);
    debug!("Converted value from {} {}",val,format!("{:04x}", val));
    Ok(to_value(format!("{:04x}", val)).unwrap())
}

lazy_static! {
    pub static ref TERA: Tera = {
        let workdir  = match env::var("GB_ROOT") {
            Ok(x)  => String::from(x) + "/src/templates/**/*.rs",
            Err(e) => String::from("src/templates/**/*.rs")
        };

        let mut tera = match Tera::new(&workdir) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera.register_filter("hex", hex);
        tera
    };
}

pub fn generate(inst: &str, out: &str) -> tera::Result<()> {
    let file = File::open(&inst).map_err(|e| {
        error!("Unable to open {}: {}", inst, e);
        e
    })?;
    let insts: Vec<Instruction> = serde_yaml::from_reader(file).expect("Unpack error");
    let mut context = Context::new();
    context.insert("insts", &insts);

    let  inst_path = String::from(out) + "/inst.rs";

    let output = match TERA.render("inst.rs", &context) {
        Ok(output) => output,
        Err(e) => {
            error!("{:?}", e);
            return Err(e);
        }
    };

    let mut o = File::create(&inst_path).expect(&format!("Cannot open \"inst.rs\" in {} for writing!", out));
    write!(o, "{}", &output);

    //formats the file appropriately
    Command::new("rustfmt").args([&inst_path]).output().expect("Failed to execute rustfmt!");

    let assembler = match TERA.render("assembler.rs", &context) {
        Ok(output) => output,
        Err(e) => {
            error!("{:?}", e);
            // for e in e.iter().skip(1) {
            //     error!("Reason: {}", e);
            // }
            return Err(e);
        }
    };

    let  assm_path = String::from(out)+ "/assembler.rs";


    let mut o = File::create(&assm_path).expect("Cannot open assembler.rs for writing!");
    write!(o, "{}", &assembler);

    Command::new("rustfmt").args([&assm_path]).output().expect("Failed to execute rustfmt!");

    Ok(())
}
