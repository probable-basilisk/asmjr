use std::env;
use std::fs::read_to_string;
use std::fs;
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
  let ops = parser::parse(&sourcefile).unwrap();
  
  let cartdata = cartridge::pack_cartridge(None, None, &ops, false);
  fs::write("cart.cart", cartdata).unwrap();
  println!("We wrote something!");
}
