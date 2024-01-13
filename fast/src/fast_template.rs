use ::libc;
extern "C" {
    pub type _GHashTable;
    pub type _xmlDict;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn g_hash_table_new(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
    ) -> *mut GHashTable;
    fn g_hash_table_insert(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlParseFile(filename: *const libc::c_char) -> xmlDocPtr;
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    static mut xmlFree: xmlFreeFunc;
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn xmlChildElementCount(parent: xmlNodePtr) -> libc::c_ulong;
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type i64_0 = int64_t;
pub type u64_0 = uint64_t;
pub type size_t = libc::c_ulong;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
pub type GEqualFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
>;
pub type GHashFunc = Option::<unsafe extern "C" fn(gconstpointer) -> guint>;
pub type ssize_t = __ssize_t;
pub type socklen_t = __socklen_t;
pub type GHashTable = _GHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub start: libc::c_ulong,
    pub end: libc::c_ulong,
    pub capacity: libc::c_ulong,
    pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_session {
    pub last_tid: u64_0,
    pub sockfd: libc::c_int,
    pub rx_buffer: *mut buffer,
    pub tx_pmap_buffer: *mut buffer,
    pub tx_message_buffer: *mut buffer,
    pub nr_messages: libc::c_int,
    pub rx_message: *mut fast_message,
    pub rx_messages: *mut fast_message,
    pub preamble: fast_preamble,
    pub pmap: fast_pmap,
    pub reset: bool,
    pub recv: Option::<
        unsafe extern "C" fn(*mut buffer, libc::c_int, size_t, libc::c_int) -> ssize_t,
    >,
    pub send: Option::<
        unsafe extern "C" fn(libc::c_int, *const msghdr, libc::c_int) -> ssize_t,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_pmap {
    pub is_valid: bool,
    pub pmap_bit: libc::c_long,
    pub nr_bytes: libc::c_ulong,
    pub bytes: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_preamble {
    pub is_valid: bool,
    pub nr_bytes: libc::c_ulong,
    pub bytes: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_message {
    pub nr_fields: libc::c_ulong,
    pub decoded: libc::c_ulong,
    pub fields: *mut fast_field,
    pub ghtab: *mut GHashTable,
    pub name: [libc::c_char; 32],
    pub flags: libc::c_int,
    pub tid: libc::c_ulong,
    pub pmap_buf: *mut buffer,
    pub msg_buf: *mut buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_field {
    pub presence: fast_presence,
    pub type_0: fast_type,
    pub op: fast_op,
    pub state: fast_state,
    pub state_previous: fast_state,
    pub has_reset: bool,
    pub flags: libc::c_int,
    pub name: [libc::c_char; 32],
    pub id: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed_0,
    pub c2rust_unnamed_1: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub int_previous: i64_0,
    pub uint_previous: u64_0,
    pub ptr_previous: *mut libc::c_void,
    pub string_previous: [libc::c_char; 256],
    pub vector_previous: [libc::c_char; 256],
    pub decimal_previous: fast_decimal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_decimal {
    pub fields: *mut fast_field,
    pub exp: i64_0,
    pub mnt: i64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub int_reset: i64_0,
    pub uint_reset: u64_0,
    pub ptr_reset: *mut libc::c_void,
    pub string_reset: [libc::c_char; 256],
    pub vector_reset: [libc::c_char; 256],
    pub decimal_reset: fast_decimal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub int_value: i64_0,
    pub uint_value: u64_0,
    pub ptr_value: *mut libc::c_void,
    pub string_value: [libc::c_char; 256],
    pub vector_value: [libc::c_char; 256],
    pub decimal_value: fast_decimal,
}
pub type fast_state = libc::c_uint;
pub const FAST_STATE_EMPTY: fast_state = 2;
pub const FAST_STATE_ASSIGNED: fast_state = 1;
pub const FAST_STATE_UNDEFINED: fast_state = 0;
pub type fast_op = libc::c_uint;
pub const FAST_OP_CONSTANT: fast_op = 5;
pub const FAST_OP_DEFAULT: fast_op = 4;
pub const FAST_OP_DELTA: fast_op = 3;
pub const FAST_OP_INCR: fast_op = 2;
pub const FAST_OP_COPY: fast_op = 1;
pub const FAST_OP_NONE: fast_op = 0;
pub type fast_type = libc::c_uint;
pub const FAST_TYPE_SEQUENCE: fast_type = 5;
pub const FAST_TYPE_DECIMAL: fast_type = 4;
pub const FAST_TYPE_VECTOR: fast_type = 3;
pub const FAST_TYPE_STRING: fast_type = 2;
pub const FAST_TYPE_UINT: fast_type = 1;
pub const FAST_TYPE_INT: fast_type = 0;
pub type fast_presence = libc::c_uint;
pub const FAST_PRESENCE_MANDATORY: fast_presence = 1;
pub const FAST_PRESENCE_OPTIONAL: fast_presence = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_sequence {
    pub pmap: fast_pmap,
    pub decoded: libc::c_ulong,
    pub length: fast_field,
    pub elements: [fast_message; 128],
}
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *mut libc::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: libc::c_int,
    pub standalone: libc::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut libc::c_void,
    pub refs: *mut libc::c_void,
    pub URL: *const xmlChar,
    pub charset: libc::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut libc::c_void,
    pub parseFlags: libc::c_int,
    pub properties: libc::c_int,
}
pub type xmlChar = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut libc::c_void,
    pub context: *mut _xmlDoc,
}
pub type xmlElementType = xmlNsType;
pub type xmlNsType = libc::c_uint;
pub const XML_DOCB_DOCUMENT_NODE: xmlNsType = 21;
pub const XML_XINCLUDE_END: xmlNsType = 20;
pub const XML_XINCLUDE_START: xmlNsType = 19;
pub const XML_NAMESPACE_DECL: xmlNsType = 18;
pub const XML_ENTITY_DECL: xmlNsType = 17;
pub const XML_ATTRIBUTE_DECL: xmlNsType = 16;
pub const XML_ELEMENT_DECL: xmlNsType = 15;
pub const XML_DTD_NODE: xmlNsType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlNsType = 13;
pub const XML_NOTATION_NODE: xmlNsType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlNsType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlNsType = 10;
pub const XML_DOCUMENT_NODE: xmlNsType = 9;
pub const XML_COMMENT_NODE: xmlNsType = 8;
pub const XML_PI_NODE: xmlNsType = 7;
pub const XML_ENTITY_NODE: xmlNsType = 6;
pub const XML_ENTITY_REF_NODE: xmlNsType = 5;
pub const XML_CDATA_SECTION_NODE: xmlNsType = 4;
pub const XML_TEXT_NODE: xmlNsType = 3;
pub const XML_ATTRIBUTE_NODE: xmlNsType = 2;
pub const XML_ELEMENT_NODE: xmlNsType = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut libc::c_void,
    pub elements: *mut libc::c_void,
    pub attributes: *mut libc::c_void,
    pub entities: *mut libc::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut libc::c_void,
    pub line: libc::c_ushort,
    pub extra: libc::c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut libc::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut libc::c_void,
}
pub type xmlAttributeType = libc::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub const _ISblank: C2RustUnnamed_2 = 1;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_2 = 8;
pub const _ISpunct: C2RustUnnamed_2 = 4;
pub const _IScntrl: C2RustUnnamed_2 = 2;
pub const _ISgraph: C2RustUnnamed_2 = 32768;
pub const _ISprint: C2RustUnnamed_2 = 16384;
pub const _ISspace: C2RustUnnamed_2 = 8192;
pub const _ISxdigit: C2RustUnnamed_2 = 4096;
pub const _ISdigit: C2RustUnnamed_2 = 2048;
pub const _ISalpha: C2RustUnnamed_2 = 1024;
pub const _ISlower: C2RustUnnamed_2 = 512;
pub const _ISupper: C2RustUnnamed_2 = 256;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn field_is_mandatory(mut field: *mut fast_field) -> bool {
    return (*field).presence as libc::c_uint
        == FAST_PRESENCE_MANDATORY as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn field_add_flags(
    mut field: *mut fast_field,
    mut flags: libc::c_int,
) {
    (*field).flags |= flags;
}
#[inline]
unsafe extern "C" fn fast_msg_add_flags(
    mut msg: *mut fast_message,
    mut flags: libc::c_int,
) {
    (*msg).flags |= flags;
}
#[inline]
unsafe extern "C" fn pmap_required(mut field: *mut fast_field) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    match (*field).op as libc::c_uint {
        5 => {
            if !field_is_mandatory(field) {
                ret = 1 as libc::c_int;
            }
        }
        1 | 2 | 4 => {
            ret = 1 as libc::c_int;
        }
        0 | 3 | _ => {}
    }
    return ret;
}
unsafe extern "C" fn fast_presence_init(
    mut node: xmlNodePtr,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut prop: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if node.is_null() {
        return 1 as libc::c_int;
    }
    prop = xmlGetProp(
        node as *const xmlNode,
        b"presence\0" as *const u8 as *const libc::c_char as *const xmlChar,
    );
    if prop.is_null() {
        (*field).presence = FAST_PRESENCE_MANDATORY;
    } else if xmlStrcmp(
        prop,
        b"mandatory\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
    {
        (*field).presence = FAST_PRESENCE_MANDATORY;
    } else if xmlStrcmp(
        prop,
        b"optional\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
    {
        (*field).presence = FAST_PRESENCE_OPTIONAL;
    } else {
        ret = 1 as libc::c_int;
    }
    xmlFree.expect("non-null function pointer")(prop as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn fast_misc_init(
    mut node: xmlNodePtr,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut prop: *mut xmlChar = 0 as *mut xmlChar;
    if node.is_null() {
        return 1 as libc::c_int;
    }
    prop = xmlGetProp(
        node as *const xmlNode,
        b"charset\0" as *const u8 as *const libc::c_char as *const xmlChar,
    );
    if !prop.is_null()
        && xmlStrcmp(
            prop,
            b"unicode\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        field_add_flags(field, 0x1 as libc::c_int);
    }
    xmlFree.expect("non-null function pointer")(prop as *mut libc::c_void);
    prop = xmlGetProp(
        node as *const xmlNode,
        b"name\0" as *const u8 as *const libc::c_char as *const xmlChar,
    );
    if !prop.is_null() {
        strncpy(
            ((*field).name).as_mut_ptr(),
            prop as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    }
    xmlFree.expect("non-null function pointer")(prop as *mut libc::c_void);
    prop = xmlGetProp(
        node as *const xmlNode,
        b"id\0" as *const u8 as *const libc::c_char as *const xmlChar,
    );
    if !prop.is_null() {
        (*field).id = atoi(prop as *const libc::c_char);
    }
    xmlFree.expect("non-null function pointer")(prop as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn fast_type_init(
    mut node: xmlNodePtr,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if node.is_null() {
        ret = 1 as libc::c_int;
    } else if xmlStrcmp(
        (*node).name,
        b"int32\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
        || xmlStrcmp(
            (*node).name,
            b"int64\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        (*field).type_0 = FAST_TYPE_INT;
    } else if xmlStrcmp(
        (*node).name,
        b"uInt32\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
        || xmlStrcmp(
            (*node).name,
            b"uInt64\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        (*field).type_0 = FAST_TYPE_UINT;
    } else if xmlStrcmp(
        (*node).name,
        b"length\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
        || xmlStrcmp(
            (*node).name,
            b"Length\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        (*field).type_0 = FAST_TYPE_UINT;
    } else if xmlStrcmp(
        (*node).name,
        b"string\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
        || xmlStrcmp(
            (*node).name,
            b"String\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        (*field).type_0 = FAST_TYPE_STRING;
    } else if xmlStrcmp(
        (*node).name,
        b"decimal\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
        || xmlStrcmp(
            (*node).name,
            b"Decimal\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        (*field).type_0 = FAST_TYPE_DECIMAL;
    } else if xmlStrcmp(
        (*node).name,
        b"sequence\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
        || xmlStrcmp(
            (*node).name,
            b"Sequence\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        (*field).type_0 = FAST_TYPE_SEQUENCE;
    } else if xmlStrcmp(
        (*node).name,
        b"exponent\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
        || xmlStrcmp(
            (*node).name,
            b"Exponent\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        (*field).type_0 = FAST_TYPE_INT;
    } else if xmlStrcmp(
        (*node).name,
        b"mantissa\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
        || xmlStrcmp(
            (*node).name,
            b"Mantissa\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        (*field).type_0 = FAST_TYPE_INT;
    } else if xmlStrcmp(
        (*node).name,
        b"bytevector\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
        || xmlStrcmp(
            (*node).name,
            b"byteVector\0" as *const u8 as *const libc::c_char as *const xmlChar,
        ) == 0
    {
        (*field).type_0 = FAST_TYPE_VECTOR;
    } else {
        ret = 1 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn fast_op_init(
    mut node: xmlNodePtr,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if node.is_null() {
        (*field).op = FAST_OP_NONE;
    } else if xmlStrcmp(
        (*node).name,
        b"copy\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
    {
        (*field).op = FAST_OP_COPY;
    } else if xmlStrcmp(
        (*node).name,
        b"delta\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
    {
        (*field).op = FAST_OP_DELTA;
    } else if xmlStrcmp(
        (*node).name,
        b"default\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
    {
        (*field).op = FAST_OP_DEFAULT;
    } else if xmlStrcmp(
        (*node).name,
        b"constant\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
    {
        (*field).op = FAST_OP_CONSTANT;
    } else if xmlStrcmp(
        (*node).name,
        b"increment\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) == 0
    {
        (*field).op = FAST_OP_INCR;
    } else {
        ret = 1 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn fast_vector_init(
    mut in_0: *const libc::c_char,
    mut out: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut hex: libc::c_int = 0;
    loop {
        if !(*in_0 != 0) {
            current_block = 7502529970979898288;
            break;
        }
        if !(*(*__ctype_b_loc()).offset(*in_0 as libc::c_int as isize) as libc::c_int
            & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            let fresh0 = in_0;
            in_0 = in_0.offset(1);
            if sscanf(
                fresh0,
                b"%02X\0" as *const u8 as *const libc::c_char,
                &mut hex as *mut libc::c_int,
            ) != 1 as libc::c_int
            {
                current_block = 1639629089557669991;
                break;
            }
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = hex as libc::c_char;
        }
        in_0 = in_0.offset(1);
        in_0;
    }
    match current_block {
        1639629089557669991 => return -(1 as libc::c_int),
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn fast_reset_init(
    mut node: xmlNodePtr,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut prop: *mut xmlChar = 0 as *mut xmlChar;
    (*field).has_reset = 0 as libc::c_int != 0;
    match (*field).type_0 as libc::c_uint {
        0 => {
            (*field).c2rust_unnamed.int_value = 0 as libc::c_int as i64_0;
            (*field).c2rust_unnamed_1.int_previous = 0 as libc::c_int as i64_0;
            if !node.is_null() {
                prop = xmlGetProp(
                    node as *const xmlNode,
                    b"value\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
                if !prop.is_null() {
                    (*field).has_reset = 1 as libc::c_int != 0;
                    (*field)
                        .c2rust_unnamed_0
                        .int_reset = strtol(
                        prop as *mut libc::c_char,
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    (*field)
                        .c2rust_unnamed
                        .int_value = (*field).c2rust_unnamed_0.int_reset;
                    (*field)
                        .c2rust_unnamed_1
                        .int_previous = (*field).c2rust_unnamed_0.int_reset;
                    xmlFree
                        .expect("non-null function pointer")(prop as *mut libc::c_void);
                }
            }
        }
        1 => {
            (*field).c2rust_unnamed.uint_value = 0 as libc::c_int as u64_0;
            (*field).c2rust_unnamed_1.uint_previous = 0 as libc::c_int as u64_0;
            if !node.is_null() {
                prop = xmlGetProp(
                    node as *const xmlNode,
                    b"value\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
                if !prop.is_null() {
                    (*field).has_reset = 1 as libc::c_int != 0;
                    (*field)
                        .c2rust_unnamed_0
                        .uint_reset = strtoul(
                        prop as *mut libc::c_char,
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    (*field)
                        .c2rust_unnamed
                        .uint_value = (*field).c2rust_unnamed_0.uint_reset;
                    (*field)
                        .c2rust_unnamed_1
                        .uint_previous = (*field).c2rust_unnamed_0.uint_reset;
                    xmlFree
                        .expect("non-null function pointer")(prop as *mut libc::c_void);
                }
            }
        }
        2 => {
            (*field)
                .c2rust_unnamed
                .string_value[0 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_char;
            (*field)
                .c2rust_unnamed_1
                .string_previous[0 as libc::c_int
                as usize] = 0 as libc::c_int as libc::c_char;
            if !node.is_null() {
                prop = xmlGetProp(
                    node as *const xmlNode,
                    b"value\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
                if !prop.is_null() {
                    (*field).has_reset = 1 as libc::c_int != 0;
                    strcpy(
                        ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr(),
                        prop as *mut libc::c_char,
                    );
                    strcpy(
                        ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                        prop as *mut libc::c_char,
                    );
                    strcpy(
                        ((*field).c2rust_unnamed_1.string_previous).as_mut_ptr(),
                        prop as *mut libc::c_char,
                    );
                    xmlFree
                        .expect("non-null function pointer")(prop as *mut libc::c_void);
                }
            }
        }
        3 => {
            memset(
                ((*field).c2rust_unnamed.vector_value).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            );
            memset(
                ((*field).c2rust_unnamed_1.vector_previous).as_mut_ptr()
                    as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            );
            if !node.is_null() {
                prop = xmlGetProp(
                    node as *const xmlNode,
                    b"value\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
                if !prop.is_null() {
                    (*field).has_reset = 1 as libc::c_int != 0;
                    ret = fast_vector_init(
                        prop as *const libc::c_char,
                        ((*field).c2rust_unnamed_0.vector_reset).as_mut_ptr(),
                    );
                    if !(ret != 0) {
                        memcpy(
                            ((*field).c2rust_unnamed.vector_value).as_mut_ptr()
                                as *mut libc::c_void,
                            ((*field).c2rust_unnamed_0.vector_reset).as_mut_ptr()
                                as *const libc::c_void,
                            ::core::mem::size_of::<[libc::c_char; 256]>()
                                as libc::c_ulong,
                        );
                        memcpy(
                            ((*field).c2rust_unnamed_1.vector_previous).as_mut_ptr()
                                as *mut libc::c_void,
                            ((*field).c2rust_unnamed_0.vector_reset).as_mut_ptr()
                                as *const libc::c_void,
                            ::core::mem::size_of::<[libc::c_char; 256]>()
                                as libc::c_ulong,
                        );
                    }
                }
            }
        }
        4 => {
            (*field).c2rust_unnamed.decimal_value.exp = 0 as libc::c_int as i64_0;
            (*field).c2rust_unnamed.decimal_value.mnt = 0 as libc::c_int as i64_0;
            (*field).c2rust_unnamed_1.decimal_previous.exp = 0 as libc::c_int as i64_0;
            (*field).c2rust_unnamed_1.decimal_previous.mnt = 0 as libc::c_int as i64_0;
        }
        5 => {
            ret = 1 as libc::c_int;
        }
        _ => {
            ret = 1 as libc::c_int;
        }
    }
    return ret;
}
unsafe extern "C" fn fast_sequence_init(
    mut node: xmlNodePtr,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut seq: *mut fast_sequence = 0 as *mut fast_sequence;
    let mut msg: *mut fast_message = 0 as *mut fast_message;
    let mut orig: *mut fast_field = 0 as *mut fast_field;
    let mut tmp: xmlNodePtr = 0 as *mut xmlNode;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut nr_fields: libc::c_int = 0;
    (*field)
        .c2rust_unnamed
        .ptr_value = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<fast_sequence>() as libc::c_ulong,
    );
    if !((*field).c2rust_unnamed.ptr_value).is_null() {
        seq = (*field).c2rust_unnamed.ptr_value as *mut fast_sequence;
        nr_fields = xmlChildElementCount(node) as libc::c_int;
        node = (*node).children;
        while !node.is_null()
            && (*node).type_0 as libc::c_uint
                != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        {
            node = (*node).next;
        }
        if !node.is_null() {
            if !(xmlStrcmp(
                (*node).name,
                b"length\0" as *const u8 as *const libc::c_char as *const xmlChar,
            ) != 0)
            {
                if !(fast_field_init(node, &mut (*seq).length) != 0) {
                    if !field_is_mandatory(field) {
                        (*seq).length.presence = FAST_PRESENCE_OPTIONAL;
                    }
                    node = (*node).next;
                    orig = field;
                    i = 0 as libc::c_int;
                    's_69: loop {
                        if !(i < 128 as libc::c_int) {
                            current_block = 17788412896529399552;
                            break;
                        }
                        msg = ((*seq).elements).as_mut_ptr().offset(i as isize);
                        tmp = node;
                        (*msg)
                            .fields = calloc(
                            nr_fields as libc::c_ulong,
                            ::core::mem::size_of::<fast_field>() as libc::c_ulong,
                        ) as *mut fast_field;
                        if ((*msg).fields).is_null() {
                            current_block = 16349547187377868190;
                            break;
                        }
                        (*msg)
                            .ghtab = g_hash_table_new(
                            Some(
                                g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint,
                            ),
                            Some(
                                g_str_equal
                                    as unsafe extern "C" fn(
                                        gconstpointer,
                                        gconstpointer,
                                    ) -> gboolean,
                            ),
                        );
                        if ((*msg).ghtab).is_null() {
                            current_block = 16349547187377868190;
                            break;
                        }
                        (*msg).nr_fields = 0 as libc::c_int as libc::c_ulong;
                        while !tmp.is_null() {
                            if (*tmp).type_0 as libc::c_uint
                                != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                            {
                                tmp = (*tmp).next;
                            } else {
                                field = ((*msg).fields).offset((*msg).nr_fields as isize);
                                if fast_field_init(tmp, field) != 0 {
                                    current_block = 16349547187377868190;
                                    break 's_69;
                                }
                                if strlen(((*field).name).as_mut_ptr()) != 0 {
                                    g_hash_table_insert(
                                        (*msg).ghtab,
                                        ((*field).name).as_mut_ptr() as gpointer,
                                        field as gpointer,
                                    );
                                }
                                if pmap_required(field) != 0 {
                                    field_add_flags(orig, 0x2 as libc::c_int);
                                }
                                (*msg).nr_fields = ((*msg).nr_fields).wrapping_add(1);
                                (*msg).nr_fields;
                                tmp = (*tmp).next;
                            }
                        }
                        i += 1;
                        i;
                    }
                    match current_block {
                        16349547187377868190 => {}
                        _ => {
                            ret = 0 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    return ret;
}
unsafe extern "C" fn fast_decimal_init_atomic(
    mut node: xmlNodePtr,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    node = (*node).children;
    while !node.is_null()
        && (*node).type_0 as libc::c_uint
            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
    {
        node = (*node).next;
    }
    ret = fast_op_init(node, field);
    if !(ret != 0) {
        ret = fast_reset_init(node, field);
        ret != 0;
    }
    return ret;
}
unsafe extern "C" fn fast_decimal_init_individ(
    mut node: xmlNodePtr,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut decimal: *mut fast_decimal = 0 as *mut fast_decimal;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    decimal = &mut (*field).c2rust_unnamed.decimal_value;
    (*decimal)
        .fields = calloc(
        2 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<fast_field>() as libc::c_ulong,
    ) as *mut fast_field;
    if !((*decimal).fields).is_null() {
        node = (*node).children;
        loop {
            if node.is_null() {
                current_block = 2979737022853876585;
                break;
            }
            if (*node).type_0 as libc::c_uint
                != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
            {
                node = (*node).next;
            } else {
                if xmlStrcmp(
                    (*node).name,
                    b"exponent\0" as *const u8 as *const libc::c_char as *const xmlChar,
                ) == 0
                {
                    ret = fast_field_init(
                        node,
                        &mut *((*decimal).fields).offset(0 as libc::c_int as isize),
                    );
                } else if xmlStrcmp(
                    (*node).name,
                    b"mantissa\0" as *const u8 as *const libc::c_char as *const xmlChar,
                ) == 0
                {
                    ret = fast_field_init(
                        node,
                        &mut *((*decimal).fields).offset(1 as libc::c_int as isize),
                    );
                } else {
                    ret = -(1 as libc::c_int);
                }
                if ret != 0 {
                    current_block = 3631892188252722159;
                    break;
                }
                node = (*node).next;
            }
        }
        match current_block {
            3631892188252722159 => {}
            _ => {
                field_add_flags(field, 0x4 as libc::c_int);
                (*((*decimal).fields).offset(0 as libc::c_int as isize))
                    .presence = (*field).presence;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn fast_field_init(
    mut node: xmlNodePtr,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    (*field).state = FAST_STATE_UNDEFINED;
    ret = fast_type_init(node, field);
    if !(ret != 0) {
        ret = fast_presence_init(node, field);
        if !(ret != 0) {
            ret = fast_misc_init(node, field);
            if !(ret != 0) {
                match (*field).type_0 as libc::c_uint {
                    0 | 1 | 2 | 3 => {
                        node = (*node).children;
                        while !node.is_null()
                            && (*node).type_0 as libc::c_uint
                                != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                        {
                            node = (*node).next;
                        }
                        ret = fast_op_init(node, field);
                        if !(ret != 0) {
                            ret = fast_reset_init(node, field);
                            ret != 0;
                        }
                    }
                    4 => {
                        (*field)
                            .c2rust_unnamed
                            .decimal_value
                            .fields = 0 as *mut fast_field;
                        ret = fast_decimal_init_atomic(node, field);
                        if !(ret == 0) {
                            ret = fast_decimal_init_individ(node, field);
                        }
                    }
                    5 => {
                        ret = fast_sequence_init(node, field);
                    }
                    _ => {
                        ret = 1 as libc::c_int;
                    }
                }
            }
        }
    }
    return ret;
}
unsafe extern "C" fn fast_message_init(
    mut node: xmlNodePtr,
    mut msg: *mut fast_message,
) -> libc::c_int {
    let mut current_block: u64;
    let mut field: *mut fast_field = 0 as *mut fast_field;
    let mut nr_fields: libc::c_int = 0;
    let mut prop: *mut xmlChar = 0 as *mut xmlChar;
    let mut ret: libc::c_int = 1 as libc::c_int;
    if !(xmlStrcmp(
        (*node).name,
        b"template\0" as *const u8 as *const libc::c_char as *const xmlChar,
    ) != 0)
    {
        prop = xmlGetProp(
            node as *const xmlNode,
            b"id\0" as *const u8 as *const libc::c_char as *const xmlChar,
        );
        if !prop.is_null() {
            (*msg)
                .tid = strtol(
                prop as *mut libc::c_char,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            ) as libc::c_ulong;
        } else {
            (*msg).tid = 0 as libc::c_int as libc::c_ulong;
        }
        xmlFree.expect("non-null function pointer")(prop as *mut libc::c_void);
        prop = xmlGetProp(
            node as *const xmlNode,
            b"name\0" as *const u8 as *const libc::c_char as *const xmlChar,
        );
        if !prop.is_null() {
            strncpy(
                ((*msg).name).as_mut_ptr(),
                prop as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            );
        } else {
            strncpy(
                ((*msg).name).as_mut_ptr(),
                b"\0" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            );
        }
        xmlFree.expect("non-null function pointer")(prop as *mut libc::c_void);
        prop = xmlGetProp(
            node as *const xmlNode,
            b"reset\0" as *const u8 as *const libc::c_char as *const xmlChar,
        );
        if !prop.is_null()
            && xmlStrcmp(
                prop,
                b"T\0" as *const u8 as *const libc::c_char as *const xmlChar,
            ) == 0
        {
            fast_msg_add_flags(msg, 0x1 as libc::c_int);
        }
        xmlFree.expect("non-null function pointer")(prop as *mut libc::c_void);
        nr_fields = xmlChildElementCount(node) as libc::c_int;
        (*msg)
            .fields = calloc(
            nr_fields as libc::c_ulong,
            ::core::mem::size_of::<fast_field>() as libc::c_ulong,
        ) as *mut fast_field;
        if !((*msg).fields).is_null() {
            (*msg).nr_fields = 0 as libc::c_int as libc::c_ulong;
            (*msg)
                .ghtab = g_hash_table_new(
                Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                Some(
                    g_str_equal
                        as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
                ),
            );
            if !((*msg).ghtab).is_null() {
                node = (*node).children;
                loop {
                    if node.is_null() {
                        current_block = 16203760046146113240;
                        break;
                    }
                    if (*node).type_0 as libc::c_uint
                        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        node = (*node).next;
                    } else {
                        field = ((*msg).fields).offset((*msg).nr_fields as isize);
                        if fast_field_init(node, field) != 0 {
                            current_block = 8109038237566646675;
                            break;
                        }
                        if strlen(((*field).name).as_mut_ptr()) != 0 {
                            g_hash_table_insert(
                                (*msg).ghtab,
                                ((*field).name).as_mut_ptr() as gpointer,
                                field as gpointer,
                            );
                        }
                        (*msg).nr_fields = ((*msg).nr_fields).wrapping_add(1);
                        (*msg).nr_fields;
                        node = (*node).next;
                    }
                }
                match current_block {
                    8109038237566646675 => {}
                    _ => {
                        ret = 0 as libc::c_int;
                    }
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fast_parse_template(
    mut self_0: *mut fast_session,
    mut xml: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut msg: *mut fast_message = 0 as *mut fast_message;
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ret: libc::c_int = 1 as libc::c_int;
    doc = xmlParseFile(xml);
    if !doc.is_null() {
        node = xmlDocGetRootElement(doc as *const xmlDoc);
        if !node.is_null() {
            if !(xmlStrcmp(
                (*node).name,
                b"templates\0" as *const u8 as *const libc::c_char as *const xmlChar,
            ) != 0)
            {
                if !(xmlChildElementCount(node) > 128 as libc::c_int as libc::c_ulong) {
                    node = (*node).children;
                    loop {
                        if node.is_null() {
                            current_block = 10599921512955367680;
                            break;
                        }
                        if (*node).type_0 as libc::c_uint
                            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                        {
                            node = (*node).next;
                        } else {
                            msg = ((*self_0).rx_messages)
                                .offset((*self_0).nr_messages as isize);
                            if fast_message_init(node, msg) != 0 {
                                current_block = 11752075399845587236;
                                break;
                            }
                            (*self_0).nr_messages += 1;
                            (*self_0).nr_messages;
                            node = (*node).next;
                        }
                    }
                    match current_block {
                        11752075399845587236 => {}
                        _ => {
                            ret = 0 as libc::c_int;
                        }
                    }
                }
            }
            xmlFreeDoc(doc);
        }
    }
    return ret;
}
