use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
pub type va_list = __builtin_va_list;
pub type FILE = _IO_FILE;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::core::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn do_die(mut format: *const libc::c_char, mut args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    ap = args.clone();
    vprintf(format, ap.as_va_list());
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn error(mut format: *const libc::c_char, mut args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    printf(b"error: \0" as *const u8 as *const libc::c_char);
    ap = args.clone();
    vprintf(format, ap.as_va_list());
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    exit(1 as libc::c_int);
}
