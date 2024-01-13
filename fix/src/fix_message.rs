use core::slice;
use std::ffi::CStr;
use std::fmt::{Debug, Display, Formatter};
use std::mem::transmute;
use ::libc;
extern "C" {
    fn buffer_sum(self_0: *mut buffer) -> u8_0;
    fn buffer_sum_range(start: *const libc::c_char, end: *const libc::c_char) -> u8_0;
    static mut io_sendmsg: io_sendmsg_t;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn checksumtoa(n: libc::c_int, s: *mut libc::c_char) -> libc::c_int;
    fn i64toa(n: int64_t, s: *mut libc::c_char) -> libc::c_int;
    fn uitoa(n: libc::c_uint, s: *mut libc::c_char) -> libc::c_int;
    fn modp_dtoa2(
        value: libc::c_double,
        buf: *mut libc::c_char,
        precision: libc::c_int,
    ) -> size_t;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub start: libc::c_ulong,
    pub end: libc::c_ulong,
    pub capacity: libc::c_ulong,
    pub data: *mut libc::c_char,
}

impl Debug for buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let str = CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(buffer_start(self).cast(), ((self.end - self.start) + 1) as usize));
            write!(f, "{:?}", &str)
        }
    }
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
#[derive(Copy, Clone, Debug)]
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


