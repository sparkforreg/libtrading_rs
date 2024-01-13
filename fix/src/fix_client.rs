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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn fix_message_type_is(self_0: *mut fix_message, type_0: fix_msg_type) -> bool;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn connect(
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
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    static mut fix_dialects: [fix_dialect; 0];
    fn fix_session_cfg_init(cfg: *mut fix_session_cfg);
    fn fix_session_new(cfg: *mut fix_session_cfg) -> *mut fix_session;
    fn fix_session_free(self_0: *mut fix_session);
    fn fix_session_time_update(self_0: *mut fix_session) -> libc::c_int;
    fn fix_session_send(
        self_0: *mut fix_session,
        msg: *mut fix_message,
        flags: libc::c_ulong,
    ) -> libc::c_int;
    fn fix_session_recv(
        self_0: *mut fix_session,
        msg: *mut *mut fix_message,
        flags: libc::c_ulong,
    ) -> libc::c_int;
    fn fix_session_new_order_single(
        session: *mut fix_session,
        fields: *mut fix_field,
        nr_fields: libc::c_long,
    ) -> libc::c_int;
    fn fix_session_keepalive(session: *mut fix_session, now: *mut timespec) -> bool;
    fn fix_session_admin(session: *mut fix_session, msg: *mut fix_message) -> bool;
    fn fix_session_logout(
        session: *mut fix_session,
        text: *const libc::c_char,
    ) -> libc::c_int;
    fn fix_session_logon(session: *mut fix_session) -> libc::c_int;
    fn do_die(format: *const libc::c_char, _: ...) -> !;
    fn error(format: *const libc::c_char, _: ...) -> !;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn __h_errno_location() -> *mut libc::c_int;
    fn hstrerror(__err_num: libc::c_int) -> *const libc::c_char;
    fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
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
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type __sig_atomic_t = libc::c_int;
pub type clockid_t = __clockid_t;
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
pub type uint64_t = __uint64_t;
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
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed_0 = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed_0 = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed_0 = 67108864;
pub const MSG_BATCH: C2RustUnnamed_0 = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed_0 = 65536;
pub const MSG_MORE: C2RustUnnamed_0 = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed_0 = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed_0 = 8192;
pub const MSG_RST: C2RustUnnamed_0 = 4096;
pub const MSG_CONFIRM: C2RustUnnamed_0 = 2048;
pub const MSG_SYN: C2RustUnnamed_0 = 1024;
pub const MSG_FIN: C2RustUnnamed_0 = 512;
pub const MSG_WAITALL: C2RustUnnamed_0 = 256;
pub const MSG_EOR: C2RustUnnamed_0 = 128;
pub const MSG_DONTWAIT: C2RustUnnamed_0 = 64;
pub const MSG_TRUNC: C2RustUnnamed_0 = 32;
pub const MSG_PROXY: C2RustUnnamed_0 = 16;
pub const MSG_CTRUNC: C2RustUnnamed_0 = 8;
pub const MSG_TRYHARD: C2RustUnnamed_0 = 4;
pub const MSG_DONTROUTE: C2RustUnnamed_0 = 4;
pub const MSG_PEEK: C2RustUnnamed_0 = 2;
pub const MSG_OOB: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_1 = 2;
pub const SHUT_WR: C2RustUnnamed_1 = 1;
pub const SHUT_RD: C2RustUnnamed_1 = 0;
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
    pub __in6_u: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
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
pub type fix_send_flag = libc::c_uint;
pub const FIX_SEND_FLAG_PRESERVE_BUFFER: fix_send_flag = 2;
pub const FIX_SEND_FLAG_PRESERVE_MSG_NUM: fix_send_flag = 1;
pub type fix_recv_flag = libc::c_uint;
pub const FIX_RECV_KEEP_IN_MSGSEQNUM: fix_recv_flag = 131072;
pub const FIX_RECV_FLAG_MSG_DONTWAIT: fix_recv_flag = 65536;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_3 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_3 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_3 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_3 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_3 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_3 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_3 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_3 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_3 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_3 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_3 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_3 = 92;
pub const IPPROTO_AH: C2RustUnnamed_3 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_3 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_3 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_3 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_3 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_3 = 33;
pub const IPPROTO_TP: C2RustUnnamed_3 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_3 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_3 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_3 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_3 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_3 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_3 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_3 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_3 = 1;
pub const IPPROTO_IP: C2RustUnnamed_3 = 0;
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut libc::c_char,
    pub h_aliases: *mut *mut libc::c_char,
    pub h_addrtype: libc::c_int,
    pub h_length: libc::c_int,
    pub h_addr_list: *mut *mut libc::c_char,
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
pub type fix_client_mode = libc::c_uint;
pub const FIX_CLIENT_ORDER: fix_client_mode = 2;
pub const FIX_CLIENT_SESSION: fix_client_mode = 1;
pub const FIX_CLIENT_SCRIPT: fix_client_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fix_client_arg {
    pub script: *const libc::c_char,
    pub output: *const libc::c_char,
    pub orders: libc::c_int,
    pub warmup_orders: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fix_client_function {
    pub fix_session_initiate: Option::<
        unsafe extern "C" fn(*mut fix_session_cfg, *mut fix_client_arg) -> libc::c_int,
    >,
    pub mode: fix_client_mode,
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
unsafe extern "C" fn timespec_delta(
    mut before: *mut timespec,
    mut after: *mut timespec,
) -> uint64_t {
    return (1000000000 as libc::c_int as libc::c_long
        * ((*after).tv_sec - (*before).tv_sec) + ((*after).tv_nsec - (*before).tv_nsec))
        as uint64_t;
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
static mut stop: sig_atomic_t = 0;
static mut fix_client_functions: [fix_client_function; 3] = unsafe {
    [
        {
            let mut init = fix_client_function {
                fix_session_initiate: Some(
                    fix_client_script
                        as unsafe extern "C" fn(
                            *mut fix_session_cfg,
                            *mut fix_client_arg,
                        ) -> libc::c_int,
                ),
                mode: FIX_CLIENT_SCRIPT,
            };
            init
        },
        {
            let mut init = fix_client_function {
                fix_session_initiate: Some(
                    fix_client_session
                        as unsafe extern "C" fn(
                            *mut fix_session_cfg,
                            *mut fix_client_arg,
                        ) -> libc::c_int,
                ),
                mode: FIX_CLIENT_SESSION,
            };
            init
        },
        {
            let mut init = fix_client_function {
                fix_session_initiate: Some(
                    fix_client_order
                        as unsafe extern "C" fn(
                            *mut fix_session_cfg,
                            *mut fix_client_arg,
                        ) -> libc::c_int,
                ),
                mode: FIX_CLIENT_ORDER,
            };
            init
        },
    ]
};
unsafe extern "C" fn signal_handler(mut signum: libc::c_int) {
    if signum == 2 as libc::c_int {
        stop = 1 as libc::c_int;
    }
}
unsafe extern "C" fn fix_logout_send(
    mut session: *mut fix_session,
    mut text: *const libc::c_char,
) -> libc::c_int {
    let mut fields: [fix_field; 1] = [
        {
            let mut init = fix_field {
                tag: Text as libc::c_int,
                type_0: FIX_TYPE_STRING,
                c2rust_unnamed: C2RustUnnamed {
                    string_value: text,
                },
            };
            init
        },
    ];
    let mut nr_fields: libc::c_long = (::core::mem::size_of::<[fix_field; 1]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<fix_field>() as libc::c_ulong)
        as libc::c_long;
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
    if text.is_null() {
        nr_fields -= 1;
        nr_fields;
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
            nr_fields: nr_fields as libc::c_ulong,
            fields: fields.as_mut_ptr(),
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    return fix_session_send(session, &mut logout_msg, 0 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn fix_client_logout(
    mut session: *mut fix_session,
    mut text: *const libc::c_char,
    mut grace: bool,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if grace {
        ret = fix_session_logout(session, text);
    } else {
        ret = fix_logout_send(session, text);
    }
    if ret != 0 {
        fprintf(stderr, b"Client Logout FAILED\n\0" as *const u8 as *const libc::c_char);
    } else {
        fprintf(stdout, b"Client Logout OK\n\0" as *const u8 as *const libc::c_char);
    }
    return ret;
}
unsafe extern "C" fn fix_client_script(
    mut cfg: *mut fix_session_cfg,
    mut arg: *mut fix_client_arg,
) -> libc::c_int {
    let mut current_block: u64;
    let mut s_container: *mut fcontainer = 0 as *mut fcontainer;
    let mut c_container: *mut fcontainer = 0 as *mut fcontainer;
    let mut session: *mut fix_session = 0 as *mut fix_session;
    let mut expected_elem: *mut felem = 0 as *mut felem;
    let mut tosend_elem: *mut felem = 0 as *mut felem;
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
                    } else {
                        ret = fix_session_logon(session);
                        if ret != 0 {
                            fprintf(
                                stderr,
                                b"Client Logon FAILED\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            fprintf(
                                stdout,
                                b"Client Logon OK\n\0" as *const u8 as *const libc::c_char,
                            );
                            expected_elem = cur_elem(s_container);
                            tosend_elem = cur_elem(c_container);
                            loop {
                                if tosend_elem.is_null() {
                                    current_block = 1836292691772056875;
                                    break;
                                }
                                if (*tosend_elem).msg.msg_seq_num != 0 {
                                    fix_session_send(
                                        session,
                                        &mut (*tosend_elem).msg,
                                        FIX_SEND_FLAG_PRESERVE_MSG_NUM as libc::c_int
                                            as libc::c_ulong,
                                    );
                                } else {
                                    fix_session_send(
                                        session,
                                        &mut (*tosend_elem).msg,
                                        0 as libc::c_int as libc::c_ulong,
                                    );
                                }
                                if !expected_elem.is_null() {
                                    while fix_session_recv(
                                        session,
                                        &mut msg,
                                        FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
                                    ) <= 0 as libc::c_int
                                    {}
                                    fprintf(
                                        stdout,
                                        b"< \0" as *const u8 as *const libc::c_char,
                                    );
                                    fprintmsg_iov(stdout, msg);
                                    if fmsgcmp(&mut (*expected_elem).msg, msg) != 0 {
                                        fprintf(
                                            stderr,
                                            b"Client: messages differ\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        fprintmsg(stderr, &mut (*expected_elem).msg);
                                        fprintmsg(stderr, msg);
                                        current_block = 1836292691772056875;
                                        break;
                                    } else if fix_message_type_is(msg, FIX_MSG_TYPE_LOGOUT) {
                                        ret = fix_client_logout(
                                            session,
                                            0 as *const libc::c_char,
                                            0 as libc::c_int != 0,
                                        );
                                        current_block = 973940853187896437;
                                        break;
                                    }
                                }
                                expected_elem = next_elem(s_container);
                                tosend_elem = next_elem(c_container);
                            }
                            match current_block {
                                973940853187896437 => {}
                                _ => {
                                    ret = if !tosend_elem.is_null() {
                                        -(1 as libc::c_int)
                                    } else {
                                        0 as libc::c_int
                                    };
                                    if !(ret != 0) {
                                        ret = fix_client_logout(
                                            session,
                                            0 as *const libc::c_char,
                                            1 as libc::c_int != 0,
                                        );
                                    }
                                }
                            }
                        }
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
unsafe extern "C" fn fix_client_session(
    mut cfg: *mut fix_session_cfg,
    mut arg: *mut fix_client_arg,
) -> libc::c_int {
    let mut current_block: u64;
    let mut session: *mut fix_session = 0 as *mut fix_session;
    let mut cur: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut prev: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut msg: *mut fix_message = 0 as *mut fix_message;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut diff: libc::c_int = 0;
    if signal(
        2 as libc::c_int,
        Some(signal_handler as unsafe extern "C" fn(libc::c_int) -> ()),
    )
        == ::core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        fprintf(
            stderr,
            b"Unable to register a signal handler\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        session = fix_session_new(cfg);
        if session.is_null() {
            fprintf(
                stderr,
                b"FIX session cannot be created\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            ret = fix_session_logon(session);
            if ret != 0 {
                fprintf(
                    stderr,
                    b"Client Logon FAILED\n\0" as *const u8 as *const libc::c_char,
                );
            } else {
                fprintf(
                    stdout,
                    b"Client Logon OK\n\0" as *const u8 as *const libc::c_char,
                );
                clock_gettime(1 as libc::c_int, &mut prev);
                while stop == 0 && (*session).active as libc::c_int != 0 {
                    clock_gettime(1 as libc::c_int, &mut cur);
                    diff = (cur.tv_sec - prev.tv_sec) as libc::c_int;
                    if diff as libc::c_double
                        > 0.1f64 * (*session).heartbtint as libc::c_double
                    {
                        prev = cur;
                        if !fix_session_keepalive(session, &mut cur) {
                            stop = 1 as libc::c_int;
                            break;
                        }
                    }
                    if fix_session_time_update(session) != 0 {
                        stop = 1 as libc::c_int;
                        break;
                    } else {
                        if !(fix_session_recv(
                            session,
                            &mut msg,
                            FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
                        ) <= 0 as libc::c_int)
                        {
                            continue;
                        }
                        fprintmsg(stdout, msg);
                        if fix_session_admin(session, msg) {
                            continue;
                        }
                        match (*msg).type_0 as libc::c_ulong {
                            5 => {
                                stop = 1 as libc::c_int;
                            }
                            _ => {
                                stop = 1 as libc::c_int;
                            }
                        }
                    }
                }
                if (*session).active {
                    ret = fix_session_logout(session, 0 as *const libc::c_char);
                    if ret != 0 {
                        fprintf(
                            stderr,
                            b"Client Logout FAILED\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        current_block = 1060509205667815787;
                    } else {
                        current_block = 11932355480408055363;
                    }
                } else {
                    current_block = 11932355480408055363;
                }
                match current_block {
                    1060509205667815787 => {}
                    _ => {
                        fprintf(
                            stdout,
                            b"Client Logout OK\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
        }
    }
    fix_session_free(session);
    return ret;
}
unsafe extern "C" fn fix_new_order_single_fields(
    mut session: *mut fix_session,
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
            tag: TransactTime as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: ((*session).str_now).as_mut_ptr(),
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
            tag: ClOrdID as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"ClOrdID\0" as *const u8 as *const libc::c_char,
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
            tag: Symbol as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"Symbol\0" as *const u8 as *const libc::c_char,
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
            tag: OrderQty as libc::c_int,
            type_0: FIX_TYPE_FLOAT,
            c2rust_unnamed: C2RustUnnamed {
                float_value: 100 as libc::c_int as libc::c_double,
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
            tag: OrdType as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"2\0" as *const u8 as *const libc::c_char,
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
            tag: Side as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: b"1\0" as *const u8 as *const libc::c_char,
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
            tag: Price as libc::c_int,
            type_0: FIX_TYPE_FLOAT,
            c2rust_unnamed: C2RustUnnamed {
                float_value: 100 as libc::c_int as libc::c_double,
            },
        };
        init
    };
    return nr;
}
unsafe extern "C" fn fix_client_order(
    mut cfg: *mut fix_session_cfg,
    mut arg: *mut fix_client_arg,
) -> libc::c_int {
    let mut current_block: u64;
    let mut min_usec: libc::c_double = 0.;
    let mut avg_usec: libc::c_double = 0.;
    let mut max_usec: libc::c_double = 0.;
    let mut total_usec: libc::c_double = 0.;
    let mut session: *mut fix_session = 0 as *mut fix_session;
    let mut fields: *mut fix_field = 0 as *mut fix_field;
    let mut msg: *mut fix_message = 0 as *mut fix_message;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut nr: libc::c_ulong = 0;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut orders: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if !arg.is_null() {
        orders = (*arg).orders;
        if !((*arg).output).is_null() {
            file = fopen((*arg).output, b"w\0" as *const u8 as *const libc::c_char);
            if file.is_null() {
                fprintf(
                    stderr,
                    b"Cannot open a file %s\n\0" as *const u8 as *const libc::c_char,
                    (*arg).output,
                );
                current_block = 10383658292319490590;
            } else {
                current_block = 5720623009719927633;
            }
        } else {
            current_block = 5720623009719927633;
        }
        match current_block {
            10383658292319490590 => {}
            _ => {
                session = fix_session_new(cfg);
                if session.is_null() {
                    fprintf(
                        stderr,
                        b"FIX session cannot be created\n\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    ret = fix_session_logon(session);
                    if ret != 0 {
                        fprintf(
                            stderr,
                            b"Client Logon FAILED\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        fprintf(
                            stdout,
                            b"Client Logon OK\n\0" as *const u8 as *const libc::c_char,
                        );
                        ret = -(1 as libc::c_int);
                        fields = calloc(
                            48 as libc::c_int as libc::c_ulong,
                            ::core::mem::size_of::<fix_field>() as libc::c_ulong,
                        ) as *mut fix_field;
                        if fields.is_null() {
                            fprintf(
                                stderr,
                                b"Cannot allocate memory\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            nr = fix_new_order_single_fields(session, fields);
                            min_usec = 1.7976931348623157e+308f64;
                            max_usec = 0 as libc::c_int as libc::c_double;
                            total_usec = 0 as libc::c_int as libc::c_double;
                            i = 0 as libc::c_int;
                            while i < (*arg).warmup_orders {
                                fix_session_new_order_single(
                                    session,
                                    fields,
                                    nr as libc::c_long,
                                );
                                loop {
                                    if fix_session_recv(
                                        session,
                                        &mut msg,
                                        MSG_DONTWAIT as libc::c_int as libc::c_ulong,
                                    ) <= 0 as libc::c_int
                                    {
                                        continue;
                                    }
                                    if fix_message_type_is(msg, FIX_MSG_TYPE_EXECUTION_REPORT) {
                                        break;
                                    }
                                }
                                i += 1;
                                i;
                            }
                            i = 0 as libc::c_int;
                            while i < orders {
                                let mut before: timespec = timespec {
                                    tv_sec: 0,
                                    tv_nsec: 0,
                                };
                                let mut after: timespec = timespec {
                                    tv_sec: 0,
                                    tv_nsec: 0,
                                };
                                let mut elapsed_usec: uint64_t = 0;
                                clock_gettime(1 as libc::c_int, &mut before);
                                fix_session_new_order_single(
                                    session,
                                    fields,
                                    nr as libc::c_long,
                                );
                                loop {
                                    if fix_session_recv(
                                        session,
                                        &mut msg,
                                        FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
                                    ) <= 0 as libc::c_int
                                    {
                                        continue;
                                    }
                                    if fix_message_type_is(msg, FIX_MSG_TYPE_EXECUTION_REPORT) {
                                        break;
                                    }
                                }
                                clock_gettime(1 as libc::c_int, &mut after);
                                elapsed_usec = (timespec_delta(&mut before, &mut after))
                                    .wrapping_div(1000 as libc::c_int as libc::c_ulong);
                                total_usec += elapsed_usec as libc::c_double;
                                min_usec = fmin(min_usec, elapsed_usec as libc::c_double);
                                max_usec = fmax(max_usec, elapsed_usec as libc::c_double);
                                if !file.is_null() {
                                    fprintf(
                                        file,
                                        b"%lu\n\0" as *const u8 as *const libc::c_char,
                                        elapsed_usec,
                                    );
                                }
                                i += 1;
                                i;
                            }
                            avg_usec = total_usec / orders as libc::c_double;
                            fprintf(
                                stdout,
                                b"Messages sent: %d\n\0" as *const u8
                                    as *const libc::c_char,
                                orders,
                            );
                            fprintf(
                                stdout,
                                b"Round-trip time: min/avg/max = %.1lf/%.1lf/%.1lf \xCE\xBCs\n\0"
                                    as *const u8 as *const libc::c_char,
                                min_usec,
                                avg_usec,
                                max_usec,
                            );
                            if (*session).active {
                                ret = fix_session_logout(session, 0 as *const libc::c_char);
                                if ret != 0 {
                                    fprintf(
                                        stderr,
                                        b"Client Logout FAILED\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    current_block = 10383658292319490590;
                                } else {
                                    current_block = 12997042908615822766;
                                }
                            } else {
                                current_block = 12997042908615822766;
                            }
                            match current_block {
                                10383658292319490590 => {}
                                _ => {
                                    fprintf(
                                        stdout,
                                        b"Client Logout OK\n\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fix_session_free(session);
    free(fields as *mut libc::c_void);
    if !file.is_null() {
        fclose(file);
    }
    return ret;
}
unsafe extern "C" fn usage() {
    printf(
        b"\n usage: %s [-m mode] [-d dialect] [-f filename] [-n orders] [-s sender-comp-id] [-t target-comp-id] [-r password] [-w warmup orders] -h hostname -p port\n\n\0"
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
unsafe extern "C" fn strclientmode(mut mode: *const libc::c_char) -> fix_client_mode {
    let mut m: fix_client_mode = FIX_CLIENT_SCRIPT;
    if strcmp(mode, b"session\0" as *const u8 as *const libc::c_char) == 0 {
        return FIX_CLIENT_SESSION
    } else if strcmp(mode, b"script\0" as *const u8 as *const libc::c_char) == 0 {
        return FIX_CLIENT_SCRIPT
    } else if strcmp(mode, b"order\0" as *const u8 as *const libc::c_char) == 0 {
        return FIX_CLIENT_ORDER
    }
    if sscanf(
        mode,
        b"%u\0" as *const u8 as *const libc::c_char,
        &mut m as *mut fix_client_mode,
    ) != 1 as libc::c_int
    {
        return FIX_CLIENT_SCRIPT;
    }
    match m as libc::c_uint {
        1 | 0 | 2 => return m,
        _ => {}
    }
    return FIX_CLIENT_SCRIPT;
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
    let mut mode: fix_client_mode = FIX_CLIENT_SCRIPT;
    let mut version: fix_version = FIX_4_4;
    let mut target_comp_id: *const libc::c_char = 0 as *const libc::c_char;
    let mut sender_comp_id: *const libc::c_char = 0 as *const libc::c_char;
    let mut arg: fix_client_arg = {
        let mut init = fix_client_arg {
            script: 0 as *const libc::c_char,
            output: 0 as *const libc::c_char,
            orders: 0,
            warmup_orders: 0,
        };
        init
    };
    let mut password: *const libc::c_char = 0 as *const libc::c_char;
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
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut saved_errno: libc::c_int = 0 as libc::c_int;
    let mut he: *mut hostent = 0 as *mut hostent;
    let mut port: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut ap: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut opt: libc::c_int = 0;
    program = __xpg_basename(*argv.offset(0 as libc::c_int as isize));
    loop {
        opt = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"f:h:p:d:s:t:m:n:o:r:w:\0" as *const u8 as *const libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            100 => {
                version = strversion(optarg);
            }
            110 => {
                arg.orders = atoi(optarg);
            }
            115 => {
                sender_comp_id = optarg;
            }
            116 => {
                target_comp_id = optarg;
            }
            114 => {
                password = optarg;
            }
            109 => {
                mode = strclientmode(optarg);
            }
            112 => {
                port = atoi(optarg);
            }
            102 => {
                arg.script = optarg;
            }
            111 => {
                arg.output = optarg;
            }
            104 => {
                host = optarg;
            }
            119 => {
                arg.warmup_orders = atoi(optarg);
            }
            _ => {
                usage();
            }
        }
    }
    if port == 0 || host.is_null() {
        usage();
    }
    fix_session_cfg_init(&mut cfg);
    cfg
        .dialect = &mut *fix_dialects.as_mut_ptr().offset(version as isize)
        as *mut fix_dialect;
    if password.is_null() {
        memset(
            (cfg.password).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    } else {
        strncpy(
            (cfg.password).as_mut_ptr(),
            password,
            (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
    }
    if sender_comp_id.is_null() {
        strncpy(
            (cfg.sender_comp_id).as_mut_ptr(),
            b"BUYSIDE\0" as *const u8 as *const libc::c_char,
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
            b"SELLSIDE\0" as *const u8 as *const libc::c_char,
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
    he = gethostbyname(host);
    if he.is_null() {
        error(
            b"Unable to look up %s (%s)\0" as *const u8 as *const libc::c_char,
            host,
            hstrerror(*__h_errno_location()),
        );
    }
    ap = (*he).h_addr_list;
    while !(*ap).is_null() {
        cfg
            .sockfd = socket(
            (*he).h_addrtype,
            SOCK_STREAM as libc::c_int,
            IPPROTO_TCP as libc::c_int,
        );
        if cfg.sockfd < 0 as libc::c_int {
            saved_errno = *__errno_location();
        } else {
            sa = {
                let mut init = sockaddr_in {
                    sin_family: (*he).h_addrtype as sa_family_t,
                    sin_port: __bswap_16(port as __uint16_t),
                    sin_addr: in_addr { s_addr: 0 },
                    sin_zero: [0; 8],
                };
                init
            };
            memcpy(
                &mut sa.sin_addr as *mut in_addr as *mut libc::c_void,
                *ap as *const libc::c_void,
                (*he).h_length as libc::c_ulong,
            );
            if !(connect(
                cfg.sockfd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &mut sa as *mut sockaddr_in as *const sockaddr,
                },
                ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
            ) < 0 as libc::c_int)
            {
                break;
            }
            saved_errno = *__errno_location();
            close(cfg.sockfd);
            cfg.sockfd = -(1 as libc::c_int);
        }
        ap = ap.offset(1);
        ap;
    }
    if cfg.sockfd < 0 as libc::c_int {
        error(
            b"Unable to connect to a socket (%s)\0" as *const u8 as *const libc::c_char,
            strerror(saved_errno),
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
    cfg.heartbtint = 15 as libc::c_int;
    match mode as libc::c_uint {
        0 => {
            ret = (fix_client_functions[mode as usize].fix_session_initiate)
                .expect("non-null function pointer")(&mut cfg, &mut arg);
        }
        2 => {
            ret = (fix_client_functions[mode as usize].fix_session_initiate)
                .expect("non-null function pointer")(&mut cfg, &mut arg);
        }
        1 => {
            ret = (fix_client_functions[mode as usize].fix_session_initiate)
                .expect("non-null function pointer")(&mut cfg, 0 as *mut fix_client_arg);
        }
        _ => {
            error(b"Invalid mode\0" as *const u8 as *const libc::c_char);
        }
    }
    shutdown(cfg.sockfd, SHUT_RDWR as libc::c_int);
    if close(cfg.sockfd) < 0 as libc::c_int {
        do_die(
            b"%s: close\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0")).as_ptr(),
        );
    }
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
