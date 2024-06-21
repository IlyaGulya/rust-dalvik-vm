use std::{f64, io};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Cursor, Read, Seek, SeekFrom};
use std::rc::Rc;
use std::sync::Arc;

use bitstream_io::{BitRead, BitReader};
use byteorder::{LittleEndian, ReadBytesExt};
use regex::Regex;

use crate::bytecode::formats::{Format31t, Format35c, Format3rc};
use crate::bytecode::instructions::{ArrayOp, ArrayOpData, BinaryOp, BinaryOp2Addr, BinaryOp2AddrData, BinaryOpData, BinaryOpLit16, BinaryOpLit16Data, BinaryOpLit8, BinaryOpLit8Data, CmpOp, CmpOpData, ConstOp, FillArrayData, IfTestOp, IfTestOpData, IfTestZOp, IfTestZOpData, InstanceFieldOp, InstanceFieldOpData, Instruction, InvokeOp, InvokeOpData, InvokeRangeOp, InvokeRangeOpData, PackedSwitchTable, SparseSwitchTable, StaticFieldOp, StaticFieldOpData, UnaryOp, UnaryOpData};
use crate::dex::access_flags::AccessFlags;
use crate::dex::endian_aware_reader::{Endianness, Leb128Ext, MUtf8Ext};
use crate::dex::raw_dex_file::{RawAnnotationSet, RawAnnotationSetRefList, RawClassDataItem, RawClassDef, RawDexFile, RawEncodedField, RawEncodedMethod, RawFieldAnnotation, RawMethodAnnotation};

#[derive(Debug, PartialEq)]
pub struct DexFile {
    pub header: Header,
    pub data: DexFileData,
    pub classes: Vec<Arc<ClassDefinition>>,
}

// TODO: Replace Rc<String> with Rc<str>?
#[derive(Debug, PartialEq)]
pub struct DexFileData {
    pub string_data: Vec<Arc<String>>,
    pub type_identifiers: Vec<Arc<String>>,
    pub prototypes: Vec<Arc<Prototype>>,
    pub fields: Vec<Arc<Field>>,
    pub methods: Vec<Arc<Method>>,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub version: u32,
    pub checksum: u32,
    pub signature: [u8; 20],
    pub file_size: u32,
    pub endianness: Endianness,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Prototype {
    pub shorty: Arc<String>,
    pub return_type: Arc<String>,
    pub parameters: Vec<Arc<String>>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub definer: Arc<String>,
    pub type_: Arc<String>,
    pub name: Arc<String>,
}

#[derive(PartialEq)]
pub struct Method {
    pub definer: Arc<String>,
    pub prototype: Arc<Prototype>,
    pub name: Arc<String>,
}

impl Method {
    pub fn full_name(&self) -> String {
        format!("{}->{}{}", self.definer, self.name, self.full_descriptor())
    }
    pub fn full_name_human_readable(&self) -> String {
        let re = Regex::new(r"(^L|;)").unwrap();
        re.replace_all(
            self.full_name()
                .replace("/", ".")
                .replace("->", ".").as_str(),
            "",
        ).to_string()
    }
    pub fn full_descriptor(&self) -> String {
        let mut descriptor = String::new();

        descriptor.push('(');

        for parameter in &self.prototype.parameters {
            descriptor.push_str(parameter);
        }
        descriptor.push(')');
        descriptor.push_str(self.prototype.return_type.as_str());

        descriptor
    }
}

impl Debug for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name_human_readable())
    }
}

#[derive(Debug, PartialEq)]
pub struct ClassDefinition {
    pub class_type: Arc<String>,
    pub access_flags: AccessFlags,
    pub superclass: Option<Arc<String>>,
    pub interfaces: Vec<Arc<String>>,
    pub source_file_name: Option<Arc<String>>,
    pub annotations: Option<Annotations>,
    pub class_data: Option<ClassData>,
    pub static_values: Vec<EncodedValue>,
}

#[derive(Debug, PartialEq)]
pub struct Annotations {
    pub class_annotations: Vec<ClassAnnotation>,
    pub field_annotations: Vec<FieldAnnotation>,
    pub method_annotations: Vec<MethodAnnotation>,
    pub parameter_annotations: Vec<ParameterAnnotation>,
}

#[derive(Debug, PartialEq)]
pub struct ClassAnnotation {
    pub visibility: Visibility,
    pub type_: Arc<String>,
    pub elements: Vec<AnnotationElement>,
}

#[derive(Debug, PartialEq)]
pub struct MethodAnnotation {
    pub method: Arc<Method>,
    pub annotations: Vec<AnnotationItem>,
}

#[derive(Debug, PartialEq)]
pub struct ParameterAnnotation {
    pub method: Arc<Method>,
    pub annotations: Vec<Vec<AnnotationItem>>,
}

