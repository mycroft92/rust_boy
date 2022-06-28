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
    fn get_fz(&self) -> bool {
        self.f & 0x80 == 0x80
    }

    fn get_fn(&self) -> bool {
        self.f & 0x40 == 0x40
    }

    fn get_fh(&self) -> bool {
        self.f & 0x20 == 0x20
    }

    fn get_fc(&self) -> bool {
        self.f & 0x10 == 0x10
    }

    fn set_fz(&mut self, val: bool) {
        if val {
            self.f = self.f | 0x80;
        } else {
            self.f = self.f & 0x70;
        }

    }

    fn set_fn(&mut self, val: bool) {
        if val {
            self.f = self.f | 0x40;
        } else {
            self.f = self.f & 0xB0;
        }

    }

    fn set_fh(&mut self, val: bool) {
        if val {
            self.f = self.f | 0x20;
        } else {
            self.f = self.f & 0xD0;
        }
    }

    fn set_fc(&mut self, val: bool) {
        if val {
            self.f = self.f | 0x10;
        } else {
            self.f = self.f & 0xE0;
        }

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
