use std::io::Read;

use adler::adler32;
use buffered_reader::File;

use crate::dex::endian_aware_reader::{EndianAwareFileReader, EndianAwareReader, Endianness, LittleEndianReader};

#[derive(Debug)]
pub struct RawStringId {
    pub offset: u32,
}

#[derive(Debug)]
pub struct RawTypeId {
    pub descriptor_idx: u32,
}

#[derive(Debug)]
pub struct RawProtoId {
    pub shorty_idx: u32,
    pub return_type_idx: u32,
    pub parameters_off: u32,
}

#[derive(Debug)]
pub struct RawFieldId {
    pub class_idx: u16,
    pub type_idx: u16,
    pub name_idx: u32,
}

#[derive(Debug)]
pub struct RawMethodId {
    pub class_idx: u16,
    pub proto_idx: u16,
    pub name_idx: u32,
}


#[derive(Debug)]
pub struct RawShortHeader {
    pub version: u32,
    // checksum of everything except magic and checksum
    pub checksum: u32,
}

#[derive(Debug)]
pub struct RawHeader {
    pub version: u32,
    pub checksum: u32,
    pub signature: [u8; 20],
    pub file_size: u32,
    pub header_size: u32,
    pub endianness: Endianness,
    pub link_size: u32,
    pub link_offset: u32,
    pub map_offset: u32,
    pub string_ids_size: u32,
    pub string_ids_offset: u32,
    pub type_ids_size: u32,
    pub type_ids_offset: u32,
    pub proto_ids_size: u32,
    pub proto_ids_offset: u32,
    pub field_ids_size: u32,
    pub field_ids_offset: u32,
    pub method_ids_size: u32,
    pub method_ids_offset: u32,
    pub class_defs_size: u32,
    pub class_defs_offset: u32,
    pub data_size: u32,
    pub data_offset: u32,
}

#[derive(Debug)]
pub struct RawClassDef {
    pub class_idx: u32,
    pub access_flags: u32,
    pub superclass_idx: u32,
    pub interfaces_off: u32,
    pub source_file_idx: u32,
    pub annotations_off: u32,
    pub class_data_off: u32,
    pub static_values_off: u32,
}


#[derive(Debug, PartialEq)]
pub struct RawClassDataItem {
    pub static_fields: Vec<RawEncodedField>,
    pub instance_fields: Vec<RawEncodedField>,
    pub direct_methods: Vec<RawEncodedMethod>,
    pub virtual_methods: Vec<RawEncodedMethod>,
}

#[derive(Debug, PartialEq)]
pub struct RawEncodedField {
    pub field_idx: usize,
    pub access_flags: u32,
}

#[derive(Debug, PartialEq)]
pub struct RawEncodedMethod {
    pub method_idx: usize,
    pub access_flags: u32,
    pub code_off: u64,
}

#[derive(Debug, PartialEq)]
pub struct RawAnnotations {
    pub class_annotations_off: u32,
    pub fld_annot: Option<Vec<RawFieldAnnotation>>,
    pub mtd_annot: Option<Vec<RawMethodAnnotation>>,
    pub prm_annot: Option<Vec<RawParameterAnnotation>>,
}

#[derive(Debug, PartialEq)]
pub struct RawAnnotationSet {
    pub annotation_offsets: Vec<u32>,
}

#[derive(Debug, PartialEq)]
pub struct RawAnnotationSetRefList {
    pub annotation_set_offsets: Vec<Option<u32>>,
}

#[derive(Debug, PartialEq)]
pub struct RawFieldAnnotation {
    pub field_idx: u32,
    pub annotations_offset: u32,
}

#[derive(Debug, PartialEq)]
pub struct RawMethodAnnotation {
    pub method_idx: u32,
    pub annotations_offset: u32,
}

#[derive(Debug, PartialEq)]
pub struct RawParameterAnnotation {
    pub method_idx: u32,
    pub annotations_offset: u32,
}

// Docs: code_item
#[derive(Debug, PartialEq)]
pub struct RawCodeItem {
    // number of registers used by this code
    pub registers_size: u16,
    // number of words of incoming arguments
    pub ins_size: u16,
    // number of words of outgoing argument space
    pub outs_size: u16,
    pub tries_size: u16,
    pub debug_info_off: u32,
    pub insns_size: u32,
    pub insns: Vec<u16>,
    pub padding: Option<u16>,
    pub tries: Option<Vec<RawTryItem>>,
    // missing the handler here: need to break up processing to handle handler_off for try_item's
}


