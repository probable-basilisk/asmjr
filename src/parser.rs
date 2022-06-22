use std::vec::Vec;
use std::collections::HashMap;
use crate::ops::{Op, OpErr, parse_op};
use crate::memmap::add_memmap_constants;

use pest_derive::Parser;
use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "asm.pest"]
struct AsmParser;

const MAX_REGISTERS: usize = 256;

fn default_aliases() -> HashMap<String, u8> {
  let mut aliases: HashMap<String, u8> = HashMap::new();
  aliases.insert("zero".to_string(), 0);
  for reg_idx in 0..MAX_REGISTERS {
    aliases.insert(format!("x{}", reg_idx), reg_idx as u8);
  }
  aliases
}

fn add_alias(aliases: &mut HashMap<String, u8>, name: &str, value: &str) -> Result<(), OpErr> {
  if let Ok(v) = parse_int::parse::<u8>(value) {
    aliases.insert(name.to_string(), v);
    return Ok(())
  }
  match aliases.get(value) {
    Some(v) => { 
      // borrow checker complains if we don't separate this
      let v = *v; 
      aliases.insert(name.to_string(), v); 
      Ok(())
    },
    _ => {
      Err(OpErr::InvalidAlias(value.to_string()))
    }
  }
}

fn add_constant(constants: &mut HashMap<String, f64>, name: &str, value: &str) -> Result<(), OpErr> {
  if let Ok(barenum) = value.parse::<f64>() {
    constants.insert(name.to_string(), barenum);
    return Ok(())
  }
  if let Ok(barenum) = parse_int::parse::<i64>(value) {
    constants.insert(name.to_string(), barenum as f64);
    return Ok(())
  }
  Err(OpErr::InvalidImmediate(value.to_string()))
}

#[derive(Debug)]
pub enum ParseErr {
  Generic(String),
  Line(usize, String, String)
}

impl ToString for ParseErr {
  fn to_string(&self) -> String {
    match self {
      ParseErr::Generic(s) => format!("Parse error: {}", s),
      ParseErr::Line(pos, line, msg) => 
        format!("Parse error on line {} [\"{}\"]: {}", pos+1, line, msg),
    }
  }
}

fn find_labels(lines: Pairs<Rule>) -> HashMap<String, f64> {
  let mut labels: HashMap<String, f64> = HashMap::new();
  let mut pc: u32 = 0;
  for line in lines.clone() {
    match line.as_rule() {
      Rule::label => {
        let label = line.into_inner().next().unwrap().as_str();
        labels.insert(label.to_string(), pc as f64);
      },
      Rule::op => {
        pc += 1;
      }
      _ => {}
    }
  }
  labels
}

fn parse_err(operr: OpErr, linepos: usize, line: &str) -> ParseErr {
  ParseErr::Line(linepos, line.to_string(), operr.to_string())
}

pub fn parse(src: &String) -> Result<Vec<Op>, ParseErr> {
  let lines = AsmParser::parse(Rule::program, src)
    .map_err(|e| ParseErr::Generic(e.to_string()))?;

  // an op can refer to a label that's defined further in the program,
  // so need to do a prepass to find label locations
  let mut constants = find_labels(lines.clone());
  add_memmap_constants(&mut constants);
  let mut aliases = default_aliases();
  let mut ops: Vec<Op> = Vec::new();

  for (linepos, line) in lines.enumerate() {
    let linestr = line.clone().as_str();

    let pc = ops.len() as u32;
    match line.as_rule() {
      Rule::alias => {
        let mut inner = line.into_inner();
        let name = inner.next().unwrap().as_str();
        let value = inner.next().unwrap().as_str();
        add_alias(&mut aliases, name, value)
          .map_err(|e| parse_err(e, linepos, linestr))?
      },
      Rule::constant => {
        let mut inner = line.into_inner();
        let name = inner.next().unwrap().as_str();
        let value = inner.next().unwrap().as_str();
        add_constant(&mut constants, name, value)
          .map_err(|e| parse_err(e, linepos, linestr))?
      },
      Rule::op => {
        let tokens: Vec<&str> = line.into_inner().map(|pair| { pair.as_str() }).collect();
        let op = parse_op(&tokens, pc, &constants, &aliases)
          .map_err(|e| parse_err(e, linepos, linestr))?;
        //println!("op: {:?}", op);
        ops.push(op);
      },
      _ => {}
    }
  }

  Ok(ops)
}

pub fn print_ops(ops: &Vec<Op>) {
  for (pc, op) in ops.iter().enumerate() {
    println!("{}: {:?}", pc, op);
  }
}
