use image::GenericImageView; // to allow calling .pixels()

pub fn load_image_rom(filename: &str) -> Vec<u8> {
    let img = image::open(filename).expect("Image file not found!");
    let (w, h) = img.dimensions();
    if w != 256 {
        println!("Warning: image width {} != 256", w);
    }

    let mut rom_bytes: Vec<u8> = Vec::with_capacity((w * h) as usize);
    for (_x, _y, pixel) in img.pixels() {
        let red = pixel[0];
        rom_bytes.push(red);
    }
    rom_bytes
}

pub fn load_file_rom(filename: &str) -> Vec<u8> {
    std::fs::read(filename).expect("Unable to read binary file!")
}
