use nom::{IResult, error::VerboseError};
use nom::number::complete::be_u16;
use nom::bytes::complete::take;



use crate::options::{Instruction, Time};
type Res<T,U> = IResult<T, U, VerboseError<T>>;