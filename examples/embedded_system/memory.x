/* memory.x */
MEMORY
{
  RAM (rwx) : ORIGIN = 0x80200000, LENGTH = 128K
}

ENTRY(_start)

SECTIONS
{
  . = ORIGIN(RAM);

  .text : {
    KEEP(*(.text.entry))
    *(.text*)
  } > RAM

  .rodata : {
    *(.rodata*)
  } > RAM

  .data : {
    *(.data*)
  } > RAM

  .bss : {
    *(.bss*)
  } > RAM
}