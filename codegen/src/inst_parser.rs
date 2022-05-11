use nom::{IResult, error::VerboseError};
use nom::number::complete::be_u16;
use nom::bytes::complete::take;
use serde::{Deserialize,Serialize};
//use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
#[derive(PartialEq, Eq, Clone, Copy)]
//#[serde(untagged)]
pub enum Time {
    One(usize),
    Two(usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
#[derive(PartialEq, Eq, Clone)]
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

type Res<T,U> = IResult<T, U, VerboseError<T>>;