#[derive(Debug, PartialEq)]
pub struct FieldAnnotation {
    pub field: Arc<Field>,
    pub annotations: Vec<AnnotationItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnnotationItem {
    pub visibility: Visibility,
    pub type_: Arc<String>,
    pub elements: Vec<AnnotationElement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Visibility {
    BUILD,
    RUNTIME,
    SYSTEM,
}

impl TryInto<Visibility> for u8 {
    type Error = String;

    fn try_into(self) -> Result<Visibility, Self::Error> {
        match self {
            0x00 => Ok(Visibility::BUILD),
            0x01 => Ok(Visibility::RUNTIME),
            0x02 => Ok(Visibility::SYSTEM),
            _ => Err(format!("Invalid visibility value: {}", self)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ClassData {
    pub static_fields: Vec<Arc<EncodedField>>,
    pub instance_fields: Vec<Arc<EncodedField>>,
    pub direct_methods: Vec<Arc<EncodedMethod>>,
    pub virtual_methods: Vec<Arc<EncodedMethod>>,
}

#[derive(Debug, PartialEq)]
pub struct EncodedField {
    pub field: Arc<Field>,
    pub access_flags: AccessFlags,
}

#[derive(Debug, PartialEq)]
pub struct EncodedMethod {
    pub method: Arc<Method>,
    pub access_flags: AccessFlags,
    pub code: Option<Arc<Code>>,
}

// Docs: code_item
#[derive(Debug, PartialEq)]
pub struct Code {
    pub file_offset: u64,
    // number of registers used by this code
    pub registers_size: u16,
    // number of words of incoming arguments
    pub ins_size: u16,
    // number of words of outgoing argument space
    pub outs_size: u16,
    pub debug_info: Option<DebugInfo>,
    pub raw_instructions: Vec<u16>,
    pub instructions: Vec<Instruction>,
    pub tries: Vec<TryItem>,
    pub handlers: Vec<EncodedCatchHandler>,
}

// Docs: try_item
#[derive(Debug, PartialEq)]
pub struct TryItem {
    pub start_addr: u32,
    pub insn_count: u16,
    pub handler_offset: u16,
}

// Docs: encoded_catch_handler
#[derive(Debug, PartialEq)]
pub struct EncodedCatchHandler {
    pub handlers: Vec<EncodedTypeAddrPair>,
    // bytecode
    // only present if size is non-positive
    pub catch_all_addr: Option<u64>,
}

// Docs: encoded_type_addr_pair
#[derive(Debug, PartialEq)]
pub struct EncodedTypeAddrPair {
    // index into type_ids list for the type of exception to catch
    pub type_: Arc<String>,
    // bytecode address of associated exception handler
    pub addr: u64,
}

// Docs: debug_info_item
#[derive(Debug, PartialEq)]
pub struct DebugInfo {
    pub line_start: u64,
    pub parameter_names: Vec<Option<Arc<String>>>,
    pub bytecode: Vec<DebugItemBytecodes>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EncodedValue {
    Byte(u8),
    Short(i16),
    Char(u16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    MethodType(Arc<Prototype>),
    MethodHandle(Arc<Method>),
    String(Arc<String>),
    Type(Arc<String>),
    Field(Arc<Field>),
    Method(Arc<Method>),
    Enum(Arc<Field>),
    Array(Vec<EncodedValue>),
    Annotation(EncodedAnnotationItem),
    Null,
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EncodedAnnotationItem {
    pub type_: Arc<String>,
    pub values: Vec<AnnotationElement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnnotationElement {
    pub name: Arc<String>,
    pub value: EncodedValue,
}

//noinspection RsEnumVariantNaming
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum DebugItemBytecodes {
    DBG_END_SEQUENCE,
    DBG_ADVANCE_PC,
    DBG_ADVANCE_LINE,
    DBG_START_LOCAL,
    DBG_START_LOCAL_EXTENDED,
    DBG_END_LOCAL,
    DBG_RESTART_LOCAL,
    DBG_SET_PROLOGUE_END,
    DBG_SET_EPILOGUE_BEGIN,
    DBG_SET_FILE,
    SPECIAL_OPCODE(u8),
}

const NO_INDEX: u32 = 0xffffffff;

pub fn parse_dex_file(raw: RawDexFile, path: &str) -> DexFile {
    let mut file = File::open(path).expect("Unable to open dex file");
    let string_data: Vec<_> = raw.string_ids.iter().map(|string_id| {
        file.seek(SeekFrom::Start(string_id.offset as u64))
            .expect(format!("Failed to seek to string data position {}",
                            string_id.offset).as_str());
        let utf16_size = file.read_uleb128().expect("Failed to read string data size");
        Arc::new(file.read_mutf8(utf16_size).expect(format!("Failed to read string data of size {} at {}", utf16_size, string_id.offset).as_str()))
    }).collect();
    let type_identifiers: Vec<_> = raw.type_ids.iter().map(|type_id| {
        string_data[type_id.descriptor_idx as usize].clone()
    }).collect();
    let prototypes: Vec<_> =
        raw.proto_ids.iter().map(|proto_id| {
            Arc::new(Prototype {
                shorty: string_data[proto_id.shorty_idx as usize].clone(),
                return_type: type_identifiers[proto_id.return_type_idx as usize].clone(),
                parameters: parse_type_list(&mut file, &type_identifiers, proto_id.parameters_off),
            })
        }).collect();

    let fields: Vec<_> = raw.field_ids.iter().map(|field_id| {
        Arc::new(Field {
            definer: type_identifiers[field_id.class_idx as usize].clone(),
            type_: type_identifiers[field_id.type_idx as usize].clone(),
            name: string_data[field_id.name_idx as usize].clone(),
        })
    }).collect();

    let methods: Vec<_> = raw.method_ids.iter().map(|method_id| {
        Arc::new(Method {
            definer: type_identifiers[method_id.class_idx as usize].clone(),
            prototype: prototypes[method_id.proto_idx as usize].clone(),
            name: string_data[method_id.name_idx as usize].clone(),
        })
    }).collect();

    let data = DexFileData {
        string_data,
        type_identifiers,
        prototypes,
        fields,
        methods,
    };

    let classes: Vec<_> = parse_classes(&mut file, &data, &raw.class_defs);

    return DexFile {
        header: Header {
            version: raw.header.version,
            checksum: raw.header.checksum,
            signature: raw.header.signature,
            file_size: raw.header.file_size,
            endianness: raw.header.endianness,
        },
        data,
        classes,
    };
}

fn parse_classes(file: &mut File, data: &DexFileData, class_defs: &Vec<RawClassDef>) -> Vec<Arc<ClassDefinition>> {
    class_defs.iter().map(|class_def| {
        let class = data.type_identifiers[class_def.class_idx as usize].clone();
        Arc::new(
            ClassDefinition {
                class_type: class,
                access_flags: AccessFlags::from(class_def.access_flags),
                superclass: if class_def.superclass_idx != NO_INDEX {
                    Some(data.type_identifiers[class_def.superclass_idx as usize].clone())
                } else { None },
                interfaces: parse_type_list(file, &data.type_identifiers, class_def.interfaces_off),
                source_file_name: if class_def.source_file_idx != NO_INDEX {
                    Some(data.string_data[class_def.source_file_idx as usize].clone())
                } else { None },
                annotations: parse_annotations(file, &data, class_def.annotations_off),
                class_data: parse_class_data(
                    file,
                    &data,
                    class_def.class_data_off,
                ),
                static_values: vec![],
            }
        )
    }).collect()
}

trait Parser: io::Read {
    fn parse_list<T, F>(&mut self, size: u64, mut parser: F) -> Vec<T>
        where
            F: FnMut(&mut Self) -> T,
    {
        (0..size).map(|_| parser(self)).collect()
    }
}

impl<R: Read> Parser for R {}

trait Also {
    fn also<F: FnOnce(&Self) -> ()>(self, f: F) -> Self where Self: Sized {
        f(&self);
        self
    }
}

impl<T> Also for T {}


fn parse_class_data(
    file: &mut File,
    data: &DexFileData,
    offset: u32,
) -> Option<ClassData> {
    if offset == 0 {
        return None;
    }

    file.seek(SeekFrom::Start(offset as u64))
        .expect(format!("Failed to seek to class_data_off position {}", offset).as_str());


    let static_fields_size = file.read_uleb128().expect("Failed to read static_fields_size");
    let instance_fields_size = file.read_uleb128().expect("Failed to read instance_fields_size");
    let direct_methods_size = file.read_uleb128().expect("Failed to read direct_methods_size");
    let virtual_methods_size = file.read_uleb128().expect("Failed to read virtual_methods_size");

    let raw = RawClassDataItem {
        static_fields: parse_raw_encoded_fields(file, static_fields_size),
        instance_fields: parse_raw_encoded_fields(file, instance_fields_size),
        direct_methods: parse_raw_encoded_methods(file, direct_methods_size),
        virtual_methods: parse_raw_encoded_methods(file, virtual_methods_size),
    };

    Some(ClassData {
        static_fields: parse_encoded_fields(raw.static_fields, data),
        instance_fields: parse_encoded_fields(raw.instance_fields, data),
        direct_methods: parse_encoded_methods(file, raw.direct_methods, data),
        virtual_methods: parse_encoded_methods(file, raw.virtual_methods, data),
    })
}

fn parse_raw_encoded_fields(file: &mut File, size: u64) -> Vec<RawEncodedField> {
    let mut prev_field_idx = 0u64;
    file.parse_list(size, |file| {
        let field_idx_diff = file.read_uleb128().expect("Failed to read field_idx_diff");
        RawEncodedField {
            field_idx: (prev_field_idx + field_idx_diff) as usize,
            access_flags: file.read_uleb128().expect("Failed to read access_flags") as u32,
        }.also(|_| {
            prev_field_idx = field_idx_diff;
        })
    })
}

fn parse_raw_encoded_methods(file: &mut File, size: u64) -> Vec<RawEncodedMethod> {
    let mut prev_method_idx = 0;
    file.parse_list(size, |file| {
        let method_idx_diff = file.read_uleb128().expect("Failed to read method_idx_diff");
        RawEncodedMethod {
            method_idx: (prev_method_idx + method_idx_diff) as usize,
            access_flags: file.read_uleb128().expect("Failed to read access_flags") as u32,
            code_off: file.read_uleb128().expect("Failed to read code_off"),
        }.also(|_| {
            prev_method_idx += method_idx_diff;
        })
    })
}

fn parse_encoded_fields(fields: Vec<RawEncodedField>, data: &DexFileData) -> Vec<Arc<EncodedField>> {
    fields
        .iter()
        .map(|field| {
            EncodedField {
                field: data.fields[field.field_idx].clone(),
                access_flags: AccessFlags::from(field.access_flags),
            }
        })
        .map(|field| Arc::new(field))
        .collect()
}

fn parse_encoded_methods(file: &mut File, methods: Vec<RawEncodedMethod>, data: &DexFileData) -> Vec<Arc<EncodedMethod>> {
    methods
        .iter()
        .map(|raw| {
            let method = data.methods[raw.method_idx].clone();
            let access_flags = AccessFlags::from(raw.access_flags);

            // println!("Parsing method: {:?}", method);

            EncodedMethod {
                method: method.clone(),
                access_flags,
                code: if raw.code_off != 0 {
                    Some(Arc::new(parse_code(file, data, raw.code_off)))
                } else {
                    let is_abstract = access_flags.contains(AccessFlags::ACC_ABSTRACT);
                    let is_native = access_flags.contains(AccessFlags::ACC_NATIVE);
                    if !is_abstract && !is_native {
                        panic!("Non-abstract and non-native method without code: {:?}", method);
                    }
                    None
                },
            }
        })
        .map(|method| Arc::new(method))
        .collect()
}

fn parse_code(file: &mut File, data: &DexFileData, offset: u64) -> Code {
    file.seek(SeekFrom::Start(offset))
        .expect(format!("Failed to seek to code offset {}", offset).as_str());

    let registers_size = file.read_u16::<LittleEndian>().expect("Failed to read registers_size");
    let ins_size = file.read_u16::<LittleEndian>().expect("Failed to read ins_size");
    let outs_size = file.read_u16::<LittleEndian>().expect("Failed to read outs_size");
    let tries_size = file.read_u16::<LittleEndian>().expect("Failed to read tries_size");
    let debug_info_offset = file.read_u32::<LittleEndian>().expect("Failed to read debug_info offset");
    let insns_count = file.read_u32::<LittleEndian>().expect("Failed to read insns_count");
    let raw_insns = parse_raw_insns(file, insns_count);
    // println!("raw_insns.len() = {}. insns_count = {}", raw_insns.len(), insns_count);
    let mut raw_u16_instructions: Vec<u16> = vec![];
    let mut cursor = Cursor::new(&raw_insns);

    loop {
        let current_position = cursor.position() / 2;
        if current_position == insns_count as u64 {
            break;
        }

        raw_u16_instructions.push(
            cursor.read_u16::<LittleEndian>().expect("Unable to read instruction")
        )
    }


    if tries_size != 0 && insns_count % 2 != 0 {
        file.read_u16::<LittleEndian>().expect("Failed to skip padding");
    }

    let tries = parse_tries(file, tries_size);
    let handlers = if tries.is_empty() {
        vec![]
    } else {
        parse_handlers(file, data)
    };

    let debug_info = parse_debug_info(file, data, debug_info_offset);
    Code {
        file_offset: offset,
        registers_size: registers_size,
        ins_size: ins_size,
        outs_size: outs_size,
        debug_info: debug_info,
        raw_instructions: raw_u16_instructions,
        instructions: parse_instructions(raw_insns, data),
        tries: tries,
        handlers: handlers,
    }
}

pub fn parse_instructions(raw_instructions: Vec<u8>, data: &DexFileData) -> Vec<Instruction> {
    // TODO: Not sure how to respect endianness inside single instruction. Currently parsing in little-endian only.
    let slice = raw_instructions.as_slice();
    // TODO: BitReader reads in big-endian only. Need to implement custom reader or create pull request to BitReader.
    let mut reader = BufReader::new(Cursor::new(slice));
    let mut result = vec![];
    let mut packed_switch_tables: HashMap<u64, Arc<PackedSwitchTable>> = HashMap::new();
    let mut sparse_switch_tables: HashMap<u64, Arc<SparseSwitchTable>> = HashMap::new();
    let mut fill_array_datas: HashMap<u64, Arc<FillArrayData>> = HashMap::new();

    loop {
        let instruction_pos = reader.stream_position().expect("Failed to get instruction position");
        let opcode = reader.read_u8();
        if opcode.is_err() {
            break;
        }
        let opcode = opcode.unwrap();
        let instruction = match opcode {
            0x00 => {
                let padding = reader.read_u8().expect("Failed to read NOP padding");
                match padding {
                    0x01 => {
                        let table =
                            packed_switch_tables.get(&instruction_pos)
                                .expect(&format!("Failed to get packed switch table for position {}", instruction_pos));

                        let size_in_bytes = table.size_in_code_units * 2;
                        let next_instruction_pos = instruction_pos + size_in_bytes as u64;
                        reader.seek(SeekFrom::Start(next_instruction_pos))
                            .expect(&format!("Failed to skip packed switch table and seek to next instruction position {}", next_instruction_pos));
                        continue;
                    }
                    0x02 => {
                        let table =
                            sparse_switch_tables.get(&instruction_pos)
                                .expect(&format!("Failed to get packed switch table for position {}", instruction_pos));

                        let size_in_bytes = table.size_in_code_units * 2;
                        let next_instruction_pos = instruction_pos + size_in_bytes as u64;
                        reader.seek(SeekFrom::Start(next_instruction_pos))
                            .expect(&format!("Failed to skip sparse switch table and seek to next instruction position {}", next_instruction_pos));
                        continue;
                    }
                    0x03 => {
                        let data = fill_array_datas.get(&instruction_pos)
                            .expect(&format!("Failed to get fill array data for position {}", instruction_pos));
                        let size_in_bytes = data.size_in_code_units * 2;
                        let next_instruction_pos = instruction_pos + size_in_bytes as u64;
                        reader.seek(SeekFrom::Start(next_instruction_pos))
                            .expect(&format!("Failed to skip fill array data and seek to next instruction position {}", next_instruction_pos));
                        continue;
                    }
                    0 => {} // correct padding
                    _ => panic!("Invalid NOP padding: {}", padding)
                }
                Instruction::NOP
            }
            0x01 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                Instruction::MOVE {
                    dest: reader.read::<u8>(4).expect("Failed to read dest"),
                    src: reader.read::<u8>(4).expect("Failed to read src"),
                }
            }
            0x02 => Instruction::MOVE_FROM16 {
                dest: reader.read_u8().expect("Failed to read dest"),
                src: reader.read_u16::<LittleEndian>().expect("Failed to read src"),
            },
            0x03 => Instruction::MOVE_16 {
                dest: reader.read_u16::<LittleEndian>().expect("Failed to read dest"),
                src: reader.read_u16::<LittleEndian>().expect("Failed to read src"),
            },
            0x04 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                Instruction::MOVE_WIDE {
                    dest: reader.read::<u8>(4).expect("Failed to read dest"),
                    src: reader.read::<u8>(4).expect("Failed to read src"),
                }
            }
            0x05 => Instruction::MOVE_WIDE_FROM16 {
                dest: reader.read_u8().expect("Failed to read dest"),
                src: reader.read_u16::<LittleEndian>().expect("Failed to read src"),
            },
            0x06 => Instruction::MOVE_WIDE_16 {
                dest: reader.read_u16::<LittleEndian>().expect("Failed to read dest"),
                src: reader.read_u16::<LittleEndian>().expect("Failed to read src"),
            },
            0x07 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                Instruction::MOVE_OBJECT {
                    dest: reader.read::<u8>(4).expect("Failed to read dest"),
                    src: reader.read::<u8>(4).expect("Failed to read src"),
                }
            }
            0x08 => Instruction::MOVE_OBJECT_FROM16 {
                dest: reader.read_u8().expect("Failed to read dest"),
                src: reader.read_u16::<LittleEndian>().expect("Failed to read src"),
            },
            0x09 => Instruction::MOVE_OBJECT_16 {
                dest: reader.read_u16::<LittleEndian>().expect("Failed to read dest"),
                src: reader.read_u16::<LittleEndian>().expect("Failed to read src"),
            },
            0x0a => Instruction::MOVE_RESULT(
                reader.read_u8().expect("Failed to read dest")
            ),
            0x0b => Instruction::MOVE_RESULT_WIDE(
                reader.read_u8().expect("Failed to read dest")
            ),
            0x0c => Instruction::MOVE_RESULT_OBJECT(
                reader.read_u8().expect("Failed to read dest")
            ),
            0x0d => Instruction::MOVE_EXCEPTION(
                reader.read_u8().expect("Failed to read dest")
            ),
            0x0e => Instruction::RETURN_VOID.also(|_| {
                reader.read_u8().expect("Failed to read padding");
            }),
            0x0f => Instruction::RETURN(
                reader.read_u8().expect("Failed to read return register")
            ),
            0x10 => Instruction::RETURN_WIDE(
                reader.read_u8().expect("Failed to read return register")
            ),
            0x11 => Instruction::RETURN_OBJECT(
                reader.read_u8().expect("Failed to read return register")
            ),
            0x12 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                Instruction::Const(
                    ConstOp::CONST_4 {
                        dest_register: reader.read::<u8>(4).expect("Failed to read dest"),
                        literal: reader.read::<i8>(4).expect("Failed to read literal"),
                    }
                )
            }
            0x13 => {
                Instruction::Const(
                    ConstOp::CONST_16 {
                        dest_register: reader.read_u8().expect("Failed to read dest"),
                        literal: reader.read_i16::<LittleEndian>().expect("Failed to read literal"),
                    }
                )
            }
            0x14 => {
                let dest_register = reader.read_u8().expect("Failed to read dest");
                let mut literal = [0; 4];
                reader.read_exact(&mut literal).expect("Failed to read literal");
                Instruction::Const(
                    ConstOp::CONST {
                        dest_register,
                        literal,
                    }
                )
            }
            0x15 => {
                let dest = reader.read_u8().expect("Failed to read dest");
                let high16 = reader.read_i16::<LittleEndian>().expect("Failed to read literal");
                Instruction::Const(
                    ConstOp::CONST_HIGH_16 {
                        dest_register: dest,
                        literal: (high16 as i32) << 16,
                    }
                )
            }
            0x16 => {
                Instruction::Const(
                    ConstOp::CONST_WIDE_16 {
                        dest_register: reader.read_u8().expect("Failed to read dest"),
                        literal: reader.read_i16::<LittleEndian>().expect("Failed to read literal"),
                    }
                )
            }

            0x17 => {
                Instruction::Const(
                    ConstOp::CONST_WIDE_32 {
                        dest_register: reader.read_u8().expect("Failed to read dest"),
                        literal: reader.read_i32::<LittleEndian>().expect("Failed to read literal"),
                    }
                )
            }
            0x18 => {
                let dest = reader.read_u8().expect("Failed to read dest");
                let mut buf: [u8; 4] = [0; 4];
                reader.read_exact(&mut buf).expect("Failed to read literal");
                Instruction::Const(
                    ConstOp::CONST_WIDE {
                        dest_register: dest,
                        literal: reader.read_i32::<LittleEndian>().expect("Failed to read literal"),
                    }
                )
            }
            0x19 => {
                let dest = reader.read_u8().expect("Failed to read dest");
                let high16 = reader.read_i16::<LittleEndian>().expect("Failed to read literal");
                Instruction::Const(
                    ConstOp::CONST_WIDE_HIGH_16 {
                        dest_register: dest,
                        literal: (high16 as i64) << 48,
                    }
                )
            }
            0x1a => {
                Instruction::Const(
                    ConstOp::CONST_STRING {
                        dest_register: reader.read_u8().expect("Failed to read register"),
                        string: data.string_data[
                            reader
                                .read_u16::<LittleEndian>()
                                .expect("Failed to read string_id") as usize
                            ].clone(),
                    }
                )
            }
            0x1b => {
                Instruction::Const(
                    ConstOp::CONST_STRING_JUMBO {
                        dest_register: reader.read_u8().expect("Failed to read register"),
                        string: data.string_data[
                            reader
                                .read_u32::<LittleEndian>()
                                .expect("Failed to read string_id") as usize
                            ].clone(),
                    }
                )
            }
            0x1c => {
                Instruction::Const(
                    ConstOp::CONST_CLASS {
                        dest_register: reader.read_u8().expect("Failed to read register"),
                        class: data.type_identifiers[
                            reader
                                .read_u16::<LittleEndian>()
                                .expect("Failed to read type_id") as usize
                            ].clone(),
                    }
                )
            }
            0x1d => Instruction::MONITOR_ENTER(
                reader.read_u8().expect("Failed to read register")
            ),
            0x1e => Instruction::MONITOR_EXIT(
                reader.read_u8().expect("Failed to read register")
            ),
            0x1f => Instruction::CHECK_CAST {
                register: reader.read_u8().expect("Failed to read register"),
                class: data.type_identifiers[
                    reader
                        .read_u16::<LittleEndian>()
                        .expect("Failed to read type_id") as usize
                    ].clone(),
            },
            0x20 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                let dest = reader.read::<u8>(4).expect("Failed to read dest");
                let object = reader.read::<u8>(4).expect("Failed to read object_register");
                let type_id = reader
                    .read::<u16>(16)
                    .expect("Failed to read type_id") as usize;
                Instruction::INSTANCE_OF {
                    dest_register: dest,
                    object_register: object,
                    class: data.type_identifiers[type_id].clone(),
                }
            }
            0x21 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                Instruction::ARRAY_LENGTH {
                    dest_register: reader.read::<u8>(4).expect("Failed to read dest"),
                    array_register: reader.read::<u8>(4).expect("Failed to read array_register"),
                }
            }
            0x22 => Instruction::NEW_INSTANCE {
                register: reader.read_u8().expect("Failed to read register"),
                class: data.type_identifiers[
                    reader
                        .read_u16::<LittleEndian>()
                        .expect("Failed to read type_id") as usize
                    ].clone(),
            },
            0x23 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                Instruction::NEW_ARRAY {
                    dest_register: reader.read::<u8>(4).expect("Failed to read dest"),
                    size_register: reader.read::<u8>(4).expect("Failed to read size_register"),
                    type_: data.type_identifiers[
                        reader
                            .read::<u16>(16)
                            .expect("Failed to read type_id") as usize
                        ].clone(),
                }
            }
            0x24 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                let format = Format35c::parse(&mut reader);
                Instruction::FILLED_NEW_ARRAY {
                    registers: format.reg_list,
                    type_: data.type_identifiers[format.bbbb as usize].clone(),
                }
            }
            0x25 => {
                let format = Format3rc::parse(&mut reader);
                Instruction::FILLED_NEW_ARRAY_RANGE {
                    type_: data.type_identifiers[format.bbbb as usize].clone(),
                    start_register: format.cccc,
                    register_count: format.reg_count,
                }
            }
            0x26 => {
                let format = Format31t::parse(&mut reader);
                let current_position = reader.stream_position().expect("Failed to get current position");
                let offset_in_bytes = format.bbbbbbbb * 2;
                let fill_data_position =
                    if offset_in_bytes > 0 {
                        instruction_pos
                            .checked_add(offset_in_bytes as u64)
                    } else {
                        instruction_pos
                            .checked_sub(offset_in_bytes.abs() as u64)
                    }.expect(&format!("Failed to calculate fill data position. Invalid offset {}", offset_in_bytes));

                Instruction::FILL_ARRAY_DATA {
                    array_register: format.aa,
                    data: parse_fill_array_data(
                        &mut reader,
                        &mut fill_array_datas,
                        fill_data_position,
                    ).also(|_| {
                        reader.seek(SeekFrom::Start(current_position))
                            .expect("Failed to seek back to next instruction position");
                    }),
                }
            }
            0x27 => Instruction::THROW(
                reader.read_u8().expect("Failed to read register")
            ),
            0x28 => Instruction::GOTO(
                reader.read_i8().expect("Failed to read offset")
            ),
            0x29 => {
                reader.read_u8().expect("Failed to read padding");
                Instruction::GOTO_16(
                    reader.read_i16::<LittleEndian>().expect("Failed to read offset")
                )
            }
            0x2a => {
                reader.read_u8().expect("Failed to read padding");
                Instruction::GOTO_32(
                    reader.read_i32::<LittleEndian>().expect("Failed to read offset")
                )
            }
            0x2b => {
                let data = Format31t::parse(&mut reader);
                let current_position = reader.stream_position().expect("Failed to get current position");

                // converting from code units to bytes since we are reading bytes
                let offset_in_bytes = data.bbbbbbbb * 2;
                let table_position =
                    if offset_in_bytes > 0 {
                        instruction_pos
                            .checked_add(offset_in_bytes as u64)
                    } else {
                        instruction_pos
                            .checked_sub(offset_in_bytes.abs() as u64)
                    }.expect(&format!("Failed to calculate table position. Invalid offset {}", data.bbbbbbbb));

                Instruction::PACKED_SWITCH {
                    register: data.aa,
                    table: parse_packed_switch_table(
                        &mut reader,
                        &mut packed_switch_tables,
                        table_position,
                    ).also(|_| {
                        reader.seek(SeekFrom::Start(current_position))
                            .expect("Failed to seek back to next instruction position");
                    }),
                }
            }
            0x2c => {
                let data = Format31t::parse(&mut reader);
                let current_position = reader.stream_position().expect("Failed to get current position");

                // converting from code units to bytes since we are reading bytes
                let offset_in_bytes = data.bbbbbbbb * 2;
                let table_position =
                    if offset_in_bytes > 0 {
                        instruction_pos
                            .checked_add(offset_in_bytes as u64)
                    } else {
                        instruction_pos
                            .checked_sub(offset_in_bytes.abs() as u64)
                    }.expect(&format!("Failed to calculate table position. Invalid offset {}", data.bbbbbbbb));

                Instruction::SPARSE_SWITCH {
                    register: data.aa,
                    table: parse_sparse_switch_table(
                        &mut reader,
                        &mut sparse_switch_tables,
                        table_position,
                    ).also(|_| {
                        reader.seek(SeekFrom::Start(current_position))
                            .expect("Failed to seek back to next instruction position");
                    }),
                }
            }
            0x2d..=0x31 => {
                let data = CmpOpData {
                    destination_register: reader.read_u8().expect("Failed to read destination_register"),
                    register_a: reader.read_u8().expect("Failed to read register_a"),
                    register_b: reader.read_u8().expect("Failed to read register_b"),
                };

                Instruction::Cmp(
                    match opcode {
                        0x2d => CmpOp::CMPL_FLOAT(data),
                        0x2e => CmpOp::CMPG_FLOAT(data),
                        0x2f => CmpOp::CMPL_DOUBLE(data),
                        0x30 => CmpOp::CMPG_DOUBLE(data),
                        0x31 => CmpOp::CMP_LONG(data),
                        _ => panic!("Unreachable")
                    })
            }
            0x32..=0x37 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                let data = IfTestOpData {
                    register_a: reader.read::<u8>(4).expect("Failed to read register_a"),
                    register_b: reader.read::<u8>(4).expect("Failed to read register_b"),
                    offset: reader.read::<i16>(16).expect("Failed to read offset"),
                };

                Instruction::IfTest(
                    match opcode {
                        0x32 => IfTestOp::IF_EQ(data),
                        0x33 => IfTestOp::IF_NE(data),
                        0x34 => IfTestOp::IF_LT(data),
                        0x35 => IfTestOp::IF_GE(data),
                        0x36 => IfTestOp::IF_GT(data),
                        0x37 => IfTestOp::IF_LE(data),
                        _ => panic!("Unreachable")
                    }
                )
            }
            0x38..=0x3d => {
                let data = IfTestZOpData {
                    register_a: reader.read_u8().expect("Failed to read register_a"),
                    offset: reader.read_i16::<LittleEndian>().expect("Failed to read offset"),
                };
                Instruction::IfTestZ(
                    match opcode {
                        0x38 => IfTestZOp::IF_EQZ(data),
                        0x39 => IfTestZOp::IF_NEZ(data),
                        0x3a => IfTestZOp::IF_LTZ(data),
                        0x3b => IfTestZOp::IF_GEZ(data),
                        0x3c => IfTestZOp::IF_GTZ(data),
                        0x3d => IfTestZOp::IF_LEZ(data),
                        _ => panic!("Unreachables")
                    }
                )
            }
            0x3e..=0x43 => panic!("Unused opcode encountered: {:02x?}", opcode),
            0x44..=0x51 => {
                let data = ArrayOpData {
                    register_or_pair: reader.read_u8().expect("Unable to read array instruction register"),
                    array_register: reader.read_u8().expect("Unable to read array register"),
                    index_register: reader.read_u8().expect("Unable to read array index register"),
                };

                Instruction::Array(
                    match opcode {
                        0x44 => ArrayOp::AGET(data),
                        0x45 => ArrayOp::AGET_WIDE(data),
                        0x46 => ArrayOp::AGET_OBJECT(data),
                        0x47 => ArrayOp::AGET_BOOLEAN(data),
                        0x48 => ArrayOp::AGET_BYTE(data),
                        0x49 => ArrayOp::AGET_CHAR(data),
                        0x4a => ArrayOp::AGET_SHORT(data),
                        0x4b => ArrayOp::APUT(data),
                        0x4c => ArrayOp::APUT_WIDE(data),
                        0x4d => ArrayOp::APUT_OBJECT(data),
                        0x4e => ArrayOp::APUT_BOOLEAN(data),
                        0x4f => ArrayOp::APUT_BYTE(data),
                        0x50 => ArrayOp::APUT_CHAR(data),
                        0x51 => ArrayOp::APUT_SHORT(data),
                        _ => panic!("Unsupported opcode: {}", opcode)
                    }
                )
            }
            0x52..=0x5f => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                let data = InstanceFieldOpData {
                    register_or_pair: reader.read::<u8>(4).expect("Failed to read register_or_pair"),
                    object_register: reader.read::<u8>(4).expect("Failed to read object_register"),
                    field: data.fields[
                        reader
                            .read::<u16>(16)
                            .expect("Failed to read field_id") as usize
                        ].clone(),
                };

                Instruction::InstanceField(
                    match opcode {
                        0x52 => InstanceFieldOp::IGET(data),
                        0x53 => InstanceFieldOp::IGET_WIDE(data),
                        0x54 => InstanceFieldOp::IGET_OBJECT(data),
                        0x55 => InstanceFieldOp::IGET_BOOLEAN(data),
                        0x56 => InstanceFieldOp::IGET_BYTE(data),
                        0x57 => InstanceFieldOp::IGET_CHAR(data),
                        0x58 => InstanceFieldOp::IGET_SHORT(data),
                        0x59 => InstanceFieldOp::IPUT(data),
                        0x5a => InstanceFieldOp::IPUT_WIDE(data),
                        0x5b => InstanceFieldOp::IPUT_OBJECT(data),
                        0x5c => InstanceFieldOp::IPUT_BOOLEAN(data),
                        0x5d => InstanceFieldOp::IPUT_BYTE(data),
                        0x5e => InstanceFieldOp::IPUT_CHAR(data),
                        0x5f => InstanceFieldOp::IPUT_SHORT(data),
                        _ => panic!("Unsupported opcode: 0x{:02x?}", opcode),
                    }
                )
            }
            0x60..=0x6d => {
                let register = reader.read_u8().expect("Failed to read register");
                let field_id = reader
                    .read_u16::<LittleEndian>()
                    .expect("Failed to read field_id") as usize;
                if field_id >= data.fields.len() {
                    panic!("Invalid field_id: {}", field_id);
                }
                let data = StaticFieldOpData {
                    register,
                    field: data.fields.get(field_id).expect(&format!("Unable to get field with index {} for instruction with opcode {:2x?}", field_id, opcode)).clone(),
                };
                Instruction::Static(
                    match opcode {
                        0x60 => StaticFieldOp::SGET(data),
                        0x61 => StaticFieldOp::SGET_WIDE(data),
                        0x62 => StaticFieldOp::SGET_OBJECT(data),
                        0x63 => StaticFieldOp::SGET_BOOLEAN(data),
                        0x64 => StaticFieldOp::SGET_BYTE(data),
                        0x65 => StaticFieldOp::SGET_CHAR(data),
                        0x66 => StaticFieldOp::SGET_SHORT(data),
                        0x67 => StaticFieldOp::SPUT(data),
                        0x68 => StaticFieldOp::SPUT_WIDE(data),
                        0x69 => StaticFieldOp::SPUT_OBJECT(data),
                        0x6a => StaticFieldOp::SPUT_BOOLEAN(data),
                        0x6b => StaticFieldOp::SPUT_BYTE(data),
                        0x6c => StaticFieldOp::SPUT_CHAR(data),
                        0x6d => StaticFieldOp::SPUT_SHORT(data),
                        _ => panic!("Unsupported opcode: 0x{:02x?}", opcode)
                    }
                )
            }
            0x6e..=0x72 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                let format = Format35c::parse(&mut reader);
                let data = InvokeOpData {
                    method: data.methods[format.bbbb as usize].clone(),
                    args_registers: format.reg_list,
                };

                // print!("Parsing method {}.   ", data.method.full_name_human_readable());

                Instruction::Invoke(
                    data,
                    match opcode {
                        0x6e => InvokeOp::INVOKE_VIRTUAL,
                        0x6f => InvokeOp::INVOKE_SUPER,
                        0x70 => InvokeOp::INVOKE_DIRECT,
                        0x71 => InvokeOp::INVOKE_STATIC,
                        0x72 => InvokeOp::INVOKE_INTERFACE,
                        _ => panic!("Unreachable")
                    },
                )
            }
            0x73 => panic!("Unused opcode encountered: {:02x?}", opcode),
            0x74..=0x78 => {
                let format = Format3rc::parse(&mut reader);
                let data = InvokeRangeOpData {
                    method: data.methods[format.bbbb as usize].clone(),
                    start_register: format.cccc,
                    register_count: format.reg_count,
                };

                Instruction::InvokeRange(
                    match opcode {
                        0x74 => InvokeRangeOp::INVOKE_VIRTUAL(data),
                        0x75 => InvokeRangeOp::INVOKE_SUPER(data),
                        0x76 => InvokeRangeOp::INVOKE_DIRECT(data),
                        0x77 => InvokeRangeOp::INVOKE_STATIC(data),
                        0x78 => InvokeRangeOp::INVOKE_INTERFACE(data),
                        _ => panic!("Unreachable")
                    }
                )
            }
            0x79..=0x7a => panic!("Unused opcode encountered: {:02x?}", opcode),
            0x7b..=0x8f => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                let data = UnaryOpData {
                    dest: reader.read::<u8>(4).expect("Failed to read dest"),
                    src: reader.read::<u8>(4).expect("Failed to read src"),
                };

                Instruction::Unary(
                    match opcode {
                        0x7b => UnaryOp::NEG_INT(data),
                        0x7c => UnaryOp::NOT_INT(data),
                        0x7d => UnaryOp::NEG_LONG(data),
                        0x7e => UnaryOp::NOT_LONG(data),
                        0x7f => UnaryOp::NEG_FLOAT(data),
                        0x80 => UnaryOp::NEG_DOUBLE(data),
                        0x81 => UnaryOp::INT_TO_LONG(data),
                        0x82 => UnaryOp::INT_TO_FLOAT(data),
                        0x83 => UnaryOp::INT_TO_DOUBLE(data),
                        0x84 => UnaryOp::LONG_TO_INT(data),
                        0x85 => UnaryOp::LONG_TO_FLOAT(data),
                        0x86 => UnaryOp::LONG_TO_DOUBLE(data),
                        0x87 => UnaryOp::FLOAT_TO_INT(data),
                        0x88 => UnaryOp::FLOAT_TO_LONG(data),
                        0x89 => UnaryOp::FLOAT_TO_DOUBLE(data),
                        0x8a => UnaryOp::DOUBLE_TO_INT(data),
                        0x8b => UnaryOp::DOUBLE_TO_LONG(data),
                        0x8c => UnaryOp::DOUBLE_TO_FLOAT(data),
                        0x8d => UnaryOp::INT_TO_BYTE(data),
                        0x8e => UnaryOp::INT_TO_CHAR(data),
                        0x8f => UnaryOp::INT_TO_SHORT(data),
                        _ => panic!("Unreachable")
                    }
                )
            }
            0x90..=0xaf => {
                let data = BinaryOpData {
                    dest: reader.read_u8().expect("Failed to read dest"),
                    src_a: reader.read_u8().expect("Failed to read src_a"),
                    src_b: reader.read_u8().expect("Failed to read src_b"),
                };

                Instruction::Binary(
                    match opcode {
                        0x90 => BinaryOp::ADD_INT(data),
                        0x91 => BinaryOp::SUB_INT(data),
                        0x92 => BinaryOp::MUL_INT(data),
                        0x93 => BinaryOp::DIV_INT(data),
                        0x94 => BinaryOp::REM_INT(data),
                        0x95 => BinaryOp::AND_INT(data),
                        0x96 => BinaryOp::OR_INT(data),
                        0x97 => BinaryOp::XOR_INT(data),
                        0x98 => BinaryOp::SHL_INT(data),
                        0x99 => BinaryOp::SHR_INT(data),
                        0x9a => BinaryOp::USHR_INT(data),
                        0x9b => BinaryOp::ADD_LONG(data),
                        0x9c => BinaryOp::SUB_LONG(data),
                        0x9d => BinaryOp::MUL_LONG(data),
                        0x9e => BinaryOp::DIV_LONG(data),
                        0x9f => BinaryOp::REM_LONG(data),
                        0xa0 => BinaryOp::AND_LONG(data),
                        0xa1 => BinaryOp::OR_LONG(data),
                        0xa2 => BinaryOp::XOR_LONG(data),
                        0xa3 => BinaryOp::SHL_LONG(data),
                        0xa4 => BinaryOp::SHR_LONG(data),
                        0xa5 => BinaryOp::USHR_LONG(data),
                        0xa6 => BinaryOp::ADD_FLOAT(data),
                        0xa7 => BinaryOp::SUB_FLOAT(data),
                        0xa8 => BinaryOp::MUL_FLOAT(data),
                        0xa9 => BinaryOp::DIV_FLOAT(data),
                        0xaa => BinaryOp::REM_FLOAT(data),
                        0xab => BinaryOp::ADD_DOUBLE(data),
                        0xac => BinaryOp::SUB_DOUBLE(data),
                        0xad => BinaryOp::MUL_DOUBLE(data),
                        0xae => BinaryOp::DIV_DOUBLE(data),
                        0xaf => BinaryOp::REM_DOUBLE(data),
                        _ => panic!("Unreachable")
                    }
                )
            }
            0xb0..=0xcf => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                let data = BinaryOp2AddrData {
                    dest: reader.read::<u8>(4).expect("Failed to read dest"),
                    src: reader.read::<u8>(4).expect("Failed to read src"),
                };

