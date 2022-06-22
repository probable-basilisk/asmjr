use std::vec::Vec;
use crate::ops::{Op, parse_op};

use pest::{
    iterators::{Pair, Pairs},
};
use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "asm.pest"] // relative to src
struct AsmParser;

// -> Result<Vec<Op>, OpErr>
pub fn parse(src: &String) {
  let lines = AsmParser::parse(Rule::program, src).unwrap_or_else(|e| panic!("{}", e));

  // Because ident_list is silent, the iterator will contain idents
  for line in lines {
    let span = line.clone().as_span();
    // A pair is a combination of the rule which matched and a span of input
    println!("Rule:    {:?}", line.as_rule());
    println!("Span:    {:?}", span);
    println!("Text:    {}", span.as_str());

    match line.as_rule() {
      Rule::op => {
        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in line.into_inner() {
          let inner_span = inner_pair.clone().as_span();
          println!("token I guess: {}", inner_span.as_str());
        }
      },
      _ => {}
    }
  }
}
