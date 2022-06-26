use asmjr::{cartridge, metadata, parser, vrom};
use clap::Parser;
use std::fs;
use std::fs::read_to_string;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Assembly source file
    #[clap(value_parser)]
    source: String,

    /// Output ECJR cartridge file
    #[clap(value_parser)]
    output: Option<String>,

    /// Load image (red channel only) into rom
    #[clap(short, long, value_parser)]
    imagerom: Option<String>,

    /// Load raw bytes into rom
    #[clap(short, long, value_parser)]
    rawrom: Option<String>,

    /// Author to embed into metadata
    #[clap(long, value_parser)]
    author: Option<String>,

    /// Readme file to embed in metadata
    #[clap(long, value_parser)]
    readme: Option<String>,

    /// Simple message to embed in metadata
    #[clap(short, long, value_parser)]
    message: Option<String>,

    /// Leave cart body uncompressed
    #[clap(short, long, action)]
    uncompressed: bool,

    /// Export bare program without .cart container
    #[clap(long, action)]
    bare: bool,

    /// Dump out ops to terminal
    #[clap(short, long, action)]
    listing: bool,
}

fn main() {
    let args = Args::parse();

    let sourcefile = read_to_string(args.source).expect("Failed to read source file!");

    let ops = match parser::parse(&sourcefile) {
        Ok(ops) => ops,
        Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    println!("Assembled {} ops.", ops.len());

    if args.listing {
        parser::print_ops(&ops);
    }

    let output = match args.output {
        Some(output) => output,
        None => {
            println!("No output file specified.");
            return;
        }
    };

    if args.bare {
        let bare_prog = cartridge::serialize_ops(&ops);
        fs::write(&output, &bare_prog).expect("Failed to write output!");
        println!(
            "Wrote {} bytes of bare program to {}.",
            bare_prog.len(),
            output
        );
        return;
    }

    let videorom = match args.imagerom {
        Some(filename) => Some(vrom::load_image_rom(&filename)),
        None => args.rawrom.map(|filename| vrom::load_file_rom(&filename)),
    };

    let readme = match args.readme {
        Some(filename) => Some(read_to_string(filename).expect("Failed to read readme file!")),
        None => args.message,
    };

    let metadata = metadata::format_metadata(args.author, readme);
    println!("Metadata: {}", metadata);

    let cartdata = cartridge::pack_cartridge(Some(metadata), videorom, &ops, !args.uncompressed);
    fs::write(&output, &cartdata).expect("Failed to write output file!");
    println!("Wrote {} bytes to {}.", cartdata.len(), output);
}
