pub mod lexer;

use std::collections::HashMap;

use lexer::{Instruction, Register, Token, TokenKind};
use shared::{Case, Op};

pub struct Parser {
    pub input: Vec<lexer::Token>,
}

#[derive(Debug)]
pub struct ParserError<'a> {
    pub cause: &'a str,
    pub responsible: &'a lexer::Token,
}

impl Into<u8> for Register {
    fn into(self) -> u8 {
        match self {
            Register::N => 0,
            Register::X => 1,
            Register::Y => 2,
            Register::Z => 3,
            Register::A => 4,
            Register::B => 5,
            Register::C => 6,
            Register::I => 7,
        }
    }
}

impl TryInto<u8> for TokenKind {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            TokenKind::Reg(r) => Ok(r.into()),
            _ => Err(()),
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Vec<shared::Op>, ParserError> {
        let mut iter = self.input.iter();
        let mut code = Vec::<Op>::new();
        let mut labels: HashMap<String, u8> = HashMap::new();
        let mut rpoints: Vec<(String, usize, &Token)> = Vec::new();
        while let Some(i) = iter.next() {
            match i.kind {
                TokenKind::Ins(ins) => match ins {
                    Instruction::Halt => {
                        code.push(Op::HALT);
                    }
                    Instruction::Print | Instruction::Shl | Instruction::Shr => {
                        let x = iter.next().ok_or(ParserError {
                            cause: "Missing a register after here",
                            responsible: i,
                        })?;
                        let x: u8 = x.kind.clone().try_into().map_err(|_| ParserError {
                            cause: "Expected a register here",
                            responsible: x,
                        })?;

                        let op = match ins {
                            Instruction::Print => Op::PRINT(x),
                            Instruction::Shl => Op::SHL(x),
                            Instruction::Shr => Op::SHR(x),
                            _ => unreachable!(),
                        };
                        code.push(op);
                    }
                    Instruction::Jmp => {
                        let mut w = iter.next().ok_or(ParserError {
                            cause: "Missing 'to' or 'if' here",
                            responsible: i,
                        })?;
                        let mut case = None;
                        if w.kind == TokenKind::If {
                            let x = iter.next().ok_or(ParserError {
                                cause: "Missing left-hand side of comparison after here",
                                responsible: w,
                            })?;
                            let c = iter.next().ok_or(ParserError {
                                cause: "Missing comparison operator after here",
                                responsible: x,
                            })?;

                            let mut y = iter.next().ok_or(ParserError {
                                cause: "Missing right-hand side of comparison after here",
                                responsible: c,
                            })?;
                            let mut c2 = None;
                            match y.kind.clone() {
                                TokenKind::Reg(_) => {}
                                tk
                                @ (TokenKind::Equal | TokenKind::Greater | TokenKind::Lesser) => {
                                    c2 = Some(tk);
                                    y = iter.next().ok_or(ParserError {
                                        cause: "Missing right-hand side of comparison after here",
                                        responsible: y,
                                    })?;
                                }
                                _ => Err(ParserError {
                                    cause: "Must be a register",
                                    responsible: y,
                                })?,
                            }

                            w = iter.next().ok_or(ParserError {
                                cause: "Missing 'to' after here",
                                responsible: y,
                            })?;

                            let x = x.kind.clone().try_into().map_err(|_| ParserError {
                                cause: "Must be a register",
                                responsible: x,
                            })?;
                            let y = y.kind.clone().try_into().map_err(|_| ParserError {
                                cause: "Must be a register",
                                responsible: y,
                            })?;

                            case = Some(match (c.kind.clone(), c2) {
                                (TokenKind::Greater, None) => Case::GRT(x, y),
                                (TokenKind::Greater, Some(TokenKind::Equal)) => Case::GRTEQ(x, y),
                                (TokenKind::Lesser, None) => Case::LSR(x, y),
                                (TokenKind::Lesser, Some(TokenKind::Equal)) => Case::LSREQ(x, y),
                                (TokenKind::Equal, Some(TokenKind::Equal)) => Case::EQ(x, y),
                                (TokenKind::Exclamation, Some(TokenKind::Equal)) => Case::NEQ(x, y),
                                _ => Err(ParserError {
                                    cause: "Expected a comparison operator here",
                                    responsible: c,
                                })?,
                            });
                        }

                        if w.kind != TokenKind::To {
                            Err(ParserError {
                                cause: "Expected 'to' here",
                                responsible: w,
                            })?
                        }

                        let to = iter.next().ok_or(ParserError {
                            cause: "Missing label after here",
                            responsible: w,
                        })?;
                        match to.kind {
                            TokenKind::Symbol(ref label) => {
                                let off = code.len();
                                let op = match case {
                                    Some(case) => Op::JMPIF(case, 0xEA),
                                    None => Op::JMP(0xEA),
                                };
                                code.push(op);
                                rpoints.push((label.to_owned(), off, to));
                            }
                            _ => Err(ParserError {
                                cause: "Is not a label",
                                responsible: to,
                            })?,
                        }
                    }
                    Instruction::Label => {
                        let w = iter.next().ok_or(ParserError {
                            cause: "Missing 'as' after here",
                            responsible: i,
                        })?;
                        if w.kind != TokenKind::As {
                            Err(ParserError {
                                cause: "Expected 'as' here",
                                responsible: w,
                            })?
                        }
                        let name = iter.next().ok_or(ParserError {
                            cause: "Missing a label name after here",
                            responsible: w,
                        })?;
                        if let TokenKind::Symbol(s) = name.kind.clone() {
                            let addr: usize = code.iter().map(|op| op.get_size()).sum();
                            labels.insert(s, addr.try_into().unwrap());
                        } else {
                            Err(ParserError {
                                cause: "Label name cannot be a number or a keyword",
                                responsible: name,
                            })?;
                        }
                    }
                    Instruction::Or | Instruction::Xor | Instruction::And => {
                        let a = iter.next().ok_or(ParserError {
                            cause: "Missing register after here",
                            responsible: i,
                        })?;
                        let w = iter.next().ok_or(ParserError {
                            cause: "Missing 'with' after here",
                            responsible: a,
                        })?;
                        let b = iter.next().ok_or(ParserError {
                            cause: "Missing register or number after here",
                            responsible: w,
                        })?;

                        let x: u8 = a.kind.clone().try_into().map_err(|_| ParserError {
                            cause: "Expected a register here",
                            responsible: a,
                        })?;

                        if w.kind != TokenKind::With {
                            Err(ParserError {
                                cause: "Expected 'with' here",
                                responsible: w,
                            })?
                        }

                        let op = match b.kind {
                            TokenKind::Number(y) => {
                                let y = y.try_into().map_err(|_| ParserError {
                                    cause: "Integers should be between 0 and 255 (included)",
                                    responsible: b,
                                })?;
                                match ins {
                                    Instruction::Xor => Op::XORRN(x, y),
                                    Instruction::Or => Op::ORRN(x, y),
                                    Instruction::And => Op::ANDRN(x, y),
                                    _ => unreachable!(),
                                }
                            }
                            TokenKind::Reg(r) => match ins {
                                Instruction::Xor => Op::XORRR(x, r.into()),
                                Instruction::Or => Op::ORRR(x, r.into()),
                                Instruction::And => Op::ANDRR(x, r.into()),
                                _ => unreachable!(),
                            },
                            _ => Err(ParserError {
                                cause: "Expected a register or a number here",
                                responsible: b,
                            })?,
                        };
                        code.push(op);
                    }

                    Instruction::Add => {
                        let a = iter.next().ok_or(ParserError {
                            cause: "Missing register or number after here",
                            responsible: i,
                        })?;
                        let w = iter.next().ok_or(ParserError {
                            cause: "Missing 'to' after here",
                            responsible: a,
                        })?;
                        let b = iter.next().ok_or(ParserError {
                            cause: "Missing register after here",
                            responsible: w,
                        })?;

                        let y = b.kind.clone().try_into().map_err(|_| ParserError {
                            cause: "Expected a register here",
                            responsible: b,
                        })?;

                        if w.kind != TokenKind::To {
                            Err(ParserError {
                                cause: "Expected 'to' here",
                                responsible: w,
                            })?
                        }

                        let op = match a.kind {
                            TokenKind::Number(x) => {
                                let x = x.try_into().map_err(|_| ParserError {
                                    cause: "Integers should be between 0 and 255 (included)",
                                    responsible: a,
                                })?;
                                Op::ADDRN(y, x)
                            }
                            TokenKind::Reg(r) => Op::ADDRR(y, r.into()),
                            _ => Err(ParserError {
                                cause: "Expected a register or a number here",
                                responsible: a,
                            })?,
                        };
                        code.push(op);
                    }

                    Instruction::Sub => {
                        let a = iter.next().ok_or(ParserError {
                            cause: "Missing register or number after here",
                            responsible: i,
                        })?;
                        let w = iter.next().ok_or(ParserError {
                            cause: "Missing 'from' after here",
                            responsible: a,
                        })?;
                        let b = iter.next().ok_or(ParserError {
                            cause: "Missing register after here",
                            responsible: w,
                        })?;

                        let y = b.kind.clone().try_into().map_err(|_| ParserError {
                            cause: "Expected a register here",
                            responsible: b,
                        })?;

                        if w.kind != TokenKind::From {
                            Err(ParserError {
                                cause: "Expected 'from' here",
                                responsible: w,
                            })?
                        }

                        let op = match a.kind {
                            TokenKind::Number(x) => {
                                let x = x.try_into().map_err(|_| ParserError {
                                    cause: "Integers should be between 0 and 255 (included)",
                                    responsible: a,
                                })?;
                                Op::SUBRN(y, x)
                            }
                            TokenKind::Reg(r) => Op::SUBRR(y, r.into()),
                            _ => Err(ParserError {
                                cause: "Expected a register or a number here",
                                responsible: a,
                            })?,
                        };
                        code.push(op);
                    }

                    Instruction::Mov => {
                        let mut a = iter.next().ok_or(ParserError {
                            cause: "Missing source after here",
                            responsible: i,
                        })?;
                        let mut deref_a = false;
                        if a.kind == TokenKind::Deref {
                            deref_a = true;
                            a = iter.next().ok_or(ParserError {
                                cause: "Nothing to dereference after here",
                                responsible: a,
                            })?
                        }

                        let w = iter.next().ok_or(ParserError {
                            cause: "Missing 'to' after here",
                            responsible: a,
                        })?;
                        if w.kind != TokenKind::To {
                            Err(ParserError {
                                cause: "Expected 'to' here",
                                responsible: w,
                            })?
                        }

                        let mut b = iter.next().ok_or(ParserError {
                            cause: "Missing destination after here",
                            responsible: w,
                        })?;
                        let mut deref_b = false;
                        if b.kind == TokenKind::Deref {
                            deref_b = true;
                            b = iter.next().ok_or(ParserError {
                                cause: "Nothing to dereference after here",
                                responsible: b,
                            })?
                        }

                        let op = match a.kind {
                            TokenKind::Number(n) => {
                                let n: u8 = n.try_into().map_err(|_| ParserError {
                                    cause: "Integers should be between 0 and 255 (included)",
                                    responsible: a,
                                })?;
                                match b.kind {
                                    TokenKind::Reg(d) => {
                                        let d: u8 = d.into();
                                        if deref_b {
                                            if deref_a {
                                                Op::MOVXA(d, n)
                                            } else {
                                                Op::MOVXN(d, n)
                                            }
                                        } else {
                                            if deref_a {
                                                Op::MOVRA(d, n)
                                            } else {
                                                Op::MOVRN(d, n)
                                            }
                                        }
                                    }
                                    TokenKind::Number(d) if deref_b => {
                                        let d: u8 = d.try_into().map_err(|_| ParserError {
                                            cause:
                                                "Integers should be between 0 and 255 (included)",
                                            responsible: b,
                                        })?;
                                        if deref_a {
                                            Op::MOVAA(d, n)
                                        } else {
                                            Op::MOVAN(d, n)
                                        }
                                    }
                                    _ => Err(ParserError {
                                        cause: "Expected a register or an address here",
                                        responsible: b,
                                    })?,
                                }
                            }
                            TokenKind::Reg(s) => {
                                let s: u8 = s.into();
                                match b.kind {
                                    TokenKind::Reg(d) => {
                                        let d: u8 = d.into();
                                        if deref_b {
                                            if deref_a {
                                                Op::MOVXX(d, s)
                                            } else {
                                                Op::MOVXR(d, s)
                                            }
                                        } else {
                                            if deref_a {
                                                Op::MOVRX(d, s)
                                            } else {
                                                Op::MOVRR(d, s)
                                            }
                                        }
                                    }
                                    TokenKind::Number(d) if deref_b => {
                                        let d: u8 = d.try_into().map_err(|_| ParserError {
                                            cause:
                                                "Integers should be between 0 and 255 (included)",
                                            responsible: b,
                                        })?;
                                        if deref_a {
                                            Op::MOVAX(d, s)
                                        } else {
                                            Op::MOVAR(d, s)
                                        }
                                    }
                                    _ => Err(ParserError {
                                        cause: "Expected a register or an address here",
                                        responsible: b,
                                    })?,
                                }
                            }
                            _ => Err(ParserError {
                                cause: "Expected a register, an address or a number here",
                                responsible: a,
                            })?,
                        };
                        code.push(op);
                    }
                },
                _ => {
                    return Err(ParserError {
                        cause: "Expected an operation or directive here",
                        responsible: &i,
                    })
                }
            }
        }

        for (name, off, token) in rpoints.into_iter() {
            let addr = *labels.get(&name).ok_or(ParserError {
                cause: "Jumping to undefined label",
                responsible: token,
            })?;
            match code.get_mut(off).unwrap() {
                Op::JMP(to) => *to = addr,
                Op::JMPIF(_, to) => *to = addr,
                _ => unreachable!(),
            }
        }

        Ok(code)
    }
}

