# MIPS_disassembly
This crate dissembles u32 into mips assembly

## Examples
Simple example
```rust
use MIPS_disassembly::get_disassembly;

let instr: u32 = 0x24a50001;
let instr_asm: String = get_disassembly(instr);
assert_eq!(instr_asm, "ADDIU $a1, $a1, 1".to_string())
```
Example using the advance version 
```rust
use MIPS_disassembly::get_disassembly_adv

let mut sym_tab: HashMap<u32, String> = HashMap::new();
sym_tab.insert(0x00000108, "decode_if".into());
let instr: u32 = 0x11a0001a;
let instr_adrs: u32 = 0x9c;
assert_eq!(
    "BEQ $t5, $zero, 26 <decode_if>",
    get_disassembly_adv(
        instr,
        instr_adrs,
        &sym_tab,
        &MipsDisassemblyOptions::new(true, true)
    )
);
```
Example of the pseudo instruction option
```rust
use MIPS_disassembly::get_disassembly_adv;
use MIPS_disassembly::MipsDisassemblyOptions;
use std::collections::HashMap;

let instr: u32 = 0x0;
assert_eq!(
    "NOP",
    get_disassembly_adv(
        instr,
        0x0,
        &HashMap::new(),
        &MipsDisassemblyOptions::new(true, true)
    )
);
assert_eq!(
    "SLL $zero, $zero, $zero",
    get_disassembly_adv(
        instr,
        0x0,
        &HashMap::new(),
        &MipsDisassemblyOptions::new(true, false)
    )
);
```

## Supported instructions

- SLL
- SRL
- SRA
- SLLV
- SRLV
- SRAV
- JR
- JALR
- SYSCALL
- ADD
- ADDU
- SUB
- SUBU
- AND
- OR
- XOR
- NOR
- SLT
- SLTU
- BLTZ
- BGEZ
- BLTZAL
- BGEZAL
- J
- JAL
- BEQ
- BNE
- BLEZ
- BGTZ
- ADDI
- ADDIU
- SLTI
- SLTIU
- ANDI
- ORI
- XORI
- LUI
- LB
- LH
- LWL
- LW
- LBU
- LHU
- LWR
- SB
- SH
- SWL
- SW
- SWR

# Supported pseudo instructions
- NOP
- B
- BAL
- MOVE
- JALR (omits $rd if its $31/$ra)
