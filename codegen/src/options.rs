use serde::{Deserialize,Serialize};
//use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
//#[serde(untagged)]
pub enum Time {
    One(usize),
    Two(usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub val: u16,
    pub operator: String,
    pub operands: Vec<String>,
    pub instr_bits: usize, //0/8/16
    pub instr_size: usize,
    pub time: Time,
    pub z: String,
    pub n: String,
    pub h: String,
    pub c: String,
}