import re # regular expressions
import numpy as np
import argparse

w_regs = "xyzabci"
regs = w_regs + "n"

lexical_map = {
    
    "add": r"^(-?\d+|[{}])\s+to\s+([{}])$".format(regs, w_regs),
    "sub": r"^(-?\d+|[{}])\s+from\s+([{}])$".format(regs, w_regs),

    "shr": r"^([{}])$".format(w_regs),
    "shl": r"^([{}])$".format(w_regs),

    "div": r"^([{}])\s+by\s+(-?\d+|[{}])$".format(w_regs, regs),
    "mul": r"^([{}])\s+by\s+(-?\d+|[{}])$".format(w_regs, regs),

    "and": r"^([{}])\s+with\s+(-?\d+|[{}])$".format(w_regs, regs),
    "xor": r"^([{}])\s+with\s+(-?\d+|[{}])$".format(w_regs, regs),
    "or": r"^([{}])\s+with\s+(-?\d+|[{}])$".format(w_regs, regs),

    #"mov": r"^([-$]?\d+|$?[{}])\s+to\s+($?\d+|$?[{}])$".format(regs, w_regs),

    "mov": r"^([-$]?\d+|[$]?[{}])\s+to\s+([$][\d]+|[$]?[{}])$".format(regs, w_regs),

    # "mov": r"^(?:(-?\d+|[{0}])\s+(to)\s+(\d+|[{}])|([{}])\s+(from)\s+\$(\d+|[{}]))$".format(regs, w_regs),
    "jmp": r"^(?:if\s+([{}])\s+(==|!=|<|>|<=|>=)\s+([{}])\s+)?to\s+(\w+)$".format(regs, regs),

    "print": r"^([{}])$".format(regs),

    "label": r"^as\s+(\w+)$",
    # "alias": r"^(-?\d+|[{}])\s+as\s+(\w+)$",
    "halt": r"^$",
}

# r: register | n: numeral | a: constant address | x: varying address
ins_map = {
    "halt": 0xFF,
    "noop": 0x00,
    
    "movrn": 0x0E,
    "movrr": 0x1E,
    "movra": 0xAE,
    "movrx": 0xBE,

    "movan": 0xE1,
    "movar": 0xE2,
    "movaa": 0xE3,
    "movax": 0xE4,

    "movxn": 0xEA,
    "movxr": 0xEB,
    "movxa": 0xEC,
    "movxx": 0xED,

    "addrn": 0x0A,
    "addrr": 0x1A,
    "subrn": 0x0B,
    "subrr": 0x1B,
    "mulrn": 0x0C,
    "mulrr": 0x1C,
    "divrn": 0x0D,
    "divrr": 0x1D,

    "andrr": 0xC5,
    "andrn": 0xC6,
    "xorrr": 0xD5,
    "xorrn": 0xD6,
    "orrr": 0xE5,
    "orrn": 0xE6,

    "shr": 0x2D,
    "shl": 0x3D,

    "print": 0xA0,

    "jmp": 0x0F,
    "jmpif": 0x1F,

    # "printr": 0xAA,
    # "printn": 0xAB,
    # "printc": 0xAC
}

cmp_map = {
    "==": 0x00,
    "!=": 0x01,
    "<": 0x02,
    ">": 0x03,
    "<=": 0x04,
    ">=": 0x05,
}

reg_map = {
    'x': 0x1,
    'y': 0x2,
    'z': 0x3,
    'a': 0x4,
    'b': 0x5,
    'c': 0x6,
    'i': 0x7,
    'n': 0x0
}

class SyntaxError(Exception):
    line = 0
    pass

class Lexer:
    def __init__(self, input):
        self.input = input

    def tokenize(self):
        lines = self.input.splitlines()

        tokens = []
        for idx in range(len(lines)):
            line = lines[idx]
            line = line.split("#", 1)[0] # remove comments
            if not re.match(r"^\s*$", line):
                try:
                    tokens.append(self.tokenize_line(line))
                except SyntaxError as e:
                    e.line = idx + 1
                    raise e
                    return
        return tokens

    def tokenize_line(self, line):
        line = line.strip().lower()
        match = re.match(r"^({})(?:\s+(.*))?$".format("|".join(lexical_map)), line)
        if match == None: raise SyntaxError("Unknown keyword")

        ins = match.groups()[0]
        args = ""
        if match.groups()[1] != None:
            args = match.groups()[1].strip()

        params = re.match(lexical_map[ins], args)
        if params == None:
            raise SyntaxError("Invalid params")
        
        return (ins, list(filter(None, params.groups())))

