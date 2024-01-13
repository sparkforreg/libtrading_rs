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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fast_session_free(self_0: *mut fast_session);
    fn fast_session_new(cfg: *mut fast_session_cfg) -> *mut fast_session;
    fn fast_parse_template(
        self_0: *mut fast_session,
        xml: *const libc::c_char,
    ) -> libc::c_int;
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
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
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
pub const SHUT_RDWR: C2RustUnnamed_2 = 2;
pub const SHUT_WR: C2RustUnnamed_2 = 1;
pub const SHUT_RD: C2RustUnnamed_2 = 0;
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
    pub __in6_u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
pub const IPPROTO_IP: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_4 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_4 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_4 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_4 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_4 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_4 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_4 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_4 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_4 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_4 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_4 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_4 = 92;
pub const IPPROTO_AH: C2RustUnnamed_4 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_4 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_4 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_4 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_4 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_4 = 33;
pub const IPPROTO_TP: C2RustUnnamed_4 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_4 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_4 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_4 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_4 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_4 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_4 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_4 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_4 = 1;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
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
unsafe extern "C" fn fast_feed_socket(mut feed: *mut fast_feed) -> libc::c_int {
    let mut current_block: u64;
    let mut group_src: ip_mreq_source = ip_mreq_source {
        imr_multiaddr: in_addr { s_addr: 0 },
        imr_interface: in_addr { s_addr: 0 },
        imr_sourceaddr: in_addr { s_addr: 0 },
    };
    let mut group: ip_mreq = ip_mreq {
        imr_multiaddr: in_addr { s_addr: 0 },
        imr_interface: in_addr { s_addr: 0 },
    };
    let mut sa: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sockfd: libc::c_int = 0;
    sockfd = socket(2 as libc::c_int, SOCK_DGRAM as libc::c_int, 0 as libc::c_int);
    if !(sockfd < 0 as libc::c_int) {
        if !(fcntl(
            sockfd,
            4 as libc::c_int,
            fcntl(sockfd, 3 as libc::c_int) | 0o4000 as libc::c_int,
        ) < 0 as libc::c_int)
        {
            if !(socket_setopt(
                sockfd,
                1 as libc::c_int,
                2 as libc::c_int,
                1 as libc::c_int,
            ) < 0 as libc::c_int)
            {
                sa = {
                    let mut init = sockaddr_in {
                        sin_family: 2 as libc::c_int as sa_family_t,
                        sin_port: __bswap_16((*feed).port as __uint16_t),
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
                if !(bind(
                    sockfd,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: &mut sa as *mut sockaddr_in as *const sockaddr,
                    },
                    ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
                ) < 0 as libc::c_int)
                {
                    if inet_addr(((*feed).sip).as_mut_ptr())
                        == 0xffffffff as libc::c_uint
                    {
                        group
                            .imr_multiaddr
                            .s_addr = inet_addr(((*feed).ip).as_mut_ptr());
                        group
                            .imr_interface
                            .s_addr = inet_addr(((*feed).lip).as_mut_ptr());
                        if setsockopt(
                            sockfd,
                            IPPROTO_IP as libc::c_int,
                            35 as libc::c_int,
                            &mut group as *mut ip_mreq as *mut libc::c_char
                                as *const libc::c_void,
                            ::core::mem::size_of::<ip_mreq>() as libc::c_ulong
                                as socklen_t,
                        ) < 0 as libc::c_int
                        {
                            current_block = 8598455085417329327;
                        } else {
                            current_block = 4166486009154926805;
                        }
                    } else {
                        group_src
                            .imr_multiaddr
                            .s_addr = inet_addr(((*feed).ip).as_mut_ptr());
                        group_src
                            .imr_sourceaddr
                            .s_addr = inet_addr(((*feed).sip).as_mut_ptr());
                        group_src
                            .imr_interface
                            .s_addr = inet_addr(((*feed).lip).as_mut_ptr());
                        if setsockopt(
                            sockfd,
                            IPPROTO_IP as libc::c_int,
                            39 as libc::c_int,
                            &mut group_src as *mut ip_mreq_source as *mut libc::c_char
                                as *const libc::c_void,
                            ::core::mem::size_of::<ip_mreq_source>() as libc::c_ulong
                                as socklen_t,
                        ) < 0 as libc::c_int
                        {
                            current_block = 8598455085417329327;
                        } else {
                            current_block = 4166486009154926805;
                        }
                    }
                    match current_block {
                        8598455085417329327 => {}
                        _ => return sockfd,
                    }
                }
            }
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fast_feed_open(mut feed: *mut fast_feed) -> libc::c_int {
    let mut sockfd: libc::c_int = 0;
    if !(*feed).active {
        (*feed).session = 0 as *mut fast_session;
        if strlen(((*feed).file).as_mut_ptr()) != 0 {
            sockfd = open(((*feed).file).as_mut_ptr(), 0 as libc::c_int);
        } else {
            sockfd = fast_feed_socket(feed);
        }
        if !(sockfd < 0 as libc::c_int) {
            (*feed).cfg.sockfd = sockfd;
            (*feed).session = fast_session_new(&mut (*feed).cfg);
            if !((*feed).session).is_null() {
                if !(fast_parse_template((*feed).session, ((*feed).xml).as_mut_ptr())
                    != 0)
                {
                    (*feed).active = 1 as libc::c_int != 0;
                    return 0 as libc::c_int;
                }
            }
        }
    }
    fast_session_free((*feed).session);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fast_feed_close(mut feed: *mut fast_feed) -> libc::c_int {
    let mut session: *mut fast_session = 0 as *mut fast_session;
    let mut sockfd: libc::c_int = 0;
    session = (*feed).session;
    if (*feed).active {
        if !session.is_null() {
            sockfd = (*session).sockfd;
            fast_session_free(session);
            shutdown(sockfd, SHUT_RDWR as libc::c_int);
            if close(sockfd) < 0 as libc::c_int {
                return -(1 as libc::c_int)
            } else {
                (*feed).session = 0 as *mut fast_session;
                (*feed).active = 0 as libc::c_int != 0;
            }
        }
    }
    return 0 as libc::c_int;
}