pub fn to_bytes(ops: Vec<Op>, dest: &mut Vec<u8>) {
    for op in ops.into_iter() {
        dest.push(op.get_opcode());
        match op {
            Op::MOVAA(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVAR(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVAN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVAX(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVXA(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVXR(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVXN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVXX(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVRA(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVRR(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVRN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MOVRX(a, b) => dest.extend_from_slice(&[a, b]),

            Op::ADDRN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::ADDRR(a, b) => dest.extend_from_slice(&[a, b]),

            Op::SUBRN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::SUBRR(a, b) => dest.extend_from_slice(&[a, b]),

            Op::MULRN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::MULRR(a, b) => dest.extend_from_slice(&[a, b]),

            Op::DIVRN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::DIVRR(a, b) => dest.extend_from_slice(&[a, b]),

            Op::ORRN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::ORRR(a, b) => dest.extend_from_slice(&[a, b]),
            Op::ANDRN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::ANDRR(a, b) => dest.extend_from_slice(&[a, b]),
            Op::XORRN(a, b) => dest.extend_from_slice(&[a, b]),
            Op::XORRR(a, b) => dest.extend_from_slice(&[a, b]),

            Op::SHR(x) => dest.push(x),
            Op::SHL(x) => dest.push(x),
            Op::PRINT(x) => dest.push(x),

            Op::JMP(to) => dest.push(to),
            Op::JMPIF(case, to) => {
                dest.push(case.get_opcode());
                match case {
                    Case::EQ(x, y) => dest.extend_from_slice(&[x, y]),
                    Case::NEQ(x, y) => dest.extend_from_slice(&[x, y]),
                    Case::GRT(x, y) => dest.extend_from_slice(&[x, y]),
                    Case::LSR(x, y) => dest.extend_from_slice(&[x, y]),
                    Case::GRTEQ(x, y) => dest.extend_from_slice(&[x, y]),
                    Case::LSREQ(x, y) => dest.extend_from_slice(&[x, y]),
                }
                dest.push(to);
            }

            Op::HALT => {}
            Op::NOOP => {}
        }
    }
}

#[test]
fn test_parser() {
    let lexer = lexer::Lexer {
        input: " add 7 to x label as lol # is our function \n jmp if z > x to lol shr a shl n halt",
    };
    let tokens = lexer.lex();

    let mut parser = Parser { input: tokens };
    let recipe = parser.parse().unwrap();
    println!("{:?}", recipe);

    let mut out = Vec::new();
    to_bytes(recipe, &mut out);
    println!("{:?}", out);
}
