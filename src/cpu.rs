use std::fmt;

/// CPU model for Z80
pub struct CPU {
    a: u8, //accumulator
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8, //Flags
    h: u8,
    l: u8,
    pc: u16, //program counter
    sp: u16, //stack pointer
    halt: bool, //low power mode
    im: bool, //interrupt manager
    stop: bool //extra low power mode
}

impl Default for CPU {
    fn default() -> CPU {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0x0,
            sp: 0x0,
            halt: false,
            im: true,
            stop: false
        }
    } 
}

fn bool_to_int(val: bool) -> u8 {
    match val {
        true => 1,
        false => 0
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
        a: [{:02x}] 
        b: [{:02x}]   c: [{:02x}]
        d: [{:02x}]   e: [{:02x}]
        h: [{:02x}]   l: [{:02x}]
        sp: [{:04x}]
        pc: [{:04x}]
        flags: Z{} N{} H{} C{} ",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,
            self.get_fz(),
            self.get_fn(),
            self.get_fh(),
            self.get_fc()
        )
    }
}

impl CPU {
    pub fn get_fz(&self) -> bool {
        self.f & 0x80 == 0x80
    }

    pub fn get_fn(&self) -> bool {
        self.f & 0x40 == 0x40
    }

    pub fn get_fh(&self) -> bool {
        self.f & 0x20 == 0x20
    }

    pub fn get_fc(&self) -> bool {
        self.f & 0x10 == 0x10
    }

    pub fn set_fz(&mut self, val: bool) {
        if val {
            self.f = self.f | 0x80;
        } else {
            self.f = self.f & 0x70;
        }
    }

    pub fn set_fn(&mut self, val: bool) {
        if val {
            self.f = self.f | 0x40;
        } else {
            self.f = self.f & 0xB0;
        }
    }

    pub fn set_fh(&mut self, val: bool) {
        if val {
            self.f = self.f | 0x20;
        } else {
            self.f = self.f & 0xD0;
        }
    }

    pub fn set_fc(&mut self, val: bool) {
        if val {
            self.f = self.f | 0x10;
        } else {
            self.f = self.f & 0xE0;
        }
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub fn set_a(&mut self, val: u8){
        self.a = val
    }

    pub fn set_b(&mut self, val: u8){
        self.b = val
    }

    pub fn set_c(&mut self, val: u8){
        self.c = val
    }

    pub fn set_d(&mut self, val: u8){
        self.d = val
    }

    pub fn set_e(&mut self, val: u8){
        self.e = val
    }

    pub fn set_f(&mut self, val: u8){
        self.f = val
    }
    
    pub fn set_h(&mut self, val: u8){
        self.h = val
    }

    pub fn set_l(&mut self, val: u8){
        self.l = val
    }

    pub fn get_a(&self) -> u8 { self.a }
    pub fn get_b(&self) -> u8 { self.b }
    pub fn get_c(&self) -> u8 { self.c }
    pub fn get_d(&self) -> u8 { self.d }
    pub fn get_e(&self) -> u8 { self.e }
    pub fn get_f(&self) -> u8 { self.f }
    pub fn get_h(&self) -> u8 { self.h }
    pub fn get_l(&self) -> u8 { self.l }
    
}

//ALU implementation
impl CPU {
    pub fn add_e() {
        
    }

}

//Need to write a raw interpreter after finishing the instruction handler
//TODO: finish CPU instruction handler
//TODO: 

#[cfg(test)]
mod test {
    use crate::cpu;

    #[test]
    fn show_CPU() {
        let cpu = cpu::CPU {a: 0, b: 0, c :0 , d: 0, e: 0, f: 0xF0, h:0 , l:0, pc: 0x1,  sp: 0x5, halt: false, im: true, stop: false };
        println!("{}",cpu);
    }
}
