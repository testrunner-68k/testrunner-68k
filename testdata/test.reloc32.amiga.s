
	section code,code

test_TestModule_reloc32
	lea	relocated_refs(pc),a0

	move.l	(a0)+,a1
	move.l	(a1),d0

	move.l	(a0)+,a1
	move.l	(a1),a1
	add.l	(a1),d0

	move.l	(a0)+,a1
	add.l	(a1),d0

	cmp.l	#$00070007,d0
	beq.s	.testSucceeded
	bra.s	.testFailed
.testSucceeded
	moveq   #1,d0
	rts
.testFailed
	moveq   #0,d0
	rts

relocated_refs
	dc.l	code_label_1
	dc.l	data_label_1
	dc.l	code_label_2

code_label_3
	dc.l	$00040004
code_label_1
	dc.l	$00010001
code_label_2
	dc.l	$00020002

	section data,data

	nop
data_label_1
	dc.l	code_label_3
