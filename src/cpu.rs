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

impl CPU {
    fn tick (&mut self, mmu: &mut MMU) {
        let opcode = mmu.read8(self.regs.pc);
    }

    fn set_bc(){

    }

    fn get_bc(){

    }

    fn set_de(){

    }

    fn get_de(){
        
    }

    fn set_hl(){

    }

    fn get_hl(){
        
    }
}

fn add8(p: u8, q: u8, c: u8) {}