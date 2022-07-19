use std::collections::HashMap;
use crate::inst_parser::{parse_inst};
use log::{debug};

lazy_static! {
    static ref inst_map: HashMap<u16 , &'static str> = {
        let mut m = HashMap::new();
// instruction mnemonic to hex code map
{%for i in insts%}    {% if i.operands | length > 0 %}m.insert({{i.val_hex}}, "{{i.operator}} {{i.operands|join(sep=",")}}"); {% else %}m.insert({{i.val_hex}}, "{{i.operator}}"); {%endif%}
{%endfor%}

    let length = {{insts| length}};
    m
    };

    static ref rev_map: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        {%for i in insts%}    {% if i.operands | length > 0 %}m.insert("{{i.operator}} {{i.operands|join(sep=",")}}", {{i.val_hex}}); {% else %}m.insert("{{i.operator}}", {{i.val_hex}}); {%endif%}
{%endfor%}
    m
    };
}

pub fn assemble(code: &str) -> Option<u16> {
    let (inst,ops) = parse_inst(code.to_lowercase());
    let ops = ops.join(",");
    //This is to remove extra spaces
    let inst_str = String::from(inst)+" "+ops; 
    //We still miss certain alt instructions
    //Will add them in a later release
    rev_map.get(inst_str)    
}