// Docs: try_item
#[derive(Debug, PartialEq)]
pub struct RawTryItem {
    // start address of the block of code covered by this entry
    // a count of 16-bit code units to the start of the first covered instruction
    pub start_addr: u32,
    // number of 16-bit code units covered by this entry
    pub insn_count: u16,
    // offset to the individual handler within the encoded_catch_handler_list block for the associated code_item
    pub handler_off: u16,
}

#[derive(Debug)]
pub struct RawDexFile {
    pub header: RawHeader,
    pub string_ids: Vec<RawStringId>,
    pub type_ids: Vec<RawTypeId>,
    pub proto_ids: Vec<RawProtoId>,
    pub field_ids: Vec<RawFieldId>,
    pub method_ids: Vec<RawMethodId>,
    pub class_defs: Vec<RawClassDef>,
}

fn parse_short_header<R: EndianAwareReader>(reader: &mut R) -> RawShortHeader {
    let inner = reader.inner();
    let magic_buf =
        inner.steal(8).expect("Failed to read magic");

    let magic =
        String::from_utf8(magic_buf)
            .expect("Failed to parse magic");

    if (!magic.starts_with("dex\n")) || (!magic.ends_with("\0")) {
        panic!("Invalid magic: {}", magic);
    }

    let version = &magic[4..7];
    let checksum = reader.read_u32().expect("Failed to read checksum");

    RawShortHeader {
        version: version.parse::<u32>()
            .expect(format!("Failed to parse version: {}", version).as_str()),
        checksum,
    }
}

fn parse_header<R: EndianAwareReader>(reader: &mut R, short_header: RawShortHeader) -> RawHeader {
    let mut signature = [0; 20];
    reader
        .inner()
        .read_exact(&mut signature).expect("Failed to read signature");

    RawHeader {
        version: short_header.version,
        checksum: short_header.checksum,
        signature: signature,
        file_size: reader.read_u32().expect("Failed to read file_size"),
        header_size: reader.read_u32().expect("Failed to read header_size"),
        endianness: match reader.read_u32().expect("Failed to read endianess") {
            0x12345678 => Endianness::Little,
            0x78563412 => Endianness::Big,
            endianess => panic!("Invalid endian tag: {}", endianess),
        },
        link_size: reader.read_u32().expect("Failed to read link_size"),
        link_offset: reader.read_u32().expect("Failed to read link_offset"),
        map_offset: reader.read_u32().expect("Failed to read map_offset"),
        string_ids_size: reader.read_u32().expect("Failed to read string_ids_size"),
        string_ids_offset: reader.read_u32().expect("Failed to read string_ids_offset"),
        type_ids_size: reader.read_u32().expect("Failed to read type_ids_size"),
        type_ids_offset: reader.read_u32().expect("Failed to read type_ids_offset"),
        proto_ids_size: reader.read_u32().expect("Failed to read proto_ids_size"),
        proto_ids_offset: reader.read_u32().expect("Failed to read proto_ids_offset"),
        field_ids_size: reader.read_u32().expect("Failed to read field_ids_size"),
        field_ids_offset: reader.read_u32().expect("Failed to read field_ids_offset"),
        method_ids_size: reader.read_u32().expect("Failed to read method_ids_size"),
        method_ids_offset: reader.read_u32().expect("Failed to read method_ids_offset"),
        class_defs_size: reader.read_u32().expect("Failed to read class_defs_size"),
        class_defs_offset: reader.read_u32().expect("Failed to read class_defs_size"),
        data_size: reader.read_u32().expect("Failed to read class_defs_size"),
        data_offset: reader.read_u32().expect("Failed to read class_defs_size"),
    }
}

fn parse_string_ids(reader: &mut EndianAwareFileReader, header: &RawHeader) -> Vec<RawStringId> {
    (0..header.string_ids_size)
        .map(|_| RawStringId {
            offset: reader.read_u32().expect("Failed to read string_id offset"),
        })
        .collect()
}

