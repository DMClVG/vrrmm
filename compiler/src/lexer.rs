use std::{ops::Range, str::FromStr};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenKind {
    Deref,
    To,
    From,
    As,
    With,
    If,

    Number(i32),
    Ins(Instruction),
    Reg(Register),

    Lesser,
    Greater,
    Equal,
    Exclamation,

    Symbol(String),
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Instruction {
    Mov,
    Add,
    Sub,
    Shl,
    Shr,
    Jmp,
    Print,
    And,
    Xor,
    Or,
    Label,
    Halt,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    X,
    Y,
    Z,
    I,
    N,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mov" => Ok(Self::Mov),
            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Sub),
            "shl" => Ok(Self::Shl),
            "shr" => Ok(Self::Shr),
            "jmp" => Ok(Self::Jmp),
            "print" => Ok(Self::Print),
            "and" => Ok(Self::And),
            "xor" => Ok(Self::Xor),
            "or" => Ok(Self::Or),
            "label" => Ok(Self::Label),
            "halt" => Ok(Self::Halt),

            _ => Err(()),
        }
    }
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "x" => Ok(Self::X),
            "y" => Ok(Self::Y),
            "z" => Ok(Self::Z),
            "i" => Ok(Self::I),
            "n" => Ok(Self::N),

            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub range: Range<usize>,
}

pub struct Lexer<'a> {
    pub input: &'a str,
}

struct SplitWord<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
    line_pos: usize,
}

impl<'a> SplitWord<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 0,
            line_pos: 0,
        }
    }
}

impl<'a> Iterator for SplitWord<'a> {
    type Item = (&'a str, usize, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            return None;
        }
        let mut range = None;
        for (idx, c) in self.input[self.pos..].char_indices() {
            match c {
                _ if c.is_ascii_punctuation() && c != '_' => {
                    if range.is_some() {
                        break;
                    } else {
                        range = Some(idx..idx + 1);
                        break;
                    }
                }
                _ if c.is_whitespace() => {
                    if range.is_some() {
                        break;
                    } else {
                        if c == '\n' {
                            self.line += 1;
                            self.line_pos = self.pos + idx + 1; // add +1 cuz idk it fix B)
                        }
                        continue;
                    }
                }
                _ => {
                    if let Some(ref mut range) = range {
                        range.end += 1;
                    } else {
                        range = Some(idx..idx + 1);
                    }
                }
            }
        }
        if let Some(range) = range {
            let res = Some((
                &self.input[self.pos..][range.clone()],
                self.line,
                (self.pos + range.start) - self.line_pos..(self.pos + range.end) - self.line_pos,
            ));

            self.pos += range.end;
            res
        } else {
            None
        }
    }
}

impl Lexer<'_> {
    pub fn lex(&self) -> Vec<Token> {
        let mut commented_line = None;
        SplitWord::new(self.input)
            .filter_map(|(word, line, range)| {
                if word == "#" {
                    commented_line = Some(line);
                }
                if commented_line.is_some() && line == commented_line.unwrap() {
                    return None;
                }
                let kind = match word.to_lowercase().as_str() {
                    "$" => TokenKind::Deref,
                    "to" => TokenKind::To,
                    "as" => TokenKind::As,
                    "from" => TokenKind::From,
                    "with" => TokenKind::With,
                    "if" => TokenKind::If,
                    "<" => TokenKind::Lesser,
                    ">" => TokenKind::Greater,
                    "=" => TokenKind::Equal,
                    "!" => TokenKind::Exclamation,
                    _ => {
                        if let Ok(ins) = word.parse::<Instruction>() {
                            TokenKind::Ins(ins)
                        } else if let Ok(reg) = word.parse::<Register>() {
                            TokenKind::Reg(reg)
                        } else if let Ok(num) = word.parse::<i32>() {
                            TokenKind::Number(num)
                        } else {
                            TokenKind::Symbol(word.to_owned())
                        }
                    }
                };
                Some(Token { kind, line, range })
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer_1() {
        let lexer = Lexer {
            input: "mov a to $5  
                    add $32 to x",
        };
        use Instruction::*;
        use Register::*;
        use TokenKind::*;
        assert_eq!(
            lexer
                .lex()
                .into_iter()
                .map(|t| { t.kind })
                .collect::<Vec<TokenKind>>(),
            vec![
                Ins(Mov),
                Reg(A),
                To,
                Deref,
                Number(5),
                Ins(Add),
                Deref,
                Number(32),
                To,
                Reg(X),
            ]
        );
    }

    #[test]
    fn test_splitword_string() {
        let mut splitter = SplitWord::new("\"lol i am  so cool    and u r $+. ,[too ] \"    ");
        assert_eq!("\"", splitter.next().unwrap().0);
        assert_eq!("lol", splitter.next().unwrap().0);
        assert_eq!("i", splitter.next().unwrap().0);
        assert_eq!("am", splitter.next().unwrap().0);
        assert_eq!("so", splitter.next().unwrap().0);
        assert_eq!("cool", splitter.next().unwrap().0);
        assert_eq!("and", splitter.next().unwrap().0);
        assert_eq!("u", splitter.next().unwrap().0);
        assert_eq!("r", splitter.next().unwrap().0);
        assert_eq!("$", splitter.next().unwrap().0);
        assert_eq!("+", splitter.next().unwrap().0);
        assert_eq!(".", splitter.next().unwrap().0);
        assert_eq!(",", splitter.next().unwrap().0);
        assert_eq!("[", splitter.next().unwrap().0);
        assert_eq!("too", splitter.next().unwrap().0);
        assert_eq!("]", splitter.next().unwrap().0);
        assert_eq!("\"", splitter.next().unwrap().0);
    }

    #[test]
    fn test_splitword_math_expression() {
        let mut splitter = SplitWord::new("(4+96* (72+6/ 3) )");
        assert_eq!("(", splitter.next().unwrap().0);
        assert_eq!("4", splitter.next().unwrap().0);
        assert_eq!("+", splitter.next().unwrap().0);
        assert_eq!("96", splitter.next().unwrap().0);
        assert_eq!("*", splitter.next().unwrap().0);
        assert_eq!("(", splitter.next().unwrap().0);
        assert_eq!("72", splitter.next().unwrap().0);
        assert_eq!("+", splitter.next().unwrap().0);
        assert_eq!("6", splitter.next().unwrap().0);
        assert_eq!("/", splitter.next().unwrap().0);
        assert_eq!("3", splitter.next().unwrap().0);
        assert_eq!(")", splitter.next().unwrap().0);
        assert_eq!(")", splitter.next().unwrap().0);
    }
}
