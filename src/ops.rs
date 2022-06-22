use phf::phf_map;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum OpArg {
  Rd,
  Rs1,
  Rs2,
  Im,
  Void,
}

#[derive(Clone)]
pub struct OpInfo {
  opcode: u8,
  argct: usize,
  args: [OpArg; 3],
  rel: bool,
}

static OPS: phf::Map<&'static str, OpInfo> = phf_map! {
  "kill" => OpInfo{opcode: 0, argct: 2, args: [OpArg::Rd, OpArg::Im, OpArg::Void], rel: false},
  "spawn" => OpInfo{opcode: 1, argct: 3, args: [OpArg::Rs1, OpArg::Rs2, OpArg::Im], rel: true},
  "xkill" => OpInfo{opcode: 2, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "xres" => OpInfo{opcode: 3, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "crid" => OpInfo{opcode: 4, argct: 1, args: [OpArg::Rd, OpArg::Void, OpArg::Void], rel: false},
  "crcfg" => OpInfo{opcode: 5, argct: 2, args: [OpArg::Rd, OpArg::Rs1, OpArg::Void], rel: false},
  "crcnd" => OpInfo{opcode: 6, argct: 2, args: [OpArg::Rd, OpArg::Rs1, OpArg::Void], rel: false},
  "crclk" => OpInfo{opcode: 7, argct: 2, args: [OpArg::Rd, OpArg::Rs1, OpArg::Void], rel: false},
  "crct" => OpInfo{opcode: 8, argct: 1, args: [OpArg::Rd, OpArg::Void, OpArg::Void], rel: false},
  "clk" => OpInfo{opcode: 9, argct: 1, args: [OpArg::Rd, OpArg::Void, OpArg::Void], rel: false},
  "mv" => OpInfo{opcode: 10, argct: 2, args: [OpArg::Rd, OpArg::Rs1, OpArg::Void], rel: false},
  "nop" => OpInfo{opcode: 11, argct: 0, args: [OpArg::Void, OpArg::Void, OpArg::Void], rel: false},
  "add" => OpInfo{opcode: 12, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "addi" => OpInfo{opcode: 13, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "sub" => OpInfo{opcode: 14, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "subi" => OpInfo{opcode: 15, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "mul" => OpInfo{opcode: 16, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "muli" => OpInfo{opcode: 17, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "div" => OpInfo{opcode: 18, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "divi" => OpInfo{opcode: 19, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "mod" => OpInfo{opcode: 20, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "modi" => OpInfo{opcode: 21, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "pow" => OpInfo{opcode: 22, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "powi" => OpInfo{opcode: 23, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "min" => OpInfo{opcode: 24, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "mini" => OpInfo{opcode: 25, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "max" => OpInfo{opcode: 26, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "maxi" => OpInfo{opcode: 27, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "eq" => OpInfo{opcode: 28, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "eqi" => OpInfo{opcode: 29, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "neq" => OpInfo{opcode: 30, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "neqi" => OpInfo{opcode: 31, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "geq" => OpInfo{opcode: 32, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "geqi" => OpInfo{opcode: 33, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "leq" => OpInfo{opcode: 34, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "leqi" => OpInfo{opcode: 35, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "lt" => OpInfo{opcode: 36, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "lti" => OpInfo{opcode: 37, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "gt" => OpInfo{opcode: 38, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "gti" => OpInfo{opcode: 39, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "and" => OpInfo{opcode: 40, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "andi" => OpInfo{opcode: 41, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "or" => OpInfo{opcode: 42, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "ori" => OpInfo{opcode: 43, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "xor" => OpInfo{opcode: 44, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "xori" => OpInfo{opcode: 45, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "lsh" => OpInfo{opcode: 46, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "lshi" => OpInfo{opcode: 47, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "rsh" => OpInfo{opcode: 48, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "rshi" => OpInfo{opcode: 49, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "li" => OpInfo{opcode: 50, argct: 2, args: [OpArg::Rd, OpArg::Im, OpArg::Void], rel: false},
  "aipc" => OpInfo{opcode: 51, argct: 2, args: [OpArg::Rd, OpArg::Im, OpArg::Void], rel: true},
  "jal" => OpInfo{opcode: 52, argct: 2, args: [OpArg::Rd, OpArg::Im, OpArg::Void], rel: true},
  "jalr" => OpInfo{opcode: 53, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "beq" => OpInfo{opcode: 54, argct: 3, args: [OpArg::Rs1, OpArg::Rs2, OpArg::Im], rel: true},
  "bne" => OpInfo{opcode: 55, argct: 3, args: [OpArg::Rs1, OpArg::Rs2, OpArg::Im], rel: true},
  "blt" => OpInfo{opcode: 56, argct: 3, args: [OpArg::Rs1, OpArg::Rs2, OpArg::Im], rel: true},
  "bge" => OpInfo{opcode: 57, argct: 3, args: [OpArg::Rs1, OpArg::Rs2, OpArg::Im], rel: true},
  "load" => OpInfo{opcode: 58, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "store" => OpInfo{opcode: 59, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "pushi" => OpInfo{opcode: 60, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "unpki" => OpInfo{opcode: 61, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "cas" => OpInfo{opcode: 62, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
  "smprm" => OpInfo{opcode: 63, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "srprm" => OpInfo{opcode: 64, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "swprm" => OpInfo{opcode: 65, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "sxprm" => OpInfo{opcode: 66, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Im], rel: false},
  "abs" => OpInfo{opcode: 67, argct: 2, args: [OpArg::Rd, OpArg::Rs1, OpArg::Void], rel: false},
  "sin" => OpInfo{opcode: 68, argct: 2, args: [OpArg::Rd, OpArg::Rs1, OpArg::Void], rel: false},
  "cos" => OpInfo{opcode: 69, argct: 2, args: [OpArg::Rd, OpArg::Rs1, OpArg::Void], rel: false},
  "atan" => OpInfo{opcode: 70, argct: 3, args: [OpArg::Rd, OpArg::Rs1, OpArg::Rs2], rel: false},
};

#[derive(Debug, Clone, Default)]
pub struct COp {
  pub opcode: u8,
  pub rd: u8,
  pub rs1: u8,
  pub rs2: u8,
}

impl COp {
  fn set_arg(&mut self, arg: &OpArg, register: u8) {
    match arg {
      OpArg::Rd => {self.rd = register},
      OpArg::Rs1 => {self.rs1 = register},
      OpArg::Rs2 => {self.rs2 = register},
      _ => {}
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct Op {
  pub op: COp,
  pub imm: f64,
}

#[derive(Debug)]
pub enum OpErr {
  Impossible,
  EmptyOp,
  InvalidAlias(String),
  InvalidOpcode(String),
  InvalidArgumentCount(usize, usize),
  InvalidImmediate(String),
  InvalidRegister(String)
}

impl ToString for OpErr {
  fn to_string(&self) -> String {
    match self {
      OpErr::Impossible => "This shouldn't be possible!".to_string(),
      OpErr::EmptyOp => "Empty opcode (grammar problem?)".to_string(),
      OpErr::InvalidAlias(s) => format!("Invalid alias: \"{}\"", s),
      OpErr::InvalidOpcode(s) => format!("Unrecognized opcode: [{}]", s),
      OpErr::InvalidArgumentCount(got, expected) => 
        format!("Wrong number of arguments: got {}, expected {}", got, expected),
      OpErr::InvalidImmediate(s) => 
        format!("Immediate \"{}\" is not a literal or known label", s),
      OpErr::InvalidRegister(s) => 
        format!("Register \"{}\" is not a literal or known alias", s),
    }
  }
}

fn parse_immediate(token: &str, pc: u32, rel: bool, labels: &HashMap<String, u32>) -> Result<f64, OpErr> {
  // Any numeric literal immediate will be left untouched
  if let Ok(barenum) = token.parse::<f64>() {
    return Ok(barenum)
  }
  if let Ok(barenum) = parse_int::parse::<i64>(token) {
    return Ok(barenum as f64)
  }
  // Not a numeric literal, try as a label
  let labelpos = match labels.get(token) {
    Some(pos) => pos,
    None => return Err(OpErr::InvalidImmediate(token.to_owned()))
  };
  if rel {
    Ok((*labelpos as f64) - (pc as f64))
  } else {
    Ok(*labelpos as f64)
  }
}

fn parse_register(token: &str, aliases: &HashMap<String, u8>) -> Result<u8, OpErr> {
  // Allow numeric literals as register designations
  if let Ok(barenum) = parse_int::parse::<u8>(token) {
    return Ok(barenum)
  }
  match aliases.get(token) {
    Some(reg) => Ok(*reg),
    None => Err(OpErr::InvalidRegister(token.to_owned()))
  }
}

pub fn parse_op(tokens: &Vec<&str>, pc: u32, labels: &HashMap<String, u32>, aliases: &HashMap<String, u8>) -> Result<Op, OpErr> {
  let name = match tokens.get(0) {
    Some(name) => name.to_lowercase(),
    _ => return Err(OpErr::EmptyOp)
  };
  let info = match OPS.get(&name) {
    Some(info) => info,
    _ => return Err(OpErr::InvalidOpcode(name.to_string())),
  };
  if tokens.len() - 1 != info.argct {
    return Err(OpErr::InvalidArgumentCount(tokens.len()-1, info.argct));
  };
  let mut ret = Op::default();
  ret.op.opcode = info.opcode;
  for (token, arg) in tokens[1..].iter().zip(info.args.iter()) {
    match arg {
      OpArg::Void => {return Err(OpErr::Impossible)},
      OpArg::Im => {
        ret.imm = parse_immediate(token, pc, info.rel, labels)?;
      },
      _ => {
        ret.op.set_arg(arg, parse_register(token, aliases)?);
      }
    }
  };
  Ok(ret)
}