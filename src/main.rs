use std::fs::read_to_string;
use std::fs;
use clap::Parser;

mod ops;
mod parser;
mod compression;
mod cartridge;
mod vrom;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  /// Assembly source file
  #[clap(value_parser)]
  source: String,

  /// Output ECJR cartridge file
  #[clap(value_parser)]
  output: String,

  /// Load image (red channel only) into rom
  #[clap(short, long, value_parser)]
  imagerom: Option<String>,

  /// Load raw bytes into rom
  #[clap(short, long, value_parser)]
  rawrom: Option<String>,

  /// Leave cart body uncompressed
  #[clap(short, long, action)]
  uncompressed: bool,
}


fn main() {
  let args = Args::parse();

  let sourcefile = read_to_string(args.source).expect("Failed to read source file!");
  let videorom = match args.imagerom {
    Some(filename) => Some(vrom::load_image_rom(&filename)),
    None => {
      match args.rawrom {
        Some(filename) => Some(vrom::load_file_rom(&filename)),
        None => None
      }
    }
  };

  let ops = match parser::parse(&sourcefile) {
    Ok(ops) => ops,
    Err(e) => {
      println!("{}", e.to_string());
      return;
    }
  };

  let cartdata = cartridge::pack_cartridge(None, videorom, &ops, !args.uncompressed);
  fs::write(&args.output, &cartdata).expect("Failed to write output file!");
  println!("Wrote {} bytes to {}.", cartdata.len(), args.output);
}
