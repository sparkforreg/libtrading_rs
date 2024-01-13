use ::libc;


extern "C" {

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
    fn fix_message_type_is(self_0: *mut fix_message, type_0: fix_msg_type) -> bool;
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fix_session_admin(session: *mut fix_session, msg: *mut fix_message) -> bool;
    fn fix_session_execution_report(
        session: *mut fix_session,
        fields: *mut fix_field,
        nr_fields: libc::c_long,
    ) -> libc::c_int;
    fn fix_session_recv(
        self_0: *mut fix_session,
        msg: *mut *mut fix_message,
        flags: libc::c_ulong,
    ) -> libc::c_int;
    fn fix_session_send(
        self_0: *mut fix_session,
        msg: *mut fix_message,
        flags: libc::c_ulong,
    ) -> libc::c_int;
    fn fix_session_free(self_0: *mut fix_session);
    fn fix_session_new(cfg: *mut fix_session_cfg) -> *mut fix_session;
    fn fix_session_cfg_init(cfg: *mut fix_session_cfg);
    static mut fix_dialects: [fix_dialect; 0];
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn do_die(format: *const libc::c_char, _: ...) -> !;
    fn error(format: *const libc::c_char, _: ...) -> !;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn fcontainer_free(container: *mut fcontainer);
    fn next_elem(self_0: *mut fcontainer) -> *mut felem;
    fn cur_elem(self_0: *mut fcontainer) -> *mut felem;
    fn fcontainer_new() -> *mut fcontainer;
    fn script_read(
        stream: *mut FILE,
        server: *mut fcontainer,
        client: *mut fcontainer,
    ) -> libc::c_int;
    fn fmsgcmp(expected: *mut fix_message, actual: *mut fix_message) -> libc::c_int;
    fn fprintmsg_iov(stream: *mut FILE, msg: *mut fix_message);
    fn fprintmsg(stream: *mut FILE, msg: *mut fix_message);
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct fix_dialect {
    pub version: fix_version,
    pub tag_type: Option::<unsafe extern "C" fn(libc::c_int) -> fix_type>,
}
pub type fix_type = libc::c_uint;
pub const FIX_TYPE_STRING_8: fix_type = 6;
pub const FIX_TYPE_MSGSEQNUM: fix_type = 5;
pub const FIX_TYPE_CHECKSUM: fix_type = 4;
pub const FIX_TYPE_STRING: fix_type = 3;
pub const FIX_TYPE_CHAR: fix_type = 2;
pub const FIX_TYPE_FLOAT: fix_type = 1;
pub const FIX_TYPE_INT: fix_type = 0;
pub type fix_version = libc::c_uint;
pub const FIXT_1_1: fix_version = 6;
pub const FIX_5_0: fix_version = 5;
pub const FIX_4_4: fix_version = 4;
pub const FIX_4_3: fix_version = 3;
pub const FIX_4_2: fix_version = 2;
pub const FIX_4_1: fix_version = 1;
pub const FIX_4_0: fix_version = 0;
pub type fix_msg_type = libc::c_ulong;
pub const FIX_MSG_TYPE_UNKNOWN: fix_msg_type = 18446744073709551615;
pub const FIX_MSG_TYPE_MAX: fix_msg_type = 28;
pub const FIX_MSG_ORDER_MASS_ACTION_REPORT: fix_msg_type = 27;
pub const FIX_MSG_ORDER_MASS_ACTION_REQUEST: fix_msg_type = 26;
pub const FIX_MSG_ORDER_MASS_STATUS_REQUEST: fix_msg_type = 25;
pub const FIX_MSG_QUOTE_ACKNOWLEDGEMENT: fix_msg_type = 24;
pub const FIX_MSG_SECURITY_DEFINITION: fix_msg_type = 23;
pub const FIX_MSG_QUOTE_CANCEL: fix_msg_type = 22;
pub const FIX_MSG_MASS_QUOTE: fix_msg_type = 21;
pub const FIX_MSG_NEW_ORDER_CROSS: fix_msg_type = 20;
pub const FIX_MSG_SECURITY_DEFINITION_REQUEST: fix_msg_type = 19;
pub const FIX_MSG_QUOTE_REQUEST: fix_msg_type = 18;
pub const FIX_MSG_ORDER_MASS_CANCEL_REPORT: fix_msg_type = 17;
pub const FIX_MSG_ORDER_MASS_CANCEL_REQUEST: fix_msg_type = 16;
pub const FIX_MSG_ORDER_CANCEL_REQUEST: fix_msg_type = 15;
pub const FIX_MSG_ORDER_CANCEL_REJECT: fix_msg_type = 14;
pub const FIX_MSG_ORDER_CANCEL_REPLACE: fix_msg_type = 13;
pub const FIX_MSG_TYPE_SECURITY_STATUS: fix_msg_type = 12;
pub const FIX_MSG_TYPE_SESSION_STATUS: fix_msg_type = 11;
pub const FIX_MSG_TYPE_INCREMENT_REFRESH: fix_msg_type = 10;
pub const FIX_MSG_TYPE_SNAPSHOT_REFRESH: fix_msg_type = 9;
pub const FIX_MSG_TYPE_NEW_ORDER_SINGLE: fix_msg_type = 8;
pub const FIX_MSG_TYPE_LOGON: fix_msg_type = 7;
pub const FIX_MSG_TYPE_EXECUTION_REPORT: fix_msg_type = 6;
pub const FIX_MSG_TYPE_LOGOUT: fix_msg_type = 5;
pub const FIX_MSG_TYPE_SEQUENCE_RESET: fix_msg_type = 4;
pub const FIX_MSG_TYPE_REJECT: fix_msg_type = 3;
pub const FIX_MSG_TYPE_RESEND_REQUEST: fix_msg_type = 2;
pub const FIX_MSG_TYPE_TEST_REQUEST: fix_msg_type = 1;
pub const FIX_MSG_TYPE_HEARTBEAT: fix_msg_type = 0;
pub type fix_tag = libc::c_uint;
pub const MDPriceLevel: fix_tag = 1023;
pub const Password: fix_tag = 554;
pub const MultiLegReportingType: fix_tag = 442;
pub const LastMsgSeqNumProcessed: fix_tag = 369;
pub const TradingSessionID: fix_tag = 336;
pub const MDUpdateAction: fix_tag = 279;
pub const MDEntrySize: fix_tag = 271;
pub const MDEntryPx: fix_tag = 270;
pub const MDEntryType: fix_tag = 269;
pub const LeavesQty: fix_tag = 151;
pub const ExecType: fix_tag = 150;
pub const ResetSeqNumFlag: fix_tag = 141;
pub const GapFillFlag: fix_tag = 123;
pub const TestReqID: fix_tag = 112;
pub const HeartBtInt: fix_tag = 108;
pub const OrdRejReason: fix_tag = 103;
pub const CXlRejReason: fix_tag = 102;
pub const EncryptMethod: fix_tag = 98;
pub const RptSeq: fix_tag = 83;
pub const TransactTime: fix_tag = 60;
pub const Text: fix_tag = 58;
pub const TargetCompID: fix_tag = 56;
pub const Symbol: fix_tag = 55;
pub const Side: fix_tag = 54;
pub const SendingTime: fix_tag = 52;
pub const SenderCompID: fix_tag = 49;
pub const SecurityID: fix_tag = 48;
pub const RefSeqNum: fix_tag = 45;
pub const Price: fix_tag = 44;
pub const PossDupFlag: fix_tag = 43;
pub const OrigClOrdID: fix_tag = 41;
pub const OrdType: fix_tag = 40;
pub const OrdStatus: fix_tag = 39;
pub const OrderQty: fix_tag = 38;
pub const OrderID: fix_tag = 37;
pub const NewSeqNo: fix_tag = 36;
pub const MsgType: fix_tag = 35;
pub const MsgSeqNum: fix_tag = 34;
pub const LastShares: fix_tag = 32;
pub const LastPx: fix_tag = 31;
pub const ExecTransType: fix_tag = 20;
pub const ExecID: fix_tag = 17;
pub const EndSeqNo: fix_tag = 16;
pub const CumQty: fix_tag = 14;
pub const ClOrdID: fix_tag = 11;
pub const CheckSum: fix_tag = 10;
pub const BodyLength: fix_tag = 9;
pub const BeginString: fix_tag = 8;
pub const BeginSeqNo: fix_tag = 7;
pub const AvgPx: fix_tag = 6;
pub const Account: fix_tag = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fix_field {
    pub tag: libc::c_int,
    pub type_0: fix_type,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub int_value: int64_t,
    pub float_value: libc::c_double,
    pub char_value: libc::c_char,
    pub string_value: *const libc::c_char,
    pub string_8_value: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fix_message {
    pub type_0: fix_msg_type,
    pub begin_string: *const libc::c_char,
    pub body_length: libc::c_ulong,
    pub msg_type: *const libc::c_char,
    pub sender_comp_id: *const libc::c_char,
    pub target_comp_id: *const libc::c_char,
    pub msg_seq_num: libc::c_ulong,
    pub check_sum: *const libc::c_char,
    pub str_now: *mut libc::c_char,
    pub head_buf: *mut buffer,
    pub body_buf: *mut buffer,
    pub nr_fields: libc::c_ulong,
    pub fields: *mut fix_field,
    pub iov: [iovec; 2],
}
pub type socklen_t = __socklen_t;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_0 = 2;
pub const SHUT_WR: C2RustUnnamed_0 = 1;
pub const SHUT_RD: C2RustUnnamed_0 = 0;
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
    pub __in6_u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub struct fix_session_cfg {
    pub sender_comp_id: [libc::c_char; 32],
    pub target_comp_id: [libc::c_char; 32],
    pub password: [libc::c_char; 32],
    pub heartbtint: libc::c_int,
    pub dialect: *mut fix_dialect,
    pub sockfd: libc::c_int,
    pub in_msg_seq_num: libc::c_ulong,
    pub out_msg_seq_num: libc::c_ulong,
    pub user_data: *mut libc::c_void,
}
pub type fix_failure_reason = libc::c_uint;
pub const FIX_FAILURE_GARBLED: fix_failure_reason = 4;
pub const FIX_FAILURE_SYSTEM: fix_failure_reason = 3;
pub const FIX_FAILURE_RECV_ZERO_B: fix_failure_reason = 2;
pub const FIX_FAILURE_CONN_CLOSED: fix_failure_reason = 1;
pub const FIX_SUCCESS: fix_failure_reason = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fix_session {
    pub dialect: *mut fix_dialect,
    pub sockfd: libc::c_int,
    pub active: bool,
    pub password: *const libc::c_char,
    pub begin_string: *const libc::c_char,
    pub sender_comp_id: *const libc::c_char,
    pub target_comp_id: *const libc::c_char,
    pub in_msg_seq_num: libc::c_ulong,
    pub out_msg_seq_num: libc::c_ulong,
    pub rx_buffer: *mut buffer,
    pub tx_head_buffer: *mut buffer,
    pub tx_body_buffer: *mut buffer,
    pub rx_message: *mut fix_message,
    pub heartbtint: libc::c_int,
    pub now: timespec,
    pub str_now: [libc::c_char; 64],
    pub rx_timestamp: timespec,
    pub tx_timestamp: timespec,
    pub testreqid: [libc::c_char; 64],
    pub tr_timestamp: timespec,
    pub tr_pending: libc::c_int,
    pub failure_reason: fix_failure_reason,
    pub user_data: *mut libc::c_void,
}
pub type fix_recv_flag = libc::c_uint;
pub const FIX_RECV_KEEP_IN_MSGSEQNUM: fix_recv_flag = 131072;
pub const FIX_RECV_FLAG_MSG_DONTWAIT: fix_recv_flag = 65536;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_2 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_2 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_2 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_2 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_2 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_2 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_2 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_2 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_2 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_2 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_2 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_2 = 92;
pub const IPPROTO_AH: C2RustUnnamed_2 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_2 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_2 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_2 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_2 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_2 = 33;
pub const IPPROTO_TP: C2RustUnnamed_2 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_2 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_2 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_2 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_2 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_2 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_2 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_2 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_2 = 1;
pub const IPPROTO_IP: C2RustUnnamed_2 = 0;
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
pub type fix_server_mode = libc::c_uint;
pub const FIX_SERVER_SESSION: fix_server_mode = 1;
pub const FIX_SERVER_SCRIPT: fix_server_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fix_server_arg {
    pub script: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fix_server_function {
    pub fix_session_accept: Option::<
        unsafe extern "C" fn(*mut fix_session_cfg, *mut fix_server_arg) -> libc::c_int,
    >,
    pub mode: fix_server_mode,
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
    pub msg: fix_message,
    pub buf: [libc::c_char; 256],
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut program: *const libc::c_char = 0 as *const libc::c_char;
static mut fix_server_functions: [fix_server_function; 2] = unsafe {
    [
        {
            let mut init = fix_server_function {
                fix_session_accept: Some(
                    fix_server_script
                        as unsafe extern "C" fn(
                            *mut fix_session_cfg,
                            *mut fix_server_arg,
                        ) -> libc::c_int,
                ),
                mode: FIX_SERVER_SCRIPT,
            };
            init
        },
        {
            let mut init = fix_server_function {
                fix_session_accept: Some(
                    fix_server_session
                        as unsafe extern "C" fn(
                            *mut fix_session_cfg,
                            *mut fix_server_arg,
                        ) -> libc::c_int,
                ),
                mode: FIX_SERVER_SESSION,
            };
            init
        },
    ]
};
unsafe extern "C" fn fix_server_logon(mut session: *mut fix_session) -> bool {
    let mut logon_msg: fix_message = fix_message {
        type_0: FIX_MSG_TYPE_HEARTBEAT,
        begin_string: 0 as *const libc::c_char,
        body_length: 0,
        msg_type: 0 as *const libc::c_char,
        sender_comp_id: 0 as *const libc::c_char,
        target_comp_id: 0 as *const libc::c_char,
        msg_seq_num: 0,
        check_sum: 0 as *const libc::c_char,
        str_now: 0 as *mut libc::c_char,
        head_buf: 0 as *mut buffer,
        body_buf: 0 as *mut buffer,
        nr_fields: 0,
        fields: 0 as *mut fix_field,
        iov: [iovec {
            iov_base: 0 as *mut libc::c_void,
            iov_len: 0,
        }; 2],
    };
    let mut msg: *mut fix_message = 0 as *mut fix_message;
    while fix_session_recv(
        session,
        &mut msg,
        FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
    ) <= 0 as libc::c_int
    {}
    if !fix_message_type_is(msg, FIX_MSG_TYPE_LOGON) {
        return 0 as libc::c_int != 0;
    }
    logon_msg = {
        let mut init = fix_message {
            type_0: FIX_MSG_TYPE_LOGON,
            begin_string: 0 as *const libc::c_char,
            body_length: 0,
            msg_type: 0 as *const libc::c_char,
            sender_comp_id: 0 as *const libc::c_char,
            target_comp_id: 0 as *const libc::c_char,
            msg_seq_num: 0,
            check_sum: 0 as *const libc::c_char,
            str_now: 0 as *mut libc::c_char,
            head_buf: 0 as *mut buffer,
            body_buf: 0 as *mut buffer,
            nr_fields: 0,
            fields: 0 as *mut fix_field,
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    fix_session_send(session, &mut logon_msg, 0 as libc::c_int as libc::c_ulong);
    (*session).active = 1 as libc::c_int != 0;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn fix_server_logout(mut session: *mut fix_session) -> bool {
    let mut logout_msg: fix_message = fix_message {
        type_0: FIX_MSG_TYPE_HEARTBEAT,
        begin_string: 0 as *const libc::c_char,
        body_length: 0,
        msg_type: 0 as *const libc::c_char,
        sender_comp_id: 0 as *const libc::c_char,
        target_comp_id: 0 as *const libc::c_char,
        msg_seq_num: 0,
        check_sum: 0 as *const libc::c_char,
        str_now: 0 as *mut libc::c_char,
        head_buf: 0 as *mut buffer,
        body_buf: 0 as *mut buffer,
        nr_fields: 0,
        fields: 0 as *mut fix_field,
        iov: [iovec {
            iov_base: 0 as *mut libc::c_void,
            iov_len: 0,
        }; 2],
    };
    let mut msg: *mut fix_message = 0 as *mut fix_message;
    let mut ret: bool = 1 as libc::c_int != 0;
    while (*session).active {
        if fix_session_recv(
            session,
            &mut msg,
            FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
        ) <= 0 as libc::c_int
        {
            continue;
        }
        if !fix_message_type_is(msg, FIX_MSG_TYPE_LOGOUT) {
            ret = 0 as libc::c_int != 0;
        }
        logout_msg = {
            let mut init = fix_message {
                type_0: FIX_MSG_TYPE_LOGOUT,
                begin_string: 0 as *const libc::c_char,
                body_length: 0,
                msg_type: 0 as *const libc::c_char,
                sender_comp_id: 0 as *const libc::c_char,
                target_comp_id: 0 as *const libc::c_char,
                msg_seq_num: 0,
                check_sum: 0 as *const libc::c_char,
                str_now: 0 as *mut libc::c_char,
                head_buf: 0 as *mut buffer,
                body_buf: 0 as *mut buffer,
                nr_fields: 0,
                fields: 0 as *mut fix_field,
                iov: [iovec {
                    iov_base: 0 as *mut libc::c_void,
                    iov_len: 0,
                }; 2],
            };
            init
        };
        fix_session_send(session, &mut logout_msg, 0 as libc::c_int as libc::c_ulong);
        break;
    }
    return ret;
}
unsafe extern "C" fn fix_server_script(
    mut cfg: *mut fix_session_cfg,
    mut arg: *mut fix_server_arg,
) -> libc::c_int {
    let mut s_container: *mut fcontainer = 0 as *mut fcontainer;
    let mut c_container: *mut fcontainer = 0 as *mut fcontainer;
    let mut session: *mut fix_session = 0 as *mut fix_session;
    let mut expected_elem: *mut felem = 0 as *mut felem;
    let mut msg: *mut fix_message = 0 as *mut fix_message;
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
            session = fix_session_new(cfg);
            if session.is_null() {
                fprintf(
                    stderr,
                    b"FIX session cannot be created\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                s_container = fcontainer_new();
                if s_container.is_null() {
                    fprintf(
                        stderr,
                        b"Cannot allocate container\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    c_container = fcontainer_new();
                    if c_container.is_null() {
                        fprintf(
                            stderr,
                            b"Cannot allocate container\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else if script_read(stream, s_container, c_container) != 0 {
                        fprintf(
                            stderr,
                            b"Invalid script: %s\n\0" as *const u8
                                as *const libc::c_char,
                            (*arg).script,
                        );
                    } else if fix_server_logon(session) {
                        fprintf(
                            stdout,
                            b"Server Logon OK\n\0" as *const u8 as *const libc::c_char,
                        );
                        expected_elem = cur_elem(c_container);
                        while !expected_elem.is_null() {
                            if fix_session_recv(
                                session,
                                &mut msg,
                                FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
                            ) <= 0 as libc::c_int
                            {
                                continue;
                            }
                            fprintf(stdout, b"> \0" as *const u8 as *const libc::c_char);
                            fprintmsg_iov(stdout, msg);
                            if fmsgcmp(&mut (*expected_elem).msg, msg) != 0 {
                                fprintf(
                                    stderr,
                                    b"Server: messages differ\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                fprintmsg(stderr, &mut (*expected_elem).msg);
                                fprintmsg(stderr, msg);
                                break;
                            } else {
                                if !fix_session_admin(session, msg) {
                                    if fix_message_type_is(msg, FIX_MSG_TYPE_LOGON) {
                                        fprintf(
                                            stderr,
                                            b"Server: repeated Logon\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        break;
                                    } else if fix_message_type_is(msg, FIX_MSG_TYPE_LOGOUT) {
                                        fprintf(
                                            stderr,
                                            b"Server: premature Logout\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        break;
                                    }
                                }
                                expected_elem = next_elem(c_container);
                            }
                        }
                        if expected_elem.is_null() {
                            if fix_server_logout(session) {
                                fprintf(
                                    stdout,
                                    b"Server Logout OK\n\0" as *const u8 as *const libc::c_char,
                                );
                                ret = 0 as libc::c_int;
                            } else {
                                fprintf(
                                    stderr,
                                    b"Server Logout FAILED\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                    } else {
                        fprintf(
                            stderr,
                            b"Server Logon FAILED\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
            }
        }
    }
    fcontainer_free(c_container);
    fcontainer_free(s_container);
    fix_session_free(session);
    if !stream.is_null() {
        fclose(stream);
    }
    return ret;
}
unsafe extern "C" fn fix_execution_report_fields(
    mut fields: *mut fix_field,
) -> libc::c_ulong {
    let mut nr: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let fresh0 = nr;
    nr = nr.wrapping_add(1);
    *fields
        .offset(
            fresh0 as isize,
        ) = {
        let mut init = fix_field {
            tag: OrderID as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"OrderID\0" as *const u8 as *const libc::c_char,
            },
        };
        init
    };
    let fresh1 = nr;
    nr = nr.wrapping_add(1);
    *fields
        .offset(
            fresh1 as isize,
        ) = {
        let mut init = fix_field {
            tag: Symbol as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"Symbol\0" as *const u8 as *const libc::c_char,
            },
        };
        init
    };
    let fresh2 = nr;
    nr = nr.wrapping_add(1);
    *fields
        .offset(
            fresh2 as isize,
        ) = {
        let mut init = fix_field {
            tag: ExecID as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"ExecID\0" as *const u8 as *const libc::c_char,
            },
        };
        init
    };
    let fresh3 = nr;
    nr = nr.wrapping_add(1);
    *fields
        .offset(
            fresh3 as isize,
        ) = {
        let mut init = fix_field {
            tag: OrdStatus as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"2\0" as *const u8 as *const libc::c_char,
            },
        };
        init
    };
    let fresh4 = nr;
    nr = nr.wrapping_add(1);
    *fields
        .offset(
            fresh4 as isize,
        ) = {
        let mut init = fix_field {
            tag: ExecType as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"0\0" as *const u8 as *const libc::c_char,
            },
        };
        init
    };
    let fresh5 = nr;
    nr = nr.wrapping_add(1);
    *fields
        .offset(
            fresh5 as isize,
        ) = {
        let mut init = fix_field {
            tag: LeavesQty as libc::c_int,
            type_0: FIX_TYPE_FLOAT,
            c2rust_unnamed: C2RustUnnamed {
                float_value: 0 as libc::c_int as libc::c_double,
            },
        };
        init
    };
    let fresh6 = nr;
    nr = nr.wrapping_add(1);
    *fields
        .offset(
            fresh6 as isize,
        ) = {
        let mut init = fix_field {
            tag: CumQty as libc::c_int,
            type_0: FIX_TYPE_FLOAT,
            c2rust_unnamed: C2RustUnnamed {
                float_value: 100 as libc::c_int as libc::c_double,
            },
        };
        init
    };
    let fresh7 = nr;
    nr = nr.wrapping_add(1);
    *fields
        .offset(
            fresh7 as isize,
        ) = {
        let mut init = fix_field {
            tag: AvgPx as libc::c_int,
            type_0: FIX_TYPE_FLOAT,
            c2rust_unnamed: C2RustUnnamed {
                float_value: 100 as libc::c_int as libc::c_double,
            },
        };
        init
    };
    let fresh8 = nr;
    nr = nr.wrapping_add(1);
    *fields
        .offset(
            fresh8 as isize,
        ) = {
        let mut init = fix_field {
            tag: Side as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"1\0" as *const u8 as *const libc::c_char,
            },
        };
        init
    };
    return nr;
}
unsafe extern "C" fn fix_server_session(
    mut cfg: *mut fix_session_cfg,
    mut arg: *mut fix_server_arg,
) -> libc::c_int {
    let mut session: *mut fix_session = 0 as *mut fix_session;
    let mut fields: *mut fix_field = 0 as *mut fix_field;
    let mut logon_msg: fix_message = fix_message {
        type_0: FIX_MSG_TYPE_HEARTBEAT,
        begin_string: 0 as *const libc::c_char,
        body_length: 0,
        msg_type: 0 as *const libc::c_char,
        sender_comp_id: 0 as *const libc::c_char,
        target_comp_id: 0 as *const libc::c_char,
        msg_seq_num: 0,
        check_sum: 0 as *const libc::c_char,
        str_now: 0 as *mut libc::c_char,
        head_buf: 0 as *mut buffer,
        body_buf: 0 as *mut buffer,
        nr_fields: 0,
        fields: 0 as *mut fix_field,
        iov: [iovec {
            iov_base: 0 as *mut libc::c_void,
            iov_len: 0,
        }; 2],
    };
    let mut msg: *mut fix_message = 0 as *mut fix_message;
    let mut nr: libc::c_ulong = 0;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    session = fix_session_new(cfg);
    if !session.is_null() {
        fix_session_recv(
            session,
            &mut msg,
            FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
        );
        logon_msg = {
            let mut init = fix_message {
                type_0: FIX_MSG_TYPE_LOGON,
                begin_string: 0 as *const libc::c_char,
                body_length: 0,
                msg_type: 0 as *const libc::c_char,
                sender_comp_id: 0 as *const libc::c_char,
                target_comp_id: 0 as *const libc::c_char,
                msg_seq_num: 0,
                check_sum: 0 as *const libc::c_char,
                str_now: 0 as *mut libc::c_char,
                head_buf: 0 as *mut buffer,
                body_buf: 0 as *mut buffer,
                nr_fields: 0,
                fields: 0 as *mut fix_field,
                iov: [iovec {
                    iov_base: 0 as *mut libc::c_void,
                    iov_len: 0,
                }; 2],
            };
            init
        };
        fix_session_send(session, &mut logon_msg, 0 as libc::c_int as libc::c_ulong);
        fields = calloc(
            48 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<fix_field>() as libc::c_ulong,
        ) as *mut fix_field;
        if !fields.is_null() {
            nr = fix_execution_report_fields(fields);
            loop {
                let mut logout_msg: fix_message = fix_message {
                    type_0: FIX_MSG_TYPE_HEARTBEAT,
                    begin_string: 0 as *const libc::c_char,
                    body_length: 0,
                    msg_type: 0 as *const libc::c_char,
                    sender_comp_id: 0 as *const libc::c_char,
                    target_comp_id: 0 as *const libc::c_char,
                    msg_seq_num: 0,
                    check_sum: 0 as *const libc::c_char,
                    str_now: 0 as *mut libc::c_char,
                    head_buf: 0 as *mut buffer,
                    body_buf: 0 as *mut buffer,
                    nr_fields: 0,
                    fields: 0 as *mut fix_field,
                    iov: [iovec {
                        iov_base: 0 as *mut libc::c_void,
                        iov_len: 0,
                    }; 2],
                };
                if fix_session_recv(
                    session,
                    &mut msg,
                    FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
                ) <= 0 as libc::c_int
                {
                    continue;
                }
                if fix_message_type_is(msg, FIX_MSG_TYPE_NEW_ORDER_SINGLE) {
                    fix_session_execution_report(session, fields, nr as libc::c_long);
                } else {
                    if !fix_message_type_is(msg, FIX_MSG_TYPE_LOGOUT) {
                        continue;
                    }
                    logout_msg = {
                        let mut init = fix_message {
                            type_0: FIX_MSG_TYPE_LOGOUT,
                            begin_string: 0 as *const libc::c_char,
                            body_length: 0,
                            msg_type: 0 as *const libc::c_char,
                            sender_comp_id: 0 as *const libc::c_char,
                            target_comp_id: 0 as *const libc::c_char,
                            msg_seq_num: 0,
                            check_sum: 0 as *const libc::c_char,
                            str_now: 0 as *mut libc::c_char,
                            head_buf: 0 as *mut buffer,
                            body_buf: 0 as *mut buffer,
                            nr_fields: 0,
                            fields: 0 as *mut fix_field,
                            iov: [iovec {
                                iov_base: 0 as *mut libc::c_void,
                                iov_len: 0,
                            }; 2],
                        };
                        init
                    };
                    fix_session_send(
                        session,
                        &mut logout_msg,
                        0 as libc::c_int as libc::c_ulong,
                    );
                    break;
                }
            }
            ret = 0 as libc::c_int;
        }
    }
    fix_session_free(session);
    free(fields as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn usage() {
    printf(
        b"\n usage: %s [-m mode] [-d dialect] [-f filename] [-s sender-comp-id] [-t target-comp-id] -p port\n\n\0"
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
unsafe extern "C" fn strservermode(mut mode: *const libc::c_char) -> fix_server_mode {
    let mut m: fix_server_mode = FIX_SERVER_SCRIPT;
    if strcmp(mode, b"session\0" as *const u8 as *const libc::c_char) == 0 {
        return FIX_SERVER_SESSION
    } else if strcmp(mode, b"script\0" as *const u8 as *const libc::c_char) == 0 {
        return FIX_SERVER_SCRIPT
    }
    if sscanf(
        mode,
        b"%u\0" as *const u8 as *const libc::c_char,
        &mut m as *mut fix_server_mode,
    ) != 1 as libc::c_int
    {
        return FIX_SERVER_SCRIPT;
    }
    match m as libc::c_uint {
        1 | 0 => return m,
        _ => {}
    }
    return FIX_SERVER_SCRIPT;
}
unsafe extern "C" fn strversion(mut dialect: *const libc::c_char) -> fix_version {
    if strcmp(dialect, b"fix42\0" as *const u8 as *const libc::c_char) == 0 {
        return FIX_4_2
    } else if strcmp(dialect, b"fix43\0" as *const u8 as *const libc::c_char) == 0 {
        return FIX_4_3
    } else if strcmp(dialect, b"fix44\0" as *const u8 as *const libc::c_char) == 0 {
        return FIX_4_4
    }
    return FIX_4_4;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut mode: fix_server_mode = FIX_SERVER_SCRIPT;
    let mut version: fix_version = FIX_4_4;
    let mut target_comp_id: *const libc::c_char = 0 as *const libc::c_char;
    let mut sender_comp_id: *const libc::c_char = 0 as *const libc::c_char;
    let mut arg: fix_server_arg = {
        let mut init = fix_server_arg {
            script: 0 as *const libc::c_char,
        };
        init
    };
    let mut cfg: fix_session_cfg = fix_session_cfg {
        sender_comp_id: [0; 32],
        target_comp_id: [0; 32],
        password: [0; 32],
        heartbtint: 0,
        dialect: 0 as *mut fix_dialect,
        sockfd: 0,
        in_msg_seq_num: 0,
        out_msg_seq_num: 0,
        user_data: 0 as *mut libc::c_void,
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
            b"f:p:d:s:t:m:\0" as *const u8 as *const libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            100 => {
                version = strversion(optarg);
            }
            115 => {
                sender_comp_id = optarg;
            }
            116 => {
                target_comp_id = optarg;
            }
            109 => {
                mode = strservermode(optarg);
            }
            112 => {
                port = atoi(optarg);
            }
            102 => {
                arg.script = optarg;
            }
            _ => {
                usage();
            }
        }
    }
    if port == 0 {
        usage();
    }
    fix_session_cfg_init(&mut cfg);
    cfg
        .dialect = &mut *fix_dialects.as_mut_ptr().offset(version as isize)
        as *mut fix_dialect;
    if sender_comp_id.is_null() {
        strncpy(
            (cfg.sender_comp_id).as_mut_ptr(),
            b"SELLSIDE\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    } else {
        strncpy(
            (cfg.sender_comp_id).as_mut_ptr(),
            sender_comp_id,
            (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    }
    if target_comp_id.is_null() {
        strncpy(
            (cfg.target_comp_id).as_mut_ptr(),
            b"BUYSIDE\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    } else {
        strncpy(
            (cfg.target_comp_id).as_mut_ptr(),
            target_comp_id,
            (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
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
        b"Server is listening to port %d...\n\0" as *const u8 as *const libc::c_char,
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
    match mode as libc::c_uint {
        0 => {
            ret = (fix_server_functions[mode as usize].fix_session_accept)
                .expect("non-null function pointer")(&mut cfg, &mut arg);
        }
        1 => {
            ret = (fix_server_functions[mode as usize].fix_session_accept)
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
