use ::libc;
extern "C" {
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn writev(__fd: libc::c_int, __iovec: *const iovec, __count: libc::c_int) -> ssize_t;
    fn sendmsg(
        __fd: libc::c_int,
        __message: *const msghdr,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
pub type io_recv_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_void, size_t, libc::c_int) -> ssize_t,
>;
pub type io_sendmsg_t = Option::<
    unsafe extern "C" fn(libc::c_int, *mut iovec, size_t, libc::c_int) -> ssize_t,
>;
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
#[no_mangle]
pub unsafe extern "C" fn sys_sendmsg(
    mut fd: libc::c_int,
    mut iov: *mut iovec,
    mut length: size_t,
    mut flags: libc::c_int,
) -> ssize_t {
    let mut msg: msghdr = {
        let mut init = msghdr {
            msg_name: 0 as *mut libc::c_void,
            msg_namelen: 0,
            msg_iov: iov,
            msg_iovlen: length,
            msg_control: 0 as *mut libc::c_void,
            msg_controllen: 0,
            msg_flags: 0,
        };
        init
    };
    return sendmsg(fd, &mut msg, flags);
}
#[no_mangle]
pub static mut io_recv: io_recv_t = Some(
    recv
        as unsafe extern "C" fn(
            libc::c_int,
            *mut libc::c_void,
            size_t,
            libc::c_int,
        ) -> ssize_t,
);
#[no_mangle]
pub static mut io_sendmsg: io_sendmsg_t = Some(
    sys_sendmsg
        as unsafe extern "C" fn(libc::c_int, *mut iovec, size_t, libc::c_int) -> ssize_t,
);
#[no_mangle]
pub unsafe extern "C" fn iov_byte_length(
    mut iov: *mut iovec,
    mut iov_len: size_t,
) -> size_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < iov_len {
        len = (len as libc::c_ulong).wrapping_add((*iov.offset(i as isize)).iov_len)
            as size_t as size_t;
        i = i.wrapping_add(1);
        i;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn xread(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut nr: ssize_t = 0;
    loop {
        nr = read(fd, buf, count);
        if !(nr < 0 as libc::c_int as libc::c_long
            && (*__errno_location() == 11 as libc::c_int
                || *__errno_location() == 4 as libc::c_int))
        {
            break;
        }
    }
    return nr;
}
#[no_mangle]
pub unsafe extern "C" fn xwrite(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut nr: ssize_t = 0;
    loop {
        nr = write(fd, buf, count);
        if !(nr < 0 as libc::c_int as libc::c_long
            && (*__errno_location() == 11 as libc::c_int
                || *__errno_location() == 4 as libc::c_int))
        {
            break;
        }
    }
    return nr;
}
#[no_mangle]
pub unsafe extern "C" fn xwritev(
    mut fd: libc::c_int,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> ssize_t {
    let mut nr: ssize_t = 0;
    loop {
        nr = writev(fd, iov, iovcnt);
        if !(nr < 0 as libc::c_int as libc::c_long
            && (*__errno_location() == 11 as libc::c_int
                || *__errno_location() == 4 as libc::c_int))
        {
            break;
        }
    }
    return nr;
}
