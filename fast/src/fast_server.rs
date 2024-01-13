use ::libc;
extern "C" {
    pub type _GHashTable;
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn fast_session_send(
        self_0: *mut fast_session,
        msg: *mut fast_message,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn fast_session_recv(
        self_0: *mut fast_session,
        flags: libc::c_int,
    ) -> *mut fast_message;
    fn fast_parse_template(
        self_0: *mut fast_session,
        xml: *const libc::c_char,
    ) -> libc::c_int;
    fn fast_session_new(cfg: *mut fast_session_cfg) -> *mut fast_session;
    fn fast_session_free(self_0: *mut fast_session);
    fn do_die(format: *const libc::c_char, _: ...) -> !;
    fn error(format: *const libc::c_char, _: ...) -> !;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fcontainer_init(self_0: *mut fcontainer, init_msg: *mut fast_message);
    fn fcontainer_free(container: *mut fcontainer);
    fn next_elem(self_0: *mut fcontainer) -> *mut felem;
    fn cur_elem(self_0: *mut fcontainer) -> *mut felem;
    fn fcontainer_new() -> *mut fcontainer;
    fn script_read(stream: *mut FILE, self_0: *mut fcontainer) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type i64_0 = int64_t;
pub type u64_0 = uint64_t;
pub type size_t = libc::c_ulong;
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
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_2 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_2 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_2 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_2 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_2 = 65536;
pub const MSG_MORE: C2RustUnnamed_2 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_2 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_2 = 8192;
pub const MSG_RST: C2RustUnnamed_2 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_2 = 2048;
pub const MSG_SYN: C2RustUnnamed_2 = 1024;
pub const MSG_FIN: C2RustUnnamed_2 = 512;
pub const MSG_WAITALL: C2RustUnnamed_2 = 256;
pub const MSG_EOR: C2RustUnnamed_2 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_2 = 64;
pub const MSG_TRUNC: C2RustUnnamed_2 = 32;
pub const MSG_PROXY: C2RustUnnamed_2 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_2 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_2 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_2 = 4;
pub const MSG_PEEK: C2RustUnnamed_2 = 2;
pub const MSG_OOB: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_3 = 2;
pub const SHUT_WR: C2RustUnnamed_3 = 1;
pub const SHUT_RD: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_session_cfg {
    pub preamble_bytes: libc::c_int,
    pub sockfd: libc::c_int,
    pub reset: bool,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_5 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_5 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_5 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_5 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_5 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_5 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_5 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_5 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_5 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_5 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_5 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_5 = 92;
pub const IPPROTO_AH: C2RustUnnamed_5 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_5 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_5 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_5 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_5 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_5 = 33;
pub const IPPROTO_TP: C2RustUnnamed_5 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_5 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_5 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_5 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_5 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_5 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_5 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_5 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_5 = 1;
pub const IPPROTO_IP: C2RustUnnamed_5 = 0;
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
pub type fast_server_mode = libc::c_uint;
pub const FAST_SERVER_PONG: fast_server_mode = 1;
pub const FAST_SERVER_SCRIPT: fast_server_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_server_arg {
    pub script: *const libc::c_char,
    pub xml: *const libc::c_char,
    pub pongs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fast_server_function {
    pub fast_session_accept: Option::<
        unsafe extern "C" fn(*mut fast_session_cfg, *mut fast_server_arg) -> libc::c_int,
    >,
    pub mode: fast_server_mode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fcontainer {
    pub nr: libc::c_ulong,
    pub cur: libc::c_ulong,
    pub felems: [felem; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct felem {
    pub msg: fast_message,
    pub buf: [libc::c_char; 4096],
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
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
static mut program: *const libc::c_char = 0 as *const libc::c_char;
static mut fast_server_functions: [fast_server_function; 2] = unsafe {
    [
        {
            let mut init = fast_server_function {
                fast_session_accept: Some(
                    fast_server_script
                        as unsafe extern "C" fn(
                            *mut fast_session_cfg,
                            *mut fast_server_arg,
                        ) -> libc::c_int,
                ),
                mode: FAST_SERVER_SCRIPT,
            };
            init
        },
        {
            let mut init = fast_server_function {
                fast_session_accept: Some(
                    fast_server_pong
                        as unsafe extern "C" fn(
                            *mut fast_session_cfg,
                            *mut fast_server_arg,
                        ) -> libc::c_int,
                ),
                mode: FAST_SERVER_PONG,
            };
            init
        },
    ]
};
unsafe extern "C" fn fast_send_prepare(
    mut msg: *mut fast_message,
    mut elem: *mut felem,
) {
    let mut elem_msg: *mut fast_message = 0 as *mut fast_message;
    let mut elem_field: *mut fast_field = 0 as *mut fast_field;
    let mut field: *mut fast_field = 0 as *mut fast_field;
    let mut i: libc::c_int = 0;
    if !(elem.is_null() || msg.is_null()) {
        elem_msg = &mut (*elem).msg;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*msg).nr_fields {
            elem_field = ((*elem_msg).fields).offset(i as isize);
            field = ((*msg).fields).offset(i as isize);
            (*field).state = (*elem_field).state;
            match (*elem_field).type_0 as libc::c_uint {
                0 => {
                    (*field)
                        .c2rust_unnamed
                        .int_value = (*elem_field).c2rust_unnamed.int_value;
                }
                1 => {
                    (*field)
                        .c2rust_unnamed
                        .uint_value = (*elem_field).c2rust_unnamed.uint_value;
                }
                2 => {
                    memcpy(
                        ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                            as *mut libc::c_void,
                        ((*elem_field).c2rust_unnamed.string_value).as_mut_ptr()
                            as *const libc::c_void,
                        (strlen(
                            ((*elem_field).c2rust_unnamed.string_value).as_mut_ptr(),
                        ))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    );
                }
                4 => {
                    (*field)
                        .c2rust_unnamed
                        .decimal_value
                        .exp = (*elem_field).c2rust_unnamed.decimal_value.exp;
                    (*field)
                        .c2rust_unnamed
                        .decimal_value
                        .mnt = (*elem_field).c2rust_unnamed.decimal_value.mnt;
                }
                3 | 5 | _ => {}
            }
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn fast_server_script(
    mut cfg: *mut fast_session_cfg,
    mut arg: *mut fast_server_arg,
) -> libc::c_int {
    let mut current_block: u64;
    let mut container: *mut fcontainer = 0 as *mut fcontainer;
    let mut session: *mut fast_session = 0 as *mut fast_session;
    let mut expected_elem: *mut felem = 0 as *mut felem;
    let mut msg: *mut fast_message = 0 as *mut fast_message;
    let mut stream: *mut FILE = 0 as *mut FILE;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    if ((*arg).script).is_null() {
        fprintf(
            stderr,
            b"No script is specified\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        stream = fopen((*arg).script, b"r\0" as *const u8 as *const libc::c_char);
        if stream.is_null() {
            fprintf(
                stderr,
                b"Opening %s failed: %s\n\0" as *const u8 as *const libc::c_char,
                (*arg).script,
                strerror(*__errno_location()),
            );
        } else {
            session = fast_session_new(cfg);
            if session.is_null() {
                fprintf(
                    stderr,
                    b"FAST session cannot be created\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                container = fcontainer_new();
                if container.is_null() {
                    fprintf(
                        stderr,
                        b"Cannot allocate container\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if fast_parse_template(session, (*arg).xml) != 0 {
                    fprintf(
                        stderr,
                        b"Cannot read template xml file\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    fcontainer_init(container, (*session).rx_messages);
                    if script_read(stream, container) != 0 {
                        fprintf(
                            stderr,
                            b"Invalid script: %s\n\0" as *const u8
                                as *const libc::c_char,
                            (*arg).script,
                        );
                    } else {
                        expected_elem = cur_elem(container);
                        msg = &mut (*expected_elem).msg;
                        expected_elem = next_elem(container);
                        loop {
                            if expected_elem.is_null() {
                                current_block = 11042950489265723346;
                                break;
                            }
                            fast_send_prepare(msg, expected_elem);
                            if fast_session_send(session, msg, 0 as libc::c_int) != 0 {
                                current_block = 14048086041423495086;
                                break;
                            }
                            expected_elem = next_elem(container);
                        }
                        match current_block {
                            14048086041423495086 => {}
                            _ => {
                                ret = 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    fcontainer_free(container);
    fast_session_free(session);
    fclose(stream);
    return ret;
}
unsafe extern "C" fn fast_pong_prepare(mut tx_msg: *mut fast_message) -> libc::c_int {
    let mut tx_field: *mut fast_field = 0 as *mut fast_field;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    if !tx_msg.is_null() {
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*tx_msg).nr_fields {
            tx_field = ((*tx_msg).fields).offset(i as isize);
            (*tx_field).state = FAST_STATE_ASSIGNED;
            if !((*tx_field).op as libc::c_uint
                == FAST_OP_CONSTANT as libc::c_int as libc::c_uint)
            {
                match (*tx_field).type_0 as libc::c_uint {
                    2 => {
                        strcpy(
                            ((*tx_field).c2rust_unnamed.string_value).as_mut_ptr(),
                            b"Forty two\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    4 => {
                        (*tx_field)
                            .c2rust_unnamed
                            .decimal_value
                            .mnt = -(42 as libc::c_int) as i64_0;
                        (*tx_field)
                            .c2rust_unnamed
                            .decimal_value
                            .exp = 42 as libc::c_int as i64_0;
                    }
                    1 => {
                        (*tx_field)
                            .c2rust_unnamed
                            .uint_value = 42 as libc::c_int as u64_0;
                    }
                    0 => {
                        (*tx_field)
                            .c2rust_unnamed
                            .int_value = -(42 as libc::c_int) as i64_0;
                    }
                    3 | 5 | _ => {}
                }
            }
            i += 1;
            i;
        }
        ret = 0 as libc::c_int;
    }
    return ret;
}
unsafe extern "C" fn fast_server_pong(
    mut cfg: *mut fast_session_cfg,
    mut arg: *mut fast_server_arg,
) -> libc::c_int {
    let mut current_block: u64;
    let mut session: *mut fast_session = 0 as *mut fast_session;
    let mut tx_msg: *mut fast_message = 0 as *mut fast_message;
    let mut rx_msg: *mut fast_message = 0 as *mut fast_message;
    let mut aux: *mut fast_session = 0 as *mut fast_session;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    session = fast_session_new(cfg);
    if session.is_null() {
        fprintf(
            stderr,
            b"FAST session cannot be created\n\0" as *const u8 as *const libc::c_char,
        );
    } else if fast_parse_template(session, (*arg).xml) != 0 {
        fprintf(
            stderr,
            b"Cannot read template xml file\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        aux = fast_session_new(cfg);
        if aux.is_null() {
            fprintf(
                stderr,
                b"FAST session cannot be created\n\0" as *const u8 as *const libc::c_char,
            );
        } else if fast_parse_template(aux, (*arg).xml) != 0 {
            fprintf(
                stderr,
                b"Cannot read template xml file\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            rx_msg = (*session).rx_messages;
            if rx_msg.is_null() {
                fprintf(
                    stderr,
                    b"Message cannot be found\n\0" as *const u8 as *const libc::c_char,
                );
            } else {
                tx_msg = (*aux).rx_messages;
                if tx_msg.is_null() {
                    fprintf(
                        stderr,
                        b"Message cannot be found\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if fast_pong_prepare(tx_msg) != 0 {
                    fprintf(
                        stderr,
                        b"Cannot initialize tx_msg\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < (*arg).pongs) {
                            current_block = 10043043949733653460;
                            break;
                        }
                        loop {
                            rx_msg = fast_session_recv(
                                session,
                                MSG_DONTWAIT as libc::c_int,
                            );
                            if !rx_msg.is_null() {
                                break;
                            }
                        }
                        if fast_session_send(session, tx_msg, 0 as libc::c_int) != 0 {
                            current_block = 14039544961630247767;
                            break;
                        }
                        i += 1;
                        i;
                    }
                    match current_block {
                        14039544961630247767 => {}
                        _ => {
                            ret = 0 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    fast_session_free(session);
    fast_session_free(aux);
    return ret;
}
unsafe extern "C" fn usage() {
    printf(
        b"\n usage: %s [-m mode] [-f filename] [-n pongs] -p port -t template\n\n\0"
            as *const u8 as *const libc::c_char,
        program,
    );
    exit(1 as libc::c_int);
}
unsafe extern "C" fn socket_setopt(
    mut sockfd: libc::c_int,
    mut level: libc::c_int,
    mut optname: libc::c_int,
    mut optval: libc::c_int,
) -> libc::c_int {
    return setsockopt(
        sockfd,
        level,
        optname,
        &mut optval as *mut libc::c_int as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
}
unsafe extern "C" fn strservermode(mut mode: *const libc::c_char) -> fast_server_mode {
    let mut m: fast_server_mode = FAST_SERVER_SCRIPT;
    if strcmp(mode, b"script\0" as *const u8 as *const libc::c_char) == 0 {
        return FAST_SERVER_SCRIPT
    } else if strcmp(mode, b"pong\0" as *const u8 as *const libc::c_char) == 0 {
        return FAST_SERVER_PONG
    }
    if sscanf(
        mode,
        b"%u\0" as *const u8 as *const libc::c_char,
        &mut m as *mut fast_server_mode,
    ) != 1 as libc::c_int
    {
        return FAST_SERVER_SCRIPT;
    }
    match m as libc::c_uint {
        0 | 1 => return m,
        _ => {}
    }
    return FAST_SERVER_SCRIPT;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut mode: fast_server_mode = FAST_SERVER_SCRIPT;
    let mut arg: fast_server_arg = {
        let mut init = fast_server_arg {
            script: 0 as *const libc::c_char,
            xml: 0 as *const libc::c_char,
            pongs: 0,
        };
        init
    };
    let mut cfg: fast_session_cfg = fast_session_cfg {
        preamble_bytes: 0,
        sockfd: 0,
        reset: false,
    };
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut sockfd: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    program = __xpg_basename(*argv.offset(0 as libc::c_int as isize));
    loop {
        opt = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"p:f:t:m:n:\0" as *const u8 as *const libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            109 => {
                mode = strservermode(optarg);
            }
            110 => {
                arg.pongs = atoi(optarg);
            }
            112 => {
                port = atoi(optarg);
            }
            102 => {
                arg.script = optarg;
            }
            116 => {
                arg.xml = optarg;
            }
            _ => {
                usage();
            }
        }
    }
    if port == 0 || (arg.xml).is_null() {
        usage();
    }
    sockfd = socket(
        2 as libc::c_int,
        SOCK_STREAM as libc::c_int,
        IPPROTO_TCP as libc::c_int,
    );
    if sockfd < 0 as libc::c_int {
        do_die(
            b"%s: cannot create socket\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
        );
    }
    if socket_setopt(
        sockfd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) < 0 as libc::c_int
    {
        do_die(
            b"%s: cannot set socket option TCP_NODELAY\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
        );
    }
    if socket_setopt(sockfd, 1 as libc::c_int, 2 as libc::c_int, 1 as libc::c_int)
        < 0 as libc::c_int
    {
        do_die(
            b"%s: cannot set socket option SO_REUSEADDR\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
        );
    }
    sa = {
        let mut init = sockaddr_in {
            sin_family: 2 as libc::c_int as sa_family_t,
            sin_port: __bswap_16(port as __uint16_t),
            sin_addr: {
                let mut init = in_addr {
                    s_addr: 0 as libc::c_int as in_addr_t,
                };
                init
            },
            sin_zero: [0; 8],
        };
        init
    };
    if bind(
        sockfd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut sa as *mut sockaddr_in as *const sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        do_die(
            b"%s: bind failed\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
        );
    }
    fprintf(
        stderr,
        b"FAST server is listening to port %d...\n\0" as *const u8
            as *const libc::c_char,
        port,
    );
    if listen(sockfd, 10 as libc::c_int) < 0 as libc::c_int {
        do_die(
            b"%s: listen failed\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
        );
    }
    cfg
        .sockfd = accept(
        sockfd,
        __SOCKADDR_ARG {
            __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
        },
        0 as *mut socklen_t,
    );
    if cfg.sockfd < 0 as libc::c_int {
        do_die(
            b"%s: accept failed\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
        );
    }
    if socket_setopt(
        cfg.sockfd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ) < 0 as libc::c_int
    {
        do_die(
            b"%s: cannot set socket option TCP_NODELAY\0" as *const u8
                as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
        );
    }
    cfg.preamble_bytes = 0 as libc::c_int;
    cfg.reset = 0 as libc::c_int != 0;
    match mode as libc::c_uint {
        0 | 1 => {
            ret = (fast_server_functions[mode as usize].fast_session_accept)
                .expect("non-null function pointer")(&mut cfg, &mut arg);
        }
        _ => {
            error(b"Invalid mode\0" as *const u8 as *const libc::c_char);
        }
    }
    shutdown(cfg.sockfd, SHUT_RDWR as libc::c_int);
    close(cfg.sockfd);
    close(sockfd);
    return ret;
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
