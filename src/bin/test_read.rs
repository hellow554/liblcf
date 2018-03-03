extern crate liblcf;

use std::fs::File;
use liblcf::lcf_reader::LcfReader;
use liblcf::traits::ReadPascalString;

fn main() {
    let mut file = File::open("RPG_RT.ldb").expect("Wrong file?");

    let mut reader = LcfReader::new_from_read(&mut file).expect("Wrong File!");
    println!("{:?}", reader.read_string());
}