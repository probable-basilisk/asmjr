use zstd::stream::copy_encode;

const COMPRESSION_LEVEL: i32 = 18;

pub fn compress_bytes(src: &Vec<u8>) -> Vec<u8> {
  let mut dest: Vec<u8> = Vec::new();
  copy_encode(src.as_slice(), &mut dest, COMPRESSION_LEVEL).unwrap();
  dest
}