use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
static mut itoa_tab: [libc::c_char; 201] = unsafe {
    *::core::mem::transmute::<
        &[u8; 201],
        &[libc::c_char; 201],
    >(
        b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\0",
    )
};
#[inline]
unsafe extern "C" fn uilog_10(mut n: libc::c_uint) -> libc::c_uint {
    match n {
        0..=9 => return 1 as libc::c_int as libc::c_uint,
        10..=99 => return 2 as libc::c_int as libc::c_uint,
        100..=999 => return 3 as libc::c_int as libc::c_uint,
        1000..=9999 => return 4 as libc::c_int as libc::c_uint,
        10000..=99999 => return 5 as libc::c_int as libc::c_uint,
        100000..=999999 => return 6 as libc::c_int as libc::c_uint,
        1000000..=9999999 => return 7 as libc::c_int as libc::c_uint,
        10000000..=99999999 => return 8 as libc::c_int as libc::c_uint,
        100000000..=999999999 => return 9 as libc::c_int as libc::c_uint,
        _ => return 10 as libc::c_int as libc::c_uint,
    };
}
unsafe extern "C" fn uitoa_general(
    mut n: libc::c_uint,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut log: libc::c_uint = uilog_10(n);
    let mut p: *mut libc::c_char = s.offset(log as isize);
    let mut src: *const libc::c_char = 0 as *const libc::c_char;
    while n as libc::c_ulong >= 100 as libc::c_ulong {
        src = itoa_tab
            .as_ptr()
            .offset(
                (2 as libc::c_ulong)
                    .wrapping_mul(
                        (n as libc::c_ulong).wrapping_rem(100 as libc::c_ulong),
                    ) as isize,
            );
        p = p.offset(-(2 as libc::c_ulong as isize));
        n = (n as libc::c_ulong).wrapping_div(100 as libc::c_ulong) as libc::c_uint
            as libc::c_uint;
        memcpy(p as *mut libc::c_void, src as *const libc::c_void, 2 as libc::c_ulong);
    }
    src = itoa_tab
        .as_ptr()
        .offset(
            (n.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong) as isize,
        )
        .offset(-(1 as libc::c_int as isize));
    p = p.offset(-1);
    p;
    match n {
        0..=9 => {}
        _ => {
            let fresh0 = src;
            src = src.offset(-1);
            let fresh1 = p;
            p = p.offset(-1);
            *fresh1 = *fresh0;
        }
    }
    let fresh2 = src;
    src = src.offset(-1);
    let fresh3 = p;
    p = p.offset(-1);
    *fresh3 = *fresh2;
    return log as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uitoa(
    mut n: libc::c_uint,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut src: *const libc::c_char = itoa_tab
        .as_ptr()
        .offset((2 as libc::c_ulong).wrapping_mul(n as libc::c_ulong) as isize);
    let mut p: *mut libc::c_char = s;
    's_25: {
        match n {
            10 ..=99 => {
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = *src.offset(0 as libc::c_int as isize);
            }
            0 ..=9 => {}
            _ => {
                break 's_25;
            }
        }
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = *src.offset(1 as libc::c_int as isize);
        return p.offset_from(s) as libc::c_long as libc::c_int;
    }
    return uitoa_general(n, p);
}
#[no_mangle]
pub unsafe extern "C" fn itoa(
    mut n: libc::c_int,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut un: libc::c_uint = n as libc::c_uint;
    let mut p: *mut libc::c_char = s;
    if n < 0 as libc::c_int {
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = '-' as i32 as libc::c_char;
        un = -n as libc::c_uint;
    }
    return p.offset(uitoa(un, p) as isize).offset_from(s) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn i64toa(
    mut n: int64_t,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    if n > 2147483647 as libc::c_int as libc::c_long
        || n < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
    {
        return sprintf(s, b"%ld\0" as *const u8 as *const libc::c_char, n)
    } else {
        return itoa(n as libc::c_int, s)
    };
}
#[no_mangle]
pub unsafe extern "C" fn checksumtoa(
    mut n: libc::c_int,
    mut s: *mut libc::c_char,
) -> libc::c_int {
    let mut src: *const libc::c_char = itoa_tab
        .as_ptr()
        .offset((2 as libc::c_ulong).wrapping_mul(n as libc::c_ulong) as isize);
    let mut p: *mut libc::c_char = s;
    match n {
        0..=99 => {
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = '0' as i32 as libc::c_char;
            let fresh8 = src;
            src = src.offset(1);
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = *fresh8;
            let fresh10 = src;
            src = src.offset(1);
            let fresh11 = p;
            p = p.offset(1);
            *fresh11 = *fresh10;
        }
        _ => return uitoa_general(n as libc::c_uint, s),
    }
    return p.offset_from(s) as libc::c_long as libc::c_int;
}
unsafe extern "C" fn strreverse(
    mut begin: *mut libc::c_char,
    mut end: *mut libc::c_char,
) {
    let mut aux: libc::c_char = 0;
    while end > begin {
        aux = *end;
        let fresh12 = end;
        end = end.offset(-1);
        *fresh12 = *begin;
        let fresh13 = begin;
        begin = begin.offset(1);
        *fresh13 = aux;
    }
}
#[no_mangle]
pub unsafe extern "C" fn modp_litoa10_zpad(
    mut value: int64_t,
    mut zpad: libc::c_int,
    mut str: *mut libc::c_char,
) -> size_t {
    let mut wstr: *mut libc::c_char = str;
    let mut uvalue: uint64_t = if value < 0 as libc::c_int as libc::c_long {
        -value as uint64_t
    } else {
        value as uint64_t
    };
    loop {
        let fresh14 = wstr;
        wstr = wstr.offset(1);
        *fresh14 = (48 as libc::c_int as libc::c_ulong)
            .wrapping_add(uvalue.wrapping_rem(10 as libc::c_int as libc::c_ulong))
            as libc::c_char;
        uvalue = (uvalue as libc::c_ulong)
            .wrapping_div(10 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
        if !(uvalue != 0) {
            break;
        }
    }
    zpad = (zpad as libc::c_long
        - (wstr.offset_from(str) as libc::c_long
            + (if value < 0 as libc::c_int as libc::c_long {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_long)) as libc::c_int;
    while zpad > 0 as libc::c_int {
        let fresh15 = wstr;
        wstr = wstr.offset(1);
        *fresh15 = '0' as i32 as libc::c_char;
        zpad -= 1;
        zpad;
    }
    if value < 0 as libc::c_int as libc::c_long {
        let fresh16 = wstr;
        wstr = wstr.offset(1);
        *fresh16 = '-' as i32 as libc::c_char;
    }
    strreverse(str, wstr.offset(-(1 as libc::c_int as isize)));
    return wstr.offset_from(str) as libc::c_long as size_t;
}

#[cfg(test)]
mod tests {
    use libc::{c_int, c_uint};
    use crate::itoa::{itoa, uitoa_general};

    #[test]
     fn itoa_test() {
        let mut buffer: Vec<u8> = Vec::with_capacity(20);

        unsafe {
            let size = itoa(c_int::from(-98), buffer.as_ptr() as *mut libc::c_char);
            assert_eq!(size, 3);
            buffer.set_len(size as usize);
        };
        let s = String::from_utf8(buffer).unwrap();
        assert_eq!("-98", s);
    }

    #[test]
    fn uitoa_general_test() {
        let mut buffer: Vec<u8> = Vec::with_capacity(20);

        unsafe {
            let size = uitoa_general(libc::c_uint::from(6789 as u32), buffer.as_ptr() as *mut libc::c_char);
            assert_eq!(size, 4);
            buffer.set_len(size as usize);
        };
        let s = String::from_utf8(buffer).unwrap();
        assert_eq!("6789", s);
    }
}
