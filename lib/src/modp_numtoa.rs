use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
static mut powers_of_10: [libc::c_double; 10] = [
    1 as libc::c_int as libc::c_double,
    10 as libc::c_int as libc::c_double,
    100 as libc::c_int as libc::c_double,
    1000 as libc::c_int as libc::c_double,
    10000 as libc::c_int as libc::c_double,
    100000 as libc::c_int as libc::c_double,
    1000000 as libc::c_int as libc::c_double,
    10000000 as libc::c_int as libc::c_double,
    100000000 as libc::c_int as libc::c_double,
    1000000000 as libc::c_int as libc::c_double,
];
unsafe extern "C" fn strreverse(
    mut begin: *mut libc::c_char,
    mut end: *mut libc::c_char,
) {
    let mut aux: libc::c_char = 0;
    while end > begin {
        aux = *end;
        let fresh0 = end;
        end = end.offset(-1);
        *fresh0 = *begin;
        let fresh1 = begin;
        begin = begin.offset(1);
        *fresh1 = aux;
    }
}
#[no_mangle]
pub unsafe extern "C" fn modp_itoa10(
    mut value: int32_t,
    mut str: *mut libc::c_char,
) -> size_t {
    let mut wstr: *mut libc::c_char = str;
    let mut uvalue: uint32_t = if value < 0 as libc::c_int {
        -value as uint32_t
    } else {
        value as uint32_t
    };
    loop {
        let fresh2 = wstr;
        wstr = wstr.offset(1);
        *fresh2 = (48 as libc::c_int as libc::c_uint)
            .wrapping_add(uvalue.wrapping_rem(10 as libc::c_int as libc::c_uint))
            as libc::c_char;
        uvalue = (uvalue as libc::c_uint).wrapping_div(10 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
        if !(uvalue != 0) {
            break;
        }
    }
    if value < 0 as libc::c_int {
        let fresh3 = wstr;
        wstr = wstr.offset(1);
        *fresh3 = '-' as i32 as libc::c_char;
    }
    *wstr = '\0' as i32 as libc::c_char;
    strreverse(str, wstr.offset(-(1 as libc::c_int as isize)));
    return wstr.offset_from(str) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn modp_uitoa10(
    mut value: uint32_t,
    mut str: *mut libc::c_char,
) -> size_t {
    let mut wstr: *mut libc::c_char = str;
    loop {
        let fresh4 = wstr;
        wstr = wstr.offset(1);
        *fresh4 = (48 as libc::c_int as libc::c_uint)
            .wrapping_add(value.wrapping_rem(10 as libc::c_int as libc::c_uint))
            as libc::c_char;
        value = (value as libc::c_uint).wrapping_div(10 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
        if !(value != 0) {
            break;
        }
    }
    *wstr = '\0' as i32 as libc::c_char;
    strreverse(str, wstr.offset(-(1 as libc::c_int as isize)));
    return wstr.offset_from(str) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn modp_litoa10(
    mut value: int64_t,
    mut str: *mut libc::c_char,
) -> size_t {
    let mut wstr: *mut libc::c_char = str;
    let mut uvalue: uint64_t = if value < 0 as libc::c_int as libc::c_long {
        -value as uint64_t
    } else {
        value as uint64_t
    };
    loop {
        let fresh5 = wstr;
        wstr = wstr.offset(1);
        *fresh5 = (48 as libc::c_int as libc::c_ulong)
            .wrapping_add(uvalue.wrapping_rem(10 as libc::c_int as libc::c_ulong))
            as libc::c_char;
        uvalue = (uvalue as libc::c_ulong)
            .wrapping_div(10 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
        if !(uvalue != 0) {
            break;
        }
    }
    if value < 0 as libc::c_int as libc::c_long {
        let fresh6 = wstr;
        wstr = wstr.offset(1);
        *fresh6 = '-' as i32 as libc::c_char;
    }
    *wstr = '\0' as i32 as libc::c_char;
    strreverse(str, wstr.offset(-(1 as libc::c_int as isize)));
    return wstr.offset_from(str) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn modp_ulitoa10(
    mut value: uint64_t,
    mut str: *mut libc::c_char,
) -> size_t {
    let mut wstr: *mut libc::c_char = str;
    loop {
        let fresh7 = wstr;
        wstr = wstr.offset(1);
        *fresh7 = (48 as libc::c_int as libc::c_ulong)
            .wrapping_add(value.wrapping_rem(10 as libc::c_int as libc::c_ulong))
            as libc::c_char;
        value = (value as libc::c_ulong).wrapping_div(10 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        if !(value != 0) {
            break;
        }
    }
    *wstr = '\0' as i32 as libc::c_char;
    strreverse(str, wstr.offset(-(1 as libc::c_int as isize)));
    return wstr.offset_from(str) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn modp_dtoa(
    mut value: libc::c_double,
    mut str: *mut libc::c_char,
    mut prec: libc::c_int,
) -> size_t {
    if !(value == value) {
        *str.offset(0 as libc::c_int as isize) = 'n' as i32 as libc::c_char;
        *str.offset(1 as libc::c_int as isize) = 'a' as i32 as libc::c_char;
        *str.offset(2 as libc::c_int as isize) = 'n' as i32 as libc::c_char;
        *str.offset(3 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return 3 as libc::c_int as size_t;
    }
    let thres_max: libc::c_double = 0x7fffffff as libc::c_int as libc::c_double;
    let mut diff: libc::c_double = 0.0f64;
    let mut wstr: *mut libc::c_char = str;
    if prec < 0 as libc::c_int {
        prec = 0 as libc::c_int;
    } else if prec > 9 as libc::c_int {
        prec = 9 as libc::c_int;
    }
    let mut neg: libc::c_int = 0 as libc::c_int;
    if value < 0 as libc::c_int as libc::c_double {
        neg = 1 as libc::c_int;
        value = -value;
    }
    let mut whole: libc::c_int = value as libc::c_int;
    let mut tmp: libc::c_double = (value - whole as libc::c_double)
        * powers_of_10[prec as usize];
    let mut frac: uint32_t = tmp as uint32_t;
    diff = tmp - frac as libc::c_double;
    if diff > 0.5f64 {
        frac = frac.wrapping_add(1);
        frac;
        if frac as libc::c_double >= powers_of_10[prec as usize] {
            frac = 0 as libc::c_int as uint32_t;
            whole += 1;
            whole;
        }
    } else if diff == 0.5f64
        && (frac == 0 as libc::c_int as libc::c_uint
            || frac & 1 as libc::c_int as libc::c_uint != 0)
    {
        frac = frac.wrapping_add(1);
        frac;
    }
    if value > thres_max {
        sprintf(
            str,
            b"%e\0" as *const u8 as *const libc::c_char,
            if neg != 0 { -value } else { value },
        );
        return strlen(str);
    }
    if prec == 0 as libc::c_int {
        diff = value - whole as libc::c_double;
        if diff > 0.5f64 {
            whole += 1;
            whole;
        } else if diff == 0.5f64 && whole & 1 as libc::c_int != 0 {
            whole += 1;
            whole;
        }
    } else {
        let mut count: libc::c_int = prec;
        loop {
            count -= 1;
            count;
            let fresh8 = wstr;
            wstr = wstr.offset(1);
            *fresh8 = (48 as libc::c_int as libc::c_uint)
                .wrapping_add(frac.wrapping_rem(10 as libc::c_int as libc::c_uint))
                as libc::c_char;
            frac = (frac as libc::c_uint).wrapping_div(10 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
            if !(frac != 0) {
                break;
            }
        }
        loop {
            let fresh9 = count;
            count = count - 1;
            if !(fresh9 > 0 as libc::c_int) {
                break;
            }
            let fresh10 = wstr;
            wstr = wstr.offset(1);
            *fresh10 = '0' as i32 as libc::c_char;
        }
        let fresh11 = wstr;
        wstr = wstr.offset(1);
        *fresh11 = '.' as i32 as libc::c_char;
    }
    loop {
        let fresh12 = wstr;
        wstr = wstr.offset(1);
        *fresh12 = (48 as libc::c_int + whole % 10 as libc::c_int) as libc::c_char;
        whole /= 10 as libc::c_int;
        if !(whole != 0) {
            break;
        }
    }
    if neg != 0 {
        let fresh13 = wstr;
        wstr = wstr.offset(1);
        *fresh13 = '-' as i32 as libc::c_char;
    }
    *wstr = '\0' as i32 as libc::c_char;
    strreverse(str, wstr.offset(-(1 as libc::c_int as isize)));
    return wstr.offset_from(str) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn modp_dtoa2(
    mut value: libc::c_double,
    mut str: *mut libc::c_char,
    mut prec: libc::c_int,
) -> size_t {
    if !(value == value) {
        *str.offset(0 as libc::c_int as isize) = 'n' as i32 as libc::c_char;
        *str.offset(1 as libc::c_int as isize) = 'a' as i32 as libc::c_char;
        *str.offset(2 as libc::c_int as isize) = 'n' as i32 as libc::c_char;
        *str.offset(3 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return 3 as libc::c_int as size_t;
    }
    let thres_max: libc::c_double = 0x7fffffff as libc::c_int as libc::c_double;
    let mut count: libc::c_int = 0;
    let mut diff: libc::c_double = 0.0f64;
    let mut wstr: *mut libc::c_char = str;
    if prec < 0 as libc::c_int {
        prec = 0 as libc::c_int;
    } else if prec > 9 as libc::c_int {
        prec = 9 as libc::c_int;
    }
    let mut neg: libc::c_int = 0 as libc::c_int;
    if value < 0 as libc::c_int as libc::c_double {
        neg = 1 as libc::c_int;
        value = -value;
    }
    let mut whole: libc::c_int = value as libc::c_int;
    let mut tmp: libc::c_double = (value - whole as libc::c_double)
        * powers_of_10[prec as usize];
    let mut frac: uint32_t = tmp as uint32_t;
    diff = tmp - frac as libc::c_double;
    if diff > 0.5f64 {
        frac = frac.wrapping_add(1);
        frac;
        if frac as libc::c_double >= powers_of_10[prec as usize] {
            frac = 0 as libc::c_int as uint32_t;
            whole += 1;
            whole;
        }
    } else if diff == 0.5f64
        && (frac == 0 as libc::c_int as libc::c_uint
            || frac & 1 as libc::c_int as libc::c_uint != 0)
    {
        frac = frac.wrapping_add(1);
        frac;
    }
    if value > thres_max {
        sprintf(
            str,
            b"%e\0" as *const u8 as *const libc::c_char,
            if neg != 0 { -value } else { value },
        );
        return strlen(str);
    }
    if prec == 0 as libc::c_int {
        diff = value - whole as libc::c_double;
        if diff > 0.5f64 {
            whole += 1;
            whole;
        } else if diff == 0.5f64 && whole & 1 as libc::c_int != 0 {
            whole += 1;
            whole;
        }
    } else if frac != 0 {
        count = prec;
        while frac.wrapping_rem(10 as libc::c_int as libc::c_uint) == 0 {
            count -= 1;
            count;
            frac = (frac as libc::c_uint).wrapping_div(10 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
        }
        loop {
            count -= 1;
            count;
            let fresh14 = wstr;
            wstr = wstr.offset(1);
            *fresh14 = (48 as libc::c_int as libc::c_uint)
                .wrapping_add(frac.wrapping_rem(10 as libc::c_int as libc::c_uint))
                as libc::c_char;
            frac = (frac as libc::c_uint).wrapping_div(10 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
            if !(frac != 0) {
                break;
            }
        }
        loop {
            let fresh15 = count;
            count = count - 1;
            if !(fresh15 > 0 as libc::c_int) {
                break;
            }
            let fresh16 = wstr;
            wstr = wstr.offset(1);
            *fresh16 = '0' as i32 as libc::c_char;
        }
        let fresh17 = wstr;
        wstr = wstr.offset(1);
        *fresh17 = '.' as i32 as libc::c_char;
    }
    loop {
        let fresh18 = wstr;
        wstr = wstr.offset(1);
        *fresh18 = (48 as libc::c_int + whole % 10 as libc::c_int) as libc::c_char;
        whole /= 10 as libc::c_int;
        if !(whole != 0) {
            break;
        }
    }
    if neg != 0 {
        let fresh19 = wstr;
        wstr = wstr.offset(1);
        *fresh19 = '-' as i32 as libc::c_char;
    }
    *wstr = '\0' as i32 as libc::c_char;
    strreverse(str, wstr.offset(-(1 as libc::c_int as isize)));
    return wstr.offset_from(str) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn modp_uitoa16(
    mut value: uint32_t,
    mut str: *mut libc::c_char,
    mut isfinal: libc::c_int,
) -> *mut libc::c_char {
    static mut hexchars: *const libc::c_char = b"0123456789ABCDEF\0" as *const u8
        as *const libc::c_char;
    *str
        .offset(
            0 as libc::c_int as isize,
        ) = *hexchars
        .offset(
            (value >> 28 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as isize,
        );
    *str
        .offset(
            1 as libc::c_int as isize,
        ) = *hexchars
        .offset(
            (value >> 24 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as isize,
        );
    *str
        .offset(
            2 as libc::c_int as isize,
        ) = *hexchars
        .offset(
            (value >> 20 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as isize,
        );
    *str
        .offset(
            3 as libc::c_int as isize,
        ) = *hexchars
        .offset(
            (value >> 16 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as isize,
        );
    *str
        .offset(
            4 as libc::c_int as isize,
        ) = *hexchars
        .offset(
            (value >> 12 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as isize,
        );
    *str
        .offset(
            5 as libc::c_int as isize,
        ) = *hexchars
        .offset(
            (value >> 8 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as isize,
        );
    *str
        .offset(
            6 as libc::c_int as isize,
        ) = *hexchars
        .offset(
            (value >> 4 as libc::c_int & 0xf as libc::c_int as libc::c_uint) as isize,
        );
    *str
        .offset(
            7 as libc::c_int as isize,
        ) = *hexchars.offset((value & 0xf as libc::c_int as libc::c_uint) as isize);
    if isfinal != 0 {
        *str.offset(8 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        return str;
    } else {
        return str.offset(8 as libc::c_int as isize)
    };
}
