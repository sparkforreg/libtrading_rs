use ::libc;
extern "C" {
    pub type buffer;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fix_msg_type_parse(s: *const libc::c_char, delim: libc::c_char) -> fix_msg_type;
    fn fix_get_field(self_0: *mut fix_message, tag: libc::c_int) -> *mut fix_field;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
}
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
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
pub type fix_type = libc::c_uint;
pub const FIX_TYPE_STRING_8: fix_type = 6;
pub const FIX_TYPE_MSGSEQNUM: fix_type = 5;
pub const FIX_TYPE_CHECKSUM: fix_type = 4;
pub const FIX_TYPE_STRING: fix_type = 3;
pub const FIX_TYPE_CHAR: fix_type = 2;
pub const FIX_TYPE_FLOAT: fix_type = 1;
pub const FIX_TYPE_INT: fix_type = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct felem {
    pub msg: fix_message,
    pub buf: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fcontainer {
    pub nr: libc::c_ulong,
    pub cur: libc::c_ulong,
    pub felems: [felem; 32],
}
#[no_mangle]
pub unsafe extern "C" fn fcontainer_new() -> *mut fcontainer {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut container: *mut fcontainer = 0 as *mut fcontainer;
    container = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<fcontainer>() as libc::c_ulong,
    ) as *mut fcontainer;
    if !container.is_null() {
        i = 0 as libc::c_int;
        loop {
            if !(i < 32 as libc::c_int) {
                current_block = 6483416627284290920;
                break;
            }
            (*container)
                .felems[i as usize]
                .msg
                .fields = calloc(
                48 as libc::c_int as libc::c_ulong,
                ::core::mem::size_of::<fix_field>() as libc::c_ulong,
            ) as *mut fix_field;
            if ((*container).felems[i as usize].msg.fields).is_null() {
                current_block = 1086159631003291120;
                break;
            }
            i += 1;
            i;
        }
        match current_block {
            1086159631003291120 => {}
            _ => return container,
        }
    }
    fcontainer_free(container);
    return 0 as *mut fcontainer;
}
#[no_mangle]
pub unsafe extern "C" fn fcontainer_free(mut container: *mut fcontainer) {
    let mut i: libc::c_int = 0;
    if container.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        free((*container).felems[i as usize].msg.fields as *mut libc::c_void);
        i += 1;
        i;
    }
    free(container as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn cur_elem(mut self_0: *mut fcontainer) -> *mut felem {
    if (*self_0).cur < (*self_0).nr {
        return &mut *((*self_0).felems).as_mut_ptr().offset((*self_0).cur as isize)
            as *mut felem
    } else {
        return 0 as *mut felem
    };
}
#[no_mangle]
pub unsafe extern "C" fn next_elem(mut self_0: *mut fcontainer) -> *mut felem {
    (*self_0).cur = ((*self_0).cur).wrapping_add(1);
    (*self_0).cur;
    return cur_elem(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn add_elem(mut self_0: *mut fcontainer) -> *mut felem {
    if (*self_0).nr < 32 as libc::c_int as libc::c_ulong {
        let fresh0 = (*self_0).nr;
        (*self_0).nr = ((*self_0).nr).wrapping_add(1);
        return &mut *((*self_0).felems).as_mut_ptr().offset(fresh0 as isize)
            as *mut felem;
    } else {
        return 0 as *mut felem
    };
}
#[no_mangle]
pub unsafe extern "C" fn init_elem(
    mut elem: *mut felem,
    mut line: *mut libc::c_char,
) -> libc::c_int {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag: libc::c_int = 0;
    if elem.is_null() {
        return 1 as libc::c_int;
    }
    start = strcpy(((*elem).buf).as_mut_ptr(), line);
    while !(*start as libc::c_int == 0 as libc::c_int
        || *start as libc::c_int == 0xa as libc::c_int)
    {
        tag = strtol(start, &mut end, 10 as libc::c_int) as libc::c_int;
        end = end.offset(1);
        end;
        match tag {
            98 | 108 | 7 | 36 | 16 => {
                let fresh1 = (*elem).msg.nr_fields;
                (*elem).msg.nr_fields = ((*elem).msg.nr_fields).wrapping_add(1);
                *((*elem).msg.fields)
                    .offset(
                        fresh1 as isize,
                    ) = {
                    let mut init = fix_field {
                        tag: tag,
                        type_0: FIX_TYPE_INT,
                        c2rust_unnamed: C2RustUnnamed {
                            int_value: strtol(
                                end,
                                0 as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            ),
                        },
                    };
                    init
                };
            }
            141 | 123 | 52 | 112 | 58 => {
                let fresh2 = (*elem).msg.nr_fields;
                (*elem).msg.nr_fields = ((*elem).msg.nr_fields).wrapping_add(1);
                *((*elem).msg.fields)
                    .offset(
                        fresh2 as isize,
                    ) = {
                    let mut init = fix_field {
                        tag: tag,
                        type_0: FIX_TYPE_STRING,
                        c2rust_unnamed: C2RustUnnamed { string_value: end },
                    };
                    init
                };
            }
            34 => {
                (*elem)
                    .msg
                    .msg_seq_num = strtol(
                    end,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_ulong;
            }
            9 => {
                (*elem)
                    .msg
                    .body_length = strtol(
                    end,
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                ) as libc::c_ulong;
            }
            49 => {
                (*elem).msg.sender_comp_id = end;
            }
            56 => {
                (*elem).msg.target_comp_id = end;
            }
            8 => {
                (*elem).msg.begin_string = end;
            }
            10 => {
                (*elem).msg.check_sum = end;
            }
            35 => {
                (*elem)
                    .msg
                    .type_0 = fix_msg_type_parse(
                    end,
                    0x1 as libc::c_int as libc::c_char,
                );
                (*elem).msg.msg_type = end;
            }
            _ => return 1 as libc::c_int,
        }
        while *end as libc::c_int != 0 as libc::c_int
            && *end as libc::c_int != 0x1 as libc::c_int
            && *end as libc::c_int != 0xa as libc::c_int
        {
            end = end.offset(1);
            end;
        }
        if *end as libc::c_int == 0 as libc::c_int
            || *end as libc::c_int == 0xa as libc::c_int
        {
            *end = 0 as libc::c_int as libc::c_char;
            break;
        } else {
            *end = 0 as libc::c_int as libc::c_char;
            start = end.offset(1 as libc::c_int as isize);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn script_read(
    mut stream: *mut FILE,
    mut server: *mut fcontainer,
    mut client: *mut fcontainer,
) -> libc::c_int {
    let mut line: [libc::c_char; 256] = [0; 256];
    let mut size: libc::c_int = 256 as libc::c_int;
    line[(size - 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    while !(fgets(line.as_mut_ptr(), size, stream)).is_null() {
        if line[(size - 1 as libc::c_int) as usize] != 0 {
            return 1 as libc::c_int;
        }
        match line[0 as libc::c_int as usize] as libc::c_int {
            99 | 67 => {
                if init_elem(
                    add_elem(client),
                    line.as_mut_ptr().offset(1 as libc::c_int as isize),
                ) != 0
                {
                    return 1 as libc::c_int;
                }
            }
            115 | 83 => {
                if init_elem(
                    add_elem(server),
                    line.as_mut_ptr().offset(1 as libc::c_int as isize),
                ) != 0
                {
                    return 1 as libc::c_int;
                }
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fstrcmp(
    mut expected: *const libc::c_char,
    mut actual: *const libc::c_char,
) -> libc::c_int {
    let mut delim: *const libc::c_char = 0 as *const libc::c_char;
    if expected.is_null() {
        return 0 as libc::c_int;
    }
    delim = actual.offset(strlen(expected) as isize);
    if *delim as libc::c_int != 0x1 as libc::c_int {
        return 1 as libc::c_int;
    }
    return (strncmp(expected, actual, strlen(expected)) != 0 as libc::c_int)
        as libc::c_int;
}
unsafe extern "C" fn fstrcasecmp(
    mut expected: *const libc::c_char,
    mut actual: *const libc::c_char,
) -> libc::c_int {
    let mut delim: *const libc::c_char = 0 as *const libc::c_char;
    if expected.is_null() {
        return 0 as libc::c_int;
    }
    delim = actual.offset(strlen(expected) as isize);
    if *delim as libc::c_int != 0x1 as libc::c_int {
        return 1 as libc::c_int;
    }
    return (strncasecmp(expected, actual, strlen(expected)) != 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fmsgcmp(
    mut expected: *mut fix_message,
    mut actual: *mut fix_message,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut actual_field: *mut fix_field = 0 as *mut fix_field;
    let mut expected_field: *mut fix_field = 0 as *mut fix_field;
    if !((*expected).body_length != 0
        && (*expected).body_length != (*actual).body_length)
    {
        if !((*expected).msg_seq_num != 0
            && (*expected).msg_seq_num != (*actual).msg_seq_num)
        {
            if !(fstrcmp((*expected).sender_comp_id, (*actual).sender_comp_id) != 0) {
                if !(fstrcmp((*expected).target_comp_id, (*actual).target_comp_id) != 0)
                {
                    if !(fstrcmp((*expected).begin_string, (*actual).begin_string) != 0)
                    {
                        if !(fstrcmp((*expected).msg_type, (*actual).msg_type) != 0) {
                            i = 0 as libc::c_int;
                            loop {
                                if !((i as libc::c_ulong) < (*expected).nr_fields) {
                                    current_block = 4495394744059808450;
                                    break;
                                }
                                expected_field = &mut *((*expected).fields)
                                    .offset(i as isize) as *mut fix_field;
                                actual_field = fix_get_field(actual, (*expected_field).tag);
                                if actual_field.is_null() {
                                    current_block = 11850672490900767948;
                                    break;
                                }
                                match (*expected_field).type_0 as libc::c_uint {
                                    3 => {
                                        if fstrcmp(
                                            (*expected_field).c2rust_unnamed.string_value,
                                            (*actual_field).c2rust_unnamed.string_value,
                                        ) != 0
                                        {
                                            current_block = 11850672490900767948;
                                            break;
                                        }
                                    }
                                    1 => {
                                        if (*expected_field).c2rust_unnamed.float_value
                                            != (*actual_field).c2rust_unnamed.float_value
                                        {
                                            current_block = 11850672490900767948;
                                            break;
                                        }
                                    }
                                    2 => {
                                        if fstrcasecmp(
                                            &mut (*expected_field).c2rust_unnamed.char_value,
                                            &mut (*actual_field).c2rust_unnamed.char_value,
                                        ) != 0
                                        {
                                            current_block = 11850672490900767948;
                                            break;
                                        }
                                    }
                                    4 | 0 => {
                                        if (*expected_field).c2rust_unnamed.int_value
                                            != (*actual_field).c2rust_unnamed.int_value
                                        {
                                            current_block = 11850672490900767948;
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                                i += 1;
                                i;
                            }
                            match current_block {
                                11850672490900767948 => {}
                                _ => return 0 as libc::c_int,
                            }
                        }
                    }
                }
            }
        }
    }
    return ret;
}
unsafe extern "C" fn fstrncpy(
    mut dest: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n && *src.offset(i as isize) as libc::c_int != 0 as libc::c_int
        && *src.offset(i as isize) as libc::c_int != 0x1 as libc::c_int
    {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
        i;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn fprintmsg(mut stream: *mut FILE, mut msg: *mut fix_message) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut field: *mut fix_field = 0 as *mut fix_field;
    let mut size: libc::c_int = ::core::mem::size_of::<[libc::c_char; 256]>()
        as libc::c_ulong as libc::c_int;
    let mut delim: libc::c_char = '|' as i32 as libc::c_char;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if msg.is_null() {
        return;
    }
    if !((*msg).begin_string).is_null() && len < size {
        len
            += snprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as libc::c_ulong,
                b"%c%d=\0" as *const u8 as *const libc::c_char,
                delim as libc::c_int,
                BeginString as libc::c_int,
            );
        len
            += fstrncpy(
                buf.as_mut_ptr().offset(len as isize),
                (*msg).begin_string,
                size - len,
            );
    }
    if (*msg).body_length != 0 && len < size {
        len
            += snprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as libc::c_ulong,
                b"%c%d=%lu\0" as *const u8 as *const libc::c_char,
                delim as libc::c_int,
                BodyLength as libc::c_int,
                (*msg).body_length,
            );
    }
    if !((*msg).msg_type).is_null() && len < size {
        len
            += snprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as libc::c_ulong,
                b"%c%d=\0" as *const u8 as *const libc::c_char,
                delim as libc::c_int,
                MsgType as libc::c_int,
            );
        len
            += fstrncpy(
                buf.as_mut_ptr().offset(len as isize),
                (*msg).msg_type,
                size - len,
            );
    }
    if !((*msg).sender_comp_id).is_null() && len < size {
        len
            += snprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as libc::c_ulong,
                b"%c%d=\0" as *const u8 as *const libc::c_char,
                delim as libc::c_int,
                SenderCompID as libc::c_int,
            );
        len
            += fstrncpy(
                buf.as_mut_ptr().offset(len as isize),
                (*msg).sender_comp_id,
                size - len,
            );
    }
    if !((*msg).target_comp_id).is_null() && len < size {
        len
            += snprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as libc::c_ulong,
                b"%c%d=\0" as *const u8 as *const libc::c_char,
                delim as libc::c_int,
                TargetCompID as libc::c_int,
            );
        len
            += fstrncpy(
                buf.as_mut_ptr().offset(len as isize),
                (*msg).target_comp_id,
                size - len,
            );
    }
    if (*msg).msg_seq_num != 0 && len < size {
        len
            += snprintf(
                buf.as_mut_ptr().offset(len as isize),
                (size - len) as libc::c_ulong,
                b"%c%d=%lu\0" as *const u8 as *const libc::c_char,
                delim as libc::c_int,
                MsgSeqNum as libc::c_int,
                (*msg).msg_seq_num,
            );
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*msg).nr_fields && len < size {
        field = ((*msg).fields).offset(i as isize);
        match (*field).type_0 as libc::c_uint {
            3 => {
                len
                    += snprintf(
                        buf.as_mut_ptr().offset(len as isize),
                        (size - len) as libc::c_ulong,
                        b"%c%d=\0" as *const u8 as *const libc::c_char,
                        delim as libc::c_int,
                        (*field).tag,
                    );
                len
                    += fstrncpy(
                        buf.as_mut_ptr().offset(len as isize),
                        (*field).c2rust_unnamed.string_value,
                        size - len,
                    );
            }
            1 => {
                len
                    += snprintf(
                        buf.as_mut_ptr().offset(len as isize),
                        (size - len) as libc::c_ulong,
                        b"%c%d=%f\0" as *const u8 as *const libc::c_char,
                        delim as libc::c_int,
                        (*field).tag,
                        (*field).c2rust_unnamed.float_value,
                    );
            }
            2 => {
                len
                    += snprintf(
                        buf.as_mut_ptr().offset(len as isize),
                        (size - len) as libc::c_ulong,
                        b"%c%d=%c\0" as *const u8 as *const libc::c_char,
                        delim as libc::c_int,
                        (*field).tag,
                        (*field).c2rust_unnamed.char_value as libc::c_int,
                    );
            }
            4 | 0 => {
                len
                    += snprintf(
                        buf.as_mut_ptr().offset(len as isize),
                        (size - len) as libc::c_ulong,
                        b"%c%d=%ld\0" as *const u8 as *const libc::c_char,
                        delim as libc::c_int,
                        (*field).tag,
                        (*field).c2rust_unnamed.int_value,
                    );
            }
            _ => {}
        }
        i += 1;
        i;
    }
    if len < size {
        let fresh3 = len;
        len = len + 1;
        buf[fresh3 as usize] = '\0' as i32 as libc::c_char;
    }
    fprintf(
        stream,
        b"%s%c\n\0" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        delim as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fprintmsg_iov(
    mut stream: *mut FILE,
    mut msg: *mut fix_message,
) {
    let mut delim: libc::c_char = '|' as i32 as libc::c_char;
    let mut i: libc::c_int = 0;
    if msg.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut start: *const libc::c_char = (*msg).iov[i as usize].iov_base
            as *const libc::c_char;
        let mut len: libc::c_uint = (*msg).iov[i as usize].iov_len as libc::c_uint;
        let mut end: *const libc::c_char = 0 as *const libc::c_char;
        loop {
            end = memchr(
                start as *const libc::c_void,
                0x1 as libc::c_int,
                len as libc::c_ulong,
            ) as *const libc::c_char;
            if end.is_null() {
                break;
            }
            fprintf(
                stdout,
                b"%c%.*s\0" as *const u8 as *const libc::c_char,
                delim as libc::c_int,
                end.offset_from(start) as libc::c_long as libc::c_int,
                start,
            );
            len = (len as libc::c_long
                - (end.offset_from(start) as libc::c_long
                    + 1 as libc::c_int as libc::c_long)) as libc::c_uint;
            start = end.offset(1 as libc::c_int as isize);
        }
        i += 1;
        i;
    }
    fprintf(stdout, b"%c\n\0" as *const u8 as *const libc::c_char, delim as libc::c_int);
}