                Instruction::Binary2Addr(
                    match opcode {
                        0xb0 => BinaryOp2Addr::ADD_INT_2ADDR(data),
                        0xb1 => BinaryOp2Addr::SUB_INT_2ADDR(data),
                        0xb2 => BinaryOp2Addr::MUL_INT_2ADDR(data),
                        0xb3 => BinaryOp2Addr::DIV_INT_2ADDR(data),
                        0xb4 => BinaryOp2Addr::REM_INT_2ADDR(data),
                        0xb5 => BinaryOp2Addr::AND_INT_2ADDR(data),
                        0xb6 => BinaryOp2Addr::OR_INT_2ADDR(data),
                        0xb7 => BinaryOp2Addr::XOR_INT_2ADDR(data),
                        0xb8 => BinaryOp2Addr::SHL_INT_2ADDR(data),
                        0xb9 => BinaryOp2Addr::SHR_INT_2ADDR(data),
                        0xba => BinaryOp2Addr::USHR_INT_2ADDR(data),
                        0xbb => BinaryOp2Addr::ADD_LONG_2ADDR(data),
                        0xbc => BinaryOp2Addr::SUB_LONG_2ADDR(data),
                        0xbd => BinaryOp2Addr::MUL_LONG_2ADDR(data),
                        0xbe => BinaryOp2Addr::DIV_LONG_2ADDR(data),
                        0xbf => BinaryOp2Addr::REM_LONG_2ADDR(data),
                        0xc0 => BinaryOp2Addr::AND_LONG_2ADDR(data),
                        0xc1 => BinaryOp2Addr::OR_LONG_2ADDR(data),
                        0xc2 => BinaryOp2Addr::XOR_LONG_2ADDR(data),
                        0xc3 => BinaryOp2Addr::SHL_LONG_2ADDR(data),
                        0xc4 => BinaryOp2Addr::SHR_LONG_2ADDR(data),
                        0xc5 => BinaryOp2Addr::USHR_LONG_2ADDR(data),
                        0xc6 => BinaryOp2Addr::ADD_FLOAT_2ADDR(data),
                        0xc7 => BinaryOp2Addr::SUB_FLOAT_2ADDR(data),
                        0xc8 => BinaryOp2Addr::MUL_FLOAT_2ADDR(data),
                        0xc9 => BinaryOp2Addr::DIV_FLOAT_2ADDR(data),
                        0xca => BinaryOp2Addr::REM_FLOAT_2ADDR(data),
                        0xcb => BinaryOp2Addr::ADD_DOUBLE_2ADDR(data),
                        0xcc => BinaryOp2Addr::SUB_DOUBLE_2ADDR(data),
                        0xcd => BinaryOp2Addr::MUL_DOUBLE_2ADDR(data),
                        0xce => BinaryOp2Addr::DIV_DOUBLE_2ADDR(data),
                        0xcf => BinaryOp2Addr::REM_DOUBLE_2ADDR(data),
                        _ => panic!("Unreachable")
                    }
                )
            }
            0xd0..=0xd7 => {
                let mut reader = BitReader::endian(&mut reader, bitstream_io::LittleEndian);
                let data = BinaryOpLit16Data {
                    dest: reader.read::<u8>(4).expect("Failed to read dest"),
                    src: reader.read::<u8>(4).expect("Failed to read src"),
                    literal: reader.read::<i16>(16).expect("Failed to read literal"),
                };

                Instruction::BinaryLit16(
                    match opcode {
                        0xd0 => BinaryOpLit16::ADD_INT_LIT16(data),
                        0xd1 => BinaryOpLit16::RSUB_INT(data),
                        0xd2 => BinaryOpLit16::MUL_INT_LIT16(data),
                        0xd3 => BinaryOpLit16::DIV_INT_LIT16(data),
                        0xd4 => BinaryOpLit16::REM_INT_LIT16(data),
                        0xd5 => BinaryOpLit16::AND_INT_LIT16(data),
                        0xd6 => BinaryOpLit16::OR_INT_LIT16(data),
                        0xd7 => BinaryOpLit16::XOR_INT_LIT16(data),
                        _ => panic!("Unreachable")
                    }
                )
            }
            0xd8..=0xe2 => {
                let data = BinaryOpLit8Data {
                    dest: reader.read_u8().expect("Failed to read dest"),
                    src: reader.read_u8().expect("Failed to read src"),
                    literal: reader.read_u8().expect("Failed to read literal"),
                };
                Instruction::BinaryLit8(
                    match opcode {
                        0xd8 => BinaryOpLit8::ADD_INT_LIT8(data),
                        0xd9 => BinaryOpLit8::RSUB_INT_LIT8(data),
                        0xda => BinaryOpLit8::MUL_INT_LIT8(data),
                        0xdb => BinaryOpLit8::DIV_INT_LIT8(data),
                        0xdc => BinaryOpLit8::REM_INT_LIT8(data),
                        0xdd => BinaryOpLit8::AND_INT_LIT8(data),
                        0xde => BinaryOpLit8::OR_INT_LIT8(data),
                        0xdf => BinaryOpLit8::XOR_INT_LIT8(data),
                        0xe0 => BinaryOpLit8::SHL_INT_LIT8(data),
                        0xe1 => BinaryOpLit8::SHR_INT_LIT8(data),
                        0xe2 => BinaryOpLit8::USHR_INT_LIT8(data),
                        _ => panic!("Unreachable! opcode: {:02x?}", opcode)
                    }
                )
            }
            0xe3..=0xf9 => panic!("Unused opcode! {:02x?}", opcode),
            // 0xfa => ,
            // 0xfb => ,
            // 0xfc => ,
            // 0xfd => ,
            // 0xfe => ,
            0xff => Instruction::CONST_METHOD_TYPE {
                dest: reader.read_u8().expect("Failed to read dest"),
                method_type: data.prototypes[
                    reader
                        .read_u16::<LittleEndian>()
                        .expect("Failed to read proto_id") as usize
                    ].clone(),
            },
            _ => panic!("Unsupported opcode: 0x{:02x?}", opcode),
        };

        // println!("Parsed instruction: {:?}", instruction);

        result.push(instruction)
    }

    result
}

