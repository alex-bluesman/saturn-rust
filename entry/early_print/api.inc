// Copyright (c) 2025 Alexander Smirnov <alex.bluesman.smirnov@gmail.com>
//
// Licensed under the MIT License (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
// http://opensource.org/licenses/MIT
//
// Unless required by applicable law or agreed to in writing, software distributed 
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR 
// CONDITIONS OF ANY KIND, either express or implied. See the License for the 
// specific language governing permissions and limitations under the License.

#ifndef _EARLY_PRINT_API_
#define _EARLY_PRINT_API_

#define early_uart_init()		\
	bl	early_uart_init

#define early_print(_s)			\
	adr	x0, 98f;		\
97:					\
	ldrb	w1, [x0], #1;		\
	cbz	w1, 99f;		\
	bl	early_putc;		\
	b	97b;			\
98:					\
	.asciz	_s;			\
	.align	2;			\
99:

#define early_print_hex()		\
	adr	x3, 98f;		\
	mov	x4, #8;			\
97:					\
	and	x1, x0, #0xf0000000;	\
	lsr	x1, x1, #28;		\
	ldrb	w1, [x3, x1];		\
	bl	early_putc;		\
	lsl	x0, x0, #4;		\
	subs	x4, x4, #1;		\
	b.ne	97b;			\
	b	99f;			\
98:					\
	.ascii	"0123456789abcdef";	\
	.align	2;			\
99:

#endif
