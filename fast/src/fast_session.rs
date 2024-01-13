use ::libc;
extern "C" {
    pub type _GHashTable;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn buffer_compact(buf: *mut buffer);
    fn buffer_nread(buf: *mut buffer, fd: libc::c_int, size: size_t) -> ssize_t;
    fn buffer_recv(
        self_0: *mut buffer,
        sockfd: libc::c_int,
        size: size_t,
        flags: libc::c_int,
    ) -> ssize_t;
    fn buffer_delete(self_0: *mut buffer);
    fn buffer_new(capacity: libc::c_ulong) -> *mut buffer;
    fn sendmsg(
        __fd: libc::c_int,
        __message: *const msghdr,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn fast_message_send(
        self_0: *mut fast_message,
        session: *mut fast_session,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn fast_message_decode(session: *mut fast_session) -> *mut fast_message;
    fn fast_message_reset(msg: *mut fast_message);
    fn fast_message_free(self_0: *mut fast_message, nr_messages: libc::c_int);
    fn fast_message_new(nr_messages: libc::c_int) -> *mut fast_message;
    fn xwritev(fd: libc::c_int, iov: *const iovec, iovcnt: libc::c_int) -> ssize_t;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type i64_0 = int64_t;
pub type u64_0 = uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub struct fast_session_cfg {
    pub preamble_bytes: libc::c_int,
    pub sockfd: libc::c_int,
    pub reset: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[inline]
unsafe extern "C" fn buffer_reset(mut buf: *mut buffer) {
    (*buf).end = 0 as libc::c_int as libc::c_ulong;
    (*buf).start = (*buf).end;
}
#[inline]
unsafe extern "C" fn buffer_remaining(mut self_0: *const buffer) -> libc::c_ulong {
    return ((*self_0).capacity).wrapping_sub((*self_0).end);
}
unsafe extern "C" fn xwritev0(
    mut fd: libc::c_int,
    mut msg: *const msghdr,
    mut flags: libc::c_int,
) -> ssize_t {
    return xwritev(fd, (*msg).msg_iov, (*msg).msg_iovlen as libc::c_int);
}
unsafe extern "C" fn buffer_nread0(
    mut buf: *mut buffer,
    mut fd: libc::c_int,
    mut size: size_t,
    mut flags: libc::c_int,
) -> ssize_t {
    return buffer_nread(buf, fd, size);
}
#[no_mangle]
pub unsafe extern "C" fn fast_session_new(
    mut cfg: *mut fast_session_cfg,
) -> *mut fast_session {
    let mut self_0: *mut fast_session = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<fast_session>() as libc::c_ulong,
    ) as *mut fast_session;
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if self_0.is_null() {
        return 0 as *mut fast_session;
    }
    (*self_0)
        .rx_buffer = buffer_new(
        (2 as libc::c_int * 2048 as libc::c_int) as libc::c_ulong,
    );
    if ((*self_0).rx_buffer).is_null() {
        fast_session_free(self_0);
        return 0 as *mut fast_session;
    }
    (*self_0)
        .tx_message_buffer = buffer_new(
        (2 as libc::c_int * 2048 as libc::c_int) as libc::c_ulong,
    );
    if ((*self_0).tx_message_buffer).is_null() {
        fast_session_free(self_0);
        return 0 as *mut fast_session;
    }
    (*self_0)
        .tx_pmap_buffer = buffer_new(
        (2 as libc::c_int * 2048 as libc::c_int) as libc::c_ulong,
    );
    if ((*self_0).tx_pmap_buffer).is_null() {
        fast_session_free(self_0);
        return 0 as *mut fast_session;
    }
    (*self_0).rx_messages = fast_message_new(128 as libc::c_int);
    if ((*self_0).rx_messages).is_null() {
        fast_session_free(self_0);
        return 0 as *mut fast_session;
    }
    if (*cfg).preamble_bytes > 8 as libc::c_int {
        fast_session_free(self_0);
        return 0 as *mut fast_session;
    } else {
        (*self_0).preamble.nr_bytes = (*cfg).preamble_bytes as libc::c_ulong;
    }
    if fstat((*cfg).sockfd, &mut statbuf) != 0 {
        fast_session_free(self_0);
        return 0 as *mut fast_session;
    }
    if !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
        == 0o140000 as libc::c_int as libc::c_uint)
    {
        (*self_0)
            .send = Some(
            xwritev0
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const msghdr,
                    libc::c_int,
                ) -> ssize_t,
        );
        (*self_0)
            .recv = Some(
            buffer_nread0
                as unsafe extern "C" fn(
                    *mut buffer,
                    libc::c_int,
                    size_t,
                    libc::c_int,
                ) -> ssize_t,
        );
    } else {
        (*self_0)
            .send = Some(
            sendmsg
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const msghdr,
                    libc::c_int,
                ) -> ssize_t,
        );
        (*self_0)
            .recv = Some(
            buffer_recv
                as unsafe extern "C" fn(
                    *mut buffer,
                    libc::c_int,
                    size_t,
                    libc::c_int,
                ) -> ssize_t,
        );
    }
    (*self_0).sockfd = (*cfg).sockfd;
    (*self_0).reset = (*cfg).reset;
    (*self_0).rx_message = 0 as *mut fast_message;
    (*self_0).last_tid = 0 as libc::c_int as u64_0;
    (*self_0).nr_messages = 0 as libc::c_int;
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn fast_session_free(mut self_0: *mut fast_session) {
    if self_0.is_null() {
        return;
    }
    fast_message_free((*self_0).rx_messages, 128 as libc::c_int);
    buffer_delete((*self_0).tx_message_buffer);
    buffer_delete((*self_0).tx_pmap_buffer);
    buffer_delete((*self_0).rx_buffer);
    free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fast_session_recv(
    mut self_0: *mut fast_session,
    mut flags: libc::c_int,
) -> *mut fast_message {
    let mut buffer: *mut buffer = (*self_0).rx_buffer;
    let mut msg: *mut fast_message = 0 as *mut fast_message;
    let mut size: size_t = 0;
    let mut nr: ssize_t = 0;
    msg = fast_message_decode(self_0);
    if !msg.is_null() {
        return msg;
    }
    size = buffer_remaining(buffer);
    if size <= 2048 as libc::c_int as libc::c_ulong {
        buffer_compact(buffer);
    }
    nr = ((*self_0).recv)
        .expect(
            "non-null function pointer",
        )(buffer, (*self_0).sockfd, 2048 as libc::c_int as size_t, flags);
    if nr <= 0 as libc::c_int as libc::c_long {
        return 0 as *mut fast_message;
    }
    return fast_message_decode(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn fast_session_send(
    mut self_0: *mut fast_session,
    mut msg: *mut fast_message,
    mut flags: libc::c_int,
) -> libc::c_int {
    (*msg).pmap_buf = (*self_0).tx_pmap_buffer;
    buffer_reset((*msg).pmap_buf);
    (*msg).msg_buf = (*self_0).tx_message_buffer;
    buffer_reset((*msg).msg_buf);
    return fast_message_send(msg, self_0, flags);
}
#[no_mangle]
pub unsafe extern "C" fn fast_session_reset(mut self_0: *mut fast_session) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*self_0).nr_messages {
        fast_message_reset(((*self_0).rx_messages).offset(i as isize));
        i += 1;
        i;
    }
}
