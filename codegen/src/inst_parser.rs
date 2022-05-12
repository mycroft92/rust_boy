//! # Instruction Parser
//! Parses each td element from webpage to produce an instruction struct
use nom::{IResult, branch::alt};
use nom::sequence::{delimited, preceded, tuple};
use nom::error::{VerboseError};
use nom::character::{complete::{ digit1,  char as nomChar,  multispace0, multispace1, alphanumeric1}, is_alphanumeric};
use nom::combinator::{all_consuming, recognize };
use nom::bytes::complete::{tag, take_while1};
use nom::multi::{ many1, separated_list0};
use std::{str};
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
    pub val_hex: String,      //opcode in hex
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

    let flag_z   = alt((nomChar('0'), nomChar('1'), nomChar('Z'), nomChar('-')));
    let flag_n   = alt((nomChar('0'), nomChar('1'), nomChar('N'), nomChar('-')));
    let flag_h   = alt((nomChar('0'), nomChar('1'), nomChar('H'), nomChar('-')));
    let flag_c   = alt((nomChar('0'), nomChar('1'), nomChar('C'), nomChar('-')));
    let (i,(_,z,_, n, _,h, _,c,_)) = tuple((multispace0, flag_z, multispace1, flag_n, multispace1, flag_h, multispace1, flag_c, multispace0))(i)?; 

    Ok((i,(z,n,h,c)))
}

fn parse_sp(i: &str) -> Res<&str,()> {
    let (i,_) = many1(tag("&nbsp;"))(i)?;
    Ok((i,()))
}

fn parse_nl(i: &str) -> Res<&str,()> {
    let (i,_) = many1(tag("<br>"))(i)?;
    Ok((i,()))
}

fn parse_operand(i: &str) -> Res<&str, String>{
    let sym  = |c| {c == '-' || c == '(' || c ==')' || c == '+'};
    let test = |c: char| {is_alphanumeric(c as u8) || sym(c) };
    let (i, id) =  take_while1(test)(i)?; //()+-num/letter
    Ok((i ,String::from(id)))
}

fn parse_mnemonic(i: &str) -> Res<&str, String>{
    let (i,o) = recognize(
        alphanumeric1)
        (i)?;
      Ok((i,String::from(o)))
}

fn parse_operands(i: &str) -> Res<&str, Vec<String>>{
    delimited(multispace0, separated_list0( tag(","), parse_operand), multispace0)(i)
}


fn parse_inst(i:&str) -> Res<&str, (String,Vec<String>)> {
    //! Returns both instruction name and operands
    tuple((parse_mnemonic, parse_operands))(i) 
}

fn parse_timeline (i: &str) -> Res<&str, (usize, Time)> {
    let (i, size) = digit1(i)?;
    let (i, cycles) = preceded(many1(parse_sp), parse_time)(i)?;
    Ok((i,(size.parse().unwrap(),cycles)))
}

pub fn parse_data (i:&str, code: u16, operand_size: usize) -> Res<&str, Instruction> {
    let (i, ((inst,ops),_,(s,t),_,(z,n,h,c)) ) = all_consuming(tuple((parse_inst,parse_nl,parse_timeline,parse_nl,parse_flags)))(i)?;
    let data = Instruction {
        val: code,
        val_hex: String::from(format!("{:#x}",code)),
        operator: inst.to_lowercase(),
        operands: ops.iter().map(|s| s.to_lowercase()).collect::<Vec<_>>(),
        instr_size: s,
        instr_operand_size: operand_size,
        time: t,
        z: z,
        h: h,
        n: n,
        c: c
     };

   Ok((i, data))  
}


#[cfg(test)]
mod tests {
    use crate::inst_parser::*;
    use nom::error::{VerboseError, VerboseErrorKind::Nom, ErrorKind};
    use nom::Err;
    use log::info;
    #[test]
    fn parse_flags0(){
        info!("{:?}",parse_flags(" - - - - "));
        assert_eq!(parse_flags(" - - - - "), Ok(("", ('-',  '-',  '-', '-'))))
    }

    #[test]
    fn parse_flags1(){
        info!("{:?}",parse_flags(" Z 1 0 -"));
        assert_eq!(parse_flags("Z 1 0 -"), Ok(("", ('Z',  '1',  '0', '-'))))
    }

    #[test]
    fn parse_flags2(){
        info!("{:?}",parse_flags("Z 10 - "));
        assert!(parse_flags("Z 10 - ").is_err())
    }

    #[test]
    fn parse_flags3(){
        info!("{:?}",all_consuming(parse_flags)(" Z H N C 1 0 "));
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
        info!("{:?}",parse_time("/31"));
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
        info!("{:?}",all_consuming(parse_time)("31/"));
        assert_eq!(all_consuming(parse_time)("31/"), 
            Err(Err::Error(VerboseError {
                errors: vec![
                    ("/", Nom(ErrorKind::Eof))
                ]
            })));
    }

    #[test]
    fn parse_inst_test(){
        info!("{:?}",parse_inst("LD DE,d16"));
    }


    #[test]
    fn parse_line(){
        info!("{:?}",parse_data("LD DE,d16<br>3&nbsp;&nbsp;12<br>- - - -",  5, 8));
        assert_eq!(
            parse_data("LD DE,d16<br>3&nbsp;&nbsp;12<br>- - - -",  5, 8),
            Ok(("",Instruction {
                z: '-',
                h: '-',
                n: '-',
                c: '-',
                time: Time::One(12),
                instr_operand_size: 8,
                instr_size: 3,
                val: 5,
                val_hex: String::from(format!("{:#x}",5)),
                operator: String::from("LD"),
                operands: vec! [String::from("DE"), String::from("d16")]
            }))
        )

    }

    #[test]
    fn parse_line1() {
        info!("{:?}",parse_data("LD A,(HL+)<br>1&nbsp;&nbsp;8<br>- - - -", 5,8));
        assert!(parse_data("LD A,(HL+)<br>1&nbsp;&nbsp;8<br>- - - -",5,8).is_ok())
    }


}