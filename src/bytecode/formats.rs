// Format ØØ|op
struct Format10x {
    op: u8,
}

// Format B|A|op
struct Format12x {
    op: u8,
    a: u8,
    b: u8,
}

// Format 11n
struct Format11n {
    op: u8,
    a: u8,
    b: i8,
}

// Format AA|op
struct Format11x {
    op: u8,
    aa: u8,
}

// Format 10t
struct Format10t {
    op: u8,
    aa: i8,
}

// Format ØØ|op AAAA
struct Format20t {
    op: u8,
    aaaa: i16,
}

// Format AA|op BBBB
struct Format20bc {
    op: u8,
    aa: u8,
    bbbb: u16,
}

// Format AA|op BBBB
struct Format22x {
    op: u8,
    aa: u8,
    bbbb: u16,
}

// Format 21t
struct Format21t {
    op: u8,
    aa: u8,
    bbbb: i16,
}

// Format 21s
struct Format21s {
    op: u8,
    aa: u8,
    bbbb: i16,
}

// Format 21h
struct Format21h {
    op: u8,
    aa: u8,
    bbbb: i16,
}

// Format 21c
struct Format21c {
    op: u8,
    aa: u8,
    bbbb: u16,
}

// Format AA|op CC|BB
struct Format23x {
    op: u8,
    aa: u8,
    bb: u8,
    cc: u8,
}

// Format 22b
struct Format22b {
    op: u8,
    aa: u8,
    bb: u8,
    cc: i8,
}

// Format B|A|op CCCC
struct Format22t {
    op: u8,
    a: u8,
    b: u8,
    cccc: i16,
}

// Format 22s
struct Format22s {
    op: u8,
    a: u8,
    b: u8,
    cccc: i16,
}

// Format 22c
struct Format22c {
    op: u8,
    a: u8,
    b: u8,
    cccc: u16,
}

// Format ØØ|op AAAAlo AAAAhi
struct Format30t {
    op: u8,
    aaaa: i32,
}

// Format ØØ|op AAAA BBBB
struct Format32x {
    op: u8,
    aaaa: u16,
    bbbb: u16,
}

// Format AA|op BBBBlo BBBBhi
struct Format31i {
    op: u8,
    aa: u8,
    bbbb: i32,
}

// Format 31t
struct Format31t {
    op: u8,
    aa: u8,
    bbbb: i32,
}

// Format 31c
struct Format31c {
    op: u8,
    aa: u8,
    bbbb: u32,
}

// Format A|G|op BBBB F|E|D|C
// Same for 35ms and 35mi
struct Format35c {
    op: u8,
    g: u8,
    bbbb: u16,
    reg_list: [u8; 5], // Массив регистров vC, vD, vE, vF, vG
}

// Format AA|op BBBB CCCC
// Same for 3rms and 3rmi
struct Format3rc {
    op: u8,
    aa: u8,
    bbbb: u16,
    cccc: u16,
}

// Format A|G|op BBBB F|E|D|C HHHH
struct Format45cc {
    op: u8,
    g: u8, // This includes both the count of argument words (A) and a register (G).
    bbbb: u16, // Method index
    reg_list: [u8; 5], // List of registers vC, vD, vE, vF, vG
    hhhh: u16, // Index into the proto_ids list
}

// Format AA|op BBBB CCCC HHHH
struct Format4rcc {
    op: u8,
    aa: u8, // Count of argument words
    bbbb: u16, // Method index
    cccc: u16, // First register (vCCCC)
    hhhh: u16, // Index into the proto_ids list
}

// Format AA|op BBBBlo BBBB BBBB BBBBhi
struct Format51l {
    op: u8,
    aa: u8, // Register (vAA)
    bbbb_lo: u16, // Lower 16 bits of the literal value
    bbbb_hi: u16, // Higher 16 bits of the literal value
}