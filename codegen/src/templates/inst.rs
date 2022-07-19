{% import "macros.rs" as macros %}
use crate::cpu::CPU;
use crate::mmu;

impl CPU {
{% for i in insts %}
/// {{i.operator}} {{i.operands | join(sep=",")}}
/// `arg` represents number of bytes read so far 
/// `code` is the current opcode read so far
#[allow(unused_variables)]
    fn op_{{i.val | hex }}(&mut self, arg: u16, code: u16, mmu: &mut MMU) -> (u8, u8) {

    {%- if i.operator == "nop" -%}
        {{ macros::nop(i=i) }}
    {%- endif -%}

        ({{i.time}}, {{i.instr_size}})
    }
{%endfor %}

    fn decode (&mut self, arg: u16, code: u16, mmu: &mut MMU) -> (u8,u8) {
        match code {
            {%-for i in insts -%}
            0x{{i.val | hex}} => op_{{i.val | hex}}(self, arg, code, mmu),
            {%endfor%}
        }
    }


    pub fn tick (&mut self, mmu: &mut MMU) -> usize {
        let opcode = mmu.read8(self.regs.pc);

        if opcode == 0xcb {
            let next           = mmu.read8(self.regs.pc+1);
            let (cycles, size) = self.decode(2/*2bytes read*/, (0xcb << 8) | (next as u16), mmu);
            self.set_pc(self.get_pc().wrapping_add(size));
            cycles
        } else {
            let (cycles, size) = self.decode(1/*1byte read*/, opcode as u16, mmu);
            self.set_pc(self.get_pc().wrapping_add(size));
            cycles
        }
    }

}