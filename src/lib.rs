use std::collections::HashMap;

// const NOP: u32 = 0;
const OP_0: u32 = 0;

const FUNCT_SLL: u32 = 0;
const FUNCT_SRL: u32 = 0b00_0010;
const FUNCT_SRA: u32 = 0b00_0011;
const FUNCT_SLLV: u32 = 0b00_0100;
const FUNCT_SRLV: u32 = 0b00_0110;
const FUNCT_SRAV: u32 = 0b00_111;
const FUNCT_JR: u32 = 0b00_1000;
const FUNCT_JALR: u32 = 0b00_1001;
const SYSCALL: u32 = 0b00_1100;
const FUNCT_ADD: u32 = 0b10_0000;
const FUNCT_ADDU: u32 = 0b10_0001;
const FUNCT_SUB: u32 = 0b10_0010;
const FUNCT_SUBU: u32 = 0b10_0011;
const FUNCT_AND: u32 = 0b10_0100;
const FUNCT_OR: u32 = 0b10_0101;
const FUNCT_XOR: u32 = 0b10_0110;
const FUNCT_NOR: u32 = 0b10_0111;
const FUNCT_SLT: u32 = 0b10_1010;
const FUNCT_SLTU: u32 = 0b10_1011;

const OP_1: u32 = 1;

const B_FUNCT_BLTZ: u32 = 0;
const B_FUNCT_BGEZ: u32 = 1;
const B_FUNCT_BLTZAL: u32 = 0b1_0000;
const B_FUNCT_BGEZAL: u32 = 0b1_0001;

const OP_J: u32 = 0b00_0010;
const OP_JAL: u32 = 0b00_0011;
const OP_BEQ: u32 = 0b00_0100;
const OP_BNE: u32 = 0b00_0101;
const OP_BLEZ: u32 = 0b00_0110;
const OP_BGTZ: u32 = 0b00_0111;

const OP_ADDI: u32 = 0b00_1000;
const OP_ADDIU: u32 = 0b00_1001;
const OP_SLTI: u32 = 0b00_1010;
const OP_SLTIU: u32 = 0b00_1011;
const OP_ANDI: u32 = 0b00_1100;
const OP_ORI: u32 = 0b00_1101;
const OP_XORI: u32 = 0b00_1110;
const OP_LUI: u32 = 0b00_1111;

const OP_CP0: u32 = 0b01_0000;
// const CP0_FUNCT_MFC0: u32 = 0; //TODO implement missing instructions
// const CP0_FUNCT_MTF0: u32 = 0b0_0100;
// const CP0_FUNCT_SPECIAL: u32 = 0b1_0000;
// const CP0_FUNCT_SPECIAL_: u32 = 0b1_0000;

const OP_LB: u32 = 0b10_0000;
const OP_LH: u32 = 0b10_0001;
const OP_LWL: u32 = 0b10_0010;
const OP_LW: u32 = 0b10_0011;
const OP_LBU: u32 = 0b10_0100;
const OP_LHU: u32 = 0b10_0101;
const OP_LWR: u32 = 0b10_0110;

const OP_SB: u32 = 0b10_1000;
const OP_SH: u32 = 0b10_1001;
const OP_SWL: u32 = 0b10_1010;
const OP_SW: u32 = 0b10_1011;
const OP_SWR: u32 = 0b10_1110;

const REG_NAMES: [&str; 32] = [
    "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1", "$t2", "$t3", "$t4",
    "$t5", "$t6", "$s7", "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7", "$t8", "$t9",
    "$k0", "$k1", "$gp", "$sp", "$fp", "$ra",
];

const REG_NUMBER: [&str; 32] = [
    "$0", "$1", "$2", "$3", "$4", "$5", "$6", "$7", "$8", "$9", "$10", "$11", "$12", "$13", "$14",
    "$15", "$16", "$17", "$18", "$19", "$20", "$21", "$22", "$23", "$24", "$25", "$26", "$27",
    "$28", "$29", "$30", "$31",
];

