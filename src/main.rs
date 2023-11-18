use std::fs;
use std::io::Read;
use std::os::unix::fs::MetadataExt;

use buffered_reader::BufferedReader;
use byteorder::{ByteOrder, ReadBytesExt};

use endian_aware_reader::EndianAwareReader;

use crate::raw_dex_file::parse_raw_dex_file;

mod endian_aware_reader;
mod raw_dex_file;
mod dex_file;


fn main() {
    // let mut file: File<()> = open_file("tests/vm/hello.dex");

    let main_class_name = "HelloWorld";
    let file = "tests/vm/hello.dex";
    let raw_dex_file = parse_raw_dex_file(file);
    let mut opened = fs::File::open(file).unwrap();
    let dex_file = dex_file::parse_dex_file(raw_dex_file, &mut opened);

    let mut file = fs::File::open(file).unwrap();

    let mut file_contents = vec![];
    file.read_to_end(&mut file_contents).unwrap();
    let lib_dex_file = dexparser::parse(&*file_contents).unwrap();

    println!("Dex file from lib: {:#?}", lib_dex_file);
    println!("Dex file: {:#?}", dex_file);
}