fn parse_packed_switch_table<R: Read + Seek>(reader: &mut R, packed_switch_tables: &mut HashMap<u64, Arc<PackedSwitchTable>>, offset: u64) -> Arc<PackedSwitchTable> {
    if let Some(table) = packed_switch_tables.get(&offset) {
        return table.clone();
    }

    reader.seek(SeekFrom::Start(offset))
        .expect(format!("Failed to seek to packed_switch_table offset {}", offset).as_str());

    let ident = reader.read_u16::<LittleEndian>().expect("Failed to read ident");
    if ident != 0x0100 {
        panic!("Invalid ident: {:02x?}", ident);
    }
    let size = reader.read_u16::<LittleEndian>().expect("Failed to read size");
    let first_key = reader.read_i32::<LittleEndian>().expect("Failed to read first_key");
    let targets = reader.parse_list(size as u64, |file| {
        file.read_i32::<LittleEndian>().expect("Failed to read target")
    });

    let table = Arc::new(
        PackedSwitchTable {
            size_in_code_units: (size as u32 * 2) + 4,
            first_key,
            targets,
        }
    );
    packed_switch_tables.insert(offset, table.clone());
    table
}

fn parse_sparse_switch_table<R: Read + Seek>(reader: &mut R, sparse_switch_tables: &mut HashMap<u64, Arc<SparseSwitchTable>>, offset: u64) -> Arc<SparseSwitchTable> {
    if let Some(table) = sparse_switch_tables.get(&offset) {
        return table.clone();
    }

    reader.seek(SeekFrom::Start(offset))
        .expect(format!("Failed to seek to sparse_switch_table offset {}", offset).as_str());

    let ident = reader.read_u16::<LittleEndian>().expect("Failed to read ident");
    if ident != 0x0200 {
        panic!("Invalid ident: {:02x?}", ident);
    }
    let size = reader.read_u16::<LittleEndian>().expect("Failed to read size");
    let keys = reader.parse_list(size as u64, |file| {
        file.read_i32::<LittleEndian>().expect("Failed to read key")
    });
    let targets = reader.parse_list(size as u64, |file| {
        file.read_i32::<LittleEndian>().expect("Failed to read target")
    });

    let table = Arc::new(
        SparseSwitchTable {
            size_in_code_units: (size as u32 * 4) + 2,
            keys,
            targets,
        }
    );
    sparse_switch_tables.insert(offset, table.clone());
    table
}

