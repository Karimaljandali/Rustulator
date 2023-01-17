// The gameboy is comprised of 8 individual 8-bit registers and 2 16-bit registers.
// These 8-bit registers can also be combined into 16-bit registers as needed with the following combinations:
// BC, DE, and HL acting as 16-bit registers when combined.
// A && F registers are special purpose with the following purposes:
// A register is reserved for the accumulator, which is short-term storage for arithmetic and logic.
// F register is reserved for flags representing things like overflow and 0.
// PC register is reserved for the Program Counter which points to the memory address of the next instruction.
// SP register is reserved for the Stack Pointer which points to the current stack position in memory.
struct Registers {
    a: u8, // Accumulator register
    f: u8, // Flag register
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16, // Program Counter
    sp: u16 // Stack Pointer
}