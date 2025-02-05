ENTRY(start)
OUTPUT_ARCH(aarch64)
PHDRS
{
	text PT_LOAD ;
}
SECTIONS
{
	. = 0x7fc00000;
	_start = .;

	.text :
	{
		*(.text)
	} : text = 0x3030

	. = ALIGN(1 << 12);
	_data = .;

	.data :
	{
		*(.data)
	} : text

	/* Stack reservation. TBD: derive stack size from global config */
	. = ALIGN(1 << 12);
	_stack = .;
	. = . + 0x1000;

	/* Align BSS start by page border to speed up flush */
	. = ALIGN(1 << 12);
	_bss_start = .;

	.bss :
	{
		*(.bss)
	} :text

	/* Align BSS end by 8-bytes border */
	. = ALIGN(1 << 3);
	_bss_end = .;

	. = ALIGN(1 << 12);
	_end = . ;
}
