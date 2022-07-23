

{% macro nop(i) %}
{% endmacro %}

{% macro ld(i) %}
    {{i.operands[1] | src_eval}}
    {{i.operands[0] | dest_eval(bits=i.instr_operand_size)}}
{% endmacro %}

{% macro inc(i) %}
    
{% endmacro %}

{% macro dec(i) %}
{% endmacro %}

{% macro rlca(i) %}
{% endmacro %}