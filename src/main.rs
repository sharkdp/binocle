use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read};
use std::path::Path;
use std::process::Command;

fn grayscale(b: u8) -> [u8; 4] {
    [b, b, b, 255]
}

fn colorful(b: u8) -> [u8; 4] {
    [b, b.overflowing_mul(2).0, b.overflowing_mul(4).0, 255]
}

fn category(b: u8) -> [u8; 4] {
    if b == 0x00 {
        [0, 0, 0, 255]
    } else if b.is_ascii_graphic() {
        [60, 255, 96, 255]
    } else if b.is_ascii_whitespace() {
        [240, 240, 240, 255]
    } else if b.is_ascii() {
        [60, 178, 255, 255]
    } else {
        [249, 53, 94, 255]
    }
}

fn color_gradient(b: u8) -> [u8; 4] {
    let gradient = colorgrad::magma();
    let color = gradient.at((b as f64)/255.0f64);
    [(color.r * 255.0) as u8, (color.g * 255.0) as u8, (color.b * 255.0) as u8, 255]
}

struct Binocle {}

fn read_binary<P: AsRef<Path>>(path: P, buffer: &mut Vec<u8>) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    reader.read_to_end(buffer)?;

    return Ok(());
}

fn write_png(width: u32, height: u32, data: &[u8]) {
    let path = Path::new(r"/tmp/out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let width = args[1].parse::<u32>().unwrap();
    
    // let width = 2 * 201u32;
    
    // let width = 1238u32;

    let mut buffer: Vec<u8> = vec![];
    read_binary("tests/bag-small", &mut buffer);

    let height = (buffer.len() as u32) / width;

    let len_truncated = (width as usize) * (height as usize);

    let mut pixel_buffer: Vec<u8> = vec![255; 4 * len_truncated];

    for i in 0..len_truncated {
        let byte = buffer[i];
        let color = color_gradient(byte);
        for j in 0..4 {
            pixel_buffer[4 * i + j] = color[j];
        }
    }

    write_png(width, height, &pixel_buffer);

    // Command::new("feh").arg("-B").arg("#333333").arg("--force-aliasing").arg("/tmp/out.png").status();
}
