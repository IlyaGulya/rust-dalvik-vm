use std::fs::File;
use std::io;
use std::io::{Seek, SeekFrom};
use std::rc::Rc;

use bitmask_enum::bitmask;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::endian_aware_reader::{Endianness, Leb128Ext, MUtf8Ext};
use crate::raw_dex_file::{RawAnnotationSet, RawAnnotationSetRefList, RawClassDataItem, RawClassDef, RawDexFile, RawEncodedField, RawEncodedMethod, RawFieldAnnotation, RawMethodAnnotation};

#[derive(Debug, PartialEq)]
pub struct DexFile {
    pub header: Header,
    pub data: DexFileData,
    pub classes: Vec<ClassDefinition>,
}

#[derive(Debug, PartialEq)]
pub struct DexFileData {
    pub string_data: Vec<Rc<String>>,
    pub type_identifiers: Vec<Rc<String>>,
    pub prototypes: Vec<Rc<Prototype>>,
    pub fields: Vec<Rc<Field>>,
    pub methods: Vec<Rc<Method>>,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub version: u32,
    pub checksum: u32,
    pub signature: [u8; 20],
    pub file_size: u32,
    pub endianness: Endianness,
}

#[derive(Debug, PartialEq)]
pub struct Prototype {
    pub shorty: Rc<String>,
    pub return_type: Rc<String>,
    pub parameters: Vec<Rc<String>>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub definer: Rc<String>,
    pub type_: Rc<String>,
    pub name: Rc<String>,
}

#[derive(Debug, PartialEq)]
pub struct Method {
    pub definer: Rc<String>,
    pub prototype: Rc<Prototype>,
    pub name: Rc<String>,
}

#[derive(Debug, PartialEq)]
pub struct ClassDefinition {
    pub class_type: Rc<String>,
    pub access_flags: AccessFlags,
    pub superclass: Option<Rc<String>>,
    pub interfaces: Vec<Rc<String>>,
    pub source_file_name: Option<Rc<String>>,
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
    pub type_: Rc<String>,
    pub elements: Vec<AnnotationElement>,
}

#[derive(Debug, PartialEq)]
pub struct MethodAnnotation {
    pub method: Rc<Method>,
    pub annotations: Vec<AnnotationItem>,
}

#[derive(Debug, PartialEq)]
pub struct ParameterAnnotation {
    pub method: Rc<Method>,
    pub annotations: Vec<Vec<AnnotationItem>>,
}

