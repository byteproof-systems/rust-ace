#[allow(unused_imports)]
use crate::__common::*;
pub use libc::group;
extern "C-unwind" {
    pub fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn euidaccess(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn eaccess(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn execveat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn lseek(
        __fd: libc::c_int,
        __offset: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn lseek64(
        __fd: libc::c_int,
        __offset: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_long;
}
#[inline]
pub unsafe extern "C-unwind" fn close(__fd: libc::c_int) -> libc::c_int {
    (libc::close(__fd as libc::c_int)) as libc::c_int
}
extern "C-unwind" {
    pub fn closefrom(__lowfd: libc::c_int);
}
#[inline]
pub unsafe extern "C-unwind" fn read(
    __fd: libc::c_int,
    __buf: *mut libc::c_void,
    __nbytes: libc::c_ulong,
) -> libc::c_long {
    (libc::read(__fd as libc::c_int, __buf as *mut libc::c_void, __nbytes as usize))
        as libc::c_long
}
#[inline]
pub unsafe extern "C-unwind" fn write(
    __fd: libc::c_int,
    __buf: *const libc::c_void,
    __n: libc::c_ulong,
) -> libc::c_long {
    (libc::write(__fd as libc::c_int, __buf as *const libc::c_void, __n as usize))
        as libc::c_long
}
extern "C-unwind" {
    pub fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: libc::c_ulong,
        __offset: libc::c_long,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn pwrite(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: libc::c_ulong,
        __offset: libc::c_long,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn pread64(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: libc::c_ulong,
        __offset: libc::c_long,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn pwrite64(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: libc::c_ulong,
        __offset: libc::c_long,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn ualarm(__value: libc::c_uint, __interval: libc::c_uint) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn usleep(__useconds: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pause() -> libc::c_int;
}
extern "C-unwind" {
    pub fn chown(
        __file: *const libc::c_char,
        __owner: libc::c_uint,
        __group: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fchown(
        __fd: libc::c_int,
        __owner: libc::c_uint,
        __group: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn lchown(
        __file: *const libc::c_char,
        __owner: libc::c_uint,
        __group: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fchownat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __owner: libc::c_uint,
        __group: libc::c_uint,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn chdir(__path: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fchdir(__fd: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getcwd(__buf: *mut libc::c_char, __size: libc::c_ulong) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn get_current_dir_name() -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn getwd(__buf: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn dup(__fd: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn dup3(
        __fd: libc::c_int,
        __fd2: libc::c_int,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub static mut __environ: *mut *mut libc::c_char;
}
extern "C" {
    pub static mut environ: *mut *mut libc::c_char;
}
extern "C-unwind" {
    pub fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fexecve(
        __fd: libc::c_int,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn execle(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn execlp(
        __file: *const libc::c_char,
        __arg: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn execvpe(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn nice(__inc: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn _exit(__status: libc::c_int);
}
extern "C-unwind" {
    pub fn pathconf(__path: *const libc::c_char, __name: libc::c_int) -> libc::c_long;
}
extern "C-unwind" {
    pub fn fpathconf(__fd: libc::c_int, __name: libc::c_int) -> libc::c_long;
}
extern "C-unwind" {
    pub fn sysconf(__name: libc::c_int) -> libc::c_long;
}
extern "C-unwind" {
    pub fn confstr(
        __name: libc::c_int,
        __buf: *mut libc::c_char,
        __len: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn getpid() -> libc::c_int;
}
extern "C-unwind" {
    pub fn getppid() -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpgrp() -> libc::c_int;
}
extern "C-unwind" {
    pub fn __getpgid(__pid: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpgid(__pid: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpgid(__pid: libc::c_int, __pgid: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpgrp() -> libc::c_int;
}
extern "C-unwind" {
    pub fn setsid() -> libc::c_int;
}
extern "C-unwind" {
    pub fn getsid(__pid: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getuid() -> libc::c_uint;
}
extern "C-unwind" {
    pub fn geteuid() -> libc::c_uint;
}
extern "C-unwind" {
    pub fn getgid() -> libc::c_uint;
}
extern "C-unwind" {
    pub fn getegid() -> libc::c_uint;
}
extern "C-unwind" {
    pub fn getgroups(__size: libc::c_int, __list: *mut libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn group_member(__gid: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setuid(__uid: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setreuid(__ruid: libc::c_uint, __euid: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn seteuid(__uid: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setgid(__gid: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setregid(__rgid: libc::c_uint, __egid: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setegid(__gid: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getresuid(
        __ruid: *mut libc::c_uint,
        __euid: *mut libc::c_uint,
        __suid: *mut libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getresgid(
        __rgid: *mut libc::c_uint,
        __egid: *mut libc::c_uint,
        __sgid: *mut libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setresuid(
        __ruid: libc::c_uint,
        __euid: libc::c_uint,
        __suid: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setresgid(
        __rgid: libc::c_uint,
        __egid: libc::c_uint,
        __sgid: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fork() -> libc::c_int;
}
extern "C-unwind" {
    pub fn vfork() -> libc::c_int;
}
extern "C-unwind" {
    pub fn _Fork() -> libc::c_int;
}
extern "C-unwind" {
    pub fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn ttyname_r(
        __fd: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isatty(__fd: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ttyslot() -> libc::c_int;
}
extern "C-unwind" {
    pub fn link(__from: *const libc::c_char, __to: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn linkat(
        __fromfd: libc::c_int,
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn symlink(
        __from: *const libc::c_char,
        __to: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: libc::c_ulong,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn symlinkat(
        __from: *const libc::c_char,
        __tofd: libc::c_int,
        __to: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn readlinkat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: libc::c_ulong,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn unlink(__name: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn rmdir(__path: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn tcgetpgrp(__fd: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn tcsetpgrp(__fd: libc::c_int, __pgrp_id: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getlogin() -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn getlogin_r(
        __name: *mut libc::c_char,
        __name_len: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setlogin(__name: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub static mut optarg: *mut libc::c_char;
}
extern "C" {
    pub static mut optind: libc::c_int;
}
extern "C" {
    pub static mut opterr: libc::c_int;
}
extern "C" {
    pub static mut optopt: libc::c_int;
}
extern "C-unwind" {
    pub fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn gethostname(__name: *mut libc::c_char, __len: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sethostname(__name: *const libc::c_char, __len: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sethostid(__id: libc::c_long) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getdomainname(__name: *mut libc::c_char, __len: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setdomainname(
        __name: *const libc::c_char,
        __len: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vhangup() -> libc::c_int;
}
extern "C-unwind" {
    pub fn revoke(__file: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn profil(
        __sample_buffer: *mut libc::c_ushort,
        __size: libc::c_ulong,
        __offset: libc::c_ulong,
        __scale: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn acct(__name: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getusershell() -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn endusershell();
}
extern "C-unwind" {
    pub fn setusershell();
}
extern "C-unwind" {
    pub fn daemon(__nochdir: libc::c_int, __noclose: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn chroot(__path: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpass(__prompt: *const libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn fsync(__fd: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn syncfs(__fd: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn gethostid() -> libc::c_long;
}
extern "C-unwind" {
    pub fn sync();
}
extern "C-unwind" {
    pub fn getpagesize() -> libc::c_int;
}
extern "C-unwind" {
    pub fn getdtablesize() -> libc::c_int;
}
extern "C-unwind" {
    pub fn truncate(__file: *const libc::c_char, __length: libc::c_long) -> libc::c_int;
}
extern "C-unwind" {
    pub fn truncate64(
        __file: *const libc::c_char,
        __length: libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ftruncate(__fd: libc::c_int, __length: libc::c_long) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ftruncate64(__fd: libc::c_int, __length: libc::c_long) -> libc::c_int;
}
extern "C-unwind" {
    pub fn brk(__addr: *mut libc::c_void) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sbrk(__delta: libc::c_long) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn syscall(__sysno: libc::c_long, ...) -> libc::c_long;
}
extern "C-unwind" {
    pub fn lockf(
        __fd: libc::c_int,
        __cmd: libc::c_int,
        __len: libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn lockf64(
        __fd: libc::c_int,
        __cmd: libc::c_int,
        __len: libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn copy_file_range(
        __infd: libc::c_int,
        __pinoff: *mut libc::c_long,
        __outfd: libc::c_int,
        __poutoff: *mut libc::c_long,
        __length: libc::c_ulong,
        __flags: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn crypt(
        __key: *const libc::c_char,
        __salt: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn swab(__from: *const libc::c_void, __to: *mut libc::c_void, __n: libc::c_long);
}
extern "C-unwind" {
    pub fn getentropy(
        __buffer: *mut libc::c_void,
        __length: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn close_range(
        __fd: libc::c_uint,
        __max_fd: libc::c_uint,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn gettid() -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcscpy(
        __dest: *mut libc::wchar_t,
        __src: *const libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcsncpy(
        __dest: *mut libc::wchar_t,
        __src: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcslcpy(
        __dest: *mut libc::wchar_t,
        __src: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcslcat(
        __dest: *mut libc::wchar_t,
        __src: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcscat(
        __dest: *mut libc::wchar_t,
        __src: *const libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcsncat(
        __dest: *mut libc::wchar_t,
        __src: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcscmp(__s1: *const libc::wchar_t, __s2: *const libc::wchar_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcsncmp(
        __s1: *const libc::wchar_t,
        __s2: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcscasecmp(
        __s1: *const libc::wchar_t,
        __s2: *const libc::wchar_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcsncasecmp(
        __s1: *const libc::wchar_t,
        __s2: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcscasecmp_l(
        __s1: *const libc::wchar_t,
        __s2: *const libc::wchar_t,
        __loc: *mut __locale_struct,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcsncasecmp_l(
        __s1: *const libc::wchar_t,
        __s2: *const libc::wchar_t,
        __n: libc::c_ulong,
        __loc: *mut __locale_struct,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcscoll(
        __s1: *const libc::wchar_t,
        __s2: *const libc::wchar_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcsxfrm(
        __s1: *mut libc::wchar_t,
        __s2: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcscoll_l(
        __s1: *const libc::wchar_t,
        __s2: *const libc::wchar_t,
        __loc: *mut __locale_struct,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcsxfrm_l(
        __s1: *mut libc::wchar_t,
        __s2: *const libc::wchar_t,
        __n: libc::c_ulong,
        __loc: *mut __locale_struct,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcsdup(__s: *const libc::wchar_t) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcschr(
        __wcs: *const libc::wchar_t,
        __wc: libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcsrchr(
        __wcs: *const libc::wchar_t,
        __wc: libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcschrnul(
        __s: *const libc::wchar_t,
        __wc: libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcscspn(
        __wcs: *const libc::wchar_t,
        __reject: *const libc::wchar_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcsspn(
        __wcs: *const libc::wchar_t,
        __accept: *const libc::wchar_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcspbrk(
        __wcs: *const libc::wchar_t,
        __accept: *const libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcsstr(
        __haystack: *const libc::wchar_t,
        __needle: *const libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcstok(
        __s: *mut libc::wchar_t,
        __delim: *const libc::wchar_t,
        __ptr: *mut *mut libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcslen(__s: *const libc::wchar_t) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcswcs(
        __haystack: *const libc::wchar_t,
        __needle: *const libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcsnlen(__s: *const libc::wchar_t, __maxlen: libc::c_ulong) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wmemchr(
        __s: *const libc::wchar_t,
        __c: libc::wchar_t,
        __n: libc::c_ulong,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wmemcmp(
        __s1: *const libc::wchar_t,
        __s2: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wmemcpy(
        __s1: *mut libc::wchar_t,
        __s2: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wmemmove(
        __s1: *mut libc::wchar_t,
        __s2: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wmemset(
        __s: *mut libc::wchar_t,
        __c: libc::wchar_t,
        __n: libc::c_ulong,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wmempcpy(
        __s1: *mut libc::wchar_t,
        __s2: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn btowc(__c: libc::c_int) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn wctob(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mbsinit(__ps: *const __mbstate_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mbrtowc(
        __pwc: *mut libc::wchar_t,
        __s: *const libc::c_char,
        __n: libc::c_ulong,
        __p: *mut __mbstate_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcrtomb(
        __s: *mut libc::c_char,
        __wc: libc::wchar_t,
        __ps: *mut __mbstate_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __mbrlen(
        __s: *const libc::c_char,
        __n: libc::c_ulong,
        __ps: *mut __mbstate_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn mbrlen(
        __s: *const libc::c_char,
        __n: libc::c_ulong,
        __ps: *mut __mbstate_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    #[link_name = "btowc"]
    pub fn __btowc_alias(__c: libc::c_int) -> libc::c_uint;
}
extern "C-unwind" {
    #[link_name = "wctob"]
    pub fn __wctob_alias(__c: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mbsrtowcs(
        __dst: *mut libc::wchar_t,
        __src: *mut *const libc::c_char,
        __len: libc::c_ulong,
        __ps: *mut __mbstate_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcsrtombs(
        __dst: *mut libc::c_char,
        __src: *mut *const libc::wchar_t,
        __len: libc::c_ulong,
        __ps: *mut __mbstate_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn mbsnrtowcs(
        __dst: *mut libc::wchar_t,
        __src: *mut *const libc::c_char,
        __nmc: libc::c_ulong,
        __len: libc::c_ulong,
        __ps: *mut __mbstate_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcsnrtombs(
        __dst: *mut libc::c_char,
        __src: *mut *const libc::wchar_t,
        __nwc: libc::c_ulong,
        __len: libc::c_ulong,
        __ps: *mut __mbstate_t,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcwidth(__c: libc::wchar_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcswidth(__s: *const libc::wchar_t, __n: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcstod(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn wcstof(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn wcstold(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn wcstof32(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn wcstof64(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn wcstof32x(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn wcstof64x(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn wcstol(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn wcstoul(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcstoll(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
    ) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn wcstoull(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
    ) -> libc::c_ulonglong;
}
extern "C-unwind" {
    pub fn wcstoq(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
    ) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn wcstouq(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
    ) -> libc::c_ulonglong;
}
extern "C-unwind" {
    pub fn wcstol_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
        __loc: *mut __locale_struct,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn wcstoul_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
        __loc: *mut __locale_struct,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcstoll_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
        __loc: *mut __locale_struct,
    ) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn wcstoull_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
        __loc: *mut __locale_struct,
    ) -> libc::c_ulonglong;
}
extern "C-unwind" {
    pub fn wcstod_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __loc: *mut __locale_struct,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn wcstof_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __loc: *mut __locale_struct,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn wcstold_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __loc: *mut __locale_struct,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn wcstof32_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __loc: *mut __locale_struct,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn wcstof64_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __loc: *mut __locale_struct,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn wcstof32x_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __loc: *mut __locale_struct,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn wcstof64x_l(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __loc: *mut __locale_struct,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn wcpcpy(
        __dest: *mut libc::wchar_t,
        __src: *const libc::wchar_t,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn wcpncpy(
        __dest: *mut libc::wchar_t,
        __src: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn open_wmemstream(
        __bufloc: *mut *mut libc::wchar_t,
        __sizeloc: *mut libc::c_ulong,
    ) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn fwide(__fp: *mut _IO_FILE, __mode: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fwprintf(
        __stream: *mut _IO_FILE,
        __format: *const libc::wchar_t,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wprintf(__format: *const libc::wchar_t, ...) -> libc::c_int;
}
extern "C-unwind" {
    pub fn swprintf(
        __s: *mut libc::wchar_t,
        __n: libc::c_ulong,
        __format: *const libc::wchar_t,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vfwprintf(
        __s: *mut _IO_FILE,
        __format: *const libc::wchar_t,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vwprintf(
        __format: *const libc::wchar_t,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vswprintf(
        __s: *mut libc::wchar_t,
        __n: libc::c_ulong,
        __format: *const libc::wchar_t,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fwscanf(
        __stream: *mut _IO_FILE,
        __format: *const libc::wchar_t,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wscanf(__format: *const libc::wchar_t, ...) -> libc::c_int;
}
extern "C-unwind" {
    pub fn swscanf(
        __s: *const libc::wchar_t,
        __format: *const libc::wchar_t,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vfwscanf(
        __s: *mut _IO_FILE,
        __format: *const libc::wchar_t,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vwscanf(
        __format: *const libc::wchar_t,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vswscanf(
        __s: *const libc::wchar_t,
        __format: *const libc::wchar_t,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fgetwc(__stream: *mut _IO_FILE) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn getwc(__stream: *mut _IO_FILE) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn getwchar() -> libc::c_uint;
}
extern "C-unwind" {
    pub fn fputwc(__wc: libc::wchar_t, __stream: *mut _IO_FILE) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn putwc(__wc: libc::wchar_t, __stream: *mut _IO_FILE) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn putwchar(__wc: libc::wchar_t) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn fgetws(
        __ws: *mut libc::wchar_t,
        __n: libc::c_int,
        __stream: *mut _IO_FILE,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn fputws(__ws: *const libc::wchar_t, __stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ungetwc(__wc: libc::c_uint, __stream: *mut _IO_FILE) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn getwc_unlocked(__stream: *mut _IO_FILE) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn getwchar_unlocked() -> libc::c_uint;
}
extern "C-unwind" {
    pub fn fgetwc_unlocked(__stream: *mut _IO_FILE) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn fputwc_unlocked(__wc: libc::wchar_t, __stream: *mut _IO_FILE) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn putwc_unlocked(__wc: libc::wchar_t, __stream: *mut _IO_FILE) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn putwchar_unlocked(__wc: libc::wchar_t) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn fgetws_unlocked(
        __ws: *mut libc::wchar_t,
        __n: libc::c_int,
        __stream: *mut _IO_FILE,
    ) -> *mut libc::wchar_t;
}
extern "C-unwind" {
    pub fn fputws_unlocked(
        __ws: *const libc::wchar_t,
        __stream: *mut _IO_FILE,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wcsftime(
        __s: *mut libc::wchar_t,
        __maxsize: libc::c_ulong,
        __format: *const libc::wchar_t,
        __tp: *const tm,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcsftime_l(
        __s: *mut libc::wchar_t,
        __maxsize: libc::c_ulong,
        __format: *const libc::wchar_t,
        __tp: *const tm,
        __loc: *mut __locale_struct,
    ) -> libc::c_ulong;
}
pub mod std {}
pub mod __gnu_cxx {}
extern "C-unwind" {
    pub fn iswalnum(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswalpha(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswcntrl(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswdigit(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswgraph(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswlower(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswprint(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswpunct(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswspace(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswupper(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswxdigit(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswblank(__wc: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wctype(__property: *const libc::c_char) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn iswctype(__wc: libc::c_uint, __desc: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn towlower(__wc: libc::c_uint) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn towupper(__wc: libc::c_uint) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn wctrans(__property: *const libc::c_char) -> *const libc::c_int;
}
extern "C-unwind" {
    pub fn towctrans(__wc: libc::c_uint, __desc: *const libc::c_int) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn iswalnum_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswalpha_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswcntrl_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswdigit_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswgraph_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswlower_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswprint_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswpunct_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswspace_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswupper_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswxdigit_l(
        __wc: libc::c_uint,
        __locale: *mut __locale_struct,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iswblank_l(__wc: libc::c_uint, __locale: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wctype_l(
        __property: *const libc::c_char,
        __locale: *mut __locale_struct,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn iswctype_l(
        __wc: libc::c_uint,
        __desc: libc::c_ulong,
        __locale: *mut __locale_struct,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn towlower_l(
        __wc: libc::c_uint,
        __locale: *mut __locale_struct,
    ) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn towupper_l(
        __wc: libc::c_uint,
        __locale: *mut __locale_struct,
    ) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn wctrans_l(
        __property: *const libc::c_char,
        __locale: *mut __locale_struct,
    ) -> *const libc::c_int;
}
extern "C-unwind" {
    pub fn towctrans_l(
        __wc: libc::c_uint,
        __desc: *const libc::c_int,
        __locale: *mut __locale_struct,
    ) -> libc::c_uint;
}
#[inline]
pub unsafe extern "C-unwind" fn memcpy(
    __dest: *mut libc::c_void,
    __src: *const libc::c_void,
    __n: libc::c_ulong,
) -> *mut libc::c_void {
    (libc::memcpy(
        __dest as *mut libc::c_void,
        __src as *const libc::c_void,
        __n as usize,
    )) as *mut libc::c_void
}
#[inline]
pub unsafe extern "C-unwind" fn memmove(
    __dest: *mut libc::c_void,
    __src: *const libc::c_void,
    __n: libc::c_ulong,
) -> *mut libc::c_void {
    (libc::memmove(
        __dest as *mut libc::c_void,
        __src as *const libc::c_void,
        __n as usize,
    )) as *mut libc::c_void
}
extern "C-unwind" {
    pub fn memccpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __c: libc::c_int,
        __n: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[inline]
pub unsafe extern "C-unwind" fn memset(
    __s: *mut libc::c_void,
    __c: libc::c_int,
    __n: libc::c_ulong,
) -> *mut libc::c_void {
    (libc::memset(__s as *mut libc::c_void, __c as libc::c_int, __n as usize))
        as *mut libc::c_void
}
#[inline]
pub unsafe extern "C-unwind" fn memcmp(
    __s1: *const libc::c_void,
    __s2: *const libc::c_void,
    __n: libc::c_ulong,
) -> libc::c_int {
    (libc::memcmp(
        __s1 as *const libc::c_void,
        __s2 as *const libc::c_void,
        __n as usize,
    )) as libc::c_int
}
extern "C-unwind" {
    pub fn __memcmpeq(
        __s1: *const libc::c_void,
        __s2: *const libc::c_void,
        __n: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "memchr"]
    pub fn memchr_u5703f9a3cf66b015(
        __s: *mut libc::c_void,
        __c: libc::c_int,
        __n: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn memchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: libc::c_ulong,
    ) -> *const libc::c_void;
}
extern "C-unwind" {
    #[link_name = "rawmemchr"]
    pub fn rawmemchr_ua86af18e31aaf4be(
        __s: *mut libc::c_void,
        __c: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *const libc::c_void;
}
extern "C-unwind" {
    #[link_name = "memrchr"]
    pub fn memrchr_u6e3522a7f9f5ca05(
        __s: *mut libc::c_void,
        __c: libc::c_int,
        __n: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn memrchr(
        __s: *const libc::c_void,
        __c: libc::c_int,
        __n: libc::c_ulong,
    ) -> *const libc::c_void;
}
extern "C-unwind" {
    pub fn strcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strncpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strcat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strncat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strcmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strncmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strxfrm(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strcoll_l(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __l: *mut __locale_struct,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strxfrm_l(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: libc::c_ulong,
        __l: *mut __locale_struct,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strndup(
        __string: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    #[link_name = "strchr"]
    pub fn strchr_u17813cfb3efa5ac3(
        __s: *mut libc::c_char,
        __c: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strchr(__s: *const libc::c_char, __c: libc::c_int) -> *const libc::c_char;
}
extern "C-unwind" {
    #[link_name = "strrchr"]
    pub fn strrchr_uc749e06178faef0b(
        __s: *mut libc::c_char,
        __c: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strrchr(__s: *const libc::c_char, __c: libc::c_int) -> *const libc::c_char;
}
extern "C-unwind" {
    #[link_name = "strchrnul"]
    pub fn strchrnul_u574108f323ce4bcc(
        __s: *mut libc::c_char,
        __c: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *const libc::c_char;
}
extern "C-unwind" {
    pub fn strcspn(
        __s: *const libc::c_char,
        __reject: *const libc::c_char,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strspn(
        __s: *const libc::c_char,
        __accept: *const libc::c_char,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    #[link_name = "strpbrk"]
    pub fn strpbrk_u0bcf0628f1a774d8(
        __s: *mut libc::c_char,
        __accept: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strpbrk(
        __s: *const libc::c_char,
        __accept: *const libc::c_char,
    ) -> *const libc::c_char;
}
extern "C-unwind" {
    #[link_name = "strstr"]
    pub fn strstr_u2343dc4b7055c78c(
        __haystack: *mut libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strstr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *const libc::c_char;
}
extern "C-unwind" {
    pub fn strtok(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn __strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    #[link_name = "strcasestr"]
    pub fn strcasestr_uf1863a8a6e74fd80(
        __haystack: *mut libc::c_char,
        __needle: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strcasestr(
        __haystack: *const libc::c_char,
        __needle: *const libc::c_char,
    ) -> *const libc::c_char;
}
extern "C-unwind" {
    pub fn memmem(
        __haystack: *const libc::c_void,
        __haystacklen: libc::c_ulong,
        __needle: *const libc::c_void,
        __needlelen: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn __mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[inline]
pub unsafe extern "C-unwind" fn strlen(__s: *const libc::c_char) -> libc::c_ulong {
    (libc::strlen(__s as *const libc::c_char)) as libc::c_ulong
}
extern "C-unwind" {
    pub fn strnlen(
        __string: *const libc::c_char,
        __maxlen: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strerror(__errnum: libc::c_int) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strerror_r(
        __errnum: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strerrordesc_np(__err: libc::c_int) -> *const libc::c_char;
}
extern "C-unwind" {
    pub fn strerrorname_np(__err: libc::c_int) -> *const libc::c_char;
}
extern "C-unwind" {
    pub fn strerror_l(
        __errnum: libc::c_int,
        __l: *mut __locale_struct,
    ) -> *mut libc::c_char;
}
#[inline]
pub unsafe extern "C-unwind" fn bcmp(
    __s1: *const libc::c_void,
    __s2: *const libc::c_void,
    __n: libc::c_ulong,
) -> libc::c_int {
    (libc::memcmp(
        __s1 as *const libc::c_void,
        __s2 as *const libc::c_void,
        __n as usize,
    )) as libc::c_int
}
extern "C-unwind" {
    pub fn bcopy(
        __src: *const libc::c_void,
        __dest: *mut libc::c_void,
        __n: libc::c_ulong,
    );
}
extern "C-unwind" {
    pub fn bzero(__s: *mut libc::c_void, __n: libc::c_ulong);
}
extern "C-unwind" {
    pub fn index(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn rindex(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn ffs(__i: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ffsl(__l: libc::c_long) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ffsll(__ll: libc::c_longlong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strcasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strcasecmp_l(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __loc: *mut __locale_struct,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strncasecmp_l(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: libc::c_ulong,
        __loc: *mut __locale_struct,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn explicit_bzero(__s: *mut libc::c_void, __n: libc::c_ulong);
}
extern "C-unwind" {
    pub fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strsignal(__sig: libc::c_int) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn sigabbrev_np(__sig: libc::c_int) -> *const libc::c_char;
}
extern "C-unwind" {
    pub fn sigdescr_np(__sig: libc::c_int) -> *const libc::c_char;
}
extern "C-unwind" {
    pub fn __stpcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn stpcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn __stpncpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn stpncpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strlcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strlcat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strverscmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strfry(__string: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn memfrob(__s: *mut libc::c_void, __n: libc::c_ulong) -> *mut libc::c_void;
}
extern "C-unwind" {
    #[link_name = "basename"]
    pub fn basename_ud72ac48d8d0a885f(
        __filename: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn basename(__filename: *const libc::c_char) -> *const libc::c_char;
}
extern "C" {
    pub static mut ace_exit_hook_marker: libc::c_int;
}
pub(crate) unsafe extern "C-unwind" fn __bswap_16(
    mut __bsx: libc::c_ushort,
) -> libc::c_ushort {
    unsafe {
        {
            return (((((((((((((((__bsx)) as libc::c_int)).wrapping_shr((8) as u32)))
                as libc::c_int)) & ((255) as libc::c_int))) as libc::c_int))
                | (((((((((((__bsx)) as libc::c_int)) & ((255) as libc::c_int)))
                    as libc::c_int))
                    .wrapping_shl((8) as u32))) as libc::c_int)) as libc::c_ushort));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __bswap_32(
    mut __bsx: libc::c_uint,
) -> libc::c_uint {
    unsafe {
        {
            return (((((((((((((((((((((__bsx)) as libc::c_uint))
                & ((4278190080i64) as libc::c_uint))) as libc::c_uint))
                .wrapping_shr((24) as u32))) as libc::c_uint))
                | (((((((((((__bsx)) as libc::c_uint)) & ((16711680) as libc::c_uint)))
                    as libc::c_uint))
                    .wrapping_shr((8) as u32))) as libc::c_uint)) as libc::c_uint))
                | (((((((((((__bsx)) as libc::c_uint)) & ((65280) as libc::c_uint)))
                    as libc::c_uint))
                    .wrapping_shl((8) as u32))) as libc::c_uint)) as libc::c_uint))
                | (((((((((((__bsx)) as libc::c_uint)) & ((255) as libc::c_uint)))
                    as libc::c_uint))
                    .wrapping_shl((24) as u32))) as libc::c_uint))) as libc::c_uint);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __bswap_64(
    mut __bsx: libc::c_ulong,
) -> libc::c_ulong {
    unsafe {
        {
            return (((((((((((((((((((((((((((((((((__bsx)) as libc::c_ulonglong))
                & ((18374686479671623680u64) as libc::c_ulonglong)))
                as libc::c_ulonglong))
                .wrapping_shr((56) as u32))) as libc::c_ulonglong))
                | (((((((((((__bsx)) as libc::c_ulonglong))
                    & ((71776119061217280i64) as libc::c_ulonglong)))
                    as libc::c_ulonglong))
                    .wrapping_shr((40) as u32))) as libc::c_ulonglong))
                as libc::c_ulonglong))
                | (((((((((((__bsx)) as libc::c_ulonglong))
                    & ((280375465082880i64) as libc::c_ulonglong)))
                    as libc::c_ulonglong))
                    .wrapping_shr((24) as u32))) as libc::c_ulonglong))
                as libc::c_ulonglong))
                | (((((((((((__bsx)) as libc::c_ulonglong))
                    & ((1095216660480i64) as libc::c_ulonglong))) as libc::c_ulonglong))
                    .wrapping_shr((8) as u32))) as libc::c_ulonglong))
                as libc::c_ulonglong))
                | (((((((((((__bsx)) as libc::c_ulonglong))
                    & ((4278190080i64) as libc::c_ulonglong))) as libc::c_ulonglong))
                    .wrapping_shl((8) as u32))) as libc::c_ulonglong))
                as libc::c_ulonglong))
                | (((((((((((__bsx)) as libc::c_ulonglong))
                    & ((16711680) as libc::c_ulonglong))) as libc::c_ulonglong))
                    .wrapping_shl((24) as u32))) as libc::c_ulonglong))
                as libc::c_ulonglong))
                | (((((((((((__bsx)) as libc::c_ulonglong))
                    & ((65280) as libc::c_ulonglong))) as libc::c_ulonglong))
                    .wrapping_shl((40) as u32))) as libc::c_ulonglong))
                as libc::c_ulonglong))
                | (((((((((((__bsx)) as libc::c_ulonglong))
                    & ((255) as libc::c_ulonglong))) as libc::c_ulonglong))
                    .wrapping_shl((56) as u32))) as libc::c_ulonglong)))
                as libc::c_ulong);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __uint16_identity(
    mut __x: libc::c_ushort,
) -> libc::c_ushort {
    unsafe {
        {
            return __x;
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __uint32_identity(
    mut __x: libc::c_uint,
) -> libc::c_uint {
    unsafe {
        {
            return __x;
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __uint64_identity(
    mut __x: libc::c_ulong,
) -> libc::c_ulong {
    unsafe {
        {
            return __x;
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
extern "C-unwind" {
    pub fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pselect(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn readahead(
        __fd: libc::c_int,
        __offset: libc::c_long,
        __count: libc::c_ulong,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn sync_file_range(
        __fd: libc::c_int,
        __offset: libc::c_long,
        __count: libc::c_long,
        __flags: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vmsplice(
        __fdout: libc::c_int,
        __iov: *const iovec,
        __count: libc::c_ulong,
        __flags: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn splice(
        __fdin: libc::c_int,
        __offin: *mut libc::c_long,
        __fdout: libc::c_int,
        __offout: *mut libc::c_long,
        __len: libc::c_ulong,
        __flags: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn tee(
        __fdin: libc::c_int,
        __fdout: libc::c_int,
        __len: libc::c_ulong,
        __flags: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn fallocate(
        __fd: libc::c_int,
        __mode: libc::c_int,
        __offset: libc::c_long,
        __len: libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fallocate64(
        __fd: libc::c_int,
        __mode: libc::c_int,
        __offset: libc::c_long,
        __len: libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn name_to_handle_at(
        __dfd: libc::c_int,
        __name: *const libc::c_char,
        __handle: *mut file_handle,
        __mnt_id: *mut libc::c_int,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn open_by_handle_at(
        __mountdirfd: libc::c_int,
        __handle: *mut file_handle,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, ...) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fcntl64(__fd: libc::c_int, __cmd: libc::c_int, ...) -> libc::c_int;
}
extern "C" {
    pub fn open(__file: *const libc::c_char, __oflag: libc::c_int, ...) -> libc::c_int;
}
extern "C-unwind" {
    pub fn open64(__file: *const libc::c_char, __oflag: libc::c_int, ...) -> libc::c_int;
}
extern "C-unwind" {
    pub fn openat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn openat64(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __oflag: libc::c_int,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn creat(__file: *const libc::c_char, __mode: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn creat64(__file: *const libc::c_char, __mode: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn posix_fadvise(
        __fd: libc::c_int,
        __offset: libc::c_long,
        __len: libc::c_long,
        __advise: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn posix_fadvise64(
        __fd: libc::c_int,
        __offset: libc::c_long,
        __len: libc::c_long,
        __advise: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn posix_fallocate(
        __fd: libc::c_int,
        __offset: libc::c_long,
        __len: libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn posix_fallocate64(
        __fd: libc::c_int,
        __offset: libc::c_long,
        __len: libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn imaxabs(__n: libc::c_long) -> libc::c_long;
}
extern "C-unwind" {
    pub fn imaxdiv(__numer: libc::c_long, __denom: libc::c_long) -> imaxdiv_t;
}
extern "C-unwind" {
    pub fn strtoimax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn strtoumax(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcstoimax(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn wcstoumax(
        __nptr: *const libc::wchar_t,
        __endptr: *mut *mut libc::wchar_t,
        __base: libc::c_int,
    ) -> libc::c_ulong;
}
extern "C" {
    pub static mut stdin: *mut _IO_FILE;
}
extern "C" {
    pub static mut stdout: *mut _IO_FILE;
}
extern "C" {
    pub static mut stderr: *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn remove(__filename: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn renameat(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn renameat2(
        __oldfd: libc::c_int,
        __old: *const libc::c_char,
        __newfd: libc::c_int,
        __new: *const libc::c_char,
        __flags: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fclose(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn tmpfile() -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn tmpfile64() -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn tmpnam(_anon_0: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn tmpnam_r(__s: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn tempnam(
        __dir: *const libc::c_char,
        __pfx: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn fflush(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fflush_unlocked(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fcloseall() -> libc::c_int;
}
extern "C-unwind" {
    pub fn fopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
    ) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut _IO_FILE,
    ) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn fopen64(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
    ) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn freopen64(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut _IO_FILE,
    ) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn fopencookie(
        __magic_cookie: *mut libc::c_void,
        __modes: *const libc::c_char,
        __io_funcs: _IO_cookie_io_functions_t,
    ) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn fmemopen(
        __s: *mut libc::c_void,
        __len: libc::c_ulong,
        __modes: *const libc::c_char,
    ) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn open_memstream(
        __bufloc: *mut *mut libc::c_char,
        __sizeloc: *mut libc::c_ulong,
    ) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn setbuf(__stream: *mut _IO_FILE, __buf: *mut libc::c_char);
}
extern "C-unwind" {
    pub fn setvbuf(
        __stream: *mut _IO_FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setbuffer(
        __stream: *mut _IO_FILE,
        __buf: *mut libc::c_char,
        __size: libc::c_ulong,
    );
}
extern "C-unwind" {
    pub fn setlinebuf(__stream: *mut _IO_FILE);
}
extern "C-unwind" {
    pub fn fprintf(
        __stream: *mut _IO_FILE,
        __format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn printf(__format: *const libc::c_char, ...) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sprintf(
        __s: *mut libc::c_char,
        __format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vfprintf(
        __s: *mut _IO_FILE,
        __format: *const libc::c_char,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vprintf(
        __fmt: *const libc::c_char,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vsprintf(
        __s: *mut libc::c_char,
        __format: *const libc::c_char,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn snprintf(
        __s: *mut libc::c_char,
        __maxlen: libc::c_ulong,
        __format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vsnprintf(
        __s: *mut libc::c_char,
        __maxlen: libc::c_ulong,
        __format: *const libc::c_char,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vdprintf(
        __fd: libc::c_int,
        __fmt: *const libc::c_char,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, ...) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fscanf(
        __stream: *mut _IO_FILE,
        __format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn scanf(__format: *const libc::c_char, ...) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sscanf(
        __s: *const libc::c_char,
        __format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vfscanf(
        __s: *mut _IO_FILE,
        __format: *const libc::c_char,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vscanf(
        __format: *const libc::c_char,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn vsscanf(
        __s: *const libc::c_char,
        __format: *const libc::c_char,
        __arg: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fgetc(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getc(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getchar() -> libc::c_int;
}
extern "C-unwind" {
    pub fn getc_unlocked(__fp: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getchar_unlocked() -> libc::c_int;
}
extern "C-unwind" {
    pub fn fgetc_unlocked(__fp: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fputc(__c: libc::c_int, __stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn putc(__c: libc::c_int, __stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn putchar(__c: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fputc_unlocked(__c: libc::c_int, __stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn putc_unlocked(__c: libc::c_int, __stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn putchar_unlocked(__c: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getw(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn putw(__w: libc::c_int, __stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut _IO_FILE,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn fgets_unlocked(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut _IO_FILE,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut libc::c_ulong,
        __delimiter: libc::c_int,
        __stream: *mut _IO_FILE,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut libc::c_ulong,
        __delimiter: libc::c_int,
        __stream: *mut _IO_FILE,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn getline(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut libc::c_ulong,
        __stream: *mut _IO_FILE,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn fputs(__s: *const libc::c_char, __stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn puts(__s: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ungetc(__c: libc::c_int, __stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fread(
        __ptr: *mut libc::c_void,
        __size: libc::c_ulong,
        __n: libc::c_ulong,
        __stream: *mut _IO_FILE,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fwrite(
        __ptr: *const libc::c_void,
        __size: libc::c_ulong,
        __n: libc::c_ulong,
        __s: *mut _IO_FILE,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fputs_unlocked(
        __s: *const libc::c_char,
        __stream: *mut _IO_FILE,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: libc::c_ulong,
        __n: libc::c_ulong,
        __stream: *mut _IO_FILE,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fwrite_unlocked(
        __ptr: *const libc::c_void,
        __size: libc::c_ulong,
        __n: libc::c_ulong,
        __stream: *mut _IO_FILE,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fseek(
        __stream: *mut _IO_FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ftell(__stream: *mut _IO_FILE) -> libc::c_long;
}
extern "C-unwind" {
    pub fn rewind(__stream: *mut _IO_FILE);
}
extern "C-unwind" {
    pub fn fseeko(
        __stream: *mut _IO_FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ftello(__stream: *mut _IO_FILE) -> libc::c_long;
}
extern "C-unwind" {
    pub fn fgetpos(__stream: *mut _IO_FILE, __pos: *mut _G_fpos_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fsetpos(__stream: *mut _IO_FILE, __pos: *const _G_fpos_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fseeko64(
        __stream: *mut _IO_FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ftello64(__stream: *mut _IO_FILE) -> libc::c_long;
}
extern "C-unwind" {
    pub fn fgetpos64(__stream: *mut _IO_FILE, __pos: *mut _G_fpos64_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fsetpos64(__stream: *mut _IO_FILE, __pos: *const _G_fpos64_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clearerr(__stream: *mut _IO_FILE);
}
extern "C-unwind" {
    pub fn feof(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ferror(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clearerr_unlocked(__stream: *mut _IO_FILE);
}
extern "C-unwind" {
    pub fn feof_unlocked(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ferror_unlocked(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn perror(__s: *const libc::c_char);
}
extern "C-unwind" {
    pub fn fileno(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fileno_unlocked(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pclose(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn popen(
        __command: *const libc::c_char,
        __modes: *const libc::c_char,
    ) -> *mut _IO_FILE;
}
extern "C-unwind" {
    pub fn ctermid(__s: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn cuserid(__s: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn obstack_printf(
        __obstack: *mut obstack,
        __format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const libc::c_char,
        __args: ::core::ffi::VaList<'_>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn flockfile(__stream: *mut _IO_FILE);
}
extern "C-unwind" {
    pub fn ftrylockfile(__stream: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn funlockfile(__stream: *mut _IO_FILE);
}
extern "C-unwind" {
    pub fn __uflow(_anon_0: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __overflow(_anon_0: *mut _IO_FILE, _anon_1: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __sysconf(__name: libc::c_int) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __sysv_signal(
        __sig: libc::c_int,
        __handler: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
    ) -> Option<unsafe extern "C-unwind" fn(libc::c_int)>;
}
extern "C-unwind" {
    pub fn sysv_signal(
        __sig: libc::c_int,
        __handler: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
    ) -> Option<unsafe extern "C-unwind" fn(libc::c_int)>;
}
extern "C-unwind" {
    pub fn signal(
        __sig: libc::c_int,
        __handler: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
    ) -> Option<unsafe extern "C-unwind" fn(libc::c_int)>;
}
extern "C-unwind" {
    pub fn kill(__pid: libc::c_int, __sig: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn killpg(__pgrp: libc::c_int, __sig: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn raise(__sig: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ssignal(
        __sig: libc::c_int,
        __handler: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
    ) -> Option<unsafe extern "C-unwind" fn(libc::c_int)>;
}
extern "C-unwind" {
    pub fn gsignal(__sig: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn psignal(__sig: libc::c_int, __s: *const libc::c_char);
}
extern "C-unwind" {
    pub fn psiginfo(__pinfo: *const siginfo_t, __s: *const libc::c_char);
}
extern "C-unwind" {
    #[link_name = "__xpg_sigpause"]
    pub fn sigpause(__sig: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigblock(__mask: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigsetmask(__mask: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn siggetmask() -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigemptyset(__set: *mut __sigset_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigfillset(__set: *mut __sigset_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigaddset(__set: *mut __sigset_t, __signo: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigdelset(__set: *mut __sigset_t, __signo: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigismember(__set: *const __sigset_t, __signo: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigisemptyset(__set: *const __sigset_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigandset(
        __set: *mut __sigset_t,
        __left: *const __sigset_t,
        __right: *const __sigset_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigorset(
        __set: *mut __sigset_t,
        __left: *const __sigset_t,
        __right: *const __sigset_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigprocmask(
        __how: libc::c_int,
        __set: *const __sigset_t,
        __oset: *mut __sigset_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigsuspend(__set: *const __sigset_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigpending(__set: *mut __sigset_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigwait(__set: *const __sigset_t, __sig: *mut libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigwaitinfo(__set: *const __sigset_t, __info: *mut siginfo_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigtimedwait(
        __set: *const __sigset_t,
        __info: *mut siginfo_t,
        __timeout: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigqueue(
        __pid: libc::c_int,
        __sig: libc::c_int,
        __val: sigval,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigreturn(__scp: *mut sigcontext) -> libc::c_int;
}
extern "C-unwind" {
    pub fn siginterrupt(__sig: libc::c_int, __interrupt: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigaltstack(__ss: *const stack_t, __oss: *mut stack_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigstack(__ss: *mut sigstack, __oss: *mut sigstack) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sighold(__sig: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigrelse(__sig: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigignore(__sig: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sigset(
        __sig: libc::c_int,
        __disp: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
    ) -> Option<unsafe extern "C-unwind" fn(libc::c_int)>;
}
extern "C-unwind" {
    pub fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_kill(__threadid: libc::c_ulong, __signo: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_sigqueue(
        __threadid: libc::c_ulong,
        __signo: libc::c_int,
        __value: sigval,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __libc_current_sigrtmin() -> libc::c_int;
}
extern "C-unwind" {
    pub fn __libc_current_sigrtmax() -> libc::c_int;
}
extern "C-unwind" {
    pub fn tgkill(
        __tgid: libc::c_int,
        __tid: libc::c_int,
        __signal: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getcontext(__ucp: *mut ucontext_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setcontext(__ucp: *const ucontext_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn swapcontext(__oucp: *mut ucontext_t, __ucp: *const ucontext_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn makecontext(
        __ucp: *mut ucontext_t,
        __func: Option<unsafe extern "C-unwind" fn()>,
        __argc: libc::c_int,
        ...
    );
}
extern "C-unwind" {
    pub fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
extern "C-unwind" {
    pub fn settimeofday(__tv: *const timeval, __tz: *const timezone) -> libc::c_int;
}
extern "C-unwind" {
    pub fn adjtime(__delta: *const timeval, __olddelta: *mut timeval) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getitimer(__which: libc::c_int, __value: *mut itimerval) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setitimer(
        __which: libc::c_int,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn utimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
}
extern "C-unwind" {
    pub fn lutimes(__file: *const libc::c_char, __tvp: *const timeval) -> libc::c_int;
}
extern "C-unwind" {
    pub fn futimes(__fd: libc::c_int, __tvp: *const timeval) -> libc::c_int;
}
extern "C-unwind" {
    pub fn futimesat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __tvp: *const timeval,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn prlimit(
        __pid: libc::c_int,
        __resource: libc::c_uint,
        __new_limit: *const rlimit,
        __old_limit: *mut rlimit,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn prlimit64(
        __pid: libc::c_int,
        __resource: libc::c_uint,
        __new_limit: *const rlimit64,
        __old_limit: *mut rlimit64,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getrlimit(__resource: libc::c_int, __rlimits: *mut rlimit) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getrlimit64(__resource: libc::c_int, __rlimits: *mut rlimit64) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setrlimit(__resource: libc::c_int, __rlimits: *const rlimit) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setrlimit64(
        __resource: libc::c_int,
        __rlimits: *const rlimit64,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getrusage(__who: libc::c_int, __usage: *mut rusage) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpriority(__which: libc::c_int, __who: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpriority(
        __which: libc::c_int,
        __who: libc::c_uint,
        __prio: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wait(__stat_loc: *mut libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn waitpid(
        __pid: libc::c_int,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn waitid(
        __idtype: libc::c_uint,
        __id: libc::c_uint,
        __infop: *mut siginfo_t,
        __options: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wait3(
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
        __usage: *mut rusage,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wait4(
        __pid: libc::c_int,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
        __usage: *mut rusage,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn alloca(__size: libc::c_ulong) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn __ctype_get_mb_cur_max() -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn atol(__nptr: *const libc::c_char) -> libc::c_long;
}
extern "C-unwind" {
    pub fn atoll(__nptr: *const libc::c_char) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn strtod(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn strtof(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn strtold(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn strtof32(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn strtof64(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn strtof32x(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn strtof64x(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn strtoul(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strtoq(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn strtouq(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulonglong;
}
extern "C-unwind" {
    pub fn strtoll(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn strtoull(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_ulonglong;
}
extern "C-unwind" {
    pub fn strfromd(
        __dest: *mut libc::c_char,
        __size: libc::c_ulong,
        __format: *const libc::c_char,
        __f: libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strfromf(
        __dest: *mut libc::c_char,
        __size: libc::c_ulong,
        __format: *const libc::c_char,
        __f: libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strfroml(
        __dest: *mut libc::c_char,
        __size: libc::c_ulong,
        __format: *const libc::c_char,
        __f: crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strfromf32(
        __dest: *mut libc::c_char,
        __size: libc::c_ulong,
        __format: *const libc::c_char,
        __f: libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strfromf64(
        __dest: *mut libc::c_char,
        __size: libc::c_ulong,
        __format: *const libc::c_char,
        __f: libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strfromf32x(
        __dest: *mut libc::c_char,
        __size: libc::c_ulong,
        __format: *const libc::c_char,
        __f: libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strfromf64x(
        __dest: *mut libc::c_char,
        __size: libc::c_ulong,
        __format: *const libc::c_char,
        __f: crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn strtol_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __loc: *mut __locale_struct,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn strtoul_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __loc: *mut __locale_struct,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strtoll_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __loc: *mut __locale_struct,
    ) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn strtoull_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __loc: *mut __locale_struct,
    ) -> libc::c_ulonglong;
}
extern "C-unwind" {
    pub fn strtod_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: *mut __locale_struct,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn strtof_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: *mut __locale_struct,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn strtold_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: *mut __locale_struct,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn strtof32_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: *mut __locale_struct,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn strtof64_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: *mut __locale_struct,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn strtof32x_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: *mut __locale_struct,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn strtof64x_l(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __loc: *mut __locale_struct,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn l64a(__n: libc::c_long) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn a64l(__s: *const libc::c_char) -> libc::c_long;
}
extern "C-unwind" {
    pub fn random() -> libc::c_long;
}
extern "C-unwind" {
    pub fn srandom(__seed: libc::c_uint);
}
extern "C-unwind" {
    pub fn initstate(
        __seed: libc::c_uint,
        __statebuf: *mut libc::c_char,
        __statelen: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn setstate(__statebuf: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn random_r(__buf: *mut random_data, __result: *mut libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn srandom_r(__seed: libc::c_uint, __buf: *mut random_data) -> libc::c_int;
}
extern "C-unwind" {
    pub fn initstate_r(
        __seed: libc::c_uint,
        __statebuf: *mut libc::c_char,
        __statelen: libc::c_ulong,
        __buf: *mut random_data,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setstate_r(
        __statebuf: *mut libc::c_char,
        __buf: *mut random_data,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn rand() -> libc::c_int;
}
extern "C-unwind" {
    pub fn srand(__seed: libc::c_uint);
}
extern "C-unwind" {
    pub fn rand_r(__seed: *mut libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn drand48() -> libc::c_double;
}
extern "C-unwind" {
    pub fn erand48(__xsubi: *mut libc::c_ushort) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lrand48() -> libc::c_long;
}
extern "C-unwind" {
    pub fn nrand48(__xsubi: *mut libc::c_ushort) -> libc::c_long;
}
extern "C-unwind" {
    pub fn mrand48() -> libc::c_long;
}
extern "C-unwind" {
    pub fn jrand48(__xsubi: *mut libc::c_ushort) -> libc::c_long;
}
extern "C-unwind" {
    pub fn srand48(__seedval: libc::c_long);
}
extern "C-unwind" {
    pub fn seed48(__seed16v: *mut libc::c_ushort) -> *mut libc::c_ushort;
}
extern "C-unwind" {
    pub fn lcong48(__param: *mut libc::c_ushort);
}
extern "C-unwind" {
    pub fn drand48_r(
        __buffer: *mut drand48_data,
        __result: *mut libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn erand48_r(
        __xsubi: *mut libc::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn nrand48_r(
        __xsubi: *mut libc::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn jrand48_r(
        __xsubi: *mut libc::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut libc::c_long,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn srand48_r(
        __seedval: libc::c_long,
        __buffer: *mut drand48_data,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn seed48_r(
        __seed16v: *mut libc::c_ushort,
        __buffer: *mut drand48_data,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn lcong48_r(
        __param: *mut libc::c_ushort,
        __buffer: *mut drand48_data,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn arc4random() -> libc::c_uint;
}
extern "C-unwind" {
    pub fn arc4random_buf(__buf: *mut libc::c_void, __size: libc::c_ulong);
}
extern "C-unwind" {
    pub fn arc4random_uniform(__upper_bound: libc::c_uint) -> libc::c_uint;
}
#[inline]
pub unsafe extern "C-unwind" fn malloc(__size: libc::c_ulong) -> *mut libc::c_void {
    (libc::malloc(__size as usize)) as *mut libc::c_void
}
#[inline]
pub unsafe extern "C-unwind" fn calloc(
    __nmemb: libc::c_ulong,
    __size: libc::c_ulong,
) -> *mut libc::c_void {
    (libc::calloc(__nmemb as usize, __size as usize)) as *mut libc::c_void
}
#[inline]
pub unsafe extern "C-unwind" fn realloc(
    __ptr: *mut libc::c_void,
    __size: libc::c_ulong,
) -> *mut libc::c_void {
    (libc::realloc(__ptr as *mut libc::c_void, __size as usize)) as *mut libc::c_void
}
#[inline]
pub unsafe extern "C-unwind" fn free(__ptr: *mut libc::c_void) {
    libc::free(__ptr as *mut libc::c_void);
}
extern "C-unwind" {
    pub fn reallocarray(
        __ptr: *mut libc::c_void,
        __nmemb: libc::c_ulong,
        __size: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn valloc(__size: libc::c_ulong) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn posix_memalign(
        __memptr: *mut *mut libc::c_void,
        __alignment: libc::c_ulong,
        __size: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn aligned_alloc(
        __alignment: libc::c_ulong,
        __size: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn abort();
}
extern "C-unwind" {
    pub fn atexit(__func: Option<unsafe extern "C-unwind" fn()>) -> libc::c_int;
}
extern "C-unwind" {
    pub fn at_quick_exit(__func: Option<unsafe extern "C-unwind" fn()>) -> libc::c_int;
}
extern "C-unwind" {
    pub fn on_exit(
        __func: Option<unsafe extern "C-unwind" fn(libc::c_int, *mut libc::c_void)>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
}
#[inline]
pub unsafe extern "C-unwind" fn exit(__status: libc::c_int) {
    libc::exit(__status as libc::c_int);
}
extern "C-unwind" {
    pub fn quick_exit(__status: libc::c_int);
}
extern "C-unwind" {
    pub fn _Exit(__status: libc::c_int);
}
extern "C-unwind" {
    pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn secure_getenv(__name: *const libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn putenv(__string: *mut libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clearenv() -> libc::c_int;
}
extern "C-unwind" {
    pub fn mktemp(__template: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkstemp64(__template: *mut libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkstemps(
        __template: *mut libc::c_char,
        __suffixlen: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkstemps64(
        __template: *mut libc::c_char,
        __suffixlen: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkdtemp(__template: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn mkostemp(__template: *mut libc::c_char, __flags: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkostemp64(
        __template: *mut libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkostemps(
        __template: *mut libc::c_char,
        __suffixlen: libc::c_int,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkostemps64(
        __template: *mut libc::c_char,
        __suffixlen: libc::c_int,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn system(__command: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn canonicalize_file_name(__name: *const libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn bsearch(
        __key: *const libc::c_void,
        __base: *const libc::c_void,
        __nmemb: libc::c_ulong,
        __size: libc::c_ulong,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn qsort(
        __base: *mut libc::c_void,
        __nmemb: libc::c_ulong,
        __size: libc::c_ulong,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
    );
}
extern "C-unwind" {
    pub fn qsort_r(
        __base: *mut libc::c_void,
        __nmemb: libc::c_ulong,
        __size: libc::c_ulong,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const libc::c_void,
                *const libc::c_void,
                *mut libc::c_void,
            ) -> libc::c_int,
        >,
        __arg: *mut libc::c_void,
    );
}
extern "C-unwind" {
    pub fn abs(__x: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn labs(__x: libc::c_long) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llabs(__x: libc::c_longlong) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn div(__numer: libc::c_int, __denom: libc::c_int) -> div_t;
}
extern "C-unwind" {
    pub fn ldiv(__numer: libc::c_long, __denom: libc::c_long) -> ldiv_t;
}
extern "C-unwind" {
    pub fn lldiv(__numer: libc::c_longlong, __denom: libc::c_longlong) -> lldiv_t;
}
extern "C-unwind" {
    pub fn ecvt(
        __value: libc::c_double,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn fcvt(
        __value: libc::c_double,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn gcvt(
        __value: libc::c_double,
        __ndigit: libc::c_int,
        __buf: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn qecvt(
        __value: crate::__f80::F80,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn qfcvt(
        __value: crate::__f80::F80,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn qgcvt(
        __value: crate::__f80::F80,
        __ndigit: libc::c_int,
        __buf: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn ecvt_r(
        __value: libc::c_double,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
        __buf: *mut libc::c_char,
        __len: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fcvt_r(
        __value: libc::c_double,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
        __buf: *mut libc::c_char,
        __len: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn qecvt_r(
        __value: crate::__f80::F80,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
        __buf: *mut libc::c_char,
        __len: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn qfcvt_r(
        __value: crate::__f80::F80,
        __ndigit: libc::c_int,
        __decpt: *mut libc::c_int,
        __sign: *mut libc::c_int,
        __buf: *mut libc::c_char,
        __len: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mblen(__s: *const libc::c_char, __n: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mbtowc(
        __pwc: *mut libc::wchar_t,
        __s: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn wctomb(__s: *mut libc::c_char, __wchar: libc::wchar_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mbstowcs(
        __pwcs: *mut libc::wchar_t,
        __s: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn wcstombs(
        __s: *mut libc::c_char,
        __pwcs: *const libc::wchar_t,
        __n: libc::c_ulong,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn rpmatch(__response: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getsubopt(
        __optionp: *mut *mut libc::c_char,
        __tokens: *const *mut libc::c_char,
        __valuep: *mut *mut libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn posix_openpt(__oflag: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn grantpt(__fd: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn unlockpt(__fd: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ptsname(__fd: libc::c_int) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn ptsname_r(
        __fd: libc::c_int,
        __buf: *mut libc::c_char,
        __buflen: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpt() -> libc::c_int;
}
extern "C-unwind" {
    pub fn getloadavg(
        __loadavg: *mut libc::c_double,
        __nelem: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn readv(
        __fd: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn writev(
        __fd: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn preadv(
        __fd: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
        __offset: libc::c_long,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn pwritev(
        __fd: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
        __offset: libc::c_long,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn preadv64(
        __fd: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
        __offset: libc::c_long,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn pwritev64(
        __fd: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
        __offset: libc::c_long,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn preadv2(
        __fp: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
        __offset: libc::c_long,
        ___flags: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn pwritev2(
        __fd: libc::c_int,
        __iodev: *const iovec,
        __count: libc::c_int,
        __offset: libc::c_long,
        __flags: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn preadv64v2(
        __fp: libc::c_int,
        __iovec: *const iovec,
        __count: libc::c_int,
        __offset: libc::c_long,
        ___flags: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn pwritev64v2(
        __fd: libc::c_int,
        __iodev: *const iovec,
        __count: libc::c_int,
        __offset: libc::c_long,
        __flags: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn process_vm_readv(
        __pid: libc::c_int,
        __lvec: *const iovec,
        __liovcnt: libc::c_ulong,
        __rvec: *const iovec,
        __riovcnt: libc::c_ulong,
        __flags: libc::c_ulong,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn process_vm_writev(
        __pid: libc::c_int,
        __lvec: *const iovec,
        __liovcnt: libc::c_ulong,
        __rvec: *const iovec,
        __riovcnt: libc::c_ulong,
        __flags: libc::c_ulong,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;
}
extern "C-unwind" {
    pub fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn socketpair(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
        __fds: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn bind(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpeername(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: libc::c_ulong,
        __flags: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: libc::c_ulong,
        __flags: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: libc::c_ulong,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: libc::c_ulong,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn sendmsg(
        __fd: libc::c_int,
        __message: *const msghdr,
        __flags: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn sendmmsg(
        __fd: libc::c_int,
        __vmessages: *mut mmsghdr,
        __vlen: libc::c_uint,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn recvmsg(
        __fd: libc::c_int,
        __message: *mut msghdr,
        __flags: libc::c_int,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn recvmmsg(
        __fd: libc::c_int,
        __vmessages: *mut mmsghdr,
        __vlen: libc::c_uint,
        __flags: libc::c_int,
        __tmo: *mut timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn accept4(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut libc::c_uint,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sockatmark(__fd: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isfdtype(__fd: libc::c_int, __fdtype: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub static mut in6addr_any: in6_addr;
}
extern "C" {
    pub static mut in6addr_loopback: in6_addr;
}
extern "C-unwind" {
    pub fn ntohl(__netlong: libc::c_uint) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn ntohs(__netshort: libc::c_ushort) -> libc::c_ushort;
}
extern "C-unwind" {
    pub fn htonl(__hostlong: libc::c_uint) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn htons(__hostshort: libc::c_ushort) -> libc::c_ushort;
}
extern "C-unwind" {
    pub fn bindresvport(
        __sockfd: libc::c_int,
        __sock_in: *mut sockaddr_in,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn bindresvport6(
        __sockfd: libc::c_int,
        __sock_in: *mut sockaddr_in6,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_option_space(__nbytes: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_option_init(
        __bp: *mut libc::c_void,
        __cmsgp: *mut *mut cmsghdr,
        __type: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_option_append(
        __cmsg: *mut cmsghdr,
        __typep: *const libc::c_uchar,
        __multx: libc::c_int,
        __plusy: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_option_alloc(
        __cmsg: *mut cmsghdr,
        __datalen: libc::c_int,
        __multx: libc::c_int,
        __plusy: libc::c_int,
    ) -> *mut libc::c_uchar;
}
extern "C-unwind" {
    pub fn inet6_option_next(
        __cmsg: *const cmsghdr,
        __tptrp: *mut *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_option_find(
        __cmsg: *const cmsghdr,
        __tptrp: *mut *mut libc::c_uchar,
        __type: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_opt_init(
        __extbuf: *mut libc::c_void,
        __extlen: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_opt_append(
        __extbuf: *mut libc::c_void,
        __extlen: libc::c_uint,
        __offset: libc::c_int,
        __type: libc::c_uchar,
        __len: libc::c_uint,
        __align: libc::c_uchar,
        __databufp: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_opt_finish(
        __extbuf: *mut libc::c_void,
        __extlen: libc::c_uint,
        __offset: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_opt_set_val(
        __databuf: *mut libc::c_void,
        __offset: libc::c_int,
        __val: *mut libc::c_void,
        __vallen: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_opt_next(
        __extbuf: *mut libc::c_void,
        __extlen: libc::c_uint,
        __offset: libc::c_int,
        __typep: *mut libc::c_uchar,
        __lenp: *mut libc::c_uint,
        __databufp: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_opt_find(
        __extbuf: *mut libc::c_void,
        __extlen: libc::c_uint,
        __offset: libc::c_int,
        __type: libc::c_uchar,
        __lenp: *mut libc::c_uint,
        __databufp: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_opt_get_val(
        __databuf: *mut libc::c_void,
        __offset: libc::c_int,
        __val: *mut libc::c_void,
        __vallen: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_rth_space(__type: libc::c_int, __segments: libc::c_int) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn inet6_rth_init(
        __bp: *mut libc::c_void,
        __bp_len: libc::c_uint,
        __type: libc::c_int,
        __segments: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn inet6_rth_add(
        __bp: *mut libc::c_void,
        __addr: *const in6_addr,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_rth_reverse(
        __in: *const libc::c_void,
        __out: *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_rth_segments(__bp: *const libc::c_void) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet6_rth_getaddr(
        __bp: *const libc::c_void,
        __index: libc::c_int,
    ) -> *mut in6_addr;
}
extern "C-unwind" {
    pub fn getipv4sourcefilter(
        __s: libc::c_int,
        __interface_addr: in_addr,
        __group: in_addr,
        __fmode: *mut libc::c_uint,
        __numsrc: *mut libc::c_uint,
        __slist: *mut in_addr,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setipv4sourcefilter(
        __s: libc::c_int,
        __interface_addr: in_addr,
        __group: in_addr,
        __fmode: libc::c_uint,
        __numsrc: libc::c_uint,
        __slist: *const in_addr,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getsourcefilter(
        __s: libc::c_int,
        __interface_addr: libc::c_uint,
        __group: *const sockaddr,
        __grouplen: libc::c_uint,
        __fmode: *mut libc::c_uint,
        __numsrc: *mut libc::c_uint,
        __slist: *mut sockaddr_storage,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setsourcefilter(
        __s: libc::c_int,
        __interface_addr: libc::c_uint,
        __group: *const sockaddr,
        __grouplen: libc::c_uint,
        __fmode: libc::c_uint,
        __numsrc: libc::c_uint,
        __slist: *const sockaddr_storage,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet_addr(__cp: *const libc::c_char) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn inet_lnaof(__in: in_addr) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn inet_makeaddr(__net: libc::c_uint, __host: libc::c_uint) -> in_addr;
}
extern "C-unwind" {
    pub fn inet_netof(__in: in_addr) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn inet_network(__cp: *const libc::c_char) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: libc::c_uint,
    ) -> *const libc::c_char;
}
extern "C-unwind" {
    pub fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet_neta(
        __net: libc::c_uint,
        __buf: *mut libc::c_char,
        __len: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn inet_net_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __bits: libc::c_int,
        __buf: *mut libc::c_char,
        __len: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn inet_net_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
        __len: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn inet_nsap_addr(
        __cp: *const libc::c_char,
        __buf: *mut libc::c_uchar,
        __len: libc::c_int,
    ) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn inet_nsap_ntoa(
        __len: libc::c_int,
        __cp: *const libc::c_uchar,
        __buf: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    #[link_name = "_Z12__ace_assertPKciS0_"]
    pub fn __ace_assert(
        file: *const libc::c_char,
        line: libc::c_int,
        expression: *const libc::c_char,
    );
}
extern "C-unwind" {
    pub fn __errno_location() -> *mut libc::c_int;
}
extern "C" {
    pub static mut program_invocation_name: *mut libc::c_char;
}
extern "C" {
    pub static mut program_invocation_short_name: *mut libc::c_char;
}
pub mod ACE_OS {
    pub use crate::full_ops_0::ACE_OS::last_error;
    pub use crate::full_ops_0::ACE_OS::last_error_u899090ac2dc66d1c;
    pub use crate::full_ops_0::ACE_OS::set_errno_to_last_error;
    pub use crate::full_ops_0::ACE_OS::set_errno_to_wsa_last_error;
    pub use crate::full_ops_0::ACE_OS::_exit_u5699558a229ba490;
    pub use crate::full_ops_0::ACE_OS::abort_uedf8dd117d7d25b9;
    pub use crate::full_ops_0::ACE_OS::atexit_u7798e3a0f6f567d9;
    pub use crate::full_ops_0::ACE_OS::atoi_ud83f836c714024f7;
    pub use crate::full_ops_0::ACE_OS::atoi_ud8167b6c711d124b;
    pub use crate::full_ops_0::ACE_OS::atol_ub19fd7dbebf787be;
    pub use crate::full_ops_0::ACE_OS::atol_ub1774fdbebd54e92;
    pub use crate::full_ops_0::ACE_OS::atof_ube8bc55909d18178;
    pub use crate::full_ops_0::ACE_OS::atof_ubecfbd590a0b40ac;
    pub use crate::full_ops_0::ACE_OS::atop;
    pub use crate::full_ops_0::ACE_OS::atop_u8b8dab5c8edb2a46;
    pub use crate::full_ops_0::ACE_OS::bsearch_u2b4191d4f3f8dfb7;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6callocEmm"]
        pub fn calloc_uda0c707404e15bad(
            elements: libc::c_ulong,
            sizeof_elements: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS4exitEi"]
        pub fn exit_ud318a3a23e137d2b(status: libc::c_int);
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS4freeEPv"]
        pub fn free_ucbe6910a56eae126(_anon_0: *mut libc::c_void);
    }
    pub use crate::full_ops_0::ACE_OS::getenv_u25dce132a786d601;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS13getenvstringsEv"]
        pub fn getenvstrings() -> *mut libc::c_char;
    }
    pub use crate::full_ops_0::ACE_OS::itoa;
    pub use crate::full_ops_0::ACE_OS::itoa_uae23ea8639966224;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS14itoa_emulationEiPci"]
        pub fn itoa_emulation(
            value: libc::c_int,
            string: *mut libc::c_char,
            radix: libc::c_int,
        ) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS14itow_emulationEiPwi"]
        pub fn itow_emulation(
            value: libc::c_int,
            string: *mut libc::wchar_t,
            radix: libc::c_int,
        ) -> *mut libc::wchar_t;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6mallocEm"]
        pub fn malloc_u80646b42522769ca(_anon_0: libc::c_ulong) -> *mut libc::c_void;
    }
    pub use crate::full_ops_0::ACE_OS::mkstemp_uab24266fd9200022;
    pub use crate::full_ops_0::ACE_OS::mkstemp_uaae02e6fd8e640ee;
    pub use crate::full_ops_0::ACE_OS::mktemp_ufd8deab57221e9e3;
    pub use crate::full_ops_0::ACE_OS::mktemp_ufdb6f2b57244fc8f;
    pub use crate::full_ops_0::ACE_OS::putenv_u2bcc4efea7b4aaa8;
    pub use crate::full_ops_0::ACE_OS::qsort_u2d7063cc71f51e00;
    pub use crate::full_ops_0::ACE_OS::setenv_u73cbf70eb60d05c0;
    pub use crate::full_ops_0::ACE_OS::unsetenv_u4a861fb4bcaedd18;
    pub use crate::full_ops_0::ACE_OS::rand_uf8dfc603800d7fea;
    pub use crate::full_ops_0::ACE_OS::rand_r_ucadeff1605579395;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7reallocEPvm"]
        pub fn realloc_u7a2f3395e781bb73(
            _anon_0: *mut libc::c_void,
            _anon_1: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
    pub use crate::full_ops_0::ACE_OS::realpath_u67561b6743ec0a17;
    pub use crate::full_ops_0::ACE_OS::realpath_ud1e77b53d7fbbe7f;
    extern "C" {
        pub static mut exit_hook_: Option<unsafe extern "C-unwind" fn()>;
    }
    pub use crate::full_ops_0::ACE_OS::set_exit_hook;
    pub use crate::full_ops_0::ACE_OS::srand_u295a01f6abd1c409;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS9strenvdupEPKc"]
        pub fn strenvdup_uc4fed4dc8e89ae4d(
            str: *const libc::c_char,
        ) -> *mut libc::c_char;
    }
    pub use crate::full_ops_0::ACE_OS::strtod_uda01274f56fb1634;
    pub use crate::full_ops_0::ACE_OS::strtod_u08f644783b562e84;
    pub use crate::full_ops_0::ACE_OS::strtol_u0d03b0a19d8ced84;
    pub use crate::full_ops_0::ACE_OS::strtol_uf26c78e4f4a19c34;
    pub use crate::full_ops_0::ACE_OS::strtoul_u8ee5025855dd0107;
    pub use crate::full_ops_0::ACE_OS::strtoul_u94c4704ba2a180f7;
    pub use crate::full_ops_0::ACE_OS::strtoll_ud3f371a6264a7b10;
    pub use crate::full_ops_0::ACE_OS::strtoll_uc9864eaf468b57c0;
    pub use crate::full_ops_0::ACE_OS::strtoull_u8c78de8984ea8f1d;
    pub use crate::full_ops_0::ACE_OS::strtoull_u73b904db009b8305;
    pub use crate::full_ops_0::ACE_OS::system_udf98ef8d850ee367;
    pub use crate::full_ops_0::ACE_OS::getprogname;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS21getprogname_emulationEv"]
        pub fn getprogname_emulation() -> *const libc::c_char;
    }
    pub use crate::full_ops_0::ACE_OS::setprogname;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS21setprogname_emulationEPKc"]
        pub fn setprogname_emulation(name: *const libc::c_char);
    }
    pub use crate::full_ops_0::ACE_OS::memchr_ua033039165df0e60;
    pub use crate::full_ops_0::ACE_OS::memchr_u7f5024e48509190d;
    pub use crate::full_ops_0::ACE_OS::memcmp_u2459c16080f3a32c;
    pub use crate::full_ops_0::ACE_OS::memcpy_u6033eb81edaf9212;
    pub use crate::full_ops_0::ACE_OS::memmove_u5dc7ae11e120ed2b;
    pub use crate::full_ops_0::ACE_OS::memset_u2b5dfc47d301370a;
    pub use crate::full_ops_0::ACE_OS::strcat_u59e0931d4af5fb3f;
    pub use crate::full_ops_0::ACE_OS::strcat_uf59e07217ba77b37;
    pub use crate::full_ops_0::ACE_OS::strchr_ue2d436a738f8836a;
    pub use crate::full_ops_0::ACE_OS::strchr_u60d85f4df3d6164c;
    pub use crate::full_ops_0::ACE_OS::strchr_u824406bee5e3796b;
    pub use crate::full_ops_0::ACE_OS::strchr_u018eb765a1d6b605;
    pub use crate::full_ops_0::ACE_OS::strcmp_u2f671283fc8b6d4a;
    pub use crate::full_ops_0::ACE_OS::strcmp_u5a69aa47cfc1401e;
    pub use crate::full_ops_0::ACE_OS::strcpy_u08e8184bcebbac89;
    pub use crate::full_ops_0::ACE_OS::strcpy_u7344f334f51e13d1;
    pub use crate::full_ops_0::ACE_OS::strcspn_u810d935abf74b6b8;
    pub use crate::full_ops_0::ACE_OS::strcspn_ud9a5aa15081c942c;
    pub use crate::full_ops_0::ACE_OS::strdup_u8b6a84c4b070659c;
    pub use crate::full_ops_0::ACE_OS::strdup_u8b930cc4b0929ec8;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS16strdup_emulationEPKw"]
        pub fn strdup_emulation(s: *const libc::wchar_t) -> *mut libc::wchar_t;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7strecpyEPcPKc"]
        pub fn strecpy(
            des: *mut libc::c_char,
            src: *const libc::c_char,
        ) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7strecpyEPwPKw"]
        pub fn strecpy_u35fead064ff3d60e(
            s: *mut libc::wchar_t,
            t: *const libc::wchar_t,
        ) -> *mut libc::wchar_t;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS8strerrorEi"]
        pub fn strerror_uba489202eb01d03e(errnum: libc::c_int) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS9strsignalEi"]
        pub fn strsignal_u43542216f9e86286(signum: libc::c_int) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS10strerror_rEiPcm"]
        pub fn strerror_r_u01ae71f59759ac2a(
            errnum: libc::c_int,
            buf: *mut libc::c_char,
            buflen: libc::c_ulong,
        ) -> *mut libc::c_char;
    }
    pub use crate::full_ops_0::ACE_OS::strlen_u07dd12a225364fa6;
    pub use crate::full_ops_0::ACE_OS::strlen_u07b44aa22513a9ba;
    pub use crate::full_ops_0::ACE_OS::strncat_u4ca56e9e01c42c12;
    pub use crate::full_ops_0::ACE_OS::strncat_u63ded26dcfa8694a;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7strnchrEPKcim"]
        pub fn strnchr(
            s: *const libc::c_char,
            c: libc::c_int,
            len: libc::c_ulong,
        ) -> *const libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7strnchrEPKwwm"]
        pub fn strnchr_u49bc8e7dd2ed2fed(
            s: *const libc::wchar_t,
            c: libc::wchar_t,
            len: libc::c_ulong,
        ) -> *const libc::wchar_t;
    }
    pub use crate::full_ops_0::ACE_OS::strnchr_uc374d3979b1df78e;
    pub use crate::full_ops_0::ACE_OS::strnchr_ueaefb033287194b4;
    pub use crate::full_ops_0::ACE_OS::strncmp_u806dcd856f7e4e27;
    pub use crate::full_ops_0::ACE_OS::strncmp_u2c46a6b8f3ce5afb;
    pub use crate::full_ops_0::ACE_OS::strncpy_u11a5be0fa5efbef0;
    pub use crate::full_ops_0::ACE_OS::strncpy_u215a910ecc187698;
    pub use crate::full_ops_0::ACE_OS::strnlen_u5f0e7e21e0d61283;
    pub use crate::full_ops_0::ACE_OS::strnlen_udde13d7b2348dbf7;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7strnstrEPKcS1_m"]
        pub fn strnstr(
            s: *const libc::c_char,
            t: *const libc::c_char,
            len: libc::c_ulong,
        ) -> *const libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7strnstrEPKwS1_m"]
        pub fn strnstr_uc392ec1665afec52(
            s: *const libc::wchar_t,
            t: *const libc::wchar_t,
            len: libc::c_ulong,
        ) -> *const libc::wchar_t;
    }
    pub use crate::full_ops_0::ACE_OS::strnstr_uffa70d71e28ef7c5;
    pub use crate::full_ops_0::ACE_OS::strnstr_u0ac0d43267717395;
    pub use crate::full_ops_0::ACE_OS::strpbrk_ue4f2715f2491638f;
    pub use crate::full_ops_0::ACE_OS::strpbrk_u66360deffe3bbc5b;
    pub use crate::full_ops_0::ACE_OS::strpbrk_ue40b1c01b34f3230;
    pub use crate::full_ops_0::ACE_OS::strpbrk_u4cf2488ec6c70f00;
    pub use crate::full_ops_0::ACE_OS::strrchr_u93b8199dbba81372;
    pub use crate::full_ops_0::ACE_OS::strrchr_u1558e24479976ea4;
    pub use crate::full_ops_0::ACE_OS::strrchr_ud9ed4b42142d6283;
    pub use crate::full_ops_0::ACE_OS::strrchr_u84e41adf43f2e2ad;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS8strsncpyEPcPKcm"]
        pub fn strsncpy(
            dst: *mut libc::c_char,
            src: *const libc::c_char,
            maxlen: libc::c_ulong,
        ) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS8strsncpyEPwPKwm"]
        pub fn strsncpy_u8a4aa104d1b86cfb(
            dst: *mut libc::wchar_t,
            src: *const libc::wchar_t,
            maxlen: libc::c_ulong,
        ) -> *mut libc::wchar_t;
    }
    pub use crate::full_ops_0::ACE_OS::strspn_u08980472613e18bb;
    pub use crate::full_ops_0::ACE_OS::strspn_u3449e7df1505feef;
    pub use crate::full_ops_0::ACE_OS::strstr_u9abb16f83c23d2e3;
    pub use crate::full_ops_0::ACE_OS::strstr_uc6972ebc10126ff7;
    pub use crate::full_ops_0::ACE_OS::strstr_u25e13fe23ca4c804;
    pub use crate::full_ops_0::ACE_OS::strstr_ud415c256dd1973d4;
    pub use crate::full_ops_0::ACE_OS::strtok_u38e7db9f5ec8adc9;
    pub use crate::full_ops_0::ACE_OS::strtok_ua6aab688880e3e11;
    pub use crate::full_ops_0::ACE_OS::strtok_r_u995a2e1f2d66cef7;
    pub use crate::full_ops_0::ACE_OS::strtok_r_ube5e32bb1b203047;
    pub use crate::full_ops_0::ACE_OS::WChar;
    pub use crate::full_ops_0::ACE_OS::fgetwc_u0a7da72346087585;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS17wcsicmp_emulationEPKwS1_"]
        pub fn wcsicmp_emulation(
            string1: *const libc::wchar_t,
            string2: *const libc::wchar_t,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS18wcsnicmp_emulationEPKwS1_m"]
        pub fn wcsnicmp_emulation(
            string1: *const libc::wchar_t,
            string2: *const libc::wchar_t,
            len: libc::c_ulong,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::wslen;
    pub use crate::full_ops_0::ACE_OS::wscpy;
    pub use crate::full_ops_0::ACE_OS::wscmp;
    pub use crate::full_ops_0::ACE_OS::wsncmp;
    pub use crate::full_ops_0::ACE_OS::ungetwc_ufd62e29b47c8f27c;
}
pub mod __gnu_debug {}
extern "C-unwind" {
    pub fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn localeconv() -> *mut lconv;
}
extern "C-unwind" {
    pub fn newlocale(
        __category_mask: libc::c_int,
        __locale: *const libc::c_char,
        __base: *mut __locale_struct,
    ) -> *mut __locale_struct;
}
extern "C-unwind" {
    pub fn duplocale(__dataset: *mut __locale_struct) -> *mut __locale_struct;
}
extern "C-unwind" {
    pub fn freelocale(__dataset: *mut __locale_struct);
}
extern "C-unwind" {
    pub fn uselocale(__dataset: *mut __locale_struct) -> *mut __locale_struct;
}
extern "C-unwind" {
    pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
extern "C-unwind" {
    pub fn __ctype_tolower_loc() -> *mut *const libc::c_int;
}
extern "C-unwind" {
    pub fn __ctype_toupper_loc() -> *mut *const libc::c_int;
}
extern "C-unwind" {
    pub fn isalnum(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isalpha(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iscntrl(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isdigit(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn islower(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isgraph(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isprint(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ispunct(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isspace(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isupper(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isxdigit(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn tolower(__c: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn toupper(__c: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isblank(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isctype(__c: libc::c_int, __mask: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isascii(__c: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn toascii(__c: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn _toupper(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn _tolower(_anon_0: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isalnum_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isalpha_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn iscntrl_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isdigit_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn islower_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isgraph_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isprint_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ispunct_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isspace_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isupper_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isxdigit_l(
        _anon_0: libc::c_int,
        _anon_1: *mut __locale_struct,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn isblank_l(_anon_0: libc::c_int, _anon_1: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __tolower_l(__c: libc::c_int, __l: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn tolower_l(__c: libc::c_int, __l: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __toupper_l(__c: libc::c_int, __l: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn toupper_l(__c: libc::c_int, __l: *mut __locale_struct) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clone(
        __fn: Option<unsafe extern "C-unwind" fn(*mut libc::c_void) -> libc::c_int>,
        __child_stack: *mut libc::c_void,
        __flags: libc::c_int,
        __arg: *mut libc::c_void,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn unshare(__flags: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_getcpu() -> libc::c_int;
}
extern "C-unwind" {
    pub fn getcpu(_anon_0: *mut libc::c_uint, _anon_1: *mut libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setns(__fd: libc::c_int, __nstype: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __sched_cpucount(
        __setsize: libc::c_ulong,
        __setp: *const cpu_set_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __sched_cpualloc(__count: libc::c_ulong) -> *mut cpu_set_t;
}
extern "C-unwind" {
    pub fn __sched_cpufree(__set: *mut cpu_set_t);
}
extern "C-unwind" {
    pub fn sched_setparam(
        __pid: libc::c_int,
        __param: *const sched_param,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_getparam(__pid: libc::c_int, __param: *mut sched_param) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_setscheduler(
        __pid: libc::c_int,
        __policy: libc::c_int,
        __param: *const sched_param,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_getscheduler(__pid: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_yield() -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_get_priority_max(__algorithm: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_get_priority_min(__algorithm: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_rr_get_interval(__pid: libc::c_int, __t: *mut timespec) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_setaffinity(
        __pid: libc::c_int,
        __cpusetsize: libc::c_ulong,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sched_getaffinity(
        __pid: libc::c_int,
        __cpusetsize: libc::c_ulong,
        __cpuset: *mut cpu_set_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clock_adjtime(__clock_id: libc::c_int, __utx: *mut timex) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clock() -> libc::c_long;
}
extern "C-unwind" {
    pub fn time(__timer: *mut libc::c_long) -> libc::c_long;
}
extern "C-unwind" {
    pub fn difftime(__time1: libc::c_long, __time0: libc::c_long) -> libc::c_double;
}
extern "C-unwind" {
    pub fn mktime(__tp: *mut tm) -> libc::c_long;
}
extern "C-unwind" {
    pub fn strftime(
        __s: *mut libc::c_char,
        __maxsize: libc::c_ulong,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strptime(
        __s: *const libc::c_char,
        __fmt: *const libc::c_char,
        __tp: *mut tm,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn strftime_l(
        __s: *mut libc::c_char,
        __maxsize: libc::c_ulong,
        __format: *const libc::c_char,
        __tp: *const tm,
        __loc: *mut __locale_struct,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn strptime_l(
        __s: *const libc::c_char,
        __fmt: *const libc::c_char,
        __tp: *mut tm,
        __loc: *mut __locale_struct,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn gmtime(__timer: *const libc::c_long) -> *mut tm;
}
extern "C-unwind" {
    pub fn localtime(__timer: *const libc::c_long) -> *mut tm;
}
extern "C-unwind" {
    pub fn gmtime_r(__timer: *const libc::c_long, __tp: *mut tm) -> *mut tm;
}
extern "C-unwind" {
    pub fn localtime_r(__timer: *const libc::c_long, __tp: *mut tm) -> *mut tm;
}
extern "C-unwind" {
    pub fn asctime(__tp: *const tm) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn ctime(__timer: *const libc::c_long) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn asctime_r(__tp: *const tm, __buf: *mut libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn ctime_r(
        __timer: *const libc::c_long,
        __buf: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub static mut __tzname: [*mut libc::c_char; 2usize];
}
extern "C" {
    pub static mut __daylight: libc::c_int;
}
extern "C" {
    pub static mut __timezone: libc::c_long;
}
extern "C" {
    pub static mut tzname: [*mut libc::c_char; 2usize];
}
extern "C-unwind" {
    pub fn tzset();
}
extern "C" {
    pub static mut daylight: libc::c_int;
}
extern "C" {
    pub static mut timezone: libc::c_long;
}
extern "C-unwind" {
    pub fn timegm(__tp: *mut tm) -> libc::c_long;
}
extern "C-unwind" {
    pub fn timelocal(__tp: *mut tm) -> libc::c_long;
}
extern "C-unwind" {
    pub fn dysize(__year: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clock_getres(__clock_id: libc::c_int, __res: *mut timespec) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clock_gettime(__clock_id: libc::c_int, __tp: *mut timespec) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clock_settime(__clock_id: libc::c_int, __tp: *const timespec) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clock_nanosleep(
        __clock_id: libc::c_int,
        __flags: libc::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn clock_getcpuclockid(
        __pid: libc::c_int,
        __clock_id: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn timer_create(
        __clock_id: libc::c_int,
        __evp: *mut sigevent,
        __timerid: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn timer_delete(__timerid: *mut libc::c_void) -> libc::c_int;
}
extern "C-unwind" {
    pub fn timer_settime(
        __timerid: *mut libc::c_void,
        __flags: libc::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn timer_gettime(
        __timerid: *mut libc::c_void,
        __value: *mut itimerspec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn timer_getoverrun(__timerid: *mut libc::c_void) -> libc::c_int;
}
extern "C-unwind" {
    pub fn timespec_get(__ts: *mut timespec, __base: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn timespec_getres(__ts: *mut timespec, __base: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub static mut getdate_err: libc::c_int;
}
extern "C-unwind" {
    pub fn getdate(__string: *const libc::c_char) -> *mut tm;
}
extern "C-unwind" {
    pub fn getdate_r(__string: *const libc::c_char, __resbufp: *mut tm) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_create(
        __newthread: *mut libc::c_ulong,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_exit(__retval: *mut libc::c_void);
}
extern "C-unwind" {
    pub fn pthread_join(
        __th: libc::c_ulong,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_tryjoin_np(
        __th: libc::c_ulong,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_timedjoin_np(
        __th: libc::c_ulong,
        __thread_return: *mut *mut libc::c_void,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_clockjoin_np(
        __th: libc::c_ulong,
        __thread_return: *mut *mut libc::c_void,
        __clockid: libc::c_int,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_detach(__th: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_self() -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn pthread_equal(
        __thread1: libc::c_ulong,
        __thread2: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getdetachstate(
        __attr: *const pthread_attr_t,
        __detachstate: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getguardsize(
        __attr: *const pthread_attr_t,
        __guardsize: *mut libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setguardsize(
        __attr: *mut pthread_attr_t,
        __guardsize: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getschedparam(
        __attr: *const pthread_attr_t,
        __param: *mut sched_param,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setschedparam(
        __attr: *mut pthread_attr_t,
        __param: *const sched_param,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getschedpolicy(
        __attr: *const pthread_attr_t,
        __policy: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setschedpolicy(
        __attr: *mut pthread_attr_t,
        __policy: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getinheritsched(
        __attr: *const pthread_attr_t,
        __inherit: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setinheritsched(
        __attr: *mut pthread_attr_t,
        __inherit: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getscope(
        __attr: *const pthread_attr_t,
        __scope: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setscope(
        __attr: *mut pthread_attr_t,
        __scope: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getstackaddr(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setstackaddr(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getstacksize(
        __attr: *const pthread_attr_t,
        __stacksize: *mut libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getstack(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut libc::c_void,
        __stacksize: *mut libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setstack(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut libc::c_void,
        __stacksize: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setaffinity_np(
        __attr: *mut pthread_attr_t,
        __cpusetsize: libc::c_ulong,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getaffinity_np(
        __attr: *const pthread_attr_t,
        __cpusetsize: libc::c_ulong,
        __cpuset: *mut cpu_set_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_getattr_default_np(__attr: *mut pthread_attr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_setsigmask_np(
        __attr: *mut pthread_attr_t,
        sigmask: *const __sigset_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_attr_getsigmask_np(
        __attr: *const pthread_attr_t,
        sigmask: *mut __sigset_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_setattr_default_np(__attr: *const pthread_attr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_getattr_np(
        __th: libc::c_ulong,
        __attr: *mut pthread_attr_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_setschedparam(
        __target_thread: libc::c_ulong,
        __policy: libc::c_int,
        __param: *const sched_param,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_getschedparam(
        __target_thread: libc::c_ulong,
        __policy: *mut libc::c_int,
        __param: *mut sched_param,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_setschedprio(
        __target_thread: libc::c_ulong,
        __prio: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_getname_np(
        __target_thread: libc::c_ulong,
        __buf: *mut libc::c_char,
        __buflen: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_setname_np(
        __target_thread: libc::c_ulong,
        __name: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_getconcurrency() -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_setconcurrency(__level: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_yield() -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_setaffinity_np(
        __th: libc::c_ulong,
        __cpusetsize: libc::c_ulong,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_getaffinity_np(
        __th: libc::c_ulong,
        __cpusetsize: libc::c_ulong,
        __cpuset: *mut cpu_set_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_once(
        __once_control: *mut libc::c_int,
        __init_routine: Option<unsafe extern "C-unwind" fn()>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_setcancelstate(
        __state: libc::c_int,
        __oldstate: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_setcanceltype(
        __type: libc::c_int,
        __oldtype: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_cancel(__th: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_testcancel();
}
extern "C-unwind" {
    pub fn __sigsetjmp(
        __env: *mut __jmp_buf_tag,
        __savemask: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_timedlock(
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_clocklock(
        __mutex: *mut pthread_mutex_t,
        __clockid: libc::c_int,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_getprioceiling(
        __mutex: *const pthread_mutex_t,
        __prioceiling: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_setprioceiling(
        __mutex: *mut pthread_mutex_t,
        __prioceiling: libc::c_int,
        __old_ceiling: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutex_consistent(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "pthread_mutex_consistent"]
    pub fn pthread_mutex_consistent_np(_anon_0: *mut pthread_mutex_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_getpshared(
        __attr: *const pthread_mutexattr_t,
        __pshared: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_getprotocol(
        __attr: *const pthread_mutexattr_t,
        __protocol: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_setprotocol(
        __attr: *mut pthread_mutexattr_t,
        __protocol: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_getprioceiling(
        __attr: *const pthread_mutexattr_t,
        __prioceiling: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_setprioceiling(
        __attr: *mut pthread_mutexattr_t,
        __prioceiling: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_getrobust(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "pthread_mutexattr_getrobust"]
    pub fn pthread_mutexattr_getrobust_np(
        _anon_0: *mut pthread_mutexattr_t,
        _anon_1: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_mutexattr_setrobust(
        __attr: *mut pthread_mutexattr_t,
        __robustness: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "pthread_mutexattr_setrobust"]
    pub fn pthread_mutexattr_setrobust_np(
        _anon_0: *mut pthread_mutexattr_t,
        _anon_1: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_tryrdlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_timedrdlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_clockrdlock(
        __rwlock: *mut pthread_rwlock_t,
        __clockid: libc::c_int,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_trywrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_timedwrlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_clockwrlock(
        __rwlock: *mut pthread_rwlock_t,
        __clockid: libc::c_int,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlockattr_init(__attr: *mut pthread_rwlockattr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlockattr_destroy(__attr: *mut pthread_rwlockattr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlockattr_getpshared(
        __attr: *const pthread_rwlockattr_t,
        __pshared: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlockattr_setpshared(
        __attr: *mut pthread_rwlockattr_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlockattr_getkind_np(
        __attr: *const pthread_rwlockattr_t,
        __pref: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_rwlockattr_setkind_np(
        __attr: *mut pthread_rwlockattr_t,
        __pref: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_cond_clockwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __clock_id: libc::c_int,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_condattr_getpshared(
        __attr: *const pthread_condattr_t,
        __pshared: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_condattr_setpshared(
        __attr: *mut pthread_condattr_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_condattr_getclock(
        __attr: *const pthread_condattr_t,
        __clock_id: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_spin_init(
        __lock: *mut libc::c_int,
        __pshared: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_spin_destroy(__lock: *mut libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_spin_lock(__lock: *mut libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_spin_trylock(__lock: *mut libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_spin_unlock(__lock: *mut libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_barrier_init(
        __barrier: *mut pthread_barrier_t,
        __attr: *const pthread_barrierattr_t,
        __count: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_barrier_destroy(__barrier: *mut pthread_barrier_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_barrier_wait(__barrier: *mut pthread_barrier_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_barrierattr_init(__attr: *mut pthread_barrierattr_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_barrierattr_destroy(
        __attr: *mut pthread_barrierattr_t,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_barrierattr_getpshared(
        __attr: *const pthread_barrierattr_t,
        __pshared: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_barrierattr_setpshared(
        __attr: *mut pthread_barrierattr_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_key_create(
        __key: *mut libc::c_uint,
        __destr_function: Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_key_delete(__key: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_getspecific(__key: libc::c_uint) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn pthread_setspecific(
        __key: libc::c_uint,
        __pointer: *const libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_getcpuclockid(
        __thread_id: libc::c_ulong,
        __clock_id: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pthread_atfork(
        __prepare: Option<unsafe extern "C-unwind" fn()>,
        __parent: Option<unsafe extern "C-unwind" fn()>,
        __child: Option<unsafe extern "C-unwind" fn()>,
    ) -> libc::c_int;
}
pub(crate) unsafe extern "C-unwind" fn __gthread_active_p() -> libc::c_int {
    unsafe {
        {
            return 1;
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_create(
    mut __threadid: *mut libc::c_ulong,
    mut __func: Option<
        unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    mut __args: *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        {
            return pthread_create(
                ((__threadid) as *mut libc::c_ulong),
                ((0) as *const pthread_attr_t),
                __func,
                ((__args) as *mut libc::c_void),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_join(
    mut __threadid: libc::c_ulong,
    mut __value_ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        {
            return pthread_join(((__threadid) as libc::c_ulong), __value_ptr);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_detach(
    mut __threadid: libc::c_ulong,
) -> libc::c_int {
    unsafe {
        {
            return pthread_detach(((__threadid) as libc::c_ulong));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_equal(
    mut __t1: libc::c_ulong,
    mut __t2: libc::c_ulong,
) -> libc::c_int {
    unsafe {
        {
            return pthread_equal(((__t1) as libc::c_ulong), ((__t2) as libc::c_ulong));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_self() -> libc::c_ulong {
    unsafe {
        {
            return ((pthread_self()) as libc::c_ulong);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_yield() -> libc::c_int {
    unsafe {
        {
            return sched_yield();
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_once(
    mut __once: *mut libc::c_int,
    mut __func: Option<unsafe extern "C-unwind" fn()>,
) -> libc::c_int {
    unsafe {
        {
            if ((__gthread_active_p()) != 0) {
                return pthread_once(((__once) as *mut libc::c_int), __func);
            } else {
                return (-((1) as libc::c_int));
            }
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_key_create(
    mut __key: *mut libc::c_uint,
    mut __dtor: Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
) -> libc::c_int {
    unsafe {
        {
            return pthread_key_create(((__key) as *mut libc::c_uint), __dtor);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_key_delete(
    mut __key: libc::c_uint,
) -> libc::c_int {
    unsafe {
        {
            return pthread_key_delete(((__key) as libc::c_uint));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_getspecific(
    mut __key: libc::c_uint,
) -> *mut libc::c_void {
    unsafe {
        {
            return pthread_getspecific(((__key) as libc::c_uint));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_setspecific(
    mut __key: libc::c_uint,
    mut __ptr: *const libc::c_void,
) -> libc::c_int {
    unsafe {
        {
            return pthread_setspecific(((__key) as libc::c_uint), __ptr);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_mutex_init_function(
    mut __mutex: *mut pthread_mutex_t,
) {
    unsafe {
        {
            if ((__gthread_active_p()) != 0) {
                pthread_mutex_init(
                    ((__mutex) as *mut pthread_mutex_t),
                    ((0) as *const pthread_mutexattr_t),
                );
            }
        }
        ()
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_mutex_destroy(
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            if ((__gthread_active_p()) != 0) {
                return pthread_mutex_destroy(((__mutex) as *mut pthread_mutex_t));
            } else {
                return 0;
            }
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_mutex_lock(
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            if ((__gthread_active_p()) != 0) {
                return pthread_mutex_lock(((__mutex) as *mut pthread_mutex_t));
            } else {
                return 0;
            }
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_mutex_trylock(
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            if ((__gthread_active_p()) != 0) {
                return pthread_mutex_trylock(((__mutex) as *mut pthread_mutex_t));
            } else {
                return 0;
            }
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_mutex_timedlock(
    mut __mutex: *mut pthread_mutex_t,
    mut __abs_timeout: *const timespec,
) -> libc::c_int {
    unsafe {
        {
            if ((__gthread_active_p()) != 0) {
                return pthread_mutex_timedlock(
                    ((__mutex) as *mut pthread_mutex_t),
                    ((__abs_timeout) as *const timespec),
                );
            } else {
                return 0;
            }
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_mutex_unlock(
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            if ((__gthread_active_p()) != 0) {
                return pthread_mutex_unlock(((__mutex) as *mut pthread_mutex_t));
            } else {
                return 0;
            }
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_recursive_mutex_lock(
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            return __gthread_mutex_lock(((__mutex) as *mut pthread_mutex_t));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_recursive_mutex_trylock(
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            return __gthread_mutex_trylock(((__mutex) as *mut pthread_mutex_t));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_recursive_mutex_timedlock(
    mut __mutex: *mut pthread_mutex_t,
    mut __abs_timeout: *const timespec,
) -> libc::c_int {
    unsafe {
        {
            return __gthread_mutex_timedlock(
                ((__mutex) as *mut pthread_mutex_t),
                __abs_timeout,
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_recursive_mutex_unlock(
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            return __gthread_mutex_unlock(((__mutex) as *mut pthread_mutex_t));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_recursive_mutex_destroy(
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            return __gthread_mutex_destroy(((__mutex) as *mut pthread_mutex_t));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_cond_broadcast(
    mut __cond: *mut pthread_cond_t,
) -> libc::c_int {
    unsafe {
        {
            return pthread_cond_broadcast(((__cond) as *mut pthread_cond_t));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_cond_signal(
    mut __cond: *mut pthread_cond_t,
) -> libc::c_int {
    unsafe {
        {
            return pthread_cond_signal(((__cond) as *mut pthread_cond_t));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_cond_wait(
    mut __cond: *mut pthread_cond_t,
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            return pthread_cond_wait(
                ((__cond) as *mut pthread_cond_t),
                ((__mutex) as *mut pthread_mutex_t),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_cond_timedwait(
    mut __cond: *mut pthread_cond_t,
    mut __mutex: *mut pthread_mutex_t,
    mut __abs_timeout: *const timespec,
) -> libc::c_int {
    unsafe {
        {
            return pthread_cond_timedwait(
                ((__cond) as *mut pthread_cond_t),
                ((__mutex) as *mut pthread_mutex_t),
                ((__abs_timeout) as *const timespec),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_cond_wait_recursive(
    mut __cond: *mut pthread_cond_t,
    mut __mutex: *mut pthread_mutex_t,
) -> libc::c_int {
    unsafe {
        {
            return __gthread_cond_wait(__cond, ((__mutex) as *mut pthread_mutex_t));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub(crate) unsafe extern "C-unwind" fn __gthread_cond_destroy(
    mut __cond: *mut pthread_cond_t,
) -> libc::c_int {
    unsafe {
        {
            return pthread_cond_destroy(((__cond) as *mut pthread_cond_t));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
extern "C" {
    pub static mut __libc_single_threaded: libc::c_char;
}
pub mod __cxxabiv1 {}
#[doc = "* @class ACE_Message_Block\n *\n * @brief Stores messages for use throughout ACE (particularly\n * in an ACE_Message_Queue).\n *\n * An ACE_Message_Block is modeled after the message data\n * structures used in System V STREAMS.  Its purpose is to\n * enable efficient manipulation of arbitrarily large messages\n * without incurring much memory copying overhead.  Here are the\n * main characteristics of an ACE_Message_Block:\n * - Contains a pointer to a reference-counted\n *   ACE_Data_Block, which in turn points to the actual data\n *   buffer.  This allows very flexible and efficient sharing of\n *   data by multiple ACE_Message_Block objects.\n * - One or more ACE_Message_Blocks can be linked to form a\n *    ``fragment chain.''\n * - ACE_Message_Blocks can be linked together in a doubly linked fashion\n *   to form a queue of messages (this is how ACE_Message_Queue works).\n *\n * @see C++NPv1, section 4.2; APG, section 12.3.2."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Message_Block {
    pub vptr: *const (),
    pub rd_ptr_: libc::c_ulong,
    pub wr_ptr_: libc::c_ulong,
    pub priority_: libc::c_ulong,
    pub cont_: *mut ACE_Message_Block,
    pub next_: *mut ACE_Message_Block,
    pub prev_: *mut ACE_Message_Block,
    pub flags_: libc::c_ulong,
    pub data_block_: *mut ACE_Data_Block,
    pub message_block_allocator_: *mut ACE_Allocator,
}
#[export_name = "_ZN7ACE_CDR5Fixed12from_integerEl"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5Fixed12from_integerEl(
    val: libc::c_long,
) -> Fixed {
    unsafe { Fixed::from_integer(val) }
}
#[export_name = "_ZN7ACE_CDR5Fixed12from_integerEm"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5Fixed12from_integerEm(
    val: libc::c_ulong,
) -> Fixed {
    unsafe { Fixed::from_integer_ue87e25e85a23c26c(val) }
}
pub type BigFloat = crate::__f80::F80;
#[export_name = "_ZN7ACE_CDR5Fixed13from_floatingEe"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5Fixed13from_floatingEe(
    val: crate::__f80::F80,
) -> Fixed {
    unsafe { Fixed::from_floating(val) }
}
#[export_name = "_ZN7ACE_CDR5Fixed11from_stringEPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5Fixed11from_stringEPKc(
    str: *const libc::c_char,
) -> Fixed {
    unsafe { Fixed::from_string(str) }
}
#[export_name = "_ZN7ACE_CDR5Fixed11from_octetsEPKhij"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5Fixed11from_octetsEPKhij(
    array: *const libc::c_uchar,
    len: libc::c_int,
    scale: libc::c_uint,
) -> Fixed {
    unsafe { Fixed::from_octets(array, len, scale) }
}
#[export_name = "_ZNK7ACE_CDR5FixedcvlEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5FixedcvlEv(
    __this: *const Fixed,
) -> libc::c_long {
    unsafe { Fixed::operator_long(__this) }
}
#[export_name = "_ZNK7ACE_CDR5FixedcveEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5FixedcveEv(
    __this: *const Fixed,
) -> crate::__f80::F80 {
    unsafe { Fixed::operator_long_double(__this) }
}
#[export_name = "_ZNK7ACE_CDR5Fixed5roundEt"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5Fixed5roundEt(
    __this: *const Fixed,
    scale: libc::c_ushort,
) -> Fixed {
    unsafe { Fixed::round(__this, scale) }
}
#[export_name = "_ZNK7ACE_CDR5Fixed8truncateEt"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5Fixed8truncateEt(
    __this: *const Fixed,
    scale: libc::c_ushort,
) -> Fixed {
    unsafe { Fixed::truncate(__this, scale) }
}
#[export_name = "_ZNK7ACE_CDR5Fixed9to_stringEPcm"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5Fixed9to_stringEPcm(
    __this: *const Fixed,
    buffer: *mut libc::c_char,
    buffer_size: libc::c_ulong,
) -> bool {
    unsafe { Fixed::to_string(__this, buffer, buffer_size) }
}
#[export_name = "_ZNK7ACE_CDR5Fixed9to_octetsERi"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5Fixed9to_octetsERi(
    __this: *const Fixed,
    n: *mut libc::c_int,
) -> *const libc::c_uchar {
    unsafe { Fixed::to_octets(__this, n) }
}
#[export_name = "_ZN7ACE_CDR5FixedpLERKS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5FixedpLERKS0_(
    __this: *mut Fixed,
    rhs: *const Fixed,
) -> *mut Fixed {
    unsafe { Fixed::operator_add_assign(__this, rhs) }
}
#[export_name = "_ZN7ACE_CDR5FixedmIERKS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5FixedmIERKS0_(
    __this: *mut Fixed,
    rhs: *const Fixed,
) -> *mut Fixed {
    unsafe { Fixed::operator_sub_assign(__this, rhs) }
}
#[export_name = "_ZN7ACE_CDR5FixedmLERKS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5FixedmLERKS0_(
    __this: *mut Fixed,
    rhs: *const Fixed,
) -> *mut Fixed {
    unsafe { Fixed::operator_mul_assign(__this, rhs) }
}
#[export_name = "_ZN7ACE_CDR5FixeddVERKS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5FixeddVERKS0_(
    __this: *mut Fixed,
    rhs: *const Fixed,
) -> *mut Fixed {
    unsafe { Fixed::operator_div_assign(__this, rhs) }
}
#[export_name = "_ZN7ACE_CDR5FixedppEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5FixedppEv(
    __this: *mut Fixed,
) -> *mut Fixed {
    unsafe { Fixed::operator_inc(__this) }
}
#[export_name = "_ZN7ACE_CDR5FixedmmEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5FixedmmEv(
    __this: *mut Fixed,
) -> *mut Fixed {
    unsafe { Fixed::operator_dec(__this) }
}
#[export_name = "_ZNK7ACE_CDR5FixedntEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5FixedntEv(
    __this: *const Fixed,
) -> bool {
    unsafe { Fixed::operator_lnot(__this) }
}
#[export_name = "_ZNK7ACE_CDR5Fixed4lessERKS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5Fixed4lessERKS0_(
    __this: *const Fixed,
    rhs: *const Fixed,
) -> bool {
    unsafe { Fixed::less(__this, rhs) }
}
#[export_name = "_ZNK7ACE_CDR5Fixed5equalERKS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5Fixed5equalERKS0_(
    __this: *const Fixed,
    rhs: *const Fixed,
) -> bool {
    unsafe { Fixed::equal(__this, rhs) }
}
#[export_name = "_ZN7ACE_CDR5Fixed9normalizeEt"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5Fixed9normalizeEt(
    __this: *mut Fixed,
    min_scale: libc::c_ushort,
) {
    unsafe { Fixed::normalize(__this, min_scale) }
}
#[export_name = "_ZN7ACE_CDR5Fixed6lshiftEi"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR5Fixed6lshiftEi(
    __this: *mut Fixed,
    digits: libc::c_int,
) -> libc::c_int {
    unsafe { Fixed::lshift(__this, digits) }
}
#[export_name = "_ZNK7ACE_CDR5Fixed11div_helper2ERKS0_RS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5Fixed11div_helper2ERKS0_RS0_(
    __this: *const Fixed,
    rhs: *const Fixed,
    r: *mut Fixed,
) -> Fixed {
    unsafe { Fixed::div_helper2(__this, rhs, r) }
}
#[export_name = "_ZNK7ACE_CDR5Fixed11div_helper1ERKS0_RS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5Fixed11div_helper1ERKS0_RS0_(
    __this: *const Fixed,
    rhs: *const Fixed,
    r: *mut Fixed,
) -> Fixed {
    unsafe { Fixed::div_helper1(__this, rhs, r) }
}
#[export_name = "_ZNK7ACE_CDR5Fixed4joinEiRKS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZNK7ACE_CDR5Fixed4joinEiRKS0_(
    __this: *const Fixed,
    digits: libc::c_int,
    bot: *const Fixed,
) -> Fixed {
    unsafe { Fixed::join(__this, digits, bot) }
}
#[doc = "* @class ACE_CDR\n *\n * @brief Keep constants and some routines common to both Output and\n * Input CDR streams."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_CDR {}
#[export_name = "_ZN7ACE_CDR12swap_2_arrayEPKcPcm"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR12swap_2_arrayEPKcPcm(
    orig: *const libc::c_char,
    target: *mut libc::c_char,
    n: libc::c_ulong,
) {
    unsafe { ACE_CDR::swap_2_array(orig, target, n) }
}
#[export_name = "_ZN7ACE_CDR12swap_4_arrayEPKcPcm"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR12swap_4_arrayEPKcPcm(
    orig: *const libc::c_char,
    target: *mut libc::c_char,
    n: libc::c_ulong,
) {
    unsafe { ACE_CDR::swap_4_array(orig, target, n) }
}
#[export_name = "_ZN7ACE_CDR12swap_8_arrayEPKcPcm"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR12swap_8_arrayEPKcPcm(
    orig: *const libc::c_char,
    target: *mut libc::c_char,
    n: libc::c_ulong,
) {
    unsafe { ACE_CDR::swap_8_array(orig, target, n) }
}
#[export_name = "_ZN7ACE_CDR13swap_16_arrayEPKcPcm"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR13swap_16_arrayEPKcPcm(
    orig: *const libc::c_char,
    target: *mut libc::c_char,
    n: libc::c_ulong,
) {
    unsafe { ACE_CDR::swap_16_array(orig, target, n) }
}
#[export_name = "_ZN7ACE_CDR8mb_alignEP17ACE_Message_Block"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR8mb_alignEP17ACE_Message_Block(
    mb: *mut ACE_Message_Block,
) {
    unsafe { ACE_CDR::mb_align(mb) }
}
#[export_name = "_ZN7ACE_CDR4growEP17ACE_Message_Blockm"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR4growEP17ACE_Message_Blockm(
    mb: *mut ACE_Message_Block,
    minsize: libc::c_ulong,
) -> libc::c_int {
    unsafe { ACE_CDR::grow(mb, minsize) }
}
#[export_name = "_ZN7ACE_CDR11consolidateEP17ACE_Message_BlockPKS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR11consolidateEP17ACE_Message_BlockPKS0_(
    dst: *mut ACE_Message_Block,
    src: *const ACE_Message_Block,
) -> libc::c_int {
    unsafe { ACE_CDR::consolidate(dst, src) }
}
#[export_name = "_ZN7ACE_CDR12total_lengthEPK17ACE_Message_BlockS2_"]
pub unsafe extern "C-unwind" fn __xtu__ZN7ACE_CDR12total_lengthEPK17ACE_Message_BlockS2_(
    begin: *const ACE_Message_Block,
    end: *const ACE_Message_Block,
) -> libc::c_ulong {
    unsafe { ACE_CDR::total_length(begin, end) }
}
#[export_name = "_ZlsRSoRKN7ACE_CDR5FixedE"]
pub unsafe extern "C-unwind" fn operator_shl(
    mut lhs: *mut crate::__cxx_std::Ostream,
    mut rhs: *const Fixed,
) -> *mut crate::__cxx_std::Ostream {
    unsafe {
        {
            let mut digits: [libc::c_char; 35usize] = unsafe { ::core::mem::zeroed() };
            <Fixed>::to_string(
                (::core::ptr::addr_of!((* rhs))) as *const Fixed,
                ((digits).as_mut_ptr() as *mut libc::c_char),
                ((::core::mem::size_of::<[libc::c_char; 35usize]>() as libc::c_ulong)
                    as libc::c_ulong),
            );
            ((*lhs)).put_cstr((digits).as_ptr() as *const libc::c_char);
            return ::core::ptr::addr_of_mut!((* lhs));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
#[export_name = "_ZrsRSiRN7ACE_CDR5FixedE"]
pub unsafe extern "C-unwind" fn operator_shr(
    mut lhs: *mut crate::__cxx_std::Istream,
    mut rhs: *mut Fixed,
) -> *mut crate::__cxx_std::Istream {
    unsafe {
        {
            let mut num: libc::c_double = unsafe { ::core::mem::zeroed() };
            (((*lhs)).extract_f64(::core::ptr::addr_of_mut!(num)));
            let mut ld: crate::__f80::F80 = unsafe { ::core::mem::zeroed() };
            ld = (crate::__f80::F80::from_f64(num));
            {
                let __v = <Fixed>::from_floating((ld));
                let __asg_p = ::core::ptr::addr_of_mut!((* rhs));
                *__asg_p = __v;
                __asg_p
            };
            return ::core::ptr::addr_of_mut!((* lhs));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub mod ACE {}
pub mod ACE_Utils {
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_unsigned_char__is_signed: bool = ((false) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_unsigned_char_;
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_unsigned_short__is_signed: bool = ((false) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_unsigned_short_;
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_unsigned_int__is_signed: bool = ((false) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_unsigned_int_;
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_unsigned_long__is_signed: bool = ((false) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_unsigned_long_;
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_unsigned_long_long__is_signed: bool = ((false) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_unsigned_long_long_;
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_signed_char__is_signed: bool = ((true) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_signed_char_;
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_short__is_signed: bool = ((true) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_short_;
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_int__is_signed: bool = ((true) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_int_;
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_long__is_signed: bool = ((true) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_long_;
    #[allow(non_upper_case_globals)]
    pub const Sign_Check_long_long__is_signed: bool = ((true) as bool);
    pub use crate::full_ops_0::ACE_Utils::Sign_Check_long_long_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_char__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_char_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_short__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_short_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_int__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_int_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_long__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_long_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_long_long__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_unsigned_long_long_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_signed_char__signed_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_signed_char__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_signed_char_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_short__signed_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_short__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_short_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_int__signed_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_int__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_int_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_long__signed_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_long__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_long_;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_long_long__signed_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_long_long__unsigned_type;
    pub use crate::full_ops_0::ACE_Utils::To_Unsigned_long_long_;
    pub use crate::full_ops_0::ACE_Utils::Safe_Comparator_long__unsigned_long__true__false_;
    pub use crate::full_ops_0::ACE_Utils::Truncator_long__unsigned_long_;
    pub use crate::full_ops_0::ACE_Utils::truncator;
    pub use crate::full_ops_0::ACE_Utils::truncate_cast___long__ub591475ebc843689;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Time_Value_zero: ACE_Time_Value;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Time_Value_max_time: ACE_Time_Value;
}
extern "C-unwind" {
    #[link_name = "_ZlsRSoRK14ACE_Time_Value"]
    pub fn operator_shl_u87aa6c49d2c7f15d(
        o: *mut crate::__cxx_std::Ostream,
        v: *const ACE_Time_Value,
    ) -> *mut crate::__cxx_std::Ostream;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Allocator_allocator_: *mut ACE_Allocator;
}
#[doc = "* @class ACE_Allocator\n *\n * @brief Interface for a dynamic memory allocator that uses inheritance\n * and dynamic binding to provide extensible mechanisms for\n * allocating and deallocating memory."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Allocator {
    pub vptr: *const (),
}
#[doc = "* @class ACE_Data_Block\n *\n * @brief Stores the data payload that is accessed via one or more\n * ACE_Message_Block's.\n *\n * This data structure is reference counted to maximize\n * sharing.  It also contains the <locking_strategy_> (which\n * protects the reference count from race conditions in\n * concurrent programs) and the <allocation_strategy_> (which\n * determines what memory pool is used to allocate the memory)."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Data_Block {
    pub vptr: *const (),
    pub type_: libc::c_int,
    pub cur_size_: libc::c_ulong,
    pub max_size_: libc::c_ulong,
    pub flags_: libc::c_ulong,
    pub base_: *mut libc::c_char,
    pub allocator_strategy_: *mut ACE_Allocator,
    pub locking_strategy_: *mut ACE_Lock,
    pub reference_count_: libc::c_int,
    pub data_block_allocator_: *mut ACE_Allocator,
}
extern "C-unwind" {
    pub fn memfd_create(
        __name: *const libc::c_char,
        __flags: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mlock2(
        __addr: *const libc::c_void,
        __length: libc::c_ulong,
        __flags: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pkey_alloc(
        __flags: libc::c_uint,
        __access_rights: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pkey_set(__key: libc::c_int, __access_rights: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pkey_get(__key: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pkey_free(__key: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn pkey_mprotect(
        __addr: *mut libc::c_void,
        __len: libc::c_ulong,
        __prot: libc::c_int,
        __pkey: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mmap(
        __addr: *mut libc::c_void,
        __len: libc::c_ulong,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: libc::c_long,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn mmap64(
        __addr: *mut libc::c_void,
        __len: libc::c_ulong,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: libc::c_long,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn munmap(__addr: *mut libc::c_void, __len: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mprotect(
        __addr: *mut libc::c_void,
        __len: libc::c_ulong,
        __prot: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn msync(
        __addr: *mut libc::c_void,
        __len: libc::c_ulong,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn madvise(
        __addr: *mut libc::c_void,
        __len: libc::c_ulong,
        __advice: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn posix_madvise(
        __addr: *mut libc::c_void,
        __len: libc::c_ulong,
        __advice: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mlock(__addr: *const libc::c_void, __len: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn munlock(__addr: *const libc::c_void, __len: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mlockall(__flags: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn munlockall() -> libc::c_int;
}
extern "C-unwind" {
    pub fn mincore(
        __start: *mut libc::c_void,
        __len: libc::c_ulong,
        __vec: *mut libc::c_uchar,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mremap(
        __addr: *mut libc::c_void,
        __old_len: libc::c_ulong,
        __new_len: libc::c_ulong,
        __flags: libc::c_int,
        ...
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn remap_file_pages(
        __start: *mut libc::c_void,
        __size: libc::c_ulong,
        __prot: libc::c_int,
        __pgoff: libc::c_ulong,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn shm_open(
        __name: *const libc::c_char,
        __oflag: libc::c_int,
        __mode: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn shm_unlink(__name: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn process_madvise(
        __pid_fd: libc::c_int,
        __iov: *const iovec,
        __count: libc::c_ulong,
        __advice: libc::c_int,
        __flags: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn process_mrelease(pidfd: libc::c_int, flags: libc::c_uint) -> libc::c_int;
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Intrusive_List_ACE_Cleanup_Info_Node_ {
    pub head_: *mut ACE_Cleanup_Info_Node,
    pub tail_: *mut ACE_Cleanup_Info_Node,
}
pub mod __pstl {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_ {
    pub prev_: *mut ACE_Cleanup_Info_Node,
    pub next_: *mut ACE_Cleanup_Info_Node,
}
extern "C-unwind" {
    pub fn ace_cleanup_destroyer(_anon_0: *mut ACE_Cleanup, param: *mut libc::c_void);
}
#[doc = "* @class ACE_Cleanup_Info_Node\n *\n * @brief For maintaining a list of ACE_Cleanup_Info items.\n *\n * For internal use by ACE_Object_Manager."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Cleanup_Info_Node {
    pub __base_0: ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_,
    pub object_: *mut libc::c_void,
    pub cleanup_hook_: Option<
        unsafe extern "C-unwind" fn(*mut libc::c_void, *mut libc::c_void),
    >,
    pub param_: *mut libc::c_void,
    pub name_: *const libc::c_char,
}
impl Drop for ACE_Cleanup_Info_Node {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_uf985c898f84f8493"]
                fn __ext(__this: *mut ACE_Cleanup_Info_Node);
            }
            __ext(self as *mut Self);
        }
    }
}
pub type ACE_Cleanup_Info_Node_List = ACE_Intrusive_List_ACE_Cleanup_Info_Node_;
#[doc = "* @class ACE_OS_Exit_Info\n *\n * @brief Hold Object Manager cleanup (exit) information.\n *\n * @internal\n *\n * For internal use by the ACE library, only."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_OS_Exit_Info {
    pub registered_objects_: ::core::mem::ManuallyDrop<
        ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
    >,
}
impl Drop for ACE_OS_Exit_Info {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_ueef24a0e6f74e4cf"]
                fn __ext(__this: *mut ACE_OS_Exit_Info);
            }
            __ext(self as *mut Self);
        }
    }
}
#[repr(C)]
pub struct ACE_Object_Manager {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_OS_Object_Manager_Manager {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Log_Msg {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Thread_Hook {
    pub _opaque: [u8; 1],
}
extern "C-unwind" {
    pub fn ACE_OS_Object_Manager_Internal_Exit_Hook();
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_OS_Object_Manager_instance_: *mut ACE_OS_Object_Manager;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_OS_Object_Manager_preallocated_object: [*mut libc::c_void; 3usize];
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_OS_Object_Manager {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Object_Manager_Base>,
    pub default_mask_: *mut __sigset_t,
    pub thread_hook_: *mut ACE_Thread_Hook,
    pub exit_info_: ::core::mem::ManuallyDrop<ACE_OS_Exit_Info>,
}
extern "C-unwind" {
    pub fn insque(__elem: *mut libc::c_void, __prev: *mut libc::c_void);
}
extern "C-unwind" {
    pub fn remque(__elem: *mut libc::c_void);
}
extern "C-unwind" {
    pub fn hsearch(__item: entry, __action: libc::c_uint) -> *mut entry;
}
extern "C-unwind" {
    pub fn hcreate(__nel: libc::c_ulong) -> libc::c_int;
}
extern "C-unwind" {
    pub fn hdestroy();
}
extern "C-unwind" {
    pub fn hsearch_r(
        __item: entry,
        __action: libc::c_uint,
        __retval: *mut *mut entry,
        __htab: *mut hsearch_data,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn hcreate_r(__nel: libc::c_ulong, __htab: *mut hsearch_data) -> libc::c_int;
}
extern "C-unwind" {
    pub fn hdestroy_r(__htab: *mut hsearch_data);
}
extern "C-unwind" {
    pub fn tsearch(
        __key: *const libc::c_void,
        __rootp: *mut *mut libc::c_void,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn tfind(
        __key: *const libc::c_void,
        __rootp: *const *mut libc::c_void,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn tdelete(
        __key: *const libc::c_void,
        __rootp: *mut *mut libc::c_void,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn twalk(
        __root: *const libc::c_void,
        __action: Option<
            unsafe extern "C-unwind" fn(*const libc::c_void, libc::c_uint, libc::c_int),
        >,
    );
}
extern "C-unwind" {
    pub fn twalk_r(
        __root: *const libc::c_void,
        _anon_1: Option<
            unsafe extern "C-unwind" fn(
                *const libc::c_void,
                libc::c_uint,
                *mut libc::c_void,
            ),
        >,
        __closure: *mut libc::c_void,
    );
}
extern "C-unwind" {
    pub fn tdestroy(
        __root: *mut libc::c_void,
        __freefct: Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    );
}
extern "C-unwind" {
    pub fn lfind(
        __key: *const libc::c_void,
        __base: *const libc::c_void,
        __nmemb: *mut libc::c_ulong,
        __size: libc::c_ulong,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn lsearch(
        __key: *const libc::c_void,
        __base: *mut libc::c_void,
        __nmemb: *mut libc::c_ulong,
        __size: libc::c_ulong,
        __compar: Option<
            unsafe extern "C-unwind" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn gettext(__msgid: *const libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn dgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn __dgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn __dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn ngettext(
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn dngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn dcngettext(
        __domainname: *const libc::c_char,
        __msgid1: *const libc::c_char,
        __msgid2: *const libc::c_char,
        __n: libc::c_ulong,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn bind_textdomain_codeset(
        __domainname: *const libc::c_char,
        __codeset: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn __fpclassify(__value: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __signbit(__value: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __isinf(__value: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __finite(__value: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __isnan(__value: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __iseqsig(__x: libc::c_double, __y: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __issignaling(__value: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn acos(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __acos(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn asin(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __asin(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atan(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atan(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atan2(__y: libc::c_double, __x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atan2(__y: libc::c_double, __x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn cos(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __cos(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sin(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __sin(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn tan(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __tan(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn cosh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __cosh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sinh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __sinh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn tanh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __tanh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sincos(
        __x: libc::c_double,
        __sinx: *mut libc::c_double,
        __cosx: *mut libc::c_double,
    );
}
extern "C-unwind" {
    pub fn __sincos(
        __x: libc::c_double,
        __sinx: *mut libc::c_double,
        __cosx: *mut libc::c_double,
    );
}
extern "C-unwind" {
    pub fn acosh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __acosh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn asinh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __asinh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atanh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atanh(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn exp(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __exp(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn frexp(__x: libc::c_double, __exponent: *mut libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __frexp(__x: libc::c_double, __exponent: *mut libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ldexp(__x: libc::c_double, __exponent: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __ldexp(__x: libc::c_double, __exponent: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log10(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log10(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn modf(__x: libc::c_double, __iptr: *mut libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __modf(__x: libc::c_double, __iptr: *mut libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn exp10(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __exp10(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn expm1(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __expm1(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log1p(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log1p(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn logb(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __logb(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn exp2(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __exp2(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log2(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log2(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn pow(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __pow(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sqrt(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __sqrt(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn hypot(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __hypot(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn cbrt(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __cbrt(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ceil(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __ceil(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fabs(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fabs(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn floor(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __floor(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmod(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmod(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn finite(__value: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn drem(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __drem(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn significand(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __significand(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn copysign(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __copysign(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nan(__tagb: *const libc::c_char) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nan(__tagb: *const libc::c_char) -> libc::c_double;
}
extern "C-unwind" {
    pub fn j0(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __j0(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn j1(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __j1(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn jn(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __jn(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn y0(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __y0(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn y1(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __y1(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn yn(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __yn(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn erf(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __erf(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn erfc(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __erfc(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lgamma(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __lgamma(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn tgamma(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __tgamma(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn gamma(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __gamma(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lgamma_r(
        _anon_0: libc::c_double,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __lgamma_r(
        _anon_0: libc::c_double,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn rint(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __rint(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nextafter(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nextafter(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nexttoward(__x: libc::c_double, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nexttoward(__x: libc::c_double, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nextdown(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nextdown(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nextup(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nextup(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn remainder(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __remainder(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn scalbn(__x: libc::c_double, __n: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __scalbn(__x: libc::c_double, __n: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ilogb(__x: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __ilogb(__x: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn llogb(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __llogb(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn scalbln(__x: libc::c_double, __n: libc::c_long) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __scalbln(__x: libc::c_double, __n: libc::c_long) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nearbyint(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nearbyint(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn round(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __round(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn trunc(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __trunc(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn remquo(
        __x: libc::c_double,
        __y: libc::c_double,
        __quo: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __remquo(
        __x: libc::c_double,
        __y: libc::c_double,
        __quo: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lrint(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lrint(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llrint(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llrint(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn lround(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lround(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llround(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llround(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn fdim(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fdim(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmax(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmax(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmin(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmin(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fma(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fma(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn roundeven(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __roundeven(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fromfp(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfp(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfp(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfp(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fromfpx(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpx(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpx(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpx(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn canonicalize(
        __cx: *mut libc::c_double,
        __x: *const libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fmaxmag(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaxmag(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminmag(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminmag(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum_num(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum_num(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum_num(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum_num(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum_mag(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum_mag(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum_mag(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum_mag(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum_mag_num(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum_mag_num(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum_mag_num(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum_mag_num(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn totalorder(
        __x: *const libc::c_double,
        __y: *const libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn totalordermag(
        __x: *const libc::c_double,
        __y: *const libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpayload(__x: *const libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __getpayload(__x: *const libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn setpayload(
        __x: *mut libc::c_double,
        __payload: libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpayloadsig(
        __x: *mut libc::c_double,
        __payload: libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn scalb(__x: libc::c_double, __n: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __scalb(__x: libc::c_double, __n: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fpclassifyf(__value: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __signbitf(__value: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __isinff(__value: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __finitef(__value: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __isnanf(__value: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __iseqsigf(__x: libc::c_float, __y: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __issignalingf(__value: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn acosf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __acosf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn asinf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __asinf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn atanf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __atanf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn atan2f(__y: libc::c_float, __x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __atan2f(__y: libc::c_float, __x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn cosf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __cosf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn sinf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __sinf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn tanf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __tanf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn coshf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __coshf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn sinhf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __sinhf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn tanhf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __tanhf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn sincosf(
        __x: libc::c_float,
        __sinx: *mut libc::c_float,
        __cosx: *mut libc::c_float,
    );
}
extern "C-unwind" {
    pub fn __sincosf(
        __x: libc::c_float,
        __sinx: *mut libc::c_float,
        __cosx: *mut libc::c_float,
    );
}
extern "C-unwind" {
    pub fn acoshf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __acoshf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn asinhf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __asinhf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn atanhf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __atanhf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn expf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __expf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn frexpf(__x: libc::c_float, __exponent: *mut libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __frexpf(__x: libc::c_float, __exponent: *mut libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ldexpf(__x: libc::c_float, __exponent: libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __ldexpf(__x: libc::c_float, __exponent: libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn logf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __logf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn log10f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __log10f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn modff(__x: libc::c_float, __iptr: *mut libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __modff(__x: libc::c_float, __iptr: *mut libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn exp10f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __exp10f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn expm1f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __expm1f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn log1pf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __log1pf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn logbf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __logbf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn exp2f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __exp2f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn log2f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __log2f(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn powf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __powf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn sqrtf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __sqrtf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn hypotf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __hypotf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn cbrtf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __cbrtf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ceilf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __ceilf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fabsf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fabsf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn floorf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __floorf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmodf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmodf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn isinff(__value: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn finitef(__value: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn dremf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __dremf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn significandf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __significandf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn copysignf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __copysignf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nanf(__tagb: *const libc::c_char) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nanf(__tagb: *const libc::c_char) -> libc::c_float;
}
extern "C-unwind" {
    pub fn isnanf(__value: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn j0f(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __j0f(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn j1f(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __j1f(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn jnf(_anon_0: libc::c_int, _anon_1: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __jnf(_anon_0: libc::c_int, _anon_1: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn y0f(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __y0f(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn y1f(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __y1f(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ynf(_anon_0: libc::c_int, _anon_1: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __ynf(_anon_0: libc::c_int, _anon_1: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn erff(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __erff(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn erfcf(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __erfcf(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn lgammaf(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __lgammaf(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn tgammaf(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __tgammaf(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn gammaf(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __gammaf(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn lgammaf_r(
        _anon_0: libc::c_float,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __lgammaf_r(
        _anon_0: libc::c_float,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn rintf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __rintf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nextafterf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nextafterf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nexttowardf(__x: libc::c_float, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nexttowardf(__x: libc::c_float, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nextdownf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nextdownf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nextupf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nextupf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn remainderf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __remainderf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn scalbnf(__x: libc::c_float, __n: libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __scalbnf(__x: libc::c_float, __n: libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ilogbf(__x: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __ilogbf(__x: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn llogbf(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __llogbf(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn scalblnf(__x: libc::c_float, __n: libc::c_long) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __scalblnf(__x: libc::c_float, __n: libc::c_long) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nearbyintf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nearbyintf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn roundf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __roundf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn truncf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __truncf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn remquof(
        __x: libc::c_float,
        __y: libc::c_float,
        __quo: *mut libc::c_int,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __remquof(
        __x: libc::c_float,
        __y: libc::c_float,
        __quo: *mut libc::c_int,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn lrintf(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lrintf(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llrintf(__x: libc::c_float) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llrintf(__x: libc::c_float) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn lroundf(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lroundf(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llroundf(__x: libc::c_float) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llroundf(__x: libc::c_float) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn fdimf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fdimf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaxf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaxf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaf(
        __x: libc::c_float,
        __y: libc::c_float,
        __z: libc::c_float,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaf(
        __x: libc::c_float,
        __y: libc::c_float,
        __z: libc::c_float,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn roundevenf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __roundevenf(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fromfpf(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpf(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpf(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpf(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fromfpxf(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpxf(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpxf(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpxf(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn canonicalizef(
        __cx: *mut libc::c_float,
        __x: *const libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fmaxmagf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaxmagf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminmagf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminmagf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaximumf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaximumf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminimumf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminimumf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaximum_numf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaximum_numf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminimum_numf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminimum_numf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaximum_magf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaximum_magf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminimum_magf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminimum_magf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaximum_mag_numf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaximum_mag_numf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminimum_mag_numf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminimum_mag_numf(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn totalorderf(
        __x: *const libc::c_float,
        __y: *const libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn totalordermagf(
        __x: *const libc::c_float,
        __y: *const libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpayloadf(__x: *const libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __getpayloadf(__x: *const libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn setpayloadf(__x: *mut libc::c_float, __payload: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpayloadsigf(
        __x: *mut libc::c_float,
        __payload: libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn scalbf(__x: libc::c_float, __n: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __scalbf(__x: libc::c_float, __n: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fpclassifyl(__value: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __signbitl(__value: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __isinfl(__value: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __finitel(__value: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __isnanl(__value: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __iseqsigl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __issignalingl(__value: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn acosl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __acosl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn asinl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __asinl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn atanl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __atanl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn atan2l(__y: crate::__f80::F80, __x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __atan2l(__y: crate::__f80::F80, __x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn cosl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __cosl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn sinl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __sinl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn tanl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __tanl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn coshl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __coshl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn sinhl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __sinhl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn tanhl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __tanhl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn sincosl(
        __x: crate::__f80::F80,
        __sinx: *mut crate::__f80::F80,
        __cosx: *mut crate::__f80::F80,
    );
}
extern "C-unwind" {
    pub fn __sincosl(
        __x: crate::__f80::F80,
        __sinx: *mut crate::__f80::F80,
        __cosx: *mut crate::__f80::F80,
    );
}
extern "C-unwind" {
    pub fn acoshl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __acoshl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn asinhl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __asinhl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn atanhl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __atanhl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn expl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __expl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn frexpl(
        __x: crate::__f80::F80,
        __exponent: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __frexpl(
        __x: crate::__f80::F80,
        __exponent: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn ldexpl(__x: crate::__f80::F80, __exponent: libc::c_int) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __ldexpl(
        __x: crate::__f80::F80,
        __exponent: libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn logl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __logl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn log10l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __log10l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn modfl(
        __x: crate::__f80::F80,
        __iptr: *mut crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __modfl(
        __x: crate::__f80::F80,
        __iptr: *mut crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn exp10l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __exp10l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn expm1l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __expm1l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn log1pl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __log1pl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn logbl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __logbl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn exp2l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __exp2l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn log2l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __log2l(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn powl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __powl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn sqrtl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __sqrtl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn hypotl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __hypotl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn cbrtl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __cbrtl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn ceill(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __ceill(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fabsl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fabsl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn floorl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __floorl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmodl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmodl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn isinfl(__value: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn finitel(__value: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn dreml(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __dreml(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn significandl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __significandl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn copysignl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __copysignl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nanl(__tagb: *const libc::c_char) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nanl(__tagb: *const libc::c_char) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn isnanl(__value: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn j0l(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __j0l(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn j1l(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __j1l(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn jnl(_anon_0: libc::c_int, _anon_1: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __jnl(_anon_0: libc::c_int, _anon_1: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn y0l(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __y0l(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn y1l(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __y1l(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn ynl(_anon_0: libc::c_int, _anon_1: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __ynl(_anon_0: libc::c_int, _anon_1: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn erfl(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __erfl(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn erfcl(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __erfcl(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn lgammal(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __lgammal(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn tgammal(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __tgammal(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn gammal(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __gammal(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn lgammal_r(
        _anon_0: crate::__f80::F80,
        __signgamp: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __lgammal_r(
        _anon_0: crate::__f80::F80,
        __signgamp: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn rintl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __rintl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nextafterl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nextafterl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nexttowardl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nexttowardl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nextdownl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nextdownl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nextupl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nextupl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn remainderl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __remainderl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn scalbnl(__x: crate::__f80::F80, __n: libc::c_int) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __scalbnl(__x: crate::__f80::F80, __n: libc::c_int) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn ilogbl(__x: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __ilogbl(__x: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn llogbl(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __llogbl(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn scalblnl(__x: crate::__f80::F80, __n: libc::c_long) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __scalblnl(__x: crate::__f80::F80, __n: libc::c_long) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nearbyintl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nearbyintl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn roundl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __roundl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn truncl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __truncl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn remquol(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __quo: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __remquol(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __quo: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn lrintl(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lrintl(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llrintl(__x: crate::__f80::F80) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llrintl(__x: crate::__f80::F80) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn lroundl(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lroundl(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llroundl(__x: crate::__f80::F80) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llroundl(__x: crate::__f80::F80) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn fdiml(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fdiml(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaxl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaxl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmal(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __z: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmal(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __z: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn roundevenl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __roundevenl(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fromfpl(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpl(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpl(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpl(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fromfpxl(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpxl(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpxl(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpxl(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn canonicalizel(
        __cx: *mut crate::__f80::F80,
        __x: *const crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fmaxmagl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaxmagl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminmagl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminmagl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaximuml(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaximuml(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminimuml(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminimuml(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaximum_numl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaximum_numl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminimum_numl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminimum_numl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaximum_magl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaximum_magl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminimum_magl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminimum_magl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaximum_mag_numl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaximum_mag_numl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminimum_mag_numl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminimum_mag_numl(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn totalorderl(
        __x: *const crate::__f80::F80,
        __y: *const crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn totalordermagl(
        __x: *const crate::__f80::F80,
        __y: *const crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpayloadl(__x: *const crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __getpayloadl(__x: *const crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn setpayloadl(
        __x: *mut crate::__f80::F80,
        __payload: crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpayloadsigl(
        __x: *mut crate::__f80::F80,
        __payload: crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn scalbl(__x: crate::__f80::F80, __n: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __scalbl(__x: crate::__f80::F80, __n: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn acosf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __acosf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn asinf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __asinf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn atanf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __atanf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn atan2f32(__y: libc::c_float, __x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __atan2f32(__y: libc::c_float, __x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn cosf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __cosf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn sinf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __sinf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn tanf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __tanf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn coshf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __coshf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn sinhf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __sinhf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn tanhf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __tanhf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn sincosf32(
        __x: libc::c_float,
        __sinx: *mut libc::c_float,
        __cosx: *mut libc::c_float,
    );
}
extern "C-unwind" {
    pub fn __sincosf32(
        __x: libc::c_float,
        __sinx: *mut libc::c_float,
        __cosx: *mut libc::c_float,
    );
}
extern "C-unwind" {
    pub fn acoshf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __acoshf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn asinhf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __asinhf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn atanhf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __atanhf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn expf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __expf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn frexpf32(__x: libc::c_float, __exponent: *mut libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __frexpf32(__x: libc::c_float, __exponent: *mut libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ldexpf32(__x: libc::c_float, __exponent: libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __ldexpf32(__x: libc::c_float, __exponent: libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn logf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __logf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn log10f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __log10f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn modff32(__x: libc::c_float, __iptr: *mut libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __modff32(__x: libc::c_float, __iptr: *mut libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn exp10f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __exp10f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn expm1f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __expm1f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn log1pf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __log1pf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn logbf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __logbf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn exp2f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __exp2f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn log2f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __log2f32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn powf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __powf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn sqrtf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __sqrtf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn hypotf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __hypotf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn cbrtf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __cbrtf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ceilf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __ceilf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fabsf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fabsf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn floorf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __floorf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmodf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmodf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn copysignf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __copysignf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nanf32(__tagb: *const libc::c_char) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nanf32(__tagb: *const libc::c_char) -> libc::c_float;
}
extern "C-unwind" {
    pub fn j0f32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __j0f32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn j1f32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __j1f32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn jnf32(_anon_0: libc::c_int, _anon_1: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __jnf32(_anon_0: libc::c_int, _anon_1: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn y0f32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __y0f32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn y1f32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __y1f32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ynf32(_anon_0: libc::c_int, _anon_1: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __ynf32(_anon_0: libc::c_int, _anon_1: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn erff32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __erff32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn erfcf32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __erfcf32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn lgammaf32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __lgammaf32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn tgammaf32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __tgammaf32(_anon_0: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn lgammaf32_r(
        _anon_0: libc::c_float,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __lgammaf32_r(
        _anon_0: libc::c_float,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn rintf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __rintf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nextafterf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nextafterf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nextdownf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nextdownf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nextupf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nextupf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn remainderf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __remainderf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn scalbnf32(__x: libc::c_float, __n: libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __scalbnf32(__x: libc::c_float, __n: libc::c_int) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ilogbf32(__x: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __ilogbf32(__x: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    pub fn llogbf32(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __llogbf32(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn scalblnf32(__x: libc::c_float, __n: libc::c_long) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __scalblnf32(__x: libc::c_float, __n: libc::c_long) -> libc::c_float;
}
extern "C-unwind" {
    pub fn nearbyintf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __nearbyintf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn roundf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __roundf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn truncf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __truncf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn remquof32(
        __x: libc::c_float,
        __y: libc::c_float,
        __quo: *mut libc::c_int,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __remquof32(
        __x: libc::c_float,
        __y: libc::c_float,
        __quo: *mut libc::c_int,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn lrintf32(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lrintf32(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llrintf32(__x: libc::c_float) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llrintf32(__x: libc::c_float) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn lroundf32(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lroundf32(__x: libc::c_float) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llroundf32(__x: libc::c_float) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llroundf32(__x: libc::c_float) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn fdimf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fdimf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaxf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaxf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaf32(
        __x: libc::c_float,
        __y: libc::c_float,
        __z: libc::c_float,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaf32(
        __x: libc::c_float,
        __y: libc::c_float,
        __z: libc::c_float,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn roundevenf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __roundevenf32(__x: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fromfpf32(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpf32(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpf32(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpf32(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fromfpxf32(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpxf32(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpxf32(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpxf32(
        __x: libc::c_float,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn canonicalizef32(
        __cx: *mut libc::c_float,
        __x: *const libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fmaxmagf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaxmagf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminmagf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminmagf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaximumf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaximumf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminimumf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminimumf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaximum_numf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaximum_numf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminimum_numf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminimum_numf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaximum_magf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaximum_magf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminimum_magf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminimum_magf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmaximum_mag_numf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fmaximum_mag_numf32(
        __x: libc::c_float,
        __y: libc::c_float,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fminimum_mag_numf32(__x: libc::c_float, __y: libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __fminimum_mag_numf32(
        __x: libc::c_float,
        __y: libc::c_float,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn totalorderf32(
        __x: *const libc::c_float,
        __y: *const libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn totalordermagf32(
        __x: *const libc::c_float,
        __y: *const libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpayloadf32(__x: *const libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __getpayloadf32(__x: *const libc::c_float) -> libc::c_float;
}
extern "C-unwind" {
    pub fn setpayloadf32(
        __x: *mut libc::c_float,
        __payload: libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpayloadsigf32(
        __x: *mut libc::c_float,
        __payload: libc::c_float,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn acosf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __acosf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn asinf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __asinf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atanf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atanf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atan2f64(__y: libc::c_double, __x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atan2f64(__y: libc::c_double, __x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn cosf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __cosf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sinf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __sinf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn tanf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __tanf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn coshf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __coshf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sinhf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __sinhf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn tanhf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __tanhf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sincosf64(
        __x: libc::c_double,
        __sinx: *mut libc::c_double,
        __cosx: *mut libc::c_double,
    );
}
extern "C-unwind" {
    pub fn __sincosf64(
        __x: libc::c_double,
        __sinx: *mut libc::c_double,
        __cosx: *mut libc::c_double,
    );
}
extern "C-unwind" {
    pub fn acoshf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __acoshf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn asinhf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __asinhf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atanhf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atanhf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn expf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __expf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn frexpf64(__x: libc::c_double, __exponent: *mut libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __frexpf64(
        __x: libc::c_double,
        __exponent: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ldexpf64(__x: libc::c_double, __exponent: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __ldexpf64(__x: libc::c_double, __exponent: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn logf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __logf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log10f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log10f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn modff64(__x: libc::c_double, __iptr: *mut libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __modff64(__x: libc::c_double, __iptr: *mut libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn exp10f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __exp10f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn expm1f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __expm1f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log1pf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log1pf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn logbf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __logbf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn exp2f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __exp2f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log2f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log2f64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn powf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __powf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sqrtf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __sqrtf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn hypotf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __hypotf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn cbrtf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __cbrtf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ceilf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __ceilf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fabsf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fabsf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn floorf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __floorf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmodf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmodf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn copysignf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __copysignf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nanf64(__tagb: *const libc::c_char) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nanf64(__tagb: *const libc::c_char) -> libc::c_double;
}
extern "C-unwind" {
    pub fn j0f64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __j0f64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn j1f64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __j1f64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn jnf64(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __jnf64(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn y0f64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __y0f64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn y1f64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __y1f64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ynf64(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __ynf64(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn erff64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __erff64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn erfcf64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __erfcf64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lgammaf64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __lgammaf64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn tgammaf64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __tgammaf64(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lgammaf64_r(
        _anon_0: libc::c_double,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __lgammaf64_r(
        _anon_0: libc::c_double,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn rintf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __rintf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nextafterf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nextafterf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nextdownf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nextdownf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nextupf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nextupf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn remainderf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __remainderf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn scalbnf64(__x: libc::c_double, __n: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __scalbnf64(__x: libc::c_double, __n: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ilogbf64(__x: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __ilogbf64(__x: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn llogbf64(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __llogbf64(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn scalblnf64(__x: libc::c_double, __n: libc::c_long) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __scalblnf64(__x: libc::c_double, __n: libc::c_long) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nearbyintf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nearbyintf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn roundf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __roundf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn truncf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __truncf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn remquof64(
        __x: libc::c_double,
        __y: libc::c_double,
        __quo: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __remquof64(
        __x: libc::c_double,
        __y: libc::c_double,
        __quo: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lrintf64(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lrintf64(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llrintf64(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llrintf64(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn lroundf64(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lroundf64(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llroundf64(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llroundf64(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn fdimf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fdimf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaxf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaxf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaf64(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaf64(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn roundevenf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __roundevenf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fromfpf64(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpf64(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpf64(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpf64(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fromfpxf64(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpxf64(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpxf64(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpxf64(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn canonicalizef64(
        __cx: *mut libc::c_double,
        __x: *const libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fmaxmagf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaxmagf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminmagf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminmagf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximumf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximumf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimumf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimumf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum_numf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum_numf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum_numf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum_numf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum_magf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum_magf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum_magf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum_magf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum_mag_numf64(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum_mag_numf64(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum_mag_numf64(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum_mag_numf64(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn totalorderf64(
        __x: *const libc::c_double,
        __y: *const libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn totalordermagf64(
        __x: *const libc::c_double,
        __y: *const libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpayloadf64(__x: *const libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __getpayloadf64(__x: *const libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn setpayloadf64(
        __x: *mut libc::c_double,
        __payload: libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpayloadsigf64(
        __x: *mut libc::c_double,
        __payload: libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn acosf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __acosf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn asinf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __asinf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atanf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atanf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atan2f32x(__y: libc::c_double, __x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atan2f32x(__y: libc::c_double, __x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn cosf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __cosf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sinf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __sinf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn tanf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __tanf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn coshf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __coshf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sinhf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __sinhf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn tanhf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __tanhf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sincosf32x(
        __x: libc::c_double,
        __sinx: *mut libc::c_double,
        __cosx: *mut libc::c_double,
    );
}
extern "C-unwind" {
    pub fn __sincosf32x(
        __x: libc::c_double,
        __sinx: *mut libc::c_double,
        __cosx: *mut libc::c_double,
    );
}
extern "C-unwind" {
    pub fn acoshf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __acoshf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn asinhf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __asinhf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn atanhf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atanhf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn expf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __expf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn frexpf32x(
        __x: libc::c_double,
        __exponent: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __frexpf32x(
        __x: libc::c_double,
        __exponent: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ldexpf32x(__x: libc::c_double, __exponent: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __ldexpf32x(__x: libc::c_double, __exponent: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn logf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __logf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log10f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log10f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn modff32x(__x: libc::c_double, __iptr: *mut libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __modff32x(
        __x: libc::c_double,
        __iptr: *mut libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn exp10f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __exp10f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn expm1f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __expm1f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log1pf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log1pf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn logbf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __logbf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn exp2f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __exp2f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn log2f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __log2f32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn powf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __powf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn sqrtf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __sqrtf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn hypotf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __hypotf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn cbrtf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __cbrtf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ceilf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __ceilf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fabsf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fabsf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn floorf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __floorf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmodf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmodf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn copysignf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __copysignf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nanf32x(__tagb: *const libc::c_char) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nanf32x(__tagb: *const libc::c_char) -> libc::c_double;
}
extern "C-unwind" {
    pub fn j0f32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __j0f32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn j1f32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __j1f32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn jnf32x(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __jnf32x(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn y0f32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __y0f32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn y1f32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __y1f32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ynf32x(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __ynf32x(_anon_0: libc::c_int, _anon_1: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn erff32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __erff32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn erfcf32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __erfcf32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lgammaf32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __lgammaf32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn tgammaf32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __tgammaf32x(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lgammaf32x_r(
        _anon_0: libc::c_double,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __lgammaf32x_r(
        _anon_0: libc::c_double,
        __signgamp: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn rintf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __rintf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nextafterf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nextafterf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nextdownf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nextdownf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nextupf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nextupf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn remainderf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __remainderf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn scalbnf32x(__x: libc::c_double, __n: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __scalbnf32x(__x: libc::c_double, __n: libc::c_int) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ilogbf32x(__x: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __ilogbf32x(__x: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    pub fn llogbf32x(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __llogbf32x(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn scalblnf32x(__x: libc::c_double, __n: libc::c_long) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __scalblnf32x(__x: libc::c_double, __n: libc::c_long) -> libc::c_double;
}
extern "C-unwind" {
    pub fn nearbyintf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __nearbyintf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn roundf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __roundf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn truncf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __truncf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn remquof32x(
        __x: libc::c_double,
        __y: libc::c_double,
        __quo: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __remquof32x(
        __x: libc::c_double,
        __y: libc::c_double,
        __quo: *mut libc::c_int,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn lrintf32x(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lrintf32x(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llrintf32x(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llrintf32x(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn lroundf32x(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lroundf32x(__x: libc::c_double) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llroundf32x(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llroundf32x(__x: libc::c_double) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn fdimf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fdimf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaxf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaxf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaf32x(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaf32x(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn roundevenf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __roundevenf32x(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fromfpf32x(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpf32x(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpf32x(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpf32x(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fromfpxf32x(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpxf32x(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpxf32x(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpxf32x(
        __x: libc::c_double,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn canonicalizef32x(
        __cx: *mut libc::c_double,
        __x: *const libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fmaxmagf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaxmagf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminmagf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminmagf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximumf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximumf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimumf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimumf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum_numf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum_numf32x(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum_numf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum_numf32x(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum_magf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum_magf32x(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum_magf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum_magf32x(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fmaximum_mag_numf32x(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fmaximum_mag_numf32x(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn fminimum_mag_numf32x(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __fminimum_mag_numf32x(
        __x: libc::c_double,
        __y: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn totalorderf32x(
        __x: *const libc::c_double,
        __y: *const libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn totalordermagf32x(
        __x: *const libc::c_double,
        __y: *const libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpayloadf32x(__x: *const libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __getpayloadf32x(__x: *const libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn setpayloadf32x(
        __x: *mut libc::c_double,
        __payload: libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpayloadsigf32x(
        __x: *mut libc::c_double,
        __payload: libc::c_double,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn acosf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __acosf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn asinf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __asinf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn atanf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __atanf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn atan2f64x(
        __y: crate::__f80::F80,
        __x: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __atan2f64x(
        __y: crate::__f80::F80,
        __x: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn cosf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __cosf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn sinf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __sinf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn tanf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __tanf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn coshf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __coshf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn sinhf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __sinhf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn tanhf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __tanhf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn sincosf64x(
        __x: crate::__f80::F80,
        __sinx: *mut crate::__f80::F80,
        __cosx: *mut crate::__f80::F80,
    );
}
extern "C-unwind" {
    pub fn __sincosf64x(
        __x: crate::__f80::F80,
        __sinx: *mut crate::__f80::F80,
        __cosx: *mut crate::__f80::F80,
    );
}
extern "C-unwind" {
    pub fn acoshf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __acoshf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn asinhf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __asinhf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn atanhf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __atanhf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn expf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __expf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn frexpf64x(
        __x: crate::__f80::F80,
        __exponent: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __frexpf64x(
        __x: crate::__f80::F80,
        __exponent: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn ldexpf64x(
        __x: crate::__f80::F80,
        __exponent: libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __ldexpf64x(
        __x: crate::__f80::F80,
        __exponent: libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn logf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __logf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn log10f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __log10f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn modff64x(
        __x: crate::__f80::F80,
        __iptr: *mut crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __modff64x(
        __x: crate::__f80::F80,
        __iptr: *mut crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn exp10f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __exp10f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn expm1f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __expm1f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn log1pf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __log1pf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn logbf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __logbf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn exp2f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __exp2f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn log2f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __log2f64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn powf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __powf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn sqrtf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __sqrtf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn hypotf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __hypotf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn cbrtf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __cbrtf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn ceilf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __ceilf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fabsf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fabsf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn floorf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __floorf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmodf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmodf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn copysignf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __copysignf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nanf64x(__tagb: *const libc::c_char) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nanf64x(__tagb: *const libc::c_char) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn j0f64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __j0f64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn j1f64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __j1f64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn jnf64x(_anon_0: libc::c_int, _anon_1: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __jnf64x(
        _anon_0: libc::c_int,
        _anon_1: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn y0f64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __y0f64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn y1f64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __y1f64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn ynf64x(_anon_0: libc::c_int, _anon_1: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __ynf64x(
        _anon_0: libc::c_int,
        _anon_1: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn erff64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __erff64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn erfcf64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __erfcf64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn lgammaf64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __lgammaf64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn tgammaf64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __tgammaf64x(_anon_0: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn lgammaf64x_r(
        _anon_0: crate::__f80::F80,
        __signgamp: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __lgammaf64x_r(
        _anon_0: crate::__f80::F80,
        __signgamp: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn rintf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __rintf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nextafterf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nextafterf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nextdownf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nextdownf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nextupf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nextupf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn remainderf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __remainderf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn scalbnf64x(__x: crate::__f80::F80, __n: libc::c_int) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __scalbnf64x(__x: crate::__f80::F80, __n: libc::c_int) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn ilogbf64x(__x: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __ilogbf64x(__x: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    pub fn llogbf64x(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __llogbf64x(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn scalblnf64x(__x: crate::__f80::F80, __n: libc::c_long) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __scalblnf64x(__x: crate::__f80::F80, __n: libc::c_long) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn nearbyintf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __nearbyintf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn roundf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __roundf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn truncf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __truncf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn remquof64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __quo: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __remquof64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __quo: *mut libc::c_int,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn lrintf64x(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lrintf64x(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llrintf64x(__x: crate::__f80::F80) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llrintf64x(__x: crate::__f80::F80) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn lroundf64x(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __lroundf64x(__x: crate::__f80::F80) -> libc::c_long;
}
extern "C-unwind" {
    pub fn llroundf64x(__x: crate::__f80::F80) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn __llroundf64x(__x: crate::__f80::F80) -> libc::c_longlong;
}
extern "C-unwind" {
    pub fn fdimf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fdimf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaxf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaxf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __z: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __z: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn roundevenf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __roundevenf64x(__x: crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fromfpf64x(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpf64x(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpf64x(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpf64x(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn fromfpxf64x(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn __fromfpxf64x(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_long;
}
extern "C-unwind" {
    pub fn ufromfpxf64x(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __ufromfpxf64x(
        __x: crate::__f80::F80,
        __round: libc::c_int,
        __width: libc::c_uint,
    ) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn canonicalizef64x(
        __cx: *mut crate::__f80::F80,
        __x: *const crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fmaxmagf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaxmagf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminmagf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminmagf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaximumf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaximumf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminimumf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminimumf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaximum_numf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaximum_numf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminimum_numf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminimum_numf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaximum_magf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaximum_magf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminimum_magf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminimum_magf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fmaximum_mag_numf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fmaximum_mag_numf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn fminimum_mag_numf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __fminimum_mag_numf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
    ) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn totalorderf64x(
        __x: *const crate::__f80::F80,
        __y: *const crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn totalordermagf64x(
        __x: *const crate::__f80::F80,
        __y: *const crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpayloadf64x(__x: *const crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __getpayloadf64x(__x: *const crate::__f80::F80) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn setpayloadf64x(
        __x: *mut crate::__f80::F80,
        __payload: crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn setpayloadsigf64x(
        __x: *mut crate::__f80::F80,
        __payload: crate::__f80::F80,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fadd(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fdiv(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ffma(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmul(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fsqrt(__x: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fsub(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn faddl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fdivl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn ffmal(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __z: crate::__f80::F80,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fmull(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fsqrtl(__x: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn fsubl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn daddl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn ddivl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn dfmal(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __z: crate::__f80::F80,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn dmull(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn dsqrtl(__x: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn dsubl(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32addf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32divf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32fmaf32x(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32mulf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32sqrtf32x(__x: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32subf32x(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32addf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32divf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32fmaf64(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32mulf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32sqrtf64(__x: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32subf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32addf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32divf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32fmaf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __z: crate::__f80::F80,
    ) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32mulf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32sqrtf64x(__x: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32subf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_float;
}
extern "C-unwind" {
    pub fn f32xaddf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xdivf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xfmaf64(
        __x: libc::c_double,
        __y: libc::c_double,
        __z: libc::c_double,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xmulf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xsqrtf64(__x: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xsubf64(__x: libc::c_double, __y: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xaddf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xdivf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xfmaf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __z: crate::__f80::F80,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xmulf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xsqrtf64x(__x: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f32xsubf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f64addf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f64divf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f64fmaf64x(
        __x: crate::__f80::F80,
        __y: crate::__f80::F80,
        __z: crate::__f80::F80,
    ) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f64mulf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f64sqrtf64x(__x: crate::__f80::F80) -> libc::c_double;
}
extern "C-unwind" {
    pub fn f64subf64x(__x: crate::__f80::F80, __y: crate::__f80::F80) -> libc::c_double;
}
extern "C" {
    pub static mut signgam: libc::c_int;
}
extern "C-unwind" {
    pub fn __iscanonicall(__x: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "_Z11iscanonicalf"]
    pub fn iscanonical_udd5798e54358852e(__val: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "_Z11iscanonicald"]
    pub fn iscanonical_udd5108e5435324d0(__val: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "_Z11iscanonicale"]
    pub fn iscanonical(__val: crate::__f80::F80) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "_Z11issignalingf"]
    pub fn issignaling_u0f011b8e77886c90(__val: libc::c_float) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "_Z11issignalingd"]
    pub fn issignaling_u0f07ab8e778dccee(__val: libc::c_double) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "_Z11issignalinge"]
    pub fn issignaling(__val: crate::__f80::F80) -> libc::c_int;
}
pub mod __anon_ns_13718 {
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct BufferAppender {
        pub buffer_: *mut libc::c_char,
        pub buffer_size_: libc::c_ulong,
        pub idx_: libc::c_ulong,
    }
    impl BufferAppender {
        pub unsafe fn new_at(
            __this: *mut Self,
            mut buffer: *mut libc::c_char,
            mut buffer_size: libc::c_ulong,
        ) {
            unsafe {
                ::core::ptr::write(
                    __this,
                    ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
                );
                ::core::ptr::write(
                    ::core::ptr::addr_of_mut!((* __this).buffer_),
                    ((buffer) as *mut libc::c_char),
                );
                ::core::ptr::write(
                    ::core::ptr::addr_of_mut!((* __this).buffer_size_),
                    ((buffer_size) as libc::c_ulong),
                );
                ::core::ptr::write(
                    ::core::ptr::addr_of_mut!((* __this).idx_),
                    ((0) as libc::c_ulong),
                );
                {}
                ()
            }
        }
        pub unsafe fn new(mut __a0: *mut libc::c_char, mut __a1: libc::c_ulong) -> Self {
            let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed()
                .assume_init();
            Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
            __obj
        }
        pub unsafe fn operator_add_assign(
            __this: *mut Self,
            mut ch: libc::c_char,
        ) -> bool {
            unsafe {
                let __this: *mut Self = __this as *mut Self;
                {
                    if ((((((*__this).idx_ as libc::c_ulong))
                        == (((((((*__this).buffer_size_) as libc::c_ulong))
                            .wrapping_sub((1) as libc::c_ulong)) as libc::c_ulong)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        return crate::__cxx_std::__Truthy::__truthy(false);
                    }
                    (*((*__this).buffer_)
                        .wrapping_offset(
                            ({
                                let __lv = &mut ((*__this).idx_);
                                let __r = *__lv;
                                *__lv = (*__lv).wrapping_add(1);
                                __r
                            }) as isize,
                        )) = ch;
                    return crate::__cxx_std::__Truthy::__truthy(true);
                }
                #[allow(unreachable_code)] { ::core::unreachable!() }
            }
        }
    }
}
extern "C-unwind" {
    pub fn __builtin_memcmp(
        _anon_0: *const libc::c_void,
        _anon_1: *const libc::c_void,
        _anon_2: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __builtin_strlen(_anon_0: *const libc::c_char) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __builtin_memchr(
        _anon_0: *const libc::c_void,
        _anon_1: libc::c_int,
        _anon_2: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn __builtin_memmove(
        _anon_0: *mut libc::c_void,
        _anon_1: *const libc::c_void,
        _anon_2: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn __builtin_memcpy(
        _anon_0: *mut libc::c_void,
        _anon_1: *const libc::c_void,
        _anon_2: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn __builtin_memset(
        _anon_0: *mut libc::c_void,
        _anon_1: libc::c_int,
        _anon_2: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn __builtin_strcmp(
        _anon_0: *const libc::c_char,
        _anon_1: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __builtin_huge_valf() -> libc::c_float;
}
extern "C-unwind" {
    pub fn __builtin_nanf(_anon_0: *const libc::c_char) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __builtin_nansf(_anon_0: *const libc::c_char) -> libc::c_float;
}
extern "C-unwind" {
    pub fn __builtin_huge_val() -> libc::c_double;
}
extern "C-unwind" {
    pub fn __builtin_nan(_anon_0: *const libc::c_char) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __builtin_nans(_anon_0: *const libc::c_char) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __builtin_huge_vall() -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __builtin_nanl(_anon_0: *const libc::c_char) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __builtin_nansl(_anon_0: *const libc::c_char) -> crate::__f80::F80;
}
extern "C-unwind" {
    pub fn __builtin_constant_p(_anon_0: libc::c_long, ...) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __builtin_expect(
        _anon_0: libc::c_long,
        _anon_1: libc::c_long,
    ) -> libc::c_long;
}
impl ACE_Message_Block {
    ///Create an empty message.
    pub unsafe fn new_at(__this: *mut Self, mut __a0: *mut ACE_Allocator) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_BlockC1EP13ACE_Allocator"]
            fn __ext(__this: *mut ACE_Message_Block, __a0: *mut ACE_Allocator);
        }
        __ext(__this as *mut ACE_Message_Block, __a0)
    }
    pub unsafe fn new(mut __a0: *mut ACE_Allocator) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    #[doc = "* Create an ACE_Message_Block that owns the specified ACE_Data_Block\n   * without copying it. If the @a flags is set to @c DONT_DELETE we\n   * don't delete the ACE_Data_Block. It is left to the client's\n   * responsibility to take care of the memory allocated for the\n   * data_block"]
    pub unsafe fn new_at_ud78bec71d8b405ec(
        __this: *mut Self,
        mut __a0: *mut ACE_Data_Block,
        mut __a1: libc::c_ulong,
        mut __a2: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_BlockC1EP14ACE_Data_BlockmP13ACE_Allocator"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                __a0: *mut ACE_Data_Block,
                __a1: libc::c_ulong,
                __a2: *mut ACE_Allocator,
            );
        }
        __ext(__this as *mut ACE_Message_Block, __a0, __a1, __a2)
    }
    pub unsafe fn new_ud78bec71d8b405ec(
        mut __a0: *mut ACE_Data_Block,
        mut __a1: libc::c_ulong,
        mut __a2: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ud78bec71d8b405ec(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    #[doc = "* Create an ACE_Message_Block that refers to @a data without\n   * copying it. The @a data memory will not be freed when this block is\n   * destroyed; memory management of @a data is left to the caller.\n   * Note that the @c size of the new ACE_Message_Block will be @a size, but\n   * the @c length will be 0 until the write pointer is changed."]
    pub unsafe fn new_at_u3b4a62d4028d8fb3(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_BlockC1EPKcmm"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                __a0: *const libc::c_char,
                __a1: libc::c_ulong,
                __a2: libc::c_ulong,
            );
        }
        __ext(__this as *mut ACE_Message_Block, __a0, __a1, __a2)
    }
    pub unsafe fn new_u3b4a62d4028d8fb3(
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_ulong,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u3b4a62d4028d8fb3(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    #[doc = "* Create an initialized message of type @a type containing @a size\n   * bytes.  The @a cont argument initializes the continuation field in\n   * the ACE_Message_Block.  If @a data == 0 then this block allocates and\n   * owns the block's memory, using @a allocator to get the data if it's\n   * non-0.  If @a data != 0 then this block refers to that memory until\n   * this this block ceases to exist; this object will not free @a data on\n   * destruction.  If @a locking_strategy is non-0 then this is used\n   * to protect regions of code that access shared state (e.g.,\n   * reference counting) from race conditions.  Note that the @c size\n   * of the ACE_Message_Block will be @a size, but the @c length will be 0\n   * until the write pointer is set. The @a data_block_allocator is used to\n   * allocate the data blocks while the @a allocator_strategy is used\n   * to allocate the buffers contained by those. The\n   * @a message_block_allocator is used to allocate new ACE_Message_Block\n   * objects when the duplicate() method is called. If a\n   * @a message_block_allocator is given, this ACE_Message_Block and\n   * future ACE_Message_Block objects created by duplicate() will be\n   * freed using this allocator when they are released.\n   * @note If you use this allocator, the ACE_Message_Block you created\n   * should have been created using this allocator because it will be\n   * released to the same allocator."]
    pub unsafe fn new_at_u88330d251d8d56ee(
        __this: *mut Self,
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Message_Block,
        mut __a3: *const libc::c_char,
        mut __a4: *mut ACE_Allocator,
        mut __a5: *mut ACE_Lock,
        mut __a6: libc::c_ulong,
        mut __a7: *const ACE_Time_Value,
        mut __a8: *const ACE_Time_Value,
        mut __a9: *mut ACE_Allocator,
        mut __a10: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_BlockC1EmiPS_PKcP13ACE_AllocatorP8ACE_LockmRK14ACE_Time_ValueS9_S4_S4_"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                __a0: libc::c_ulong,
                __a1: libc::c_int,
                __a2: *mut ACE_Message_Block,
                __a3: *const libc::c_char,
                __a4: *mut ACE_Allocator,
                __a5: *mut ACE_Lock,
                __a6: libc::c_ulong,
                __a7: *const ACE_Time_Value,
                __a8: *const ACE_Time_Value,
                __a9: *mut ACE_Allocator,
                __a10: *mut ACE_Allocator,
            );
        }
        __ext(
            __this as *mut ACE_Message_Block,
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
            __a7,
            __a8,
            __a9,
            __a10,
        )
    }
    pub unsafe fn new_u88330d251d8d56ee(
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Message_Block,
        mut __a3: *const libc::c_char,
        mut __a4: *mut ACE_Allocator,
        mut __a5: *mut ACE_Lock,
        mut __a6: libc::c_ulong,
        mut __a7: *const ACE_Time_Value,
        mut __a8: *const ACE_Time_Value,
        mut __a9: *mut ACE_Allocator,
        mut __a10: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u88330d251d8d56ee(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
            __a7,
            __a8,
            __a9,
            __a10,
        );
        __obj
    }
    #[doc = "* A copy constructor. This constructor is a bit different. If the\n   * incoming Message Block has a data block from the stack this\n   * constructor does a deep copy ie. allocates a new data block on\n   * the heap and does a copy of the data from the incoming message\n   * block. As a final note, the alignment information is used to\n   * align the data block if it is created afresh. If the incoming\n   * @a mb has a data block has a data block allocated from the heap,\n   * then this constructor just duplicates (ie. a shallow copy) the\n   * data block of the incoming @a mb."]
    pub unsafe fn new_at_u993a93dea1ea86cd(
        __this: *mut Self,
        mut __a0: *const ACE_Message_Block,
        mut __a1: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_BlockC1ERKS_m"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                __a0: *const ACE_Message_Block,
                __a1: libc::c_ulong,
            );
        }
        __ext(__this as *mut ACE_Message_Block, __a0, __a1)
    }
    pub unsafe fn new_u993a93dea1ea86cd(
        mut __a0: *const ACE_Message_Block,
        mut __a1: libc::c_ulong,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u993a93dea1ea86cd(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    #[doc = "* Create a Message Block that assumes it has ownership of @a data,\n   * but in reality it doesn't (i.e., cannot delete it since it didn't\n   * malloc it!).  Note that the @c size of the Message_Block will\n   * be @a size, but the @a length  will be 0 until <wr_ptr> is set."]
    pub unsafe fn init(
        __this: *mut Self,
        mut data: *const libc::c_char,
        mut size: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block4initEPKcm"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                data: *const libc::c_char,
                size: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Message_Block, data, size)
    }
    #[doc = "* Create an initialized message of type @a type containing @a size\n   * bytes.  The @a cont argument initializes the continuation field in\n   * the Message_Block.  If @a data == 0 then we create and own the\n   * @a data, using @a allocator_strategy to get the data if it's non-0.  If\n   * @a data != 0 we assume that we have ownership of the @a data till\n   * this object ceases to exist  (and don't delete it during\n   * destruction).  If @a locking_strategy is non-0 then this is used\n   * to protect regions of code that access shared state (e.g.,\n   * reference counting) from race conditions.  Note that the @a size\n   * of the Message_Block will be @a size, but the @a length will be 0\n   * until <wr_ptr> is set. The @a data_block_allocator is use to\n   * allocate the data blocks while the @a allocator_strategy is used\n   * to allocate the buffers contained by those."]
    pub unsafe fn init_u14798fa021f25e35(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut r#type: libc::c_int,
        mut cont: *mut ACE_Message_Block,
        mut data: *const libc::c_char,
        mut allocator_strategy: *mut ACE_Allocator,
        mut locking_strategy: *mut ACE_Lock,
        mut priority: libc::c_ulong,
        mut execution_time: *const ACE_Time_Value,
        mut deadline_time: *const ACE_Time_Value,
        mut data_block_allocator: *mut ACE_Allocator,
        mut message_block_allocator: *mut ACE_Allocator,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block4initEmiPS_PKcP13ACE_AllocatorP8ACE_LockmRK14ACE_Time_ValueS9_S4_S4_"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                size: libc::c_ulong,
                r#type: libc::c_int,
                cont: *mut ACE_Message_Block,
                data: *const libc::c_char,
                allocator_strategy: *mut ACE_Allocator,
                locking_strategy: *mut ACE_Lock,
                priority: libc::c_ulong,
                execution_time: *const ACE_Time_Value,
                deadline_time: *const ACE_Time_Value,
                data_block_allocator: *mut ACE_Allocator,
                message_block_allocator: *mut ACE_Allocator,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_Message_Block,
            size,
            r#type,
            cont,
            data,
            allocator_strategy,
            locking_strategy,
            priority,
            execution_time,
            deadline_time,
            data_block_allocator,
            message_block_allocator,
        )
    }
    ///Get type of the message.
    pub unsafe fn msg_type(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::msg_type(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *const ACE_Data_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set type of the message.
    pub unsafe fn msg_type_u96eef9130ed7ee95(__this: *mut Self, mut t: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Data_Block>::msg_type_u9911b9cef7bd0539(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *mut ACE_Data_Block,
                    t,
                );
            }
            ()
        }
    }
    ///Find out what type of message this is.
    pub unsafe fn is_data_msg(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut mt: libc::c_int = ((<ACE_Message_Block>::msg_type(
                    (__this) as *const ACE_Message_Block,
                )) as libc::c_int);
                return (((((((((((((mt as libc::c_int))
                    == ((((1 as libc::c_int)) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                    || (((((mt as libc::c_int))
                        == ((((2 as libc::c_int)) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                    || (((((mt as libc::c_int))
                        == ((((131 as libc::c_int)) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)) as libc::c_int)) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Find out what class of message this is (there are two classes,
  /// @c normal messages and @c high-priority messages).*/
    pub unsafe fn msg_class(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((<ACE_Message_Block>::msg_type(
                    (__this) as *const ACE_Message_Block,
                ) as libc::c_int)) < ((((128 as libc::c_int)) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (((0 as libc::c_int)) as libc::c_int);
                } else {
                    if (((((<ACE_Message_Block>::msg_type(
                        (__this) as *const ACE_Message_Block,
                    ) as libc::c_int)) < ((((512 as libc::c_int)) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        return (((128 as libc::c_int)) as libc::c_int);
                    } else {
                        return (((512 as libc::c_int)) as libc::c_int);
                    }
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Bitwise-or the @a more_flags into the existing message flags and
  /// return the new value.*/
    pub unsafe fn set_flags(
        __this: *mut Self,
        mut more_flags: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::set_flags(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *mut ACE_Data_Block,
                    more_flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Clear the message flag bits specified in @a less_flags and return
  /// the new value.*/
    pub unsafe fn clr_flags(
        __this: *mut Self,
        mut less_flags: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::clr_flags(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *mut ACE_Data_Block,
                    less_flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the current message flags.
    pub unsafe fn flags(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::flags(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *const ACE_Data_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "Bitwise-or the @a more_flags into the existing message flags and\n  /// return the new value.\n  /** @todo I think the following set of methods could not be used at\n   *  all. May be they are useless. Let us have it so that we don't\n   *  mess up memory management of the Message_Block. Somebody correct\n   *  me if I am totally totally wrong.."]
    pub unsafe fn set_self_flags(
        __this: *mut Self,
        mut more_flags: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ({
                    let __lv = ::core::ptr::addr_of_mut!((* __this).flags_);
                    unsafe {
                        *__lv = ((((*__lv)) as libc::c_ulong))
                            | (((more_flags)) as libc::c_ulong);
                        *__lv
                    }
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Clear the message flag bits specified in @a less_flags and return
  /// the new value.*/
    pub unsafe fn clr_self_flags(
        __this: *mut Self,
        mut less_flags: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ({
                    let __lv = ::core::ptr::addr_of_mut!((* __this).flags_);
                    unsafe {
                        *__lv = ((((*__lv)) as libc::c_ulong))
                            & (((!((less_flags)))) as libc::c_ulong);
                        *__lv
                    }
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the current message flags.
    pub unsafe fn self_flags(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).flags_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get priority of the message.
    pub unsafe fn msg_priority(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).priority_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set priority of the message.
    pub unsafe fn msg_priority_u65505cfc65301d38(
        __this: *mut Self,
        mut pri: libc::c_ulong,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).priority_ = pri;
            }
            ()
        }
    }
    ///Get execution time associated with the message.
    pub unsafe fn msg_execution_time(__this: *const Self) -> *const ACE_Time_Value {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of!(ACE_Time_Value_zero);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set execution time associated with the message.
    pub unsafe fn msg_execution_time_u82e16c1a80e26183(
        __this: *mut Self,
        mut et: *const ACE_Time_Value,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let _ = ((*et));
                };
            }
            ()
        }
    }
    ///Get absolute time of deadline associated with the message.
    pub unsafe fn msg_deadline_time(__this: *const Self) -> *const ACE_Time_Value {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of!(ACE_Time_Value_max_time);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set absolute time of deadline associated with the message.
    pub unsafe fn msg_deadline_time_u66d782eb3be90bf9(
        __this: *mut Self,
        mut dt: *const ACE_Time_Value,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let _ = ((*dt));
                };
            }
            ()
        }
    }
    /**Return an exact "deep copy" of the message, i.e., create fresh
  /// new copies of all the Data_Blocks and continuations.*/
    pub unsafe fn clone(
        __this: *const Self,
        mut mask: libc::c_ulong,
    ) -> *mut ACE_Message_Block {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Message_Block5cloneEm"]
            fn __ext(
                __this: *const ACE_Message_Block,
                mask: libc::c_ulong,
            ) -> *mut ACE_Message_Block;
        }
        __ext(__this as *const ACE_Message_Block, mask)
    }
    ///Return a "shallow" copy that increments our reference count by 1.
    pub unsafe fn duplicate(__this: *const Self) -> *mut ACE_Message_Block {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Message_Block9duplicateEv"]
            fn __ext(__this: *const ACE_Message_Block) -> *mut ACE_Message_Block;
        }
        __ext(__this as *const ACE_Message_Block)
    }
    #[doc = "* Return a \"shallow\" copy that increments our reference count by 1.\n   * This is similar to CORBA's _duplicate() method, which is useful\n   * if you want to eliminate lots of checks for NULL @a mb pointers\n   * before calling _duplicate() on them."]
    pub unsafe fn duplicate_uca087717be6b68c1(
        mut mb: *const ACE_Message_Block,
    ) -> *mut ACE_Message_Block {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block9duplicateEPKS_"]
            fn __ext(mb: *const ACE_Message_Block) -> *mut ACE_Message_Block;
        }
        __ext(mb)
    }
    #[doc = "* Decrease the shared ACE_Data_Block's reference count by 1.  If the\n   * ACE_Data_Block's reference count goes to 0, it is deleted.\n   * In all cases, this ACE_Message_Block is deleted - it must have come\n   * from the heap, or there will be trouble.\n   *\n   * release() is designed to release the continuation chain; the\n   * destructor is not.  If we make the destructor release the\n   * continuation chain by calling release() or delete on the message\n   * blocks in the continuation chain, the following code will not\n   * work since the message block in the continuation chain is not off\n   * the heap:\n   *\n   *  ACE_Message_Block mb1 (1024);\n   *  ACE_Message_Block mb2 (1024);\n   *\n   *  mb1.cont (&mb2);\n   *\n   * And hence, call release() on a dynamically allocated message\n   * block. This will release all the message blocks in the\n   * continuation chain.  If you call delete or let the message block\n   * fall off the stack, cleanup of the message blocks in the\n   * continuation chain becomes the responsibility of the user.\n   *\n   * @retval 0, always, and the object this method was invoked on is no\n   *            longer valid."]
    pub unsafe fn release(__this: *mut Self) -> *mut ACE_Message_Block {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block7releaseEv"]
            fn __ext(__this: *mut ACE_Message_Block) -> *mut ACE_Message_Block;
        }
        __ext(__this as *mut ACE_Message_Block)
    }
    #[doc = "* This behaves like the non-static method release(), except that it\n   * checks if @a mb is 0.  This is similar to CORBA::release(), which\n   * is useful if you want to eliminate lots of checks for NULL\n   * pointers before calling release() on them.  Returns @a mb."]
    pub unsafe fn release_ubc51e64ee0ea988c(
        mut mb: *mut ACE_Message_Block,
    ) -> *mut ACE_Message_Block {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block7releaseEPS_"]
            fn __ext(mb: *mut ACE_Message_Block) -> *mut ACE_Message_Block;
        }
        __ext(mb)
    }
    #[doc = "* Copies data into this ACE_Message_Block. Data is copied into the\n   * block starting at the current write pointer.\n   *\n   * @param buf  Pointer to the buffer to copy from.\n   * @param n    The number of bytes to copy.\n   *\n   * @retval 0  on success; the write pointer is advanced by @arg n.\n   * @retval -1 if the amount of free space following the write pointer\n   *            in the block is less than @arg n. Free space can be checked\n   *            by calling space()."]
    pub unsafe fn copy(
        __this: *mut Self,
        mut buf: *const libc::c_char,
        mut n: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block4copyEPKcm"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                buf: *const libc::c_char,
                n: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Message_Block, buf, n)
    }
    #[doc = "* Copies a 0-terminated character string into this ACE_Message_Block.\n   * The string is copied into the block starting at the current write\n   * pointer. The 0-terminator is included in the copied data.\n   *\n   * @param buf  Pointer to the character string to copy from.\n   *\n   * @retval 0  on success; the write pointer is advanced by the string's\n   *            length, including the 0 terminator.\n   * @retval -1 if the amount of free space following the write pointer\n   *            in the block is less than required to hold the entire string.\n   *            Free space can be checked by calling space()."]
    pub unsafe fn copy_ua964e52317a447cb(
        __this: *mut Self,
        mut buf: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block4copyEPKc"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                buf: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Message_Block, buf)
    }
    /**Normalizes data in the top-level Message_Block to align with the base,
  /// i.e., it "shifts" the data pointed to by <rd_ptr> down to the <base> and
  /// then readjusts <rd_ptr> to point to <base> and <wr_ptr> to point
  /// to <base> + the length of the moved data.  Returns -1 and does
  /// nothing if the <rd_ptr> is > <wr_ptr>, else 0 on success.*/
    pub unsafe fn crunch(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block6crunchEv"]
            fn __ext(__this: *mut ACE_Message_Block) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Message_Block)
    }
    /**Resets the Message Block data to contain nothing, i.e., sets the
  /// read and write pointers to align with the base.*/
    pub unsafe fn reset(__this: *mut Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).rd_ptr_ = ((0) as libc::c_ulong);
                (*__this).wr_ptr_ = ((0) as libc::c_ulong);
            }
            ()
        }
    }
    #[doc = "Access all the allocators in the message block.\n  /// @todo Not sure whether we would need finer control while\n  /// trying to access allocators ie. a method for every allocator.\n  /**\n   * This method returns the allocators only from the first message\n   * block in the chain.\n   *\n   * @param allocator_strategy Strategy used to allocate the\n   *                           underlying buffer\n   *\n   * @param data_block_allocator Strategy used to allocate the\n   *                             underlying data block\n   *\n   * @param message_block_allocator Strategy used to allocate the\n   *                                message block"]
    pub unsafe fn access_allocators(
        __this: *mut Self,
        mut allocator_strategy: *mut *mut ACE_Allocator,
        mut data_block_allocator: *mut *mut ACE_Allocator,
        mut message_block_allocator: *mut *mut ACE_Allocator,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*allocator_strategy) = (*(*__this).data_block_).allocator_strategy_;
                (*data_block_allocator) = (*(*__this).data_block_).data_block_allocator_;
                (*message_block_allocator) = (*__this).message_block_allocator_;
            }
            ()
        }
    }
    #[doc = "Reset all the allocators in the message block.\n  /// @todo Not sure whether we would need finer control while\n  /// trying to reset allocators ie. a method for every allocator.\n  /**\n   * This method resets the allocators in all the message blocks in\n   * the chain."]
    pub unsafe fn reset_allocators(
        __this: *mut Self,
        mut allocator_strategy: *mut ACE_Allocator,
        mut data_block_allocator: *mut ACE_Allocator,
        mut message_block_allocator: *mut ACE_Allocator,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*(*__this).data_block_).allocator_strategy_ = allocator_strategy;
                (*(*__this).data_block_).data_block_allocator_ = data_block_allocator;
                (*__this).message_block_allocator_ = message_block_allocator;
                if ((((!(<ACE_Message_Block>::cont((__this) as *const ACE_Message_Block))
                    .is_null()) as libc::c_int) as libc::c_int) != 0)
                {
                    <ACE_Message_Block>::reset_allocators(
                        (<ACE_Message_Block>::cont((__this) as *const ACE_Message_Block))
                            as *mut ACE_Message_Block,
                        allocator_strategy,
                        data_block_allocator,
                        message_block_allocator,
                    );
                }
            }
            ()
        }
    }
    ///Get message data.
    pub unsafe fn base(__this: *const Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::base(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *const ACE_Data_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set message data (doesn't reallocate).
    pub unsafe fn base_uccb15d65e8c7ce84(
        __this: *mut Self,
        mut msg_data: *mut libc::c_char,
        mut msg_length: libc::c_ulong,
        mut msg_flags: libc::c_ulong,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).rd_ptr_ = ((0) as libc::c_ulong);
                (*__this).wr_ptr_ = ((0) as libc::c_ulong);
                <ACE_Data_Block>::base_u44df93754cea9c80(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *mut ACE_Data_Block,
                    msg_data,
                    msg_length,
                    ((msg_flags) as libc::c_ulong),
                );
            }
            ()
        }
    }
    ///Return a pointer to 1 past the end of the allocated data in a message.
    pub unsafe fn end(__this: *const Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::end(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *const ACE_Data_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Return a pointer to 1 past the end of the allotted data in a message.\n   * Allotted data may be less than allocated data  if a value smaller than\n   * capacity() to is passed to size()."]
    pub unsafe fn mark(__this: *const Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::mark(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *const ACE_Data_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the read pointer.
    pub unsafe fn rd_ptr(__this: *const Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (<ACE_Message_Block>::base((__this) as *const ACE_Message_Block))
                    .wrapping_offset(((*__this).rd_ptr_) as isize);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the read pointer to @a ptr.
    pub unsafe fn rd_ptr_u4c9504a2c1e343b2(
        __this: *mut Self,
        mut new_ptr: *mut libc::c_char,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).rd_ptr_ = (((new_ptr)
                    .offset_from(
                        <ACE_Message_Block>::base((__this) as *const ACE_Message_Block),
                    )) as libc::c_long as libc::c_ulong);
            }
            ()
        }
    }
    ///Set the read pointer ahead @a n bytes.
    pub unsafe fn rd_ptr_u0d0bd23428e552e7(__this: *mut Self, mut n: libc::c_ulong) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __lv = ::core::ptr::addr_of_mut!((* __this).rd_ptr_);
                    unsafe {
                        *__lv = ((((*__lv)) as libc::c_ulong))
                            .wrapping_add((n) as libc::c_ulong);
                        *__lv
                    }
                };
            }
            ()
        }
    }
    ///Get the write pointer.
    pub unsafe fn wr_ptr(__this: *const Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (<ACE_Message_Block>::base((__this) as *const ACE_Message_Block))
                    .wrapping_offset(((*__this).wr_ptr_) as isize);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the write pointer to @a ptr.
    pub unsafe fn wr_ptr_u16d0e11bb2cda475(
        __this: *mut Self,
        mut new_ptr: *mut libc::c_char,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).wr_ptr_ = (((new_ptr)
                    .offset_from(
                        <ACE_Message_Block>::base((__this) as *const ACE_Message_Block),
                    )) as libc::c_long as libc::c_ulong);
            }
            ()
        }
    }
    /**Set the write pointer ahead @a n bytes.  This is used to compute
  /// the <length> of a message.*/
    pub unsafe fn wr_ptr_u53e1f82b9da75e2e(__this: *mut Self, mut n: libc::c_ulong) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __lv = ::core::ptr::addr_of_mut!((* __this).wr_ptr_);
                    unsafe {
                        *__lv = ((((*__lv)) as libc::c_ulong))
                            .wrapping_add((n) as libc::c_ulong);
                        *__lv
                    }
                };
            }
            ()
        }
    }
    ///Get the length of the message
    pub unsafe fn length(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((((*__this).wr_ptr_) as libc::c_ulong))
                    .wrapping_sub(((*__this).rd_ptr_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the length of the message
    pub unsafe fn length_ub925b1861573fb42(__this: *mut Self, mut len: libc::c_ulong) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).wr_ptr_ = ((((*__this).rd_ptr_) as libc::c_ulong))
                    .wrapping_add((len) as libc::c_ulong);
            }
            ()
        }
    }
    /**Get the length of the Message_Blocks, including chained
  /// Message_Blocks.*/
    pub unsafe fn total_length(__this: *const Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Message_Block12total_lengthEv"]
            fn __ext(__this: *const ACE_Message_Block) -> libc::c_ulong;
        }
        __ext(__this as *const ACE_Message_Block)
    }
    /**Get the total number of bytes in all Message_Blocks, including
  /// chained Message_Blocks.*/
    pub unsafe fn total_size(__this: *const Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Message_Block10total_sizeEv"]
            fn __ext(__this: *const ACE_Message_Block) -> libc::c_ulong;
        }
        __ext(__this as *const ACE_Message_Block)
    }
    /**Get the total number of bytes and total length in all
  /// Message_Blocks, including chained Message_Blocks.*/
    pub unsafe fn total_size_and_length(
        __this: *const Self,
        mut mb_size: *mut libc::c_ulong,
        mut mb_length: *mut libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Message_Block21total_size_and_lengthERmS0_"]
            fn __ext(
                __this: *const ACE_Message_Block,
                mb_size: *mut libc::c_ulong,
                mb_length: *mut libc::c_ulong,
            );
        }
        __ext(__this as *const ACE_Message_Block, mb_size, mb_length)
    }
    /**Get the number of bytes in the top-level Message_Block (i.e.,
  /// does not consider the bytes in chained Message_Blocks).*/
    pub unsafe fn size(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::size(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *const ACE_Data_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Set the number of bytes in the top-level Message_Block,\n   * reallocating space if necessary.  However, the @c rd_ptr_ and\n   * @c wr_ptr_ remain at the original offsets into the buffer, even if\n   * it is reallocated.  Returns 0 if successful, else -1."]
    pub unsafe fn size_uc20a7745501f5111(
        __this: *mut Self,
        mut length: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block4sizeEm"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                length: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Message_Block, length)
    }
    /**Get the number of allocated bytes in all Message_Block, including
  /// chained Message_Blocks.*/
    pub unsafe fn total_capacity(__this: *const Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Message_Block14total_capacityEv"]
            fn __ext(__this: *const ACE_Message_Block) -> libc::c_ulong;
        }
        __ext(__this as *const ACE_Message_Block)
    }
    ///Get the number of allocated bytes in the top-level Message_Block.
    pub unsafe fn capacity(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::capacity(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *const ACE_Data_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Get the number of bytes available after the <wr_ptr_> in the
  /// top-level Message_Block.*/
    pub unsafe fn space(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((<ACE_Message_Block>::mark(
                    (__this) as *const ACE_Message_Block,
                ))
                    .offset_from(
                        <ACE_Message_Block>::wr_ptr((__this) as *const ACE_Message_Block),
                    )) as libc::c_long as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Get a pointer to the data block. Note that the ACE_Message_Block\n   * still references the block; this call does not change the reference\n   * count."]
    pub unsafe fn data_block(__this: *const Self) -> *mut ACE_Data_Block {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).data_block_) as *mut ACE_Data_Block);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Set a new data block pointer. The original ACE_Data_Block is released\n   * as a result of this call. If you need to keep the original block, call\n   * <replace_data_block> instead. Upon return, this ACE_Message_Block\n   * holds a pointer to the new ACE_Data_Block, taking over the reference\n   * you held on it prior to the call."]
    pub unsafe fn data_block_u9cc47bebe9c9dd81(
        __this: *mut Self,
        mut _anon_0: *mut ACE_Data_Block,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block10data_blockEP14ACE_Data_Block"]
            fn __ext(__this: *mut ACE_Message_Block, _anon_0: *mut ACE_Data_Block);
        }
        __ext(__this as *mut ACE_Message_Block, _anon_0)
    }
    /**Set a new data block pointer. A pointer to the original ACE_Data_Block
  /// is returned, and not released (as it is with <data_block>).*/
    pub unsafe fn replace_data_block(
        __this: *mut Self,
        mut db: *mut ACE_Data_Block,
    ) -> *mut ACE_Data_Block {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut old: *mut ACE_Data_Block = (*__this).data_block_;
                (*__this).data_block_ = db;
                if ((((!(db).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    <ACE_Message_Block>::rd_ptr_u4c9504a2c1e343b2(
                        (__this) as *mut ACE_Message_Block,
                        <ACE_Data_Block>::base(
                            (<ACE_Message_Block>::data_block(
                                (__this) as *const ACE_Message_Block,
                            )) as *const ACE_Data_Block,
                        ),
                    );
                    <ACE_Message_Block>::wr_ptr_u16d0e11bb2cda475(
                        (__this) as *mut ACE_Message_Block,
                        <ACE_Data_Block>::base(
                            (<ACE_Message_Block>::data_block(
                                (__this) as *const ACE_Message_Block,
                            )) as *const ACE_Data_Block,
                        ),
                    );
                }
                return old;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the continuation field.
    pub unsafe fn cont(__this: *const Self) -> *mut ACE_Message_Block {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).cont_) as *mut ACE_Message_Block);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the continuation field.
    pub unsafe fn cont_u9515391441f35afa(
        __this: *mut Self,
        mut cont_msg: *mut ACE_Message_Block,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).cont_ = cont_msg;
            }
            ()
        }
    }
    ///Get link to next message.
    pub unsafe fn next(__this: *const Self) -> *mut ACE_Message_Block {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).next_) as *mut ACE_Message_Block);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set link to next message.
    pub unsafe fn next_u2a153200e1956933(
        __this: *mut Self,
        mut next_msg: *mut ACE_Message_Block,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).next_ = next_msg;
            }
            ()
        }
    }
    ///Get link to prev message.
    pub unsafe fn prev(__this: *const Self) -> *mut ACE_Message_Block {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).prev_) as *mut ACE_Message_Block);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set link to prev message.
    pub unsafe fn prev_u64dba7576981ae73(
        __this: *mut Self,
        mut next_msg: *mut ACE_Message_Block,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).prev_ = next_msg;
            }
            ()
        }
    }
    ///Get the locking strategy.
    pub unsafe fn locking_strategy(__this: *mut Self) -> *mut ACE_Lock {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Data_Block>::locking_strategy(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *mut ACE_Data_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set a new locking strategy and return the hold one.
    pub unsafe fn locking_strategy_u09153c79a0e33081(
        __this: *mut Self,
        mut nls: *mut ACE_Lock,
    ) -> *mut ACE_Lock {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ols: *mut ACE_Lock = <ACE_Data_Block>::locking_strategy(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *mut ACE_Data_Block,
                );
                <ACE_Data_Block>::locking_strategy_u842da8c0d677dd6d(
                    (<ACE_Message_Block>::data_block(
                        (__this) as *const ACE_Message_Block,
                    )) as *mut ACE_Data_Block,
                    nls,
                );
                return ols;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the current reference count.
    pub unsafe fn reference_count(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return if (!(<ACE_Message_Block>::data_block(
                    (__this) as *const ACE_Message_Block,
                ))
                    .is_null())
                {
                    <ACE_Data_Block>::reference_count(
                        (<ACE_Message_Block>::data_block(
                            (__this) as *const ACE_Message_Block,
                        )) as *const ACE_Data_Block,
                    )
                } else {
                    0
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Message_Block4dumpEv"]
            fn __ext(__this: *const ACE_Message_Block);
        }
        __ext(__this as *const ACE_Message_Block)
    }
    ///Perform the actual initialization.
    pub unsafe fn new_at_u5eb85f81d7c78c95(
        __this: *mut Self,
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Message_Block,
        mut __a3: *const libc::c_char,
        mut __a4: *mut ACE_Allocator,
        mut __a5: *mut ACE_Lock,
        mut __a6: libc::c_ulong,
        mut __a7: libc::c_ulong,
        mut __a8: *const ACE_Time_Value,
        mut __a9: *const ACE_Time_Value,
        mut __a10: *mut ACE_Data_Block,
        mut __a11: *mut ACE_Allocator,
        mut __a12: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_BlockC1EmiPS_PKcP13ACE_AllocatorP8ACE_LockmmRK14ACE_Time_ValueS9_P14ACE_Data_BlockS4_S4_"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                __a0: libc::c_ulong,
                __a1: libc::c_int,
                __a2: *mut ACE_Message_Block,
                __a3: *const libc::c_char,
                __a4: *mut ACE_Allocator,
                __a5: *mut ACE_Lock,
                __a6: libc::c_ulong,
                __a7: libc::c_ulong,
                __a8: *const ACE_Time_Value,
                __a9: *const ACE_Time_Value,
                __a10: *mut ACE_Data_Block,
                __a11: *mut ACE_Allocator,
                __a12: *mut ACE_Allocator,
            );
        }
        __ext(
            __this as *mut ACE_Message_Block,
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
            __a7,
            __a8,
            __a9,
            __a10,
            __a11,
            __a12,
        )
    }
    pub unsafe fn new_u5eb85f81d7c78c95(
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Message_Block,
        mut __a3: *const libc::c_char,
        mut __a4: *mut ACE_Allocator,
        mut __a5: *mut ACE_Lock,
        mut __a6: libc::c_ulong,
        mut __a7: libc::c_ulong,
        mut __a8: *const ACE_Time_Value,
        mut __a9: *const ACE_Time_Value,
        mut __a10: *mut ACE_Data_Block,
        mut __a11: *mut ACE_Allocator,
        mut __a12: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u5eb85f81d7c78c95(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
            __a7,
            __a8,
            __a9,
            __a10,
            __a11,
            __a12,
        );
        __obj
    }
    /**Internal release implementation
  /// Returns 1 if the data block has to be destroyed.*/
    pub unsafe fn release_i(__this: *mut Self, mut lock: *mut ACE_Lock) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block9release_iEP8ACE_Lock"]
            fn __ext(__this: *mut ACE_Message_Block, lock: *mut ACE_Lock) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Message_Block, lock)
    }
    ///Perform the actual initialization.
    pub unsafe fn init_i(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut r#type: libc::c_int,
        mut cont: *mut ACE_Message_Block,
        mut data: *const libc::c_char,
        mut allocator_strategy: *mut ACE_Allocator,
        mut locking_strategy: *mut ACE_Lock,
        mut flags: libc::c_ulong,
        mut priority: libc::c_ulong,
        mut execution_time: *const ACE_Time_Value,
        mut deadline_time: *const ACE_Time_Value,
        mut db: *mut ACE_Data_Block,
        mut data_block_allocator: *mut ACE_Allocator,
        mut message_block_allocator: *mut ACE_Allocator,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_Block6init_iEmiPS_PKcP13ACE_AllocatorP8ACE_LockmmRK14ACE_Time_ValueS9_P14ACE_Data_BlockS4_S4_"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                size: libc::c_ulong,
                r#type: libc::c_int,
                cont: *mut ACE_Message_Block,
                data: *const libc::c_char,
                allocator_strategy: *mut ACE_Allocator,
                locking_strategy: *mut ACE_Lock,
                flags: libc::c_ulong,
                priority: libc::c_ulong,
                execution_time: *const ACE_Time_Value,
                deadline_time: *const ACE_Time_Value,
                db: *mut ACE_Data_Block,
                data_block_allocator: *mut ACE_Allocator,
                message_block_allocator: *mut ACE_Allocator,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_Message_Block,
            size,
            r#type,
            cont,
            data,
            allocator_strategy,
            locking_strategy,
            flags,
            priority,
            execution_time,
            deadline_time,
            db,
            data_block_allocator,
            message_block_allocator,
        )
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Message_Block,
    ) -> *mut ACE_Message_Block {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_BlockaSERKS_"]
            fn __ext(
                __this: *mut ACE_Message_Block,
                _anon_0: *const ACE_Message_Block,
            ) -> *mut ACE_Message_Block;
        }
        __ext(__this as *mut ACE_Message_Block, _anon_0)
    }
    pub unsafe fn new_at_ub38592127936cf58(
        __this: *mut Self,
        mut __a0: *const ACE_Message_Block,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Message_BlockC1ERKS_"]
            fn __ext(__this: *mut ACE_Message_Block, __a0: *const ACE_Message_Block);
        }
        __ext(__this as *mut ACE_Message_Block, __a0)
    }
    pub unsafe fn new_ub38592127936cf58(mut __a0: *const ACE_Message_Block) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ub38592127936cf58(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_CDR {
    #[doc = "* Do byte swapping for each basic IDL type size.  There exist only\n   * routines to put byte, halfword (2 bytes), word (4 bytes),\n   * doubleword (8 bytes) and quadword (16 byte); because those are\n   * the IDL basic type sizes."]
    pub unsafe fn swap_2(mut orig: *const libc::c_char, mut target: *mut libc::c_char) {
        unsafe {
            {
                (*((target as *mut libc::c_ushort))) = ((__bswap_16(
                    (((*((orig as *const libc::c_ushort)))) as libc::c_ushort),
                )) as libc::c_ushort);
            }
            ()
        }
    }
    pub unsafe fn swap_4(mut orig: *const libc::c_char, mut target: *mut libc::c_char) {
        unsafe {
            {
                (*((target as *mut libc::c_uint))) = ((__bswap_32(
                    (((*((orig as *const libc::c_uint)))) as libc::c_uint),
                )) as libc::c_uint);
            }
            ()
        }
    }
    pub unsafe fn swap_8(mut orig: *const libc::c_char, mut target: *mut libc::c_char) {
        unsafe {
            {
                (*((target as *mut libc::c_ulong))) = ((__bswap_64(
                    (((*((orig as *const libc::c_ulong)))) as libc::c_ulong),
                )) as libc::c_ulong);
            }
            ()
        }
    }
    pub unsafe fn swap_16(mut orig: *const libc::c_char, mut target: *mut libc::c_char) {
        unsafe {
            {
                <ACE_CDR>::swap_8((orig).wrapping_offset((8) as isize), target);
                <ACE_CDR>::swap_8(orig, (target).wrapping_offset((8) as isize));
            }
            ()
        }
    }
    pub unsafe fn swap_2_array(
        mut orig: *const libc::c_char,
        mut target: *mut libc::c_char,
        mut n: libc::c_ulong,
    ) {
        unsafe {
            {
                let mut o4: *const libc::c_char = ((ACE_ptr_align_binary(
                    orig,
                    ((4) as libc::c_ulong),
                )) as *const libc::c_char);
                if ((((((orig) as *const u8)) != (((o4) as *const u8))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    <ACE_CDR>::swap_2(orig, target);
                    {
                        orig = (orig).wrapping_offset((2) as isize);
                        orig
                    };
                    {
                        target = (target).wrapping_offset((2) as isize);
                        target
                    };
                    {
                        let __lv = &mut (n);
                        *__lv = (*__lv).wrapping_sub(1);
                        *__lv
                    };
                }
                if (((((n as libc::c_ulong)) == (((0) as libc::c_ulong))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return;
                }
                let mut end: *const libc::c_char = (((orig)
                    .wrapping_offset(
                        ((((2) as libc::c_ulong))
                            .wrapping_mul(
                                (((((n) as libc::c_ulong)) & ((((!(3)))) as libc::c_ulong)))
                                    as libc::c_ulong,
                            )) as isize,
                    )) as *const libc::c_char);
                if ((((((target) as *const u8))
                    == (((ACE_ptr_align_binary(
                        ((target) as *const libc::c_char),
                        ((4) as libc::c_ulong),
                    )) as *const u8))) as libc::c_int as libc::c_int) != 0)
                {
                    'while_0: loop {
                        if !(((((((orig) as *const u8)) < (((end) as *const u8)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break 'while_0;
                        }
                        'cont_0: loop {
                            {
                                {
                                    let mut a: libc::c_uint = (((*((orig
                                        as *const libc::c_uint)))) as libc::c_uint);
                                    let mut b: libc::c_uint = (((*(((orig)
                                        .wrapping_offset((4) as isize) as *const libc::c_uint))))
                                        as libc::c_uint);
                                    unsafe {
                                        ::core::arch::asm!(
                                            "bswap {0:e}", inout(reg) a, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "bswap {0:e}", inout(reg) b, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "rol $16, {0:e}", inout(reg) a, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "rol $16, {0:e}", inout(reg) b, options(att_syntax)
                                        );
                                    }
                                    (*((target as *mut libc::c_uint))) = a;
                                    (*(((target).wrapping_offset((4) as isize)
                                        as *mut libc::c_uint))) = b;
                                    {
                                        orig = (orig).wrapping_offset((8) as isize);
                                        orig
                                    };
                                    {
                                        target = (target).wrapping_offset((8) as isize);
                                        target
                                    };
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                    }
                } else {
                    'while_1: loop {
                        if !(((((((orig) as *const u8)) < (((end) as *const u8)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break 'while_1;
                        }
                        'cont_1: loop {
                            {
                                {
                                    let mut a: libc::c_uint = (((*((orig
                                        as *const libc::c_uint)))) as libc::c_uint);
                                    let mut b: libc::c_uint = (((*(((orig)
                                        .wrapping_offset((4) as isize) as *const libc::c_uint))))
                                        as libc::c_uint);
                                    unsafe {
                                        ::core::arch::asm!(
                                            "bswap {0:e}", inout(reg) a, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "bswap {0:e}", inout(reg) b, options(att_syntax)
                                        );
                                    }
                                    (*(((target).wrapping_offset((2) as isize)
                                        as *mut libc::c_ushort))) = (((((a) as libc::c_uint))
                                        & ((0xffff) as libc::c_uint)) as libc::c_ushort);
                                    (*(((target).wrapping_offset((6) as isize)
                                        as *mut libc::c_ushort))) = (((((b) as libc::c_uint))
                                        & ((0xffff) as libc::c_uint)) as libc::c_ushort);
                                    unsafe {
                                        ::core::arch::asm!(
                                            "shrl $16, {0:e}", inout(reg) a, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "shrl $16, {0:e}", inout(reg) b, options(att_syntax)
                                        );
                                    }
                                    (*(((target).wrapping_offset((0) as isize)
                                        as *mut libc::c_ushort))) = (((((a) as libc::c_uint))
                                        & ((0xffff) as libc::c_uint)) as libc::c_ushort);
                                    (*(((target).wrapping_offset((4) as isize)
                                        as *mut libc::c_ushort))) = (((((b) as libc::c_uint))
                                        & ((0xffff) as libc::c_uint)) as libc::c_ushort);
                                    {
                                        orig = (orig).wrapping_offset((8) as isize);
                                        orig
                                    };
                                    {
                                        target = (target).wrapping_offset((8) as isize);
                                        target
                                    };
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_1;
                        }
                    }
                }
                'switch_0: {
                    let __c: libc::c_ulong = (((n) as libc::c_ulong))
                        & ((3) as libc::c_ulong);
                    #[allow(unused_assignments)]
                    let mut __m: u32 = 3;
                    {
                        let __cv_0: libc::c_ulong = (3 as libc::c_ulong);
                        if __c == __cv_0 {
                            __m = 0;
                        }
                    }
                    if __m == 3 {
                        let __cv_1: libc::c_ulong = (2 as libc::c_ulong);
                        if __c == __cv_1 {
                            __m = 1;
                        }
                    }
                    if __m == 3 {
                        let __cv_2: libc::c_ulong = (1 as libc::c_ulong);
                        if __c == __cv_2 {
                            __m = 2;
                        }
                    }
                    if __m <= 0 {
                        <ACE_CDR>::swap_2(orig, target);
                        {
                            orig = (orig).wrapping_offset((2) as isize);
                            orig
                        };
                        {
                            target = (target).wrapping_offset((2) as isize);
                            target
                        };
                    }
                    if __m <= 1 {
                        <ACE_CDR>::swap_2(orig, target);
                        {
                            orig = (orig).wrapping_offset((2) as isize);
                            orig
                        };
                        {
                            target = (target).wrapping_offset((2) as isize);
                            target
                        };
                    }
                    if __m <= 2 {
                        <ACE_CDR>::swap_2(orig, target);
                    }
                    #[allow(unreachable_code)] break 'switch_0;
                }
            }
            ()
        }
    }
    pub unsafe fn swap_4_array(
        mut orig: *const libc::c_char,
        mut target: *mut libc::c_char,
        mut n: libc::c_ulong,
    ) {
        unsafe {
            {
                let mut o8: *const libc::c_char = ((ACE_ptr_align_binary(
                    orig,
                    ((8) as libc::c_ulong),
                )) as *const libc::c_char);
                if ((((((orig) as *const u8)) != (((o8) as *const u8))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    <ACE_CDR>::swap_4(orig, target);
                    {
                        orig = (orig).wrapping_offset((4) as isize);
                        orig
                    };
                    {
                        target = (target).wrapping_offset((4) as isize);
                        target
                    };
                    {
                        let __lv = &mut (n);
                        *__lv = (*__lv).wrapping_sub(1);
                        *__lv
                    };
                }
                if (((((n as libc::c_ulong)) == (((0) as libc::c_ulong))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return;
                }
                let mut end: *const libc::c_char = (((orig)
                    .wrapping_offset(
                        ((((4) as libc::c_ulong))
                            .wrapping_mul(
                                (((((n) as libc::c_ulong)) & ((((!(3)))) as libc::c_ulong)))
                                    as libc::c_ulong,
                            )) as isize,
                    )) as *const libc::c_char);
                if ((((((target) as *const u8))
                    == (((ACE_ptr_align_binary(
                        ((target) as *const libc::c_char),
                        ((8) as libc::c_ulong),
                    )) as *const u8))) as libc::c_int as libc::c_int) != 0)
                {
                    'while_0: loop {
                        if !(((((((orig) as *const u8)) < (((end) as *const u8)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break 'while_0;
                        }
                        'cont_0: loop {
                            {
                                {
                                    let mut a: libc::c_ulong = (((*((orig
                                        as *const libc::c_long)))) as libc::c_ulong);
                                    let mut b: libc::c_ulong = (((*(((orig)
                                        .wrapping_offset((8) as isize) as *const libc::c_long))))
                                        as libc::c_ulong);
                                    unsafe {
                                        ::core::arch::asm!(
                                            "bswapq {0}", inout(reg) a, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "bswapq {0}", inout(reg) b, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "rol $32, {0}", inout(reg) a, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "rol $32, {0}", inout(reg) b, options(att_syntax)
                                        );
                                    }
                                    (*((target as *mut libc::c_long))) = ((a) as libc::c_long);
                                    (*(((target).wrapping_offset((8) as isize)
                                        as *mut libc::c_long))) = ((b) as libc::c_long);
                                    {
                                        orig = (orig).wrapping_offset((16) as isize);
                                        orig
                                    };
                                    {
                                        target = (target).wrapping_offset((16) as isize);
                                        target
                                    };
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                    }
                } else {
                    'while_1: loop {
                        if !(((((((orig) as *const u8)) < (((end) as *const u8)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break 'while_1;
                        }
                        'cont_1: loop {
                            {
                                {
                                    let mut a: libc::c_ulong = (((*((orig
                                        as *const libc::c_long)))) as libc::c_ulong);
                                    let mut b: libc::c_ulong = (((*(((orig)
                                        .wrapping_offset((8) as isize) as *const libc::c_long))))
                                        as libc::c_ulong);
                                    unsafe {
                                        ::core::arch::asm!(
                                            "bswapq {0}", inout(reg) a, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "bswapq {0}", inout(reg) b, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "rol $32, {0}", inout(reg) a, options(att_syntax)
                                        );
                                    }
                                    unsafe {
                                        ::core::arch::asm!(
                                            "rol $32, {0}", inout(reg) b, options(att_syntax)
                                        );
                                    }
                                    let mut c1: libc::c_uint = ((((a) as libc::c_ulong))
                                        .wrapping_shr((32) as u32) as libc::c_uint);
                                    let mut c2: libc::c_uint = (((((a) as libc::c_ulong))
                                        & ((0xffffffffu32) as libc::c_ulong)) as libc::c_uint);
                                    let mut c3: libc::c_uint = ((((b) as libc::c_ulong))
                                        .wrapping_shr((32) as u32) as libc::c_uint);
                                    let mut c4: libc::c_uint = (((((b) as libc::c_ulong))
                                        & ((0xffffffffu32) as libc::c_ulong)) as libc::c_uint);
                                    (*(((target).wrapping_offset((0) as isize)
                                        as *mut libc::c_uint))) = c2;
                                    (*(((target).wrapping_offset((4) as isize)
                                        as *mut libc::c_uint))) = c1;
                                    (*(((target).wrapping_offset((8) as isize)
                                        as *mut libc::c_uint))) = c4;
                                    (*(((target).wrapping_offset((12) as isize)
                                        as *mut libc::c_uint))) = c3;
                                    {
                                        orig = (orig).wrapping_offset((16) as isize);
                                        orig
                                    };
                                    {
                                        target = (target).wrapping_offset((16) as isize);
                                        target
                                    };
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_1;
                        }
                    }
                }
                'switch_0: {
                    let __c: libc::c_ulong = (((n) as libc::c_ulong))
                        & ((3) as libc::c_ulong);
                    #[allow(unused_assignments)]
                    let mut __m: u32 = 3;
                    {
                        let __cv_0: libc::c_ulong = (3 as libc::c_ulong);
                        if __c == __cv_0 {
                            __m = 0;
                        }
                    }
                    if __m == 3 {
                        let __cv_1: libc::c_ulong = (2 as libc::c_ulong);
                        if __c == __cv_1 {
                            __m = 1;
                        }
                    }
                    if __m == 3 {
                        let __cv_2: libc::c_ulong = (1 as libc::c_ulong);
                        if __c == __cv_2 {
                            __m = 2;
                        }
                    }
                    if __m <= 0 {
                        <ACE_CDR>::swap_4(orig, target);
                        {
                            orig = (orig).wrapping_offset((4) as isize);
                            orig
                        };
                        {
                            target = (target).wrapping_offset((4) as isize);
                            target
                        };
                    }
                    if __m <= 1 {
                        <ACE_CDR>::swap_4(orig, target);
                        {
                            orig = (orig).wrapping_offset((4) as isize);
                            orig
                        };
                        {
                            target = (target).wrapping_offset((4) as isize);
                            target
                        };
                    }
                    if __m <= 2 {
                        <ACE_CDR>::swap_4(orig, target);
                    }
                    #[allow(unreachable_code)] break 'switch_0;
                }
            }
            ()
        }
    }
    pub unsafe fn swap_8_array(
        mut orig: *const libc::c_char,
        mut target: *mut libc::c_char,
        mut n: libc::c_ulong,
    ) {
        unsafe {
            {
                let mut end: *const libc::c_char = (((orig)
                    .wrapping_offset(
                        ((((8) as libc::c_ulong)).wrapping_mul((n) as libc::c_ulong))
                            as isize,
                    )) as *const libc::c_char);
                'while_0: loop {
                    if !(((((((orig) as *const u8)) < (((end) as *const u8)))
                        as libc::c_int as libc::c_int) != 0))
                    {
                        break 'while_0;
                    }
                    'cont_0: loop {
                        {
                            {
                                <ACE_CDR>::swap_8(orig, target);
                                {
                                    orig = (orig).wrapping_offset((8) as isize);
                                    orig
                                };
                                {
                                    target = (target).wrapping_offset((8) as isize);
                                    target
                                };
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                }
            }
            ()
        }
    }
    pub unsafe fn swap_16_array(
        mut orig: *const libc::c_char,
        mut target: *mut libc::c_char,
        mut n: libc::c_ulong,
    ) {
        unsafe {
            {
                let mut end: *const libc::c_char = (((orig)
                    .wrapping_offset(
                        ((((16) as libc::c_ulong)).wrapping_mul((n) as libc::c_ulong))
                            as isize,
                    )) as *const libc::c_char);
                'while_0: loop {
                    if !(((((((orig) as *const u8)) < (((end) as *const u8)))
                        as libc::c_int as libc::c_int) != 0))
                    {
                        break 'while_0;
                    }
                    'cont_0: loop {
                        {
                            {
                                <ACE_CDR>::swap_16(orig, target);
                                {
                                    orig = (orig).wrapping_offset((16) as isize);
                                    orig
                                };
                                {
                                    target = (target).wrapping_offset((16) as isize);
                                    target
                                };
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                }
            }
            ()
        }
    }
    /**Align the message block to ACE_CDR::MAX_ALIGNMENT,
  /// set by the CORBA spec at 8 bytes.*/
    pub unsafe fn mb_align(mut mb: *mut ACE_Message_Block) {
        unsafe {
            {
                let mut start: *mut libc::c_char = ((ACE_ptr_align_binary(
                    ((<ACE_Message_Block>::base((mb) as *const ACE_Message_Block))
                        as *const libc::c_char),
                    (((8 as libc::c_int)) as libc::c_ulong),
                )) as *mut libc::c_char);
                <ACE_Message_Block>::rd_ptr_u4c9504a2c1e343b2(
                    (mb) as *mut ACE_Message_Block,
                    ((start) as *mut libc::c_char),
                );
                <ACE_Message_Block>::wr_ptr_u16d0e11bb2cda475(
                    (mb) as *mut ACE_Message_Block,
                    ((start) as *mut libc::c_char),
                );
            }
            ()
        }
    }
    #[doc = "* Compute the size of the smallest buffer that can contain at least\n   * @a minsize bytes.\n   * To understand how a \"best fit\" is computed look at the\n   * algorithm in the code.\n   * Basically the buffers grow exponentially, up to a certain point,\n   * then the buffer size grows linearly.\n   * The advantage of this algorithm is that is rapidly grows to a\n   * large value, but does not explode at the end."]
    pub unsafe fn first_size(mut minsize: libc::c_ulong) -> libc::c_ulong {
        unsafe {
            {
                if (((((minsize as libc::c_ulong)) == (((0) as libc::c_ulong)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (((512 as libc::c_int)) as libc::c_ulong);
                }
                let mut newsize: libc::c_ulong = (((512 as libc::c_int))
                    as libc::c_ulong);
                'while_0: loop {
                    if !((((((newsize as libc::c_ulong))
                        < (((minsize) as libc::c_ulong))) as libc::c_int as libc::c_int)
                        != 0))
                    {
                        break 'while_0;
                    }
                    'cont_0: loop {
                        {
                            {
                                if (((((newsize as libc::c_ulong))
                                    < ((((65536 as libc::c_int)) as libc::c_ulong)))
                                    as libc::c_int as libc::c_int) != 0)
                                {
                                    {
                                        newsize = (((newsize) as libc::c_ulong))
                                            .wrapping_shl((1) as u32);
                                        newsize
                                    };
                                } else {
                                    {
                                        newsize = (((newsize) as libc::c_ulong))
                                            .wrapping_add(((65536 as libc::c_int)) as libc::c_ulong);
                                        newsize
                                    };
                                }
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                }
                return newsize;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Compute not the smallest, but the second smallest buffer that
  /// will fir @a minsize bytes.*/
    pub unsafe fn next_size(mut minsize: libc::c_ulong) -> libc::c_ulong {
        unsafe {
            {
                let mut newsize: libc::c_ulong = <ACE_CDR>::first_size(minsize);
                if (((((newsize as libc::c_ulong)) == (((minsize) as libc::c_ulong)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    if (((((newsize as libc::c_ulong))
                        < ((((65536 as libc::c_int)) as libc::c_ulong))) as libc::c_int
                        as libc::c_int) != 0)
                    {
                        {
                            newsize = (((newsize) as libc::c_ulong))
                                .wrapping_shl((1) as u32);
                            newsize
                        };
                    } else {
                        {
                            newsize = (((newsize) as libc::c_ulong))
                                .wrapping_add(((65536 as libc::c_int)) as libc::c_ulong);
                            newsize
                        };
                    }
                }
                return newsize;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Increase the capacity of mb to contain at least @a minsize bytes.\n   * If @a minsize is zero the size is increased by an amount at least\n   * large enough to contain any of the basic IDL types.\n   * @retval -1 Failure\n   * @retval 0 Success."]
    pub unsafe fn grow(
        mut mb: *mut ACE_Message_Block,
        mut minsize: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            {
                let mut newsize: libc::c_ulong = <ACE_CDR>::first_size(
                    (((minsize) as libc::c_ulong))
                        .wrapping_add(((8 as libc::c_int)) as libc::c_ulong),
                );
                if (((((newsize as libc::c_ulong))
                    <= (((<ACE_Message_Block>::size((mb) as *const ACE_Message_Block))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    return 0;
                }
                let mut db: *mut ACE_Data_Block = {
                    let __obj: *mut ACE_Data_Block = (<ACE_Message_Block>::data_block(
                        (mb) as *const ACE_Message_Block,
                    )) as *mut ACE_Data_Block;
                    let __vt: *const __Vtbl_ue8090ae954631e85 = *(__obj
                        as *const *const __Vtbl_ue8090ae954631e85);
                    ((*__vt)
                        .vfn_u332d56177ae0a20a)(__obj, ((0) as libc::c_ulong), newsize)
                };
                if (((((db).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return (-((1) as libc::c_int));
                }
                let mut mb_len: libc::c_ulong = <ACE_Message_Block>::length(
                    (mb) as *const ACE_Message_Block,
                );
                let mut start: *mut libc::c_char = ACE_ptr_align_binary(
                    ((<ACE_Data_Block>::base((db) as *const ACE_Data_Block))
                        as *const libc::c_char),
                    (((8 as libc::c_int)) as libc::c_ulong),
                );
                ACE_OS::memcpy_u6033eb81edaf9212(
                    ((start) as *mut libc::c_void),
                    ((<ACE_Message_Block>::rd_ptr((mb) as *const ACE_Message_Block))
                        as *const libc::c_void),
                    mb_len,
                );
                <ACE_Message_Block>::data_block_u9cc47bebe9c9dd81(
                    (mb) as *mut ACE_Message_Block,
                    db,
                );
                <ACE_Message_Block>::rd_ptr_u4c9504a2c1e343b2(
                    (mb) as *mut ACE_Message_Block,
                    start,
                );
                <ACE_Message_Block>::wr_ptr_u16d0e11bb2cda475(
                    (mb) as *mut ACE_Message_Block,
                    (((start).wrapping_offset((mb_len) as isize)) as *mut libc::c_char),
                );
                <ACE_Message_Block>::clr_self_flags(
                    (mb) as *mut ACE_Message_Block,
                    (((1 as libc::c_int)) as libc::c_ulong),
                );
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Copy a message block chain into a single message block,\n   * preserving the alignment of the first message block of the\n   * original stream, not the following message blocks.\n   * @retval -1 Failure\n   * @retval 0 Success."]
    pub unsafe fn consolidate(
        mut dst: *mut ACE_Message_Block,
        mut src: *const ACE_Message_Block,
    ) -> libc::c_int {
        unsafe {
            {
                if (((((src).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return 0;
                }
                let mut newsize: libc::c_ulong = ((<ACE_CDR>::first_size(
                    (((<ACE_CDR>::total_length(src, ((0) as *const ACE_Message_Block)))
                        as libc::c_ulong))
                        .wrapping_add(((8 as libc::c_int)) as libc::c_ulong),
                )) as libc::c_ulong);
                if (((((<ACE_Message_Block>::size_uc20a7745501f5111(
                    (dst) as *mut ACE_Message_Block,
                    ((newsize) as libc::c_ulong),
                ) as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                let mut srcalign: libc::c_long = ((((<ACE_Message_Block>::rd_ptr(
                    (src) as *const ACE_Message_Block,
                ) as libc::c_long)) as libc::c_long))
                    % (((8 as libc::c_int)) as libc::c_long);
                let mut dstalign: libc::c_long = ((((<ACE_Message_Block>::rd_ptr(
                    (dst) as *const ACE_Message_Block,
                ) as libc::c_long)) as libc::c_long))
                    % (((8 as libc::c_int)) as libc::c_long);
                let mut offset: libc::c_long = (((srcalign) as libc::c_long))
                    .wrapping_sub((dstalign) as libc::c_long);
                if (((((offset as libc::c_long)) < (((0) as libc::c_long)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    {
                        offset = (((offset) as libc::c_long))
                            .wrapping_add(((8 as libc::c_int)) as libc::c_long);
                        offset
                    };
                }
                <ACE_Message_Block>::rd_ptr_u0d0bd23428e552e7(
                    (dst) as *mut ACE_Message_Block,
                    (offset as libc::c_ulong),
                );
                <ACE_Message_Block>::wr_ptr_u16d0e11bb2cda475(
                    (dst) as *mut ACE_Message_Block,
                    <ACE_Message_Block>::rd_ptr((dst) as *const ACE_Message_Block),
                );
                {
                    let mut i: *const ACE_Message_Block = src;
                    'for_0: loop {
                        if !(((((!(i).is_null()) as libc::c_int) as libc::c_int) != 0)) {
                            break;
                        }
                        'cont_0: loop {
                            {
                                {
                                    if ((((((<ACE_Message_Block>::wr_ptr(
                                        (dst) as *const ACE_Message_Block,
                                    )) as *const u8))
                                        != (((<ACE_Message_Block>::rd_ptr(
                                            (i) as *const ACE_Message_Block,
                                        )) as *const u8))) as libc::c_int as libc::c_int) != 0)
                                    {
                                        <ACE_Message_Block>::copy(
                                            (dst) as *mut ACE_Message_Block,
                                            ((<ACE_Message_Block>::rd_ptr(
                                                (i) as *const ACE_Message_Block,
                                            )) as *const libc::c_char),
                                            <ACE_Message_Block>::length((i) as *const ACE_Message_Block),
                                        );
                                    } else {
                                        <ACE_Message_Block>::wr_ptr_u53e1f82b9da75e2e(
                                            (dst) as *mut ACE_Message_Block,
                                            <ACE_Message_Block>::length((i) as *const ACE_Message_Block),
                                        );
                                    }
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                        {
                            let __v = (<ACE_Message_Block>::cont(
                                (i) as *const ACE_Message_Block,
                            )) as *const ACE_Message_Block;
                            i = __v;
                            __v
                        };
                    }
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn total_length(
        mut begin: *const ACE_Message_Block,
        mut end: *const ACE_Message_Block,
    ) -> libc::c_ulong {
        unsafe {
            {
                let mut l: libc::c_ulong = ((0) as libc::c_ulong);
                {
                    let mut i: *const ACE_Message_Block = begin;
                    'for_0: loop {
                        if !(((((((i) as *const u8)) != (((end) as *const u8)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break;
                        }
                        'cont_0: loop {
                            {
                                {
                                    l = (((l) as libc::c_ulong))
                                        .wrapping_add(
                                            (<ACE_Message_Block>::length(
                                                (i) as *const ACE_Message_Block,
                                            )) as libc::c_ulong,
                                        );
                                    l
                                };
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                        {
                            let __v = (<ACE_Message_Block>::cont(
                                (i) as *const ACE_Message_Block,
                            )) as *const ACE_Message_Block;
                            i = __v;
                            __v
                        };
                    }
                }
                return l;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Allocator {
    ///Get pointer to a default ACE_Allocator.
    pub unsafe fn instance() -> *mut ACE_Allocator {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Allocator8instanceEv"]
            fn __ext() -> *mut ACE_Allocator;
        }
        __ext()
    }
    /**Set pointer to a process-wide ACE_Allocator and return existing
  /// pointer.*/
    pub unsafe fn instance_uf32e88f2148ecbfa(
        mut _anon_0: *mut ACE_Allocator,
    ) -> *mut ACE_Allocator {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Allocator8instanceEPS_"]
            fn __ext(_anon_0: *mut ACE_Allocator) -> *mut ACE_Allocator;
        }
        __ext(_anon_0)
    }
    ///Delete the dynamically allocated Singleton
    pub unsafe fn close_singleton() {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Allocator15close_singletonEv"]
            fn __ext();
        }
        __ext()
    }
    ///"No-op" constructor (needed to make certain compilers happy).
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_AllocatorC1Ev"]
            fn __ext(__this: *mut ACE_Allocator);
        }
        __ext(__this as *mut ACE_Allocator)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
}
impl ACE_Data_Block {
    ///Default "do-nothing" constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_BlockC1Ev"]
            fn __ext(__this: *mut ACE_Data_Block);
        }
        __ext(__this as *mut ACE_Data_Block)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Initialize.
    pub unsafe fn new_at_u26305eeec0fb63a6(
        __this: *mut Self,
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: *const libc::c_char,
        mut __a3: *mut ACE_Allocator,
        mut __a4: *mut ACE_Lock,
        mut __a5: libc::c_ulong,
        mut __a6: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_BlockC1EmiPKcP13ACE_AllocatorP8ACE_LockmS3_"]
            fn __ext(
                __this: *mut ACE_Data_Block,
                __a0: libc::c_ulong,
                __a1: libc::c_int,
                __a2: *const libc::c_char,
                __a3: *mut ACE_Allocator,
                __a4: *mut ACE_Lock,
                __a5: libc::c_ulong,
                __a6: *mut ACE_Allocator,
            );
        }
        __ext(__this as *mut ACE_Data_Block, __a0, __a1, __a2, __a3, __a4, __a5, __a6)
    }
    pub unsafe fn new_u26305eeec0fb63a6(
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: *const libc::c_char,
        mut __a3: *mut ACE_Allocator,
        mut __a4: *mut ACE_Lock,
        mut __a5: libc::c_ulong,
        mut __a6: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u26305eeec0fb63a6(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
        );
        __obj
    }
    ///Get type of the message.
    pub unsafe fn msg_type(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).type_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set type of the message.
    pub unsafe fn msg_type_u9911b9cef7bd0539(__this: *mut Self, mut t: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).type_ = t;
            }
            ()
        }
    }
    ///Get message data pointer
    pub unsafe fn base(__this: *const Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).base_) as *mut libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set message data pointer (doesn't reallocate).
    pub unsafe fn base_u44df93754cea9c80(
        __this: *mut Self,
        mut data: *mut libc::c_char,
        mut size: libc::c_ulong,
        mut mflags: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_Block4baseEPcmm"]
            fn __ext(
                __this: *mut ACE_Data_Block,
                data: *mut libc::c_char,
                size: libc::c_ulong,
                mflags: libc::c_ulong,
            );
        }
        __ext(__this as *mut ACE_Data_Block, data, size, mflags)
    }
    ///Return a pointer to 1 past the end of the allocated data in a message.
    pub unsafe fn end(__this: *const Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((*__this).base_).wrapping_offset(((*__this).max_size_) as isize);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Return a pointer to 1 past the end of the allotted data in a message.\n   * The allotted data may be less than allocated data if <size()> is passed\n   * an argument less than <capacity()>."]
    pub unsafe fn mark(__this: *const Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((*__this).base_).wrapping_offset(((*__this).cur_size_) as isize);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Get the total amount of allotted space in the message.  The amount of
  /// allotted space may be less than allocated space.*/
    pub unsafe fn size(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).cur_size_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Set the total amount of space in the message.  Returns 0 if
  /// successful, else -1.*/
    pub unsafe fn size_u05f0dc1612ed1a4d(
        __this: *mut Self,
        mut length: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_Block4sizeEm"]
            fn __ext(__this: *mut ACE_Data_Block, length: libc::c_ulong) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Data_Block, length)
    }
    ///Get the total amount of allocated space.
    pub unsafe fn capacity(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).max_size_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Return an exact \"deep copy\" of the message, i.e., create fresh\n   * new copies of all the Data_Blocks and continuations.\n   * Notice that Data_Blocks can act as \"Prototypes\", i.e. derived\n   * classes can override this method and create instances of\n   * themselves."]
    pub unsafe fn clone(
        __this: *const Self,
        mut mask: libc::c_ulong,
    ) -> *mut ACE_Data_Block {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Data_Block5cloneEm"]
            fn __ext(
                __this: *const ACE_Data_Block,
                mask: libc::c_ulong,
            ) -> *mut ACE_Data_Block;
        }
        __ext(__this as *const ACE_Data_Block, mask)
    }
    #[doc = "* As clone above, but it does not copy the contents of the buffer,\n   * i.e., create a new Data_Block of the same dynamic type, with the\n   * same allocator, locking_strategy, and with the same amount of\n   * storage available (if @a max_size is zero) but the buffer is unitialized.\n   * If @a max_size is specified other than zero, it will be used when\n   * creating the new data block."]
    pub unsafe fn clone_nocopy(
        __this: *const Self,
        mut mask: libc::c_ulong,
        mut max_size: libc::c_ulong,
    ) -> *mut ACE_Data_Block {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Data_Block12clone_nocopyEmm"]
            fn __ext(
                __this: *const ACE_Data_Block,
                mask: libc::c_ulong,
                max_size: libc::c_ulong,
            ) -> *mut ACE_Data_Block;
        }
        __ext(__this as *const ACE_Data_Block, mask, max_size)
    }
    ///Return a "shallow" copy that increments our reference count by 1.
    pub unsafe fn duplicate(__this: *mut Self) -> *mut ACE_Data_Block {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_Block9duplicateEv"]
            fn __ext(__this: *mut ACE_Data_Block) -> *mut ACE_Data_Block;
        }
        __ext(__this as *mut ACE_Data_Block)
    }
    #[doc = "* Decrease the shared reference count by 1.  If the reference count\n   * is > 0 then return this; else if reference count == 0 then delete\n   * @c this and @a mb and return 0.  Behavior is undefined if reference\n   * count < 0."]
    pub unsafe fn release(
        __this: *mut Self,
        mut lock: *mut ACE_Lock,
    ) -> *mut ACE_Data_Block {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_Block7releaseEP8ACE_Lock"]
            fn __ext(
                __this: *mut ACE_Data_Block,
                lock: *mut ACE_Lock,
            ) -> *mut ACE_Data_Block;
        }
        __ext(__this as *mut ACE_Data_Block, lock)
    }
    /**Bitwise-or the @a more_flags into the existing message flags and
  /// return the new value.*/
    pub unsafe fn set_flags(
        __this: *mut Self,
        mut more_flags: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ({
                    let __lv = ::core::ptr::addr_of_mut!((* __this).flags_);
                    unsafe {
                        *__lv = ((((*__lv)) as libc::c_ulong))
                            | (((more_flags)) as libc::c_ulong);
                        *__lv
                    }
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Clear the message flag bits specified in @a less_flags and return
  /// the new value.*/
    pub unsafe fn clr_flags(
        __this: *mut Self,
        mut less_flags: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ({
                    let __lv = ::core::ptr::addr_of_mut!((* __this).flags_);
                    unsafe {
                        *__lv = ((((*__lv)) as libc::c_ulong))
                            & (((!((less_flags)))) as libc::c_ulong);
                        *__lv
                    }
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the current message flags.
    pub unsafe fn flags(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).flags_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Obtain the allocator strategy.
    pub unsafe fn allocator_strategy(__this: *const Self) -> *mut ACE_Allocator {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).allocator_strategy_) as *mut ACE_Allocator);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the locking strategy.
    pub unsafe fn locking_strategy(__this: *mut Self) -> *mut ACE_Lock {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).locking_strategy_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set a new locking strategy and return the hold one.
    pub unsafe fn locking_strategy_u842da8c0d677dd6d(
        __this: *mut Self,
        mut nls: *mut ACE_Lock,
    ) -> *mut ACE_Lock {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ols: *mut ACE_Lock = (*__this).locking_strategy_;
                (*__this).locking_strategy_ = nls;
                return ols;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Data_Block4dumpEv"]
            fn __ext(__this: *const ACE_Data_Block);
        }
        __ext(__this as *const ACE_Data_Block)
    }
    ///Get the current reference count.
    pub unsafe fn reference_count(__this: *const Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Data_Block15reference_countEv"]
            fn __ext(__this: *const ACE_Data_Block) -> libc::c_int;
        }
        __ext(__this as *const ACE_Data_Block)
    }
    ///Get the allocator used to create this object
    pub unsafe fn data_block_allocator(__this: *const Self) -> *mut ACE_Allocator {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).data_block_allocator_) as *mut ACE_Allocator);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Internal release implementation
    pub unsafe fn release_i(__this: *mut Self) -> *mut ACE_Data_Block {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_Block9release_iEv"]
            fn __ext(__this: *mut ACE_Data_Block) -> *mut ACE_Data_Block;
        }
        __ext(__this as *mut ACE_Data_Block)
    }
    ///Internal get the current reference count.
    pub unsafe fn reference_count_i(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).reference_count_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn release_no_delete(
        __this: *mut Self,
        mut lock: *mut ACE_Lock,
    ) -> *mut ACE_Data_Block {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_Block17release_no_deleteEP8ACE_Lock"]
            fn __ext(
                __this: *mut ACE_Data_Block,
                lock: *mut ACE_Lock,
            ) -> *mut ACE_Data_Block;
        }
        __ext(__this as *mut ACE_Data_Block, lock)
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Data_Block,
    ) -> *mut ACE_Data_Block {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_BlockaSERKS_"]
            fn __ext(
                __this: *mut ACE_Data_Block,
                _anon_0: *const ACE_Data_Block,
            ) -> *mut ACE_Data_Block;
        }
        __ext(__this as *mut ACE_Data_Block, _anon_0)
    }
    pub unsafe fn new_at_ua86b7b60defc080a(
        __this: *mut Self,
        mut __a0: *const ACE_Data_Block,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Data_BlockC1ERKS_"]
            fn __ext(__this: *mut ACE_Data_Block, __a0: *const ACE_Data_Block);
        }
        __ext(__this as *mut ACE_Data_Block, __a0)
    }
    pub unsafe fn new_ua86b7b60defc080a(mut __a0: *const ACE_Data_Block) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ua86b7b60defc080a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_Cleanup_Info_Node {
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_Cleanup_Info_NodeC1Ev"]
            fn __ext(__this: *mut ACE_Cleanup_Info_Node);
        }
        __ext(__this as *mut ACE_Cleanup_Info_Node)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    pub unsafe fn new_at_u2f1a9d3ae456588c(
        __this: *mut Self,
        mut __a0: *mut libc::c_void,
        mut __a1: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void, *mut libc::c_void),
        >,
        mut __a2: *mut libc::c_void,
        mut __a3: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_Cleanup_Info_NodeC1EPvPFvS0_S0_ES0_PKc"]
            fn __ext(
                __this: *mut ACE_Cleanup_Info_Node,
                __a0: *mut libc::c_void,
                __a1: Option<
                    unsafe extern "C-unwind" fn(*mut libc::c_void, *mut libc::c_void),
                >,
                __a2: *mut libc::c_void,
                __a3: *const libc::c_char,
            );
        }
        __ext(__this as *mut ACE_Cleanup_Info_Node, __a0, __a1, __a2, __a3)
    }
    pub unsafe fn new_u2f1a9d3ae456588c(
        mut __a0: *mut libc::c_void,
        mut __a1: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void, *mut libc::c_void),
        >,
        mut __a2: *mut libc::c_void,
        mut __a3: *const libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u2f1a9d3ae456588c(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
        );
        __obj
    }
    ///Equality operator.
    pub unsafe fn operator_eq(
        __this: *const Self,
        mut o: *const ACE_Cleanup_Info_Node,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK21ACE_Cleanup_Info_NodeeqERKS_"]
            fn __ext(
                __this: *const ACE_Cleanup_Info_Node,
                o: *const ACE_Cleanup_Info_Node,
            ) -> bool;
        }
        __ext(__this as *const ACE_Cleanup_Info_Node, o)
    }
    ///Inequality operator.
    pub unsafe fn operator_ne(
        __this: *const Self,
        mut o: *const ACE_Cleanup_Info_Node,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK21ACE_Cleanup_Info_NodeneERKS_"]
            fn __ext(
                __this: *const ACE_Cleanup_Info_Node,
                o: *const ACE_Cleanup_Info_Node,
            ) -> bool;
        }
        __ext(__this as *const ACE_Cleanup_Info_Node, o)
    }
    pub unsafe fn object(__this: *mut Self) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).object_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn cleanup_hook(
        __this: *mut Self,
    ) -> Option<unsafe extern "C-unwind" fn(*mut libc::c_void, *mut libc::c_void)> {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).cleanup_hook_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn param(__this: *mut Self) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).param_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_ {
    pub unsafe fn prev_s6ee06b7416244c71(
        __this: *const Self,
    ) -> *mut ACE_Cleanup_Info_Node {
        extern "C-unwind" {
            #[link_name = "_ZNK23ACE_Intrusive_List_NodeI21ACE_Cleanup_Info_NodeE4prevEv"]
            fn __ext(
                __this: *const ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_,
            ) -> *mut ACE_Cleanup_Info_Node;
        }
        __ext(__this as *const ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_)
    }
    pub unsafe fn prev_s7a54ec704e553481(
        __this: *mut Self,
        mut _anon_0: *mut ACE_Cleanup_Info_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Intrusive_List_NodeI21ACE_Cleanup_Info_NodeE4prevEPS0_"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_,
                _anon_0: *mut ACE_Cleanup_Info_Node,
            );
        }
        __ext(__this as *mut ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_, _anon_0)
    }
    pub unsafe fn next_s6ee06b7416244c71(
        __this: *const Self,
    ) -> *mut ACE_Cleanup_Info_Node {
        extern "C-unwind" {
            #[link_name = "_ZNK23ACE_Intrusive_List_NodeI21ACE_Cleanup_Info_NodeE4nextEv"]
            fn __ext(
                __this: *const ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_,
            ) -> *mut ACE_Cleanup_Info_Node;
        }
        __ext(__this as *const ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_)
    }
    pub unsafe fn next_s7a54ec704e553481(
        __this: *mut Self,
        mut _anon_0: *mut ACE_Cleanup_Info_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Intrusive_List_NodeI21ACE_Cleanup_Info_NodeE4nextEPS0_"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_,
                _anon_0: *mut ACE_Cleanup_Info_Node,
            );
        }
        __ext(__this as *mut ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_, _anon_0)
    }
    #[doc = "Constructor\n  /**\n   * The constructor is protected, because only derived classes should\n   * be instantiated."]
    pub unsafe fn new_at_s86266f1ec226070c(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Intrusive_List_NodeI21ACE_Cleanup_Info_NodeEC1Ev"]
            fn __ext(__this: *mut ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_);
        }
        __ext(__this as *mut ACE_Intrusive_List_Node_ACE_Cleanup_Info_Node_)
    }
    pub unsafe fn new_s86266f1ec226070c() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s86266f1ec226070c(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
}
impl ACE_Intrusive_List_ACE_Cleanup_Info_Node_ {
    /**Constructor.  Use user specified allocation strategy
  /// if specified.*/
    pub unsafe fn new_at_s7bc33ba496dc67ed(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeEC1Ev"]
            fn __ext(__this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_);
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_)
    }
    pub unsafe fn new_s7bc33ba496dc67ed() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s7bc33ba496dc67ed(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Returns true if the container is empty, otherwise returns false.
    pub unsafe fn is_empty(__this: *const Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE8is_emptyEv"]
            fn __ext(__this: *const ACE_Intrusive_List_ACE_Cleanup_Info_Node_) -> bool;
        }
        __ext(__this as *const ACE_Intrusive_List_ACE_Cleanup_Info_Node_)
    }
    ///Insert an element at the beginning of the list
    pub unsafe fn push_front(__this: *mut Self, mut node: *mut ACE_Cleanup_Info_Node) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE10push_frontEPS0_"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
                node: *mut ACE_Cleanup_Info_Node,
            );
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_, node)
    }
    ///Insert an element at the end of the list
    pub unsafe fn push_back(__this: *mut Self, mut node: *mut ACE_Cleanup_Info_Node) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE9push_backEPS0_"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
                node: *mut ACE_Cleanup_Info_Node,
            );
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_, node)
    }
    ///Remove the element at the beginning of the list
    pub unsafe fn pop_front(__this: *mut Self) -> *mut ACE_Cleanup_Info_Node {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE9pop_frontEv"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
            ) -> *mut ACE_Cleanup_Info_Node;
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_)
    }
    ///Remove the element at the end of the list
    pub unsafe fn pop_back(__this: *mut Self) -> *mut ACE_Cleanup_Info_Node {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE8pop_backEv"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
            ) -> *mut ACE_Cleanup_Info_Node;
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_)
    }
    ///Get the element at the head of the queue
    pub unsafe fn head(__this: *const Self) -> *mut ACE_Cleanup_Info_Node {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE4headEv"]
            fn __ext(
                __this: *const ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
            ) -> *mut ACE_Cleanup_Info_Node;
        }
        __ext(__this as *const ACE_Intrusive_List_ACE_Cleanup_Info_Node_)
    }
    ///Get the element at the tail of the queue
    pub unsafe fn tail(__this: *const Self) -> *mut ACE_Cleanup_Info_Node {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE4tailEv"]
            fn __ext(
                __this: *const ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
            ) -> *mut ACE_Cleanup_Info_Node;
        }
        __ext(__this as *const ACE_Intrusive_List_ACE_Cleanup_Info_Node_)
    }
    #[doc = "Remove a element from the list\n  /**\n   * Verify that the element is still in the list before removing it."]
    pub unsafe fn remove(__this: *mut Self, mut node: *mut ACE_Cleanup_Info_Node) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE6removeEPS0_"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
                node: *mut ACE_Cleanup_Info_Node,
            );
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_, node)
    }
    ///Swap two lists
    pub unsafe fn swap(
        __this: *mut Self,
        mut rhs: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE4swapERS1_"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
                rhs: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
            );
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_, rhs)
    }
    #[doc = "Remove a element from the list without checking\n  /**\n   * No attempts are performed to check if T* really belongs to the\n   * list.  The effects of removing an invalid element are unspecified"]
    pub unsafe fn unsafe_remove(
        __this: *mut Self,
        mut node: *mut ACE_Cleanup_Info_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeE13unsafe_removeEPS0_"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
                node: *mut ACE_Cleanup_Info_Node,
            );
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_, node)
    }
    pub unsafe fn new_at_s61b41f66774921af(
        __this: *mut Self,
        mut __a0: *const ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeEC1ERKS1_"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
                __a0: *const ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
            );
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_, __a0)
    }
    pub unsafe fn new_s61b41f66774921af(
        mut __a0: *const ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s61b41f66774921af(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
    ) -> *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_ {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Intrusive_ListI21ACE_Cleanup_Info_NodeEaSERKS1_"]
            fn __ext(
                __this: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
                _anon_0: *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_,
            ) -> *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_;
        }
        __ext(__this as *mut ACE_Intrusive_List_ACE_Cleanup_Info_Node_, _anon_0)
    }
}
impl ACE_OS_Exit_Info {
    ///Default constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_OS_Exit_InfoC1Ev"]
            fn __ext(__this: *mut ACE_OS_Exit_Info);
        }
        __ext(__this as *mut ACE_OS_Exit_Info)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Use to register a cleanup hook.
    pub unsafe fn at_exit_i(
        __this: *mut Self,
        mut object: *mut libc::c_void,
        mut cleanup_hook: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void, *mut libc::c_void),
        >,
        mut param: *mut libc::c_void,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_OS_Exit_Info9at_exit_iEPvPFvS0_S0_ES0_PKc"]
            fn __ext(
                __this: *mut ACE_OS_Exit_Info,
                object: *mut libc::c_void,
                cleanup_hook: Option<
                    unsafe extern "C-unwind" fn(*mut libc::c_void, *mut libc::c_void),
                >,
                param: *mut libc::c_void,
                name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_OS_Exit_Info, object, cleanup_hook, param, name)
    }
    /**Look for a registered cleanup hook object.  Returns true if already
  /// registered, false if not.*/
    pub unsafe fn find(__this: *mut Self, mut object: *mut libc::c_void) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_OS_Exit_Info4findEPv"]
            fn __ext(__this: *mut ACE_OS_Exit_Info, object: *mut libc::c_void) -> bool;
        }
        __ext(__this as *mut ACE_OS_Exit_Info, object)
    }
    /**Remove a registered cleanup hook object.  Returns true if removed
  /// false if not.*/
    pub unsafe fn remove(__this: *mut Self, mut object: *mut libc::c_void) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_OS_Exit_Info6removeEPv"]
            fn __ext(__this: *mut ACE_OS_Exit_Info, object: *mut libc::c_void) -> bool;
        }
        __ext(__this as *mut ACE_OS_Exit_Info, object)
    }
    /**Call all registered cleanup hooks, in reverse order of
  /// registration.*/
    pub unsafe fn call_hooks(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_OS_Exit_Info10call_hooksEv"]
            fn __ext(__this: *mut ACE_OS_Exit_Info);
        }
        __ext(__this as *mut ACE_OS_Exit_Info)
    }
}
impl ACE_OS_Object_Manager {
    ///Explicitly initialize.
    pub unsafe fn init(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager4initEv"]
            fn __ext(__this: *mut ACE_OS_Object_Manager) -> libc::c_int;
        }
        __ext(__this as *mut ACE_OS_Object_Manager)
    }
    ///Explicitly destroy.
    pub unsafe fn fini(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager4finiEv"]
            fn __ext(__this: *mut ACE_OS_Object_Manager) -> libc::c_int;
        }
        __ext(__this as *mut ACE_OS_Object_Manager)
    }
    #[doc = "* Returns 1 before the ACE_OS_Object_Manager has been\n   * constructed.  See <ACE_Object_Manager::starting_up> for more\n   * information."]
    pub unsafe fn starting_up() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager11starting_upEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    /**Returns 1 after the ACE_OS_Object_Manager has been destroyed.
  /// See <ACE_Object_Manager::shutting_down> for more information.*/
    pub unsafe fn shutting_down() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager13shutting_downEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    /**Accesses a default signal set used, for example, in
  /// ACE_Sig_Guard methods.*/
    pub unsafe fn default_mask() -> *mut __sigset_t {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager12default_maskEv"]
            fn __ext() -> *mut __sigset_t;
        }
        __ext()
    }
    ///Returns the current thread hook for the process.
    pub unsafe fn thread_hook() -> *mut ACE_Thread_Hook {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager11thread_hookEv"]
            fn __ext() -> *mut ACE_Thread_Hook;
        }
        __ext()
    }
    ///Returns the existing thread hook and assign a <new_thread_hook>.
    pub unsafe fn thread_hook_u8ca6168bec16b970(
        mut new_thread_hook: *mut ACE_Thread_Hook,
    ) -> *mut ACE_Thread_Hook {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager11thread_hookEP15ACE_Thread_Hook"]
            fn __ext(new_thread_hook: *mut ACE_Thread_Hook) -> *mut ACE_Thread_Hook;
        }
        __ext(new_thread_hook)
    }
    ///Constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_ManagerC1Ev"]
            fn __ext(__this: *mut ACE_OS_Object_Manager);
        }
        __ext(__this as *mut ACE_OS_Object_Manager)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Accessor to singleton instance.
    pub unsafe fn instance() -> *mut ACE_OS_Object_Manager {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager8instanceEv"]
            fn __ext() -> *mut ACE_OS_Object_Manager;
        }
        __ext()
    }
    ///For <ACE_OS::atexit> support.
    pub unsafe fn at_exit(
        __this: *mut Self,
        mut func: Option<unsafe extern "C-unwind" fn()>,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager7at_exitEPFvvEPKc"]
            fn __ext(
                __this: *mut ACE_OS_Object_Manager,
                func: Option<unsafe extern "C-unwind" fn()>,
                name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_OS_Object_Manager, func, name)
    }
    ///For use by init () and fini (), to consolidate error reporting.
    pub unsafe fn print_error_message(
        mut line_number: libc::c_uint,
        mut message: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_OS_Object_Manager19print_error_messageEjPKc"]
            fn __ext(line_number: libc::c_uint, message: *const libc::c_char);
        }
        __ext(line_number, message)
    }
}
pub unsafe fn __vdtor_u705b46bc57370e50(__this: *mut ACE_Message_Block) {
    let _ = Box::from_raw(__this as *mut ACE_Message_Block);
}
pub unsafe fn __vthunk_ou089570e96204ada4_iu089570e96204ada4(
    __this: *mut ACE_Message_Block,
    p0: libc::c_ulong,
) -> *mut ACE_Message_Block {
    <ACE_Message_Block>::clone((__this as *mut ACE_Message_Block), p0)
}
pub unsafe fn __vthunk_ou7f107e13c4d790cf_iu7f107e13c4d790cf(
    __this: *mut ACE_Message_Block,
) -> *mut ACE_Message_Block {
    <ACE_Message_Block>::duplicate((__this as *mut ACE_Message_Block))
}
pub unsafe fn __vthunk_ou80b95875dc4ea94c_iu80b95875dc4ea94c(
    __this: *mut ACE_Message_Block,
) -> *mut ACE_Message_Block {
    <ACE_Message_Block>::release((__this as *mut ACE_Message_Block))
}
#[repr(C)]
pub struct __Vtbl_u705b46bc57370e50 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Message_Block),
    pub vfn_u089570e96204ada4: unsafe fn(
        *mut ACE_Message_Block,
        libc::c_ulong,
    ) -> *mut ACE_Message_Block,
    pub vfn_u7f107e13c4d790cf: unsafe fn(
        *mut ACE_Message_Block,
    ) -> *mut ACE_Message_Block,
    pub vfn_u80b95875dc4ea94c: unsafe fn(
        *mut ACE_Message_Block,
    ) -> *mut ACE_Message_Block,
}
pub static __VTBL_u705b46bc57370e50: __Vtbl_u705b46bc57370e50 = __Vtbl_u705b46bc57370e50 {
    __type_info: &__TYPEINFO_17ACE_Message_Block,
    __vdtor: __vdtor_u705b46bc57370e50,
    vfn_u089570e96204ada4: __vthunk_ou089570e96204ada4_iu089570e96204ada4,
    vfn_u7f107e13c4d790cf: __vthunk_ou7f107e13c4d790cf_iu7f107e13c4d790cf,
    vfn_u80b95875dc4ea94c: __vthunk_ou80b95875dc4ea94c_iu80b95875dc4ea94c,
};
pub unsafe fn __vdtor_uf2113694993e252c(__this: *mut ACE_Allocator) {
    let _ = Box::from_raw(__this as *mut ACE_Allocator);
}
pub unsafe fn __vthunk_ou685215409e23bf32_iu685215409e23bf32(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
) -> *mut libc::c_void {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou0a9372cacdda8cbe_iu0a9372cacdda8cbe(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
    p1: libc::c_char,
) -> *mut libc::c_void {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oued53ccfa62009d93_iued53ccfa62009d93(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
    p1: libc::c_ulong,
    p2: libc::c_char,
) -> *mut libc::c_void {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oucc7a27ee055bb87e_iucc7a27ee055bb87e(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
) {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouaeedb459d846087b_iuaeedb459d846087b(
    __this: *mut ACE_Allocator,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou7bff1870c893b3fe_iu7bff1870c893b3fe(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut libc::c_void,
    p2: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou381e2dddd3465a71_iu381e2dddd3465a71(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou41a4de2216226892_iu41a4de2216226892(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou22342900ef7c0f5d_iu22342900ef7c0f5d(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouce55bcfdd7d4af38_iuce55bcfdd7d4af38(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou18ccae12f60528e3_iu18ccae12f60528e3(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou62da3684ac8c6bf7_iu62da3684ac8c6bf7(
    __this: *mut ACE_Allocator,
    p0: libc::c_long,
    p1: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou7db10b00cce8fa5e_iu7db10b00cce8fa5e(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouada926f987c0415f_iuada926f987c0415f(
    __this: *mut ACE_Allocator,
    p0: libc::c_long,
    p1: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouf36ac9d4f584a786_iuf36ac9d4f584a786(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouac8ca8237e7c8154_iuac8ca8237e7c8154(
    __this: *mut ACE_Allocator,
) {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_uf2113694993e252c {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Allocator),
    pub vfn_u685215409e23bf32: unsafe fn(
        *mut ACE_Allocator,
        libc::c_ulong,
    ) -> *mut libc::c_void,
    pub vfn_u0a9372cacdda8cbe: unsafe fn(
        *mut ACE_Allocator,
        libc::c_ulong,
        libc::c_char,
    ) -> *mut libc::c_void,
    pub vfn_ued53ccfa62009d93: unsafe fn(
        *mut ACE_Allocator,
        libc::c_ulong,
        libc::c_ulong,
        libc::c_char,
    ) -> *mut libc::c_void,
    pub vfn_ucc7a27ee055bb87e: unsafe fn(*mut ACE_Allocator, *mut libc::c_void),
    pub vfn_uaeedb459d846087b: unsafe fn(*mut ACE_Allocator) -> libc::c_int,
    pub vfn_u7bff1870c893b3fe: unsafe fn(
        *mut ACE_Allocator,
        *const libc::c_char,
        *mut libc::c_void,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u381e2dddd3465a71: unsafe fn(
        *mut ACE_Allocator,
        *const libc::c_char,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
    pub vfn_u41a4de2216226892: unsafe fn(
        *mut ACE_Allocator,
        *const libc::c_char,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
    pub vfn_u22342900ef7c0f5d: unsafe fn(
        *mut ACE_Allocator,
        *const libc::c_char,
    ) -> libc::c_int,
    pub vfn_uce55bcfdd7d4af38: unsafe fn(
        *mut ACE_Allocator,
        *const libc::c_char,
    ) -> libc::c_int,
    pub vfn_u18ccae12f60528e3: unsafe fn(
        *mut ACE_Allocator,
        *const libc::c_char,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
    pub vfn_u62da3684ac8c6bf7: unsafe fn(
        *mut ACE_Allocator,
        libc::c_long,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u7db10b00cce8fa5e: unsafe fn(
        *mut ACE_Allocator,
        *mut libc::c_void,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uada926f987c0415f: unsafe fn(
        *mut ACE_Allocator,
        libc::c_long,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uf36ac9d4f584a786: unsafe fn(
        *mut ACE_Allocator,
        *mut libc::c_void,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uac8ca8237e7c8154: unsafe fn(*mut ACE_Allocator),
}
pub static __VTBL_uf2113694993e252c: __Vtbl_uf2113694993e252c = __Vtbl_uf2113694993e252c {
    __type_info: &__TYPEINFO_13ACE_Allocator,
    __vdtor: __vdtor_uf2113694993e252c,
    vfn_u685215409e23bf32: __vthunk_ou685215409e23bf32_iu685215409e23bf32,
    vfn_u0a9372cacdda8cbe: __vthunk_ou0a9372cacdda8cbe_iu0a9372cacdda8cbe,
    vfn_ued53ccfa62009d93: __vthunk_oued53ccfa62009d93_iued53ccfa62009d93,
    vfn_ucc7a27ee055bb87e: __vthunk_oucc7a27ee055bb87e_iucc7a27ee055bb87e,
    vfn_uaeedb459d846087b: __vthunk_ouaeedb459d846087b_iuaeedb459d846087b,
    vfn_u7bff1870c893b3fe: __vthunk_ou7bff1870c893b3fe_iu7bff1870c893b3fe,
    vfn_u381e2dddd3465a71: __vthunk_ou381e2dddd3465a71_iu381e2dddd3465a71,
    vfn_u41a4de2216226892: __vthunk_ou41a4de2216226892_iu41a4de2216226892,
    vfn_u22342900ef7c0f5d: __vthunk_ou22342900ef7c0f5d_iu22342900ef7c0f5d,
    vfn_uce55bcfdd7d4af38: __vthunk_ouce55bcfdd7d4af38_iuce55bcfdd7d4af38,
    vfn_u18ccae12f60528e3: __vthunk_ou18ccae12f60528e3_iu18ccae12f60528e3,
    vfn_u62da3684ac8c6bf7: __vthunk_ou62da3684ac8c6bf7_iu62da3684ac8c6bf7,
    vfn_u7db10b00cce8fa5e: __vthunk_ou7db10b00cce8fa5e_iu7db10b00cce8fa5e,
    vfn_uada926f987c0415f: __vthunk_ouada926f987c0415f_iuada926f987c0415f,
    vfn_uf36ac9d4f584a786: __vthunk_ouf36ac9d4f584a786_iuf36ac9d4f584a786,
    vfn_uac8ca8237e7c8154: __vthunk_ouac8ca8237e7c8154_iuac8ca8237e7c8154,
};
pub unsafe fn __vdtor_ue8090ae954631e85(__this: *mut ACE_Data_Block) {
    let _ = Box::from_raw(__this as *mut ACE_Data_Block);
}
pub unsafe fn __vthunk_ouff1425b2a1b400b8_iuff1425b2a1b400b8(
    __this: *mut ACE_Data_Block,
    p0: libc::c_ulong,
) -> *mut ACE_Data_Block {
    <ACE_Data_Block>::clone((__this as *mut ACE_Data_Block), p0)
}
pub unsafe fn __vthunk_ou332d56177ae0a20a_iu332d56177ae0a20a(
    __this: *mut ACE_Data_Block,
    p0: libc::c_ulong,
    p1: libc::c_ulong,
) -> *mut ACE_Data_Block {
    <ACE_Data_Block>::clone_nocopy((__this as *mut ACE_Data_Block), p0, p1)
}
pub unsafe fn __vthunk_ou87d1892604b2398c_iu87d1892604b2398c(
    __this: *mut ACE_Data_Block,
) -> *mut ACE_Data_Block {
    <ACE_Data_Block>::release_i((__this as *mut ACE_Data_Block))
}
#[repr(C)]
pub struct __Vtbl_ue8090ae954631e85 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Data_Block),
    pub vfn_uff1425b2a1b400b8: unsafe fn(
        *mut ACE_Data_Block,
        libc::c_ulong,
    ) -> *mut ACE_Data_Block,
    pub vfn_u332d56177ae0a20a: unsafe fn(
        *mut ACE_Data_Block,
        libc::c_ulong,
        libc::c_ulong,
    ) -> *mut ACE_Data_Block,
    pub vfn_u87d1892604b2398c: unsafe fn(*mut ACE_Data_Block) -> *mut ACE_Data_Block,
}
pub static __VTBL_ue8090ae954631e85: __Vtbl_ue8090ae954631e85 = __Vtbl_ue8090ae954631e85 {
    __type_info: &__TYPEINFO_14ACE_Data_Block,
    __vdtor: __vdtor_ue8090ae954631e85,
    vfn_uff1425b2a1b400b8: __vthunk_ouff1425b2a1b400b8_iuff1425b2a1b400b8,
    vfn_u332d56177ae0a20a: __vthunk_ou332d56177ae0a20a_iu332d56177ae0a20a,
    vfn_u87d1892604b2398c: __vthunk_ou87d1892604b2398c_iu87d1892604b2398c,
};
pub unsafe fn __vdtor_uff0c65993f46a4f5(__this: *mut ACE_Object_Manager_Base) {
    let _ = Box::from_raw(__this as *mut ACE_OS_Object_Manager);
}
pub unsafe fn __vthunk_oud77ccbde1c101f57_iu2ae16bb83518d3fd(
    __this: *mut ACE_Object_Manager_Base,
) -> libc::c_int {
    <ACE_OS_Object_Manager>::init((__this as *mut ACE_OS_Object_Manager))
}
pub unsafe fn __vthunk_ouc1123aef57ad2151_iue800bfd5a4e8ffa3(
    __this: *mut ACE_Object_Manager_Base,
) -> libc::c_int {
    <ACE_OS_Object_Manager>::fini((__this as *mut ACE_OS_Object_Manager))
}
pub static __VTBL_uff0c65993f46a4f5: __Vtbl_uff0c65993f46a4f5 = __Vtbl_uff0c65993f46a4f5 {
    __type_info: &__TYPEINFO_21ACE_OS_Object_Manager,
    __vdtor: __vdtor_uff0c65993f46a4f5,
    vfn_u2ae16bb83518d3fd: __vthunk_oud77ccbde1c101f57_iu2ae16bb83518d3fd,
    vfn_ue800bfd5a4e8ffa3: __vthunk_ouc1123aef57ad2151_iue800bfd5a4e8ffa3,
};
