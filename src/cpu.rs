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
    pc: u8, //program counter
    sp: u8, //stack pointer
    halt: bool,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
        a: {} 
        b: {}   c: {}
        d: {}   e: {}
        h: {}   l: {}
        sp: {}
        pc: {}
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
    fn get_fz(&self) -> u8 {
        self.f & 0x80
    }

    fn get_fn(&self) -> u8 {
        self.f & 0x40
    }

    fn get_fh(&self) -> u8 {
        self.f & 0x20
    }

    fn get_fc(&self) -> u8 {
        self.f & 0x1
    }
}

#[cfg(test)]
mod test {
    use crate::cpu;

    #[test]
    fn showCPU() {
        let cpu = cpu::CPU {a: 0, b: 0, c :0 , d: 0, e: 0, f: 0xF, h:0 , l:0, pc: 0x1,  sp: 0x5, halt: false };
        println!("{}",cpu);
    }
}
