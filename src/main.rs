use std::env;
use std::fs::read_to_string;
mod ops;
mod parser;
mod cartridge;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  if args.len() < 2 {
    return;
  }
  let sourcefile = read_to_string(args[1].clone()).unwrap();
  parser::parse(&sourcefile).unwrap();
  println!("Done");
}
