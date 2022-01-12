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
pub enum Op {
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

impl Op {
    pub fn get_opcode(&self) -> u8 {
        match *self {
            Op::HALT => 0xFF,
            Op::NOOP => 0x00,
            
            Op::MOVRN(_, _) => 0x0E,
            Op::MOVRR(_, _) => 0x1E,
            Op::MOVRA(_, _) => 0xAE,
            Op::MOVRX(_, _) => 0xBE,
        
            Op::MOVAN(_, _) => 0xE1,
            Op::MOVAR(_, _) => 0xE2,
            Op::MOVAA(_, _) => 0xE3,
            Op::MOVAX(_, _) => 0xE4,
        
            Op::MOVXN(_, _) => 0xEA,
            Op::MOVXR(_, _) => 0xEB,
            Op::MOVXA(_, _) => 0xEC,
            Op::MOVXX(_, _) => 0xED,
        
            Op::ADDRN(_, _) => 0x0A,
            Op::ADDRR(_, _) => 0x1A,
            Op::SUBRN(_, _) => 0x0B,
            Op::SUBRR(_, _) => 0x1B,
            Op::MULRN(_, _) => 0x0C,
            Op::MULRR(_, _) => 0x1C,
            Op::DIVRN(_, _) => 0x0D,
            Op::DIVRR(_, _) => 0x1D,

            Op::ANDRR(_, _) => 0xC5,
            Op::ANDRN(_, _) => 0xC6,
            Op::XORRR(_, _) => 0xD5,
            Op::XORRN(_, _) => 0xD6,
            Op::ORRR(_, _) => 0xE5,
            Op::ORRN(_, _) => 0xE6,

            Op::SHR(_) => 0x2D,
            Op::SHL(_) => 0x3D,

            Op::PRINT(_) => 0xA0,
        
            Op::JMP(_) => 0x0F,
            Op::JMPIF(_, _) => 0x1F,
        }
    }

    pub fn get_size(&self) -> usize {
        match *self {
            Op::HALT => 1,
            Op::NOOP => 1,
            
            Op::MOVRN(_, _) => 3,
            Op::MOVRR(_, _) => 3,
            Op::MOVRA(_, _) => 3,
            Op::MOVRX(_, _) => 3,
        
            Op::MOVAN(_, _) => 3,
            Op::MOVAR(_, _) => 3,
            Op::MOVAA(_, _) => 3,
            Op::MOVAX(_, _) => 3,
        
            Op::MOVXN(_, _) => 3,
            Op::MOVXR(_, _) => 3,
            Op::MOVXA(_, _) => 3,
            Op::MOVXX(_, _) => 3,
        
            Op::ADDRN(_, _) => 3,
            Op::ADDRR(_, _) => 3,
            Op::SUBRN(_, _) => 3,
            Op::SUBRR(_, _) => 3,
            Op::MULRN(_, _) => 3,
            Op::MULRR(_, _) => 3,
            Op::DIVRN(_, _) => 3,
            Op::DIVRR(_, _) => 3,

            Op::ANDRR(_, _) => 3,
            Op::ANDRN(_, _) => 3,
            Op::XORRR(_, _) => 3,
            Op::XORRN(_, _) => 3,
            Op::ORRR(_, _) => 3,
            Op::ORRN(_, _) => 3,

            Op::SHR(_) => 2,
            Op::SHL(_) => 2,

            Op::PRINT(_) => 2,
        
            Op::JMP(_) => 2,
            Op::JMPIF(_, _) => 5,
        }
    }
}
