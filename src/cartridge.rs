use std::vec::Vec;
use crate::ops::Op;

// 4 bytes for (opcode, rd, rs1, rs2), 8 bytes for double immediate
const OPSIZE: usize = 12;
// u32
const SIZESIZE: usize = 4;

// The program serialization format is:
// [4 bytes: LE u32 of opcount]
// [8 bytes * opcount: op immediates]
// [4 bytes * opcount: op data]
pub fn serialize_ops(oplist: &Vec<Op>) -> Vec<u8> {
  let mut data: Vec<u8> = Vec::with_capacity(SIZESIZE + oplist.len()*OPSIZE);

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