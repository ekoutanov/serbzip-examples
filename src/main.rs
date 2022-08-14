use std::fs::File;
use std::{fs};
use std::io::{BufReader, BufWriter};
use serbzip_core::codecs::balkanoid::{Balkanoid, Dict};
use serbzip_core::codecs::Codec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // this codec needs a dictionary to work from
    let mut dict_reader = BufReader::new(File::open("../serbzip/dict.blk")?);
    let dict = Dict::read_from_binary_image(&mut dict_reader)?;
    let codec = Balkanoid::new(&dict);

    // compress a line and check the output
    let input_line = "Ah, distinctly I remember it was in the bleak December";
    let compressed_line = codec.compress_line(input_line);
    println!("compressed_line is '{compressed_line}'");

    // expand the line; check that it matches the original
    let expanded_line = codec.expand_line(&compressed_line)?;
    println!("expanded_line is '{expanded_line}'");

    // codecs also have helper methods for parsing I/O streams
    let mut input_reader = BufReader::new(File::open("../serbzip/test_data/antigonish.txt")?);
    let mut output_writer = BufWriter::new(File::create("antigonish.sz")?);
    codec.compress(&mut input_reader, &mut output_writer)?;
    assert!(fs::metadata("antigonish.sz")?.is_file());
    Ok(())
}