fn parse_fill_array_data<R: Read + Seek>(
    reader: &mut R,
    fill_array_data: &mut HashMap<u64, Arc<FillArrayData>>,
    offset: u64,
) -> Arc<FillArrayData> {
    if let Some(data) = fill_array_data.get(&offset) {
        return data.clone();
    }

    reader.seek(SeekFrom::Start(offset))
        .expect(format!("Failed to seek to fill_array_data offset {}", offset).as_str());

    let ident = reader.read_u16::<LittleEndian>().expect("Failed to read ident");
    if ident != 0x0300 {
        panic!("Invalid ident: {:02x?}", ident);
    }
    let element_width = reader.read_u16::<LittleEndian>().expect("Failed to read element_width");
    let size = reader.read_u32::<LittleEndian>().expect("Failed to read size");
    let data = reader.parse_list(size as u64, |file| {
        file.read_u8().expect("Failed to read data")
    });

    let data = Arc::new(FillArrayData {
        size_in_code_units: (size * element_width as u32 + 1) / 2 + 4,
        element_width,
        size,
        data,
    });
    fill_array_data.insert(offset, data.clone());
    data
}

fn parse_tries(file: &mut File, tries_size: u16) -> Vec<TryItem> {
    if tries_size == 0 {
        return vec![];
    }

    file.parse_list(tries_size as u64, |file| {
        TryItem {
            start_addr: file.read_u32::<LittleEndian>().expect("Failed to read start_addr"),
            insn_count: file.read_u16::<LittleEndian>().expect("Failed to read insn_count"),
            handler_offset: file.read_u16::<LittleEndian>().expect("Failed to read handler_offset"),
        }
    })
}

