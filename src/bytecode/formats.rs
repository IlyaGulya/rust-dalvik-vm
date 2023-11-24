use std::io::Read;

use bitstream_io::{BitRead, BitReader, LittleEndian};

// Format ØØ|op
pub struct Format10x {
    op: u8,
}

// Format B|A|op
pub struct Format12x {
    op: u8,
    a: u8,
    b: u8,
}

// Format 11n
pub struct Format11n {
    op: u8,
    a: u8,
    b: i8,
}

// Format AA|op
pub struct Format11x {
    op: u8,
    aa: u8,
}

// Format 10t
pub struct Format10t {
    op: u8,
    aa: i8,
}

// Format ØØ|op AAAA
pub struct Format20t {
    op: u8,
    aaaa: i16,
}

// Format AA|op BBBB
pub struct Format20bc {
    op: u8,
    aa: u8,
    bbbb: u16,
}

// Format AA|op BBBB
pub struct Format22x {
    op: u8,
    aa: u8,
    bbbb: u16,
}

// Format 21t
pub struct Format21t {
    op: u8,
    aa: u8,
    bbbb: i16,
}

// Format 21s
pub struct Format21s {
    op: u8,
    aa: u8,
    bbbb: i16,
}

// Format 21h
pub struct Format21h {
    op: u8,
    aa: u8,
    bbbb: i16,
}

// Format 21c
pub struct Format21c {
    op: u8,
    aa: u8,
    bbbb: u16,
}

// Format AA|op CC|BB
pub struct Format23x {
    op: u8,
    aa: u8,
    bb: u8,
    cc: u8,
}

// Format 22b
pub struct Format22b {
    op: u8,
    aa: u8,
    bb: u8,
    cc: i8,
}

// Format B|A|op CCCC
pub struct Format22t {
    op: u8,
    a: u8,
    b: u8,
    cccc: i16,
}

// Format 22s
pub struct Format22s {
    op: u8,
    a: u8,
    b: u8,
    cccc: i16,
}

// Format 22c
pub struct Format22c {
    op: u8,
    a: u8,
    b: u8,
    cccc: u16,
}

// Format ØØ|op AAAAlo AAAAhi
pub struct Format30t {
    op: u8,
    aaaa: i32,
}

// Format ØØ|op AAAA BBBB
pub struct Format32x {
    op: u8,
    aaaa: u16,
    bbbb: u16,
}

// Format AA|op BBBBlo BBBBhi
pub struct Format31i {
    op: u8,
    aa: u8,
    bbbb: i32,
}

// Format 31t
pub struct Format31t {
    op: u8,
    aa: u8,
    bbbb: i32,
}

// Format 31c
pub struct Format31c {
    op: u8,
    aa: u8,
    bbbb: u32,
}

// Format A|G|op BBBB F|E|D|C
// Same for 35ms and 35mi
pub struct Format35c {
    pub bbbb: u16,
    pub reg_list: Vec<u8>, // Массив регистров vC, vD, vE, vF, vG
}

impl Format35c where {
    pub fn parse<R: Read>(reader: &mut BitReader<R, LittleEndian>) -> Format35c {
        let g = reader.read::<u8>(4).expect("Failed to read G");
        let arg_count = reader.read::<u8>(4).expect("Failed to read A");
        let bbbb = reader.read::<u16>(16).expect("Failed to read BBBB");
        let mut registers: Vec<u8> = (0..arg_count).map(|_| {
            reader.read::<u8>(4).expect("Failed to read arg_register")
        }).collect();
        if arg_count == 5 {
            registers.push(g);
        }
        return Format35c {
            bbbb,
            reg_list: registers,
        };
    }
}

// Format AA|op BBBB CCCC
// Same for 3rms and 3rmi
pub struct Format3rc {
    op: u8,
    aa: u8,
    bbbb: u16,
    cccc: u16,
}

// Format A|G|op BBBB F|E|D|C HHHH
pub struct Format45cc {
    op: u8,
    g: u8,
    // This includes both the count of argument words (A) and a register (G).
    bbbb: u16,
    // Method index
    reg_list: [u8; 5],
    // List of registers vC, vD, vE, vF, vG
    hhhh: u16, // Index into the proto_ids list
}

// Format AA|op BBBB CCCC HHHH
pub struct Format4rcc {
    op: u8,
    aa: u8,
    // Count of argument words
    bbbb: u16,
    // Method index
    cccc: u16,
    // First register (vCCCC)
    hhhh: u16, // Index into the proto_ids list
}

// Format AA|op BBBBlo BBBB BBBB BBBBhi
pub struct Format51l {
    op: u8,
    aa: u8,
    // Register (vAA)
    bbbb_lo: u16,
    // Lower 16 bits of the literal value
    bbbb_hi: u16, // Higher 16 bits of the literal value
}