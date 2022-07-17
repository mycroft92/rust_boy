

pub struct MMU {
    pub mem: [u8; 65536]
}


pub trait Memory {
    fn read8(&self, addr: u16) -> u8;
    fn write8(&mut self, addr: u16, val: u8); 
    fn read16(&self, addr: u16) -> u16;
    fn write16(&mut self, addr: u16, val: u16);
}

impl Memory for MMU{
    fn read8 (&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    fn read16 (&self, addr: u16) -> u16 {
        let lower  = self.mem[addr as usize];
        let higher = self.mem[(addr + 1) as usize];
        (higher as u16) << 8 | lower as u16
    }

    fn write8 (&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }

    fn write16 (&mut self, addr: u16, val: u16) {
        self.write8(addr, val as u8);          //lower word
        self.write8(addr+1, (val >> 8) as u8); //upper word
    }
}