fn parse_handlers(file: &mut File, data: &DexFileData) -> Vec<EncodedCatchHandler> {
    let size = file.read_uleb128().expect("Failed to read handlers size");
    file.parse_list(size, |file| {
        parse_encoded_catch_handler(file, data)
    })
}

fn parse_encoded_catch_handler(file: &mut File, data: &DexFileData) -> EncodedCatchHandler {
    let size = file.read_sleb128().expect("Failed to read handler size");
    let handlers = (0..size.abs())
        .map(|_| {
            let type_idx = file.read_uleb128().expect("Failed to read type_idx");
            let addr = file.read_uleb128().expect("Failed to read addr");
            EncodedTypeAddrPair {
                type_: Arc::new(String::from("")),
                addr,
            }
        })
        .collect();
    let catch_all_addr = if size < 0 {
        Some(file.read_uleb128().expect("Failed to read catch_all_addr"))
    } else {
        None
    };
    EncodedCatchHandler {
        handlers,
        catch_all_addr,
    }
}

fn parse_raw_insns(file: &mut File, insns_count: u32) -> Vec<u8> {
    let mut buf = vec![0u8; (insns_count * 2) as usize];
    file.read_exact(buf.as_mut_slice()).expect("Failed to read instructions");
    buf
}

fn parse_debug_info(file: &mut File, data: &DexFileData, offset: u32) -> Option<DebugInfo> {
    if offset == 0 {
        return None;
    }

    file.seek(SeekFrom::Start(offset as u64))
        .expect(format!("Failed to seek to debug_info offset {}", offset).as_str());

    let line_start = file.read_uleb128().expect("Failed to read line_start");
    let parameter_names = parse_parameter_names(file, data);
    let bytecode = parse_debug_item_bytecodes(file);

    Some(DebugInfo {
        line_start,
        parameter_names,
        bytecode,
    })
}

