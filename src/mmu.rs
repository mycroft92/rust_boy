
use log;


pub struct SimpleMMU {
    pub mem: [u8; 65536],
    pub handlers: std::hash
}

pub enum MemValue {
    PassThrough,
    Block,
    Replace(u8)
}
pub trait mmu {
    ///returns byte data at a particular addr
    fn get8(&self, addr: u16) -> u8;
    ///sets byte data at a particular addr
    fn set8(&mut self, addr: u16, val: u8);
}

impl mmu for SimpleMMU {
    fn get8(&self, addr) -> u8 {
        self.mem[addr]
    } 

    fn set8(&mut self, addr: u16, val: u8) {
        self.mem[addr] = val;
    }
}