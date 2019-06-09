
        section code,code

start
        nop
        nop

test_Module1_Case1
        moveq   #1,d0
        rts

test_Module2_Case2
        moveq   #1,d0
        rts

        section data,data

        nop
test_Module1_Case2
        moveq   #1,d0
        rts
irrelevant_label
