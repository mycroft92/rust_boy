use nom::{IResult, Err , branch::alt};
use nom::sequence::{delimited, preceded, terminated};
use nom::error::{VerboseError, VerboseErrorKind::Nom, ErrorKind};
use nom::character::complete::{digit0, digit1, one_of,  multispace0, multispace1};
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

#[derive(Debug, PartialEq, Clone)]
struct Flags {
    z: char,
    n: char,
    h: char,
    c: char
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "z: {}, n: {},h: {}, c: {}", self.z, self.n, self.h, self.c)
    }
}

impl Flags {
    fn tuple(&self) -> (char, char, char, char) {
        (self.z, self.n, self.h, self.c)
    }
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

// fn flag(i: &str) -> Res<&str, String> {
//     let (i,f) = one_of("znhc10-")(i)?;
//     Ok((i,String::from(f)))
// }

fn parse_flags(i: &str) -> Res<&str, Flags> {
    let (i,flags) = delimited(multispace0, separated_list1(tag(" "), one_of("ZNHC10-")), multispace0)(i)?; 
    //4 instances of FLAG with SPACE in between, optional SPACE at start and END
    //let (i,flags) = terminated(many_m_n(4, 4, preceded(multispace0,  one_of("ZNHC10-"))),multispace0)(i)?; 

    if flags.len() < 4 {
        return Err(Err::Error(
            VerboseError {
                errors: vec! [(i, Nom(ErrorKind::ManyMN))]
            }
        ))
    }
    let flags = Flags {
        z: *flags.get(0).unwrap(),
        n: *flags.get(1).unwrap(),
        h: *flags.get(2).unwrap(),
        c: *flags.get(3).unwrap(),
        };
    Ok((i,flags))
}



#[cfg(test)]
mod tests {
    use crate::inst_parser::*;
    #[test]
    fn parse_flags0(){
        println!("{:?}",parse_flags(" - - - - "));
        assert_eq!(parse_flags(" - - - - "), Ok(("", Flags {z: '-', n: '-', h: '-', c:'-'})))
    }

    #[test]
    fn parse_flags1(){
        println!("{:?}",parse_flags(" Z 1 0 -"));
        assert_eq!(parse_flags("Z 1 0 -"), Ok(("", Flags {z: 'Z', n: '1', h: '0', c:'-'})))
    }

    #[test]
    fn parse_flags2(){
        println!("{:?}",parse_flags("Z 10 - "));
        assert!(parse_flags("Z 10 - ").is_err())
    }

    #[test]
    fn parse_flags3(){
        println!("{:?}",all_consuming(parse_flags)((" Z H N C 1 0 ")));
        assert!(all_consuming(parse_flags)((" Z H N C 1 0 ")).is_ok())
        // assert_eq!(all_consuming(parse_flags)((" Z H N C 1 0 ")), 
        //     Err(Err::Error(
        //         VerboseError {
        //             errors: vec! [(" 1 0 ", Nom(ErrorKind::ManyMN))]
        //         }
        //     )));
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