fn parse_parameter_names(file: &mut File, data: &DexFileData) -> Vec<Option<Arc<String>>> {
    let size = file.read_uleb128().expect("Failed to read parameter_names size");
    (0..size)
        .map(|_| {
            file.read_uleb128p1()
                .expect("Failed to read parameter name index")
                .map(|idx| data.string_data[idx as usize].clone())
        })
        .collect()
}

fn parse_debug_item_bytecodes(file: &mut File) -> Vec<DebugItemBytecodes> {
    let mut bytecodes = vec![];
    loop {
        let opcode = file.read_u8().expect("Failed to read debug item opcode");
        match opcode {
            0x00 => bytecodes.push(DebugItemBytecodes::DBG_END_SEQUENCE),
            0x01 => bytecodes.push(DebugItemBytecodes::DBG_ADVANCE_PC),
            0x02 => bytecodes.push(DebugItemBytecodes::DBG_ADVANCE_LINE),
            0x03 => bytecodes.push(DebugItemBytecodes::DBG_START_LOCAL),
            0x04 => bytecodes.push(DebugItemBytecodes::DBG_START_LOCAL_EXTENDED),
            0x05 => bytecodes.push(DebugItemBytecodes::DBG_END_LOCAL),
            0x06 => bytecodes.push(DebugItemBytecodes::DBG_RESTART_LOCAL),
            0x07 => bytecodes.push(DebugItemBytecodes::DBG_SET_PROLOGUE_END),
            0x08 => bytecodes.push(DebugItemBytecodes::DBG_SET_EPILOGUE_BEGIN),
            0x09 => bytecodes.push(DebugItemBytecodes::DBG_SET_FILE),
            0x0a..=0xff => bytecodes.push(DebugItemBytecodes::SPECIAL_OPCODE(opcode)),
            _ => panic!("Invalid debug item opcode: {}", opcode),
        }
        if opcode == 0x00 {
            break;
        }
    }
    bytecodes
}

fn parse_annotations(file: &mut File, data: &DexFileData, offset: u32) -> Option<Annotations> {
    if offset == 0 {
        return None;
    }
    file.seek(SeekFrom::Start(offset as u64))
        .expect(format!("Failed to seek to annotations_off position {}", offset).as_str());

    let class_annotations_offset = file.read_u32::<LittleEndian>().expect("Failed to read class_annotations_offset");
    let field_annotation_count = file.read_u32::<LittleEndian>().expect("Failed to read field_annotation_count");
    let method_annotation_count = file.read_u32::<LittleEndian>().expect("Failed to read method_annotation_count");
    let parameter_annotation_count = file.read_u32::<LittleEndian>().expect("Failed to read parameter_annotation_count");

    let raw_field_annotations = file.parse_list(field_annotation_count as u64, |file| {
        let field_idx = file.read_u32::<LittleEndian>().expect("Failed to read field_idx");
        let annotations_offset = file.read_u32::<LittleEndian>().expect("Failed to read field_annotation_offset");

        RawFieldAnnotation {
            field_idx,
            annotations_offset,
        }
    });
    let raw_method_annotations = file.parse_list(method_annotation_count as u64, |file| {
        let method_idx = file.read_u32::<LittleEndian>().expect("Failed to read method_idx");
        let annotations_offset = file.read_u32::<LittleEndian>().expect("Failed to read method_annotation_offset");

        RawMethodAnnotation {
            method_idx,
            annotations_offset,
        }
    });
    let raw_parameter_annotations = file.parse_list(parameter_annotation_count as u64, |file| {
        let method_idx = file.read_u32::<LittleEndian>().expect("Failed to read method_idx");
        let annotations_offset = file.read_u32::<LittleEndian>().expect("Failed to read parameter_annotation_offset");

        RawMethodAnnotation {
            method_idx,
            annotations_offset,
        }
    });
    let field_annotations = raw_field_annotations.iter().map(|field_annotation| {
        FieldAnnotation {
            field: data.fields[field_annotation.field_idx as usize].clone(),
            annotations: parse_annotation_set(file, data, field_annotation.annotations_offset),
        }
    }).collect();
    let method_annotations = raw_method_annotations.iter().map(|method_annotation| {
        MethodAnnotation {
            method: data.methods[method_annotation.method_idx as usize].clone(),
            annotations: parse_annotation_set(file, data, method_annotation.annotations_offset),
        }
    }).collect();
    let parameter_annotations = raw_parameter_annotations.iter().map(|parameter_annotation| {
        let ref_list = parse_raw_annotation_set_ref_list(file, data, parameter_annotation.annotations_offset);
        ParameterAnnotation {
            method: data.methods[parameter_annotation.method_idx as usize].clone(),
            annotations: ref_list.annotation_set_offsets.into_iter().map(|annotation_set_offset| {
                match annotation_set_offset {
                    None => vec![],
                    Some(offset) => parse_annotation_set(file, data, offset)
                }
            }).collect(),
        }
    }).collect();

    let raw_class_annotation_set = parse_raw_annotation_set(file, class_annotations_offset);

    Some(
        Annotations {
            class_annotations: raw_class_annotation_set.annotation_offsets.into_iter().map(|annotation_offset| {
                let item = parse_annotation_item(file, data, annotation_offset);
                ClassAnnotation {
                    visibility: item.visibility,
                    type_: item.type_.clone(),
                    elements: item.elements,
                }
            }).collect(),
            field_annotations,
            method_annotations,
            parameter_annotations,
        }
    )
}

fn parse_raw_annotation_set(file: &mut File, offset: u32) -> RawAnnotationSet {
    if offset == 0 {
        return RawAnnotationSet {
            annotation_offsets: vec![],
        };
    }
    file.seek(SeekFrom::Start(offset as u64))
        .expect(format!("Failed to seek to annotations_off position {}", offset).as_str());

    let size = file.read_u32::<LittleEndian>().expect("Failed to read size");
    let annotation_offsets = file.parse_list(size as u64, |file| {
        file.read_u32::<LittleEndian>().expect("Failed to read annotation offset")
    });

    RawAnnotationSet {
        annotation_offsets,
    }
}

