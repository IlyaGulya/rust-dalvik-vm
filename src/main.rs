use std::fs;
use std::io::Read;
use std::os::unix::fs::MetadataExt;

use buffered_reader::BufferedReader;
use byteorder::{ByteOrder, ReadBytesExt};
use crate::dex::dex_file;
use crate::dex::dex_file::ClassDefinition;
use crate::dex::raw_dex_file::parse_raw_dex_file;

use crate::runtime::{Runtime, RuntimeExt};

mod runtime;
mod dex;


fn main() {
    // let mut file: File<()> = open_file("tests/vm/hello.dex");

    let main_class_name = "HelloWorld".to_string();
    let file = "tests/vm/hello.dex";
    let raw_dex_file = parse_raw_dex_file(file);
    let mut opened = fs::File::open(file).unwrap();
    // let dex_file = dex_file::parse_dex_file(raw_dex_file, &mut opened);

    let mut file = fs::File::open(file).unwrap();

    let mut file_contents = vec![];
    file.read_to_end(&mut file_contents).unwrap();
    let lib_dex_file = dexparser::parse(&*file_contents).unwrap();

    // let class =
    //     dex_file
    //         .classes
    //         .iter()
    //         .find(|c| c.class_type.to_string() == format!("L{}", main_class_name)).unwrap();

    println!("Dex file from lib: {:#?}", lib_dex_file);
    // println!("Dex file: {:#?}", dex_file);
}





