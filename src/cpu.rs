use std::fmt;
use log::{info};
use crate::mmu::{*};

//Registers of z80 CPU
#[derive(Debug, Clone)]
struct Registers {
    a : u8,    //accumulator
    b : u8,
    c : u8,
    d : u8,
    e : u8,
    f : u8,    //Flags
    h : u8,
    l : u8,
    pc: u16, //program counter
    sp: u16, //stack pointer
}

pub enum STATE{
    Halt,
    Stop,
    Interrupt,
    Run
}

pub struct CPU {
    regs : Registers,
    state: STATE
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "a: [{:02x}],
             b: [{:02x}],  c: [{:02x}]
             d: [{:02x}],  e: [{:02x}]
             f: [Z:{} N:{} H:{} C:{}]
             h: [{:02x}],  l: [{:02x}]
             pc: [{:04x}]
             sp: [{:04x}]",
            self.a,
            self.b, self.c,
            self.d, self.e,
            self.f & 0x80 == 0x80, self.f & 0x40 == 0x40, self.f & 0x20 == 0x20, self.f & 0x10 == 0x10,
            self.h, self.l,
            self.pc,
            self.pc
        )
    }

}

impl fmt::Display for STATE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            STATE::Halt => "Halt",
            STATE::Stop => "Stop",
            STATE::Interrupt => "Interrupt",
            STATE::Run  => "Run"
        };
        write!(f,
            "{}", s
        )
    }
}
impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "regs:
                    {}
             state: {}",
            self.regs,
            self.state)
    }
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            regs : Registers { a: 0, b: 0, c: 0, d: 0, e: 0, f: 0, h: 0, l: 0, pc: 0, sp: 0},
            state: STATE::Run 
        }
    }
}

fn add8(p: u8, q: u8, c: u8) -> u8 {0}
fn add16(p: u16, q: u16, c: u8) -> u16 {0}
