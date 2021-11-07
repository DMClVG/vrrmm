use std::vec;

use shared::*;

#[derive(Debug)]
enum State {
    Running {pc: usize},
    Halted(u8), // error code
    Null,
}
struct Machine {
   registers: Vec<u8>,
   memory: Vec<u8>,
   state: State
}

impl Machine {
    fn new(code: Vec<u8>) -> Self {
        let mut machine = Self {
            registers: vec![0u8; 8],
            memory: vec![0u8; 256],
            state: State::Null,
        };
        machine.memory[..code.len()].copy_from_slice(code.as_slice());
        machine
    }
    
    fn next_byte(&mut self) -> u8 {
        if let State::Running { ref mut pc } = self.state {
            let val = self.memory[*pc];
            *pc += 1;
            val
        } else {
            unimplemented!()
        }
    }

    fn next_ins(&mut self) -> Option<Instruction> {
        let ins_code = self.next_byte();

        match ins_code {
            0x00 => Some(Instruction::NOOP),
            0xFF => Some(Instruction::HALT),

            0x0E => { 
                Some(Instruction::MOVRN(self.next_byte(), self.next_byte()))
            },
            0x1E => { 
                Some(Instruction::MOVRR(self.next_byte(), self.next_byte()))
            },
            0xAE => { 
                Some(Instruction::MOVRA(self.next_byte(), self.next_byte()))
            },
            0xBE => { 
                Some(Instruction::MOVRX(self.next_byte(), self.next_byte()))
            },

            0xE1 => { 
                Some(Instruction::MOVAN(self.next_byte(), self.next_byte()))
            },
            0xE2 => { 
                Some(Instruction::MOVAR(self.next_byte(), self.next_byte()))
            },
            0xE3 => { 
                Some(Instruction::MOVAA(self.next_byte(), self.next_byte()))
            },
            0xE4 => { 
                Some(Instruction::MOVAX(self.next_byte(), self.next_byte()))
            },

            0xEA => { 
                Some(Instruction::MOVXN(self.next_byte(), self.next_byte()))
            },
            0xEB => { 
                Some(Instruction::MOVXR(self.next_byte(), self.next_byte()))
            },
            0xEC => { 
                Some(Instruction::MOVXA(self.next_byte(), self.next_byte()))
            },
            0xED => { 
                Some(Instruction::MOVXX(self.next_byte(), self.next_byte()))
            },

            0x0A => { 
                Some(Instruction::ADDRN(self.next_byte(), self.next_byte()))
            },
            0x1A => { 
                Some(Instruction::ADDRR(self.next_byte(), self.next_byte()))
            },
            0x0B => { 
                Some(Instruction::SUBRN(self.next_byte(), self.next_byte()))
            },
            0x1B => { 
                Some(Instruction::SUBRR(self.next_byte(), self.next_byte()))
            },
            0x0C => { 
                Some(Instruction::MULRN(self.next_byte(), self.next_byte()))
            },
            0x1C => { 
                Some(Instruction::MULRR(self.next_byte(), self.next_byte()))
            },
            0x0D => { 
                Some(Instruction::DIVRN(self.next_byte(), self.next_byte()))
            },
            0x1D => { 
                Some(Instruction::DIVRR(self.next_byte(), self.next_byte()))
            },

            0xC5 => {
                Some(Instruction::ANDRR(self.next_byte(), self.next_byte()))
            },
            0xC6 => {
                Some(Instruction::ANDRN(self.next_byte(), self.next_byte()))
            },
            0xD5 => {
                Some(Instruction::XORRR(self.next_byte(), self.next_byte()))
            },
            0xD6 => {
                Some(Instruction::XORRN(self.next_byte(), self.next_byte()))
            },
            0xE5 => {
                Some(Instruction::ORRR(self.next_byte(), self.next_byte()))
            },
            0xE6 => {
                Some(Instruction::ORRN(self.next_byte(), self.next_byte()))
            },

            0x2D => { 
                Some(Instruction::SHR(self.next_byte()))
            },
            0x3D => { 
                Some(Instruction::SHL(self.next_byte()))
            },

            0xA0 => {
                Some(Instruction::PRINT(self.next_byte()))
            },

            0x0F => {
                Some(Instruction::JMP(self.next_byte()))
            }
            0x1F => {
                let case: Case = match self.next_byte() {
                    0x00 => Case::EQ(self.next_byte(), self.next_byte()),
                    0x01 => Case::NEQ(self.next_byte(), self.next_byte()),
                    0x02 => Case::LSR(self.next_byte(), self.next_byte()),
                    0x03 => Case::GRT(self.next_byte(), self.next_byte()),
                    0x04 => Case::LSREQ(self.next_byte(), self.next_byte()),
                    0x05 => Case::GRTEQ(self.next_byte(), self.next_byte()),
                    _ => {
                        unimplemented!()
                    }
                };
                Some(Instruction::JMPIF(case, self.next_byte()))
            }

            _ => {
                None
            }
        }
    }


