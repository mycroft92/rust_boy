//! We are going to generate the opcode handling from the yaml stuff
//! Uses tera for template generation
//! Read from instruction_list.yaml and generate the code in the following way:

//! fn opcode_XX(&mut self, arg: u16, &mut MMU) -> (time, number of bytes consumed)
use crate::inst_parser::{Instruction, Time, Flags};
use log::{debug, error};
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use tera::Tera;
use tera::{to_value, try_get_value, Context, Value};
use std::process::{Command};
use std::env;



pub fn hex(val: & Value, _: & HashMap<String, Value>) -> tera::Result<Value> {
    let val: u16 = try_get_value!("hex", "value", u16, val);
    debug!("Converted value from {} {}",val,format!("{:04x}", val));
    Ok(to_value(format!("{:04x}", val)).unwrap())
}

fn  is_mem <T: std::cmp::PartialEq> (k: T, v: Vec<T>) -> bool {
    for i in v.iter() {
        if *i == k { return true }
    }
    false
}

//https://meganesulli.com/generate-gb-opcodes/
// fn dest_help (v: &str) -> String {
//     let search = vec!["a","b","c","d","e","f","h","l","pc","sp","bc","de","hl"];
//     let bit16  = vec!["a16","a8"];
    
//     if is_mem(v, search) {
//         return String::from(v)+" what did I get "+ v
//     }
//     if is_mem(v, bit16) {
//         return String::from(&v[0..1])
//     }
//     String::new()
// }


//given an operand, find the corresponding way to set that location (this is the target of the operation)
fn dest_eval (val: & Value, map: & HashMap<String, Value>) -> tera::Result<Value> {
    let val = try_get_value!("dest_eval", "value", String, val);
    let bits = try_get_value!("dest_eval", "bits", u8, map.get("bits").unwrap());
    let search = vec!["a","b","c","d","e","f","h","l","pc","sp","bc","de","hl"];
    //let immediate   = vec!["a16","d16"];

    //fill the closing brace in macro along with arg
    let out = match (& val).starts_with("(") {
        //We are assuming the instructions are correct in addressed mode
        true  =>  { if is_mem(&val[1..val.len()-1], search) { 
                        format!( "mmu.write{}(self.get_{}(),v); ", bits, &val[1..val.len()-1])
                    } 
                    else if &val[1..val.len()-1] == "a16"  
                        {format!( "mmu.write{}(mem.read16(self.get_pc()), v); /*update pc by 2*/", bits )} 
                    else {format!( "/*Default*/mmu.write{}(mem.read{}(self.get_pc() as u16) ,v);  /*update pc by 1*/", bits, bits)} //a8 case
                    
                  },  // just take the first char, 
        false =>  { if is_mem(&val[0..], search) { format!( "self.set_{}(v);", val)} else { val}} 
    };
    
    Ok(to_value(out).unwrap())
}

//given an operand, find the corresponding way to get that location (this is the source of the operation)
fn src_eval (val: & Value, map : & HashMap<String, Value>) -> tera::Result<Value> {
    let val = try_get_value!("src_eval", "value", String, val);
    let bits = try_get_value!("dest_eval", "bits", u8, map.get("bits").unwrap());
    let search = vec!["a","b","c","d","e","f","h","l","pc","sp","bc","de","hl"];
    let immediate = vec!["a16","d16"];
    //To handle (FF00h+a16) type instructions
    //fill the closing brace in macro along with arg
    let out = match (& val).starts_with("(") {
        //We are assuming the instructions are correct in addressed mode
        true  =>  { if is_mem(&val[1..val.len()-1], search) { 
                        format!( " let v: u8 = mmu.read8(self.get_{}());  ", &val[1..val.len()-1])
                    } 
                    else if &val[1..val.len()-1] == "a8"  
                        {format!( "let v: u16 = mmu.read8(self.get_pc()); /*To update pc by 1*/")} 
                    else if is_mem(&val[1..val.len()-1], immediate) {format!( "/*Default true*/ let v = mmu.read16(self.get_pc()) /*{} To update pc by 2 */; ",val)}
                    else {error!("Unexpected operand! : {}", val);  val}               
                    
                  },  // just take the first char, 
        false =>  { if is_mem(&val[0..], search) { format!( "let v: u{} = self.get_{}();", bits, val)} 
                    else if is_mem(&val[0..], immediate) {format!( "let v: u16 = mmu.read16(self.get_pc()); /*Update PC by 2*/")} 
                    else if &val[0..] == "d8" {format!( "let v: u8 = mmu.read8(self.get_pc()); /*Update PC by 1*/")} 
                    else {error!("Unexpected operand! : {}", val); val}} 
    };
    Ok(to_value(out).unwrap())
}

