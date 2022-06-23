use crate::compression::compress_bytes;
use crate::ops::Op;
use std::vec::Vec;

use prost::Message;

// this is generated by prost
pub mod cart {
    include!(concat!(env!("OUT_DIR"), "/ecjr.cart.rs"));
}

// 4 bytes for (opcode, rd, rs1, rs2), 8 bytes for double immediate
const OPSIZE: usize = 12;
// u32
const SIZESIZE: usize = 4;

// The program serialization format is:
// [4 bytes: LE u32 of opcount]
// [8 bytes * opcount: op immediates]
// [4 bytes * opcount: op data]
pub fn serialize_ops(oplist: &Vec<Op>) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(SIZESIZE + oplist.len() * OPSIZE);

    let size_bytes = (oplist.len() as u32).to_le_bytes();
    data.extend_from_slice(&size_bytes);

    for op in oplist.iter() {
        data.extend_from_slice(&op.imm.to_le_bytes());
    }

    for op in oplist.iter() {
        data.push(op.op.opcode);
        data.push(op.op.rd);
        data.push(op.op.rs1);
        data.push(op.op.rs2);
    }

    data
}

pub fn pack_cartridge(
    metadata: Option<String>,
    videorom: Option<Vec<u8>>,
    program: &Vec<Op>,
    compress: bool,
) -> Vec<u8> {
    let metadata = match metadata {
        Some(meta) => meta,
        None => "{}".to_string(),
    };
    let videorom = match videorom {
        Some(rom) => rom,
        None => Vec::new(),
    };
    let cartridge_body = cart::Cartridge {
        metadata: metadata,
        program: serialize_ops(program),
        videorom: videorom,
    };

    let mut serialized_body = Vec::new();
    serialized_body.reserve(cartridge_body.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    cartridge_body.encode(&mut serialized_body).unwrap();
    let uncompressed_size = serialized_body.len() as u32;
    let (compressed_size, final_body) = if compress {
        let original_size = serialized_body.len();
        let compressed = compress_bytes(&serialized_body);
        println!("Compressed {} -> {}", original_size, compressed.len());
        (compressed.len() as u32, compressed)
    } else {
        (0u32, serialized_body)
    };

    let mut final_data: Vec<u8> = Vec::new();
    final_data.extend_from_slice("ECJRV004".as_bytes());
    final_data.extend_from_slice(&uncompressed_size.to_le_bytes());
    final_data.extend_from_slice(&compressed_size.to_le_bytes());
    final_data.extend(final_body);

    final_data
}