class ParserError(Exception):
    pass

class Parser:
    def __init__(self, lines):
        self.lines = lines

    def parse(self):
        def parse_num(str, signed: bool = False):
            try:
                val = int(str)
            except ValueError:
                raise ParserError("Value is not a number")
            if signed and not val in range(-128, 128):
                raise ParserError("Signed value must range from -128 to 127")
            if not signed and not val in range(0, 256):
                raise ParserError("Unsigned value must range from 0 to 255")
            return val

        def is_reg(str):
            return regs.count(str) == 1

        label_lookup = {}
        labels = {}

        binary = np.array([], dtype=np.uint8)
        for tokens in self.lines:
            verb = tokens[0]
            params = tokens[1]
            if verb == "add" or verb == "sub":
                opcode = str(verb)
                one = params[0]
                two = reg_map[params[1]]

                opcode += "r"
                if is_reg(one):
                    one = reg_map[one]
                    opcode += "r"
                else:
                    one = parse_num(one, False)
                    opcode += "n"

                binary = np.append(binary, [ins_map[opcode], two, one])
            elif verb == "shr" or verb == "shl":
                opcode = str(verb)
                register = reg_map[params[0]]

                binary = np.append(binary, [ins_map[opcode], register])
            elif verb == "mul" or verb == "div" or verb == "and" or verb == "xor" or verb == "or":
                opcode = str(verb)
                one = reg_map[params[0]]
                two = params[1]

                opcode += "r"
                if is_reg(two):
                    two = reg_map[two]
                    opcode += "r"
                else:
                    two = parse_num(two)
                    if verb == "div" and two == 0:
                        raise ParserError("Cannot divide by 0")
                    opcode += "n"
                
                binary = np.append(binary, [ins_map[opcode], one, two])
            elif verb == "mov":
                opcode = str(verb)
                one = params[0]
                two = params[1]

                if two.startswith("$"):
                    two = two[1:]
                    if is_reg(two):
                        two = reg_map[two]
                        opcode += "x"
                    else:
                        two = parse_num(two, False)
                        opcode += "a"
                else:
                    two = reg_map[two]
                    opcode += "r"

                if one.startswith("$"):
                    one = one[1:]
                    if is_reg(one):
                        one = reg_map[one]
                        opcode += "x"
                    else:
                        one = parse_num(one, False)
                        opcode += "a"
                else:
                    if is_reg(one):
                        one = reg_map[one]
                        opcode += "r"
                    else:
                        one = parse_num(one)
                        opcode += "n"
                binary = np.append(binary, [ins_map[opcode], two, one])
            elif verb == "print":
                reg = reg_map[params[0]]
                binary = np.append(binary, [ins_map["print"], reg])
            elif verb == "label":
                if params[0] in labels:
                    raise ParserError("Label '%s' already defined" % params[0])
                labels[params[0]] = np.size(binary)
            elif verb == "jmp":
                if len(params) == 1:
                    binary = np.append(binary, [ins_map["jmp"], 0xEA])
                else:
                    one = reg_map[params[0]]
                    op = cmp_map[params[1]]
                    two = reg_map[params[2]]
                    binary = np.append(binary, [ins_map["jmpif"], op, one, two, 0xEA])
                label = params[-1]
                if not label in label_lookup:
                    label_lookup[label] = []
                label_lookup[label].append(np.size(binary) - 1) # replace jump addr with label addr later on
            elif verb == "halt":
                binary = np.append(binary, [ins_map["halt"]])

        for (lookup, pos) in label_lookup.items(): # look up label references and replace with final addr
            if not lookup in labels:
                raise ParserError("Unkown label '%s'" % lookup)
            else:
                for indx in pos:
                    np.put(binary, indx, labels[lookup])

        return binary


argparser = argparse.ArgumentParser(description="Compiles code for execution on the DMVM")
argparser.add_argument("-i", "--input", help="path to input file", action="store", required=True)
argparser.add_argument("-o", "--output", help="path to output file", action="store", required=True)
args = argparser.parse_args()

f = open(args.input, "r")
doc = f.read()

lexer = Lexer(doc)
lines = None

try:
    lines = lexer.tokenize()
except Exception as e:
    print("Syntax error on line {}: {}".format(e.line, str(e)))
    exit(-1)

parser = Parser(lines)
try:
    binary = parser.parse()
    binary.astype("uint8").tofile(open(args.output, "w"))
    print("Compilation successful. Output is stored in", args.output)
except ParserError as e :
     print("Parser error:", e.with_traceback(None))
     exit(-1)
