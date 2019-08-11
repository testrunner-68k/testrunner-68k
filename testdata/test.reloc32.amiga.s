
        section code,code
start:
        move.l  #data_label_1,d0
        nop
code_label_1:
        move.l  #data_label_2,d1
        move.l  #code_label_1,a0
code_label_2:

        section bss,bss

        section data_c,data_c

        nop
data_label_1:
        nop
data_label_2:
