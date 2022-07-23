

{% macro nop(i) %}
{% endmacro %}

{% macro ld(i) %}
    {{i.operands[1] | src_eval(bits=i.instr_operand_size)}}
    {{i.operands[0] | dest_eval(bits=i.instr_operand_size)}}
{% endmacro %}

{% macro inc(i) %}
    {{i.operands[0] | src_eval(bits=i.instr_operand_size)}}
    let v = v.wrapping_add(1);
    {{i.operands[0] | dest_eval(bits=i.instr_operand_size)}}
{% endmacro %}

{% macro dec(i) %}
    {{i.operands[0] | src_eval(bits=i.instr_operand_size)}}
    let v = v.wrapping_sub(1);
    {{i.operands[0] | dest_eval(bits=i.instr_operand_size)}}
{% endmacro %}

{% macro rlca(i) %}
{% endmacro %}