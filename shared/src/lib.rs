pub type CAddress = u8;
pub type VAddress = u8;
pub type Register = u8;
pub type Numeral = u8;

#[derive(Debug, Clone, Copy)]
pub enum Case
{
    EQ(Register, Register),
    NEQ(Register, Register),
    LSR(Register, Register),
    GRT(Register, Register),
    LSREQ(Register, Register),
    GRTEQ(Register, Register),
}

impl Case {
    pub fn get_opcode(&self) -> u8 {
        match *self {
            Case::EQ(_, _) => 0x00,
            Case::NEQ(_, _) => 0x01,
            Case::LSR(_, _) => 0x02,
            Case::GRT(_, _) => 0x03,
            Case::LSREQ(_, _) => 0x04,
            Case::GRTEQ(_, _) => 0x05,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    HALT,
    NOOP,

    MOVRN(Register, Numeral),
    MOVRR(Register, Register),
    MOVRA(Register, CAddress),
    MOVRX(Register, VAddress),

    MOVAN(CAddress, Numeral),
    MOVAR(CAddress, Register),
    MOVAA(CAddress, CAddress),
    MOVAX(CAddress, VAddress),

    MOVXN(VAddress, Numeral),
    MOVXR(VAddress, Register),
    MOVXA(VAddress, CAddress),
    MOVXX(VAddress, VAddress),

    ADDRN(Register, Numeral),
    ADDRR(Register, Register),
    SUBRN(Register, Numeral),
    SUBRR(Register, Register),
    MULRN(Register, Numeral),
    MULRR(Register, Register),
    DIVRN(Register, Numeral),
    DIVRR(Register, Register),

    ANDRR(Register, Register),
    ANDRN(Register, Numeral),
    XORRR(Register, Register),
    XORRN(Register, Numeral),
    ORRR(Register, Register),
    ORRN(Register, Numeral),

    SHR(Register),
    SHL(Register),

    PRINT(Register),

    JMP(CAddress),
    JMPIF(Case, CAddress)
}

impl Instruction {
    pub fn get_opcode(&self) -> u8 {
        match *self {
            Instruction::HALT => 0xFF,
            Instruction::NOOP => 0x00,
            
            Instruction::MOVRN(_, _) => 0x0E,
            Instruction::MOVRR(_, _) => 0x1E,
            Instruction::MOVRA(_, _) => 0xAE,
            Instruction::MOVRX(_, _) => 0xBE,
        
            Instruction::MOVAN(_, _) => 0xE1,
            Instruction::MOVAR(_, _) => 0xE2,
            Instruction::MOVAA(_, _) => 0xE3,
            Instruction::MOVAX(_, _) => 0xE4,
        
            Instruction::MOVXN(_, _) => 0xEA,
            Instruction::MOVXR(_, _) => 0xEB,
            Instruction::MOVXA(_, _) => 0xEC,
            Instruction::MOVXX(_, _) => 0xED,
        
            Instruction::ADDRN(_, _) => 0x0A,
            Instruction::ADDRR(_, _) => 0x1A,
            Instruction::SUBRN(_, _) => 0x0B,
            Instruction::SUBRR(_, _) => 0x1B,
            Instruction::MULRN(_, _) => 0x0C,
            Instruction::MULRR(_, _) => 0x1C,
            Instruction::DIVRN(_, _) => 0x0D,
            Instruction::DIVRR(_, _) => 0x1D,

            Instruction::ANDRR(_, _) => 0xC5,
            Instruction::ANDRN(_, _) => 0xC6,
            Instruction::XORRR(_, _) => 0xD5,
            Instruction::XORRN(_, _) => 0xD6,
            Instruction::ORRR(_, _) => 0xE5,
            Instruction::ORRN(_, _) => 0xE6,

            Instruction::SHR(_) => 0x2D,
            Instruction::SHL(_) => 0x3D,

            Instruction::PRINT(_) => 0xA0,
        
            Instruction::JMP(_) => 0x0F,
            Instruction::JMPIF(_, _) => 0x1F,
        }
    }
}
