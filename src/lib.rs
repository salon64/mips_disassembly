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
    "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1", "t2", "t3", "t4", "t5", "t6",
    "s7", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp", "fp",
    "ra",
];



/// Takes in MIPS machine code and return MIPS assembly
pub fn get_dissassembly(machine_code: u32) -> String{

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
                        format!("SLL {}, {}, {}", REG_NAMES[rd], REG_NAMES[rt], REG_NAMES[shamt])
                    }
                    FUNCT_SRL => {
                        format!("SRL {}, {}, {}", REG_NAMES[rd as usize], REG_NAMES[rt], REG_NAMES[shamt])
                    }
                    FUNCT_SRA => {
                        format!("SRA {} {} {}", REG_NAMES[rd],REG_NAMES[rt],REG_NAMES[shamt])
                    }
                    FUNCT_SLLV => {
                        format!("SLLV {}, {}, {}", REG_NAMES[rd], REG_NAMES[rt], REG_NAMES[rs])
                    }
                    FUNCT_SRLV => {
                        format!("SRLV {}, {}, {}", REG_NAMES[rd], REG_NAMES[rt], REG_NAMES[rs])
                    }
                    FUNCT_SRAV => {
                        format!("SRAV {}, {}, {}", REG_NAMES[rd], REG_NAMES[rt], REG_NAMES[rs])
                    }
                    FUNCT_JR => {
                        format!("JR {}", REG_NAMES[rs])
                    }
                    FUNCT_JALR => {
                        if rd == 31 {
                            format!("JALR {}", REG_NAMES[rs])
                        }
                        else {
                            format!("JALR {}, {}", REG_NAMES[rd], REG_NAMES[rs])
                        }
                    }
                    SYSCALL => { 
                        "SYSCALL".to_owned()
                    }
                    FUNCT_ADD => {
                        format!("ADD {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
                    }
                    FUNCT_ADDU => {
                        format!("ADDU {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
                    }
                    FUNCT_SUB => {
                        format!("SUB {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
                    }
                    FUNCT_SUBU => {
                        format!("SUBU {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
                    }
                    FUNCT_AND => {
                        format!("AND {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
                    }
                    FUNCT_OR => {
                        format!("OR {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
                    }
                    FUNCT_XOR => {
                        format!("XOR {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
                    }
                    FUNCT_NOR => {
                        format!("NOR {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
                    }
                    FUNCT_SLT => {
                        format!("SLT {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
                    }
                    FUNCT_SLTU => {
                        format!("SLTU {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs], REG_NAMES[rt])
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
                        format!("BGEZ {}, {}", REG_NAMES[rs], immediate)
                    }
                    B_FUNCT_BLTZ => {
                        format!("BLTZ {}, {}", REG_NAMES[rs], immediate)
                    }
                    B_FUNCT_BGEZAL => {
                        format!("BGEZAL {}, {}", REG_NAMES[rs], immediate)
                    }
                    B_FUNCT_BLTZAL => {
                        format!("BLTZAL {}, {}", REG_NAMES[rs], immediate)
                    }
                    _ => {
                        "not supported argument".to_owned()
                    }
                }
            }
            OP_J => {
                format!("J {}", target)
            }
            OP_JAL => {
                format!("JAL {}", target)
            }
            OP_BEQ => {
                format!("BEQ {}, {}, {}", REG_NAMES[rs], REG_NAMES[rt], immediate)
            }
            OP_BNE => {
                format!("BNE {}, {}, {}", REG_NAMES[rs], REG_NAMES[rt], immediate)
            }
            OP_BLEZ => {
                format!("BLEZ {}, {}", REG_NAMES[rs], immediate)
            }
            OP_BGTZ => {
                format!("BGTZ {}, {}", REG_NAMES[rs], immediate)
            }

            OP_ADDI => {
                format!("ADDI {}, {}, {}", REG_NAMES[rt], REG_NAMES[rs], immediate)
            }
            OP_ADDIU => {
                format!("ADDIU {}, {}, {}", REG_NAMES[rt], REG_NAMES[rs], immediate)
            }
            OP_SLTI => {
                format!("SLTI {}, {}, {}", REG_NAMES[rt], REG_NAMES[rs], immediate)
            }
            OP_SLTIU => {
                format!("SLTIU {}, {}, {}", REG_NAMES[rt], REG_NAMES[rs], immediate)
            }
            OP_ANDI => {
                format!("ANDI {}, {}, {}", REG_NAMES[rt], REG_NAMES[rs], immediate)
            }
            OP_ORI => {
                format!("ORI {}, {}, {}", REG_NAMES[rt], REG_NAMES[rs], immediate)
            }
            OP_XORI => {
                format!("XORI {}, {}, {}", REG_NAMES[rt], REG_NAMES[rs], immediate)
            }
            OP_LUI => {
                format!("LUI {}, {}, {}", REG_NAMES[rt], REG_NAMES[rs], immediate)
            }
            OP_CP0 => { 
                "CP0".to_owned()
            }
            OP_LB => {
                format!("LB {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            }
            OP_LBU => {
                format!("LBU {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            }
            OP_LH => {
                format!("LH {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            }
            OP_LHU => {
                format!("LHU {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            }
            OP_LW => {
                format!("LW {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            }

            OP_SB => {
                format!("SB {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            }
            OP_SH => {
                format!("Sh {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            } 
            OP_SW => {
                format!("SW {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            }

            OP_LWL => {
                format!("LWL {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            } 
            OP_LWR => {
                format!("LWR {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            } 
            OP_SWL => {
                format!("SWL {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            }
            OP_SWR => {
                format!("SWR {}, {}({})", REG_NAMES[rt], immediate, rs as i16)
            }

            _ => {
                "not supported argument".to_owned()
            }
        }

}