/// This struct is used to pass options to the disassembly instructions
///
/// use_reg_names: if set to tru the register will be "$t1", if false the style will be "$9"
///
/// pseudo_instructions: if set to true instructions like SLL $zero $zero $zero will be converted to "NOP"
pub struct MipsDisassemblyOptions {
    pub use_reg_names: bool,
    pub pseudo_instructions: bool,
}
impl MipsDisassemblyOptions {
    pub fn new(use_reg_names: bool, pseudo_instructions: bool) -> Self {
        Self {
            use_reg_names,
            pseudo_instructions,
        }
    }
}

impl Default for MipsDisassemblyOptions {
    fn default() -> Self {
        Self {
            use_reg_names: true,
            pseudo_instructions: true,
        }
    }
}

// 24a50001        addiu   a1,a1,1
/// takes in a mips machine code and returns a string
/// ```
/// use MIPS_disassembly::get_disassembly;
///
/// let instr: u32 = 0x24a50001;
/// let instr_asm: String = get_disassembly(instr);
/// assert_eq!(instr_asm, "ADDIU $a1, $a1, 1".to_string())
/// ```
pub fn get_disassembly(machine_code: u32) -> String {
    get_disassembly_adv(
        machine_code,
        0,
        &HashMap::new(),
        &MipsDisassemblyOptions::default(),
    )
}
/// Takes in MIPS machine code and return MIPS assembly
/// # Examples
/// ```
/// use MIPS_disassembly::get_disassembly_adv;
/// use MIPS_disassembly::MipsDisassemblyOptions;
/// use std::collections::HashMap;
///
/// let mut sym_tab: HashMap<u32, String> = HashMap::new();
/// sym_tab.insert(0x00000108, "decode_if".into());
/// let instr: u32 = 0x11a0001a;
/// let instr_adrs: u32 = 0x9c;
/// assert_eq!(
///     "BEQ $t5, $zero, 26 <decode_if>",
///     get_disassembly_adv(
///         instr,
///         instr_adrs,
///         &sym_tab,
///         &MipsDisassemblyOptions::new(true, true)
///     )
/// );
/// ```
pub fn get_disassembly_adv(
    machine_code: u32,
    instruction_address: u32,
    symbol_table: &HashMap<u32, String>,
    options: &MipsDisassemblyOptions,
) -> String {
    let reg_names = match options.use_reg_names {
        true => REG_NAMES,
        false => REG_NUMBER,
    };
    let pse = options.pseudo_instructions;

    if machine_code == 0 && pse {
        return "NOP".into();
    }

    let op = (machine_code >> 26) & 0x0000_003f;
    let rs = ((machine_code >> 21) & 0x0000_001f) as usize;
    let rt = ((machine_code >> 16) & 0x0000_001f) as usize;
    let rd = ((machine_code >> 11) & 0x0000_001f) as usize;
    let shamt = ((machine_code >> 6) & 0x0000_001f) as usize;
    let funct = machine_code & 0x0000_003f;
    let immediate: i16 = (machine_code & 0x0000_ffff) as i16;
    let target = machine_code & 0x03ff_ffff;

    //format!(" {}, {}, {}", REG_NAMES[], REG_NAMES[], REG_NAMES[]);

    // match the opcode
    match op {
        OP_0 => match funct {
            FUNCT_SLL => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SLL", reg_names[rd], reg_names[rt], reg_names[shamt]
                )
            }
            FUNCT_SRL => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SRL", reg_names[rd as usize], reg_names[rt], reg_names[shamt]
                )
            }
            FUNCT_SRA => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SRA", reg_names[rd], reg_names[rt], reg_names[shamt]
                )
            }
            FUNCT_SLLV => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SLLV", reg_names[rd], reg_names[rt], reg_names[rs]
                )
            }
            FUNCT_SRLV => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SRLV", reg_names[rd], reg_names[rt], reg_names[rs]
                )
            }
            FUNCT_SRAV => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SRAV", reg_names[rd], reg_names[rt], reg_names[rs]
                )
            }
            FUNCT_JR => {
                format!("{:<6}, {}", "JR", reg_names[rs])
            }
            FUNCT_JALR => {
                if pse && rd == 31 {
                    format!("{:<6}, {}", "JALR", reg_names[rs])
                } else {
                    format!("{:<6}, {:<5}, {}", "JALR", reg_names[rd], reg_names[rs])
                }
            }
            SYSCALL => format!("{:<6}", "SYSCALL").to_owned(),
            FUNCT_ADD => {
                if pse && rt == 0 {
                    format!("{:<6}, {}", "MOVE", reg_names[rd], reg_names[rs])
                } else {
                    format!(
                        "{:<6}, {:<5}, {}",
                        "ADD", reg_names[rd], reg_names[rs], reg_names[rt]
                    )
                }
            }
            FUNCT_ADDU => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "ADDU", reg_names[rd], reg_names[rs], reg_names[rt]
                )
            }
            FUNCT_SUB => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SUB", reg_names[rd], reg_names[rs], reg_names[rt]
                )
            }
            FUNCT_SUBU => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SUBU", reg_names[rd], reg_names[rs], reg_names[rt]
                )
            }
            FUNCT_AND => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "AND", reg_names[rd], reg_names[rs], reg_names[rt]
                )
            }
            FUNCT_OR => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "OR", reg_names[rd], reg_names[rs], reg_names[rt]
                )
            }
            FUNCT_XOR => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "XOR", reg_names[rd], reg_names[rs], reg_names[rt]
                )
            }
            FUNCT_NOR => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "NOR", reg_names[rd], reg_names[rs], reg_names[rt]
                )
            }
            FUNCT_SLT => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SLT", reg_names[rd], reg_names[rs], reg_names[rt]
                )
            }
            FUNCT_SLTU => {
                format!(
                    "{:<6}, {:<5}, {}",
                    "SLTU", reg_names[rd], reg_names[rs], reg_names[rt]
                )
            }
            _ => "not supported argument".to_owned(),
        },
        OP_1 => {
            let b_funct: u32 = (machine_code >> 16) & 0b11111;

            match b_funct {
                B_FUNCT_BGEZ => {
                    format!(
                        "{:<6}, {:<5}",
                        "BGEZ",
                        reg_names[rs],
                        symbol_branch(instruction_address, immediate, symbol_table)
                    )
                }
                B_FUNCT_BLTZ => {
                    format!(
                        "{:<6}, {:<5}",
                        "BLTZ",
                        reg_names[rs],
                        symbol_branch(instruction_address, immediate, symbol_table)
                    )
                }
                B_FUNCT_BGEZAL => {
                    if pse && rs == 0 {
                        format!(
                            "{:<6}, {}",
                            "BAL",
                            symbol_branch(instruction_address, immediate, symbol_table)
                        )
                    } else {
                        format!(
                            "{:<6}, {:<5}",
                            "BGEZAL",
                            reg_names[rs],
                            symbol_branch(instruction_address, immediate, symbol_table)
                        )
                    }
                }
                B_FUNCT_BLTZAL => {
                    format!(
                        "{:<6}, {:<5}",
                        "BLTZAL",
                        reg_names[rs],
                        symbol_branch(instruction_address, immediate, symbol_table)
                    )
                }
                _ => "not supported argument".to_owned(),
            }
        }
        OP_J => {
            format!(
                "{:<6}, {}",
                "J",
                symbol_jump(instruction_address, target, symbol_table)
            )
        }
        OP_JAL => {
            format!(
                "{:<6}, {}",
                "JAL",
                symbol_jump(instruction_address, target, symbol_table)
            )
        }
        OP_BEQ => {
            if pse && rs == 0 && rt == 0 {
                format!(
                    "{:<6}, {}",
                    "B",
                    symbol_branch(instruction_address, immediate, symbol_table)
                )
            } else {
                format!(
                    "{:<6}, {:<5}, {}",
                    "BEQ",
                    reg_names[rs],
                    reg_names[rt],
                    symbol_branch(instruction_address, immediate, symbol_table)
                )
            }
        }
        OP_BNE => {
            format!(
                "{:<6}, {:<5}, {}",
                "BNE",
                reg_names[rs],
                reg_names[rt],
                symbol_branch(instruction_address, immediate, symbol_table)
            )
        }
        OP_BLEZ => {
            format!(
                "{:<6}, {:<5}",
                "BLEZ",
                reg_names[rs],
                symbol_branch(instruction_address, immediate, symbol_table)
            )
        }
        OP_BGTZ => {
            format!(
                "{:<6}, {:<5}",
                "BGTZ",
                reg_names[rs],
                symbol_branch(instruction_address, immediate, symbol_table)
            )
        }
        OP_ADDI => {
            format!(
                "{:<6}, {:<5}, {}",
                "ADDI", reg_names[rt], reg_names[rs], immediate
            )
        }
        OP_ADDIU => {
            format!(
                "{:<6}, {:<5}, {}",
                "ADDIU", reg_names[rt], reg_names[rs], immediate
            )
        }
        OP_SLTI => {
            format!(
                "{:<6}, {:<5}, {}",
                "SLTI", reg_names[rt], reg_names[rs], immediate
            )
        }
        OP_SLTIU => {
            format!(
                "{:<6}, {:<5}, {}",
                "SLTIU", reg_names[rt], reg_names[rs], immediate
            )
        }
        OP_ANDI => {
            format!(
                "{:<6}, {:<5}, {}",
                "ANDI", reg_names[rt], reg_names[rs], immediate
            )
        }
        OP_ORI => {
            format!(
                "{:<6}, {:<5}, {}",
                "ORI", reg_names[rt], reg_names[rs], immediate
            )
        }
        OP_XORI => {
            format!(
                "{:<6}, {:<5}, {}",
                "XORI", reg_names[rt], reg_names[rs], immediate
            )
        }
        OP_LUI => {
            format!(
                "{:<6}, {:<5}, {}",
                "LUI", reg_names[rt], reg_names[rs], immediate
            )
        }
        OP_CP0 => format!("{:<6}", "CP0").to_owned(),
        OP_LB => {
            format!(
                "{:<6}, {:<5}({})",
                "LB", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_LBU => {
            format!(
                "{:<6}, {:<5}({})",
                "LBU", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_LH => {
            format!(
                "{:<6}, {:<5}({})",
                "LH", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_LHU => {
            format!(
                "{:<6}, {:<5}({})",
                "LHU", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_LW => {
            format!(
                "{:<6}, {:<5}({})",
                "LW", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_LWL => {
            format!(
                "{:<6}, {:<5}({})",
                "LWL", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_LWR => {
            format!(
                "{:<6}, {:<5}({})",
                "LWR", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_SB => {
            format!(
                "{:<6}, {:<5}({})",
                "SB", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_SH => {
            format!(
                "{:<6}, {:<5}({})",
                "SH", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_SW => {
            format!(
                "{:<6}, {:<5}({})",
                "SW", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_SWL => {
            format!(
                "{:<6}, {:<5}({})",
                "SWL", reg_names[rt], immediate, reg_names[rs]
            )
        }
        OP_SWR => {
            format!(
                "{:<6}, {:<5}({})",
                "SWR", reg_names[rt], immediate, reg_names[rs]
            )
        }

        _ => "not supported argument".to_owned(),
    }
}

fn calc_branch(cur_adrs: u32, imm: i16) -> u32 {
    cur_adrs
        .overflowing_add(4)
        .0
        .overflowing_add(((imm as i32) as u32).overflowing_shl(2).0)
        .0
}

fn calc_jump(current_adrs: u32, target: u32) -> u32 {
    (current_adrs & 0xf000_0000) | ((target << 2) & 0x0fff_ffff)
}

fn symbol_branch(cur_adrs: u32, imm: i16, symbol_table: &HashMap<u32, String>) -> String {
    let adrs = calc_branch(cur_adrs, imm);
    match symbol_table.get(&adrs) {
        Some(sym) => format!("{} <{}>", imm, sym),
        None => format!("{}", imm),
    }
}

fn symbol_jump(cur_adrs: u32, target: u32, symbol_table: &HashMap<u32, String>) -> String {
    let adrs = calc_jump(cur_adrs, target);
    match symbol_table.get(&adrs) {
        Some(sym) => format!("{} <{}>", target, sym),
        None => format!("{}", target),
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
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
    }
    #[test]
    fn pseudo_instructions_test() {
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
    }
}
