use std::cmp::min;
use wasm_bindgen::prelude::*;

mod compression;
mod memmap;

pub mod cartridge;
pub mod metadata;
pub mod ops;
pub mod parser;

fn bounded_copy(dest: &mut [u8], src: &[u8]) -> usize {
    let ncopy = min(dest.len(), src.len());
    dest[..ncopy].copy_from_slice(src);
    ncopy
}

#[wasm_bindgen]
pub fn assemble_simple(src: &str, rom: &[u8], dest: &mut [u8]) -> i32 {
    let ops = match parser::parse(src) {
        Ok(ops) => ops,
        Err(e) => {
            let strmsg = e.to_string();
            let copied = bounded_copy(dest, strmsg.as_bytes());
            return -(copied as i32);
        }
    };
    let vrom = if rom.len() > 0 {
        Some(rom.to_vec())
    } else {
        None
    };

    let data = cartridge::pack_cartridge(None, vrom, &ops, true);
    let ncopied = bounded_copy(dest, &data);
    
    ncopied as i32
}
