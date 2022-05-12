use nom::{IResult, Err , branch::alt};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::error::{VerboseError, VerboseErrorKind::Nom, ErrorKind};
use nom::character::complete::{digit0, digit1, one_of, char as nomChar,  multispace0, multispace1};
use nom::combinator::{all_consuming };
use nom::bytes::complete::{tag};
use nom::multi::{many_m_n, separated_list1, separated_list0};

use std::fmt;
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
    pub z: char,
    pub n: char,
    pub h: char,
    pub c: char,
}

 type Res<T,U> = IResult<T, U, VerboseError<T>>;


fn two_parse(input: &str) -> Res<&str, Time> {
    let (s,o) = digit1(input)?;
    let t1 : usize = o.parse().unwrap(); 
    let (s,_) = tag("/")(s)?;
    let (s,o) = digit1(s)?;
    let t2 : usize =  o.parse().unwrap();
    Ok((s,Time::Two(t1,t2))) 
} 

fn one_parse(i: &str) -> Res<&str, Time> {
    let (s,o) = digit1(i)?;
    Ok((s,Time::One(o.parse().unwrap())))
}

fn parse_time(i: &str) -> Res<&str,Time> {
    alt((two_parse, one_parse))(i)
}

fn parse_flags(i: &str) -> Res<&str, (char,char,char,char)> {

    let flagZ   = alt((nomChar('0'), nomChar('1'), nomChar('Z'), nomChar('-')));
    let flagN   = alt((nomChar('0'), nomChar('1'), nomChar('N'), nomChar('-')));
    let flagH   = alt((nomChar('0'), nomChar('1'), nomChar('H'), nomChar('-')));
    let flagC   = alt((nomChar('0'), nomChar('1'), nomChar('C'), nomChar('-')));
    let (i,(_,z,_, n, _,h, _,c,_)) = tuple((multispace0, flagZ, multispace1, flagN, multispace1, flagH, multispace1, flagC, multispace0))(i)?; 

    Ok((i,(z,n,h,c)))
}


#[cfg(test)]
mod tests {
    use crate::inst_parser::*;
    #[test]
    fn parse_flags0(){
        println!("{:?}",parse_flags(" - - - - "));
        assert_eq!(parse_flags(" - - - - "), Ok(("", ('-',  '-',  '-', '-'))))
    }

    #[test]
    fn parse_flags1(){
        println!("{:?}",parse_flags(" Z 1 0 -"));
        assert_eq!(parse_flags("Z 1 0 -"), Ok(("", ('Z',  '1',  '0', '-'))))
    }

    #[test]
    fn parse_flags2(){
        println!("{:?}",parse_flags("Z 10 - "));
        assert!(parse_flags("Z 10 - ").is_err())
    }

    #[test]
    fn parse_flags3(){
        println!("{:?}",all_consuming(parse_flags)((" Z H N C 1 0 ")));
        assert!(all_consuming(parse_flags)(" Z H N C 1 0 ").is_err())
    }

    #[test]
    fn parse_two() {
        assert_eq!(parse_time("25/28"), Ok(("", Time::Two(25,28))));
    }
    #[test]
    fn parse_one(){
        assert_eq!(parse_time("31"), Ok(("", Time::One(31))));
    }

    #[test]
    fn parse_fail(){
        println!("{:?}",parse_time("/31"));
        assert_eq!(parse_time("/31"), 
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("/31", Nom(ErrorKind::Digit)),
                    ("/31",Nom(ErrorKind::Alt))
                ]
            })));
    }

    #[test]
    fn parse_fail2(){
        println!("{:?}",all_consuming(parse_time)("31/"));
        assert_eq!(all_consuming(parse_time)("31/"), 
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("/", Nom(ErrorKind::Eof))
                ]
            })));
    }
}