use std::vec::Vec;
use std::collections::HashMap;
use crate::ops::{Op, OpErr, parse_op};

use pest_derive::Parser;
use pest::Parser;

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

fn add_alias(aliases: &mut HashMap<String, u8>, newname: &str, aliased: &str) -> Result<(), OpErr> {
  if let Ok(v) = parse_int::parse::<u8>(aliased) {
    aliases.insert(newname.to_string(), v);
    return Ok(())
  }
  match aliases.get(aliased) {
    Some(v) => { 
      // borrow checker complains if we don't separate this
      let v = *v; 
      aliases.insert(newname.to_string(), v); 
      Ok(())
    },
    _ => {
      Err(OpErr::InvalidAlias(aliased.to_string()))
    }
  }
}

pub fn parse(src: &String) -> Result<Vec<Op>, OpErr> {
  let mut labels: HashMap<String, u32> = HashMap::new();
  let mut aliases = default_aliases();
  let mut ops: Vec<Op> = Vec::new();

  let lines = AsmParser::parse(Rule::program, src).unwrap_or_else(|e| panic!("{}", e));

  // a line can use a label further ahead in the program, so we need to do a prepass
  // to find the label locations, which also means keeping track of the PC
  let mut pc: u32 = 0;
  for line in lines.clone() {
    match line.as_rule() {
      Rule::label => {
        let label = line.into_inner().next().unwrap().as_str();
        labels.insert(label.to_string(), pc);
      },
      Rule::op => {
        pc += 1;
      }
      _ => {}
    }
  }

  for line in lines {
    let span = line.clone().as_span();
    // A pair is a combination of the rule which matched and a span of input
    println!("Rule:    {:?}", line.as_rule());
    println!("Text:    {}", span.as_str());

    match line.as_rule() {
      Rule::alias => {
        let mut inner = line.into_inner();
        let newname = inner.next().unwrap().as_str();
        let aliased = inner.next().unwrap().as_str();
        add_alias(&mut aliases, newname, aliased)?
      },
      Rule::op => {
        let tokens: Vec<&str> = line.into_inner().map(|pair| { pair.as_str() }).collect();
        let pc = ops.len() as u32;
        let op = parse_op(&tokens, pc, &labels, &aliases)?;
        println!("op: {:?}", op);
        ops.push(op);
      },
      _ => {}
    }
  }
  Ok(ops)
}
