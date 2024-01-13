use ::libc;
extern "C" {
    pub type _GHashTable;
    pub type _GTree;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type _xmlDict;
    pub type ldat;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fast_books_fini(set: *mut fast_book_set) -> libc::c_int;
    fn fast_books_init(set: *mut fast_book_set) -> libc::c_int;
    fn fast_books_update(set: *mut fast_book_set) -> libc::c_int;
    fn fast_books_subscribe(
        set: *mut fast_book_set,
        book: *mut fast_book,
    ) -> libc::c_int;
    fn ob_init(ob: *mut order_book) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn g_list_last(list: *mut GList) -> *mut GList;
    fn g_list_nth_data(list: *mut GList, n: guint) -> gpointer;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn xmlStrcmp(str1: *const xmlChar, str2: *const xmlChar) -> libc::c_int;
    fn xmlFreeDoc(cur: xmlDocPtr);
    fn xmlDocGetRootElement(doc: *const xmlDoc) -> xmlNodePtr;
    fn xmlGetProp(node: *const xmlNode, name: *const xmlChar) -> *mut xmlChar;
    fn xmlParseFile(filename: *const libc::c_char) -> xmlDocPtr;
    fn cbreak() -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn initscr() -> *mut WINDOW;
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short) -> libc::c_int;
    fn mvprintw(
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn noecho() -> libc::c_int;
    fn start_color() -> libc::c_int;
    fn wattr_on(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wattr_off(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wclear(_: *mut WINDOW) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    static mut stdscr: *mut WINDOW;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type i64_0 = int64_t;
pub type u64_0 = uint64_t;
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
pub type GTree = _GTree;
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
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
pub union C2RustUnnamed_11 {
    pub int_reset: i64_0,
    pub uint_reset: u64_0,
    pub ptr_reset: *mut libc::c_void,
    pub string_reset: [libc::c_char; 256],
    pub vector_reset: [libc::c_char; 256],
    pub decimal_reset: fast_decimal,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
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
pub struct fast_session_cfg {
    pub preamble_bytes: libc::c_int,
    pub sockfd: libc::c_int,
    pub reset: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_feed {
    pub session: *mut fast_session,
    pub cfg: fast_session_cfg,
    pub recv_num: u64_0,
    pub file: [libc::c_char; 64],
    pub xml: [libc::c_char; 128],
    pub lip: [libc::c_char; 32],
    pub sip: [libc::c_char; 32],
    pub ip: [libc::c_char; 32],
    pub active: bool,
    pub port: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ob_level {
    pub seq_num: libc::c_ulong,
    pub price: libc::c_ulong,
    pub size: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct order_book {
    pub ghbids: *mut GHashTable,
    pub ghasks: *mut GHashTable,
    pub gtbids: *mut GTree,
    pub gtasks: *mut GTree,
    pub glbids: *mut GList,
    pub glasks: *mut GList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_book {
    pub tick: fast_decimal,
    pub ob: order_book,
    pub session: [libc::c_char; 32],
    pub symbol: [libc::c_char; 32],
    pub rptseq: u64_0,
    pub snpseq: u64_0,
    pub flags: libc::c_int,
    pub secid: u64_0,
    pub num: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_book_set {
    pub inc_feeds: [fast_feed; 2],
    pub snp_feeds: [fast_feed; 2],
    pub inc_feeds_num: libc::c_ulong,
    pub snp_feeds_num: libc::c_ulong,
    pub books_mask: [u64_0; 1],
    pub books: [fast_book; 10],
    pub books_num: libc::c_ulong,
    pub inc_gap_mode: bool,
    pub inc_msg_num: u64_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type xmlChar = libc::c_uchar;
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
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut libc::c_void,
    pub context: *mut _xmlDoc,
}
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
pub type xmlElementType = libc::c_uint;
pub const XML_DOCB_DOCUMENT_NODE: xmlElementType = 21;
pub const XML_XINCLUDE_END: xmlElementType = 20;
pub const XML_XINCLUDE_START: xmlElementType = 19;
pub const XML_NAMESPACE_DECL: xmlElementType = 18;
pub const XML_ENTITY_DECL: xmlElementType = 17;
pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const XML_ELEMENT_DECL: xmlElementType = 15;
pub const XML_DTD_NODE: xmlElementType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const XML_NOTATION_NODE: xmlElementType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const XML_DOCUMENT_NODE: xmlElementType = 9;
pub const XML_COMMENT_NODE: xmlElementType = 8;
pub const XML_PI_NODE: xmlElementType = 7;
pub const XML_ENTITY_NODE: xmlElementType = 6;
pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const XML_TEXT_NODE: xmlElementType = 3;
pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const XML_ELEMENT_NODE: xmlElementType = 1;
pub type xmlNsType = xmlElementType;
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
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlDoc = _xmlDoc;
pub type chtype = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cchar_t {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: libc::c_int,
}
pub type attr_t = chtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub type WINDOW = _win_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn snp_feed_add(mut set: *mut fast_book_set) -> *mut fast_feed {
    let mut feed: *mut fast_feed = 0 as *mut fast_feed;
    if (*set).snp_feeds_num >= 2 as libc::c_int as libc::c_ulong {
        return 0 as *mut fast_feed
    } else {
        feed = ((*set).snp_feeds).as_mut_ptr().offset((*set).snp_feeds_num as isize);
        memset(
            feed as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<fast_feed>() as libc::c_ulong,
        );
        (*set).snp_feeds_num = ((*set).snp_feeds_num).wrapping_add(1);
        (*set).snp_feeds_num;
        return feed;
    };
}
#[inline]
unsafe extern "C" fn inc_feed_add(mut set: *mut fast_book_set) -> *mut fast_feed {
    let mut feed: *mut fast_feed = 0 as *mut fast_feed;
    if (*set).inc_feeds_num >= 2 as libc::c_int as libc::c_ulong {
        return 0 as *mut fast_feed
    } else {
        feed = ((*set).inc_feeds).as_mut_ptr().offset((*set).inc_feeds_num as isize);
        memset(
            feed as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<fast_feed>() as libc::c_ulong,
        );
        (*set).inc_feeds_num = ((*set).inc_feeds_num).wrapping_add(1);
        (*set).inc_feeds_num;
        return feed;
    };
}
#[inline]
unsafe extern "C" fn fast_book_add(mut set: *mut fast_book_set) -> *mut fast_book {
    let mut book: *mut fast_book = 0 as *mut fast_book;
    if !((*set).books_num >= 10 as libc::c_int as libc::c_ulong) {
        book = ((*set).books).as_mut_ptr().offset((*set).books_num as isize);
        memset(
            book as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<fast_book>() as libc::c_ulong,
        );
        (*book).num = (*set).books_num as libc::c_int;
        (*set).books_num = ((*set).books_num).wrapping_add(1);
        (*set).books_num;
        if !(ob_init(&mut (*book).ob) != 0) {
            return book;
        }
    }
    return 0 as *mut fast_book;
}
#[inline]
unsafe extern "C" fn book_has_mask(
    mut set: *mut fast_book_set,
    mut book: *mut fast_book,
) -> u64_0 {
    return (*set).books_mask[((*book).num / 64 as libc::c_int) as usize]
        & ((1 as libc::c_int) << (*book).num % 64 as libc::c_int) as libc::c_ulong;
}
static mut stop: sig_atomic_t = 0;
unsafe extern "C" fn signal_handler(mut signum: libc::c_int) {
    if signum == 2 as libc::c_int {
        stop = 1 as libc::c_int;
    }
}
unsafe extern "C" fn fast_books_print(mut set: *mut fast_book_set) {
    let mut book: *mut fast_book = 0 as *mut fast_book;
    let mut level: *mut ob_level = 0 as *mut ob_level;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut mask: u64_0 = 0 as libc::c_int as u64_0;
    let mut list: *mut GList = 0 as *mut GList;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*set).books_num {
        book = ((*set).books).as_mut_ptr().offset(i as isize);
        mask |= book_has_mask(set, book);
        i += 1;
        i;
    }
    if mask == 0 {
        return;
    }
    wclear(stdscr);
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*set).books_num {
        book = ((*set).books).as_mut_ptr().offset(i as isize);
        col = 15 as libc::c_int * i;
        row = 0 as libc::c_int;
        let fresh0 = row;
        row = row + 1;
        mvprintw(
            fresh0,
            col,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            ((*book).symbol).as_mut_ptr(),
        );
        wattr_on(
            stdscr,
            (1 as libc::c_int as chtype) << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        list = g_list_last((*book).ob.glasks);
        while !list.is_null() {
            level = g_list_nth_data(list, 0 as libc::c_int as guint) as *mut ob_level;
            let fresh1 = row;
            row = row + 1;
            mvprintw(
                fresh1,
                col,
                b"%6lu %6lu\n\0" as *const u8 as *const libc::c_char,
                (*level).price,
                (*level).size,
            );
            list = if !list.is_null() { (*list).prev } else { 0 as *mut GList };
        }
        wattr_off(
            stdscr,
            (1 as libc::c_int as chtype) << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        wattr_on(
            stdscr,
            (2 as libc::c_int as chtype) << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        list = g_list_last((*book).ob.glbids);
        while !list.is_null() {
            level = g_list_nth_data(list, 0 as libc::c_int as guint) as *mut ob_level;
            let fresh2 = row;
            row = row + 1;
            mvprintw(
                fresh2,
                col,
                b"%6lu %6lu\n\0" as *const u8 as *const libc::c_char,
                (*level).price,
                (*level).size,
            );
            list = if !list.is_null() { (*list).prev } else { 0 as *mut GList };
        }
        wattr_off(
            stdscr,
            (2 as libc::c_int as chtype) << 0 as libc::c_int + 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint)
                    << 0 as libc::c_int + 8 as libc::c_int,
            0 as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    wrefresh(stdscr);
}
unsafe extern "C" fn parse_feeds(
    mut node: xmlNodePtr,
    mut set: *mut fast_book_set,
    mut template: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut feed: *mut fast_feed = 0 as *mut fast_feed;
    let mut ptr: xmlNodePtr = 0 as *mut xmlNode;
    let mut prop: *mut xmlChar = 0 as *mut xmlChar;
    node = (*node).children;
    loop {
        if node.is_null() {
            current_block = 572715077006366937;
            break;
        }
        if (*node).type_0 as libc::c_uint
            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        {
            node = (*node).next;
        } else {
            if xmlStrcmp(
                (*node).name,
                b"feed\0" as *const u8 as *const libc::c_char as *const xmlChar,
            ) != 0
            {
                current_block = 8397882087117207020;
                break;
            }
            prop = xmlGetProp(
                node as *const xmlNode,
                b"type\0" as *const u8 as *const libc::c_char as *const xmlChar,
            );
            if prop.is_null() {
                current_block = 8397882087117207020;
                break;
            }
            if xmlStrcmp(
                prop,
                b"increment\0" as *const u8 as *const libc::c_char as *const xmlChar,
            ) == 0
            {
                feed = inc_feed_add(set);
            } else if xmlStrcmp(
                prop,
                b"snapshot\0" as *const u8 as *const libc::c_char as *const xmlChar,
            ) == 0
            {
                feed = snp_feed_add(set);
            } else {
                feed = 0 as *mut fast_feed;
            }
            if feed.is_null() {
                fprintf(
                    stderr,
                    b"Cannot add a feed\n\0" as *const u8 as *const libc::c_char,
                );
                current_block = 8397882087117207020;
                break;
            } else {
                strncpy(
                    ((*feed).xml).as_mut_ptr(),
                    template,
                    ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                );
                ptr = (*node).children;
                while !ptr.is_null() {
                    if (*ptr).type_0 as libc::c_uint
                        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        ptr = (*ptr).next;
                    } else {
                        prop = xmlGetProp(
                            ptr as *const xmlNode,
                            b"value\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        );
                        if xmlStrcmp(
                            (*ptr).name,
                            b"port\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            (*feed).port = atoi(prop as *const libc::c_char);
                        } else if xmlStrcmp(
                            (*ptr).name,
                            b"ip\0" as *const u8 as *const libc::c_char as *const xmlChar,
                        ) == 0
                        {
                            strncpy(
                                ((*feed).ip).as_mut_ptr(),
                                prop as *const libc::c_char,
                                ::core::mem::size_of::<[libc::c_char; 32]>()
                                    as libc::c_ulong,
                            );
                        } else if xmlStrcmp(
                            (*ptr).name,
                            b"lip\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            strncpy(
                                ((*feed).lip).as_mut_ptr(),
                                prop as *const libc::c_char,
                                ::core::mem::size_of::<[libc::c_char; 32]>()
                                    as libc::c_ulong,
                            );
                        } else if xmlStrcmp(
                            (*ptr).name,
                            b"sip\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            strncpy(
                                ((*feed).sip).as_mut_ptr(),
                                prop as *const libc::c_char,
                                ::core::mem::size_of::<[libc::c_char; 32]>()
                                    as libc::c_ulong,
                            );
                        } else if xmlStrcmp(
                            (*ptr).name,
                            b"file\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            strncpy(
                                ((*feed).file).as_mut_ptr(),
                                prop as *const libc::c_char,
                                ::core::mem::size_of::<[libc::c_char; 64]>()
                                    as libc::c_ulong,
                            );
                        } else if xmlStrcmp(
                            (*ptr).name,
                            b"reset\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            (*feed).cfg.reset = 1 as libc::c_int != 0;
                        } else if xmlStrcmp(
                            (*ptr).name,
                            b"preamble\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            (*feed)
                                .cfg
                                .preamble_bytes = atoi(prop as *const libc::c_char);
                        }
                        ptr = (*ptr).next;
                    }
                }
                node = (*node).next;
            }
        }
    }
    match current_block {
        8397882087117207020 => return -(1 as libc::c_int),
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn parse_books(
    mut node: xmlNodePtr,
    mut set: *mut fast_book_set,
) -> libc::c_int {
    let mut current_block: u64;
    let mut book: *mut fast_book = 0 as *mut fast_book;
    let mut ptr: xmlNodePtr = 0 as *mut xmlNode;
    let mut prop: *mut xmlChar = 0 as *mut xmlChar;
    node = (*node).children;
    's_10: loop {
        if node.is_null() {
            current_block = 15768484401365413375;
            break;
        }
        if (*node).type_0 as libc::c_uint
            != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
        {
            node = (*node).next;
        } else {
            if xmlStrcmp(
                (*node).name,
                b"book\0" as *const u8 as *const libc::c_char as *const xmlChar,
            ) != 0
            {
                current_block = 11502022065142981707;
                break;
            }
            book = fast_book_add(set);
            if book.is_null() {
                fprintf(
                    stderr,
                    b"Cannot add a book\n\0" as *const u8 as *const libc::c_char,
                );
                current_block = 11502022065142981707;
                break;
            } else {
                prop = xmlGetProp(
                    node as *const xmlNode,
                    b"symbol\0" as *const u8 as *const libc::c_char as *const xmlChar,
                );
                if prop.is_null() {
                    current_block = 11502022065142981707;
                    break;
                }
                strncpy(
                    ((*book).symbol).as_mut_ptr(),
                    prop as *const libc::c_char,
                    ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                );
                ptr = (*node).children;
                while !ptr.is_null() {
                    if (*ptr).type_0 as libc::c_uint
                        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        ptr = (*ptr).next;
                    } else {
                        prop = xmlGetProp(
                            ptr as *const xmlNode,
                            b"value\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        );
                        if prop.is_null() {
                            current_block = 11502022065142981707;
                            break 's_10;
                        }
                        if xmlStrcmp(
                            (*ptr).name,
                            b"id\0" as *const u8 as *const libc::c_char as *const xmlChar,
                        ) == 0
                        {
                            (*book).secid = atol(prop as *const libc::c_char) as u64_0;
                        } else if xmlStrcmp(
                            (*ptr).name,
                            b"tick_mnt\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            (*book)
                                .tick
                                .mnt = atoi(prop as *const libc::c_char) as i64_0;
                        } else if xmlStrcmp(
                            (*ptr).name,
                            b"tick_exp\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            (*book)
                                .tick
                                .exp = atoi(prop as *const libc::c_char) as i64_0;
                        } else if xmlStrcmp(
                            (*ptr).name,
                            b"session\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            strncpy(
                                ((*book).session).as_mut_ptr(),
                                prop as *const libc::c_char,
                                ::core::mem::size_of::<[libc::c_char; 32]>()
                                    as libc::c_ulong,
                            );
                        }
                        ptr = (*ptr).next;
                    }
                }
                node = (*node).next;
            }
        }
    }
    match current_block {
        11502022065142981707 => return -(1 as libc::c_int),
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn parse_config(
    mut set: *mut fast_book_set,
    mut config: *const libc::c_char,
    mut template: *const libc::c_char,
) -> libc::c_int {
    let mut node: xmlNodePtr = 0 as *mut xmlNode;
    let mut doc: xmlDocPtr = 0 as *mut xmlDoc;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    doc = xmlParseFile(config);
    if !doc.is_null() {
        node = xmlDocGetRootElement(doc as *const xmlDoc);
        if !node.is_null() {
            if !(xmlStrcmp(
                (*node).name,
                b"config\0" as *const u8 as *const libc::c_char as *const xmlChar,
            ) != 0)
            {
                node = (*node).children;
                while !node.is_null() {
                    if (*node).type_0 as libc::c_uint
                        != XML_ELEMENT_NODE as libc::c_int as libc::c_uint
                    {
                        node = (*node).next;
                    } else {
                        if xmlStrcmp(
                            (*node).name,
                            b"feeds\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            ret = parse_feeds(node, set, template);
                        } else if xmlStrcmp(
                            (*node).name,
                            b"books\0" as *const u8 as *const libc::c_char
                                as *const xmlChar,
                        ) == 0
                        {
                            ret = parse_books(node, set);
                        }
                        if ret != 0 {
                            break;
                        }
                        node = (*node).next;
                    }
                }
            }
            xmlFreeDoc(doc);
        }
    }
    return ret;
}
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"\n usage: orderbook -t, --template template -c, --config config\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut book_set: *mut fast_book_set = 0 as *mut fast_book_set;
    let mut template: *const libc::c_char = 0 as *const libc::c_char;
    let mut config: *const libc::c_char = 0 as *const libc::c_char;
    let mut book: *mut fast_book = 0 as *mut fast_book;
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_9 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut opt_index: libc::c_int = 0 as libc::c_int;
    let mut opt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut short_opt: *const libc::c_char = b"t:c:\0" as *const u8
        as *const libc::c_char;
    let long_opt: [option; 3] = [
        {
            let mut init = option {
                name: b"template\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"config\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    loop {
        opt = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            short_opt,
            long_opt.as_ptr(),
            &mut opt_index,
        );
        if !(opt != -(1 as libc::c_int)) {
            current_block = 2868539653012386629;
            break;
        }
        match opt {
            116 => {
                template = optarg;
            }
            99 => {
                config = optarg;
            }
            _ => {
                usage();
                current_block = 7588806087723804427;
                break;
            }
        }
    }
    match current_block {
        2868539653012386629 => {
            if config.is_null() || template.is_null() {
                usage();
            } else {
                sa
                    .__sigaction_handler
                    .sa_handler = Some(
                    signal_handler as unsafe extern "C" fn(libc::c_int) -> (),
                );
                sigemptyset(&mut sa.sa_mask);
                sa.sa_flags = 0 as libc::c_int;
                if sigaction(2 as libc::c_int, &mut sa, 0 as *mut sigaction)
                    == -(1 as libc::c_int)
                {
                    fprintf(
                        stderr,
                        b"Unable to register a signal handler\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    book_set = calloc(
                        1 as libc::c_int as libc::c_ulong,
                        ::core::mem::size_of::<fast_book_set>() as libc::c_ulong,
                    ) as *mut fast_book_set;
                    if book_set.is_null() {
                        fprintf(
                            stderr,
                            b"Unable to allocate book set memory\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if !(parse_config(book_set, config, template) != 0) {
                        if fast_books_init(book_set) != 0 {
                            fprintf(
                                stderr,
                                b"Cannot initialize a book set\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            i = 0 as libc::c_int;
                            loop {
                                if !((i as libc::c_ulong) < (*book_set).books_num) {
                                    current_block = 14818589718467733107;
                                    break;
                                }
                                book = ((*book_set).books).as_mut_ptr().offset(i as isize);
                                if fast_books_subscribe(book_set, book) != 0 {
                                    fprintf(
                                        stderr,
                                        b"Cannot subscribe to the book %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        ((*book).symbol).as_mut_ptr(),
                                    );
                                    current_block = 7588806087723804427;
                                    break;
                                } else {
                                    i += 1;
                                    i;
                                }
                            }
                            match current_block {
                                7588806087723804427 => {}
                                _ => {
                                    initscr();
                                    cbreak();
                                    noecho();
                                    start_color();
                                    init_pair(
                                        1 as libc::c_int as libc::c_short,
                                        7 as libc::c_int as libc::c_short,
                                        1 as libc::c_int as libc::c_short,
                                    );
                                    init_pair(
                                        2 as libc::c_int as libc::c_short,
                                        7 as libc::c_int as libc::c_short,
                                        2 as libc::c_int as libc::c_short,
                                    );
                                    loop {
                                        if !(stop == 0) {
                                            current_block = 7333393191927787629;
                                            break;
                                        }
                                        if fast_books_update(book_set) != 0 {
                                            fprintf(
                                                stderr,
                                                b"Books update failed\n\0" as *const u8
                                                    as *const libc::c_char,
                                            );
                                            current_block = 7588806087723804427;
                                            break;
                                        } else {
                                            fast_books_print(book_set);
                                        }
                                    }
                                    match current_block {
                                        7588806087723804427 => {}
                                        _ => {
                                            if fast_books_fini(book_set) != 0 {
                                                fprintf(
                                                    stderr,
                                                    b"Books are not finalized\n\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                endwin();
                                                free(book_set as *mut libc::c_void);
                                                return 0 as libc::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    free(book_set as *mut libc::c_void);
    return 1 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
