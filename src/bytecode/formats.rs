use std::io::{BufReader, Read};

use bitstream_io::{BitRead, BitReader, LittleEndian};
use byteorder::ReadBytesExt;

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
    pub aa: u8,
    pub bbbbbbbb: i32,
}

// Format 31t
pub struct Format31t {
    pub aa: u8,
    pub bbbbbbbb: i32,
}

impl Format31t {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Format31t {
        return Format31t {
            aa: reader.read_u8().expect("Failed to read AA"),
            bbbbbbbb: reader.read_i32::<byteorder::LittleEndian>().expect("Failed to read BBBB"),
        };
    }
}

// Format 31c
pub struct Format31c {
    aa: u8,
    bbbb: u32,
}

// Format A|G|op BBBB F|E|D|C
// Same for 35ms and 35mi
pub struct Format35c {
    pub bbbb: u16,
    pub reg_list: Vec<u8>,
}

impl Format35c where {
    pub fn parse<R: Read>(reader: &mut BitReader<R, LittleEndian>) -> Format35c {
        let g = reader.read::<u8>(4).expect("Failed to read G");
        let a = reader.read::<u8>(4).expect("Failed to read A");
        let bbbb = reader.read::<u16>(16).expect("Failed to read BBBB");
        let c = reader.read::<u8>(4).expect("Failed to read C");
        let d = reader.read::<u8>(4).expect("Failed to read C");
        let e = reader.read::<u8>(4).expect("Failed to read C");
        let f = reader.read::<u8>(4).expect("Failed to read C");
        let mut registers = vec![c, d, e, f, g];
        // println!("Args: {}. Registers: {:?}", a, registers);
        registers.truncate(a as usize);
        return Format35c {
            bbbb,
            reg_list: registers,
        };
    }
}

// Format AA|op BBBB CCCC
// Same for 3rms and 3rmi
pub struct Format3rc {
    pub reg_count: u8,
    pub bbbb: u16,
    pub cccc: u16,
}

impl Format3rc {
    pub fn parse<R: Read>(reader: &mut R) -> Format3rc {
        let reg_count = reader.read_u8().expect("Failed to read reg_count");
        return Format3rc {
            reg_count,
            bbbb: reader.read_u16::<byteorder::LittleEndian>().expect(
                format!("Failed to read bbbb for reg_count {}", reg_count).as_str(),
            ),
            cccc: reader.read_u16::<byteorder::LittleEndian>().expect("Failed to read cccc"),
        };
    }
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