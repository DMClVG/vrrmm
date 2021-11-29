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

    fn fetch(&mut self) -> Option<Op> {
        let ins_code = self.next_byte();

        match ins_code {
            0x00 => Some(Op::NOOP),
            0xFF => Some(Op::HALT),

            0x0E => { 
                Some(Op::MOVRN(self.next_byte(), self.next_byte()))
            },
            0x1E => { 
                Some(Op::MOVRR(self.next_byte(), self.next_byte()))
            },
            0xAE => { 
                Some(Op::MOVRA(self.next_byte(), self.next_byte()))
            },
            0xBE => { 
                Some(Op::MOVRX(self.next_byte(), self.next_byte()))
            },

            0xE1 => { 
                Some(Op::MOVAN(self.next_byte(), self.next_byte()))
            },
            0xE2 => { 
                Some(Op::MOVAR(self.next_byte(), self.next_byte()))
            },
            0xE3 => { 
                Some(Op::MOVAA(self.next_byte(), self.next_byte()))
            },
            0xE4 => { 
                Some(Op::MOVAX(self.next_byte(), self.next_byte()))
            },

            0xEA => { 
                Some(Op::MOVXN(self.next_byte(), self.next_byte()))
            },
            0xEB => { 
                Some(Op::MOVXR(self.next_byte(), self.next_byte()))
            },
            0xEC => { 
                Some(Op::MOVXA(self.next_byte(), self.next_byte()))
            },
            0xED => { 
                Some(Op::MOVXX(self.next_byte(), self.next_byte()))
            },

            0x0A => { 
                Some(Op::ADDRN(self.next_byte(), self.next_byte()))
            },
            0x1A => { 
                Some(Op::ADDRR(self.next_byte(), self.next_byte()))
            },
            0x0B => { 
                Some(Op::SUBRN(self.next_byte(), self.next_byte()))
            },
            0x1B => { 
                Some(Op::SUBRR(self.next_byte(), self.next_byte()))
            },
            0x0C => { 
                Some(Op::MULRN(self.next_byte(), self.next_byte()))
            },
            0x1C => { 
                Some(Op::MULRR(self.next_byte(), self.next_byte()))
            },
            0x0D => { 
                Some(Op::DIVRN(self.next_byte(), self.next_byte()))
            },
            0x1D => { 
                Some(Op::DIVRR(self.next_byte(), self.next_byte()))
            },

            0xC5 => {
                Some(Op::ANDRR(self.next_byte(), self.next_byte()))
            },
            0xC6 => {
                Some(Op::ANDRN(self.next_byte(), self.next_byte()))
            },
            0xD5 => {
                Some(Op::XORRR(self.next_byte(), self.next_byte()))
            },
            0xD6 => {
                Some(Op::XORRN(self.next_byte(), self.next_byte()))
            },
            0xE5 => {
                Some(Op::ORRR(self.next_byte(), self.next_byte()))
            },
            0xE6 => {
                Some(Op::ORRN(self.next_byte(), self.next_byte()))
            },

            0x2D => { 
                Some(Op::SHR(self.next_byte()))
            },
            0x3D => { 
                Some(Op::SHL(self.next_byte()))
            },

            0xA0 => {
                Some(Op::PRINT(self.next_byte()))
            },

            0x0F => {
                Some(Op::JMP(self.next_byte()))
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
                Some(Op::JMPIF(case, self.next_byte()))
            }

            _ => {
                None
            }
        }
    }


    fn execute(&mut self, ins: Op) {
        match ins {
            Op::MOVRN(dest, src) => {
                self.registers[dest as usize] = src as u8;
            },
            Op::MOVRR(dest, src) => {
                self.registers[dest as usize] = self.registers[src as usize];
            },
            Op::MOVRA(dest, src) => {
                self.registers[dest as usize] = self.memory[src as usize];
            },
            Op::MOVRX(dest, src) => {
                self.registers[dest as usize] = self.memory[self.registers[src as usize] as usize];
            },

            Op::MOVAN(dest, src) => {
                self.memory[dest as usize] = src as u8;
            },
            Op::MOVAR(dest, src) => {
                self.memory[dest as usize] = self.registers[src as usize];
            },
            Op::MOVAA(dest, src) => {
                self.memory[dest as usize] = self.memory[src as usize];
            },
            Op::MOVAX(dest, src) => {
                self.memory[dest as usize] = self.memory[self.registers[src as usize] as usize];
            },

            Op::MOVXN(dest, src) => {
                self.memory[self.registers[dest as usize] as usize] = src as u8;
            },
            Op::MOVXR(dest, src) => {
                self.memory[self.registers[dest as usize] as usize] = self.registers[src as usize];
            },
            Op::MOVXA(dest, src) => {
                self.memory[self.registers[dest as usize] as usize] = self.memory[src as usize];
            },
            Op::MOVXX(dest, src) => {
                self.memory[self.registers[dest as usize] as usize] = self.memory[self.registers[src as usize] as usize];
            },

            Op::ADDRN(dest, by) => {
                self.registers[dest as usize] = self.registers[dest as usize].wrapping_add(by);
            },
            Op::ADDRR(dest, src) => {
                self.registers[dest as usize] = self.registers[dest as usize].wrapping_add(self.registers[src as usize]);
            },
            Op::SUBRN(dest, by) => {
                self.registers[dest as usize] = self.registers[dest as usize].wrapping_sub(by);
            },
            Op::SUBRR(dest, src) => {
                self.registers[dest as usize] = self.registers[dest as usize].wrapping_sub(self.registers[src as usize]);
            },

            Op::ANDRR(a, b) => {
                self.registers[a as usize] &= self.registers[b as usize];
            },
            Op::ANDRN(a, b) => {
                self.registers[a as usize] &= b;
            },
            Op::XORRR(a, b) => {
                self.registers[a as usize] ^= self.registers[b as usize];
            },
            Op::XORRN(a, b) => {
                self.registers[a as usize] ^= b;
            },
            Op::ORRR(a, b) => {
                self.registers[a as usize] |= self.registers[b as usize];
            },
            Op::ORRN(a, b) => {
                self.registers[a as usize] |= b;
            },

            Op::SHR(reg) => {
                self.registers[reg as usize] >>= 1;
            },
            Op::SHL(reg) => {
                self.registers[reg as usize] <<= 1;
            },

            Op::PRINT(reg) => {
                print!("{}", self.registers[reg as usize] as char);
            },

            Op::JMP(to) => {
                match self.state {
                    State::Running { ref mut pc } => {
                        *pc = to as usize;
                    },
                    _ => {
                        unreachable!();
                    }
                }
            },

            Op::JMPIF(case, to) => {
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

            Op::HALT => {
                self.state = State::Halted(self.registers[0x6]); // exit code is register c on halt
            }

            Op::NOOP => {},
            
            _ => {
                unimplemented!();
            }
        }
    }

    fn run(&mut self) {
        self.state = State::Running { pc: 0 };
        loop {
            let next = self.fetch();

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

    println!("REGISTERS: {:?}", vm.registers);
}
