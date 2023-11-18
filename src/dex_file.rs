use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::rc::Rc;

use bitmask_enum::bitmask;

use crate::endian_aware_reader::{Endianness, Leb128Ext, MUtf8Ext};
use crate::raw_dex_file::RawDexFile;

#[derive(Debug, PartialEq)]
pub struct DexFile {
    pub header: Header,
    pub string_data: Vec<Rc<String>>,
    pub type_identifiers: Vec<Rc<String>>,
    pub prototypes: Vec<Rc<Prototype>>,
    pub fields: Vec<Rc<Field>>,
    pub methods: Vec<Rc<Method>>,
    pub classes: Vec<ClassDefinition>,
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
    pub annotations: Vec<AnnotationItem>,
}

#[derive(Debug, PartialEq)]
pub struct FieldAnnotation {
    pub field_data: Rc<Field>,
    pub annotations: Vec<AnnotationItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnnotationItem {
    pub visibility: Visibility,
    pub type_: Rc<String>,
    pub annotations: Vec<AnnotationElement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Visibility {
    BUILD,
    RUNTIME,
    SYSTEM,
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
    pub access_flags: Vec<AccessFlags>,
}

#[derive(Debug, PartialEq)]
pub struct EncodedMethod {
    pub method: Rc<Method>,
    pub access_flags: Vec<AccessFlags>,
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
    pub catch_all_addr: Option<u32>,
}

// Docs: encoded_type_addr_pair
#[derive(Debug, PartialEq)]
pub struct EncodedTypeAddrPair {
    // index into type_ids list for the type of exception to catch
    pub type_: Rc<String>,
    // bytecode address of associated exception handler
    pub addr: u32,
}

// Docs: debug_info_item
#[derive(Debug, PartialEq)]
pub struct DebugInfo {
    pub line_start: u32,
    pub parameter_names: Vec<i32>,
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


//noinspection RsEnumVariantNaming
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

    let classes: Vec<_> = raw.class_defs.iter().map(|class_def| {
        let f = AccessFlags::from(class_def.access_flags);
        ClassDefinition {
            class_type: type_identifiers[class_def.class_idx as usize].clone(),
            access_flags: AccessFlags {
                bits: class_def.access_flags,
            },
            superclass: None,
            interfaces: vec![],
            source_file_name: None,
            annotations: None,
            class_data: None,
            static_values: vec![],
        }
    }).collect();

    return DexFile {
        header: Header {
            version: raw.header.version,
            checksum: raw.header.checksum,
            signature: raw.header.signature,
            file_size: raw.header.file_size,
            endianness: raw.header.endianness,
        },
        string_data: string_data,
        type_identifiers: type_identifiers,
        prototypes: prototypes,
        fields: fields,
        methods: methods,
        classes: classes,
    };
}

fn parse_type_list(file: &mut File, type_identifiers: &Vec<Rc<String>>, offset: u32) -> Vec<Rc<String>> {
    file.seek(SeekFrom::Start(offset as u64))
        .expect(format!("Failed to seek to proto_id parameters_off position {}", offset).as_str());
    let size = file.read_uleb128().expect("Failed to read parameters size");
    (0..size)
        .map(|_| file.read_uleb128().expect("Failed to read parameter type"))
        .map(|idx| type_identifiers[idx as usize].clone())
        .collect()
}