fn parse_raw_annotation_set_ref_list(file: &mut File, data: &DexFileData, offset: u32) -> RawAnnotationSetRefList {
    file.seek(SeekFrom::Start(offset as u64))
        .expect(format!("Failed to seek to annotations_off position {}", offset).as_str());

    let size = file.read_u32::<LittleEndian>().expect("Failed to read size");
    let annotation_set_offsets = file.parse_list(size as u64, |file| {
        let annotation_set_offset = file.read_u32::<LittleEndian>().expect("Failed to read annotation offset");

        if annotation_set_offset != 0 {
            Some(annotation_set_offset)
        } else {
            None
        }
    });

    RawAnnotationSetRefList {
        annotation_set_offsets,
    }
}

fn parse_annotation_set(file: &mut File, data: &DexFileData, offset: u32) -> Vec<AnnotationItem> {
    if offset == 0 {
        return vec![];
    }
    file.seek(SeekFrom::Start(offset as u64))
        .expect(format!("Failed to seek to annotations_off position {}", offset).as_str());

    let size = file.read_u32::<LittleEndian>().expect("Failed to read size");
    let entries = file.parse_list(size as u64, |file| {
        file.read_u32::<LittleEndian>().expect("Failed to read annotation offset")
    });

    entries
        .into_iter()
        .map(|entry| {
            parse_annotation_item(file, data, entry)
        })
        .collect()
}

fn parse_annotation_item(file: &mut File, data: &DexFileData, offset: u32) -> AnnotationItem {
    file.seek(SeekFrom::Start(offset as u64))
        .expect(format!("Failed to seek to annotation offset: {}", offset).as_str());

    let visibility = file.read_u8().expect("Failed to read visibility");
    let type_idx = file.read_uleb128().expect("Failed to read type_idx");
    let size = file.read_uleb128().expect("Failed to read size");
    let elements = parse_annotation_elements(file, data, size);

    AnnotationItem {
        visibility: visibility.try_into().expect("Failed to parse visibility"),
        type_: data.type_identifiers[type_idx as usize].clone(),
        elements,
    }
}

fn parse_encoded_value(file: &mut File, data: &DexFileData) -> EncodedValue {
    let header = &file.read_u8().expect("Failed to read value type and arg");
    let value_arg = (header >> 5) as usize;
    let value_type = 0b0001_1111 & header;
    let mut bit_reader = BitReader::endian(file, bitstream_io::LittleEndian);
    let size_bytes = (value_arg + 1) as u32;
    let size_bits = (size_bytes) * 8;
    match value_type {
        0x00 => {
            debug_assert!(value_arg == 0);
            EncodedValue::Byte(bit_reader.read::<u8>(8).expect("Failed to read byte"))
        }
        0x02 => {
            debug_assert!(value_arg < 2);
            EncodedValue::Short(bit_reader.read(size_bits).expect("Failed to read short"))
        }
        0x03 => {
            debug_assert!(value_arg < 2);
            EncodedValue::Char(bit_reader.read(size_bits).expect("Failed to read char"))
        }
        0x04 => {
            debug_assert!(value_arg < 4);
            EncodedValue::Int(bit_reader.read(size_bits).expect("Failed to read int"))
        }
        0x06 => {
            debug_assert!(value_arg < 8);
            EncodedValue::Long(bit_reader.read(size_bits).expect("Failed to read long"))
        }
        0x10 => {
            debug_assert!(value_arg < 4);
            let buf: Vec<u8> =
                read_value(bit_reader.into_reader(), size_bytes as usize, true)
                    .expect("Failed to read double");

            let mut extended_buf = [0u8; 4];
            extended_buf[..buf.len()].copy_from_slice(&buf);
            let mut cursor = Cursor::new(extended_buf);

            EncodedValue::Float(cursor.read_f32::<LittleEndian>().expect("Failed to read float"))
        }
        0x11 => {
            debug_assert!(value_arg < 8);
            let buf: Vec<u8> =
                read_value(bit_reader.into_reader(), size_bytes as usize, true)
                    .expect("Failed to read double");

            let mut extended_buf = [0u8; 8];
            extended_buf[..buf.len()].copy_from_slice(&buf);
            let mut cursor = Cursor::new(extended_buf);

            EncodedValue::Double(cursor.read_f64::<LittleEndian>().expect("Failed to read double"))
        }
        0x15 => {
            debug_assert!(value_arg < 4);
            EncodedValue::MethodType(data.prototypes[bit_reader.read::<u32>(size_bits).expect("Failed to read method_type_idx") as usize].clone())
        }
        0x16 => {
            debug_assert!(value_arg < 4);
            EncodedValue::MethodHandle(data.methods[bit_reader.read::<u32>(size_bits).expect("Failed to read method_handle_idx") as usize].clone())
        }
        0x17 => {
            debug_assert!(value_arg < 4);
            EncodedValue::String(data.string_data[bit_reader.read::<u32>(size_bits).expect("Failed to read string_idx") as usize].clone())
        }
        0x18 => {
            debug_assert!(value_arg < 4);
            EncodedValue::Type(data.type_identifiers[bit_reader.read::<u32>(size_bits).expect("Failed to read type_idx") as usize].clone())
        }
        0x19 => {
            debug_assert!(value_arg < 4);
            EncodedValue::Field(data.fields[bit_reader.read::<u32>(size_bits).expect("Failed to read field_idx") as usize].clone())
        }
        0x1a => {
            debug_assert!(value_arg < 4);
            EncodedValue::Method(data.methods[bit_reader.read::<u32>(size_bits).expect("Failed to read method_idx") as usize].clone())
        }
        0x1b => {
            debug_assert!(value_arg < 4);
            EncodedValue::Enum(data.fields[bit_reader.read::<u32>(size_bits).expect("Failed to read field_idx") as usize].clone())
        }
        0x1c => {
            debug_assert!(value_arg == 0);
            EncodedValue::Array(parse_encoded_array(bit_reader.into_reader(), data))
        }
        0x1d => {
            debug_assert!(value_arg == 0);
            EncodedValue::Annotation(parse_encoded_annotation(bit_reader.into_reader(), data))
        }
        0x1e => {
            debug_assert!(value_arg == 0);
            EncodedValue::Null
        }
        0x1f => {
            debug_assert!(value_arg < 2);
            EncodedValue::Boolean(value_arg != 0)
        }
        _ => {
            panic!("Invalid encoded value type {} at {}", value_type, bit_reader.into_reader().stream_position().expect("Unable to read stream position"))
        }
    }
}

fn read_value<R: Read>(reader: &mut R, size: usize, sign_extend: bool) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; size];
    reader.read_exact(&mut buf[..size])?;
    if sign_extend && buf[size - 1] & 0x80 != 0 {
        for byte in &mut buf[size..] {
            *byte = 0xFF;
        }
    }
    Ok(buf)
}

fn parse_encoded_array(file: &mut File, data: &DexFileData) -> Vec<EncodedValue> {
    let size = file.read_uleb128().expect("Failed to read size");
    file.parse_list(size, |file| {
        parse_encoded_value(file, data)
    })
}

fn parse_encoded_annotation(file: &mut File, data: &DexFileData) -> EncodedAnnotationItem {
    let type_idx = file.read_uleb128().expect("Failed to read type_idx");
    let size = file.read_uleb128().expect("Failed to read size");
    let elements = parse_annotation_elements(file, data, size);

    EncodedAnnotationItem {
        type_: data.type_identifiers[type_idx as usize].clone(),
        values: elements,
    }
}

fn parse_annotation_elements(file: &mut File, data: &DexFileData, size: u64) -> Vec<AnnotationElement> {
    file.parse_list(size, |file| {
        let name_idx = file.read_uleb128().expect("Failed to read name_idx");
        let value = parse_encoded_value(file, data);
        AnnotationElement {
            name: data.string_data[name_idx as usize].clone(),
            value,
        }
    })
}


fn parse_type_list(file: &mut File, type_identifiers: &Vec<Arc<String>>, offset: u32) -> Vec<Arc<String>> {
    if offset == 0 {
        return vec![];
    }

    file.seek(SeekFrom::Start(offset as u64))
        .expect(format!("Failed to seek to position {}", offset).as_str());
    let size = file.read_u32::<LittleEndian>().expect("Failed to read type list size");
    (0..size)
        .map(|_| file.read_u16::<LittleEndian>().expect("Failed to read type index"))
        .map(|idx| {
            type_identifiers[idx as usize].clone()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::testing::interpreter::get_method_frame;

    #[test]
    fn test_instructions_parser() {
        let instructions = get_method_frame(indoc! {"
            .method public static main([Ljava/lang/String;)V

                .registers 4

                .line 151
                const-string v0, \"Null output stream\"

                invoke-static {p1, v0}, Ljava/io/PrintStream;->requireNonNull(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;

                move-result-object p1

                check-cast p1, Ljava/io/OutputStream;

                invoke-direct {p0, p2, p1}, Ljava/io/PrintStream;-><init>(ZLjava/io/OutputStream;)V

                .line 152
                return-void

            .end method
        "});
    }
}