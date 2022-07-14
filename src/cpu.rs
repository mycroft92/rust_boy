use std::fmt;
use log::{info};

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
//Should return, res and the flags as output
// retursn res, half_carry, carry, zero in that order
fn add8(r1: u8, r2: u8, c: bool) -> (u8, bool) {
    let (r,c1) = r1.overflowing_add(r2) ;
    let (r,c2) = if c {r.overflowing_add(1 as u8)} else {(r,c1)};
    (r,c1 || c2)
}

fn sub8() {

}

fn add16(){

}

fn sub16(){

}

fn signed()  {

}

//Need to write a raw interpreter after finishing the instruction handler
//TODO: finish CPU instruction handler
//TODO: 

#[test]
fn test_add8() {
    print!("Output: ({:?})\n", add8(0x0a, 0xfa, true));
    // assert_eq!(add8(0x12, 0x22, false), (0x34, false, false, false));
    // assert_eq!(add8(0x12, 0x22, true), (0x35, false, false, false));
    // assert_eq!(add8(0x12, 0x2f, false), (0x41, true, false, false));
    // assert_eq!(add8(0x12, 0x2f, true), (0x42, true, false, false));
    // assert_eq!(add8(0x12, 0xf0, false), (0x02, false, true, false));
    // assert_eq!(add8(0x12, 0xf0, true), (0x03, false, true, false));
    // assert_eq!(add8(0x0a, 0xfa, false), (0x04, true, true, false));
    // assert_eq!(add8(0x0a, 0xfa, true), (0x05, true, true, false));
    // assert_eq!(add8(0x00, 0x00, false), (0x00, false, false, true));
    // assert_eq!(add8(0x20, 0xe0, false), (0x00, false, true, true));
    // assert_eq!(add8(0x08, 0xf8, false), (0x00, true, true, true));
    // assert_eq!(add8(0x07, 0xf8, true), (0x00, true, true, true));
}

// #[test]
// fn test_sub8() {
//     assert_eq!(sub8(0x12, 0x10, false), (0x02, false, false, false));
//     assert_eq!(sub8(0x34, 0x22, true), (0x11, false, false, false));
//     assert_eq!(sub8(0x32, 0x2f, false), (0x03, true, false, false));
//     assert_eq!(sub8(0x32, 0x2e, true), (0x03, true, false, false));
//     assert_eq!(sub8(0x12, 0xf0, false), (0x22, false, true, false));
//     assert_eq!(sub8(0x12, 0xe0, true), (0x31, false, true, false));
//     assert_eq!(sub8(0x0a, 0xef, false), (0x1b, true, true, false));
//     assert_eq!(sub8(0x20, 0x5a, true), (0xc5, true, true, false));
//     assert_eq!(sub8(0x12, 0x12, false), (0x00, false, false, true));
//     assert_eq!(sub8(0x88, 0x87, true), (0x00, false, false, true));
// }

// #[test]
// fn test_add16() {
//     assert_eq!(add16(0x1200, 0x1000, false), (0x2200, false, false, false));
//     assert_eq!(add16(0x1134, 0x1222, true), (0x2357, false, false, false));
//     assert_eq!(add16(0xf231, 0x2a13, false), (0x1c44, false, true, false));
//     assert_eq!(add16(0xf231, 0x2a13, true), (0x1c45, false, true, false));
//     assert_eq!(add16(0xf631, 0x2a03, false), (0x2034, true, true, false));
//     assert_eq!(add16(0xf631, 0x2a03, true), (0x2035, true, true, false));
// }

// #[test]
// fn test_signed() {
//     assert_eq!(signed(0x0a), 0x000a);
//     assert_eq!(signed(0x8a), 0xff8a);
// }

#[cfg(test)]
mod test {
    use crate::cpu;

    #[test]
    fn show_CPU() {
        let cpu = cpu::CPU {a: 0, b: 0, c :0 , d: 0, e: 0, f: 0xF0, h:0 , l:0, pc: 0x1,  sp: 0x5, halt: false, im: true, stop: false };
        println!("{}",cpu);
    }
}
