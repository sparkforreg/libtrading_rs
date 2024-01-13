use ::libc;
extern "C" {
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn fix_message_new() -> *mut fix_message;
    fn fix_message_free(self_0: *mut fix_message);
    fn fix_message_parse(
        self_0: *mut fix_message,
        dialect: *mut fix_dialect,
        buffer: *mut buffer,
        flags: libc::c_ulong,
    ) -> libc::c_int;
    fn fix_message_send(
        self_0: *mut fix_message,
        sockfd: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn buffer_new(capacity: libc::c_ulong) -> *mut buffer;
    fn buffer_delete(self_0: *mut buffer);
    fn buffer_recv(
        self_0: *mut buffer,
        sockfd: libc::c_int,
        size: size_t,
        flags: libc::c_int,
    ) -> ssize_t;
    fn buffer_compact(buf: *mut buffer);
    static mut fix_dialects: [fix_dialect; 0];
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
pub const FIX_SEND_FLAG_PRESERVE_MSG_NUM: fix_send_flag = 1;
pub const FIX_RECV_KEEP_IN_MSGSEQNUM: fix_recv_flag = 131072;
pub const FIX_RECV_FLAG_MSG_DONTWAIT: fix_recv_flag = 65536;
pub type fix_send_flag = libc::c_uint;
pub const FIX_SEND_FLAG_PRESERVE_BUFFER: fix_send_flag = 2;
pub type fix_recv_flag = libc::c_uint;
#[inline]
unsafe extern "C" fn buffer_remaining(mut self_0: *const buffer) -> libc::c_ulong {
    return ((*self_0).capacity).wrapping_sub((*self_0).end);
}
#[inline]
unsafe extern "C" fn buffer_reset(mut buf: *mut buffer) {
    (*buf).end = 0 as libc::c_int as libc::c_ulong;
    (*buf).start = (*buf).end;
}
static mut begin_strings: [*const libc::c_char; 7] = [
    b"FIX.4.0\0" as *const u8 as *const libc::c_char,
    b"FIX.4.1\0" as *const u8 as *const libc::c_char,
    b"FIX.4.2\0" as *const u8 as *const libc::c_char,
    b"FIX.4.3\0" as *const u8 as *const libc::c_char,
    b"FIX.4.4\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    b"FIXT.1.1\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn fix_session_cfg_init(mut cfg: *mut fix_session_cfg) {
    memset(
        cfg as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<fix_session_cfg>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_cfg_new(
    mut sender_comp_id: *const libc::c_char,
    mut target_comp_id: *const libc::c_char,
    mut heartbtint: libc::c_int,
    mut dialect: *const libc::c_char,
    mut sockfd: libc::c_int,
) -> *mut fix_session_cfg {
    let mut cfg: *mut fix_session_cfg = 0 as *mut fix_session_cfg;
    let mut version: fix_version = FIX_4_0;
    cfg = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<fix_session_cfg>() as libc::c_ulong,
    ) as *mut fix_session_cfg;
    strncpy(
        ((*cfg).sender_comp_id).as_mut_ptr(),
        sender_comp_id,
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    strncpy(
        ((*cfg).target_comp_id).as_mut_ptr(),
        target_comp_id,
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    (*cfg).heartbtint = heartbtint;
    version = (if strcmp(dialect, b"fixt-1.1\0" as *const u8 as *const libc::c_char) == 0
    {
        FIXT_1_1 as libc::c_int
    } else if strcmp(dialect, b"libtrading_rs-4.0\0" as *const u8 as *const libc::c_char) == 0 {
        FIX_4_0 as libc::c_int
    } else if strcmp(dialect, b"libtrading_rs-4.1\0" as *const u8 as *const libc::c_char) == 0 {
        FIX_4_1 as libc::c_int
    } else if strcmp(dialect, b"libtrading_rs-4.2\0" as *const u8 as *const libc::c_char) == 0 {
        FIX_4_2 as libc::c_int
    } else if strcmp(dialect, b"libtrading_rs-4.3\0" as *const u8 as *const libc::c_char) == 0 {
        FIX_4_3 as libc::c_int
    } else {
        FIX_4_4 as libc::c_int
    }) as fix_version;
    (*cfg)
        .dialect = &mut *fix_dialects.as_mut_ptr().offset(version as isize)
        as *mut fix_dialect;
    (*cfg).sockfd = sockfd;
    return cfg;
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_new(
    mut cfg: *mut fix_session_cfg,
) -> *mut fix_session {
    let mut self_0: *mut fix_session = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<fix_session>() as libc::c_ulong,
    ) as *mut fix_session;
    if self_0.is_null() {
        return 0 as *mut fix_session;
    }
    (*self_0).dialect = (*cfg).dialect;
    (*self_0).rx_buffer = buffer_new(4096 as libc::c_ulong);
    if ((*self_0).rx_buffer).is_null() {
        fix_session_free(self_0);
        return 0 as *mut fix_session;
    }
    (*self_0).tx_head_buffer = buffer_new(256 as libc::c_ulong);
    if ((*self_0).tx_head_buffer).is_null() {
        fix_session_free(self_0);
        return 0 as *mut fix_session;
    }
    (*self_0).tx_body_buffer = buffer_new(1024 as libc::c_ulong);
    if ((*self_0).tx_body_buffer).is_null() {
        fix_session_free(self_0);
        return 0 as *mut fix_session;
    }
    (*self_0).rx_message = fix_message_new();
    if ((*self_0).rx_message).is_null() {
        fix_session_free(self_0);
        return 0 as *mut fix_session;
    }
    if fix_session_time_update(self_0) != 0 {
        fix_session_free(self_0);
        return 0 as *mut fix_session;
    }
    (*self_0).rx_timestamp = (*self_0).now;
    (*self_0).tx_timestamp = (*self_0).now;
    (*self_0).begin_string = begin_strings[(*(*cfg).dialect).version as usize];
    (*self_0).sender_comp_id = ((*cfg).sender_comp_id).as_mut_ptr();
    (*self_0).target_comp_id = ((*cfg).target_comp_id).as_mut_ptr();
    (*self_0).heartbtint = (*cfg).heartbtint;
    (*self_0).password = ((*cfg).password).as_mut_ptr();
    (*self_0).sockfd = (*cfg).sockfd;
    (*self_0).tr_pending = 0 as libc::c_int;
    (*self_0)
        .in_msg_seq_num = if (*cfg).in_msg_seq_num > 0 as libc::c_int as libc::c_ulong {
        (*cfg).in_msg_seq_num
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    (*self_0)
        .out_msg_seq_num = if (*cfg).out_msg_seq_num > 1 as libc::c_int as libc::c_ulong
    {
        (*cfg).out_msg_seq_num
    } else {
        1 as libc::c_int as libc::c_ulong
    };
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_free(mut self_0: *mut fix_session) {
    if self_0.is_null() {
        return;
    }
    buffer_delete((*self_0).rx_buffer);
    buffer_delete((*self_0).tx_head_buffer);
    buffer_delete((*self_0).tx_body_buffer);
    fix_message_free((*self_0).rx_message);
    free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_time_update_monotonic(
    mut self_0: *mut fix_session,
    mut monotonic: *mut timespec,
) -> libc::c_int {
    (*self_0).now = *monotonic;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_time_update_realtime(
    mut self_0: *mut fix_session,
    mut realtime: *mut timespec,
) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tm: *mut tm = 0 as *mut tm;
    let mut fmt: [libc::c_char; 64] = [0; 64];
    tv.tv_sec = (*realtime).tv_sec;
    tv.tv_usec = (*realtime).tv_nsec / 1000 as libc::c_int as libc::c_long;
    tm = gmtime(&mut tv.tv_sec);
    if tm.is_null() {
        return -(1 as libc::c_int)
    } else {
        strftime(
            fmt.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%Y%m%d-%H:%M:%S\0" as *const u8 as *const libc::c_char,
            tm,
        );
        snprintf(
            ((*self_0).str_now).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%s.%03ld\0" as *const u8 as *const libc::c_char,
            fmt.as_mut_ptr(),
            tv.tv_usec / 1000 as libc::c_int as libc::c_long,
        );
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_time_update(
    mut self_0: *mut fix_session,
) -> libc::c_int {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if !(clock_gettime(1 as libc::c_int, &mut ts) != 0) {
        if !(fix_session_time_update_monotonic(self_0, &mut ts) != 0) {
            if !(clock_gettime(0 as libc::c_int, &mut ts) != 0) {
                return fix_session_time_update_realtime(self_0, &mut ts);
            }
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_send(
    mut self_0: *mut fix_session,
    mut msg: *mut fix_message,
    mut flags: libc::c_ulong,
) -> libc::c_int {
    (*msg).begin_string = (*self_0).begin_string;
    (*msg).sender_comp_id = (*self_0).sender_comp_id;
    (*msg).target_comp_id = (*self_0).target_comp_id;
    if !(flags != 0 && FIX_SEND_FLAG_PRESERVE_MSG_NUM as libc::c_int != 0) {
        let fresh0 = (*self_0).out_msg_seq_num;
        (*self_0).out_msg_seq_num = ((*self_0).out_msg_seq_num).wrapping_add(1);
        (*msg).msg_seq_num = fresh0;
    }
    (*msg).head_buf = (*self_0).tx_head_buffer;
    buffer_reset((*msg).head_buf);
    (*msg).body_buf = (*self_0).tx_body_buffer;
    buffer_reset((*msg).body_buf);
    (*self_0).tx_timestamp = (*self_0).now;
    (*msg).str_now = ((*self_0).str_now).as_mut_ptr();
    return fix_message_send(msg, (*self_0).sockfd, flags as libc::c_int);
}
#[inline]
unsafe extern "C" fn fix_session_buffer_full(mut session: *mut fix_session) -> bool {
    return buffer_remaining((*session).rx_buffer)
        <= (256 as libc::c_ulong).wrapping_add(1024 as libc::c_ulong);
}
unsafe extern "C" fn translate_recv_flags(mut flags: libc::c_ulong) -> libc::c_int {
    /*
    return if flags & FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong != 0 {
        MSG_DONTWAIT as libc::c_int
    } else {
        0 as libc::c_int
    };

     */
    0 as libc::c_int

}
#[no_mangle]
pub unsafe extern "C" fn fix_session_recv(
    mut self_0: *mut fix_session,
    mut res: *mut *mut fix_message,
    mut flags: libc::c_ulong,
) -> libc::c_int {
    let mut msg: *mut fix_message = (*self_0).rx_message;
    let mut buffer: *mut buffer = (*self_0).rx_buffer;
    (*self_0).failure_reason = FIX_SUCCESS;
    let mut size: size_t = 0;
    if fix_message_parse(msg, (*self_0).dialect, buffer, flags) == 0 {
        (*self_0).rx_timestamp = (*self_0).now;
        if flags & FIX_RECV_KEEP_IN_MSGSEQNUM as libc::c_int as libc::c_ulong == 0 {
            (*self_0).in_msg_seq_num = ((*self_0).in_msg_seq_num).wrapping_add(1);
            (*self_0).in_msg_seq_num;
        }
    } else {
        if fix_session_buffer_full(self_0) {
            buffer_compact(buffer);
        }
        size = buffer_remaining(buffer);
        if size > (256 as libc::c_ulong).wrapping_add(1024 as libc::c_ulong) {
            let mut nr: ssize_t = 0;
            size = (size as libc::c_ulong)
                .wrapping_sub((256 as libc::c_ulong).wrapping_add(1024 as libc::c_ulong))
                as size_t as size_t;
            nr = buffer_recv(
                buffer,
                (*self_0).sockfd,
                size,
                translate_recv_flags(flags),
            );
            if nr <= 0 as libc::c_int as libc::c_long {
                (*self_0)
                    .failure_reason = (if nr == 0 as libc::c_int as libc::c_long {
                    FIX_FAILURE_CONN_CLOSED as libc::c_int
                } else {
                    FIX_FAILURE_SYSTEM as libc::c_int
                }) as fix_failure_reason;
                return -(1 as libc::c_int);
            }
        }
        if fix_message_parse(msg, (*self_0).dialect, buffer, flags) == 0 {
            (*self_0).rx_timestamp = (*self_0).now;
            if flags & FIX_RECV_KEEP_IN_MSGSEQNUM as libc::c_int as libc::c_ulong == 0 {
                (*self_0).in_msg_seq_num = ((*self_0).in_msg_seq_num).wrapping_add(1);
                (*self_0).in_msg_seq_num;
            }
        } else {
            return 0 as libc::c_int
        }
    }
    *res = msg;
    return 1 as libc::c_int;
}
