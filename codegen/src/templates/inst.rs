{% import "macros.rs" as macros %}
use crate::cpu::CPU;
use crate::mmu::MMU;

impl CPU {
    pub fn get_fz(&self) -> bool {
        self.regs.f & 0x80 == 0x80
    }

    pub fn get_fn(&self) -> bool {
        self.regs.f & 0x40 == 0x40
    }

    pub fn get_fh(&self) -> bool {
        self.regs.f & 0x20 == 0x20
    }

    pub fn get_fc(&self) -> bool {
        self.regs.f & 0x10 == 0x10
    }

    pub fn set_fz(&mut self, val: bool) {
        if val {
            self.regs.f = self.regs.f | 0x80;
        } else {
            self.regs.f = self.regs.f & 0x70;
        }
    }

    pub fn set_fn(&mut self, val: bool) {
        if val {
            self.regs.f = self.regs.f | 0x40;
        } else {
            self.regs.f = self.regs.f & 0xB0;
        }
    }

    pub fn set_fh(&mut self, val: bool) {
        if val {
            self.regs.f = self.regs.f | 0x20;
        } else {
            self.regs.f = self.regs.f & 0xD0;
        }
    }

    pub fn set_fc(&mut self, val: bool) {
        if val {
            self.regs.f = self.regs.f | 0x10;
        } else {
            self.regs.f = self.regs.f & 0xE0;
        }
    }

    pub fn get_bc(&self) -> u16 {
        (self.regs.b as u16) << 8 | (self.regs.c as u16)
    }

    pub fn set_bc(&mut self, val: u16) {
        self.regs.b = (val >> 8) as u8;
        self.regs.c = val as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.regs.d as u16) << 8 | (self.regs.e as u16)
    }

    pub fn set_de(&mut self, val: u16) {
        self.regs.d = (val >> 8) as u8;
        self.regs.e = val as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.regs.h as u16) << 8 | (self.regs.l as u16)
    }

    pub fn set_hl(&mut self, val: u16) {
        self.regs.h = (val >> 8) as u8;
        self.regs.l = val as u8;
    }

    pub fn set_a(&mut self, val: u8){
        self.regs.a = val
    }

    pub fn set_b(&mut self, val: u8){
        self.regs.b = val
    }

    pub fn set_c(&mut self, val: u8){
        self.regs.c = val
    }

    pub fn set_d(&mut self, val: u8){
        self.regs.d = val
    }

    pub fn set_e(&mut self, val: u8){
        self.regs.e = val
    }

    pub fn set_f(&mut self, val: u8){
        self.regs.f = val
    }
    
    pub fn set_h(&mut self, val: u8){
        self.regs.h = val
    }

    pub fn set_l(&mut self, val: u8){
        self.regs.l = val
    }

    pub fn set_pc(&mut self, val: u16) {
        self.regs.pc = val;
    }

    pub fn get_pc(& self) -> u16 {
        self.regs.pc 
    }

    pub fn get_a(&self) -> u8 { self.regs.a }
    pub fn get_b(&self) -> u8 { self.regs.b }
    pub fn get_c(&self) -> u8 { self.regs.c }
    pub fn get_d(&self) -> u8 { self.regs.d }
    pub fn get_e(&self) -> u8 { self.regs.e }
    pub fn get_f(&self) -> u8 { self.regs.f }
    pub fn get_h(&self) -> u8 { self.regs.h }
    pub fn get_l(&self) -> u8 { self.regs.l }
    
}

impl CPU {
{% for i in insts %}
/// {{i.operator}} {{i.operands | join(sep=",")}}
/// `code` is the current opcode read so far
    #[allow(unused_variables)]
    fn op_{{i.val | hex }}(&mut self, code: u16, mmu: &mut MMU) -> (u8, u8) {

    {%- if i.operator == "nop" -%}
        {{ macros::nop(i=i) }}
    {%- endif -%}

        ({{i.time | time_cond_false }}, {{i.instr_size}})
    }
{%endfor %}

    fn decode (&mut self, code: u16, mmu: &mut MMU) -> (u8,u8) {
        match code {
            {%-for i in insts -%}
            0x{{i.val | hex}} => op_{{i.val | hex}}(self, code, mmu),
            {%endfor%}
            _ => panic!("Invalid opcode: {:04x}: {:04x}", self.get_pc(), code)
        }
    }


    pub fn tick (&mut self, mmu: &mut MMU) -> usize {
        let opcode = mmu.read8(self.regs.pc);
        self.set_pc(self.get_pc()+1); //increment the PC by 1 for the byte read

        if opcode == 0xcb {
            let next           = mmu.read8(self.regs.pc+1);
            self.set_pc(self.get_pc()+1); //increment the PC by 1 for the byte read
            let (cycles, size) = self.decode((0xcb << 8) | (next as u16), mmu);
            //self.set_pc(self.get_pc().wrapping_add(size));
            cycles
        } else {
            let (cycles, size) = self.decode(opcode as u16, mmu);
            //self.set_pc(self.get_pc().wrapping_add(size));
            cycles
        }
    }

}