fn parse_type_ids(reader: &mut EndianAwareFileReader, header: &RawHeader) -> Vec<RawTypeId> {
    (0..header.type_ids_size)
        .map(|_| RawTypeId {
            descriptor_idx: reader.read_u32().expect("Failed to read descriptor_idx"),
        })
        .collect()
}

fn parse_proto_ids(reader: &mut EndianAwareFileReader, header: &RawHeader) -> Vec<RawProtoId> {
    (0..header.proto_ids_size)
        .map(|_| RawProtoId {
            shorty_idx: reader.read_u32().expect("Failed to read shorty_idx"),
            return_type_idx: reader.read_u32().expect("Failed to read return_type_idx"),
            parameters_off: reader.read_u32().expect("Failed to read parameters_off"),
        })
        .collect()
}

fn parse_field_ids(reader: &mut EndianAwareFileReader, header: &RawHeader) -> Vec<RawFieldId> {
    (0..header.field_ids_size)
        .map(|_| {
            RawFieldId {
                class_idx: reader.read_u16().expect("Failed to read class_idx"),
                type_idx: reader.read_u16().expect("Failed to read type_idx"),
                name_idx: reader.read_u32().expect("Failed to read name_idx"),
            }
        })
        .collect()
}

fn parse_method_ids(reader: &mut EndianAwareFileReader, header: &RawHeader) -> Vec<RawMethodId> {
    (0..header.method_ids_size)
        .map(|_| RawMethodId {
            class_idx: reader.read_u16().expect("Failed to read class_idx"),
            proto_idx: reader.read_u16().expect("Failed to read proto_idx"),
            name_idx: reader.read_u32().expect("Failed to read name_idx"),
        })
        .collect()
}

fn parse_class_defs(file: &mut EndianAwareFileReader, header: &RawHeader) -> Vec<RawClassDef> {
    (0..header.class_defs_size)
        .map(|_| RawClassDef {
            class_idx: file.read_u32().expect("Failed to read class_idx"),
            access_flags: file.read_u32().expect("Failed to read access_flags"),
            superclass_idx: file.read_u32().expect("Failed to read superclass_idx"),
            interfaces_off: file.read_u32().expect("Failed to read interfaces_off"),
            source_file_idx: file.read_u32().expect("Failed to read source_file_idx"),
            annotations_off: file.read_u32().expect("Failed to read annotations_off"),
            class_data_off: file.read_u32().expect("Failed to read class_data_off"),
            static_values_off: file.read_u32().expect("Failed to read static_values_off"),
        })
        .collect::<Vec<RawClassDef>>()
}

const MAGIC_LEN: usize = 8;
const CHECKSUM_LEN: usize = 4;

pub fn open_file(path: &str) -> File<()> {
    buffered_reader::File::open(path).unwrap()
}

pub fn parse_raw_dex_file(path: &str) -> RawDexFile {
    let mut file = open_file(path);
    let mut reader = EndianAwareFileReader::Little(
        LittleEndianReader {
            inner: file,
        }
    );
    let short_header = parse_short_header(&mut reader);
    checksum(&mut reader, short_header.checksum);

    file = open_file("tests/vm/hello.dex");
    reader = EndianAwareFileReader::Little(
        LittleEndianReader {
            inner: file,
        }
    );
    let mut buffer = [0; MAGIC_LEN + CHECKSUM_LEN];
    reader.inner().read_exact(&mut buffer).unwrap();

    let header = parse_header(&mut reader, short_header);
    let string_ids = parse_string_ids(&mut reader, &header);
    let type_ids = parse_type_ids(&mut reader, &header);
    let proto_ids = parse_proto_ids(&mut reader, &header);
    let field_ids = parse_field_ids(&mut reader, &header);
    let method_ids = parse_method_ids(&mut reader, &header);
    let class_defs = parse_class_defs(&mut reader, &header);

    RawDexFile {
        header,
        string_ids,
        type_ids,
        proto_ids,
        field_ids,
        method_ids,
        class_defs,
    }
}

fn checksum(file: &mut EndianAwareFileReader, checksum: u32) {
    // calculate Adler32 checksum
    let file_checksum =
        adler32(file.inner().buffer())
            .expect("Failed to calculate checksum");

    if file_checksum != checksum {
        panic!("Checksum mismatch: {} != {}", file_checksum, checksum);
    }
}