// fn operand_eval (val: & Value, _: & HashMap<String, Value>) -> tera::Result<Value> {
//     match val {
//         tera::Value::Array(v)  => println!("{:?}",v),
//         tera::Value::String(s) => println!("{:?}",s),
//         _                     => println!("dont know bro {:?}", val)
//     };
//     Ok(to_value(val).unwrap())
// }


fn set_flag (val: & Value, map: & HashMap<String, Value>) -> tera::Result<Value> {
    let val = try_get_value!("set_flag", "value", char, val);
    let flag = try_get_value!("dest_eval", "flag", char, map.get("flag").unwrap());
    if val == '-' { return Ok(to_value("").unwrap())}
    else if val == '1' { }
    Ok(to_value(val).unwrap())
}

///If there's only one time unit return that else return the time unit corresponding to taking the branch
fn time_cond_true(val: & Value, _: & HashMap<String, Value>) -> tera::Result<Value> {
    let t = try_get_value!("time_cond_true", "value", Time, val);
    match t {
        Time::One(t)     => Ok(to_value(t).unwrap()),
        Time::Two(tt,tf) => Ok(to_value(tt).unwrap())
    }
}

///If there's only one time unit return that else return the time unit corresponding to NOT taking the branch
fn time_cond_false(val: & Value, _: & HashMap<String, Value>) -> tera::Result<Value> {
    let t = try_get_value!("time_cond_false", "value", Time, val);
    match t {
        Time::One(t)     => Ok(to_value(t).unwrap()),
        Time::Two(tt,tf) => Ok(to_value(tf).unwrap())
    }
}



lazy_static! {
    pub static ref TERA: Tera = {
        let workdir  = match env::var("GB_ROOT") {
            Ok(x)  => String::from(x) + "/codegen/src/templates/**/*.rs",
            Err(e) => String::from("src/templates/**/*.rs")
        };
        println!("Workdir for tera is {}",workdir);
        let mut tera = match Tera::new(&workdir) {
            Ok(t)  => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera.register_filter("hex", hex);
        tera.register_filter("time_cond_true", time_cond_true);
        tera.register_filter("time_cond_false", time_cond_false);
        tera.register_filter("dest_eval", dest_eval);
        tera.register_filter("src_eval" , src_eval);
        tera.register_filter("set_flag" , set_flag);
        //To write getting and setting according to operands
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
    // insts always have 2 operands only, should be easy to handle

    let  inst_path = String::from(out) + "/src/inst.rs";

    let output = match TERA.render("inst.rs", &context) {
        Ok(output) => output,
        Err(e)     => {
            error!("{:?}", e);
            return Err(e);
        }
    };

    let mut o = File::create(&inst_path).expect(&format!("Cannot open \"inst.rs\" in {} for writing!", out));
    write!(o, "{}", &output);

    //formats the file appropriately
    Command::new("rustfmt").args(["--force",&inst_path]).output().expect("Failed to execute rustfmt!");

    let assembler = match TERA.render("assembler.rs", &context) {
        Ok(output) => output,
        Err(e)     => {
            error!("{:?}", e);
            return Err(e);
        }
    };

    let  assm_path = String::from(out)+ "/src/assembler.rs";

    println!("assm_path: {} inst_path: {}",assm_path, inst_path);
    let mut o = File::create(&assm_path).expect("Cannot open assembler.rs for writing!");
    write!(o, "{}", &assembler);

    Command::new("rustfmt").args(["--force",&assm_path]).output().expect("Failed to execute rustfmt!");

    Ok(())
}
