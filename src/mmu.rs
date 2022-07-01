
use log;
use std::collections::HashMap;

pub struct SimpleMMU {
    pub mem: [u8; 65536],
    //pub handlers: HashMap<(u16,u16), MemHandler>
}

pub enum MemValue {
    PassThrough,
    Block,
    Replace(u8)
}

pub trait MemHandler {
    fn on_read(&self, mmu: &SimpleMMU, addr: u16) -> MemValue;

    fn on_write(&self, mmu: &SimpleMMU, addr: u16, value: u8) -> MemValue;
}

impl SimpleMMU {
    pub fn new() -> SimpleMMU {
        SimpleMMU {
            mem: [0u8; 65536],
            //handlers: HashMap::new()
        }
    }

    pub fn get(&self, addr: u16) {

    }

    pub fn get8(&self, addr: usize) -> u8 {
        self.mem[addr]
    } 

    pub fn set8(&mut self, addr: usize, val: u8) {
        self.mem[addr] = val;
    }

    pub fn get16(&self, addr: usize) -> u16 {
        let l = self.get8(addr);
        let h = self.get8(addr+1);
        (h as u16) << 8 | l as u16
    } 

    pub fn set16(&mut self, addr: usize, val: u16) {
        self.set8(addr, val as u8);
        self.set8(addr, (val >> 8) as u8)

    }
}