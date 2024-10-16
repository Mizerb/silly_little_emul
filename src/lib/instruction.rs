

enum instructions{
    
}

pub trait Instruction {
    fn execute(&self, cpu: &mut CPU, opCode: &u32) -> u8;
    fn get_tcycles(&self) -> u8;
}

// 8 bit arithmetic

pub struct ADC;

impl Instruction for ADC {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct ADD;

impl Instruction for ADD {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct AND;

impl Instruction for AND {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct CP;

impl Instruction for CP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct DEC;

impl Instruction for DEC {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct INC;

impl Instruction for INC {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct OR;

impl Instruction for OR {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct SBC;

impl Instruction for SBC {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct SUB;

impl Instruction for SUB {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct XOR;

impl Instruction for XOR {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

// 16 bit arithmetic

pub struct ADD_16;

impl Instruction for ADD_16 {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct DEC_16;

impl Instruction for DEC_16 {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct INC_16;

impl Instruction for INC_16 {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

// 8 Bit Operations Instructions

pub struct BIT;

impl Instruction for BIT {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RES;

impl Instruction for RES {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct SET;

impl Instruction for SET {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct SWAP;

impl Instruction for SWAP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

// Bit shift Instructions 

pub struct RL;

impl Instruction for RL {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RLA;

impl Instruction for RLA {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RLC;

impl Instruction for RLC {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RLCA;

impl Instruction for RLCA {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RR;

impl Instruction for RR {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RRA;

impl Instruction for RRA {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RRC;

impl Instruction for RRC {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RRCA;

impl Instruction for RRCA {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct SLA;

impl Instruction for SLA {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct SRA;

impl Instruction for SRA {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct SRL;

impl Instruction for SRL {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

// Load Instructions

pub struct LD;

impl Instruction for LD {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct LDH;

impl Instruction for LDH {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

// Jump & Subrouts

pub struct CALL;

impl Instruction for CALL {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct JP;

impl Instruction for JP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct JR;

impl Instruction for JR {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RET;

impl Instruction for RET {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RETI;

impl Instruction for RETI {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct RST;

impl Instruction for RST {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

// Stack Instructions

pub struct ADD_SP;

impl Instruction for ADD_SP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct DEC_SP;

impl Instruction for DEC_SP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct INC_SP;

impl Instruction for INC_SP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct LD_SP;

impl Instruction for LD_SP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct POP; 

impl Instruction for POP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct PUSH;

impl Instruction for PUSH {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

// Other Miscelaneous Instructions that are fun to write 

pub struct CCF;

impl Instruction for CCF {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct CPL;

impl Instruction for CPL {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct DAA;

impl Instruction for DAA {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct DI;

impl Instruction for DI {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct EI;

impl Instruction for EI {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct HALT;

impl Instruction for HALT {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct NOP;

impl Instruction for NOP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct SCF;

impl Instruction for SCF {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}

pub struct STOP;

impl Instruction for STOP {
    fn execute(&self, cpu: &mut CPU, opCode :&u32) -> u8 {
        0
    }

    fn get_tcycles(&self) -> u8 {
        4
    }
}