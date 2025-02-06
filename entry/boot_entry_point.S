#include "early_print/api.inc"

.global start
start:
	msr     DAIFSet, 0xf

	ldr	x0, =_stack
	mov	x1, #4096	// TBD: derivce stack size from global config
	add	x0, x0, x1
	mov	sp, x0

	early_uart_init()
	early_print("= Project Saturn 2025 =\r\n")

	b _start_rust_kernel
