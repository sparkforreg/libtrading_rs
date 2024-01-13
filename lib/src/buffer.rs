use ::libc;
extern "C" {
    pub type internal_state;
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    static mut io_recv: io_recv_t;
    fn xread(fd: libc::c_int, buf: *mut libc::c_void, count: size_t) -> ssize_t;
    fn xwrite(fd: libc::c_int, buf: *const libc::c_void, count: size_t) -> ssize_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __ssize_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type u8_0 = uint8_t;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type va_list = __builtin_va_list;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buffer {
    pub start: libc::c_ulong,
    pub end: libc::c_ulong,
    pub capacity: libc::c_ulong,
    pub data: *mut libc::c_char,
}
pub type io_recv_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t, libc::c_int) -> ssize_t,
>;
#[inline]
unsafe extern "C" fn buffer_advance(mut self_0: *mut buffer, mut n: libc::c_long) {
    (*self_0).start = ((*self_0).start).wrapping_add(n as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn buffer_size(mut self_0: *const buffer) -> libc::c_ulong {
    return ((*self_0).end).wrapping_sub((*self_0).start);
}
#[inline]
unsafe extern "C" fn buffer_end(mut self_0: *const buffer) -> *mut libc::c_char {
    return &mut *((*self_0).data).offset((*self_0).end as isize) as *mut libc::c_char;
}
#[inline]
unsafe extern "C" fn buffer_remaining(mut self_0: *const buffer) -> libc::c_ulong {
    return ((*self_0).capacity).wrapping_sub((*self_0).end);
}
#[inline]
unsafe extern "C" fn buffer_start(mut self_0: *const buffer) -> *mut libc::c_char {
    return &mut *((*self_0).data).offset((*self_0).start as isize) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_new(mut capacity: libc::c_ulong) -> *mut buffer {
    let mut buf: *mut buffer = 0 as *mut buffer;
    buf = malloc(
        (::core::mem::size_of::<buffer>() as libc::c_ulong).wrapping_add(capacity),
    ) as *mut buffer;
    if buf.is_null() {
        return 0 as *mut buffer;
    }
    (*buf)
        .data = (buf as *mut libc::c_void)
        .offset(::core::mem::size_of::<buffer>() as libc::c_ulong as isize)
        as *mut libc::c_char;
    (*buf).capacity = capacity;
    (*buf).start = 0 as libc::c_int as libc::c_ulong;
    (*buf).end = 0 as libc::c_int as libc::c_ulong;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_delete(mut buf: *mut buffer) {
    free(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_sum_range(
    mut start: *const libc::c_char,
    mut end: *const libc::c_char,
) -> u8_0 {
    let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    ptr = start;
    while ptr < end {
        sum = sum.wrapping_add(*ptr as libc::c_ulong);
        ptr = ptr.offset(1);
        ptr;
    }
    return sum as u8_0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_sum(mut buf: *mut buffer) -> u8_0 {
    return buffer_sum_range(
        ((*buf).data).offset((*buf).start as isize),
        ((*buf).data).offset((*buf).end as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn buffer_append(mut dst: *mut buffer, mut src: *mut buffer) {
    let mut len: size_t = buffer_size(src);
    if len > buffer_remaining(dst) {
        len = buffer_remaining(dst);
    }
    memcpy(
        ((*dst).data).offset((*dst).start as isize) as *mut libc::c_void,
        ((*src).data).offset((*src).start as isize) as *const libc::c_void,
        len,
    );
    (*dst).start = ((*dst).start).wrapping_add(len);
    (*dst).end = ((*dst).end).wrapping_add(len);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_printf(
    mut buf: *mut buffer,
    mut format: *const libc::c_char,
    mut args: ...
) -> bool {
    let mut size: size_t = 0;
    let mut ap: ::core::ffi::VaListImpl;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    end = buffer_end(buf);
    size = buffer_remaining(buf);
    ap = args.clone();
    len = vsnprintf(end, size, format, ap.as_va_list());
    if len < 0 as libc::c_int || len as libc::c_ulong >= size {
        return 0 as libc::c_int != 0;
    }
    (*buf).end = ((*buf).end).wrapping_add(len as libc::c_ulong);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_recv(
    mut buf: *mut buffer,
    mut sockfd: libc::c_int,
    mut size: size_t,
    mut flags: libc::c_int,
) -> ssize_t {
    let mut count: size_t = 0;
    let mut len: ssize_t = 0;
    let mut end: *mut libc::c_void = 0 as *mut libc::c_void;
    end = buffer_end(buf) as *mut libc::c_void;
    count = buffer_remaining(buf);
    if count > size {
        count = size;
    }
    len = io_recv.expect("non-null function pointer")(sockfd, end, count, flags);
    if len < 0 as libc::c_int as libc::c_long {
        libc::perror("Asd".as_ptr() as *const libc::c_char);
        return len;
    }
    (*buf).end = ((*buf).end).wrapping_add(len as libc::c_ulong);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_read(
    mut buf: *mut buffer,
    mut fd: libc::c_int,
) -> ssize_t {
    let mut count: size_t = 0;
    let mut len: ssize_t = 0;
    let mut end: *mut libc::c_void = 0 as *mut libc::c_void;
    end = buffer_end(buf) as *mut libc::c_void;
    count = buffer_remaining(buf);
    len = read(fd, end, count);
    if len < 0 as libc::c_int as libc::c_long {
        return len;
    }
    (*buf).end = ((*buf).end).wrapping_add(len as libc::c_ulong);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_xread(
    mut buf: *mut buffer,
    mut fd: libc::c_int,
) -> ssize_t {
    let mut count: size_t = 0;
    let mut len: ssize_t = 0;
    let mut end: *mut libc::c_void = 0 as *mut libc::c_void;
    end = buffer_end(buf) as *mut libc::c_void;
    count = buffer_remaining(buf);
    len = xread(fd, end, count);
    if len < 0 as libc::c_int as libc::c_long {
        return len;
    }
    (*buf).end = ((*buf).end).wrapping_add(len as libc::c_ulong);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_nread(
    mut buf: *mut buffer,
    mut fd: libc::c_int,
    mut size: size_t,
) -> ssize_t {
    let mut count: size_t = 0;
    let mut len: ssize_t = 0;
    let mut end: *mut libc::c_void = 0 as *mut libc::c_void;
    end = buffer_end(buf) as *mut libc::c_void;
    count = buffer_remaining(buf);
    if count > size {
        count = size;
    }
    len = read(fd, end, count);
    if len < 0 as libc::c_int as libc::c_long {
        return len;
    }
    (*buf).end = ((*buf).end).wrapping_add(len as libc::c_ulong);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_nxread(
    mut buf: *mut buffer,
    mut fd: libc::c_int,
    mut size: size_t,
) -> ssize_t {
    let mut count: size_t = 0;
    let mut len: ssize_t = 0;
    let mut end: *mut libc::c_void = 0 as *mut libc::c_void;
    end = buffer_end(buf) as *mut libc::c_void;
    count = buffer_remaining(buf);
    if count > size {
        count = size;
    }
    len = xread(fd, end, count);
    if len < 0 as libc::c_int as libc::c_long {
        return len;
    }
    (*buf).end = ((*buf).end).wrapping_add(len as libc::c_ulong);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_write(
    mut buf: *mut buffer,
    mut fd: libc::c_int,
) -> ssize_t {
    let mut count: size_t = 0;
    let mut start: *mut libc::c_void = 0 as *mut libc::c_void;
    start = buffer_start(buf) as *mut libc::c_void;
    count = buffer_size(buf);
    return write(fd, start, count);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_xwrite(
    mut buf: *mut buffer,
    mut fd: libc::c_int,
) -> ssize_t {
    let mut count: size_t = 0;
    let mut start: *mut libc::c_void = 0 as *mut libc::c_void;
    start = buffer_start(buf) as *mut libc::c_void;
    count = buffer_size(buf);
    return xwrite(fd, start, count);
}
#[no_mangle]
pub unsafe extern "C" fn buffer_compact(mut buf: *mut buffer) {
    let mut count: size_t = 0;
    let mut start: *mut libc::c_void = 0 as *mut libc::c_void;
    start = buffer_start(buf) as *mut libc::c_void;
    count = buffer_size(buf);
    memmove((*buf).data as *mut libc::c_void, start, count);
    (*buf).start = 0 as libc::c_int as libc::c_ulong;
    (*buf).end = count;
}
#[no_mangle]
pub unsafe extern "C" fn buffer_inflate(
    mut comp_buf: *mut buffer,
    mut uncomp_buf: *mut buffer,
    mut stream: *mut z_stream,
) -> ssize_t {
    let mut nr: libc::c_ulong = 0;
    let mut ret: ssize_t = 0;
    let mut err: libc::c_int = 0;
    nr = buffer_size(comp_buf);
    if nr == 0 {
        return 0 as libc::c_int as ssize_t;
    }
    if nr as libc::c_ulonglong > (1 as libc::c_ulonglong) << 18 as libc::c_int {
        nr = ((1 as libc::c_ulonglong) << 18 as libc::c_int) as libc::c_ulong;
    }
    (*stream).avail_in = nr as uInt;
    (*stream).avail_out = buffer_remaining(uncomp_buf) as uInt;
    (*stream).next_out = buffer_end(uncomp_buf) as *mut libc::c_void as *mut Bytef;
    loop {
        err = inflate(stream, 0 as libc::c_int);
        match err {
            1 | -5 | 0 => {}
            _ => return -(1 as libc::c_int) as ssize_t,
        }
        if !(err == 0 && (*stream).avail_out == 0) {
            break;
        }
    }
    buffer_advance(
        comp_buf,
        nr.wrapping_sub((*stream).avail_in as libc::c_ulong) as libc::c_long,
    );
    ret = (buffer_remaining(uncomp_buf))
        .wrapping_sub((*stream).avail_out as libc::c_ulong) as ssize_t;
    (*uncomp_buf).end = ((*uncomp_buf).end).wrapping_add(ret as libc::c_ulong);
    return ret;
}
