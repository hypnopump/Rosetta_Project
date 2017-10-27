// Rust implementation of the write_txt code for Rosetta Stone
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::prelude::*;

fn main() {
    let mut file_in = File::open("data.txt").expect("opening data file");
    File::create("write_rs.txt").expect("Creating output file");
    let mut file_out = OpenOptions::new().write(true).append(true).open("write_rs.txt").expect("Opening output file");
    let mut text = String::new();
    file_in.read_to_string(&mut text).expect("reading file");
    for line in text.lines() {
        writeln!(file_out, "{}", line).expect("writing first line to output file");
        break;
    }
    writeln!(file_out); // writing blank line!
    for line in text.lines().rev() {
        write!(file_out, "{}", line).expect("writing last line to output file");
        break;
    }
}
