use std::env;
use std::fs::read_to_string;
use std::fs;
mod ops;
mod parser;
mod compression;
mod cartridge;
mod vrom;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
  if args.len() < 2 {
    return;
  }
  let sourcefile = read_to_string(args[1].clone()).unwrap();
  let ops = match parser::parse(&sourcefile) {
    Ok(ops) => ops,
    Err(e) => {
      println!("{}", e.to_string());
      return;
    }
  };
  
  let videorom = vrom::load_image_rom("rom.png");

  let cartdata = cartridge::pack_cartridge(None, Some(videorom), &ops, true);
  fs::write("cart.cart", cartdata).unwrap();
  println!("We wrote something!");
}
