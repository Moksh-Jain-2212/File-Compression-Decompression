// input file
//    ↓
// open/read file
//    ↓
// compress bytes
//    ↓
// write compressed bytes
//    ↓
// output.gz

use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::io::Write;
use std::io::{Read, copy, BufReader};
use std::fs::File;


fn main() {
    
    let input_file = File::open("/home/moksh/Rust Projects/compress_file/src/temp.txt").expect("Failed to open input file");
    let output_file = File::create("/home/moksh/Rust Projects/compress_file/src/output.txt.gz").expect("Failed to create output file");

    let mut encoder = GzEncoder::new(output_file, Compression::default());
    let mut reader: BufReader<File> = BufReader::new(input_file);

    copy(&mut reader, &mut encoder).expect("Compression failed");
    encoder.finish().expect("Failed to finish compression");

    println!("Compression complete");

    let input = File::open("/home/moksh/Rust Projects/compress_file/src/output.txt.gz")
        .expect("Could not open compressed file");

    let mut decoder = GzDecoder::new(input);

    let mut output = File::create("/home/moksh/Rust Projects/compress_file/src/output.txt")
        .expect("Could not create output file");

    copy(&mut decoder, &mut output)
        .expect("Could not decompress file");

    println!("Decompression complete");

}