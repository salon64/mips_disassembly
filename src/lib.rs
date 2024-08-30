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
    "$zero", "$at", "$v0", "$v1", "$a0", "$a1", "$a2", "$a3", "$t0", "$t1", "$t2", "$t3", "$t4", "$t5", "$t6",
    "$s7", "$s0", "$s1", "$s2", "$s3", "$s4", "$s5", "$s6", "$s7", "$t8", "$t9", "$k0", "$k1", "$gp", "$sp", "$fp",
    "$ra",
];

const REG_NUMBER: [&str; 32] = [
    "$0","$1","$2","$3","$4","$5","$6","$7","$8","$9","$10","$11","$12","$13","$14","$15","$16","$17","$18","$19","$20","$21","$22","$23","$24","$25","$26","$27","$28","$29","$30","$31"
];
// 24a50001        addiu   a1,a1,1
/// takes in a mips machine code and returns a string
/// ```
/// use MIPS_disassembly::get_dissassembly;
/// 
/// let instr: u32 = 0x24a50001;
/// let instr_asm: String = get_dissassembly(instr);
/// assert_eq!(instr_asm, "ADDIU $a1, $a1, 1".to_string())
/// ```
pub fn get_dissassembly(machine_code: u32) -> String{
    get_dissassembly_adv(machine_code, 0, HashMap::new(),true)
}
/// Takes in MIPS machine code and return MIPS assembly
fn get_dissassembly_adv(machine_code: u32, innstruction_address:u32, symbol_table: HashMap<u32,String>, use_reg_names: bool) -> String{

    let reg_names = match use_reg_names {
        true => REG_NAMES,
        false => REG_NUMBER,
    };

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
            OP_0 => {
                match funct {
                    FUNCT_SLL => {
                        format!("SLL {}, {}, {}", reg_names[rd], reg_names[rt], reg_names[shamt])
                    }
                    FUNCT_SRL => {
                        format!("SRL {}, {}, {}", reg_names[rd as usize], reg_names[rt], reg_names[shamt])
                    }
                    FUNCT_SRA => {
                        format!("SRA {} {} {}", reg_names[rd],reg_names[rt],reg_names[shamt])
                    }
                    FUNCT_SLLV => {
                        format!("SLLV {}, {}, {}", reg_names[rd], reg_names[rt], reg_names[rs])
                    }
                    FUNCT_SRLV => {
                        format!("SRLV {}, {}, {}", reg_names[rd], reg_names[rt], reg_names[rs])
                    }
                    FUNCT_SRAV => {
                        format!("SRAV {}, {}, {}", reg_names[rd], reg_names[rt], reg_names[rs])
                    }
                    FUNCT_JR => {
                        format!("JR {}", reg_names[rs])
                    }
                    FUNCT_JALR => {
                        if rd == 31 {
                            format!("JALR {}", reg_names[rs])
                        }
                        else {
                            format!("JALR {}, {}", reg_names[rd], reg_names[rs])
                        }
                    }
                    SYSCALL => { 
                        "SYSCALL".to_owned()
                    }
                    FUNCT_ADD => {
                        format!("ADD {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    FUNCT_ADDU => {
                        format!("ADDU {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    FUNCT_SUB => {
                        format!("SUB {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    FUNCT_SUBU => {
                        format!("SUBU {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    FUNCT_AND => {
                        format!("AND {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    FUNCT_OR => {
                        format!("OR {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    FUNCT_XOR => {
                        format!("XOR {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    FUNCT_NOR => {
                        format!("NOR {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    FUNCT_SLT => {
                        format!("SLT {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    FUNCT_SLTU => {
                        format!("SLTU {}, {}, {}", reg_names[rd], reg_names[rs], reg_names[rt])
                    }
                    _ => {
                        "not supported argument".to_owned()
                    }
                }
            }
            OP_1 => {
                let b_funct: u32 = (machine_code >> 16) & 0b11111;

                match b_funct {
                    B_FUNCT_BGEZ => {
                        format!("BGEZ {}, {}", reg_names[rs], symbol_branch(innstruction_address, immediate, symbol_table))
                    }
                    B_FUNCT_BLTZ => {
                        format!("BLTZ {}, {}", reg_names[rs], symbol_branch(innstruction_address, immediate, symbol_table))
                    }
                    B_FUNCT_BGEZAL => {
                        format!("BGEZAL {}, {}", reg_names[rs], symbol_branch(innstruction_address, immediate, symbol_table))
                    }
                    B_FUNCT_BLTZAL => {
                        format!("BLTZAL {}, {}", reg_names[rs], symbol_branch(innstruction_address, immediate, symbol_table))
                    }
                    _ => {
                        "not supported argument".to_owned()
                    }
                }
            }
            OP_J => {
                format!("J {}", symbol_jump(innstruction_address, target, symbol_table))
            }
            OP_JAL => {
                format!("JAL {}", symbol_jump(innstruction_address, target, symbol_table))
            }
            OP_BEQ => {
                format!("BEQ {}, {}, {}", reg_names[rs], reg_names[rt], symbol_branch(innstruction_address, immediate, symbol_table))
            }
            OP_BNE => {
                format!("BNE {}, {}, {}", reg_names[rs], reg_names[rt], symbol_branch(innstruction_address, immediate, symbol_table))
            }
            OP_BLEZ => {
                format!("BLEZ {}, {}", reg_names[rs], symbol_branch(innstruction_address, immediate, symbol_table))
            }
            OP_BGTZ => {
                format!("BGTZ {}, {}", reg_names[rs], symbol_branch(innstruction_address, immediate, symbol_table))
            }

            OP_ADDI => {
                format!("ADDI {}, {}, {}", reg_names[rt], reg_names[rs], immediate)
            }
            OP_ADDIU => {
                format!("ADDIU {}, {}, {}", reg_names[rt], reg_names[rs], immediate)
            }
            OP_SLTI => {
                format!("SLTI {}, {}, {}", reg_names[rt], reg_names[rs], immediate)
            }
            OP_SLTIU => {
                format!("SLTIU {}, {}, {}", reg_names[rt], reg_names[rs], immediate)
            }
            OP_ANDI => {
                format!("ANDI {}, {}, {}", reg_names[rt], reg_names[rs], immediate)
            }
            OP_ORI => {
                format!("ORI {}, {}, {}", reg_names[rt], reg_names[rs], immediate)
            }
            OP_XORI => {
                format!("XORI {}, {}, {}", reg_names[rt], reg_names[rs], immediate)
            }
            OP_LUI => {
                format!("LUI {}, {}, {}", reg_names[rt], reg_names[rs], immediate)
            }
            OP_CP0 => { 
                "CP0".to_owned()
            }
            OP_LB => {
                format!("LB {}, {}({})", reg_names[rt], immediate, rs as i16)
            }
            OP_LBU => {
                format!("LBU {}, {}({})", reg_names[rt], immediate, rs as i16)
            }
            OP_LH => {
                format!("LH {}, {}({})", reg_names[rt], immediate, rs as i16)
            }
            OP_LHU => {
                format!("LHU {}, {}({})", reg_names[rt], immediate, rs as i16)
            }
            OP_LW => {
                format!("LW {}, {}({})", reg_names[rt], immediate, rs as i16)
            }

            OP_SB => {
                format!("SB {}, {}({})", reg_names[rt], immediate, rs as i16)
            }
            OP_SH => {
                format!("Sh {}, {}({})", reg_names[rt], immediate, rs as i16)
            } 
            OP_SW => {
                format!("SW {}, {}({})", reg_names[rt], immediate, rs as i16)
            }

            OP_LWL => {
                format!("LWL {}, {}({})", reg_names[rt], immediate, rs as i16)
            } 
            OP_LWR => {
                format!("LWR {}, {}({})", reg_names[rt], immediate, rs as i16)
            } 
            OP_SWL => {
                format!("SWL {}, {}({})", reg_names[rt], immediate, rs as i16)
            }
            OP_SWR => {
                format!("SWR {}, {}({})", reg_names[rt], immediate, rs as i16)
            }

            _ => {
                "not supported argument".to_owned()
            }
        }

}


fn calc_branch(cur_adrs: u32, imm: i16) -> u32{
    cur_adrs.overflowing_add(4).0.overflowing_add(((imm as i32) as u32).overflowing_shl(2).0).0
}

fn calc_jump(cur_adrs: u32, target: u32) -> u32{
    (cur_adrs & 0xf000_0000) | ((target << 2) & 0x0fff_ffff)
}

fn symbol_branch(cur_adrs: u32, imm: i16, symbol_table: HashMap<u32,String>) -> String {
    let adrs = calc_branch(cur_adrs, imm);
    match symbol_table.get(&adrs) {
        Some(sym) => format!("{} <{}>", imm, sym),
        None => format!("{}",imm),
    }
}

fn symbol_jump(cur_adrs: u32, target: u32, symbol_table: HashMap<u32,String>) -> String {
    let adrs = calc_jump(cur_adrs, target);
    match symbol_table.get(&adrs) {
        Some(sym) => format!("{} <{}>", target, sym),
        None => format!("{}",target),
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test(){
        let mut sym_tab: HashMap<u32, String> = HashMap::new();
        sym_tab.insert(0x00000108, "decode_if".into());
        let instr: u32 = 0x11a0001a; 
        let instr_adrs: u32 = 0x9c;
        assert_eq!("BEQ $t5, $zero, 26 <decode_if>",get_dissassembly_adv(instr, instr_adrs, sym_tab, true));
    }
}





