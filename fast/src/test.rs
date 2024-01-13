use ::libc;
extern "C" {
    pub type _GHashTable;
    pub type buffer;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type i64_0 = int64_t;
pub type u64_0 = uint64_t;
pub type size_t = libc::c_ulong;
pub type GHashTable = _GHashTable;
pub type fast_type = libc::c_uint;
pub const FAST_TYPE_SEQUENCE: fast_type = 5;
pub const FAST_TYPE_DECIMAL: fast_type = 4;
pub const FAST_TYPE_VECTOR: fast_type = 3;
pub const FAST_TYPE_STRING: fast_type = 2;
pub const FAST_TYPE_UINT: fast_type = 1;
pub const FAST_TYPE_INT: fast_type = 0;
pub type fast_op = libc::c_uint;
pub const FAST_OP_CONSTANT: fast_op = 5;
pub const FAST_OP_DEFAULT: fast_op = 4;
pub const FAST_OP_DELTA: fast_op = 3;
pub const FAST_OP_INCR: fast_op = 2;
pub const FAST_OP_COPY: fast_op = 1;
pub const FAST_OP_NONE: fast_op = 0;
pub type fast_presence = libc::c_uint;
pub const FAST_PRESENCE_MANDATORY: fast_presence = 1;
pub const FAST_PRESENCE_OPTIONAL: fast_presence = 0;
pub type fast_state = libc::c_uint;
pub const FAST_STATE_EMPTY: fast_state = 2;
pub const FAST_STATE_ASSIGNED: fast_state = 1;
pub const FAST_STATE_UNDEFINED: fast_state = 0;
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
pub struct fast_decimal {
    pub fields: *mut fast_field,
    pub exp: i64_0,
    pub mnt: i64_0,
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
pub struct fast_sequence {
    pub pmap: fast_pmap,
    pub decoded: libc::c_ulong,
    pub length: fast_field,
    pub elements: [fast_message; 128],
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
    pub msg: fast_message,
    pub buf: [libc::c_char; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fcontainer {
    pub nr: libc::c_ulong,
    pub cur: libc::c_ulong,
    pub felems: [felem; 32],
}
#[inline]
unsafe extern "C" fn field_state_empty(mut field: *mut fast_field) -> bool {
    return (*field).state as libc::c_uint
        == FAST_STATE_EMPTY as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn field_set_empty(mut field: *mut fast_field) {
    (*field).state = FAST_STATE_EMPTY;
}
#[inline]
unsafe extern "C" fn field_is_mandatory(mut field: *mut fast_field) -> bool {
    return (*field).presence as libc::c_uint
        == FAST_PRESENCE_MANDATORY as libc::c_int as libc::c_uint;
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
                128 as libc::c_int as libc::c_ulong,
                ::core::mem::size_of::<fast_field>() as libc::c_ulong,
            ) as *mut fast_field;
            if ((*container).felems[i as usize].msg.fields).is_null() {
                current_block = 12296633977995419023;
                break;
            }
            i += 1;
            i;
        }
        match current_block {
            12296633977995419023 => {}
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
pub unsafe extern "C" fn fcontainer_init(
    mut self_0: *mut fcontainer,
    mut init_msg: *mut fast_message,
) {
    let mut i: libc::c_int = 0;
    let mut nr_fields: libc::c_ulong = 0;
    let mut msg: *mut fast_message = 0 as *mut fast_message;
    let mut fields: *mut fast_field = 0 as *mut fast_field;
    nr_fields = (*init_msg).nr_fields;
    fields = (*init_msg).fields;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        msg = &mut (*((*self_0).felems).as_mut_ptr().offset(i as isize)).msg;
        (*msg).nr_fields = nr_fields;
        (*msg).tid = 0 as libc::c_int as libc::c_ulong;
        memcpy(
            (*msg).fields as *mut libc::c_void,
            fields as *const libc::c_void,
            nr_fields.wrapping_mul(::core::mem::size_of::<fast_field>() as libc::c_ulong),
        );
        i += 1;
        i;
    }
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
    let mut current_block: u64;
    let mut field: *mut fast_field = 0 as *mut fast_field;
    let mut i: libc::c_ulong = 0;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    if !elem.is_null() {
        start = strncpy(
            ((*elem).buf).as_mut_ptr(),
            line,
            4096 as libc::c_int as libc::c_ulong,
        );
        i = 0 as libc::c_int as libc::c_ulong;
        loop {
            if !(i < (*elem).msg.nr_fields) {
                current_block = 14401909646449704462;
                break;
            }
            field = ((*elem).msg.fields).offset(i as isize);
            if strncmp(
                start,
                b"none\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                if field_is_mandatory(field) {
                    current_block = 3077528482775769914;
                    break;
                }
                field_set_empty(field);
                start = start.offset(5 as libc::c_int as isize);
            } else {
                match (*field).type_0 as libc::c_uint {
                    0 => {
                        (*field)
                            .c2rust_unnamed
                            .int_value = strtol(start, &mut end, 10 as libc::c_int);
                        start = end.offset(1 as libc::c_int as isize);
                    }
                    1 => {
                        (*field)
                            .c2rust_unnamed
                            .uint_value = strtoul(start, &mut end, 10 as libc::c_int);
                        start = end.offset(1 as libc::c_int as isize);
                    }
                    2 => {
                        if sscanf(
                            start,
                            b"%[^.,;|\n]s\0" as *const u8 as *const libc::c_char,
                            ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                        ) != 1 as libc::c_int
                        {
                            current_block = 3077528482775769914;
                            break;
                        }
                        start = start
                            .offset(
                                strlen(((*field).c2rust_unnamed.string_value).as_mut_ptr())
                                    as isize,
                            )
                            .offset(1 as libc::c_int as isize);
                    }
                    4 => {
                        (*field)
                            .c2rust_unnamed
                            .decimal_value
                            .exp = strtol(start, &mut end, 10 as libc::c_int);
                        start = end.offset(1 as libc::c_int as isize);
                        (*field)
                            .c2rust_unnamed
                            .decimal_value
                            .mnt = strtol(start, &mut end, 10 as libc::c_int);
                        start = end.offset(1 as libc::c_int as isize);
                    }
                    3 | 5 | _ => {
                        current_block = 3077528482775769914;
                        break;
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        match current_block {
            3077528482775769914 => {}
            _ => return 0 as libc::c_int,
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn script_read(
    mut stream: *mut FILE,
    mut self_0: *mut fcontainer,
) -> libc::c_int {
    let mut current_block: u64;
    let mut line: [libc::c_char; 4096] = [0; 4096];
    let mut size: libc::c_int = 4096 as libc::c_int;
    line[(size - 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
    add_elem(self_0);
    loop {
        if (fgets(line.as_mut_ptr(), size, stream)).is_null() {
            current_block = 15240798224410183470;
            break;
        }
        if line[(size - 1 as libc::c_int) as usize] != 0 {
            current_block = 15176578939689683289;
            break;
        }
        if init_elem(add_elem(self_0), line.as_mut_ptr()) != 0 {
            current_block = 15176578939689683289;
            break;
        }
    }
    match current_block {
        15240798224410183470 => return 0 as libc::c_int,
        _ => return 1 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn fmsgcmp(
    mut expected: *mut fast_message,
    mut actual: *mut fast_message,
) -> libc::c_int {
    let mut current_block: u64;
    let mut expected_field: *mut fast_field = 0 as *mut fast_field;
    let mut actual_field: *mut fast_field = 0 as *mut fast_field;
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0;
    if !((*expected).nr_fields != (*actual).nr_fields) {
        i = 0 as libc::c_int;
        loop {
            if !((i as libc::c_ulong) < (*expected).nr_fields) {
                current_block = 14401909646449704462;
                break;
            }
            actual_field = ((*actual).fields).offset(i as isize);
            expected_field = ((*expected).fields).offset(i as isize);
            if (*actual_field).presence as libc::c_uint
                != (*expected_field).presence as libc::c_uint
            {
                current_block = 8084644604950946301;
                break;
            }
            if (*actual_field).type_0 as libc::c_uint
                != (*expected_field).type_0 as libc::c_uint
            {
                current_block = 8084644604950946301;
                break;
            }
            if (*actual_field).op as libc::c_uint != (*expected_field).op as libc::c_uint
            {
                current_block = 8084644604950946301;
                break;
            }
            if field_state_empty(actual_field) {
                if !field_state_empty(expected_field) {
                    current_block = 8084644604950946301;
                    break;
                }
            } else {
                match (*expected_field).type_0 as libc::c_uint {
                    0 => {
                        current_block = 831993526624502787;
                        match current_block {
                            15078260472193513398 => {
                                if strcmp(
                                    ((*actual_field).c2rust_unnamed.string_value).as_mut_ptr(),
                                    ((*expected_field).c2rust_unnamed.string_value).as_mut_ptr(),
                                ) != 0
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            831993526624502787 => {
                                if (*actual_field).c2rust_unnamed.int_value
                                    != (*expected_field).c2rust_unnamed.int_value
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            9692668577222593701 => {
                                if (*actual_field).c2rust_unnamed.uint_value
                                    != (*expected_field).c2rust_unnamed.uint_value
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            _ => {
                                if (*actual_field).c2rust_unnamed.decimal_value.exp
                                    != (*expected_field).c2rust_unnamed.decimal_value.exp
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                                if (*actual_field).c2rust_unnamed.decimal_value.mnt
                                    != (*expected_field).c2rust_unnamed.decimal_value.mnt
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                        }
                    }
                    1 => {
                        current_block = 9692668577222593701;
                        match current_block {
                            15078260472193513398 => {
                                if strcmp(
                                    ((*actual_field).c2rust_unnamed.string_value).as_mut_ptr(),
                                    ((*expected_field).c2rust_unnamed.string_value).as_mut_ptr(),
                                ) != 0
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            831993526624502787 => {
                                if (*actual_field).c2rust_unnamed.int_value
                                    != (*expected_field).c2rust_unnamed.int_value
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            9692668577222593701 => {
                                if (*actual_field).c2rust_unnamed.uint_value
                                    != (*expected_field).c2rust_unnamed.uint_value
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            _ => {
                                if (*actual_field).c2rust_unnamed.decimal_value.exp
                                    != (*expected_field).c2rust_unnamed.decimal_value.exp
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                                if (*actual_field).c2rust_unnamed.decimal_value.mnt
                                    != (*expected_field).c2rust_unnamed.decimal_value.mnt
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                        }
                    }
                    2 => {
                        current_block = 15078260472193513398;
                        match current_block {
                            15078260472193513398 => {
                                if strcmp(
                                    ((*actual_field).c2rust_unnamed.string_value).as_mut_ptr(),
                                    ((*expected_field).c2rust_unnamed.string_value).as_mut_ptr(),
                                ) != 0
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            831993526624502787 => {
                                if (*actual_field).c2rust_unnamed.int_value
                                    != (*expected_field).c2rust_unnamed.int_value
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            9692668577222593701 => {
                                if (*actual_field).c2rust_unnamed.uint_value
                                    != (*expected_field).c2rust_unnamed.uint_value
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            _ => {
                                if (*actual_field).c2rust_unnamed.decimal_value.exp
                                    != (*expected_field).c2rust_unnamed.decimal_value.exp
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                                if (*actual_field).c2rust_unnamed.decimal_value.mnt
                                    != (*expected_field).c2rust_unnamed.decimal_value.mnt
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                        }
                    }
                    4 => {
                        current_block = 8156856794521447929;
                        match current_block {
                            15078260472193513398 => {
                                if strcmp(
                                    ((*actual_field).c2rust_unnamed.string_value).as_mut_ptr(),
                                    ((*expected_field).c2rust_unnamed.string_value).as_mut_ptr(),
                                ) != 0
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            831993526624502787 => {
                                if (*actual_field).c2rust_unnamed.int_value
                                    != (*expected_field).c2rust_unnamed.int_value
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            9692668577222593701 => {
                                if (*actual_field).c2rust_unnamed.uint_value
                                    != (*expected_field).c2rust_unnamed.uint_value
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                            _ => {
                                if (*actual_field).c2rust_unnamed.decimal_value.exp
                                    != (*expected_field).c2rust_unnamed.decimal_value.exp
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                                if (*actual_field).c2rust_unnamed.decimal_value.mnt
                                    != (*expected_field).c2rust_unnamed.decimal_value.mnt
                                {
                                    current_block = 8084644604950946301;
                                    break;
                                }
                            }
                        }
                    }
                    3 | 5 | _ => {}
                }
            }
            i += 1;
            i;
        }
        match current_block {
            8084644604950946301 => {}
            _ => {
                ret = 0 as libc::c_int;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn snprintmsg(
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut msg: *mut fast_message,
) -> libc::c_int {
    let mut field: *mut fast_field = 0 as *mut fast_field;
    let mut delim: libc::c_char = '|' as i32 as libc::c_char;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if !msg.is_null() {
        if (len as libc::c_ulong) < size {
            len
                += snprintf(
                    buf.offset(len as isize),
                    size.wrapping_sub(len as libc::c_ulong),
                    b"%c\0" as *const u8 as *const libc::c_char,
                    delim as libc::c_int,
                );
        }
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*msg).nr_fields && (len as libc::c_ulong) < size {
            field = ((*msg).fields).offset(i as isize);
            if field_state_empty(field) {
                len
                    += snprintf(
                        buf.offset(len as isize),
                        size.wrapping_sub(len as libc::c_ulong),
                        b"none%c\0" as *const u8 as *const libc::c_char,
                        delim as libc::c_int,
                    );
            } else {
                match (*field).type_0 as libc::c_uint {
                    0 => {
                        len
                            += snprintf(
                                buf.offset(len as isize),
                                size.wrapping_sub(len as libc::c_ulong),
                                b"%ld%c\0" as *const u8 as *const libc::c_char,
                                (*field).c2rust_unnamed.int_value,
                                delim as libc::c_int,
                            );
                    }
                    1 => {
                        len
                            += snprintf(
                                buf.offset(len as isize),
                                size.wrapping_sub(len as libc::c_ulong),
                                b"%lu%c\0" as *const u8 as *const libc::c_char,
                                (*field).c2rust_unnamed.uint_value,
                                delim as libc::c_int,
                            );
                    }
                    2 => {
                        len
                            += snprintf(
                                buf.offset(len as isize),
                                size.wrapping_sub(len as libc::c_ulong),
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                ((*field).c2rust_unnamed.string_value).as_mut_ptr(),
                                delim as libc::c_int,
                            );
                    }
                    3 => {
                        len
                            += snprintf(
                                buf.offset(len as isize),
                                size.wrapping_sub(len as libc::c_ulong),
                                b"%s%c\0" as *const u8 as *const libc::c_char,
                                ((*field).c2rust_unnamed.vector_value).as_mut_ptr(),
                                delim as libc::c_int,
                            );
                    }
                    4 => {
                        len
                            += snprintf(
                                buf.offset(len as isize),
                                size.wrapping_sub(len as libc::c_ulong),
                                b"%ld%c\0" as *const u8 as *const libc::c_char,
                                (*field).c2rust_unnamed.decimal_value.exp,
                                delim as libc::c_int,
                            );
                        len
                            += snprintf(
                                buf.offset(len as isize),
                                size.wrapping_sub(len as libc::c_ulong),
                                b"%ld%c\0" as *const u8 as *const libc::c_char,
                                (*field).c2rust_unnamed.decimal_value.mnt,
                                delim as libc::c_int,
                            );
                    }
                    5 => {
                        len
                            += snprintseq(
                                buf.offset(len as isize),
                                size.wrapping_sub(len as libc::c_ulong),
                                field,
                            );
                    }
                    _ => {}
                }
            }
            i += 1;
            i;
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn snprintseq(
    mut buf: *mut libc::c_char,
    mut size: size_t,
    mut field: *mut fast_field,
) -> libc::c_int {
    let mut seq: *mut fast_sequence = 0 as *mut fast_sequence;
    let mut msg: *mut fast_message = 0 as *mut fast_message;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if !(field.is_null()
        || (*field).type_0 as libc::c_uint
            != FAST_TYPE_SEQUENCE as libc::c_int as libc::c_uint)
    {
        if (len as libc::c_ulong) < size {
            len
                += snprintf(
                    buf.offset(len as isize),
                    size.wrapping_sub(len as libc::c_ulong),
                    b"\n<sequence>\n\0" as *const u8 as *const libc::c_char,
                );
        }
        seq = (*field).c2rust_unnamed.ptr_value as *mut fast_sequence;
        i = 1 as libc::c_int;
        while i as libc::c_ulong <= (*seq).length.c2rust_unnamed.uint_value
            && (len as libc::c_ulong) < size
        {
            msg = ((*seq).elements).as_mut_ptr().offset(i as isize);
            len
                += snprintmsg(
                    buf.offset(len as isize),
                    size.wrapping_sub(len as libc::c_ulong),
                    msg,
                );
            len
                += snprintf(
                    buf.offset(len as isize),
                    size.wrapping_sub(len as libc::c_ulong),
                    b"\n\0" as *const u8 as *const libc::c_char,
                );
            i += 1;
            i;
        }
        if (len as libc::c_ulong) < size {
            len
                += snprintf(
                    buf.offset(len as isize),
                    size.wrapping_sub(len as libc::c_ulong),
                    b"</sequence>\0" as *const u8 as *const libc::c_char,
                );
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn fprintmsg(mut stream: *mut FILE, mut msg: *mut fast_message) {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut size: libc::c_int = ::core::mem::size_of::<[libc::c_char; 4096]>()
        as libc::c_ulong as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    len += snprintmsg(buf.as_mut_ptr(), size as size_t, msg);
    if len < size {
        let fresh1 = len;
        len = len + 1;
        buf[fresh1 as usize] = '\0' as i32 as libc::c_char;
    }
    fprintf(stream, b"%s\n\0" as *const u8 as *const libc::c_char, buf.as_mut_ptr());
}
