use bitmask_enum::bitmask;

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
