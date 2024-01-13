use ::libc;
extern "C" {
    pub type _GHashTable;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn g_hash_table_new(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_str_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_str_hash(v: gconstpointer) -> guint;
}
pub type __uint8_t = libc::c_uchar;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type i64_0 = int64_t;
pub type u8_0 = uint8_t;
pub type u64_0 = uint64_t;
pub type size_t = libc::c_ulong;
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
pub type GEqualFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
>;
pub type GHashFunc = Option::<unsafe extern "C" fn(gconstpointer) -> guint>;
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
pub struct fast_sequence {
    pub pmap: fast_pmap,
    pub decoded: libc::c_ulong,
    pub length: fast_field,
    pub elements: [fast_message; 128],
}
#[inline]
unsafe extern "C" fn field_state_empty(mut field: *mut fast_field) -> bool {
    return (*field).state as libc::c_uint
        == FAST_STATE_EMPTY as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn field_state_empty_previous(mut field: *mut fast_field) -> bool {
    return (*field).state_previous as libc::c_uint
        == FAST_STATE_EMPTY as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn field_is_mandatory(mut field: *mut fast_field) -> bool {
    return (*field).presence as libc::c_uint
        == FAST_PRESENCE_MANDATORY as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn field_has_reset_value(mut field: *mut fast_field) -> bool {
    return (*field).has_reset;
}
#[inline]
unsafe extern "C" fn field_has_flags(
    mut field: *mut fast_field,
    mut flags: libc::c_int,
) -> libc::c_int {
    return (*field).flags & flags;
}
#[inline]
unsafe extern "C" fn pmap_is_set(
    mut pmap: *mut fast_pmap,
    mut bit: libc::c_ulong,
) -> bool {
    if bit.wrapping_div(7 as libc::c_int as libc::c_ulong) >= (*pmap).nr_bytes {
        return 0 as libc::c_int != 0;
    }
    return (*pmap).bytes[bit.wrapping_div(7 as libc::c_int as libc::c_ulong) as usize]
        as libc::c_int
        & (1 as libc::c_int)
            << (6 as libc::c_int as libc::c_ulong)
                .wrapping_sub(bit.wrapping_rem(7 as libc::c_int as libc::c_ulong)) != 0;
}
#[inline]
unsafe extern "C" fn pmap_set(mut pmap: *mut fast_pmap, mut bit: libc::c_ulong) -> bool {
    if bit.wrapping_div(7 as libc::c_int as libc::c_ulong) >= (*pmap).nr_bytes {
        return 0 as libc::c_int != 0;
    }
    (*pmap)
        .bytes[bit.wrapping_div(7 as libc::c_int as libc::c_ulong)
        as usize] = ((*pmap)
        .bytes[bit.wrapping_div(7 as libc::c_int as libc::c_ulong) as usize]
        as libc::c_int
        | (1 as libc::c_int)
            << (6 as libc::c_int as libc::c_ulong)
                .wrapping_sub(bit.wrapping_rem(7 as libc::c_int as libc::c_ulong)))
        as libc::c_char;
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn transfer_size_int(mut data: i64_0) -> libc::c_int {
    let mut tmp: i64_0 = if data >= 0 as libc::c_int as libc::c_long {
        data
    } else {
        !data
    };
    if tmp >> 6 as libc::c_int == 0 {
        return 1 as libc::c_int
    } else if tmp >> 13 as libc::c_int == 0 {
        return 2 as libc::c_int
    } else if tmp >> 20 as libc::c_int == 0 {
        return 3 as libc::c_int
    } else if tmp >> 27 as libc::c_int == 0 {
        return 4 as libc::c_int
    } else if tmp >> 34 as libc::c_int == 0 {
        return 5 as libc::c_int
    } else if tmp >> 41 as libc::c_int == 0 {
        return 6 as libc::c_int
    } else if tmp >> 48 as libc::c_int == 0 {
        return 7 as libc::c_int
    } else if tmp >> 55 as libc::c_int == 0 {
        return 8 as libc::c_int
    } else {
        return 9 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn transfer_size_uint(mut data: u64_0) -> libc::c_int {
    if data >> 7 as libc::c_int == 0 {
        return 1 as libc::c_int
    } else if data >> 14 as libc::c_int == 0 {
        return 2 as libc::c_int
    } else if data >> 21 as libc::c_int == 0 {
        return 3 as libc::c_int
    } else if data >> 28 as libc::c_int == 0 {
        return 4 as libc::c_int
    } else if data >> 35 as libc::c_int == 0 {
        return 5 as libc::c_int
    } else if data >> 42 as libc::c_int == 0 {
        return 6 as libc::c_int
    } else if data >> 49 as libc::c_int == 0 {
        return 7 as libc::c_int
    } else if data >> 56 as libc::c_int == 0 {
        return 8 as libc::c_int
    } else {
        return 9 as libc::c_int
    };
}
#[inline]
unsafe extern "C" fn buffer_start(mut self_0: *const buffer) -> *mut libc::c_char {
    return &mut *((*self_0).data).offset((*self_0).start as isize) as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn buffer_advance(mut self_0: *mut buffer, mut n: libc::c_long) {
    (*self_0).start = ((*self_0).start).wrapping_add(n as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn buffer_get_8(mut self_0: *mut buffer) -> u8_0 {
    let fresh0 = (*self_0).start;
    (*self_0).start = ((*self_0).start).wrapping_add(1);
    return *((*self_0).data).offset(fresh0 as isize) as u8_0;
}
#[inline]
unsafe extern "C" fn buffer_size(mut self_0: *const buffer) -> libc::c_ulong {
    return ((*self_0).end).wrapping_sub((*self_0).start);
}
#[inline]
unsafe extern "C" fn buffer_peek_8(mut self_0: *const buffer) -> u8_0 {
    return *((*self_0).data).offset((*self_0).start as isize) as u8_0;
}
#[inline]
unsafe extern "C" fn buffer_get_n(
    mut self_0: *mut buffer,
    mut n: libc::c_int,
    mut dst: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = buffer_get_8(self_0) as libc::c_char;
        i += 1;
        i;
    }
}
#[inline]
unsafe extern "C" fn buffer_to_iovec(mut b: *mut buffer, mut iov: *mut iovec) {
    (*iov).iov_base = (*b).data as *mut libc::c_void;
    (*iov).iov_len = (*b).end;
}
#[inline]
unsafe extern "C" fn buffer_put(mut self_0: *mut buffer, mut byte: libc::c_char) {
    let fresh2 = (*self_0).end;
    (*self_0).end = ((*self_0).end).wrapping_add(1);
    *((*self_0).data).offset(fresh2 as isize) = byte;
}
#[inline]
unsafe extern "C" fn buffer_remaining(mut self_0: *const buffer) -> libc::c_ulong {
    return ((*self_0).capacity).wrapping_sub((*self_0).end);
}
unsafe extern "C" fn parse_uint(
    mut buffer: *mut buffer,
    mut value: *mut u64_0,
) -> libc::c_int {
    let mut current_block: u64;
    let bytes: libc::c_int = 9 as libc::c_int;
    let mut result: u64_0 = 0;
    let mut i: libc::c_int = 0;
    let mut c: u8_0 = 0;
    result = 0 as libc::c_int as u64_0;
    i = 0 as libc::c_int;
    loop {
        if !(i < bytes) {
            current_block = 11812396948646013369;
            break;
        }
        if buffer_size(buffer) == 0 {
            current_block = 1532624277541458025;
            break;
        }
        c = buffer_get_8(buffer);
        if c as libc::c_int & 0x80 as libc::c_int != 0 {
            result = result << 7 as libc::c_int
                | (c as libc::c_int & 0x7f as libc::c_int) as libc::c_ulong;
            *value = result;
            return 0 as libc::c_int;
        }
        result = result << 7 as libc::c_int | c as libc::c_ulong;
        i += 1;
        i;
    }
    match current_block {
        1532624277541458025 => return -(2 as libc::c_int),
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn parse_int(
    mut buffer: *mut buffer,
    mut value: *mut i64_0,
) -> libc::c_int {
    let mut current_block: u64;
    let bytes: libc::c_int = 10 as libc::c_int;
    let mut result: i64_0 = 0;
    let mut i: libc::c_int = 0;
    let mut c: u8_0 = 0;
    result = 0 as libc::c_int as i64_0;
    if !(buffer_size(buffer) == 0) {
        if buffer_peek_8(buffer) as libc::c_int & 0x40 as libc::c_int != 0 {
            result = -(1 as libc::c_int) as i64_0;
        }
        i = 0 as libc::c_int;
        loop {
            if !(i < bytes) {
                current_block = 1856101646708284338;
                break;
            }
            if buffer_size(buffer) == 0 {
                current_block = 7046939862894676505;
                break;
            }
            c = buffer_get_8(buffer);
            if c as libc::c_int & 0x80 as libc::c_int != 0 {
                result = result << 7 as libc::c_int
                    | (c as libc::c_int & 0x7f as libc::c_int) as libc::c_long;
                *value = result;
                return 0 as libc::c_int;
            }
            result = result << 7 as libc::c_int | c as libc::c_long;
            i += 1;
            i;
        }
        match current_block {
            7046939862894676505 => {}
            _ => return -(1 as libc::c_int),
        }
    }
    return -(2 as libc::c_int);
}
unsafe extern "C" fn parse_string(
    mut buffer: *mut buffer,
    mut value: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut len: libc::c_int = 0;
    let mut c: u8_0 = 0;
    len = 0 as libc::c_int;
    loop {
        if !((len as libc::c_ulong)
            < size.wrapping_sub(1 as libc::c_int as libc::c_ulong))
        {
            current_block = 17216689946888361452;
            break;
        }
        if buffer_size(buffer) == 0 {
            current_block = 13925162555608785600;
            break;
        }
        c = buffer_get_8(buffer);
        if c as libc::c_int & 0x80 as libc::c_int != 0 {
            let fresh3 = len;
            len = len + 1;
            *value
                .offset(
                    fresh3 as isize,
                ) = (c as libc::c_int & 0x7f as libc::c_int) as libc::c_char;
            *value.offset(len as isize) = '\0' as i32 as libc::c_char;
            return len;
        } else {
            let fresh4 = len;
            len = len + 1;
            *value.offset(fresh4 as isize) = c as libc::c_char;
        }
    }
    match current_block {
        13925162555608785600 => return -(2 as libc::c_int),
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn parse_bytes(
    mut buffer: *mut buffer,
    mut value: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: u8_0 = 0;
    if buffer_size(buffer) < len as libc::c_ulong {
        return -(2 as libc::c_int)
    } else {
        i = 0 as libc::c_int;
        while i < len {
            c = buffer_get_8(buffer);
            *value.offset(i as isize) = c as libc::c_char;
            i += 1;
            i;
        }
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn parse_pmap(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
) -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_char = 0;
    (*pmap).nr_bytes = 0 as libc::c_int as libc::c_ulong;
    loop {
        if !((*pmap).nr_bytes < 8 as libc::c_int as libc::c_ulong) {
            current_block = 11875828834189669668;
            break;
        }
        if buffer_size(buffer) == 0 {
            current_block = 17362110199563880934;
            break;
        }
        c = buffer_get_8(buffer) as libc::c_char;
        let fresh5 = (*pmap).nr_bytes;
        (*pmap).nr_bytes = ((*pmap).nr_bytes).wrapping_add(1);
        (*pmap).bytes[fresh5 as usize] = c;
        if c as libc::c_int & 0x80 as libc::c_int != 0 {
            return 0 as libc::c_int;
        }
    }
    match current_block {
        17362110199563880934 => return -(2 as libc::c_int),
        _ => return -(1 as libc::c_int),
    };
}
unsafe extern "C" fn fast_decode_uint(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut tmp: i64_0 = 0;
    match (*field).op as libc::c_uint {
        0 => {
            ret = parse_uint(buffer, &mut (*field).c2rust_unnamed.uint_value);
            if ret != 0 {
                current_block = 11355694495147257559;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                if field_is_mandatory(field) {
                    current_block = 8102658916883067714;
                } else {
                    if (*field).c2rust_unnamed.uint_value == 0 {
                        (*field).state = FAST_STATE_EMPTY;
                    } else {
                        (*field)
                            .c2rust_unnamed
                            .uint_value = ((*field).c2rust_unnamed.uint_value)
                            .wrapping_sub(1);
                        (*field).c2rust_unnamed.uint_value;
                    }
                    current_block = 8102658916883067714;
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 => {
                        current_block = 17833640586817200534;
                        match current_block {
                            17833640586817200534 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .uint_value = (*field).c2rust_unnamed_0.uint_reset;
                                    current_block = 8102658916883067714;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 8102658916883067714;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    current_block = 8102658916883067714;
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 13767817607354810862;
                        match current_block {
                            17833640586817200534 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .uint_value = (*field).c2rust_unnamed_0.uint_reset;
                                    current_block = 8102658916883067714;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 8102658916883067714;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    current_block = 8102658916883067714;
                                }
                            }
                        }
                    }
                    1 | _ => {
                        current_block = 8102658916883067714;
                    }
                }
            } else {
                ret = parse_uint(buffer, &mut (*field).c2rust_unnamed.uint_value);
                if ret != 0 {
                    current_block = 11355694495147257559;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if field_is_mandatory(field) {
                        current_block = 8102658916883067714;
                    } else {
                        if (*field).c2rust_unnamed.uint_value == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                        } else {
                            (*field)
                                .c2rust_unnamed
                                .uint_value = ((*field).c2rust_unnamed.uint_value)
                                .wrapping_sub(1);
                            (*field).c2rust_unnamed.uint_value;
                        }
                        current_block = 8102658916883067714;
                    }
                }
            }
        }
        2 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 => {
                        current_block = 17587702030281983968;
                        match current_block {
                            4710620441759487043 => {
                                (*field)
                                    .c2rust_unnamed
                                    .uint_value = ((*field).c2rust_unnamed.uint_value)
                                    .wrapping_add(1);
                                (*field).c2rust_unnamed.uint_value;
                                current_block = 8102658916883067714;
                            }
                            17587702030281983968 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .uint_value = (*field).c2rust_unnamed_0.uint_reset;
                                    current_block = 8102658916883067714;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 8102658916883067714;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    current_block = 8102658916883067714;
                                }
                            }
                        }
                    }
                    1 => {
                        current_block = 4710620441759487043;
                        match current_block {
                            4710620441759487043 => {
                                (*field)
                                    .c2rust_unnamed
                                    .uint_value = ((*field).c2rust_unnamed.uint_value)
                                    .wrapping_add(1);
                                (*field).c2rust_unnamed.uint_value;
                                current_block = 8102658916883067714;
                            }
                            17587702030281983968 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .uint_value = (*field).c2rust_unnamed_0.uint_reset;
                                    current_block = 8102658916883067714;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 8102658916883067714;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    current_block = 8102658916883067714;
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 6458530745706687221;
                        match current_block {
                            4710620441759487043 => {
                                (*field)
                                    .c2rust_unnamed
                                    .uint_value = ((*field).c2rust_unnamed.uint_value)
                                    .wrapping_add(1);
                                (*field).c2rust_unnamed.uint_value;
                                current_block = 8102658916883067714;
                            }
                            17587702030281983968 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .uint_value = (*field).c2rust_unnamed_0.uint_reset;
                                    current_block = 8102658916883067714;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 8102658916883067714;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 11355694495147257559;
                                } else {
                                    current_block = 8102658916883067714;
                                }
                            }
                        }
                    }
                    _ => {
                        current_block = 8102658916883067714;
                    }
                }
            } else {
                ret = parse_uint(buffer, &mut (*field).c2rust_unnamed.uint_value);
                if ret != 0 {
                    current_block = 11355694495147257559;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if field_is_mandatory(field) {
                        current_block = 8102658916883067714;
                    } else {
                        if (*field).c2rust_unnamed.uint_value == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                        } else {
                            (*field)
                                .c2rust_unnamed
                                .uint_value = ((*field).c2rust_unnamed.uint_value)
                                .wrapping_sub(1);
                            (*field).c2rust_unnamed.uint_value;
                        }
                        current_block = 8102658916883067714;
                    }
                }
            }
        }
        3 => {
            ret = parse_int(buffer, &mut tmp);
            if ret != 0 {
                current_block = 11355694495147257559;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                if tmp < 0 as libc::c_int as libc::c_long {
                    (*field)
                        .c2rust_unnamed
                        .uint_value = ((*field).c2rust_unnamed.uint_value
                        as libc::c_ulong)
                        .wrapping_sub(-tmp as libc::c_ulong) as u64_0 as u64_0;
                } else {
                    (*field)
                        .c2rust_unnamed
                        .uint_value = ((*field).c2rust_unnamed.uint_value
                        as libc::c_ulong)
                        .wrapping_add(tmp as libc::c_ulong) as u64_0 as u64_0;
                }
                if field_is_mandatory(field) {
                    current_block = 8102658916883067714;
                } else {
                    if tmp == 0 {
                        (*field).state = FAST_STATE_EMPTY;
                    } else if tmp > 0 as libc::c_int as libc::c_long {
                        (*field)
                            .c2rust_unnamed
                            .uint_value = ((*field).c2rust_unnamed.uint_value)
                            .wrapping_sub(1);
                        (*field).c2rust_unnamed.uint_value;
                    }
                    current_block = 8102658916883067714;
                }
            }
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 | 1 | 2 => {
                        if field_has_reset_value(field) {
                            (*field).state = FAST_STATE_ASSIGNED;
                            (*field)
                                .c2rust_unnamed
                                .uint_value = (*field).c2rust_unnamed_0.uint_reset;
                            current_block = 8102658916883067714;
                        } else if field_is_mandatory(field) {
                            ret = -(1 as libc::c_int);
                            current_block = 11355694495147257559;
                        } else {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 8102658916883067714;
                        }
                    }
                    _ => {
                        current_block = 8102658916883067714;
                    }
                }
            } else {
                ret = parse_uint(buffer, &mut (*field).c2rust_unnamed.uint_value);
                if ret != 0 {
                    current_block = 11355694495147257559;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if field_is_mandatory(field) {
                        current_block = 8102658916883067714;
                    } else {
                        if (*field).c2rust_unnamed.uint_value == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                        } else {
                            (*field)
                                .c2rust_unnamed
                                .uint_value = ((*field).c2rust_unnamed.uint_value)
                                .wrapping_sub(1);
                            (*field).c2rust_unnamed.uint_value;
                        }
                        current_block = 8102658916883067714;
                    }
                }
            }
        }
        5 => {
            if (*field).state as libc::c_uint
                != FAST_STATE_ASSIGNED as libc::c_int as libc::c_uint
            {
                (*field)
                    .c2rust_unnamed
                    .uint_value = (*field).c2rust_unnamed_0.uint_reset;
            }
            (*field).state = FAST_STATE_ASSIGNED;
            if field_is_mandatory(field) {
                current_block = 8102658916883067714;
            } else {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                    (*field).state = FAST_STATE_EMPTY;
                }
                current_block = 8102658916883067714;
            }
        }
        _ => return -(1 as libc::c_int),
    }
    match current_block {
        11355694495147257559 => return ret,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn fast_decode_int(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut tmp: i64_0 = 0;
    match (*field).op as libc::c_uint {
        0 => {
            ret = parse_int(buffer, &mut (*field).c2rust_unnamed.int_value);
            if ret != 0 {
                current_block = 12393217337207382707;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                if field_is_mandatory(field) {
                    current_block = 10261677128829721533;
                } else {
                    if (*field).c2rust_unnamed.int_value == 0 {
                        (*field).state = FAST_STATE_EMPTY;
                    } else if (*field).c2rust_unnamed.int_value
                        > 0 as libc::c_int as libc::c_long
                    {
                        (*field).c2rust_unnamed.int_value -= 1;
                        (*field).c2rust_unnamed.int_value;
                    }
                    current_block = 10261677128829721533;
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 => {
                        current_block = 10830055424993547762;
                        match current_block {
                            10830055424993547762 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .int_value = (*field).c2rust_unnamed_0.int_reset;
                                    current_block = 10261677128829721533;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 10261677128829721533;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    current_block = 10261677128829721533;
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 9187258533171224838;
                        match current_block {
                            10830055424993547762 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .int_value = (*field).c2rust_unnamed_0.int_reset;
                                    current_block = 10261677128829721533;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 10261677128829721533;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    current_block = 10261677128829721533;
                                }
                            }
                        }
                    }
                    1 | _ => {
                        current_block = 10261677128829721533;
                    }
                }
            } else {
                ret = parse_int(buffer, &mut (*field).c2rust_unnamed.int_value);
                if ret != 0 {
                    current_block = 12393217337207382707;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if field_is_mandatory(field) {
                        current_block = 10261677128829721533;
                    } else {
                        if (*field).c2rust_unnamed.int_value == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                        } else if (*field).c2rust_unnamed.int_value
                            > 0 as libc::c_int as libc::c_long
                        {
                            (*field).c2rust_unnamed.int_value -= 1;
                            (*field).c2rust_unnamed.int_value;
                        }
                        current_block = 10261677128829721533;
                    }
                }
            }
        }
        2 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 => {
                        current_block = 735919960667768294;
                        match current_block {
                            7422578204891577835 => {
                                (*field).c2rust_unnamed.int_value += 1;
                                (*field).c2rust_unnamed.int_value;
                                current_block = 10261677128829721533;
                            }
                            735919960667768294 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .int_value = (*field).c2rust_unnamed_0.int_reset;
                                    current_block = 10261677128829721533;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 10261677128829721533;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    current_block = 10261677128829721533;
                                }
                            }
                        }
                    }
                    1 => {
                        current_block = 7422578204891577835;
                        match current_block {
                            7422578204891577835 => {
                                (*field).c2rust_unnamed.int_value += 1;
                                (*field).c2rust_unnamed.int_value;
                                current_block = 10261677128829721533;
                            }
                            735919960667768294 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .int_value = (*field).c2rust_unnamed_0.int_reset;
                                    current_block = 10261677128829721533;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 10261677128829721533;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    current_block = 10261677128829721533;
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 12304244970779737330;
                        match current_block {
                            7422578204891577835 => {
                                (*field).c2rust_unnamed.int_value += 1;
                                (*field).c2rust_unnamed.int_value;
                                current_block = 10261677128829721533;
                            }
                            735919960667768294 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .int_value = (*field).c2rust_unnamed_0.int_reset;
                                    current_block = 10261677128829721533;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 10261677128829721533;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 12393217337207382707;
                                } else {
                                    current_block = 10261677128829721533;
                                }
                            }
                        }
                    }
                    _ => {
                        current_block = 10261677128829721533;
                    }
                }
            } else {
                ret = parse_int(buffer, &mut (*field).c2rust_unnamed.int_value);
                if ret != 0 {
                    current_block = 12393217337207382707;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if field_is_mandatory(field) {
                        current_block = 10261677128829721533;
                    } else {
                        if (*field).c2rust_unnamed.int_value == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                        } else if (*field).c2rust_unnamed.int_value
                            > 0 as libc::c_int as libc::c_long
                        {
                            (*field).c2rust_unnamed.int_value -= 1;
                            (*field).c2rust_unnamed.int_value;
                        }
                        current_block = 10261677128829721533;
                    }
                }
            }
        }
        3 => {
            ret = parse_int(buffer, &mut tmp);
            if ret != 0 {
                current_block = 12393217337207382707;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                (*field).c2rust_unnamed.int_value += tmp;
                if field_is_mandatory(field) {
                    current_block = 10261677128829721533;
                } else {
                    if tmp == 0 {
                        (*field).state = FAST_STATE_EMPTY;
                    } else if tmp > 0 as libc::c_int as libc::c_long {
                        (*field).c2rust_unnamed.int_value -= 1;
                        (*field).c2rust_unnamed.int_value;
                    }
                    current_block = 10261677128829721533;
                }
            }
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 | 1 | 2 => {
                        if field_has_reset_value(field) {
                            (*field).state = FAST_STATE_ASSIGNED;
                            (*field)
                                .c2rust_unnamed
                                .int_value = (*field).c2rust_unnamed_0.int_reset;
                            current_block = 10261677128829721533;
                        } else if field_is_mandatory(field) {
                            ret = -(1 as libc::c_int);
                            current_block = 12393217337207382707;
                        } else {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 10261677128829721533;
                        }
                    }
                    _ => {
                        current_block = 10261677128829721533;
                    }
                }
            } else {
                ret = parse_int(buffer, &mut (*field).c2rust_unnamed.int_value);
                if ret != 0 {
                    current_block = 12393217337207382707;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if field_is_mandatory(field) {
                        current_block = 10261677128829721533;
                    } else {
                        if (*field).c2rust_unnamed.int_value == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                        } else if (*field).c2rust_unnamed.int_value
                            > 0 as libc::c_int as libc::c_long
                        {
                            (*field).c2rust_unnamed.int_value -= 1;
                            (*field).c2rust_unnamed.int_value;
                        }
                        current_block = 10261677128829721533;
                    }
                }
            }
        }
        5 => {
            if (*field).state as libc::c_uint
                != FAST_STATE_ASSIGNED as libc::c_int as libc::c_uint
            {
                (*field).c2rust_unnamed.int_value = (*field).c2rust_unnamed_0.int_reset;
            }
            (*field).state = FAST_STATE_ASSIGNED;
            if field_is_mandatory(field) {
                current_block = 10261677128829721533;
            } else {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                    (*field).state = FAST_STATE_EMPTY;
                }
                current_block = 10261677128829721533;
            }
        }
        _ => return -(1 as libc::c_int),
    }
    match current_block {
        12393217337207382707 => return ret,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn fast_decode_unicode(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut len: u64_0 = 0;
    match (*field).op as libc::c_uint {
        0 => {
            ret = parse_uint(buffer, &mut len);
            if ret != 0 {
                current_block = 16777306785825562595;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                if !field_is_mandatory(field) {
                    if len == 0 {
                        (*field).state = FAST_STATE_EMPTY;
                        current_block = 12758904613967585247;
                    } else {
                        len = len.wrapping_sub(1);
                        len;
                        current_block = 10886091980245723256;
                    }
                } else {
                    current_block = 10886091980245723256;
                }
                match current_block {
                    12758904613967585247 => {}
                    _ => {
                        ret = parse_bytes(
                            buffer,
                            ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                            len as libc::c_int,
                        );
                        if ret != 0 {
                            current_block = 16777306785825562595;
                        } else {
                            memset(
                                ((*field).c2rust_unnamed.string_value)
                                    .as_mut_ptr()
                                    .offset(len as isize) as *mut libc::c_void,
                                0 as libc::c_int,
                                ::core::mem::size_of::<u64_0>() as libc::c_ulong,
                            );
                            current_block = 12758904613967585247;
                        }
                    }
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 => {
                        current_block = 12702584525077410494;
                        match current_block {
                            12702584525077410494 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    memcpy(
                                        ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                                            as *mut libc::c_void,
                                        ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()
                                            as *const libc::c_void,
                                        (strlen(
                                            ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr(),
                                        ))
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                    current_block = 12758904613967585247;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 16777306785825562595;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 12758904613967585247;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 16777306785825562595;
                                } else {
                                    current_block = 12758904613967585247;
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 3828254842371226310;
                        match current_block {
                            12702584525077410494 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    memcpy(
                                        ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                                            as *mut libc::c_void,
                                        ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()
                                            as *const libc::c_void,
                                        (strlen(
                                            ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr(),
                                        ))
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                    current_block = 12758904613967585247;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 16777306785825562595;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 12758904613967585247;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 16777306785825562595;
                                } else {
                                    current_block = 12758904613967585247;
                                }
                            }
                        }
                    }
                    1 | _ => {
                        current_block = 12758904613967585247;
                    }
                }
            } else {
                ret = parse_uint(buffer, &mut len);
                if ret != 0 {
                    current_block = 16777306785825562595;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if !field_is_mandatory(field) {
                        if len == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 12758904613967585247;
                        } else {
                            len = len.wrapping_sub(1);
                            len;
                            current_block = 15897653523371991391;
                        }
                    } else {
                        current_block = 15897653523371991391;
                    }
                    match current_block {
                        12758904613967585247 => {}
                        _ => {
                            ret = parse_bytes(
                                buffer,
                                ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                                len as libc::c_int,
                            );
                            if ret != 0 {
                                current_block = 16777306785825562595;
                            } else {
                                memset(
                                    ((*field).c2rust_unnamed.string_value)
                                        .as_mut_ptr()
                                        .offset(len as isize) as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ::core::mem::size_of::<u64_0>() as libc::c_ulong,
                                );
                                current_block = 12758904613967585247;
                            }
                        }
                    }
                }
            }
        }
        2 => {
            ret = -(1 as libc::c_int);
            current_block = 16777306785825562595;
        }
        3 => {
            ret = -(1 as libc::c_int);
            current_block = 16777306785825562595;
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 | 1 | 2 => {
                        if field_has_reset_value(field) {
                            (*field).state = FAST_STATE_ASSIGNED;
                            memcpy(
                                ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                                    as *mut libc::c_void,
                                ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()
                                    as *const libc::c_void,
                                (strlen(
                                    ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr(),
                                ))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                            current_block = 12758904613967585247;
                        } else if field_is_mandatory(field) {
                            ret = -(1 as libc::c_int);
                            current_block = 16777306785825562595;
                        } else {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 12758904613967585247;
                        }
                    }
                    _ => {
                        current_block = 12758904613967585247;
                    }
                }
            } else {
                ret = parse_uint(buffer, &mut len);
                if ret != 0 {
                    current_block = 16777306785825562595;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if !field_is_mandatory(field) {
                        if len == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 12758904613967585247;
                        } else {
                            len = len.wrapping_sub(1);
                            len;
                            current_block = 8347882395825654554;
                        }
                    } else {
                        current_block = 8347882395825654554;
                    }
                    match current_block {
                        12758904613967585247 => {}
                        _ => {
                            ret = parse_bytes(
                                buffer,
                                ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                                len as libc::c_int,
                            );
                            if ret != 0 {
                                current_block = 16777306785825562595;
                            } else {
                                memset(
                                    ((*field).c2rust_unnamed.string_value)
                                        .as_mut_ptr()
                                        .offset(len as isize) as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ::core::mem::size_of::<u64_0>() as libc::c_ulong,
                                );
                                current_block = 12758904613967585247;
                            }
                        }
                    }
                }
            }
        }
        5 => {
            if (*field).state as libc::c_uint
                != FAST_STATE_ASSIGNED as libc::c_int as libc::c_uint
            {
                memcpy(
                    ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                        as *mut libc::c_void,
                    ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()
                        as *const libc::c_void,
                    (strlen(((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            }
            (*field).state = FAST_STATE_ASSIGNED;
            if field_is_mandatory(field) {
                current_block = 12758904613967585247;
            } else {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                    (*field).state = FAST_STATE_EMPTY;
                }
                current_block = 12758904613967585247;
            }
        }
        _ => return -(1 as libc::c_int),
    }
    match current_block {
        16777306785825562595 => return ret,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn fast_decode_ascii(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut delta: [libc::c_char; 32] = [0; 32];
    let mut lenb: i64_0 = 0;
    let mut lend: i64_0 = 0;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: i64_0 = 0;
    let mut ret: libc::c_int = 0;
    match (*field).op as libc::c_uint {
        0 => {
            ret = parse_string(
                buffer,
                ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                256 as libc::c_int as size_t,
            );
            if ret < 0 as libc::c_int {
                current_block = 8095853014078906026;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                if field_is_mandatory(field) {
                    current_block = 16108440464692313034;
                } else {
                    if ret == 1 as libc::c_int
                        && (*field)
                            .c2rust_unnamed
                            .string_value[0 as libc::c_int as usize] == 0
                    {
                        (*field).state = FAST_STATE_EMPTY;
                    }
                    current_block = 16108440464692313034;
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 => {
                        current_block = 5280017764795594125;
                        match current_block {
                            5280017764795594125 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    memcpy(
                                        ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                                            as *mut libc::c_void,
                                        ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()
                                            as *const libc::c_void,
                                        (strlen(
                                            ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr(),
                                        ))
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                    current_block = 16108440464692313034;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 8095853014078906026;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 16108440464692313034;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 8095853014078906026;
                                } else {
                                    current_block = 16108440464692313034;
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 14999487078017347527;
                        match current_block {
                            5280017764795594125 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    memcpy(
                                        ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                                            as *mut libc::c_void,
                                        ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()
                                            as *const libc::c_void,
                                        (strlen(
                                            ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr(),
                                        ))
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                                    );
                                    current_block = 16108440464692313034;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 8095853014078906026;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 16108440464692313034;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 8095853014078906026;
                                } else {
                                    current_block = 16108440464692313034;
                                }
                            }
                        }
                    }
                    1 | _ => {
                        current_block = 16108440464692313034;
                    }
                }
            } else {
                ret = parse_string(
                    buffer,
                    ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                    256 as libc::c_int as size_t,
                );
                if ret < 0 as libc::c_int {
                    current_block = 8095853014078906026;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if field_is_mandatory(field) {
                        current_block = 16108440464692313034;
                    } else {
                        if ret == 1 as libc::c_int
                            && (*field)
                                .c2rust_unnamed
                                .string_value[0 as libc::c_int as usize] == 0
                        {
                            (*field).state = FAST_STATE_EMPTY;
                        }
                        current_block = 16108440464692313034;
                    }
                }
            }
        }
        2 => {
            ret = -(1 as libc::c_int);
            current_block = 8095853014078906026;
        }
        3 => {
            ret = parse_int(buffer, &mut length);
            if ret != 0 {
                current_block = 8095853014078906026;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                if !field_is_mandatory(field) {
                    if length == 0 {
                        (*field).state = FAST_STATE_EMPTY;
                        current_block = 16108440464692313034;
                    } else {
                        if length > 0 as libc::c_int as libc::c_long {
                            length -= 1;
                            length;
                        }
                        current_block = 14832935472441733737;
                    }
                } else {
                    current_block = 14832935472441733737;
                }
                match current_block {
                    16108440464692313034 => {}
                    _ => {
                        ret = parse_string(
                            buffer,
                            delta.as_mut_ptr(),
                            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                        );
                        if ret < 0 as libc::c_int {
                            current_block = 8095853014078906026;
                        } else {
                            base = ((*field).c2rust_unnamed.string_value).as_mut_ptr();
                            lenb = strlen(base) as i64_0;
                            lend = strlen(delta.as_mut_ptr()) as i64_0;
                            if length >= 0 as libc::c_int as libc::c_long {
                                if lenb < length {
                                    ret = -(1 as libc::c_int);
                                    current_block = 8095853014078906026;
                                } else if lenb + lend - length
                                    + 1 as libc::c_int as libc::c_long
                                    > 256 as libc::c_int as libc::c_long
                                {
                                    ret = -(1 as libc::c_int);
                                    current_block = 8095853014078906026;
                                } else {
                                    strncpy(
                                        base.offset(lenb as isize).offset(-(length as isize)),
                                        delta.as_mut_ptr(),
                                        (lend + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                                    );
                                    current_block = 16108440464692313034;
                                }
                            } else {
                                length = -(length + 1 as libc::c_int as libc::c_long);
                                if strlen(base) < length as libc::c_ulong {
                                    ret = -(1 as libc::c_int);
                                    current_block = 8095853014078906026;
                                } else if lenb + lend - length
                                    + 1 as libc::c_int as libc::c_long
                                    > 256 as libc::c_int as libc::c_long
                                {
                                    ret = -(1 as libc::c_int);
                                    current_block = 8095853014078906026;
                                } else {
                                    memmove(
                                        base.offset(lend as isize) as *mut libc::c_void,
                                        base.offset(length as isize) as *const libc::c_void,
                                        (lenb - length + 1 as libc::c_int as libc::c_long)
                                            as libc::c_ulong,
                                    );
                                    strncpy(base, delta.as_mut_ptr(), lend as libc::c_ulong);
                                    current_block = 16108440464692313034;
                                }
                            }
                        }
                    }
                }
            }
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 | 1 | 2 => {
                        if field_has_reset_value(field) {
                            (*field).state = FAST_STATE_ASSIGNED;
                            memcpy(
                                ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                                    as *mut libc::c_void,
                                ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()
                                    as *const libc::c_void,
                                (strlen(
                                    ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr(),
                                ))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                            );
                            current_block = 16108440464692313034;
                        } else if field_is_mandatory(field) {
                            ret = -(1 as libc::c_int);
                            current_block = 8095853014078906026;
                        } else {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 16108440464692313034;
                        }
                    }
                    _ => {
                        current_block = 16108440464692313034;
                    }
                }
            } else {
                ret = parse_string(
                    buffer,
                    ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                    256 as libc::c_int as size_t,
                );
                if ret < 0 as libc::c_int {
                    current_block = 8095853014078906026;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if field_is_mandatory(field) {
                        current_block = 16108440464692313034;
                    } else {
                        if ret == 1 as libc::c_int
                            && (*field)
                                .c2rust_unnamed
                                .string_value[0 as libc::c_int as usize] == 0
                        {
                            (*field).state = FAST_STATE_EMPTY;
                        }
                        current_block = 16108440464692313034;
                    }
                }
            }
        }
        5 => {
            if (*field).state as libc::c_uint
                != FAST_STATE_ASSIGNED as libc::c_int as libc::c_uint
            {
                memcpy(
                    ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                        as *mut libc::c_void,
                    ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()
                        as *const libc::c_void,
                    (strlen(((*field).c2rust_unnamed_0.string_reset).as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
            }
            (*field).state = FAST_STATE_ASSIGNED;
            if field_is_mandatory(field) {
                current_block = 16108440464692313034;
            } else {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                    (*field).state = FAST_STATE_EMPTY;
                }
                current_block = 16108440464692313034;
            }
        }
        _ => return -(1 as libc::c_int),
    }
    match current_block {
        8095853014078906026 => return ret,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn fast_decode_string(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    if field_has_flags(field, 0x1 as libc::c_int) != 0 {
        return fast_decode_unicode(buffer, pmap, field)
    } else {
        return fast_decode_ascii(buffer, pmap, field)
    };
}
unsafe extern "C" fn fast_decode_decimal_atomic(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut exp: i64_0 = 0;
    let mut mnt: i64_0 = 0;
    match (*field).op as libc::c_uint {
        0 => {
            ret = parse_int(buffer, &mut exp);
            if ret != 0 {
                current_block = 2180010920890925313;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                if !field_is_mandatory(field) {
                    if exp == 0 {
                        (*field).state = FAST_STATE_EMPTY;
                        current_block = 10369920510435091891;
                    } else {
                        if exp > 0 as libc::c_int as libc::c_long {
                            exp -= 1;
                            exp;
                        }
                        current_block = 5720623009719927633;
                    }
                } else {
                    current_block = 5720623009719927633;
                }
                match current_block {
                    10369920510435091891 => {}
                    _ => {
                        if exp > 63 as libc::c_int as libc::c_long
                            || exp < -(63 as libc::c_int) as libc::c_long
                        {
                            ret = -(1 as libc::c_int);
                            current_block = 2180010920890925313;
                        } else {
                            ret = parse_int(buffer, &mut mnt);
                            if ret != 0 {
                                current_block = 2180010920890925313;
                            } else {
                                (*field).c2rust_unnamed.decimal_value.exp = exp;
                                (*field).c2rust_unnamed.decimal_value.mnt = mnt;
                                current_block = 10369920510435091891;
                            }
                        }
                    }
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 => {
                        current_block = 6402543970446428107;
                        match current_block {
                            6402543970446428107 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .decimal_value
                                        .exp = (*field).c2rust_unnamed_0.decimal_reset.exp;
                                    (*field)
                                        .c2rust_unnamed
                                        .decimal_value
                                        .mnt = (*field).c2rust_unnamed_0.decimal_reset.mnt;
                                    current_block = 10369920510435091891;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 2180010920890925313;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 10369920510435091891;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 2180010920890925313;
                                } else {
                                    current_block = 10369920510435091891;
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 10905962476644674133;
                        match current_block {
                            6402543970446428107 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    (*field)
                                        .c2rust_unnamed
                                        .decimal_value
                                        .exp = (*field).c2rust_unnamed_0.decimal_reset.exp;
                                    (*field)
                                        .c2rust_unnamed
                                        .decimal_value
                                        .mnt = (*field).c2rust_unnamed_0.decimal_reset.mnt;
                                    current_block = 10369920510435091891;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 2180010920890925313;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 10369920510435091891;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 2180010920890925313;
                                } else {
                                    current_block = 10369920510435091891;
                                }
                            }
                        }
                    }
                    1 | _ => {
                        current_block = 10369920510435091891;
                    }
                }
            } else {
                ret = parse_int(buffer, &mut exp);
                if ret != 0 {
                    current_block = 2180010920890925313;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if !field_is_mandatory(field) {
                        if exp == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 10369920510435091891;
                        } else {
                            if exp > 0 as libc::c_int as libc::c_long {
                                exp -= 1;
                                exp;
                            }
                            current_block = 11048769245176032998;
                        }
                    } else {
                        current_block = 11048769245176032998;
                    }
                    match current_block {
                        10369920510435091891 => {}
                        _ => {
                            if exp > 63 as libc::c_int as libc::c_long
                                || exp < -(63 as libc::c_int) as libc::c_long
                            {
                                ret = -(1 as libc::c_int);
                                current_block = 2180010920890925313;
                            } else {
                                ret = parse_int(buffer, &mut mnt);
                                if ret != 0 {
                                    current_block = 2180010920890925313;
                                } else {
                                    (*field).c2rust_unnamed.decimal_value.exp = exp;
                                    (*field).c2rust_unnamed.decimal_value.mnt = mnt;
                                    current_block = 10369920510435091891;
                                }
                            }
                        }
                    }
                }
            }
        }
        2 => {
            ret = -(1 as libc::c_int);
            current_block = 2180010920890925313;
        }
        3 => {
            ret = parse_int(buffer, &mut exp);
            if ret != 0 {
                current_block = 2180010920890925313;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                if !field_is_mandatory(field) {
                    if exp == 0 {
                        (*field).state = FAST_STATE_EMPTY;
                        current_block = 10369920510435091891;
                    } else {
                        if exp > 0 as libc::c_int as libc::c_long {
                            exp -= 1;
                            exp;
                        }
                        current_block = 12556861819962772176;
                    }
                } else {
                    current_block = 12556861819962772176;
                }
                match current_block {
                    10369920510435091891 => {}
                    _ => {
                        if (*field).c2rust_unnamed.decimal_value.exp + exp
                            > 63 as libc::c_int as libc::c_long
                            || (*field).c2rust_unnamed.decimal_value.exp + exp
                                < -(63 as libc::c_int) as libc::c_long
                        {
                            ret = -(1 as libc::c_int);
                            current_block = 2180010920890925313;
                        } else {
                            ret = parse_int(buffer, &mut mnt);
                            if ret != 0 {
                                current_block = 2180010920890925313;
                            } else {
                                (*field).c2rust_unnamed.decimal_value.exp += exp;
                                (*field).c2rust_unnamed.decimal_value.mnt += mnt;
                                current_block = 10369920510435091891;
                            }
                        }
                    }
                }
            }
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 | 1 | 2 => {
                        if field_has_reset_value(field) {
                            (*field).state = FAST_STATE_ASSIGNED;
                            (*field)
                                .c2rust_unnamed
                                .decimal_value
                                .exp = (*field).c2rust_unnamed_0.decimal_reset.exp;
                            (*field)
                                .c2rust_unnamed
                                .decimal_value
                                .mnt = (*field).c2rust_unnamed_0.decimal_reset.mnt;
                            current_block = 10369920510435091891;
                        } else if field_is_mandatory(field) {
                            ret = -(1 as libc::c_int);
                            current_block = 2180010920890925313;
                        } else {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 10369920510435091891;
                        }
                    }
                    _ => {
                        current_block = 10369920510435091891;
                    }
                }
            } else {
                ret = parse_int(buffer, &mut exp);
                if ret != 0 {
                    current_block = 2180010920890925313;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if !field_is_mandatory(field) {
                        if exp == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 10369920510435091891;
                        } else {
                            if exp > 0 as libc::c_int as libc::c_long {
                                exp -= 1;
                                exp;
                            }
                            current_block = 5658374378798827547;
                        }
                    } else {
                        current_block = 5658374378798827547;
                    }
                    match current_block {
                        10369920510435091891 => {}
                        _ => {
                            if exp > 63 as libc::c_int as libc::c_long
                                || exp < -(63 as libc::c_int) as libc::c_long
                            {
                                ret = -(1 as libc::c_int);
                                current_block = 2180010920890925313;
                            } else {
                                ret = parse_int(buffer, &mut mnt);
                                if ret != 0 {
                                    current_block = 2180010920890925313;
                                } else {
                                    (*field).c2rust_unnamed.decimal_value.exp = exp;
                                    (*field).c2rust_unnamed.decimal_value.mnt = mnt;
                                    current_block = 10369920510435091891;
                                }
                            }
                        }
                    }
                }
            }
        }
        5 => {
            if (*field).state as libc::c_uint
                != FAST_STATE_ASSIGNED as libc::c_int as libc::c_uint
            {
                (*field)
                    .c2rust_unnamed
                    .decimal_value
                    .exp = (*field).c2rust_unnamed_0.decimal_reset.exp;
                (*field)
                    .c2rust_unnamed
                    .decimal_value
                    .mnt = (*field).c2rust_unnamed_0.decimal_reset.mnt;
            }
            (*field).state = FAST_STATE_ASSIGNED;
            if field_is_mandatory(field) {
                current_block = 10369920510435091891;
            } else {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                    (*field).state = FAST_STATE_EMPTY;
                }
                current_block = 10369920510435091891;
            }
        }
        _ => return -(1 as libc::c_int),
    }
    match current_block {
        2180010920890925313 => return ret,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn fast_decode_decimal_individ(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut decimal: *mut fast_decimal = 0 as *mut fast_decimal;
    decimal = &mut (*field).c2rust_unnamed.decimal_value;
    ret = fast_decode_int(
        buffer,
        pmap,
        &mut *((*decimal).fields).offset(0 as libc::c_int as isize),
    );
    if !(ret != 0) {
        if field_state_empty(
            &mut *((*decimal).fields).offset(0 as libc::c_int as isize),
        ) {
            if field_is_mandatory(field) {
                ret = -(1 as libc::c_int);
            }
            (*field).state = FAST_STATE_EMPTY;
        } else if (*((*decimal).fields).offset(0 as libc::c_int as isize))
            .c2rust_unnamed
            .int_value > 63 as libc::c_int as libc::c_long
            || (*((*decimal).fields).offset(0 as libc::c_int as isize))
                .c2rust_unnamed
                .int_value < -(63 as libc::c_int) as libc::c_long
        {
            ret = -(1 as libc::c_int);
        } else {
            ret = fast_decode_int(
                buffer,
                pmap,
                &mut *((*decimal).fields).offset(1 as libc::c_int as isize),
            );
            if !(ret != 0) {
                (*field).state = FAST_STATE_ASSIGNED;
                (*decimal)
                    .exp = (*((*decimal).fields).offset(0 as libc::c_int as isize))
                    .c2rust_unnamed
                    .int_value;
                (*decimal)
                    .mnt = (*((*decimal).fields).offset(1 as libc::c_int as isize))
                    .c2rust_unnamed
                    .int_value;
            }
        }
    }
    return ret;
}
unsafe extern "C" fn fast_decode_decimal(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    if field_has_flags(field, 0x4 as libc::c_int) == 0 {
        return fast_decode_decimal_atomic(buffer, pmap, field)
    } else {
        return fast_decode_decimal_individ(buffer, pmap, field)
    };
}
unsafe extern "C" fn fast_decode_vector(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut len: u64_0 = 0;
    match (*field).op as libc::c_uint {
        0 => {
            ret = parse_uint(buffer, &mut len);
            if ret != 0 {
                current_block = 7195696641523860423;
            } else {
                (*field).state = FAST_STATE_ASSIGNED;
                if !field_is_mandatory(field) {
                    if len == 0 {
                        (*field).state = FAST_STATE_EMPTY;
                        current_block = 12758904613967585247;
                    } else {
                        len = len.wrapping_sub(1);
                        len;
                        current_block = 10886091980245723256;
                    }
                } else {
                    current_block = 10886091980245723256;
                }
                match current_block {
                    12758904613967585247 => {}
                    _ => {
                        ret = parse_bytes(
                            buffer,
                            ((*field).c2rust_unnamed.vector_value).as_mut_ptr(),
                            len as libc::c_int,
                        );
                        if ret != 0 {
                            current_block = 7195696641523860423;
                        } else {
                            memset(
                                ((*field).c2rust_unnamed.vector_value)
                                    .as_mut_ptr()
                                    .offset(len as isize) as *mut libc::c_void,
                                0 as libc::c_int,
                                ::core::mem::size_of::<u64_0>() as libc::c_ulong,
                            );
                            current_block = 12758904613967585247;
                        }
                    }
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 => {
                        current_block = 8385920623611837023;
                        match current_block {
                            8385920623611837023 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    memcpy(
                                        ((*field).c2rust_unnamed.vector_value).as_mut_ptr()
                                            as *mut libc::c_void,
                                        ((*field).c2rust_unnamed_0.vector_reset).as_mut_ptr()
                                            as *const libc::c_void,
                                        ::core::mem::size_of::<[libc::c_char; 256]>()
                                            as libc::c_ulong,
                                    );
                                    current_block = 12758904613967585247;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 7195696641523860423;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 12758904613967585247;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 7195696641523860423;
                                } else {
                                    current_block = 12758904613967585247;
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 8068347855768005278;
                        match current_block {
                            8385920623611837023 => {
                                if field_has_reset_value(field) {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    memcpy(
                                        ((*field).c2rust_unnamed.vector_value).as_mut_ptr()
                                            as *mut libc::c_void,
                                        ((*field).c2rust_unnamed_0.vector_reset).as_mut_ptr()
                                            as *const libc::c_void,
                                        ::core::mem::size_of::<[libc::c_char; 256]>()
                                            as libc::c_ulong,
                                    );
                                    current_block = 12758904613967585247;
                                } else if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 7195696641523860423;
                                } else {
                                    (*field).state = FAST_STATE_EMPTY;
                                    current_block = 12758904613967585247;
                                }
                            }
                            _ => {
                                if field_is_mandatory(field) {
                                    ret = -(1 as libc::c_int);
                                    current_block = 7195696641523860423;
                                } else {
                                    current_block = 12758904613967585247;
                                }
                            }
                        }
                    }
                    1 | _ => {
                        current_block = 12758904613967585247;
                    }
                }
            } else {
                ret = parse_uint(buffer, &mut len);
                if ret != 0 {
                    current_block = 7195696641523860423;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if !field_is_mandatory(field) {
                        if len == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 12758904613967585247;
                        } else {
                            len = len.wrapping_sub(1);
                            len;
                            current_block = 15897653523371991391;
                        }
                    } else {
                        current_block = 15897653523371991391;
                    }
                    match current_block {
                        12758904613967585247 => {}
                        _ => {
                            ret = parse_bytes(
                                buffer,
                                ((*field).c2rust_unnamed.vector_value).as_mut_ptr(),
                                len as libc::c_int,
                            );
                            if ret != 0 {
                                current_block = 7195696641523860423;
                            } else {
                                memset(
                                    ((*field).c2rust_unnamed.vector_value)
                                        .as_mut_ptr()
                                        .offset(len as isize) as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ::core::mem::size_of::<u64_0>() as libc::c_ulong,
                                );
                                current_block = 12758904613967585247;
                            }
                        }
                    }
                }
            }
        }
        2 => {
            ret = -(1 as libc::c_int);
            current_block = 7195696641523860423;
        }
        3 => {
            ret = -(1 as libc::c_int);
            current_block = 7195696641523860423;
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                match (*field).state as libc::c_uint {
                    0 | 1 | 2 => {
                        if field_has_reset_value(field) {
                            (*field).state = FAST_STATE_ASSIGNED;
                            memcpy(
                                ((*field).c2rust_unnamed.vector_value).as_mut_ptr()
                                    as *mut libc::c_void,
                                ((*field).c2rust_unnamed_0.vector_reset).as_mut_ptr()
                                    as *const libc::c_void,
                                ::core::mem::size_of::<[libc::c_char; 256]>()
                                    as libc::c_ulong,
                            );
                            current_block = 12758904613967585247;
                        } else if field_is_mandatory(field) {
                            ret = -(1 as libc::c_int);
                            current_block = 7195696641523860423;
                        } else {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 12758904613967585247;
                        }
                    }
                    _ => {
                        current_block = 12758904613967585247;
                    }
                }
            } else {
                ret = parse_uint(buffer, &mut len);
                if ret != 0 {
                    current_block = 7195696641523860423;
                } else {
                    (*field).state = FAST_STATE_ASSIGNED;
                    if !field_is_mandatory(field) {
                        if len == 0 {
                            (*field).state = FAST_STATE_EMPTY;
                            current_block = 12758904613967585247;
                        } else {
                            len = len.wrapping_sub(1);
                            len;
                            current_block = 8347882395825654554;
                        }
                    } else {
                        current_block = 8347882395825654554;
                    }
                    match current_block {
                        12758904613967585247 => {}
                        _ => {
                            ret = parse_bytes(
                                buffer,
                                ((*field).c2rust_unnamed.vector_value).as_mut_ptr(),
                                len as libc::c_int,
                            );
                            if ret != 0 {
                                current_block = 7195696641523860423;
                            } else {
                                memset(
                                    ((*field).c2rust_unnamed.vector_value)
                                        .as_mut_ptr()
                                        .offset(len as isize) as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ::core::mem::size_of::<u64_0>() as libc::c_ulong,
                                );
                                current_block = 12758904613967585247;
                            }
                        }
                    }
                }
            }
        }
        5 => {
            if (*field).state as libc::c_uint
                != FAST_STATE_ASSIGNED as libc::c_int as libc::c_uint
            {
                memcpy(
                    ((*field).c2rust_unnamed.vector_value).as_mut_ptr()
                        as *mut libc::c_void,
                    ((*field).c2rust_unnamed_0.vector_reset).as_mut_ptr()
                        as *const libc::c_void,
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                );
            }
            (*field).state = FAST_STATE_ASSIGNED;
            if field_is_mandatory(field) {
                current_block = 12758904613967585247;
            } else {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !pmap_is_set(pmap, (*pmap).pmap_bit as libc::c_ulong) {
                    (*field).state = FAST_STATE_EMPTY;
                }
                current_block = 12758904613967585247;
            }
        }
        _ => return -(1 as libc::c_int),
    }
    match current_block {
        7195696641523860423 => return ret,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn fast_decode_sequence(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut seq: *mut fast_sequence = 0 as *mut fast_sequence;
    let mut msg: *mut fast_message = 0 as *mut fast_message;
    let mut spmap: *mut fast_pmap = 0 as *mut fast_pmap;
    let mut cur: *mut fast_field = 0 as *mut fast_field;
    let mut spmap_bit: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut pmap_req: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    seq = (*field).c2rust_unnamed.ptr_value as *mut fast_sequence;
    spmap = 0 as *mut fast_pmap;
    msg = 0 as *mut fast_message;
    if (*seq).decoded == 0 {
        start = buffer_start(buffer);
        ret = fast_decode_uint(buffer, pmap, &mut (*seq).length);
        if ret != 0 {
            current_block = 14647476514036352060;
        } else if field_state_empty(&mut (*seq).length) {
            if field_is_mandatory(field) {
                ret = -(1 as libc::c_int);
            }
            current_block = 14647476514036352060;
        } else if (*seq).length.c2rust_unnamed.uint_value
            >= 128 as libc::c_int as libc::c_ulong
        {
            ret = -(1 as libc::c_int);
            current_block = 14647476514036352060;
        } else {
            (*seq).decoded = 1 as libc::c_int as libc::c_ulong;
            current_block = 3512920355445576850;
        }
    } else {
        current_block = 3512920355445576850;
    }
    match current_block {
        3512920355445576850 => {
            pmap_req = field_has_flags(field, 0x2 as libc::c_int);
            spmap = &mut (*seq).pmap;
            's_79: loop {
                if !((*seq).decoded <= (*seq).length.c2rust_unnamed.uint_value) {
                    current_block = 14945149239039849694;
                    break;
                }
                if pmap_req != 0 && !(*spmap).is_valid {
                    start = buffer_start(buffer);
                    ret = parse_pmap(buffer, spmap);
                    if ret != 0 {
                        current_block = 14647476514036352060;
                        break;
                    }
                    (*spmap).is_valid = 1 as libc::c_int != 0;
                    (*spmap).pmap_bit = -(1 as libc::c_int) as libc::c_long;
                }
                msg = ((*seq).elements).as_mut_ptr();
                while (*msg).decoded < (*msg).nr_fields {
                    cur = ((*msg.offset((*seq).decoded as isize)).fields)
                        .offset((*msg).decoded as isize);
                    field = ((*msg).fields).offset((*msg).decoded as isize);
                    start = buffer_start(buffer);
                    spmap_bit = (*spmap).pmap_bit;
                    match (*field).type_0 as libc::c_uint {
                        0 => {
                            ret = fast_decode_int(buffer, spmap, field);
                            if ret != 0 {
                                current_block = 14647476514036352060;
                                break 's_79;
                            }
                            (*cur)
                                .c2rust_unnamed
                                .int_value = (*field).c2rust_unnamed.int_value;
                            (*cur).state = (*field).state;
                        }
                        1 => {
                            ret = fast_decode_uint(buffer, spmap, field);
                            if ret != 0 {
                                current_block = 14647476514036352060;
                                break 's_79;
                            }
                            (*cur)
                                .c2rust_unnamed
                                .uint_value = (*field).c2rust_unnamed.uint_value;
                            (*cur).state = (*field).state;
                        }
                        2 => {
                            ret = fast_decode_string(buffer, spmap, field);
                            if ret != 0 {
                                current_block = 14647476514036352060;
                                break 's_79;
                            }
                            strcpy(
                                ((*cur).c2rust_unnamed.string_value).as_mut_ptr(),
                                ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                            );
                            (*cur).state = (*field).state;
                        }
                        3 => {
                            ret = fast_decode_vector(buffer, spmap, field);
                            if ret != 0 {
                                current_block = 14647476514036352060;
                                break 's_79;
                            }
                        }
                        4 => {
                            ret = fast_decode_decimal(buffer, spmap, field);
                            if ret != 0 {
                                current_block = 14647476514036352060;
                                break 's_79;
                            }
                            (*cur)
                                .c2rust_unnamed
                                .decimal_value
                                .exp = (*field).c2rust_unnamed.decimal_value.exp;
                            (*cur)
                                .c2rust_unnamed
                                .decimal_value
                                .mnt = (*field).c2rust_unnamed.decimal_value.mnt;
                            (*cur).state = (*field).state;
                        }
                        5 => {
                            ret = -(1 as libc::c_int);
                            if ret != 0 {
                                current_block = 14647476514036352060;
                                break 's_79;
                            }
                        }
                        _ => {
                            ret = -(1 as libc::c_int);
                            current_block = 14647476514036352060;
                            break 's_79;
                        }
                    }
                    (*msg).decoded = ((*msg).decoded).wrapping_add(1);
                    (*msg).decoded;
                }
                (*spmap).is_valid = 0 as libc::c_int != 0;
                (*msg).decoded = 0 as libc::c_int as libc::c_ulong;
                (*seq).decoded = ((*seq).decoded).wrapping_add(1);
                (*seq).decoded;
            }
            match current_block {
                14647476514036352060 => {}
                _ => {
                    (*seq).decoded = 0 as libc::c_int as libc::c_ulong;
                }
            }
        }
        _ => {}
    }
    if ret == -(2 as libc::c_int) {
        buffer_advance(buffer, start.offset_from(buffer_start(buffer)) as libc::c_long);
        if !msg.is_null() {
            (*spmap).pmap_bit = spmap_bit;
        }
    }
    return ret;
}
#[inline]
unsafe extern "C" fn fast_get_msg(
    mut msgs: *mut fast_message,
    mut tid: libc::c_int,
) -> *mut fast_message {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        if (*msgs.offset(i as isize)).tid == tid as libc::c_ulong {
            return &mut *msgs.offset(i as isize) as *mut fast_message;
        }
        i += 1;
        i;
    }
    return 0 as *mut fast_message;
}
#[no_mangle]
pub unsafe extern "C" fn fast_message_decode(
    mut session: *mut fast_session,
) -> *mut fast_message {
    let mut current_block: u64;
    let mut preamble: *mut fast_preamble = 0 as *mut fast_preamble;
    let mut msg: *mut fast_message = 0 as *mut fast_message;
    let mut field: *mut fast_field = 0 as *mut fast_field;
    let mut pmap: *mut fast_pmap = 0 as *mut fast_pmap;
    let mut buffer: *mut buffer = 0 as *mut buffer;
    let mut pmap_bit: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut tid: u64_0 = 0;
    let mut ret: libc::c_int = 0;
    preamble = &mut (*session).preamble;
    buffer = (*session).rx_buffer;
    pmap = &mut (*session).pmap;
    msg = 0 as *mut fast_message;
    if (*preamble).nr_bytes != 0 && !(*preamble).is_valid {
        start = buffer_start(buffer);
        ret = -(2 as libc::c_int);
        if buffer_size(buffer) < (*preamble).nr_bytes {
            current_block = 6023672673073491528;
        } else {
            buffer_get_n(
                buffer,
                (*preamble).nr_bytes as libc::c_int,
                ((*preamble).bytes).as_mut_ptr(),
            );
            (*preamble).is_valid = 1 as libc::c_int != 0;
            current_block = 2868539653012386629;
        }
    } else {
        current_block = 2868539653012386629;
    }
    match current_block {
        2868539653012386629 => {
            if !(*pmap).is_valid {
                start = buffer_start(buffer);
                ret = parse_pmap(buffer, pmap);
                if ret != 0 {
                    current_block = 6023672673073491528;
                } else {
                    (*pmap).is_valid = 1 as libc::c_int != 0;
                    (*pmap).pmap_bit = 0 as libc::c_int as libc::c_long;
                    current_block = 9606288038608642794;
                }
            } else {
                current_block = 9606288038608642794;
            }
            match current_block {
                6023672673073491528 => {}
                _ => {
                    if ((*session).rx_message).is_null() {
                        start = buffer_start(buffer);
                        if pmap_is_set(pmap, 0 as libc::c_int as libc::c_ulong) {
                            ret = parse_uint(buffer, &mut tid);
                            if ret != 0 {
                                current_block = 6023672673073491528;
                            } else {
                                current_block = 26972500619410423;
                            }
                        } else {
                            tid = (*session).last_tid;
                            current_block = 26972500619410423;
                        }
                        match current_block {
                            6023672673073491528 => {}
                            _ => {
                                (*session)
                                    .rx_message = fast_get_msg(
                                    (*session).rx_messages,
                                    tid as libc::c_int,
                                );
                                if ((*session).rx_message).is_null() {
                                    ret = -(1 as libc::c_int);
                                    current_block = 6023672673073491528;
                                } else {
                                    current_block = 10652014663920648156;
                                }
                            }
                        }
                    } else {
                        current_block = 10652014663920648156;
                    }
                    match current_block {
                        6023672673073491528 => {}
                        _ => {
                            msg = (*session).rx_message;
                            loop {
                                if !((*msg).decoded < (*msg).nr_fields) {
                                    current_block = 12381812505308290051;
                                    break;
                                }
                                field = ((*msg).fields).offset((*msg).decoded as isize);
                                start = buffer_start(buffer);
                                pmap_bit = (*pmap).pmap_bit;
                                match (*field).type_0 as libc::c_uint {
                                    0 => {
                                        ret = fast_decode_int(buffer, pmap, field);
                                        if ret != 0 {
                                            current_block = 6023672673073491528;
                                            break;
                                        }
                                    }
                                    1 => {
                                        ret = fast_decode_uint(buffer, pmap, field);
                                        if ret != 0 {
                                            current_block = 6023672673073491528;
                                            break;
                                        }
                                    }
                                    2 => {
                                        ret = fast_decode_string(buffer, pmap, field);
                                        if ret != 0 {
                                            current_block = 6023672673073491528;
                                            break;
                                        }
                                    }
                                    3 => {
                                        ret = fast_decode_vector(buffer, pmap, field);
                                        if ret != 0 {
                                            current_block = 6023672673073491528;
                                            break;
                                        }
                                    }
                                    4 => {
                                        ret = fast_decode_decimal(buffer, pmap, field);
                                        if ret != 0 {
                                            current_block = 6023672673073491528;
                                            break;
                                        }
                                    }
                                    5 => {
                                        ret = fast_decode_sequence(buffer, pmap, field);
                                        if ret != 0 {
                                            start = buffer_start(buffer);
                                            current_block = 6023672673073491528;
                                            break;
                                        }
                                    }
                                    _ => {
                                        ret = -(1 as libc::c_int);
                                        current_block = 6023672673073491528;
                                        break;
                                    }
                                }
                                (*msg).decoded = ((*msg).decoded).wrapping_add(1);
                                (*msg).decoded;
                            }
                            match current_block {
                                6023672673073491528 => {}
                                _ => {
                                    (*session).last_tid = (*msg).tid;
                                    (*session).rx_message = 0 as *mut fast_message;
                                    (*preamble).is_valid = 0 as libc::c_int != 0;
                                    (*pmap).is_valid = 0 as libc::c_int != 0;
                                    (*msg).decoded = 0 as libc::c_int as libc::c_ulong;
                                    return msg;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if ret == -(2 as libc::c_int) {
        buffer_advance(buffer, start.offset_from(buffer_start(buffer)) as libc::c_long);
        if !msg.is_null() {
            (*pmap).pmap_bit = pmap_bit;
        }
    }
    return 0 as *mut fast_message;
}
#[no_mangle]
pub unsafe extern "C" fn fast_message_new(
    mut nr_messages: libc::c_int,
) -> *mut fast_message {
    let mut self_0: *mut fast_message = calloc(
        nr_messages as libc::c_ulong,
        ::core::mem::size_of::<fast_message>() as libc::c_ulong,
    ) as *mut fast_message;
    if self_0.is_null() {
        return 0 as *mut fast_message;
    }
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn fast_fields_free(mut self_0: *mut fast_message) {
    let mut seq: *mut fast_sequence = 0 as *mut fast_sequence;
    let mut field: *mut fast_field = 0 as *mut fast_field;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if self_0.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*self_0).nr_fields {
        field = ((*self_0).fields).offset(i as isize);
        if (*field).type_0 as libc::c_uint
            == FAST_TYPE_SEQUENCE as libc::c_int as libc::c_uint
        {
            seq = (*field).c2rust_unnamed.ptr_value as *mut fast_sequence;
            j = 0 as libc::c_int;
            while j < 128 as libc::c_int {
                g_hash_table_destroy(
                    (*((*seq).elements).as_mut_ptr().offset(j as isize)).ghtab,
                );
                free(
                    (*((*seq).elements).as_mut_ptr().offset(j as isize)).fields
                        as *mut libc::c_void,
                );
                j += 1;
                j;
            }
            free((*field).c2rust_unnamed.ptr_value);
        } else if (*field).type_0 as libc::c_uint
            == FAST_TYPE_DECIMAL as libc::c_int as libc::c_uint
        {
            free((*field).c2rust_unnamed.decimal_value.fields as *mut libc::c_void);
        }
        i += 1;
        i;
    }
    if !((*self_0).ghtab).is_null() {
        g_hash_table_destroy((*self_0).ghtab);
    }
    free((*self_0).fields as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fast_message_free(
    mut self_0: *mut fast_message,
    mut nr_messages: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if self_0.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < nr_messages {
        fast_fields_free(self_0.offset(i as isize));
        i += 1;
        i;
    }
    free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fast_message_copy(
    mut dst: *mut fast_message,
    mut src: *mut fast_message,
) -> libc::c_int {
    let mut current_block: u64;
    let mut dst_seq: *mut fast_sequence = 0 as *mut fast_sequence;
    let mut src_seq: *mut fast_sequence = 0 as *mut fast_sequence;
    let mut dst_field: *mut fast_field = 0 as *mut fast_field;
    let mut src_field: *mut fast_field = 0 as *mut fast_field;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !dst.is_null() {
        memcpy(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            ::core::mem::size_of::<fast_message>() as libc::c_ulong,
        );
        (*dst)
            .fields = calloc(
            (*src).nr_fields,
            ::core::mem::size_of::<fast_field>() as libc::c_ulong,
        ) as *mut fast_field;
        if !((*dst).fields).is_null() {
            (*dst)
                .ghtab = g_hash_table_new(
                Some(g_str_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                Some(
                    g_str_equal
                        as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
                ),
            );
            if !((*dst).ghtab).is_null() {
                i = 0 as libc::c_int;
                's_36: loop {
                    if !((i as libc::c_ulong) < (*src).nr_fields) {
                        current_block = 4761528863920922185;
                        break;
                    }
                    dst_field = ((*dst).fields).offset(i as isize);
                    src_field = ((*src).fields).offset(i as isize);
                    match (*src_field).type_0 as libc::c_uint {
                        0 | 1 | 2 | 4 | 3 => {
                            memcpy(
                                dst_field as *mut libc::c_void,
                                src_field as *const libc::c_void,
                                ::core::mem::size_of::<fast_field>() as libc::c_ulong,
                            );
                            if strlen(((*dst_field).name).as_mut_ptr()) != 0 {
                                g_hash_table_insert(
                                    (*dst).ghtab,
                                    ((*dst_field).name).as_mut_ptr() as gpointer,
                                    dst_field as gpointer,
                                );
                            }
                        }
                        5 => {
                            memcpy(
                                dst_field as *mut libc::c_void,
                                src_field as *const libc::c_void,
                                ::core::mem::size_of::<fast_field>() as libc::c_ulong,
                            );
                            if strlen(((*dst_field).name).as_mut_ptr()) != 0 {
                                g_hash_table_insert(
                                    (*dst).ghtab,
                                    ((*dst_field).name).as_mut_ptr() as gpointer,
                                    dst_field as gpointer,
                                );
                            }
                            src_seq = (*src_field).c2rust_unnamed.ptr_value
                                as *mut fast_sequence;
                            (*dst_field)
                                .c2rust_unnamed
                                .ptr_value = calloc(
                                1 as libc::c_int as libc::c_ulong,
                                ::core::mem::size_of::<fast_sequence>() as libc::c_ulong,
                            );
                            if ((*dst_field).c2rust_unnamed.ptr_value).is_null() {
                                current_block = 12162064855724997737;
                                break;
                            }
                            dst_seq = (*dst_field).c2rust_unnamed.ptr_value
                                as *mut fast_sequence;
                            memcpy(
                                dst_seq as *mut libc::c_void,
                                src_seq as *const libc::c_void,
                                ::core::mem::size_of::<fast_sequence>() as libc::c_ulong,
                            );
                            j = 0 as libc::c_int;
                            while j < 128 as libc::c_int {
                                if fast_message_copy(
                                    ((*dst_seq).elements).as_mut_ptr().offset(j as isize),
                                    ((*src_seq).elements).as_mut_ptr().offset(j as isize),
                                ) != 0
                                {
                                    while j > 0 as libc::c_int {
                                        j -= 1;
                                        free(
                                            (*dst_seq).elements[j as usize].fields as *mut libc::c_void,
                                        );
                                    }
                                    free((*dst_field).c2rust_unnamed.ptr_value);
                                    current_block = 12162064855724997737;
                                    break 's_36;
                                } else {
                                    j += 1;
                                    j;
                                }
                            }
                        }
                        _ => {
                            current_block = 12162064855724997737;
                            break;
                        }
                    }
                    i += 1;
                    i;
                }
                match current_block {
                    12162064855724997737 => {}
                    _ => return 0 as libc::c_int,
                }
            }
        }
    }
    free((*dst).fields as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fast_message_reset(mut msg: *mut fast_message) {
    let mut seq: *mut fast_sequence = 0 as *mut fast_sequence;
    let mut field: *mut fast_field = 0 as *mut fast_field;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*msg).nr_fields {
        field = ((*msg).fields).offset(i as isize);
        match (*field).type_0 as libc::c_uint {
            0 => {
                if (*field).has_reset {
                    (*field)
                        .c2rust_unnamed
                        .int_value = (*field).c2rust_unnamed_0.int_reset;
                    (*field)
                        .c2rust_unnamed_1
                        .int_previous = (*field).c2rust_unnamed_0.int_reset;
                } else {
                    (*field).c2rust_unnamed.int_value = 0 as libc::c_int as i64_0;
                    (*field).c2rust_unnamed_1.int_previous = 0 as libc::c_int as i64_0;
                }
            }
            1 => {
                if (*field).has_reset {
                    (*field)
                        .c2rust_unnamed
                        .uint_value = (*field).c2rust_unnamed_0.uint_reset;
                    (*field)
                        .c2rust_unnamed_1
                        .uint_previous = (*field).c2rust_unnamed_0.uint_reset;
                } else {
                    (*field).c2rust_unnamed.uint_value = 0 as libc::c_int as u64_0;
                    (*field).c2rust_unnamed_1.uint_previous = 0 as libc::c_int as u64_0;
                }
            }
            2 => {
                if (*field).has_reset {
                    strcpy(
                        ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                        ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr(),
                    );
                    strcpy(
                        ((*field).c2rust_unnamed_1.string_previous).as_mut_ptr(),
                        ((*field).c2rust_unnamed_0.string_reset).as_mut_ptr(),
                    );
                } else {
                    (*field)
                        .c2rust_unnamed
                        .string_value[0 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_char;
                    (*field)
                        .c2rust_unnamed_1
                        .string_previous[0 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_char;
                }
            }
            3 => {
                if (*field).has_reset {
                    memcpy(
                        ((*field).c2rust_unnamed.vector_value).as_mut_ptr()
                            as *mut libc::c_void,
                        ((*field).c2rust_unnamed_0.vector_reset).as_mut_ptr()
                            as *const libc::c_void,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                    memcpy(
                        ((*field).c2rust_unnamed_1.vector_previous).as_mut_ptr()
                            as *mut libc::c_void,
                        ((*field).c2rust_unnamed_0.vector_reset).as_mut_ptr()
                            as *const libc::c_void,
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    );
                } else {
                    memset(
                        ((*field).c2rust_unnamed.vector_value).as_mut_ptr()
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<u64_0>() as libc::c_ulong,
                    );
                    memset(
                        ((*field).c2rust_unnamed_1.vector_previous).as_mut_ptr()
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<u64_0>() as libc::c_ulong,
                    );
                }
            }
            4 => {
                if (*field).has_reset {
                    (*field)
                        .c2rust_unnamed
                        .decimal_value
                        .exp = (*field).c2rust_unnamed_0.decimal_reset.exp;
                    (*field)
                        .c2rust_unnamed
                        .decimal_value
                        .mnt = (*field).c2rust_unnamed_0.decimal_reset.mnt;
                    (*field)
                        .c2rust_unnamed_1
                        .decimal_previous
                        .exp = (*field).c2rust_unnamed_0.decimal_reset.exp;
                    (*field)
                        .c2rust_unnamed_1
                        .decimal_previous
                        .mnt = (*field).c2rust_unnamed_0.decimal_reset.mnt;
                } else {
                    (*field)
                        .c2rust_unnamed
                        .decimal_value
                        .exp = 0 as libc::c_int as i64_0;
                    (*field)
                        .c2rust_unnamed
                        .decimal_value
                        .mnt = 0 as libc::c_int as i64_0;
                    (*field)
                        .c2rust_unnamed_1
                        .decimal_previous
                        .exp = 0 as libc::c_int as i64_0;
                    (*field)
                        .c2rust_unnamed_1
                        .decimal_previous
                        .mnt = 0 as libc::c_int as i64_0;
                }
            }
            5 => {
                seq = (*field).c2rust_unnamed.ptr_value as *mut fast_sequence;
                fast_message_reset(((*seq).elements).as_mut_ptr());
            }
            _ => {}
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn transfer_int(
    mut buffer: *mut buffer,
    mut tmp: i64_0,
) -> libc::c_int {
    let mut size: libc::c_int = transfer_size_int(tmp);
    if buffer_remaining(buffer) < size as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    let mut current_block_11: u64;
    match size {
        9 => {
            buffer_put(
                buffer,
                (tmp >> 56 as libc::c_int & 0x7f as libc::c_int as libc::c_long)
                    as libc::c_char,
            );
            current_block_11 = 11406935920385684369;
        }
        8 => {
            current_block_11 = 11406935920385684369;
        }
        7 => {
            current_block_11 = 9757539273797450110;
        }
        6 => {
            current_block_11 = 14573465794555766312;
        }
        5 => {
            current_block_11 = 5757338610095050103;
        }
        4 => {
            current_block_11 = 10183941632857222787;
        }
        3 => {
            current_block_11 = 13963057594542240560;
        }
        2 => {
            current_block_11 = 12685651785920772735;
        }
        1 => {
            current_block_11 = 4932308718341151874;
        }
        _ => return -(1 as libc::c_int),
    }
    match current_block_11 {
        11406935920385684369 => {
            buffer_put(
                buffer,
                (tmp >> 49 as libc::c_int & 0x7f as libc::c_int as libc::c_long)
                    as libc::c_char,
            );
            current_block_11 = 9757539273797450110;
        }
        _ => {}
    }
    match current_block_11 {
        9757539273797450110 => {
            buffer_put(
                buffer,
                (tmp >> 42 as libc::c_int & 0x7f as libc::c_int as libc::c_long)
                    as libc::c_char,
            );
            current_block_11 = 14573465794555766312;
        }
        _ => {}
    }
    match current_block_11 {
        14573465794555766312 => {
            buffer_put(
                buffer,
                (tmp >> 35 as libc::c_int & 0x7f as libc::c_int as libc::c_long)
                    as libc::c_char,
            );
            current_block_11 = 5757338610095050103;
        }
        _ => {}
    }
    match current_block_11 {
        5757338610095050103 => {
            buffer_put(
                buffer,
                (tmp >> 28 as libc::c_int & 0x7f as libc::c_int as libc::c_long)
                    as libc::c_char,
            );
            current_block_11 = 10183941632857222787;
        }
        _ => {}
    }
    match current_block_11 {
        10183941632857222787 => {
            buffer_put(
                buffer,
                (tmp >> 21 as libc::c_int & 0x7f as libc::c_int as libc::c_long)
                    as libc::c_char,
            );
            current_block_11 = 13963057594542240560;
        }
        _ => {}
    }
    match current_block_11 {
        13963057594542240560 => {
            buffer_put(
                buffer,
                (tmp >> 14 as libc::c_int & 0x7f as libc::c_int as libc::c_long)
                    as libc::c_char,
            );
            current_block_11 = 12685651785920772735;
        }
        _ => {}
    }
    match current_block_11 {
        12685651785920772735 => {
            buffer_put(
                buffer,
                (tmp >> 7 as libc::c_int & 0x7f as libc::c_int as libc::c_long)
                    as libc::c_char,
            );
        }
        _ => {}
    }
    buffer_put(
        buffer,
        (tmp & 0x7f as libc::c_int as libc::c_long | 0x80 as libc::c_int as libc::c_long)
            as libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn fast_encode_int(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tmp: i64_0 = (*field).c2rust_unnamed.int_value;
    let mut pset: bool = 1 as libc::c_int != 0;
    match (*field).op as libc::c_uint {
        0 => {
            pset = 0 as libc::c_int != 0;
            if !field_is_mandatory(field) {
                tmp = if tmp >= 0 as libc::c_int as libc::c_long {
                    tmp + 1 as libc::c_int as libc::c_long
                } else {
                    tmp
                };
                if field_state_empty(field) {
                    current_block = 4534053114469571413;
                } else {
                    current_block = 7095457783677275021;
                }
            } else {
                current_block = 7095457783677275021;
            }
            match current_block {
                4534053114469571413 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 1331673515479353454;
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                tmp = if tmp >= 0 as libc::c_int as libc::c_long {
                    tmp + 1 as libc::c_int as libc::c_long
                } else {
                    tmp
                };
                if field_state_empty(field) {
                    current_block = 4534053114469571413;
                } else {
                    current_block = 1917311967535052937;
                }
            } else {
                current_block = 1917311967535052937;
            }
            match current_block {
                4534053114469571413 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 => {
                            current_block = 13744750694277150434;
                            match current_block {
                                13744750694277150434 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 1331673515479353454;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 1331673515479353454;
                                    } else if (*field).c2rust_unnamed.int_value
                                        != (*field).c2rust_unnamed_1.int_previous
                                    {
                                        current_block = 1331673515479353454;
                                    } else {
                                        current_block = 13460095289871124136;
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 13585583586825826409;
                            match current_block {
                                13744750694277150434 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 1331673515479353454;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 1331673515479353454;
                                    } else if (*field).c2rust_unnamed.int_value
                                        != (*field).c2rust_unnamed_1.int_previous
                                    {
                                        current_block = 1331673515479353454;
                                    } else {
                                        current_block = 13460095289871124136;
                                    }
                                }
                            }
                        }
                        2 | _ => {
                            current_block = 3997098394886633522;
                        }
                    }
                }
            }
        }
        2 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                tmp = if tmp >= 0 as libc::c_int as libc::c_long {
                    tmp + 1 as libc::c_int as libc::c_long
                } else {
                    tmp
                };
                if field_state_empty(field) {
                    current_block = 4534053114469571413;
                } else {
                    current_block = 224731115979188411;
                }
            } else {
                current_block = 224731115979188411;
            }
            match current_block {
                4534053114469571413 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 => {
                            current_block = 10294396706215297295;
                            match current_block {
                                10294396706215297295 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 1331673515479353454;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 1331673515479353454;
                                    } else if (*field).c2rust_unnamed.int_value
                                        != (*field).c2rust_unnamed_1.int_previous
                                            + 1 as libc::c_int as libc::c_long
                                    {
                                        current_block = 1331673515479353454;
                                    } else {
                                        (*field).c2rust_unnamed_1.int_previous += 1;
                                        (*field).c2rust_unnamed_1.int_previous;
                                        current_block = 13460095289871124136;
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 5264912573233387743;
                            match current_block {
                                10294396706215297295 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 1331673515479353454;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 1331673515479353454;
                                    } else if (*field).c2rust_unnamed.int_value
                                        != (*field).c2rust_unnamed_1.int_previous
                                            + 1 as libc::c_int as libc::c_long
                                    {
                                        current_block = 1331673515479353454;
                                    } else {
                                        (*field).c2rust_unnamed_1.int_previous += 1;
                                        (*field).c2rust_unnamed_1.int_previous;
                                        current_block = 13460095289871124136;
                                    }
                                }
                            }
                        }
                        2 | _ => {
                            current_block = 3997098394886633522;
                        }
                    }
                }
            }
        }
        3 => {
            tmp = (*field).c2rust_unnamed.int_value
                - (*field).c2rust_unnamed_1.int_previous;
            pset = 0 as libc::c_int != 0;
            if !field_is_mandatory(field) {
                tmp = if tmp >= 0 as libc::c_int as libc::c_long {
                    tmp + 1 as libc::c_int as libc::c_long
                } else {
                    tmp
                };
                if field_state_empty(field) {
                    current_block = 4534053114469571413;
                } else {
                    current_block = 11913429853522160501;
                }
            } else {
                current_block = 11913429853522160501;
            }
            match current_block {
                4534053114469571413 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 1331673515479353454;
                }
            }
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                tmp = if tmp >= 0 as libc::c_int as libc::c_long {
                    tmp + 1 as libc::c_int as libc::c_long
                } else {
                    tmp
                };
                if field_state_empty(field) {
                    current_block = 4534053114469571413;
                } else {
                    current_block = 4090602189656566074;
                }
            } else {
                current_block = 4090602189656566074;
            }
            match current_block {
                4534053114469571413 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 | 1 => {
                            (*field).state = FAST_STATE_ASSIGNED;
                            current_block = 1331673515479353454;
                        }
                        2 | _ => {
                            current_block = 3997098394886633522;
                        }
                    }
                }
            }
        }
        5 => {
            if !field_is_mandatory(field) {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !field_state_empty(field) {
                    pmap_set(pmap, (*pmap).pmap_bit as libc::c_ulong);
                    current_block = 1345366029464561491;
                } else {
                    current_block = 13460095289871124136;
                }
            } else {
                current_block = 1345366029464561491;
            }
            match current_block {
                13460095289871124136 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 13460095289871124136;
                }
            }
        }
        _ => {
            current_block = 3997098394886633522;
        }
    }
    match current_block {
        4534053114469571413 => {
            tmp = 0 as libc::c_int as i64_0;
            (*field).c2rust_unnamed.int_value = (*field).c2rust_unnamed_1.int_previous;
            current_block = 1331673515479353454;
        }
        13460095289871124136 => return 0 as libc::c_int,
        _ => {}
    }
    match current_block {
        1331673515479353454 => {
            (*field).c2rust_unnamed_1.int_previous = (*field).c2rust_unnamed.int_value;
            (*field).state_previous = (*field).state;
            if !(transfer_int(buffer, tmp) != 0) {
                if pset {
                    pmap_set(pmap, (*pmap).pmap_bit as libc::c_ulong);
                }
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn transfer_uint(
    mut buffer: *mut buffer,
    mut tmp: u64_0,
) -> libc::c_int {
    let mut size: libc::c_int = transfer_size_uint(tmp);
    if buffer_remaining(buffer) < size as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    let mut current_block_11: u64;
    match size {
        9 => {
            buffer_put(
                buffer,
                (tmp >> 56 as libc::c_int & 0x7f as libc::c_int as libc::c_ulong)
                    as libc::c_char,
            );
            current_block_11 = 15430037752557703696;
        }
        8 => {
            current_block_11 = 15430037752557703696;
        }
        7 => {
            current_block_11 = 3322080646735221946;
        }
        6 => {
            current_block_11 = 357725852828348823;
        }
        5 => {
            current_block_11 = 4605574453288239744;
        }
        4 => {
            current_block_11 = 13473140350658731071;
        }
        3 => {
            current_block_11 = 7021423963501825306;
        }
        2 => {
            current_block_11 = 4849209021823711227;
        }
        1 => {
            current_block_11 = 2047138341103699312;
        }
        _ => return -(1 as libc::c_int),
    }
    match current_block_11 {
        15430037752557703696 => {
            buffer_put(
                buffer,
                (tmp >> 49 as libc::c_int & 0x7f as libc::c_int as libc::c_ulong)
                    as libc::c_char,
            );
            current_block_11 = 3322080646735221946;
        }
        _ => {}
    }
    match current_block_11 {
        3322080646735221946 => {
            buffer_put(
                buffer,
                (tmp >> 42 as libc::c_int & 0x7f as libc::c_int as libc::c_ulong)
                    as libc::c_char,
            );
            current_block_11 = 357725852828348823;
        }
        _ => {}
    }
    match current_block_11 {
        357725852828348823 => {
            buffer_put(
                buffer,
                (tmp >> 35 as libc::c_int & 0x7f as libc::c_int as libc::c_ulong)
                    as libc::c_char,
            );
            current_block_11 = 4605574453288239744;
        }
        _ => {}
    }
    match current_block_11 {
        4605574453288239744 => {
            buffer_put(
                buffer,
                (tmp >> 28 as libc::c_int & 0x7f as libc::c_int as libc::c_ulong)
                    as libc::c_char,
            );
            current_block_11 = 13473140350658731071;
        }
        _ => {}
    }
    match current_block_11 {
        13473140350658731071 => {
            buffer_put(
                buffer,
                (tmp >> 21 as libc::c_int & 0x7f as libc::c_int as libc::c_ulong)
                    as libc::c_char,
            );
            current_block_11 = 7021423963501825306;
        }
        _ => {}
    }
    match current_block_11 {
        7021423963501825306 => {
            buffer_put(
                buffer,
                (tmp >> 14 as libc::c_int & 0x7f as libc::c_int as libc::c_ulong)
                    as libc::c_char,
            );
            current_block_11 = 4849209021823711227;
        }
        _ => {}
    }
    match current_block_11 {
        4849209021823711227 => {
            buffer_put(
                buffer,
                (tmp >> 7 as libc::c_int & 0x7f as libc::c_int as libc::c_ulong)
                    as libc::c_char,
            );
        }
        _ => {}
    }
    buffer_put(
        buffer,
        (tmp & 0x7f as libc::c_int as libc::c_ulong
            | 0x80 as libc::c_int as libc::c_ulong) as libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn fast_encode_uint(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tmp: u64_0 = (*field).c2rust_unnamed.uint_value;
    let mut pset: bool = 1 as libc::c_int != 0;
    let mut delta: i64_0 = 0 as libc::c_int as i64_0;
    match (*field).op as libc::c_uint {
        0 => {
            pset = 0 as libc::c_int != 0;
            if !field_is_mandatory(field) {
                tmp = (tmp as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as u64_0 as u64_0;
                if field_state_empty(field) {
                    current_block = 3550994845552149839;
                } else {
                    current_block = 15427931788582360902;
                }
            } else {
                current_block = 15427931788582360902;
            }
            match current_block {
                3550994845552149839 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 637229377017864886;
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                tmp = (tmp as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as u64_0 as u64_0;
                if field_state_empty(field) {
                    current_block = 3550994845552149839;
                } else {
                    current_block = 2868539653012386629;
                }
            } else {
                current_block = 2868539653012386629;
            }
            match current_block {
                3550994845552149839 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 => {
                            current_block = 4644547718933586525;
                            match current_block {
                                4644547718933586525 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 637229377017864886;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 637229377017864886;
                                    } else if (*field).c2rust_unnamed.uint_value
                                        != (*field).c2rust_unnamed_1.uint_previous
                                    {
                                        current_block = 637229377017864886;
                                    } else {
                                        current_block = 1423531122933789233;
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 15155453326694199230;
                            match current_block {
                                4644547718933586525 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 637229377017864886;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 637229377017864886;
                                    } else if (*field).c2rust_unnamed.uint_value
                                        != (*field).c2rust_unnamed_1.uint_previous
                                    {
                                        current_block = 637229377017864886;
                                    } else {
                                        current_block = 1423531122933789233;
                                    }
                                }
                            }
                        }
                        2 | _ => {
                            current_block = 1984051432127314216;
                        }
                    }
                }
            }
        }
        2 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                tmp = (tmp as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as u64_0 as u64_0;
                if field_state_empty(field) {
                    current_block = 3550994845552149839;
                } else {
                    current_block = 4495394744059808450;
                }
            } else {
                current_block = 4495394744059808450;
            }
            match current_block {
                3550994845552149839 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 => {
                            current_block = 9500422015507347587;
                            match current_block {
                                9500422015507347587 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 637229377017864886;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 637229377017864886;
                                    } else if (*field).c2rust_unnamed.uint_value
                                        != ((*field).c2rust_unnamed_1.uint_previous)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    {
                                        current_block = 637229377017864886;
                                    } else {
                                        (*field)
                                            .c2rust_unnamed_1
                                            .uint_previous = ((*field).c2rust_unnamed_1.uint_previous)
                                            .wrapping_add(1);
                                        (*field).c2rust_unnamed_1.uint_previous;
                                        current_block = 1423531122933789233;
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 8762614210985740066;
                            match current_block {
                                9500422015507347587 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 637229377017864886;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 637229377017864886;
                                    } else if (*field).c2rust_unnamed.uint_value
                                        != ((*field).c2rust_unnamed_1.uint_previous)
                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    {
                                        current_block = 637229377017864886;
                                    } else {
                                        (*field)
                                            .c2rust_unnamed_1
                                            .uint_previous = ((*field).c2rust_unnamed_1.uint_previous)
                                            .wrapping_add(1);
                                        (*field).c2rust_unnamed_1.uint_previous;
                                        current_block = 1423531122933789233;
                                    }
                                }
                            }
                        }
                        2 | _ => {
                            current_block = 1984051432127314216;
                        }
                    }
                }
            }
        }
        3 => {
            delta = ((*field).c2rust_unnamed.uint_value)
                .wrapping_sub((*field).c2rust_unnamed_1.uint_previous) as i64_0;
            pset = 0 as libc::c_int != 0;
            if !field_is_mandatory(field) {
                delta = if delta >= 0 as libc::c_int as libc::c_long {
                    delta + 1 as libc::c_int as libc::c_long
                } else {
                    delta
                };
                if field_state_empty(field) {
                    current_block = 3550994845552149839;
                } else {
                    current_block = 8704759739624374314;
                }
            } else {
                current_block = 8704759739624374314;
            }
            match current_block {
                3550994845552149839 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    tmp = delta as u64_0;
                    current_block = 637229377017864886;
                }
            }
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                tmp = (tmp as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as u64_0 as u64_0;
                if field_state_empty(field) {
                    current_block = 3550994845552149839;
                } else {
                    current_block = 14072441030219150333;
                }
            } else {
                current_block = 14072441030219150333;
            }
            match current_block {
                3550994845552149839 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 | 1 => {
                            (*field).state = FAST_STATE_ASSIGNED;
                            current_block = 637229377017864886;
                        }
                        2 | _ => {
                            current_block = 1984051432127314216;
                        }
                    }
                }
            }
        }
        5 => {
            if !field_is_mandatory(field) {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !field_state_empty(field) {
                    pmap_set(pmap, (*pmap).pmap_bit as libc::c_ulong);
                    current_block = 10150597327160359210;
                } else {
                    current_block = 1423531122933789233;
                }
            } else {
                current_block = 10150597327160359210;
            }
            match current_block {
                1423531122933789233 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 1423531122933789233;
                }
            }
        }
        _ => {
            current_block = 1984051432127314216;
        }
    }
    match current_block {
        3550994845552149839 => {
            tmp = 0 as libc::c_int as u64_0;
            (*field).c2rust_unnamed.uint_value = (*field).c2rust_unnamed_1.uint_previous;
            current_block = 637229377017864886;
        }
        1423531122933789233 => return 0 as libc::c_int,
        _ => {}
    }
    match current_block {
        637229377017864886 => {
            (*field).c2rust_unnamed_1.uint_previous = (*field).c2rust_unnamed.uint_value;
            (*field).state_previous = (*field).state;
            if (*field).op as libc::c_uint
                != FAST_OP_DELTA as libc::c_int as libc::c_uint
            {
                if transfer_uint(buffer, tmp) != 0 {
                    current_block = 1984051432127314216;
                } else {
                    current_block = 9241535491006583629;
                }
            } else if transfer_int(buffer, tmp as i64_0) != 0 {
                current_block = 1984051432127314216;
            } else {
                current_block = 9241535491006583629;
            }
            match current_block {
                1984051432127314216 => {}
                _ => {
                    if pset {
                        pmap_set(pmap, (*pmap).pmap_bit as libc::c_ulong);
                    }
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn transfer_string(
    mut buffer: *mut buffer,
    mut tmp: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if tmp.is_null() {
        current_block = 3250561932312552084;
    } else {
        size = strlen(tmp) as libc::c_int;
        if size == 0 {
            size = 1 as libc::c_int;
        }
        if buffer_remaining(buffer) < size as libc::c_ulong {
            current_block = 10817849844502915509;
        } else {
            i = 0 as libc::c_int;
            while i < size {
                buffer_put(buffer, *tmp.offset(i as isize));
                i += 1;
                i;
            }
            current_block = 3250561932312552084;
        }
    }
    match current_block {
        3250561932312552084 => {
            if !(buffer_remaining(buffer) < 1 as libc::c_int as libc::c_ulong) {
                buffer_put(buffer, 0x80 as libc::c_int as libc::c_char);
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn fast_encode_string(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tmp: *mut libc::c_char = ((*field).c2rust_unnamed.string_value).as_mut_ptr();
    let mut pset: bool = 1 as libc::c_int != 0;
    match (*field).op as libc::c_uint {
        0 => {
            pset = 0 as libc::c_int != 0;
            if !field_is_mandatory(field) {
                if field_state_empty(field) {
                    current_block = 4275452180526721189;
                } else {
                    current_block = 735147466149431745;
                }
            } else {
                current_block = 735147466149431745;
            }
            match current_block {
                4275452180526721189 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 2756452209188106582;
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                if field_state_empty(field) {
                    current_block = 4275452180526721189;
                } else {
                    current_block = 5720623009719927633;
                }
            } else {
                current_block = 5720623009719927633;
            }
            match current_block {
                4275452180526721189 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 => {
                            current_block = 4669592256920103309;
                            match current_block {
                                4669592256920103309 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 2756452209188106582;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 2756452209188106582;
                                    } else if strcmp(
                                        ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                                        ((*field).c2rust_unnamed_1.string_previous).as_mut_ptr(),
                                    ) != 0
                                    {
                                        current_block = 2756452209188106582;
                                    } else {
                                        current_block = 15897653523371991391;
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 443900374527332424;
                            match current_block {
                                4669592256920103309 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 2756452209188106582;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 2756452209188106582;
                                    } else if strcmp(
                                        ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                                        ((*field).c2rust_unnamed_1.string_previous).as_mut_ptr(),
                                    ) != 0
                                    {
                                        current_block = 2756452209188106582;
                                    } else {
                                        current_block = 15897653523371991391;
                                    }
                                }
                            }
                        }
                        2 | _ => {
                            current_block = 14752179045446968347;
                        }
                    }
                }
            }
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                if field_state_empty(field) {
                    current_block = 4275452180526721189;
                } else {
                    current_block = 26972500619410423;
                }
            } else {
                current_block = 26972500619410423;
            }
            match current_block {
                4275452180526721189 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 | 1 => {
                            (*field).state = FAST_STATE_ASSIGNED;
                            current_block = 2756452209188106582;
                        }
                        2 | _ => {
                            current_block = 14752179045446968347;
                        }
                    }
                }
            }
        }
        5 => {
            if !field_is_mandatory(field) {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !field_state_empty(field) {
                    pmap_set(pmap, (*pmap).pmap_bit as libc::c_ulong);
                    current_block = 16924917904204750491;
                } else {
                    current_block = 15897653523371991391;
                }
            } else {
                current_block = 16924917904204750491;
            }
            match current_block {
                15897653523371991391 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 15897653523371991391;
                }
            }
        }
        2 | 3 | _ => {
            current_block = 14752179045446968347;
        }
    }
    match current_block {
        4275452180526721189 => {
            tmp = 0 as *mut libc::c_char;
            memcpy(
                ((*field).c2rust_unnamed.string_value).as_mut_ptr() as *mut libc::c_void,
                ((*field).c2rust_unnamed_1.string_previous).as_mut_ptr()
                    as *const libc::c_void,
                (strlen(((*field).c2rust_unnamed_1.string_previous).as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            current_block = 2756452209188106582;
        }
        15897653523371991391 => return 0 as libc::c_int,
        _ => {}
    }
    match current_block {
        2756452209188106582 => {
            memcpy(
                ((*field).c2rust_unnamed_1.string_previous).as_mut_ptr()
                    as *mut libc::c_void,
                ((*field).c2rust_unnamed.string_value).as_mut_ptr()
                    as *const libc::c_void,
                (strlen(((*field).c2rust_unnamed.string_value).as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            (*field).state_previous = (*field).state;
            if !(transfer_string(buffer, tmp) != 0) {
                if pset {
                    pmap_set(pmap, (*pmap).pmap_bit as libc::c_ulong);
                }
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn fast_encode_decimal_atomic(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut current_block: u64;
    let mut exp: i64_0 = (*field).c2rust_unnamed.decimal_value.exp;
    let mut mnt: i64_0 = (*field).c2rust_unnamed.decimal_value.mnt;
    let mut pset: bool = 1 as libc::c_int != 0;
    let mut empty: bool = 0 as libc::c_int != 0;
    match (*field).op as libc::c_uint {
        0 => {
            pset = 0 as libc::c_int != 0;
            if !field_is_mandatory(field) {
                exp = if exp >= 0 as libc::c_int as libc::c_long {
                    exp + 1 as libc::c_int as libc::c_long
                } else {
                    exp
                };
                if field_state_empty(field) {
                    current_block = 2520615190005585665;
                } else {
                    current_block = 11875828834189669668;
                }
            } else {
                current_block = 11875828834189669668;
            }
            match current_block {
                2520615190005585665 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 6740919169260623103;
                }
            }
        }
        1 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                exp = if exp >= 0 as libc::c_int as libc::c_long {
                    exp + 1 as libc::c_int as libc::c_long
                } else {
                    exp
                };
                if field_state_empty(field) {
                    current_block = 2520615190005585665;
                } else {
                    current_block = 13536709405535804910;
                }
            } else {
                current_block = 13536709405535804910;
            }
            match current_block {
                2520615190005585665 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 => {
                            current_block = 242942688411373325;
                            match current_block {
                                242942688411373325 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 6740919169260623103;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 6740919169260623103;
                                    } else if (*field).c2rust_unnamed.decimal_value.exp
                                        != (*field).c2rust_unnamed_1.decimal_previous.exp
                                        || (*field).c2rust_unnamed.decimal_value.mnt
                                            != (*field).c2rust_unnamed_1.decimal_previous.mnt
                                    {
                                        current_block = 6740919169260623103;
                                    } else {
                                        current_block = 11743904203796629665;
                                    }
                                }
                            }
                        }
                        1 => {
                            current_block = 2025808612723279696;
                            match current_block {
                                242942688411373325 => {
                                    (*field).state = FAST_STATE_ASSIGNED;
                                    current_block = 6740919169260623103;
                                }
                                _ => {
                                    if field_state_empty_previous(field) {
                                        current_block = 6740919169260623103;
                                    } else if (*field).c2rust_unnamed.decimal_value.exp
                                        != (*field).c2rust_unnamed_1.decimal_previous.exp
                                        || (*field).c2rust_unnamed.decimal_value.mnt
                                            != (*field).c2rust_unnamed_1.decimal_previous.mnt
                                    {
                                        current_block = 6740919169260623103;
                                    } else {
                                        current_block = 11743904203796629665;
                                    }
                                }
                            }
                        }
                        2 | _ => {
                            current_block = 3742237686972679509;
                        }
                    }
                }
            }
        }
        2 => {
            current_block = 11743904203796629665;
        }
        3 => {
            exp = (*field).c2rust_unnamed.decimal_value.exp
                - (*field).c2rust_unnamed_1.decimal_previous.exp;
            mnt = (*field).c2rust_unnamed.decimal_value.mnt
                - (*field).c2rust_unnamed_1.decimal_previous.mnt;
            pset = 0 as libc::c_int != 0;
            if !field_is_mandatory(field) {
                exp = if exp >= 0 as libc::c_int as libc::c_long {
                    exp + 1 as libc::c_int as libc::c_long
                } else {
                    exp
                };
                if field_state_empty(field) {
                    current_block = 2520615190005585665;
                } else {
                    current_block = 11298138898191919651;
                }
            } else {
                current_block = 11298138898191919651;
            }
            match current_block {
                2520615190005585665 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 6740919169260623103;
                }
            }
        }
        4 => {
            (*pmap).pmap_bit += 1;
            (*pmap).pmap_bit;
            if !field_is_mandatory(field) {
                exp = if exp >= 0 as libc::c_int as libc::c_long {
                    exp + 1 as libc::c_int as libc::c_long
                } else {
                    exp
                };
                if field_state_empty(field) {
                    current_block = 2520615190005585665;
                } else {
                    current_block = 1608152415753874203;
                }
            } else {
                current_block = 1608152415753874203;
            }
            match current_block {
                2520615190005585665 => {}
                _ => {
                    match (*field).state as libc::c_uint {
                        0 | 1 => {
                            (*field).state = FAST_STATE_ASSIGNED;
                            current_block = 6740919169260623103;
                        }
                        2 | _ => {
                            current_block = 3742237686972679509;
                        }
                    }
                }
            }
        }
        5 => {
            if !field_is_mandatory(field) {
                (*pmap).pmap_bit += 1;
                (*pmap).pmap_bit;
                if !field_state_empty(field) {
                    pmap_set(pmap, (*pmap).pmap_bit as libc::c_ulong);
                    current_block = 6417057564578538666;
                } else {
                    current_block = 11743904203796629665;
                }
            } else {
                current_block = 6417057564578538666;
            }
            match current_block {
                11743904203796629665 => {}
                _ => {
                    (*field).state = FAST_STATE_ASSIGNED;
                    current_block = 11743904203796629665;
                }
            }
        }
        _ => {
            current_block = 3742237686972679509;
        }
    }
    match current_block {
        2520615190005585665 => {
            exp = 0 as libc::c_int as i64_0;
            empty = 1 as libc::c_int != 0;
            current_block = 6740919169260623103;
        }
        11743904203796629665 => return 0 as libc::c_int,
        _ => {}
    }
    match current_block {
        6740919169260623103 => {
            if !empty {
                (*field)
                    .c2rust_unnamed_1
                    .decimal_previous
                    .exp = (*field).c2rust_unnamed.decimal_value.exp;
                (*field)
                    .c2rust_unnamed_1
                    .decimal_previous
                    .mnt = (*field).c2rust_unnamed.decimal_value.mnt;
            }
            (*field).state_previous = (*field).state;
            if !(transfer_int(buffer, exp) != 0) {
                if !(!empty && transfer_int(buffer, mnt) != 0) {
                    if pset {
                        pmap_set(pmap, (*pmap).pmap_bit as libc::c_ulong);
                    }
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn fast_encode_decimal_individ(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut decimal: *mut fast_decimal = 0 as *mut fast_decimal;
    let mut ret: libc::c_int = 0;
    decimal = &mut (*field).c2rust_unnamed.decimal_value;
    if !field_state_empty(field) {
        (*((*decimal).fields).offset(0 as libc::c_int as isize))
            .c2rust_unnamed
            .int_value = (*decimal).exp;
        (*((*decimal).fields).offset(1 as libc::c_int as isize))
            .c2rust_unnamed
            .int_value = (*decimal).mnt;
    }
    (*((*decimal).fields).offset(0 as libc::c_int as isize)).state = (*field).state;
    (*((*decimal).fields).offset(1 as libc::c_int as isize)).state = (*field).state;
    ret = fast_encode_int(
        buffer,
        pmap,
        &mut *((*decimal).fields).offset(0 as libc::c_int as isize),
    );
    if !(ret != 0) {
        if !field_state_empty(
            &mut *((*decimal).fields).offset(0 as libc::c_int as isize),
        ) {
            (*field).state = FAST_STATE_ASSIGNED;
            ret = fast_encode_int(
                buffer,
                pmap,
                &mut *((*decimal).fields).offset(1 as libc::c_int as isize),
            );
            ret != 0;
        }
    }
    return ret;
}
unsafe extern "C" fn fast_encode_decimal(
    mut buffer: *mut buffer,
    mut pmap: *mut fast_pmap,
    mut field: *mut fast_field,
) -> libc::c_int {
    if field_has_flags(field, 0x4 as libc::c_int) == 0 {
        return fast_encode_decimal_atomic(buffer, pmap, field)
    } else {
        return fast_encode_decimal_individ(buffer, pmap, field)
    };
}
#[no_mangle]
pub unsafe extern "C" fn fast_message_encode(mut msg: *mut fast_message) -> libc::c_int {
    let mut current_block: u64;
    let mut field: *mut fast_field = 0 as *mut fast_field;
    let mut pmap: fast_pmap = fast_pmap {
        is_valid: false,
        pmap_bit: 0,
        nr_bytes: 0,
        bytes: [0; 8],
    };
    let mut i: libc::c_int = 0;
    pmap.nr_bytes = 8 as libc::c_int as libc::c_ulong;
    memset(
        (pmap.bytes).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        pmap.nr_bytes,
    );
    pmap_set(&mut pmap, 0 as libc::c_int as libc::c_ulong);
    pmap.pmap_bit = 0 as libc::c_int as libc::c_long;
    if !(transfer_uint((*msg).msg_buf, (*msg).tid) != 0) {
        i = 0 as libc::c_int;
        loop {
            if !((i as libc::c_ulong) < (*msg).nr_fields) {
                current_block = 5689001924483802034;
                break;
            }
            field = ((*msg).fields).offset(i as isize);
            match (*field).type_0 as libc::c_uint {
                0 => {
                    if fast_encode_int((*msg).msg_buf, &mut pmap, field) != 0 {
                        current_block = 17468973740251664835;
                        break;
                    }
                }
                1 => {
                    if fast_encode_uint((*msg).msg_buf, &mut pmap, field) != 0 {
                        current_block = 17468973740251664835;
                        break;
                    }
                }
                2 => {
                    if fast_encode_string((*msg).msg_buf, &mut pmap, field) != 0 {
                        current_block = 17468973740251664835;
                        break;
                    }
                }
                4 => {
                    if fast_encode_decimal((*msg).msg_buf, &mut pmap, field) != 0 {
                        current_block = 17468973740251664835;
                        break;
                    }
                }
                3 | 5 | _ => {
                    current_block = 17468973740251664835;
                    break;
                }
            }
            i += 1;
            i;
        }
        match current_block {
            17468973740251664835 => {}
            _ => {
                i = 8 as libc::c_int;
                while i > 0 as libc::c_int {
                    if pmap.bytes[(i - 1 as libc::c_int) as usize] != 0 {
                        break;
                    }
                    pmap.nr_bytes = (pmap.nr_bytes).wrapping_sub(1);
                    pmap.nr_bytes;
                    i -= 1;
                    i;
                }
                if !(buffer_remaining((*msg).pmap_buf) < pmap.nr_bytes) {
                    i = 0 as libc::c_int;
                    while (i as libc::c_ulong)
                        < (pmap.nr_bytes).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        buffer_put(
                            (*msg).pmap_buf,
                            (pmap.bytes[i as usize] as libc::c_int & 0x7f as libc::c_int)
                                as libc::c_char,
                        );
                        i += 1;
                        i;
                    }
                    buffer_put(
                        (*msg).pmap_buf,
                        (pmap
                            .bytes[(pmap.nr_bytes)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize]
                            as libc::c_int | 0x80 as libc::c_int) as libc::c_char,
                    );
                    return 0 as libc::c_int;
                }
            }
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fast_message_send(
    mut self_0: *mut fast_message,
    mut session: *mut fast_session,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut iov: [iovec; 2] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 2];
    let mut msg: msghdr = msghdr {
        msg_name: 0 as *mut libc::c_void,
        msg_namelen: 0,
        msg_iov: 0 as *mut iovec,
        msg_iovlen: 0,
        msg_control: 0 as *mut libc::c_void,
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut ret: libc::c_int = 0 as libc::c_int;
    ret = fast_message_encode(self_0);
    if !(ret != 0) {
        buffer_to_iovec(
            (*self_0).pmap_buf,
            &mut *iov.as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        buffer_to_iovec(
            (*self_0).msg_buf,
            &mut *iov.as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        msg = {
            let mut init = msghdr {
                msg_name: 0 as *mut libc::c_void,
                msg_namelen: 0,
                msg_iov: iov.as_mut_ptr(),
                msg_iovlen: 2 as libc::c_int as size_t,
                msg_control: 0 as *mut libc::c_void,
                msg_controllen: 0,
                msg_flags: 0,
            };
            init
        };
        if ((*session).send)
            .expect(
                "non-null function pointer",
            )((*session).sockfd, &mut msg, 0 as libc::c_int)
            < 0 as libc::c_int as libc::c_long
        {
            ret = -(1 as libc::c_int);
        }
    }
    (*self_0).msg_buf = 0 as *mut buffer;
    (*self_0).pmap_buf = (*self_0).msg_buf;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn fast_get_field(
    mut msg: *mut fast_message,
    mut name: *const libc::c_char,
) -> *mut fast_field {
    return g_hash_table_lookup((*msg).ghtab, name as gconstpointer) as *mut fast_field;
}