#[derive(Debug, PartialEq)]
pub struct FieldAnnotation {
    pub field: Rc<Field>,
    pub annotations: Vec<AnnotationItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnnotationItem {
    pub visibility: Visibility,
    pub type_: Rc<String>,
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
    pub static_fields: Vec<EncodedField>,
    pub instance_fields: Vec<EncodedField>,
    pub direct_methods: Vec<EncodedMethod>,
    pub virtual_methods: Vec<EncodedMethod>,
}

#[derive(Debug, PartialEq)]
pub struct EncodedField {
    pub field: Rc<Field>,
    pub access_flags: AccessFlags,
}

#[derive(Debug, PartialEq)]
pub struct EncodedMethod {
    pub method: Rc<Method>,
    pub access_flags: AccessFlags,
    pub code: Option<Code>,
}

// Docs: code_item
#[derive(Debug, PartialEq)]
pub struct Code {
    // number of registers used by this code
    pub registers_size: u16,
    // number of words of incoming arguments
    pub ins_size: u16,
    // number of words of outgoing argument space
    pub outs_size: u16,
    pub debug_info: Option<DebugInfo>,
    pub insns: Vec<u16>,
    pub tries: Vec<TryItem>,
    pub handlers: Vec<EncodedCatchHandler>,
}

// Docs: try_item
#[derive(Debug, PartialEq)]
pub struct TryItem {
    pub code_units: Vec<u16>,
    pub handler: EncodedCatchHandler,
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
    pub type_: Rc<String>,
    // bytecode address of associated exception handler
    pub addr: u64,
}

// Docs: debug_info_item
#[derive(Debug, PartialEq)]
pub struct DebugInfo {
    pub line_start: u64,
    pub parameter_names: Vec<Option<Rc<String>>>,
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
    MethodType(Rc<Prototype>),
    MethodHandle(Rc<Method>),
    String(Rc<String>),
    Type(Rc<String>),
    Field(Rc<Field>),
    Method(Rc<Method>),
    Enum(Rc<Field>),
    Array(Vec<EncodedValue>),
    Annotation(EncodedAnnotationItem),
    Null,
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EncodedAnnotationItem {
    pub type_: Rc<String>,
    pub values: Vec<AnnotationElement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnnotationElement {
    pub name: Rc<String>,
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


#[allow(non_camel_case_types)]
#[bitmask(u32)]
pub enum AccessFlags {
    // 0x1
    ACC_PUBLIC,

    // 0x2
    ACC_PRIVATE,

    // 0x4
    ACC_PROTECTED,

    // 0x8
    ACC_STATIC,

    // 0x10
    ACC_FINAL,

    // 0x20
    ACC_SYNCHRONIZED,

    // 0x40
    ACC_VOLATILE_OR_BRIDGE,

    // 0x80
    ACC_TRANSIENT_ORVARARGS,

    // 0x100
    ACC_NATIVE,

    // 0x200
    ACC_INTERFACE,

    // 0x400
    ACC_ABSTRACT,

    // 0x800
    ACC_STRICT,

    // 0x1000
    ACC_SYNTHETIC,

    // 0x2000
    ACC_ANNOTATION,

    // 0x4000
    ACC_ENUM,

    // 0x8000
    UNUSED,

    // 0x10000
    ACC_CONSTRUCTOR,

    // 0x20000
    ACC_DECLARED_SYNCHRONIZED,

}

const NO_INDEX: u32 = 0xffffffff;

pub fn parse_dex_file(raw: RawDexFile, file: &mut File) -> DexFile {
    let string_data: Vec<_> = raw.string_ids.iter().map(|string_id| {
        file.seek(SeekFrom::Start(string_id.offset as u64))
            .expect(format!("Failed to seek to string data position {}",
                            string_id.offset).as_str());
        let utf16_size = file.read_uleb128().expect("Failed to read string data size");
        Rc::new(file.read_mutf8(utf16_size).expect("Failed to read string data"))
    }).collect();
    let type_identifiers: Vec<_> = raw.type_ids.iter().map(|type_id| {
        string_data[type_id.descriptor_idx as usize].clone()
    }).collect();
    let prototypes: Vec<_> =
        raw.proto_ids.iter().map(|proto_id| {
            Rc::new(Prototype {
                shorty: string_data[proto_id.shorty_idx as usize].clone(),
                return_type: type_identifiers[proto_id.return_type_idx as usize].clone(),
                parameters: parse_type_list(file, &type_identifiers, proto_id.parameters_off),
            })
        }).collect();

    let fields: Vec<_> = raw.field_ids.iter().map(|field_id| {
        Rc::new(Field {
            definer: type_identifiers[field_id.class_idx as usize].clone(),
            type_: type_identifiers[field_id.type_idx as usize].clone(),
            name: string_data[field_id.name_idx as usize].clone(),
        })
    }).collect();

    let methods: Vec<_> = raw.method_ids.iter().map(|method_id| {
        Rc::new(Method {
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

    let classes: Vec<_> = parse_classes(file, &data, &raw.class_defs);

    return DexFile {
        header: Header {
            version: raw.header.version,
            checksum: raw.header.checksum,
            signature: raw.header.signature,
            file_size: raw.header.file_size,
            endianness: raw.header.endianness,
        },
        data: data,
        classes: classes,
    };
}

fn parse_classes(file: &mut File, data: &DexFileData, class_defs: &Vec<RawClassDef>) -> Vec<ClassDefinition> {
    class_defs.iter().map(|class_def| {
        let annotations = parse_annotations(file, &data, class_def.annotations_off);
        ClassDefinition {
            class_type: data.type_identifiers[class_def.class_idx as usize].clone(),
            access_flags: AccessFlags {
                bits: class_def.access_flags,
            },
            superclass: if class_def.class_idx != NO_INDEX {
                Some(data.type_identifiers[class_def.superclass_idx as usize].clone())
            } else { None },
            interfaces: parse_type_list(file, &data.type_identifiers, class_def.interfaces_off),
            source_file_name: if class_def.source_file_idx != NO_INDEX {
                Some(data.string_data[class_def.source_file_idx as usize].clone())
            } else { None },
            annotations: None, // TODO parse annotations
            class_data: parse_class_data(
                file,
                &data,
                class_def.class_data_off,
            ),
            static_values: vec![],
        }
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

trait Also {
    fn also<F: FnOnce(&Self) -> ()>(self, f: F) -> Self where Self: Sized {
        f(&self);
        self
    }
}

impl<T> Also for T {}

impl Parser for File {}

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
            prev_method_idx = method_idx_diff;
        })
    })
}

fn parse_encoded_fields(fields: Vec<RawEncodedField>, data: &DexFileData) -> Vec<EncodedField> {
    fields
        .iter()
        .map(|field| {
            EncodedField {
                field: data.fields[field.field_idx].clone(),
                access_flags: AccessFlags {
                    bits: field.access_flags,
                },
            }
        })
        .collect()
}

fn parse_encoded_methods(file: &mut File, methods: Vec<RawEncodedMethod>, data: &DexFileData) -> Vec<EncodedMethod> {
    methods
        .iter()
        .map(|raw| {
            let method = data.methods[raw.method_idx].clone();
            let access_flags = AccessFlags {
                bits: raw.access_flags,
            };

            EncodedMethod {
                method: method.clone(),
                access_flags,
                code: if raw.code_off != 0 {
                    Some(parse_code(file, data, raw.code_off))
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
    let insns = parse_insns(file, insns_count);

    let tries = parse_tries(file, tries_size);

    let debug_info = parse_debug_info(file, data, debug_info_offset);
    Code {
        registers_size: registers_size,
        ins_size: ins_size,
        outs_size: outs_size,
        debug_info: debug_info,
        insns: insns,
        tries: tries,
        handlers: vec![],
    }
}

fn parse_tries(file: &mut File, tries_size: u16) -> Vec<TryItem> {
    if (tries_size == 0) {
        return vec![];
    }
    file.seek(SeekFrom::Current(2)).expect("Failed to skip tries padding");

    (0..tries_size)
        .map(|_| {
            let start_addr = file.read_u32::<LittleEndian>().expect("Failed to read start_addr");
            let insn_count = file.read_u16::<LittleEndian>().expect("Failed to read insn_count");
            let handler_offset = file.read_u16::<LittleEndian>().expect("Failed to read handler_offset");

            file.seek(SeekFrom::Start(start_addr as u64)).expect("Failed to skip tries padding");
            let code_units = parse_insns(file, insn_count as u32);
            TryItem {
                code_units,
                handler: parse_encoded_catch_handler(file, handler_offset),
            }
        })
        .collect()
}

fn parse_encoded_catch_handler(file: &mut File, handler_offset: u16) -> EncodedCatchHandler {
    file.seek(SeekFrom::Start(handler_offset as u64)).expect("Failed to seek to handler_offset");
    let size = file.read_sleb128().expect("Failed to read handler size");
    let handlers = (0..size.abs())
        .map(|_| {
            let type_idx = file.read_uleb128().expect("Failed to read type_idx");
            let addr = file.read_uleb128().expect("Failed to read addr");
            EncodedTypeAddrPair {
                type_: Rc::new(String::from("")),
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

fn parse_insns(file: &mut File, insns_count: u32) -> Vec<u16> {
    (0..insns_count)
        .map(|_| file.read_u16::<LittleEndian>().expect("Failed to read insn"))
        .collect()
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

fn parse_parameter_names(file: &mut File, data: &DexFileData) -> Vec<Option<Rc<String>>> {
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

fn parse_annotations(file: &mut File, data: &DexFileData, offset: u32) -> Annotations {
    if offset == 0 {
        return Annotations {
            class_annotations: vec![],
            field_annotations: vec![],
            method_annotations: vec![],
            parameter_annotations: vec![],
        };
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
    let value_type = file.read_u8().expect("Failed to read value_type");
    match value_type {
        0x00 => EncodedValue::Byte(file.read_u8().expect("Failed to read byte")),
        0x02 => EncodedValue::Short(file.read_i16::<LittleEndian>().expect("Failed to read short")),
        0x03 => EncodedValue::Char(file.read_u16::<LittleEndian>().expect("Failed to read char")),
        0x04 => EncodedValue::Int(file.read_i32::<LittleEndian>().expect("Failed to read int")),
        0x06 => EncodedValue::Long(file.read_i64::<LittleEndian>().expect("Failed to read long")),
        0x10 => EncodedValue::Float(file.read_f32::<LittleEndian>().expect("Failed to read float")),
        0x11 => EncodedValue::Double(file.read_f64::<LittleEndian>().expect("Failed to read double")),
        0x15 => EncodedValue::MethodType(data.prototypes[file.read_u32::<LittleEndian>().expect("Failed to read method_type_idx") as usize].clone()),
        0x16 => EncodedValue::MethodHandle(data.methods[file.read_u32::<LittleEndian>().expect("Failed to read method_handle_idx") as usize].clone()),
        0x17 => EncodedValue::String(data.string_data[file.read_u32::<LittleEndian>().expect("Failed to read string_idx") as usize].clone()),
        0x18 => EncodedValue::Type(data.type_identifiers[file.read_u32::<LittleEndian>().expect("Failed to read type_idx") as usize].clone()),
        0x19 => EncodedValue::Field(data.fields[file.read_u32::<LittleEndian>().expect("Failed to read field_idx") as usize].clone()),
        0x1a => EncodedValue::Method(data.methods[file.read_u32::<LittleEndian>().expect("Failed to read method_idx") as usize].clone()),
        0x1b => EncodedValue::Enum(data.fields[file.read_u32::<LittleEndian>().expect("Failed to read field_idx") as usize].clone()),
        0x1c => EncodedValue::Array(parse_encoded_array(file, data)),
        0x1d => EncodedValue::Annotation(parse_encoded_annotation(file, data)),
        0x1e => EncodedValue::Null,
        0x1f => EncodedValue::Boolean(file.read_u8().expect("Failed to read boolean") != 0),
        _ => panic!("Invalid encoded value type: {}", value_type),
    }
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
        let name_idx = file.read_u32::<LittleEndian>().expect("Failed to read name_idx");
        let value = parse_encoded_value(file, data);
        AnnotationElement {
            name: data.string_data[name_idx as usize].clone(),
            value,
        }
    })
}


fn parse_type_list(file: &mut File, type_identifiers: &Vec<Rc<String>>, offset: u32) -> Vec<Rc<String>> {
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