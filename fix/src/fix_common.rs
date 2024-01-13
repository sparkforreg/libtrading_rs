use ::libc;
extern "C" {
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn fix_get_field(self_0: *mut fix_message, tag: libc::c_int) -> *mut fix_field;
    fn fix_get_string(
        field: *mut fix_field,
        buffer: *mut libc::c_char,
        len: libc::c_ulong,
    ) -> *const libc::c_char;
    fn fix_message_type_is(self_0: *mut fix_message, type_0: fix_msg_type) -> bool;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type clockid_t = __clockid_t;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
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
pub struct  fix_message {
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
#[inline]
unsafe extern "C" fn fix_msg_expected(
    mut session: *mut fix_session,
    mut msg: *mut fix_message,
) -> bool {
    return (*msg).msg_seq_num == (*session).in_msg_seq_num
        || fix_message_type_is(msg, FIX_MSG_TYPE_SEQUENCE_RESET) as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_keepalive(
    mut session: *mut fix_session,
    mut now: *mut timespec,
) -> bool {
    let mut diff: libc::c_int = 0;
    if (*session).tr_pending == 0 {
        diff = ((*now).tv_sec - (*session).rx_timestamp.tv_sec) as libc::c_int;
        if diff as libc::c_double > 1.2f64 * (*session).heartbtint as libc::c_double {
            fix_session_test_request(session);
        }
    } else {
        diff = ((*now).tv_sec - (*session).tr_timestamp.tv_sec) as libc::c_int;
        if diff as libc::c_double > 0.5f64 * (*session).heartbtint as libc::c_double {
            return 0 as libc::c_int != 0;
        }
    }
    diff = ((*now).tv_sec - (*session).tx_timestamp.tv_sec) as libc::c_int;
    if diff > (*session).heartbtint {
        fix_session_heartbeat(session, 0 as *const libc::c_char);
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn fix_do_unexpected(
    mut session: *mut fix_session,
    mut msg: *mut fix_message,
) -> libc::c_int {
    let mut text: [libc::c_char; 128] = [0; 128];
    if (*msg).msg_seq_num > (*session).in_msg_seq_num {
        let mut end_seq_no: libc::c_ulong = 0;
        if (*(*session).dialect).version as libc::c_uint
            <= FIX_4_1 as libc::c_int as libc::c_uint
        {
            end_seq_no = 999999 as libc::c_int as libc::c_ulong;
        } else {
            end_seq_no = 0 as libc::c_int as libc::c_ulong;
        }
        fix_session_resend_request(session, (*session).in_msg_seq_num, end_seq_no);
        (*session).in_msg_seq_num = ((*session).in_msg_seq_num).wrapping_sub(1);
        (*session).in_msg_seq_num;
    } else if (*msg).msg_seq_num < (*session).in_msg_seq_num {
        snprintf(
            text.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"MsgSeqNum too low, expecting %lu received %lu\0" as *const u8
                as *const libc::c_char,
            (*session).in_msg_seq_num,
            (*msg).msg_seq_num,
        );
        (*session).in_msg_seq_num = ((*session).in_msg_seq_num).wrapping_sub(1);
        (*session).in_msg_seq_num;
        if (fix_get_field(msg, PossDupFlag as libc::c_int)).is_null() {
            fix_session_logout(session, text.as_mut_ptr());
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_admin(
    mut session: *mut fix_session,
    mut msg: *mut fix_message,
) -> bool {
    let mut current_block: u64;
    let mut field: *mut fix_field = 0 as *mut fix_field;
    if !fix_msg_expected(session, msg) {
        fix_do_unexpected(session, msg);
    } else {
        match (*msg).type_0 as libc::c_ulong {
            0 => {
                field = fix_get_field(msg, TestReqID as libc::c_int);
                if !field.is_null()
                    && strncmp(
                        (*field).c2rust_unnamed.string_value,
                        ((*session).testreqid).as_mut_ptr(),
                        strlen(((*session).testreqid).as_mut_ptr()),
                    ) == 0
                {
                    (*session).tr_pending = 0 as libc::c_int;
                }
                current_block = 2415383508987514339;
            }
            1 => {
                let mut id: [libc::c_char; 128] = *::core::mem::transmute::<
                    &[u8; 128],
                    &mut [libc::c_char; 128],
                >(
                    b"TestReqID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                );
                field = fix_get_field(msg, TestReqID as libc::c_int);
                if !field.is_null() {
                    fix_get_string(
                        field,
                        id.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    );
                }
                fix_session_heartbeat(session, id.as_mut_ptr());
                current_block = 2415383508987514339;
            }
            2 => {
                let mut begin_seq_num: libc::c_ulong = 0;
                let mut end_seq_num: libc::c_ulong = 0;
                field = fix_get_field(msg, BeginSeqNo as libc::c_int);
                if field.is_null() {
                    current_block = 16790958123778133386;
                } else {
                    begin_seq_num = (*field).c2rust_unnamed.int_value as libc::c_ulong;
                    field = fix_get_field(msg, EndSeqNo as libc::c_int);
                    if field.is_null() {
                        current_block = 16790958123778133386;
                    } else {
                        end_seq_num = (*field).c2rust_unnamed.int_value as libc::c_ulong;
                        fix_session_sequence_reset(
                            session,
                            begin_seq_num,
                            end_seq_num.wrapping_add(1 as libc::c_int as libc::c_ulong),
                            1 as libc::c_int != 0,
                        );
                        current_block = 2415383508987514339;
                    }
                }
            }
            4 => {
                let mut exp_seq_num: libc::c_ulong = 0;
                let mut new_seq_num: libc::c_ulong = 0;
                let mut msg_seq_num: libc::c_ulong = 0;
                let mut text: [libc::c_char; 128] = [0; 128];
                field = fix_get_field(msg, GapFillFlag as libc::c_int);
                if !field.is_null()
                    && strncmp(
                        (*field).c2rust_unnamed.string_value,
                        b"Y\0" as *const u8 as *const libc::c_char,
                        1 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                    field = fix_get_field(msg, NewSeqNo as libc::c_int);
                    if !field.is_null() {
                        exp_seq_num = (*session).in_msg_seq_num;
                        new_seq_num = (*field).c2rust_unnamed.int_value as libc::c_ulong;
                        msg_seq_num = (*msg).msg_seq_num;
                        if msg_seq_num > exp_seq_num {
                            fix_session_resend_request(
                                session,
                                exp_seq_num,
                                msg_seq_num,
                            );
                            (*session)
                                .in_msg_seq_num = ((*session).in_msg_seq_num)
                                .wrapping_sub(1);
                            (*session).in_msg_seq_num;
                        } else if msg_seq_num < exp_seq_num {
                            snprintf(
                                text.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong,
                                b"MsgSeqNum too low, expecting %lu received %lu\0"
                                    as *const u8 as *const libc::c_char,
                                exp_seq_num,
                                msg_seq_num,
                            );
                            (*session)
                                .in_msg_seq_num = ((*session).in_msg_seq_num)
                                .wrapping_sub(1);
                            (*session).in_msg_seq_num;
                            if (fix_get_field(msg, PossDupFlag as libc::c_int)).is_null()
                            {
                                fix_session_logout(session, text.as_mut_ptr());
                            }
                        } else if new_seq_num > msg_seq_num {
                            (*session)
                                .in_msg_seq_num = new_seq_num
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        } else {
                            snprintf(
                                text.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong,
                                b"Attempt to lower sequence number, invalid value NewSeqNum = %lu\0"
                                    as *const u8 as *const libc::c_char,
                                new_seq_num,
                            );
                            fix_session_reject(session, msg_seq_num, text.as_mut_ptr());
                        }
                    }
                } else {
                    field = fix_get_field(msg, NewSeqNo as libc::c_int);
                    if !field.is_null() {
                        exp_seq_num = (*session).in_msg_seq_num;
                        new_seq_num = (*field).c2rust_unnamed.int_value as libc::c_ulong;
                        if new_seq_num > exp_seq_num {
                            (*session)
                                .in_msg_seq_num = new_seq_num
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
                        } else if new_seq_num < exp_seq_num {
                            snprintf(
                                text.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong,
                                b"Value is incorrect (too low) %lu\0" as *const u8
                                    as *const libc::c_char,
                                new_seq_num,
                            );
                            (*session)
                                .in_msg_seq_num = ((*session).in_msg_seq_num)
                                .wrapping_sub(1);
                            (*session).in_msg_seq_num;
                            fix_session_reject(session, exp_seq_num, text.as_mut_ptr());
                        }
                    }
                }
                current_block = 2415383508987514339;
            }
            _ => {
                current_block = 16790958123778133386;
            }
        }
        match current_block {
            2415383508987514339 => {}
            _ => return 0 as libc::c_int != 0,
        }
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_logon(
    mut session: *mut fix_session,
) -> libc::c_int {
    let mut response: *mut fix_message = 0 as *mut fix_message;
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
    let mut fields: [fix_field; 4] = [
        {
            let mut init = fix_field {
                tag: EncryptMethod as libc::c_int,
                type_0: FIX_TYPE_INT,
                c2rust_unnamed: C2RustUnnamed {
                    int_value: 0 as libc::c_int as int64_t,
                },
            };
            init
        },
        {
            let mut init = fix_field {
                tag: ResetSeqNumFlag as libc::c_int,
                type_0: FIX_TYPE_STRING,
                c2rust_unnamed: C2RustUnnamed {
                    string_value: b"Y\0" as *const u8 as *const libc::c_char,
                },
            };
            init
        },
        {
            let mut init = fix_field {
                tag: HeartBtInt as libc::c_int,
                type_0: FIX_TYPE_INT,
                c2rust_unnamed: C2RustUnnamed {
                    int_value: (*session).heartbtint as int64_t,
                },
            };
            init
        },
        {
            let mut init = fix_field {
                tag: Password as libc::c_int,
                type_0: FIX_TYPE_STRING,
                c2rust_unnamed: C2RustUnnamed {
                    string_value: (*session).password,
                },
            };
            init
        },
    ];
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
            nr_fields: (::core::mem::size_of::<[fix_field; 4]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<fix_field>() as libc::c_ulong),
            fields: fields.as_mut_ptr(),
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    if ((*session).password).is_null() || strlen((*session).password) == 0 {
        logon_msg.nr_fields = (logon_msg.nr_fields).wrapping_sub(1);
        logon_msg.nr_fields;
    }
    fix_session_send(session, &mut logon_msg, 0 as libc::c_int as libc::c_ulong);
    (*session).active = 1 as libc::c_int != 0;
    loop {
        if fix_session_recv(
            session,
            &mut response,
            FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
        ) <= 0 as libc::c_int
        {
            continue;
        }
        if fix_msg_expected(session, response) {
            break;
        }
        if fix_do_unexpected(session, response) != 0 {
            return -(1 as libc::c_int);
        }
    }
    if !fix_message_type_is(response, FIX_MSG_TYPE_LOGON) {
        fix_session_logout(
            session,
            b"First message not a logon\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_logout(
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
    let mut response: *mut fix_message = 0 as *mut fix_message;
    let mut start: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut end: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
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
    fix_session_send(session, &mut logout_msg, 0 as libc::c_int as libc::c_ulong);
    clock_gettime(1 as libc::c_int, &mut start);
    (*session).active = 0 as libc::c_int != 0;
    loop {
        clock_gettime(1 as libc::c_int, &mut end);
        if end.tv_sec - start.tv_sec > 2 as libc::c_int as libc::c_long {
            return 0 as libc::c_int;
        }
        if fix_session_recv(
            session,
            &mut response,
            FIX_RECV_FLAG_MSG_DONTWAIT as libc::c_int as libc::c_ulong,
        ) <= 0 as libc::c_int
        {
            continue;
        }
        if !fix_session_admin(session, response) {
            break;
        }
    }
    if fix_message_type_is(response, FIX_MSG_TYPE_LOGOUT) {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_heartbeat(
    mut session: *mut fix_session,
    mut test_req_id: *const libc::c_char,
) -> libc::c_int {
    let mut heartbeat_msg: fix_message = fix_message {
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
    let mut fields: [fix_field; 1] = [fix_field {
        tag: 0,
        type_0: FIX_TYPE_INT,
        c2rust_unnamed: C2RustUnnamed { int_value: 0 },
    }; 1];
    let mut nr_fields: libc::c_int = 0 as libc::c_int;
    if !test_req_id.is_null() {
        let fresh0 = nr_fields;
        nr_fields = nr_fields + 1;
        fields[fresh0
            as usize] = {
            let mut init = fix_field {
                tag: TestReqID as libc::c_int,
                type_0: FIX_TYPE_STRING,
                c2rust_unnamed: C2RustUnnamed {
                    string_value: test_req_id,
                },
            };
            init
        };
    }
    heartbeat_msg = {
        let mut init = fix_message {
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
            nr_fields: nr_fields as libc::c_ulong,
            fields: fields.as_mut_ptr(),
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    return fix_session_send(
        session,
        &mut heartbeat_msg,
        0 as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_test_request(
    mut session: *mut fix_session,
) -> libc::c_int {
    let mut test_req_msg: fix_message = fix_message {
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
    let mut fields: [fix_field; 1] = [
        {
            let mut init = fix_field {
                tag: TestReqID as libc::c_int,
                type_0: FIX_TYPE_STRING,
                c2rust_unnamed: C2RustUnnamed {
                    string_value: ((*session).str_now).as_mut_ptr(),
                },
            };
            init
        },
    ];
    strncpy(
        ((*session).testreqid).as_mut_ptr(),
        ((*session).str_now).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    test_req_msg = {
        let mut init = fix_message {
            type_0: FIX_MSG_TYPE_TEST_REQUEST,
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
            nr_fields: (::core::mem::size_of::<[fix_field; 1]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<fix_field>() as libc::c_ulong),
            fields: fields.as_mut_ptr(),
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    (*session).tr_timestamp = (*session).now;
    (*session).tr_pending = 1 as libc::c_int;
    return fix_session_send(
        session,
        &mut test_req_msg,
        0 as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_resend_request(
    mut session: *mut fix_session,
    mut bgn: libc::c_ulong,
    mut end: libc::c_ulong,
) -> libc::c_int {
    let mut resend_request_msg: fix_message = fix_message {
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
    let mut fields: [fix_field; 2] = [
        {
            let mut init = fix_field {
                tag: BeginSeqNo as libc::c_int,
                type_0: FIX_TYPE_INT,
                c2rust_unnamed: C2RustUnnamed {
                    int_value: bgn as int64_t,
                },
            };
            init
        },
        {
            let mut init = fix_field {
                tag: EndSeqNo as libc::c_int,
                type_0: FIX_TYPE_INT,
                c2rust_unnamed: C2RustUnnamed {
                    int_value: end as int64_t,
                },
            };
            init
        },
    ];
    resend_request_msg = {
        let mut init = fix_message {
            type_0: FIX_MSG_TYPE_RESEND_REQUEST,
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
            nr_fields: (::core::mem::size_of::<[fix_field; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<fix_field>() as libc::c_ulong),
            fields: fields.as_mut_ptr(),
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    return fix_session_send(
        session,
        &mut resend_request_msg,
        0 as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_reject(
    mut session: *mut fix_session,
    mut refseqnum: libc::c_ulong,
    mut text: *mut libc::c_char,
) -> libc::c_int {
    let mut reject_msg: fix_message = fix_message {
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
    let mut fields: [fix_field; 2] = [
        {
            let mut init = fix_field {
                tag: RefSeqNum as libc::c_int,
                type_0: FIX_TYPE_INT,
                c2rust_unnamed: C2RustUnnamed {
                    int_value: refseqnum as int64_t,
                },
            };
            init
        },
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
    let mut nr_fields: libc::c_long = (::core::mem::size_of::<[fix_field; 2]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<fix_field>() as libc::c_ulong)
        as libc::c_long;
    if text.is_null() {
        nr_fields -= 1;
        nr_fields;
    }
    reject_msg = {
        let mut init = fix_message {
            type_0: FIX_MSG_TYPE_REJECT,
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
    return fix_session_send(session, &mut reject_msg, 0 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_sequence_reset(
    mut session: *mut fix_session,
    mut msg_seq_num: libc::c_ulong,
    mut new_seq_num: libc::c_ulong,
    mut gap_fill: bool,
) -> libc::c_int {
    let mut sequence_reset_msg: fix_message = fix_message {
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
    let mut fields: [fix_field; 2] = [
        {
            let mut init = fix_field {
                tag: NewSeqNo as libc::c_int,
                type_0: FIX_TYPE_INT,
                c2rust_unnamed: C2RustUnnamed {
                    int_value: new_seq_num as int64_t,
                },
            };
            init
        },
        {
            let mut init = fix_field {
                tag: GapFillFlag as libc::c_int,
                type_0: FIX_TYPE_STRING,
                c2rust_unnamed: C2RustUnnamed {
                    string_value: b"Y\0" as *const u8 as *const libc::c_char,
                },
            };
            init
        },
    ];
    let mut nr_fields: libc::c_long = (::core::mem::size_of::<[fix_field; 2]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<fix_field>() as libc::c_ulong)
        as libc::c_long;
    if !gap_fill {
        nr_fields -= 1;
        nr_fields;
    }
    sequence_reset_msg = {
        let mut init = fix_message {
            type_0: FIX_MSG_TYPE_SEQUENCE_RESET,
            begin_string: 0 as *const libc::c_char,
            body_length: 0,
            msg_type: 0 as *const libc::c_char,
            sender_comp_id: 0 as *const libc::c_char,
            target_comp_id: 0 as *const libc::c_char,
            msg_seq_num: msg_seq_num,
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
    return fix_session_send(
        session,
        &mut sequence_reset_msg,
        FIX_SEND_FLAG_PRESERVE_MSG_NUM as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_new_order_single(
    mut session: *mut fix_session,
    mut fields: *mut fix_field,
    mut nr_fields: libc::c_long,
) -> libc::c_int {
    let mut new_order_single_msg: fix_message = fix_message {
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
    new_order_single_msg = {
        let mut init = fix_message {
            type_0: FIX_MSG_TYPE_NEW_ORDER_SINGLE,
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
            fields: fields,
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    return fix_session_send(
        session,
        &mut new_order_single_msg,
        0 as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_order_cancel_request(
    mut session: *mut fix_session,
    mut fields: *mut fix_field,
    mut nr_fields: libc::c_long,
) -> libc::c_int {
    let mut order_cancel_request: fix_message = fix_message {
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
    order_cancel_request = {
        let mut init = fix_message {
            type_0: FIX_MSG_ORDER_CANCEL_REQUEST,
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
            fields: fields,
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    return fix_session_send(
        session,
        &mut order_cancel_request,
        0 as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_order_cancel_replace(
    mut session: *mut fix_session,
    mut fields: *mut fix_field,
    mut nr_fields: libc::c_long,
) -> libc::c_int {
    let mut order_cancel_replace: fix_message = fix_message {
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
    order_cancel_replace = {
        let mut init = fix_message {
            type_0: FIX_MSG_ORDER_CANCEL_REPLACE,
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
            fields: fields,
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    return fix_session_send(
        session,
        &mut order_cancel_replace,
        0 as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fix_session_execution_report(
    mut session: *mut fix_session,
    mut fields: *mut fix_field,
    mut nr_fields: libc::c_long,
) -> libc::c_int {
    let mut new_order_single_msg: fix_message = fix_message {
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
    new_order_single_msg = {
        let mut init = fix_message {
            type_0: FIX_MSG_TYPE_EXECUTION_REPORT,
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
            fields: fields,
            iov: [iovec {
                iov_base: 0 as *mut libc::c_void,
                iov_len: 0,
            }; 2],
        };
        init
    };
    return fix_session_send(
        session,
        &mut new_order_single_msg,
        0 as libc::c_int as libc::c_ulong,
    );
}