    fn execute(&mut self, ins: Instruction) {
        match ins {
            Instruction::MOVRN(dest, src) => {
                self.registers[dest as usize] = src as u8;
            },
            Instruction::MOVRR(dest, src) => {
                self.registers[dest as usize] = self.registers[src as usize];
            },
            Instruction::MOVRA(dest, src) => {
                self.registers[dest as usize] = self.memory[src as usize];
            },
            Instruction::MOVRX(dest, src) => {
                self.registers[dest as usize] = self.memory[self.registers[src as usize] as usize];
            },

            Instruction::MOVAN(dest, src) => {
                self.memory[dest as usize] = src as u8;
            },
            Instruction::MOVAR(dest, src) => {
                self.memory[dest as usize] = self.registers[src as usize];
            },
            Instruction::MOVAA(dest, src) => {
                self.memory[dest as usize] = self.memory[src as usize];
            },
            Instruction::MOVAX(dest, src) => {
                self.memory[dest as usize] = self.memory[self.registers[src as usize] as usize];
            },

            Instruction::MOVXN(dest, src) => {
                self.memory[self.registers[dest as usize] as usize] = src as u8;
            },
            Instruction::MOVXR(dest, src) => {
                self.memory[self.registers[dest as usize] as usize] = self.registers[src as usize];
            },
            Instruction::MOVXA(dest, src) => {
                self.memory[self.registers[dest as usize] as usize] = self.memory[src as usize];
            },
            Instruction::MOVXX(dest, src) => {
                self.memory[self.registers[dest as usize] as usize] = self.memory[self.registers[src as usize] as usize];
            },

            Instruction::ADDRN(dest, by) => {
                self.registers[dest as usize] = self.registers[dest as usize].wrapping_add(by);
            },
            Instruction::ADDRR(dest, src) => {
                self.registers[dest as usize] = self.registers[dest as usize].wrapping_add(self.registers[src as usize]);
            },
            Instruction::SUBRN(dest, by) => {
                self.registers[dest as usize] = self.registers[dest as usize].wrapping_sub(by);
            },
            Instruction::SUBRR(dest, src) => {
                self.registers[dest as usize] = self.registers[dest as usize].wrapping_sub(self.registers[src as usize]);
            },

            Instruction::ANDRR(a, b) => {
                self.registers[a as usize] &= self.registers[b as usize];
            },
            Instruction::ANDRN(a, b) => {
                self.registers[a as usize] &= b;
            },
            Instruction::XORRR(a, b) => {
                self.registers[a as usize] ^= self.registers[b as usize];
            },
            Instruction::XORRN(a, b) => {
                self.registers[a as usize] ^= b;
            },
            Instruction::ORRR(a, b) => {
                self.registers[a as usize] |= self.registers[b as usize];
            },
            Instruction::ORRN(a, b) => {
                self.registers[a as usize] |= b;
            },

            Instruction::SHR(reg) => {
                self.registers[reg as usize] >>= 1;
            },
            Instruction::SHL(reg) => {
                self.registers[reg as usize] <<= 1;
            },

            Instruction::PRINT(reg) => {
                print!("{}", self.registers[reg as usize] as char);
            },

            Instruction::JMP(to) => {
                match self.state {
                    State::Running { ref mut pc } => {
                        *pc = to as usize;
                    },
                    _ => {
                        unreachable!();
                    }
                }
            },

            Instruction::JMPIF(case, to) => {
                match self.state {
                    State::Running { ref mut pc } => {
                        
                        let is_true = match case {
                            Case::EQ(a, b) => {
                                self.registers[a as usize] == self.registers[b as usize]
                            },
                            Case::NEQ(a, b) => {
                                self.registers[a as usize] != self.registers[b as usize]
                            },
                            Case::GRT(a, b) => {
                                self.registers[a as usize] > self.registers[b as usize]
                            },
                            Case::LSR(a, b) => {
                                self.registers[a as usize] < self.registers[b as usize]
                            },
                            Case::GRTEQ(a, b) => {
                                self.registers[a as usize] >= self.registers[b as usize]
                            },
                            Case::LSREQ(a, b) => {
                                self.registers[a as usize] <= self.registers[b as usize]
                            },
                        };
                        if is_true {
                            *pc = to as usize;
                        }
                    },
                    _ => {
                        unreachable!();
                    }
                }
            },

            Instruction::HALT => {
                self.state = State::Halted(self.registers[0x6]); // exit code is register c on halt
            }

            Instruction::NOOP => {},
            
            _ => {
                unimplemented!();
            }
        }
    }

    fn run(&mut self) {
        self.state = State::Running { pc: 0 };
        loop {
            let next = self.next_ins();

            if let Some(ins) = next {
                // println!("{:?}", ins);
                self.execute(ins);
            }
            if let State::Halted(error) = self.state {
                println!("\nVM HALTED. EXIT CODE: {}", error);
                break;
            } else if let State::Running { pc } = self.state {
                if pc == self.memory.len() {
                    println!("\nVM HALTED. REACHED EOF");
                    break;
                }
            }
        }
    }
}



use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct ClArgs {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}

fn main() {
    let args = ClArgs::from_args();

    let mut vm = Machine::new(std::fs::read(args.input).expect("Unable to read input file"));
    vm.run();
}