pub type fix_parse_flag = libc::c_uint;
pub const FIX_PARSE_FLAG_NO_TYPE: fix_parse_flag = 2;
pub const FIX_PARSE_FLAG_NO_CSUM: fix_parse_flag = 1;
pub type u8_0 = uint8_t;
pub const FIX_SEND_FLAG_PRESERVE_BUFFER: fix_send_flag = 2;
pub type io_sendmsg_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut iovec, size_t, libc::c_int) -> ssize_t,
>;
pub type fix_send_flag = libc::c_uint;
pub const FIX_SEND_FLAG_PRESERVE_MSG_NUM: fix_send_flag = 1;
#[inline]
unsafe extern "C" fn fix_message_size(mut self_0: *mut fix_message) -> size_t {
    return ((*self_0).iov[0 as libc::c_int as usize].iov_len)
        .wrapping_add((*self_0).iov[1 as libc::c_int as usize].iov_len);
}
#[inline]
unsafe extern "C" fn buffer_put(mut self_0: *mut buffer, mut byte: libc::c_char) {
    let fresh0 = (*self_0).end;
    (*self_0).end = ((*self_0).end).wrapping_add(1);
    *((*self_0).data).offset(fresh0 as isize) = byte;
}
#[inline]
unsafe extern "C" fn buffer_end(mut self_0: *const buffer) -> *mut libc::c_char {
    return &mut *((*self_0).data).offset((*self_0).end as isize) as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn buffer_to_iovec(mut b: *mut buffer, mut iov: *mut iovec) {
    (*iov).iov_base = (*b).data as *mut libc::c_void;
    (*iov).iov_len = (*b).end;
}
#[inline]
unsafe extern "C" fn buffer_peek_8(mut self_0: *const buffer) -> u8_0 {
    return *((*self_0).data).offset((*self_0).start as isize) as u8_0;
}
#[inline]
unsafe extern "C" fn buffer_find(
    mut buf: *mut buffer,
    mut c: u8_0,
) -> *mut libc::c_char {
    while buffer_size(buf) != 0 {
        if buffer_peek_8(buf) as libc::c_int == c as libc::c_int {
            return buffer_start(buf);
        }
        buffer_advance(buf, 1 as libc::c_int as libc::c_long);
    }
    return 0 as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn buffer_advance(mut self_0: *mut buffer, mut n: libc::c_long) {
    (*self_0).start = ((*self_0).start).wrapping_add(n as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn buffer_start(mut self_0: *const buffer) -> *mut libc::c_char {
    return &mut *((*self_0).data).offset((*self_0).start as isize) as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn buffer_size(mut self_0: *const buffer) -> libc::c_ulong {
    return ((*self_0).end).wrapping_sub((*self_0).start);
}
#[no_mangle]
pub static mut fix_msg_types: [*const libc::c_char; 28] = [
    b"0\0" as *const u8 as *const libc::c_char,
    b"1\0" as *const u8 as *const libc::c_char,
    b"2\0" as *const u8 as *const libc::c_char,
    b"3\0" as *const u8 as *const libc::c_char,
    b"4\0" as *const u8 as *const libc::c_char,
    b"5\0" as *const u8 as *const libc::c_char,
    b"8\0" as *const u8 as *const libc::c_char,
    b"A\0" as *const u8 as *const libc::c_char,
    b"D\0" as *const u8 as *const libc::c_char,
    b"W\0" as *const u8 as *const libc::c_char,
    b"X\0" as *const u8 as *const libc::c_char,
    b"h\0" as *const u8 as *const libc::c_char,
    b"f\0" as *const u8 as *const libc::c_char,
    b"G\0" as *const u8 as *const libc::c_char,
    b"9\0" as *const u8 as *const libc::c_char,
    b"F\0" as *const u8 as *const libc::c_char,
    b"q\0" as *const u8 as *const libc::c_char,
    b"r\0" as *const u8 as *const libc::c_char,
    b"R\0" as *const u8 as *const libc::c_char,
    b"c\0" as *const u8 as *const libc::c_char,
    b"s\0" as *const u8 as *const libc::c_char,
    b"i\0" as *const u8 as *const libc::c_char,
    b"Z\0" as *const u8 as *const libc::c_char,
    b"d\0" as *const u8 as *const libc::c_char,
    b"b\0" as *const u8 as *const libc::c_char,
    b"AF\0" as *const u8 as *const libc::c_char,
    b"CA\0" as *const u8 as *const libc::c_char,
    b"BZ\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn fix_msg_type_parse(
    mut s: *const libc::c_char,
    delim: libc::c_char,
) -> fix_msg_type {
    if *s.offset(1 as libc::c_int as isize) as libc::c_int != delim as libc::c_int {
        if *s.offset(2 as libc::c_int as isize) as libc::c_int != delim as libc::c_int {
            return FIX_MSG_TYPE_UNKNOWN;
        }
        if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'A' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'F' as i32
        {
            return FIX_MSG_ORDER_MASS_STATUS_REQUEST
        } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'C' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'A' as i32
        {
            return FIX_MSG_ORDER_MASS_ACTION_REQUEST
        } else if *s.offset(0 as libc::c_int as isize) as libc::c_int == 'B' as i32
            && *s.offset(1 as libc::c_int as isize) as libc::c_int == 'Z' as i32
        {
            return FIX_MSG_ORDER_MASS_ACTION_REPORT
        } else {
            return FIX_MSG_TYPE_UNKNOWN
        }
    }
    match *s.offset(0 as libc::c_int as isize) as libc::c_int {
        48 => return FIX_MSG_TYPE_HEARTBEAT,
        49 => return FIX_MSG_TYPE_TEST_REQUEST,
        50 => return FIX_MSG_TYPE_RESEND_REQUEST,
        51 => return FIX_MSG_TYPE_REJECT,
        52 => return FIX_MSG_TYPE_SEQUENCE_RESET,
        53 => return FIX_MSG_TYPE_LOGOUT,
        56 => return FIX_MSG_TYPE_EXECUTION_REPORT,
        57 => return FIX_MSG_ORDER_CANCEL_REJECT,
        65 => return FIX_MSG_TYPE_LOGON,
        68 => return FIX_MSG_TYPE_NEW_ORDER_SINGLE,
        70 => return FIX_MSG_ORDER_CANCEL_REQUEST,
        71 => return FIX_MSG_ORDER_CANCEL_REPLACE,
        87 => return FIX_MSG_TYPE_SNAPSHOT_REFRESH,
        88 => return FIX_MSG_TYPE_INCREMENT_REFRESH,
        104 => return FIX_MSG_TYPE_SESSION_STATUS,
        102 => return FIX_MSG_TYPE_SECURITY_STATUS,
        113 => return FIX_MSG_ORDER_MASS_CANCEL_REQUEST,
        114 => return FIX_MSG_ORDER_MASS_CANCEL_REPORT,
        82 => return FIX_MSG_QUOTE_REQUEST,
        99 => return FIX_MSG_SECURITY_DEFINITION_REQUEST,
        115 => return FIX_MSG_NEW_ORDER_CROSS,
        105 => return FIX_MSG_MASS_QUOTE,
        90 => return FIX_MSG_QUOTE_CANCEL,
        100 => return FIX_MSG_SECURITY_DEFINITION,
        98 => return FIX_MSG_QUOTE_ACKNOWLEDGEMENT,
        _ => return FIX_MSG_TYPE_UNKNOWN,
    };
}
#[no_mangle]
pub unsafe extern "C" fn fix_atoi64(
    mut p: *const libc::c_char,
    mut end: *mut *const libc::c_char,
) -> int64_t {
    let mut ret: int64_t = 0 as libc::c_int as int64_t;
    let mut neg: bool = 0 as libc::c_int != 0;
    if *p as libc::c_int == '-' as i32 {
        neg = 1 as libc::c_int != 0;
        p = p.offset(1);
        p;
    }
    while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        ret = ret * 10 as libc::c_int as libc::c_long
            + (*p as libc::c_int - '0' as i32) as libc::c_long;
        p = p.offset(1);
        p;
    }
    if neg {
        ret = -ret;
    }
    if !end.is_null() {
        *end = p;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn fix_uatoi(
    mut p: *const libc::c_char,
    mut end: *mut *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        ret = ret * 10 as libc::c_int + (*p as libc::c_int - '0' as i32);
        p = p.offset(1);
        p;
    }
    if !end.is_null() {
        *end = p;
    }
    return ret;
}
unsafe extern "C" fn parse_tag(
    mut self_0: *mut buffer,
    mut tag: *mut libc::c_int,
) -> libc::c_int {
    let mut delim: *const libc::c_char = 0 as *const libc::c_char;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    start = buffer_start(self_0);
    delim = buffer_find(self_0, '=' as i32 as u8_0);
    if delim.is_null() || *delim as libc::c_int != '=' as i32 {
        return 1 as libc::c_int;
    }
    ret = fix_uatoi(start, &mut end);
    if end != delim {
        return 2 as libc::c_int;
    }
    buffer_advance(self_0, 1 as libc::c_int as libc::c_long);
    *tag = ret;
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_value(
    mut self_0: *mut buffer,
    mut value: *mut *const libc::c_char,
) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    start = buffer_start(self_0);
    end = buffer_find(self_0, 0x1 as libc::c_int as u8_0);
    if end.is_null() || *end as libc::c_int != 0x1 as libc::c_int {
        return 1 as libc::c_int;
    }
    buffer_advance(self_0, 1 as libc::c_int as libc::c_long);
    *value = start;
    return 0 as libc::c_int;
}
unsafe extern "C" fn next_tag(mut self_0: *mut buffer) {
    let mut delim: *mut libc::c_char = 0 as *mut libc::c_char;
    delim = buffer_find(self_0, 0x1 as libc::c_int as u8_0);
    if delim.is_null() {
        return;
    }
    if *delim as libc::c_int != 0x1 as libc::c_int {
        return;
    }
    buffer_advance(self_0, 1 as libc::c_int as libc::c_long);
}
unsafe extern "C" fn match_field(
    mut self_0: *mut buffer,
    mut tag: libc::c_int,
    mut value: *mut *const libc::c_char,
) -> libc::c_int {
    let mut ptag: libc::c_int = 0;
    let mut ret: libc::c_int = 0;

    ret = parse_tag(self_0, &mut ptag);
    if !(ret != 0) {
        if ptag != tag {
            ret = 2 as libc::c_int;
        } else {
            return parse_value(self_0, value)
        }
    }
    next_tag(self_0);
    return ret;
}
unsafe extern "C" fn parse_field(
    mut self_0: *mut buffer,
    mut tag: *mut libc::c_int,
    mut value: *mut *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = parse_tag(self_0, tag);
    if ret != 0 {
        next_tag(self_0);
        return ret;
    } else {
        return parse_value(self_0, value)
    };
}
unsafe extern "C" fn fix_tag_type(mut tag: libc::c_int) -> fix_type {
    match tag {
        10 => return FIX_TYPE_CHECKSUM,
        369 => return FIX_TYPE_INT,
        1023 => return FIX_TYPE_INT,
        7 => return FIX_TYPE_INT,
        45 => return FIX_TYPE_INT,
        16 => return FIX_TYPE_INT,
        36 => return FIX_TYPE_INT,
        83 => return FIX_TYPE_INT,
        123 => return FIX_TYPE_STRING,
        43 => return FIX_TYPE_STRING,
        48 => return FIX_TYPE_STRING,
        112 => return FIX_TYPE_STRING,
        34 => return FIX_TYPE_MSGSEQNUM,
        271 => return FIX_TYPE_FLOAT,
        32 => return FIX_TYPE_FLOAT,
        151 => return FIX_TYPE_FLOAT,
        270 => return FIX_TYPE_FLOAT,
        38 => return FIX_TYPE_FLOAT,
        14 => return FIX_TYPE_FLOAT,
        31 => return FIX_TYPE_FLOAT,
        6 => return FIX_TYPE_FLOAT,
        44 => return FIX_TYPE_FLOAT,
        336 => return FIX_TYPE_STRING,
        279 => return FIX_TYPE_STRING,
        60 => return FIX_TYPE_STRING,
        20 => return FIX_TYPE_STRING,
        41 => return FIX_TYPE_STRING,
        269 => return FIX_TYPE_STRING,
        39 => return FIX_TYPE_STRING,
        150 => return FIX_TYPE_STRING,
        554 => return FIX_TYPE_STRING,
        1 => return FIX_TYPE_STRING,
        11 => return FIX_TYPE_STRING,
        37 => return FIX_TYPE_STRING,
        40 => return FIX_TYPE_STRING,
        17 => return FIX_TYPE_STRING,
        55 => return FIX_TYPE_STRING,
        54 => return FIX_TYPE_STRING,
        58 => return FIX_TYPE_STRING,
        103 => return FIX_TYPE_INT,
        442 => return FIX_TYPE_CHAR,
        _ => return FIX_TYPE_STRING,
    };
}
unsafe extern "C" fn rest_of_message(
    mut self_0: *mut fix_message,
    mut dialect: *mut fix_dialect,
    mut buffer: *mut buffer,
) {
    let mut tag: libc::c_int = 0 as libc::c_int;
    let mut tag_ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut nr_fields: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut type_0: fix_type = FIX_TYPE_INT;
    (*self_0).nr_fields = 0 as libc::c_int as libc::c_ulong;
    loop {
        if parse_field(buffer, &mut tag, &mut tag_ptr) != 0 {
            return;
        }
        type_0 = ((*dialect).tag_type).expect("non-null function pointer")(tag);
        match type_0 as libc::c_uint {
            0 => {
                let fresh1 = nr_fields;
                nr_fields = nr_fields.wrapping_add(1);
                *((*self_0).fields)
                    .offset(
                        fresh1 as isize,
                    ) = {
                    let mut init = fix_field {
                        tag: tag,
                        type_0: FIX_TYPE_INT,
                        c2rust_unnamed: C2RustUnnamed {
                            int_value: fix_atoi64(tag_ptr, 0 as *mut *const libc::c_char),
                        },
                    };
                    init
                };
            }
            1 => {
                let fresh2 = nr_fields;
                nr_fields = nr_fields.wrapping_add(1);
                *((*self_0).fields)
                    .offset(
                        fresh2 as isize,
                    ) = {
                    let mut init = fix_field {
                        tag: tag,
                        type_0: FIX_TYPE_FLOAT,
                        c2rust_unnamed: C2RustUnnamed {
                            float_value: strtod(tag_ptr, 0 as *mut *mut libc::c_char),
                        },
                    };
                    init
                };
            }
            2 => {
                let fresh3 = nr_fields;
                nr_fields = nr_fields.wrapping_add(1);
                *((*self_0).fields)
                    .offset(
                        fresh3 as isize,
                    ) = {
                    let mut init = fix_field {
                        tag: tag,
                        type_0: FIX_TYPE_CHAR,
                        c2rust_unnamed: C2RustUnnamed {
                            char_value: *tag_ptr.offset(0 as libc::c_int as isize),
                        },
                    };
                    init
                };
            }
            3 => {
                let fresh4 = nr_fields;
                nr_fields = nr_fields.wrapping_add(1);
                *((*self_0).fields)
                    .offset(
                        fresh4 as isize,
                    ) = {
                    let mut init = fix_field {
                        tag: tag,
                        type_0: FIX_TYPE_STRING,
                        c2rust_unnamed: C2RustUnnamed {
                            string_value: tag_ptr,
                        },
                    };
                    init
                };
            }
            4 => {
                break;
            }
            5 => {
                (*self_0)
                    .msg_seq_num = fix_uatoi(tag_ptr, 0 as *mut *const libc::c_char)
                    as libc::c_ulong;
            }
            _ => {}
        }
    }
    (*self_0).nr_fields = nr_fields;
}
unsafe extern "C" fn verify_checksum(
    mut self_0: *mut fix_message,
    mut buffer: *mut buffer,
) -> bool {
    let mut cksum: libc::c_int = 0;
    let mut actual: libc::c_int = 0;
    cksum = fix_uatoi((*self_0).check_sum, 0 as *mut *const libc::c_char);
    actual = buffer_sum_range(
        ((*self_0).begin_string).offset(-(2 as libc::c_int as isize)),
        ((*self_0).check_sum).offset(-(3 as libc::c_int as isize)),
    ) as libc::c_int;
    return actual == cksum;
}
unsafe extern "C" fn checksum(
    mut self_0: *mut fix_message,
    mut buffer: *mut buffer,
    mut flags: libc::c_ulong,
) -> libc::c_int {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut offset: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    start = buffer_start(buffer);
    offset = start.offset_from(((*self_0).msg_type).offset(-(3 as libc::c_int as isize)))
        as libc::c_long as libc::c_int;
    if (buffer_size(buffer)).wrapping_add(offset as libc::c_ulong)
        < ((*self_0).body_length).wrapping_add(7 as libc::c_int as libc::c_ulong)
    {
        ret = 1 as libc::c_int;
    } else if flags & FIX_PARSE_FLAG_NO_CSUM as libc::c_int as libc::c_ulong != 0 {
        ret = 0 as libc::c_int;
    } else {
        buffer_advance(
            buffer,
            ((*self_0).body_length).wrapping_sub(offset as libc::c_ulong) as libc::c_long,
        );
        ret = match_field(buffer, CheckSum as libc::c_int, &mut (*self_0).check_sum);
        if !(ret != 0) {
            if !verify_checksum(self_0, buffer) {
                ret = 2 as libc::c_int;
            } else {
                buffer_advance(
                    buffer,
                    start.offset_from(buffer_start(buffer)) as libc::c_long,
                );
            }
        }
    }
    return ret;
}
unsafe extern "C" fn parse_msg_type(
    mut self_0: *mut fix_message,
    mut flags: libc::c_ulong,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = match_field(
        (*self_0).head_buf,
        MsgType as libc::c_int,
        &mut (*self_0).msg_type,
    );
    if !(ret != 0) {
        if flags & FIX_PARSE_FLAG_NO_TYPE as libc::c_int as libc::c_ulong == 0 {
            (*self_0)
                .type_0 = fix_msg_type_parse(
                (*self_0).msg_type,
                0x1 as libc::c_int as libc::c_char,
            );
            if fix_message_type_is(self_0, FIX_MSG_TYPE_UNKNOWN) {
                ret = 2 as libc::c_int;
            }
        } else {
            (*self_0).type_0 = FIX_MSG_TYPE_UNKNOWN;
        }
    }
    return ret;
}
unsafe extern "C" fn parse_body_length(mut self_0: *mut fix_message) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    ret = match_field((*self_0).head_buf, BodyLength as libc::c_int, &mut ptr);
    if !(ret != 0) {
        len = fix_uatoi(ptr, 0 as *mut *const libc::c_char);
        (*self_0).body_length = len as libc::c_ulong;
        if len <= 0 as libc::c_int
            || len as libc::c_ulong
                > (256 as libc::c_ulong).wrapping_add(1024 as libc::c_ulong)
        {
            ret = 2 as libc::c_int;
        }
    }
    return ret;
}
unsafe extern "C" fn parse_begin_string(mut self_0: *mut fix_message) -> libc::c_int {
    return match_field(
        (*self_0).head_buf,
        BeginString as libc::c_int,
        &mut (*self_0).begin_string,
    );
}
unsafe extern "C" fn first_three_fields(
    mut self_0: *mut fix_message,
    mut flags: libc::c_ulong,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    ret = parse_begin_string(self_0);
    if !(ret != 0) {
        ret = parse_body_length(self_0);
        if !(ret != 0) {
            return parse_msg_type(self_0, flags);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fix_message_parse(
    mut self_0: *mut fix_message,
    mut dialect: *mut fix_dialect,
    mut buffer: *mut buffer,
    mut flags: libc::c_ulong,
) -> libc::c_int {
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0;
    (*self_0).head_buf = buffer;
    loop {
        ret = 1 as libc::c_int;
        start = buffer_start(buffer);
        if !(buffer_size(buffer) == 0) {
            ret = first_three_fields(self_0, flags);
            if !(ret != 0) {
                ret = checksum(self_0, buffer, flags);
                if !(ret != 0) {
                    rest_of_message(self_0, dialect, buffer);
                    (*self_0)
                        .iov[0 as libc::c_int as usize]
                        .iov_base = start as *mut libc::c_void;
                    (*self_0)
                        .iov[0 as libc::c_int as usize]
                        .iov_len = (buffer_start(buffer)).offset_from(start)
                        as libc::c_long as size_t;
                    return 0 as libc::c_int;
                }
            }
        }
        if ret != 1 as libc::c_int {
            continue;
        }
        buffer_advance(buffer, start.offset_from(buffer_start(buffer)) as libc::c_long);
        return -(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fix_get_field_count(
    mut self_0: *mut fix_message,
) -> libc::c_int {
    return (*self_0).nr_fields as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fix_get_field_at(
    mut self_0: *mut fix_message,
    mut i: libc::c_int,
) -> *mut fix_field {
    return if (i as libc::c_ulong) < (*self_0).nr_fields {
        &mut *((*self_0).fields).offset(i as isize) as *mut fix_field
    } else {
        0 as *mut fix_field
    };
}
#[no_mangle]
pub unsafe extern "C" fn fix_get_field(
    mut self_0: *mut fix_message,
    mut tag: libc::c_int,
) -> *mut fix_field {
    let mut i: libc::c_ulong = 0;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (*self_0).nr_fields {
        if (*((*self_0).fields).offset(i as isize)).tag == tag {
            return &mut *((*self_0).fields).offset(i as isize) as *mut fix_field;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut fix_field;
}
#[no_mangle]
pub unsafe extern "C" fn fix_message_validate(mut self_0: *mut fix_message) {}
#[no_mangle]
pub unsafe extern "C" fn fix_get_string(
    mut field: *mut fix_field,
    mut buffer: *mut libc::c_char,
    mut len: libc::c_ulong,
) -> *const libc::c_char {
    let mut count: libc::c_ulong = 0;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    start = (*field).c2rust_unnamed.string_value;
    end = memchr(start as *const libc::c_void, 0x1 as libc::c_int, len)
        as *const libc::c_char;
    if end.is_null() {
        return 0 as *const libc::c_char;
    }
    count = end.offset_from(start) as libc::c_long as libc::c_ulong;
    if len < count {
        return 0 as *const libc::c_char;
    }
    strncpy(buffer, start, count);
    *buffer.offset(count as isize) = '\0' as i32 as libc::c_char;
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn fix_get_float(
    mut self_0: *mut fix_message,
    mut tag: libc::c_int,
    mut _default_: libc::c_double,
) -> libc::c_double {
    let mut field: *mut fix_field = fix_get_field(self_0, tag);
    return if !field.is_null() {
        (*field).c2rust_unnamed.float_value
    } else {
        _default_
    };
}
#[no_mangle]
pub unsafe extern "C" fn fix_get_int(
    mut self_0: *mut fix_message,
    mut tag: libc::c_int,
    mut _default_: int64_t,
) -> int64_t {
    let mut field: *mut fix_field = fix_get_field(self_0, tag);
    return if !field.is_null() { (*field).c2rust_unnamed.int_value } else { _default_ };
}
#[no_mangle]
pub unsafe extern "C" fn fix_get_char(
    mut self_0: *mut fix_message,
    mut tag: libc::c_int,
    mut _default_: libc::c_char,
) -> libc::c_char {
    let mut field: *mut fix_field = fix_get_field(self_0, tag);
    return (if !field.is_null() {
        (*field).c2rust_unnamed.char_value as libc::c_int
    } else {
        _default_ as libc::c_int
    }) as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn fix_message_new() -> *mut fix_message {
    let mut self_0: *mut fix_message = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<fix_message>() as libc::c_ulong,
    ) as *mut fix_message;
    if self_0.is_null() {
        return 0 as *mut fix_message;
    }
    (*self_0)
        .fields = calloc(
        48 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<fix_field>() as libc::c_ulong,
    ) as *mut fix_field;
    if ((*self_0).fields).is_null() {
        fix_message_free(self_0);
        return 0 as *mut fix_message;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn fix_message_free(mut self_0: *mut fix_message) {
    if self_0.is_null() {
        return;
    }
    free((*self_0).fields as *mut libc::c_void);
    free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fix_message_add_field(
    mut self_0: *mut fix_message,
    mut field: *mut fix_field,
) {
    if (*self_0).nr_fields < 48 as libc::c_int as libc::c_ulong {
        *((*self_0).fields).offset((*self_0).nr_fields as isize) = *field;
        (*self_0).nr_fields = ((*self_0).nr_fields).wrapping_add(1);
        (*self_0).nr_fields;
    }
}
#[no_mangle]
pub unsafe extern "C" fn fix_message_type_is(
    mut self_0: *mut fix_message,
    mut type_0: fix_msg_type,
) -> bool {
    return (*self_0).type_0 as libc::c_ulong == type_0 as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn fix_field_unparse(
    mut self_0: *mut fix_field,
    mut buffer: *mut buffer,
) -> bool {
    (*buffer)
        .end = ((*buffer).end)
        .wrapping_add(
            uitoa((*self_0).tag as libc::c_uint, buffer_end(buffer)) as libc::c_ulong,
        );
    buffer_put(buffer, '=' as i32 as libc::c_char);
    match (*self_0).type_0 as libc::c_uint {
        3 => {
            let mut p: *const libc::c_char = (*self_0).c2rust_unnamed.string_value;
            while *p != 0 {
                let fresh5 = p;
                p = p.offset(1);
                buffer_put(buffer, *fresh5);
            }
        }
        6 => {
            let mut i: libc::c_int = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong
                && (*self_0).c2rust_unnamed.string_8_value[i as usize] as libc::c_int
                    != 0
            {
                buffer_put(buffer, (*self_0).c2rust_unnamed.string_8_value[i as usize]);
                i += 1;
                i;
            }
        }
        2 => {
            buffer_put(buffer, (*self_0).c2rust_unnamed.char_value);
        }
        1 => {
            (*buffer)
                .end = ((*buffer).end)
                .wrapping_add(
                    modp_dtoa2(
                        (*self_0).c2rust_unnamed.float_value,
                        buffer_end(buffer),
                        7 as libc::c_int,
                    ),
                );
        }
        0 => {
            (*buffer)
                .end = ((*buffer).end)
                .wrapping_add(
                    i64toa((*self_0).c2rust_unnamed.int_value, buffer_end(buffer))
                        as libc::c_ulong,
                );
        }
        4 => {
            (*buffer)
                .end = ((*buffer).end)
                .wrapping_add(
                    checksumtoa(
                        (*self_0).c2rust_unnamed.int_value as libc::c_int,
                        buffer_end(buffer),
                    ) as libc::c_ulong,
                );
        }
        _ => {}
    }
    buffer_put(buffer, 0x1 as libc::c_int as libc::c_char);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn fix_message_unparse(mut self_0: *mut fix_message) {
    let mut sender_comp_id: fix_field = fix_field {
        tag: 0,
        type_0: FIX_TYPE_INT,
        c2rust_unnamed: C2RustUnnamed { int_value: 0 },
    };
    let mut target_comp_id: fix_field = fix_field {
        tag: 0,
        type_0: FIX_TYPE_INT,
        c2rust_unnamed: C2RustUnnamed { int_value: 0 },
    };
    let mut begin_string: fix_field = fix_field {
        tag: 0,
        type_0: FIX_TYPE_INT,
        c2rust_unnamed: C2RustUnnamed { int_value: 0 },
    };
    let mut sending_time: fix_field = fix_field {
        tag: 0,
        type_0: FIX_TYPE_INT,
        c2rust_unnamed: C2RustUnnamed { int_value: 0 },
    };
    let mut body_length: fix_field = fix_field {
        tag: 0,
        type_0: FIX_TYPE_INT,
        c2rust_unnamed: C2RustUnnamed { int_value: 0 },
    };
    let mut msg_seq_num: fix_field = fix_field {
        tag: 0,
        type_0: FIX_TYPE_INT,
        c2rust_unnamed: C2RustUnnamed { int_value: 0 },
    };
    let mut check_sum: fix_field = fix_field {
        tag: 0,
        type_0: FIX_TYPE_INT,
        c2rust_unnamed: C2RustUnnamed { int_value: 0 },
    };
    let mut msg_type: fix_field = fix_field {
        tag: 0,
        type_0: FIX_TYPE_INT,
        c2rust_unnamed: C2RustUnnamed { int_value: 0 },
    };
    let mut cksum: libc::c_ulong = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    strncpy(
        buf.as_mut_ptr(),
        (*self_0).str_now,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    msg_type = if (*self_0).type_0 as libc::c_ulong
        != FIX_MSG_TYPE_UNKNOWN as libc::c_ulong
    {
        {
            let mut init = fix_field {
                tag: MsgType as libc::c_int,
                type_0: FIX_TYPE_STRING,
                c2rust_unnamed: C2RustUnnamed {
                    string_value: fix_msg_types[(*self_0).type_0 as usize],
                },
            };
            init
        }
    } else {
        let mut init = fix_field {
            tag: MsgType as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: (*self_0).msg_type,
            },
        };
        init
    };
    sender_comp_id = {
        let mut init = fix_field {
            tag: SenderCompID as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: (*self_0).sender_comp_id,
            },
        };
        init
    };
    target_comp_id = {
        let mut init = fix_field {
            tag: TargetCompID as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: (*self_0).target_comp_id,
            },
        };
        init
    };
    msg_seq_num = {
        let mut init = fix_field {
            tag: MsgSeqNum as libc::c_int,
            type_0: FIX_TYPE_INT,
            c2rust_unnamed: C2RustUnnamed {
                int_value: (*self_0).msg_seq_num as int64_t,
            },
        };
        init
    };
    sending_time = {
        let mut init = fix_field {
            tag: SendingTime as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: buf.as_mut_ptr(),
            },
        };
        init
    };
    fix_field_unparse(&mut msg_type, (*self_0).body_buf);
    fix_field_unparse(&mut sender_comp_id, (*self_0).body_buf);
    fix_field_unparse(&mut target_comp_id, (*self_0).body_buf);
    fix_field_unparse(&mut msg_seq_num, (*self_0).body_buf);
    fix_field_unparse(&mut sending_time, (*self_0).body_buf);
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*self_0).nr_fields {
        fix_field_unparse(
            &mut *((*self_0).fields).offset(i as isize),
            (*self_0).body_buf,
        );
        i += 1;
        i;
    }
    begin_string = {
        let mut init = fix_field {
            tag: BeginString as libc::c_int,
            type_0: FIX_TYPE_STRING,
            c2rust_unnamed: C2RustUnnamed {
                string_value: (*self_0).begin_string,
            },
        };
        init
    };
    body_length = {
        let mut init = fix_field {
            tag: BodyLength as libc::c_int,
            type_0: FIX_TYPE_INT,
            c2rust_unnamed: C2RustUnnamed {
                int_value: buffer_size((*self_0).body_buf) as int64_t,
            },
        };
        init
    };
    fix_field_unparse(&mut begin_string, (*self_0).head_buf);
    fix_field_unparse(&mut body_length, (*self_0).head_buf);
    cksum = (buffer_sum((*self_0).head_buf) as libc::c_int
        + buffer_sum((*self_0).body_buf) as libc::c_int) as libc::c_ulong;
    check_sum = {
        let mut init = fix_field {
            tag: CheckSum as libc::c_int,
            type_0: FIX_TYPE_CHECKSUM,
            c2rust_unnamed: C2RustUnnamed {
                int_value: cksum.wrapping_rem(256 as libc::c_int as libc::c_ulong)
                    as int64_t,
            },
        };
        init
    };
    fix_field_unparse(&mut check_sum, (*self_0).body_buf);
}
#[no_mangle]
pub unsafe extern "C" fn fix_message_send(
    mut self_0: *mut fix_message,
    mut sockfd: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut msg_size: size_t = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if flags & FIX_SEND_FLAG_PRESERVE_BUFFER as libc::c_int == 0 {
        fix_message_unparse(self_0);
    }
    buffer_to_iovec(
        (*self_0).head_buf,
        &mut *((*self_0).iov).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    buffer_to_iovec(
        (*self_0).body_buf,
        &mut *((*self_0).iov).as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    ret = io_sendmsg
        .expect(
            "non-null function pointer",
        )(
        sockfd,
        ((*self_0).iov).as_mut_ptr(),
        2 as libc::c_int as size_t,
        0 as libc::c_int,
    ) as libc::c_int;
    msg_size = fix_message_size(self_0);
    if flags & FIX_SEND_FLAG_PRESERVE_BUFFER as libc::c_int == 0 {
        (*self_0).body_buf = 0 as *mut buffer;
        (*self_0).head_buf = (*self_0).body_buf;
    }
    if ret >= 0 as libc::c_int {
        return msg_size.wrapping_sub(ret as libc::c_ulong) as libc::c_int
    } else {
        return ret
    };
}
#[no_mangle]
pub static mut fix_dialects: [fix_dialect; 7] = unsafe {
    [
        {
            let mut init = fix_dialect {
                version: FIX_4_0,
                tag_type: Some(
                    fix_tag_type as unsafe extern "C" fn(libc::c_int) -> fix_type,
                ),
            };
            init
        },
        {
            let mut init = fix_dialect {
                version: FIX_4_1,
                tag_type: Some(
                    fix_tag_type as unsafe extern "C" fn(libc::c_int) -> fix_type,
                ),
            };
            init
        },
        {
            let mut init = fix_dialect {
                version: FIX_4_2,
                tag_type: Some(
                    fix_tag_type as unsafe extern "C" fn(libc::c_int) -> fix_type,
                ),
            };
            init
        },
        {
            let mut init = fix_dialect {
                version: FIX_4_3,
                tag_type: Some(
                    fix_tag_type as unsafe extern "C" fn(libc::c_int) -> fix_type,
                ),
            };
            init
        },
        {
            let mut init = fix_dialect {
                version: FIX_4_4,
                tag_type: Some(
                    fix_tag_type as unsafe extern "C" fn(libc::c_int) -> fix_type,
                ),
            };
            init
        },
        fix_dialect {
            version: FIX_4_0,
            tag_type: None,
        },
        {
            let mut init = fix_dialect {
                version: FIXT_1_1,
                tag_type: Some(
                    fix_tag_type as unsafe extern "C" fn(libc::c_int) -> fix_type,
                ),
            };
            init
        },
    ]
};
