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
///Defines the structure of an ACE logging record.
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Log_Record {
    pub length_: libc::c_int,
    pub type_: libc::c_uint,
    pub secs_: libc::c_long,
    pub usecs_: libc::c_uint,
    pub pid_: libc::c_uint,
    pub msg_data_: *mut libc::c_char,
    pub msg_data_size_: libc::c_ulong,
    pub category_: *mut ACE_Log_Category_TSS,
}
impl Drop for ACE_Log_Record {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {
                if (!((*__this).msg_data_).is_null()) {
                    {
                        let __data = (*__this).msg_data_ as *mut libc::c_char;
                        if !__data.is_null() {
                            ::libc::free(__data as *mut libc::c_void);
                        }
                    };
                }
            }
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_uec9f6c59dd7281f1(
    __this: *mut ACE_Log_Record,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
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
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Addr_sap_any: ACE_Addr;
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
    pub use crate::full_ops_0::ACE_OS::fcntl_ue9a749ea668b010b;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS4openEPKciji"]
        pub fn open_u20e73ad42bc94ee0(
            filename: *const libc::c_char,
            mode: libc::c_int,
            perms: libc::c_uint,
            sa: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS4openEPKwiji"]
        pub fn open_u3ea2b23d28764574(
            filename: *const libc::wchar_t,
            mode: libc::c_int,
            perms: libc::c_uint,
            sa: libc::c_int,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::getmsg;
    pub use crate::full_ops_0::ACE_OS::getpmsg;
    pub use crate::full_ops_0::ACE_OS::fattach;
    pub use crate::full_ops_0::ACE_OS::fdetach;
    pub use crate::full_ops_0::ACE_OS::ioctl_u5ac77b5f0779af02;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS5ioctlEimPvmS0_mPmP14ACE_OVERLAPPEDPFvmmS3_mE"]
        pub fn ioctl_u8cb58ee7404e374c(
            socket: libc::c_int,
            io_control_code: libc::c_ulong,
            in_buffer_p: *mut libc::c_void,
            in_buffer: libc::c_ulong,
            out_buffer_p: *mut libc::c_void,
            out_buffer: libc::c_ulong,
            bytes_returned: *mut libc::c_ulong,
            overlapped: *mut super::ACE_OVERLAPPED,
            func: Option<
                unsafe extern "C-unwind" fn(
                    libc::c_ulong,
                    libc::c_ulong,
                    *mut super::ACE_OVERLAPPED,
                    libc::c_ulong,
                ),
            >,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS5ioctlEimR7ACE_QoSPmPvmP14ACE_OVERLAPPEDPFvmmS5_mE"]
        pub fn ioctl_u3ca0e31dfc98a8b9(
            socket: libc::c_int,
            io_control_code: libc::c_ulong,
            ace_qos: *mut super::ACE_QoS,
            bytes_returned: *mut libc::c_ulong,
            buffer_p: *mut libc::c_void,
            buffer: libc::c_ulong,
            overlapped: *mut super::ACE_OVERLAPPED,
            func: Option<
                unsafe extern "C-unwind" fn(
                    libc::c_ulong,
                    libc::c_ulong,
                    *mut super::ACE_OVERLAPPED,
                    libc::c_ulong,
                ),
            >,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::isastream;
    pub use crate::full_ops_0::ACE_OS::putmsg;
    pub use crate::full_ops_0::ACE_OS::putpmsg;
    pub use crate::full_ops_0::ACE_OS::access_ud59681e785d3d49a;
    pub use crate::full_ops_0::ACE_OS::access_u559fd1837b205e7e;
    pub use crate::full_ops_0::ACE_OS::alarm_u3d46a5b3c95563f0;
    pub use crate::full_ops_0::ACE_OS::allocation_granularity;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS14argv_to_stringEiPPcRS0_bb"]
        pub fn argv_to_string_u240466127d20addd(
            argc: libc::c_int,
            argv: *mut *mut libc::c_char,
            buf: *mut *mut libc::c_char,
            substitute_env_args: bool,
            quote_args: bool,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS14argv_to_stringEPPcRS0_bb"]
        pub fn argv_to_string(
            argv: *mut *mut libc::c_char,
            buf: *mut *mut libc::c_char,
            substitute_env_args: bool,
            quote_args: bool,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::chdir_u77b018072c3d9162;
    pub use crate::full_ops_0::ACE_OS::chdir_u776c20072c03d22e;
    pub use crate::full_ops_0::ACE_OS::rmdir_u55c427815a4e2c1c;
    pub use crate::full_ops_0::ACE_OS::rmdir_u55ecaf815a706548;
    pub use crate::full_ops_0::ACE_OS::close_u07c1eb6343f8c667;
    pub use crate::full_ops_0::ACE_OS::dup_u8dba25e4b6f24d00;
    pub use crate::full_ops_0::ACE_OS::dup_u80d980bec1e84cf8;
    pub use crate::full_ops_0::ACE_OS::dup2_uc67e543d2abcb368;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS5execlEPKcS1_z"]
        pub fn execl_ub81cd3482485e0c0(
            path: *const libc::c_char,
            arg0: *const libc::c_char,
            ...
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6execleEPKcS1_z"]
        pub fn execle_u057fbff73b71fbb9(
            path: *const libc::c_char,
            arg0: *const libc::c_char,
            ...
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6execlpEPKcS1_z"]
        pub fn execlp_u53e722b3191e043a(
            file: *const libc::c_char,
            arg0: *const libc::c_char,
            ...
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::execv_u2d5c5effd88bba3e;
    pub use crate::full_ops_0::ACE_OS::execve_uc4d493e58c8f0655;
    pub use crate::full_ops_0::ACE_OS::execvp_u52a756775140b0d4;
    pub use crate::full_ops_0::ACE_OS::fork_ud832228bf11480c3;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS4forkEPKc"]
        pub fn fork_uc56682233d614dbc(program_name: *const libc::c_char) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS9fork_execEPPc"]
        pub fn fork_exec(argv: *mut *mut libc::c_char) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::fsync_uc3bcf0de9f93b7e0;
    pub use crate::full_ops_0::ACE_OS::ftruncate_uc81bc2f492b8838e;
    pub use crate::full_ops_0::ACE_OS::getcwd_u59ba0e69ac4b4b7e;
    pub use crate::full_ops_0::ACE_OS::getcwd_udb544f106a3553ea;
    pub use crate::full_ops_0::ACE_OS::getgid_ubba9fd39b1327a93;
    pub use crate::full_ops_0::ACE_OS::getegid_uc39dbea24b4f3558;
    pub use crate::full_ops_0::ACE_OS::getopt_uf5bf2969536d8e54;
    pub use crate::full_ops_0::ACE_OS::getpagesize_u45b438574b8edc31;
    pub use crate::full_ops_0::ACE_OS::getpgid_u40c8520cead7e48b;
    pub use crate::full_ops_0::ACE_OS::getpid_ucc9c2c9e176d9ca8;
    pub use crate::full_ops_0::ACE_OS::getppid_ueab0d2dc69dce544;
    pub use crate::full_ops_0::ACE_OS::getuid_udb2e58c7bca9eb99;
    pub use crate::full_ops_0::ACE_OS::geteuid_u833f5a0c3ae648fa;
    pub use crate::full_ops_0::ACE_OS::hostname;
    pub use crate::full_ops_0::ACE_OS::hostname_u09c889fea4956c0d;
    pub use crate::full_ops_0::ACE_OS::isatty_u2dcfdd2284f4a431;
    pub use crate::full_ops_0::ACE_OS::lseek_u7c6593c8f5712dbe;
    pub use crate::full_ops_0::ACE_OS::llseek;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS14num_processorsEv"]
        pub fn num_processors() -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS21num_processors_onlineEv"]
        pub fn num_processors_online() -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE_OS::pipe_u635316640e09601f;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS5preadEiPvml"]
        pub fn pread_u1b611c818e4074d0(
            handle: libc::c_int,
            buf: *mut libc::c_void,
            nbyte: libc::c_ulong,
            offset: libc::c_long,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6pwriteEiPKvml"]
        pub fn pwrite_u004420a1c470f7a8(
            handle: libc::c_int,
            buf: *const libc::c_void,
            nbyte: libc::c_ulong,
            offset: libc::c_long,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE_OS::read_u821a89e28b7331a1;
    pub use crate::full_ops_0::ACE_OS::read_uacaa68bd37a20f7b;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6read_nEiPvmPm"]
        pub fn read_n_ud44962ad8cae18c7(
            handle: libc::c_int,
            buf: *mut libc::c_void,
            len: libc::c_ulong,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE_OS::readlink_u50893a169239ebd1;
    pub use crate::full_ops_0::ACE_OS::sbrk_u1c803ce7ca4578c0;
    pub use crate::full_ops_0::ACE_OS::setgid_ub95cfd531ee84f33;
    pub use crate::full_ops_0::ACE_OS::setegid_uc56c94f341dcc67c;
    pub use crate::full_ops_0::ACE_OS::setpgid_u242b3044f4f12393;
    pub use crate::full_ops_0::ACE_OS::setregid_u50366ede2d592b3e;
    pub use crate::full_ops_0::ACE_OS::setreuid_u70d13597a915cfc4;
    pub use crate::full_ops_0::ACE_OS::setsid_ua5c41d8548bb754b;
    pub use crate::full_ops_0::ACE_OS::setuid_ud46f4fdcc4c92835;
    pub use crate::full_ops_0::ACE_OS::seteuid_u9ade6efc9e19b83a;
    pub use crate::full_ops_0::ACE_OS::sleep_u6237c3be3be7aa52;
    pub use crate::full_ops_0::ACE_OS::sleep_u1b7b7e36e28cf584;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS14string_to_argvEPcRiRPS0_b"]
        pub fn string_to_argv(
            buf: *mut libc::c_char,
            argc: *mut libc::c_int,
            argv: *mut *mut *mut libc::c_char,
            substitute_env_args: bool,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::swab_u7c4f2e50bd3808a6;
    pub use crate::full_ops_0::ACE_OS::sysconf_ufbfaa6ac06fab24e;
    pub use crate::full_ops_0::ACE_OS::sysinfo;
    pub use crate::full_ops_0::ACE_OS::truncate_u5efbca46e55bf033;
    pub use crate::full_ops_0::ACE_OS::ualarm_u2e796b6d2f58f7c7;
    pub use crate::full_ops_0::ACE_OS::ualarm_u008923c59d4cdea2;
    pub use crate::full_ops_0::ACE_OS::unlink_ucc448fde77c19cbd;
    pub use crate::full_ops_0::ACE_OS::unlink_ucc6d97de77e4af69;
    pub use crate::full_ops_0::ACE_OS::write_uba4fd70f52b99861;
    pub use crate::full_ops_0::ACE_OS::write_uad61deb925fc5dbb;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7write_nEiPKvmPm"]
        pub fn write_n_u8774988f05bbbbd1(
            handle: libc::c_int,
            buf: *const libc::c_void,
            len: libc::c_ulong,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS5unameEP7utsname"]
        pub fn uname_u87f3d152753a529e(name: *mut super::utsname) -> libc::c_int;
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
    pub use crate::full_ops_0::ACE_OS::accept_u8430b27f80401846;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6acceptEiP8sockaddrPiRK21ACE_Accept_QoS_Params"]
        pub fn accept_u01a5c7d903c3928a(
            handle: libc::c_int,
            addr: *mut super::sockaddr,
            addrlen: *mut libc::c_int,
            qos_params: *const super::ACE_Accept_QoS_Params,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::bind_uef4d981726093bc5;
    pub use crate::full_ops_0::ACE_OS::closesocket;
    pub use crate::full_ops_0::ACE_OS::connect_u21719e7a4d49e872;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7connectEiPK8sockaddriRK14ACE_QoS_Params"]
        pub fn connect_uc00b8b6355089164(
            handle: libc::c_int,
            addr: *const super::sockaddr,
            addrlen: libc::c_int,
            qos_params: *const super::ACE_QoS_Params,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::enum_protocols;
    pub use crate::full_ops_0::ACE_OS::getpeername_uad82bd4458b2c991;
    pub use crate::full_ops_0::ACE_OS::getsockname_ua0d322ba11951c19;
    pub use crate::full_ops_0::ACE_OS::getsockopt_ue319435db01d3776;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS9join_leafEiPK8sockaddriRK14ACE_QoS_Params"]
        pub fn join_leaf(
            socket: libc::c_int,
            name: *const super::sockaddr,
            namelen: libc::c_int,
            qos_params: *const super::ACE_QoS_Params,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::listen_u279ec0941f56712e;
    pub use crate::full_ops_0::ACE_OS::recv_u32019f19ae792060;
    pub use crate::full_ops_0::ACE_OS::recvfrom_u289b3b4f999bdcf7;
    pub use crate::full_ops_0::ACE_OS::recvfrom_ued0283f1caeddad0;
    pub use crate::full_ops_0::ACE_OS::recvmsg_u2eff40656fa046cd;
    pub use crate::full_ops_0::ACE_OS::recvv_u5bf3764b3b283ae9;
    pub use crate::full_ops_0::ACE_OS::send_u46b905aa4005e9e7;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS14send_partial_iEiPKcmi"]
        pub fn send_partial_i(
            handle: libc::c_int,
            buf: *const libc::c_char,
            len: libc::c_ulong,
            flags: libc::c_int,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE_OS::sendmsg_ue8fec1119c766cbe;
    pub use crate::full_ops_0::ACE_OS::sendto_uf1a8458778d74112;
    pub use crate::full_ops_0::ACE_OS::sendto_ub6e6c93c067ba01d;
    pub use crate::full_ops_0::ACE_OS::sendv_u2741ac8ad81ed9fe;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS15sendv_partial_iEiPK5ioveci"]
        pub fn sendv_partial_i(
            handle: libc::c_int,
            iov: *const super::iovec,
            iovcnt: libc::c_int,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE_OS::setsockopt_u491187844361ffa3;
    pub use crate::full_ops_0::ACE_OS::shutdown_u7313014062aea7b1;
    pub use crate::full_ops_0::ACE_OS::if_nametoindex_ubbaac5483350f21a;
    pub use crate::full_ops_0::ACE_OS::if_indextoname_ue23378cfbda0d15f;
    pub use crate::full_ops_0::ACE_OS::if_nameindex_ua9a99c9914fbbb2c;
    pub use crate::full_ops_0::ACE_OS::if_freenameindex_u1fa9da4f500f38cb;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS11socket_initEii"]
        pub fn socket_init(
            version_high: libc::c_int,
            version_low: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS11socket_finiEv"]
        pub fn socket_fini() -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::socket_u707bc17eb37e5e24;
    pub use crate::full_ops_0::ACE_OS::socket_u03f77517e3b1f817;
    pub use crate::full_ops_0::ACE_OS::socketpair_u1b7fa3fde1c9db8e;
    pub use crate::full_ops_0::ACE_OS::readv_u0f79db0b81259088;
    pub use crate::full_ops_0::ACE_OS::writev_u9b20468f161f7ab3;
    pub use crate::full_ops_0::ACE_OS::ace_flock_t;
    pub use crate::full_ops_0::ACE_OS::clearerr_ue05057310b33b5c9;
    pub use crate::full_ops_0::ACE_OS::cuserid_u902792616be43eeb;
    pub use crate::full_ops_0::ACE_OS::cuserid_u11f351bab0dd5f7f;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS8asprintfEPPcPKcz"]
        pub fn asprintf_u65a187f722295b8d(
            bufp: *mut *mut libc::c_char,
            format: *const libc::c_char,
            ...
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS8asprintfEPPwPKwz"]
        pub fn asprintf_u12c5997d8588a495(
            bufp: *mut *mut libc::wchar_t,
            format: *const libc::wchar_t,
            ...
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::fclose_u07d30b70956bcb1f;
    pub use crate::full_ops_0::ACE_OS::fdopen_u3da0f96d0ebfd1da;
    pub use crate::full_ops_0::ACE_OS::fflush_ub53d4035e1b86ad5;
    pub use crate::full_ops_0::ACE_OS::fgetc_ua8ec9df85630c044;
    pub use crate::full_ops_0::ACE_OS::getc_u4951814d41ae1ebe;
    pub use crate::full_ops_0::ACE_OS::fgetpos_u4ae5c92132a9b3a7;
    pub use crate::full_ops_0::ACE_OS::fgets_uafd841fb54fd292c;
    pub use crate::full_ops_0::ACE_OS::fgets_u39129b14cfbe6f08;
    pub use crate::full_ops_0::ACE_OS::flock_init;
    pub use crate::full_ops_0::ACE_OS::flock_destroy;
    pub use crate::full_ops_0::ACE_OS::flock_rdlock;
    pub use crate::full_ops_0::ACE_OS::flock_tryrdlock;
    pub use crate::full_ops_0::ACE_OS::flock_trywrlock;
    pub use crate::full_ops_0::ACE_OS::flock_unlock;
    pub use crate::full_ops_0::ACE_OS::flock_wrlock;
    pub use crate::full_ops_0::ACE_OS::fileno_u68414c3a6d7d791a;
    pub use crate::full_ops_0::ACE_OS::fopen_ua6d43bbd3a566b87;
    pub use crate::full_ops_0::ACE_OS::fopen_u3ca562cdba122729;
    pub use crate::full_ops_0::ACE_OS::fopen_u0fee9b8df0a102d3;
    pub use crate::full_ops_0::ACE_OS::fopen_u62b78f4d172e84a1;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7fprintfEP8_IO_FILEPKcz"]
        pub fn fprintf_u7a83295f0e5d324f(
            fp: *mut super::_IO_FILE,
            format: *const libc::c_char,
            ...
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7fprintfEP8_IO_FILEPKwz"]
        pub fn fprintf_uce705d5ead3af64b(
            fp: *mut super::_IO_FILE,
            format: *const libc::wchar_t,
            ...
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::ungetc_u60080a2bf4a1a7a7;
    pub use crate::full_ops_0::ACE_OS::fputc_ub7ff014eae86c7e1;
    pub use crate::full_ops_0::ACE_OS::putc_ua4055e823273f55b;
    pub use crate::full_ops_0::ACE_OS::fputs_ue90ba72683355fc6;
    pub use crate::full_ops_0::ACE_OS::fputs_u4a81b8bac701ca52;
    pub use crate::full_ops_0::ACE_OS::fread_u7e9e336bcc2b5ed8;
    pub use crate::full_ops_0::ACE_OS::freopen_u78b7073518dbb152;
    pub use crate::full_ops_0::ACE_OS::fseek_u50b3da0cc13ebc1e;
    pub use crate::full_ops_0::ACE_OS::fsetpos_u29381f0d0a3c01f3;
    pub use crate::full_ops_0::ACE_OS::ftell_u0eeaa41898f5c0c6;
    pub use crate::full_ops_0::ACE_OS::fwrite_u88345e2f33e8c57c;
    pub use crate::full_ops_0::ACE_OS::perror_u62bb6eeba7fd9162;
    pub use crate::full_ops_0::ACE_OS::perror_u627776eba7c3d22e;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6printfEPKcz"]
        pub fn printf_u3be153d67f031433(format: *const libc::c_char, ...) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6printfEPKwz"]
        pub fn printf_ue93a1fd6e13a12d7(
            format: *const libc::wchar_t,
            ...
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::puts_u99e7d5550f5ecb86;
    pub use crate::full_ops_0::ACE_OS::puts_u99bf0d550f3c259a;
    pub use crate::full_ops_0::ACE_OS::rename_u44ba87123659fee3;
    pub use crate::full_ops_0::ACE_OS::rename_u16513e459cac8bcf;
    pub use crate::full_ops_0::ACE_OS::rewind_u949a149e78ad06a4;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS8snprintfEPcmPKcz"]
        pub fn snprintf_u9a28649a3b771eeb(
            buf: *mut libc::c_char,
            maxlen: libc::c_ulong,
            format: *const libc::c_char,
            ...
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS8snprintfEPwmPKwz"]
        pub fn snprintf_u649150efea9fee6b(
            buf: *mut libc::wchar_t,
            maxlen: libc::c_ulong,
            format: *const libc::wchar_t,
            ...
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7sprintfEPcPKcz"]
        pub fn sprintf_u124e1cc3d4c1a9fc(
            buf: *mut libc::c_char,
            format: *const libc::c_char,
            ...
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS7sprintfEPwPKwz"]
        pub fn sprintf_ud7576b55745c336c(
            buf: *mut libc::wchar_t,
            format: *const libc::wchar_t,
            ...
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::tempnam_u14bf393544678d3f;
    pub use crate::full_ops_0::ACE_OS::tempnam_u52d05a0f876a0beb;
    pub use crate::full_ops_0::ACE_OS::vasprintf_uf99bc2083182119a;
    pub use crate::full_ops_0::ACE_OS::vprintf_u01e81082ba4c3478;
    pub use crate::full_ops_0::ACE_OS::vfprintf_u987d4e89a24e4338;
    pub use crate::full_ops_0::ACE_OS::vsprintf_u244a12457fe38a73;
    pub use crate::full_ops_0::ACE_OS::vsnprintf_u82a2749143e3ac84;
    pub use crate::full_ops_0::ACE_OS::vasprintf_u116ec636510b91c2;
    pub use crate::full_ops_0::ACE_OS::vprintf_uebd9f98ac54a526c;
    pub use crate::full_ops_0::ACE_OS::vfprintf_u56d71c75cec59c2c;
    pub use crate::full_ops_0::ACE_OS::vsprintf_uceb65c606a90f8d3;
    pub use crate::full_ops_0::ACE_OS::vsnprintf_u5faa4708bec76a1c;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS20vaswprintf_emulationEPPwPKwP13__va_list_tag"]
        pub fn vaswprintf_emulation(
            bufp: *mut *mut libc::wchar_t,
            format: *const libc::wchar_t,
            argptr: ::core::ffi::VaList<'_>,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::endpwent_u2d594e606f92f98c;
    pub use crate::full_ops_0::ACE_OS::getpwent_uda1d578186579c79;
    pub use crate::full_ops_0::ACE_OS::getpwnam_u0e28381801657da9;
    pub use crate::full_ops_0::ACE_OS::getpwnam_r_u758ea1eeedd9ccc5;
    pub use crate::full_ops_0::ACE_OS::setpwent_u99dca3d77dc469b5;
    pub use crate::full_ops_0::ACE_OS::creat_ubfc3785cacc9200f;
    pub use crate::full_ops_0::ACE_OS::filesize_uc42c876f8a2ca43a;
    pub use crate::full_ops_0::ACE_OS::filesize;
    pub use crate::full_ops_0::ACE_OS::fstat_u4a66c24ea8739c1b;
    pub use crate::full_ops_0::ACE_OS::lstat_uda0ff31bc6712b4a;
    pub use crate::full_ops_0::ACE_OS::lstat_u2a3cf10dc49a71ae;
    pub use crate::full_ops_0::ACE_OS::mkdir_ucfe7d85d9c86c941;
    pub use crate::full_ops_0::ACE_OS::mkdir_u520989045ae37695;
    pub use crate::full_ops_0::ACE_OS::mkfifo_u463594cc85e8e50a;
    pub use crate::full_ops_0::ACE_OS::stat_u2ae4cb015063d662;
    pub use crate::full_ops_0::ACE_OS::stat_ufb146cfc36e30e66;
    pub use crate::full_ops_0::ACE_OS::umask_uc870dde6e298b316;
    pub use crate::full_ops_0::ACE_OS::kill_udafa4deef137b5a3;
    pub use crate::full_ops_0::ACE_OS::pthread_sigmask_u533b55567f192961;
    pub use crate::full_ops_0::ACE_OS::sigaction_uf038829092b02270;
    pub use crate::full_ops_0::ACE_OS::sigaddset_u87d182dd58bf700c;
    pub use crate::full_ops_0::ACE_OS::sigdelset_u44b27dd4262df326;
    pub use crate::full_ops_0::ACE_OS::sigemptyset_udf9bdeb276d35426;
    pub use crate::full_ops_0::ACE_OS::sigfillset_ubda4b03e91b23d38;
    pub use crate::full_ops_0::ACE_OS::sigismember_u8594864a428d04c3;
    pub use crate::full_ops_0::ACE_OS::signal_u868eb443292c1ed5;
    pub use crate::full_ops_0::ACE_OS::sigprocmask_u59dc781e76b5dcae;
    pub use crate::full_ops_0::ACE_OS::sigsuspend_u142bb65bb8557f0c;
    pub use crate::full_ops_0::ACE_OS::raise_u1a23e0b80c8bee52;
    extern "C" {
        pub static mut NULL_thread: libc::c_ulong;
    }
    extern "C" {
        pub static mut NULL_hthread: libc::c_ulong;
    }
    extern "C" {
        pub static mut NULL_key: libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS11cleanup_tssEj"]
        pub fn cleanup_tss(main_thread: libc::c_uint);
    }
    pub use crate::full_ops_0::ACE_OS::condattr_init;
    pub use crate::full_ops_0::ACE_OS::condattr_synctype;
    pub use crate::full_ops_0::ACE_OS::condattr_destroy;
    pub use crate::full_ops_0::ACE_OS::condattr_setclock;
    pub use crate::full_ops_0::ACE_OS::cond_broadcast;
    pub use crate::full_ops_0::ACE_OS::cond_destroy;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS9cond_initEP14pthread_cond_tsPKcPv"]
        pub fn cond_init_ue107a1ce456396d5(
            cv: *mut super::pthread_cond_t,
            r#type: libc::c_short,
            name: *const libc::c_char,
            arg: *mut libc::c_void,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::cond_init;
    pub use crate::full_ops_0::ACE_OS::cond_init_ue269727c7a5fd051;
    pub use crate::full_ops_0::ACE_OS::cond_init_u9382a67582494ab7;
    pub use crate::full_ops_0::ACE_OS::cond_signal;
    pub use crate::full_ops_0::ACE_OS::cond_timedwait;
    pub use crate::full_ops_0::ACE_OS::cond_wait;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS13event_destroyEP11ACE_event_t"]
        pub fn event_destroy(event: *mut super::ACE_event_t) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::event_init_u47e307a4cf3a9d59;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS10event_initEP11ACE_event_tiP18pthread_condattr_tiiPKcPvi"]
        pub fn event_init(
            event: *mut super::ACE_event_t,
            r#type: libc::c_int,
            attributes: *mut super::pthread_condattr_t,
            manual_reset: libc::c_int,
            initial_state: libc::c_int,
            name: *const libc::c_char,
            arg: *mut libc::c_void,
            sa: libc::c_int,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::event_init_u301de5fc0c8e521d;
    pub use crate::full_ops_0::ACE_OS::event_init_uded8cfdb7e8126e1;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS11event_pulseEP11ACE_event_t"]
        pub fn event_pulse(event: *mut super::ACE_event_t) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS11event_resetEP11ACE_event_t"]
        pub fn event_reset(event: *mut super::ACE_event_t) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS12event_signalEP11ACE_event_t"]
        pub fn event_signal(event: *mut super::ACE_event_t) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS15event_timedwaitEP11ACE_event_tP14ACE_Time_Valuei"]
        pub fn event_timedwait(
            event: *mut super::ACE_event_t,
            timeout: *mut super::ACE_Time_Value,
            use_absolute_time: libc::c_int,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::event_wait;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS13lwp_getparamsER16ACE_Sched_Params"]
        pub fn lwp_getparams(_anon_0: *mut super::ACE_Sched_Params) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS13lwp_setparamsERK16ACE_Sched_Params"]
        pub fn lwp_setparams(_anon_0: *const super::ACE_Sched_Params) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS13mutex_destroyEP15pthread_mutex_t"]
        pub fn mutex_destroy(m: *mut super::pthread_mutex_t) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS10mutex_initEP15pthread_mutex_tiPKcP19pthread_mutexattr_tii"]
        pub fn mutex_init(
            m: *mut super::pthread_mutex_t,
            lock_scope: libc::c_int,
            name: *const libc::c_char,
            arg: *mut super::pthread_mutexattr_t,
            sa: libc::c_int,
            lock_type: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS10mutex_initEP15pthread_mutex_tiPKwP19pthread_mutexattr_tii"]
        pub fn mutex_init_u6e18c531e078042b(
            m: *mut super::pthread_mutex_t,
            lock_scope: libc::c_int,
            name: *const libc::wchar_t,
            arg: *mut super::pthread_mutexattr_t,
            sa: libc::c_int,
            lock_type: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS10mutex_lockEP15pthread_mutex_t"]
        pub fn mutex_lock(m: *mut super::pthread_mutex_t) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS10mutex_lockEP15pthread_mutex_tRi"]
        pub fn mutex_lock_u0c1e8676d442c3cc(
            m: *mut super::pthread_mutex_t,
            abandoned: *mut libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS10mutex_lockEP15pthread_mutex_tRK14ACE_Time_Value"]
        pub fn mutex_lock_uc739b38639033558(
            m: *mut super::pthread_mutex_t,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::mutex_lock_uff44566eb337270c;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS18mutex_lock_cleanupEPv"]
        pub fn mutex_lock_cleanup(mutex: *mut libc::c_void);
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS13mutex_trylockEP15pthread_mutex_t"]
        pub fn mutex_trylock(m: *mut super::pthread_mutex_t) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS13mutex_trylockEP15pthread_mutex_tRi"]
        pub fn mutex_trylock_u4cc89e8105f89a99(
            m: *mut super::pthread_mutex_t,
            abandoned: *mut libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS12mutex_unlockEP15pthread_mutex_t"]
        pub fn mutex_unlock(m: *mut super::pthread_mutex_t) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::priority_control;
    pub use crate::full_ops_0::ACE_OS::recursive_mutex_cond_unlock;
    pub use crate::full_ops_0::ACE_OS::recursive_mutex_cond_relock;
    pub use crate::full_ops_0::ACE_OS::recursive_mutex_destroy;
    pub use crate::full_ops_0::ACE_OS::recursive_mutex_init;
    pub use crate::full_ops_0::ACE_OS::recursive_mutex_lock;
    pub use crate::full_ops_0::ACE_OS::recursive_mutex_lock_ub8a8d00bad55dd79;
    pub use crate::full_ops_0::ACE_OS::recursive_mutex_lock_ub73fefaba0bdcd05;
    pub use crate::full_ops_0::ACE_OS::recursive_mutex_trylock;
    pub use crate::full_ops_0::ACE_OS::recursive_mutex_unlock;
    pub use crate::full_ops_0::ACE_OS::rw_rdlock;
    pub use crate::full_ops_0::ACE_OS::rw_tryrdlock;
    pub use crate::full_ops_0::ACE_OS::rw_trywrlock;
    pub use crate::full_ops_0::ACE_OS::rw_trywrlock_upgrade;
    pub use crate::full_ops_0::ACE_OS::rw_unlock;
    pub use crate::full_ops_0::ACE_OS::rw_wrlock;
    pub use crate::full_ops_0::ACE_OS::rwlock_destroy;
    pub use crate::full_ops_0::ACE_OS::rwlock_init;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS12sched_paramsERK16ACE_Sched_Paramsl"]
        pub fn sched_params(
            _anon_0: *const super::ACE_Sched_Params,
            id: libc::c_long,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS16scheduling_classEPKcRl"]
        pub fn scheduling_class(
            class_name: *const libc::c_char,
            _anon_1: *mut libc::c_long,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::sema_destroy;
    pub use crate::full_ops_0::ACE_OS::sema_init_u1271b3e1cae4c1a8;
    pub use crate::full_ops_0::ACE_OS::sema_init;
    pub use crate::full_ops_0::ACE_OS::sema_init_u66dd41daa44b07fc;
    pub use crate::full_ops_0::ACE_OS::sema_init_u8b833f501fd35ff2;
    pub use crate::full_ops_0::ACE_OS::sema_avoid_unlink;
    pub use crate::full_ops_0::ACE_OS::sema_unlink;
    pub use crate::full_ops_0::ACE_OS::sema_post;
    pub use crate::full_ops_0::ACE_OS::sema_post_u0b7e2d3bf2a93de8;
    pub use crate::full_ops_0::ACE_OS::sema_trywait;
    pub use crate::full_ops_0::ACE_OS::sema_wait;
    pub use crate::full_ops_0::ACE_OS::sema_wait_ue969f83bc0266c04;
    pub use crate::full_ops_0::ACE_OS::sema_wait_u98411807670c2b18;
    pub use crate::full_ops_0::ACE_OS::semctl_ued1baeab063da9d3;
    pub use crate::full_ops_0::ACE_OS::semget_u459a57c0bbce4950;
    pub use crate::full_ops_0::ACE_OS::semop_u80b06a9c8b5a703c;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS21set_scheduling_paramsERK16ACE_Sched_Paramsl"]
        pub fn set_scheduling_params(
            _anon_0: *const super::ACE_Sched_Params,
            id: libc::c_long,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::sigtimedwait_uc938a6d86bfdc237;
    pub use crate::full_ops_0::ACE_OS::sigwait_ud14aa508eada76f2;
    pub use crate::full_ops_0::ACE_OS::sigwaitinfo_ufc951fc5a90c285c;
    pub use crate::full_ops_0::ACE_OS::thr_cancel;
    pub use crate::full_ops_0::ACE_OS::thr_cmp;
    pub use crate::full_ops_0::ACE_OS::thr_continue;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS10thr_createEPFPvS0_ES0_lPmS3_lS0_mP23ACE_Base_Thread_AdapterPPKc"]
        pub fn thr_create(
            func: Option<
                unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
            >,
            args: *mut libc::c_void,
            flags: libc::c_long,
            thr_id: *mut libc::c_ulong,
            t_handle: *mut libc::c_ulong,
            priority: libc::c_long,
            stack: *mut libc::c_void,
            stacksize: libc::c_ulong,
            thread_adapter: *mut super::ACE_Base_Thread_Adapter,
            thr_name: *mut *const libc::c_char,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::thr_equal;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS8thr_exitEPv"]
        pub fn thr_exit(status: *mut libc::c_void);
    }
    pub use crate::full_ops_0::ACE_OS::thr_getconcurrency;
    pub use crate::full_ops_0::ACE_OS::thr_getprio;
    pub use crate::full_ops_0::ACE_OS::thr_getprio_u470508780c2aa1c6;
    pub use crate::full_ops_0::ACE_OS::thr_getspecific_native;
    pub use crate::full_ops_0::ACE_OS::thr_getspecific;
    pub use crate::full_ops_0::ACE_OS::thr_join;
    pub use crate::full_ops_0::ACE_OS::thr_join_u1274f10f177d1aed;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS16thr_get_affinityEmmP9cpu_set_t"]
        pub fn thr_get_affinity(
            thr_id: libc::c_ulong,
            cpu_set_size: libc::c_ulong,
            cpu_mask: *mut super::cpu_set_t,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS16thr_set_affinityEmmPK9cpu_set_t"]
        pub fn thr_set_affinity(
            thr_id: libc::c_ulong,
            cpu_set_size: libc::c_ulong,
            cpu_mask: *const super::cpu_set_t,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS14thr_key_detachEj"]
        pub fn thr_key_detach(key: libc::c_uint) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS12thr_key_usedEj"]
        pub fn thr_key_used(key: libc::c_uint) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS20thr_keycreate_nativeEPjPFvPvE"]
        pub fn thr_keycreate_native(
            key: *mut libc::c_uint,
            _anon_1: Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS13thr_keycreateEPjPFvPvE"]
        pub fn thr_keycreate(
            key: *mut libc::c_uint,
            _anon_1: Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS18thr_keyfree_nativeEj"]
        pub fn thr_keyfree_native(key: libc::c_uint) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS11thr_keyfreeEj"]
        pub fn thr_keyfree(key: libc::c_uint) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::thr_kill;
    pub use crate::full_ops_0::ACE_OS::thr_min_stack;
    pub use crate::full_ops_0::ACE_OS::thr_self;
    pub use crate::full_ops_0::ACE_OS::thr_self_u35bfcd9d5906cbcf;
    pub use crate::full_ops_0::ACE_OS::thr_name;
    pub use crate::full_ops_0::ACE_OS::thr_id;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS10thr_gettidEv"]
        pub fn thr_gettid() -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::thr_gettid_uca28ac7180496daa;
    pub use crate::full_ops_0::ACE_OS::thr_setcancelstate;
    pub use crate::full_ops_0::ACE_OS::thr_setcanceltype;
    pub use crate::full_ops_0::ACE_OS::thr_setconcurrency;
    pub use crate::full_ops_0::ACE_OS::thr_setprio_ucfa01c0139522e03;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS11thr_setprioEi"]
        pub fn thr_setprio(prio: libc::c_int) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS22thr_setspecific_nativeEjPv"]
        pub fn thr_setspecific_native(
            key: libc::c_uint,
            data: *mut libc::c_void,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS15thr_setspecificEjPv"]
        pub fn thr_setspecific(
            key: libc::c_uint,
            data: *mut libc::c_void,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::thr_sigsetmask;
    pub use crate::full_ops_0::ACE_OS::thr_suspend;
    pub use crate::full_ops_0::ACE_OS::thr_testcancel;
    pub use crate::full_ops_0::ACE_OS::thr_yield;
    pub use crate::full_ops_0::ACE_OS::thread_mutex_destroy;
    pub use crate::full_ops_0::ACE_OS::thread_mutex_init;
    pub use crate::full_ops_0::ACE_OS::thread_mutex_init_u7211d54e5d52dd40;
    pub use crate::full_ops_0::ACE_OS::thread_mutex_lock;
    pub use crate::full_ops_0::ACE_OS::thread_mutex_lock_u60ab414c30bff129;
    pub use crate::full_ops_0::ACE_OS::thread_mutex_lock_u00620e1377f06cf5;
    pub use crate::full_ops_0::ACE_OS::thread_mutex_trylock;
    pub use crate::full_ops_0::ACE_OS::thread_mutex_unlock;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS11unique_nameEPKvPcm"]
        pub fn unique_name_u335a6b57ecb175f1(
            object: *const libc::c_void,
            name: *mut libc::c_char,
            length: libc::c_ulong,
        );
    }
    pub use crate::full_ops_0::ACE_OS::madvise_udf79f790df65d77b;
    pub use crate::full_ops_0::ACE_OS::mmap_uf18c0366a1aaeae8;
    pub use crate::full_ops_0::ACE_OS::mprotect_ufb2560969f20c857;
    pub use crate::full_ops_0::ACE_OS::msync_u3075bc10fec907f5;
    pub use crate::full_ops_0::ACE_OS::munmap_u78403a470c5b78d3;
    pub use crate::full_ops_0::ACE_OS::shm_open_u8832aeab90456f8d;
    pub use crate::full_ops_0::ACE_OS::shm_unlink_u95e94a846074d1ac;
    pub use crate::full_ops_0::ACE_OS::gettimeofday_u3220bcbbceb90f45;
    pub use crate::full_ops_0::ACE_OS::gettimeofday_;
    pub use crate::full_ops_0::ACE_OS::ace_isalnum;
    pub use crate::full_ops_0::ACE_OS::ace_isalpha;
    pub use crate::full_ops_0::ACE_OS::ace_isblank;
    pub use crate::full_ops_0::ACE_OS::ace_isascii;
    pub use crate::full_ops_0::ACE_OS::ace_iscntrl;
    pub use crate::full_ops_0::ACE_OS::ace_isdigit;
    pub use crate::full_ops_0::ACE_OS::ace_isgraph;
    pub use crate::full_ops_0::ACE_OS::ace_islower;
    pub use crate::full_ops_0::ACE_OS::ace_isprint;
    pub use crate::full_ops_0::ACE_OS::ace_ispunct;
    pub use crate::full_ops_0::ACE_OS::ace_isspace;
    pub use crate::full_ops_0::ACE_OS::ace_isupper;
    pub use crate::full_ops_0::ACE_OS::ace_isxdigit;
    pub use crate::full_ops_0::ACE_OS::ace_tolower;
    pub use crate::full_ops_0::ACE_OS::ace_towlower;
    pub use crate::full_ops_0::ACE_OS::ace_toupper;
    pub use crate::full_ops_0::ACE_OS::ace_towupper;
    pub use crate::full_ops_0::ACE_OS::ace_isctype;
    pub use crate::full_ops_0::ACE_OS::inet_addr_ue24d339db234ab04;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS9inet_atonEPKcP7in_addr"]
        pub fn inet_aton_u1dc29b8522f89aaa(
            strptr: *const libc::c_char,
            addr: *mut super::in_addr,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE_OS::inet_ntoa_u1583121b3c107924;
    pub use crate::full_ops_0::ACE_OS::inet_ntop_ufa1224988bfba6dc;
    pub use crate::full_ops_0::ACE_OS::inet_pton_u954637153700152b;
}
extern "C-unwind" {
    pub fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
}
extern "C-unwind" {
    pub fn stat64(__file: *const libc::c_char, __buf: *mut stat64) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fstat64(__fd: libc::c_int, __buf: *mut stat64) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fstatat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fstatat64(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __buf: *mut stat64,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
}
extern "C-unwind" {
    pub fn lstat64(__file: *const libc::c_char, __buf: *mut stat64) -> libc::c_int;
}
extern "C-unwind" {
    pub fn chmod(__file: *const libc::c_char, __mode: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn lchmod(__file: *const libc::c_char, __mode: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fchmod(__fd: libc::c_int, __mode: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fchmodat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __mode: libc::c_uint,
        __flag: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn umask(__mask: libc::c_uint) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn getumask() -> libc::c_uint;
}
extern "C-unwind" {
    pub fn mkdir(__path: *const libc::c_char, __mode: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkdirat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __mode: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mknod(
        __path: *const libc::c_char,
        __mode: libc::c_uint,
        __dev: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mknodat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __mode: libc::c_uint,
        __dev: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkfifo(__path: *const libc::c_char, __mode: libc::c_uint) -> libc::c_int;
}
extern "C-unwind" {
    pub fn mkfifoat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __mode: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
}
extern "C-unwind" {
    pub fn statx(
        __dirfd: libc::c_int,
        __path: *const libc::c_char,
        __flags: libc::c_int,
        __mask: libc::c_uint,
        __buf: *mut statx,
    ) -> libc::c_int;
}
pub mod ACE {
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9set_flagsEii"]
        pub fn set_flags(handle: libc::c_int, flags: libc::c_int) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9clr_flagsEii"]
        pub fn clr_flags(handle: libc::c_int, flags: libc::c_int) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE::get_flags;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE13major_versionEv"]
        pub fn major_version() -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE13minor_versionEv"]
        pub fn minor_version() -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE13micro_versionEv"]
        pub fn micro_version() -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE12beta_versionEv"]
        pub fn beta_version() -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE13compiler_nameEv"]
        pub fn compiler_name() -> *const libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE22compiler_major_versionEv"]
        pub fn compiler_major_version() -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE22compiler_minor_versionEv"]
        pub fn compiler_minor_version() -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE21compiler_beta_versionEv"]
        pub fn compiler_beta_version() -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE14out_of_handlesEi"]
        pub fn out_of_handles(error: libc::c_int) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE10wild_matchEPKcS1_bb"]
        pub fn wild_match(
            s: *const libc::c_char,
            pattern: *const libc::c_char,
            case_sensitive: bool,
            character_classes: bool,
        ) -> bool;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE4recvEiPvmiPK14ACE_Time_Value"]
        pub fn recv_ubc58a1994981cdc8(
            handle: libc::c_int,
            buf: *mut libc::c_void,
            len: libc::c_ulong,
            flags: libc::c_int,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE4recvEiPvmPK14ACE_Time_Value"]
        pub fn recv_ufcaf818c3c730c74(
            handle: libc::c_int,
            buf: *mut libc::c_void,
            len: libc::c_ulong,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7recvmsgEiP6msghdriPK14ACE_Time_Value"]
        pub fn recvmsg_ud0b6852c2d7ce63c(
            handle: libc::c_int,
            msg: *mut super::msghdr,
            flags: libc::c_int,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8recvfromEiPciiP8sockaddrPiPK14ACE_Time_Value"]
        pub fn recvfrom_u88b4ad8d8c4c2595(
            handle: libc::c_int,
            buf: *mut libc::c_char,
            len: libc::c_int,
            flags: libc::c_int,
            addr: *mut super::sockaddr,
            addrlen: *mut libc::c_int,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE::recv_n_u94a488480e849282;
    pub use crate::full_ops_0::ACE::recv_n_uf7085c287ac65f5e;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE4recvEimz"]
        pub fn recv_ue6095f44fab5805d(
            handle: libc::c_int,
            n: libc::c_ulong,
            ...
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE5recvvEiP5ioveciPK14ACE_Time_Value"]
        pub fn recvv(
            handle: libc::c_int,
            iov: *mut super::iovec,
            iovcnt: libc::c_int,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE::recvv_n;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6recv_nEiP17ACE_Message_BlockPK14ACE_Time_ValuePm"]
        pub fn recv_n(
            handle: libc::c_int,
            message_block: *mut super::ACE_Message_Block,
            timeout: *const super::ACE_Time_Value,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE4sendEiPKvmiPK14ACE_Time_Value"]
        pub fn send_ue0793508d1a7ed9b(
            handle: libc::c_int,
            buf: *const libc::c_void,
            len: libc::c_ulong,
            flags: libc::c_int,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE4sendEiPKvmPK14ACE_Time_Value"]
        pub fn send_u8622bdbd94726127(
            handle: libc::c_int,
            buf: *const libc::c_void,
            len: libc::c_ulong,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7sendmsgEiPK6msghdriPK14ACE_Time_Value"]
        pub fn sendmsg_u4bdd8a5369fa21d3(
            handle: libc::c_int,
            msg: *const super::msghdr,
            flags: libc::c_int,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6sendtoEiPKciiPK8sockaddriPK14ACE_Time_Value"]
        pub fn sendto_u39d71d775d080700(
            handle: libc::c_int,
            buf: *const libc::c_char,
            len: libc::c_int,
            flags: libc::c_int,
            addr: *const super::sockaddr,
            addrlen: libc::c_int,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE::send_n_ucbbbc5a02ac043c9;
    pub use crate::full_ops_0::ACE::send_n_ufa3c0d7f3dea0191;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE4sendEimz"]
        pub fn send_uc53816df24ab747f(
            handle: libc::c_int,
            n: libc::c_ulong,
            ...
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE5sendvEiPK5ioveciPK14ACE_Time_Value"]
        pub fn sendv(
            handle: libc::c_int,
            iov: *const super::iovec,
            iovcnt: libc::c_int,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE::sendv_n;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6send_nEiPK17ACE_Message_BlockPK14ACE_Time_ValuePm"]
        pub fn send_n(
            handle: libc::c_int,
            message_block: *const super::ACE_Message_Block,
            timeout: *const super::ACE_Time_Value,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE::read_n;
    pub use crate::full_ops_0::ACE::write_n_u941d14db176920fc;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7write_nEiPK17ACE_Message_BlockPm"]
        pub fn write_n(
            handle: libc::c_int,
            message_block: *const super::ACE_Message_Block,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7readv_nEiP5ioveciPm"]
        pub fn readv_n(
            handle: libc::c_int,
            iov: *mut super::iovec,
            iovcnt: libc::c_int,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8writev_nEiPK5ioveciPm"]
        pub fn writev_n(
            handle: libc::c_int,
            iov: *const super::iovec,
            iovcnt: libc::c_int,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE19handle_timed_acceptEiP14ACE_Time_Valueb"]
        pub fn handle_timed_accept(
            listener: libc::c_int,
            timeout: *mut super::ACE_Time_Value,
            restart: bool,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE21handle_timed_completeEiPK14ACE_Time_Valuei"]
        pub fn handle_timed_complete(
            listener: libc::c_int,
            timeout: *const super::ACE_Time_Value,
            is_tli: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE16set_handle_limitEii"]
        pub fn set_handle_limit(
            new_limit: libc::c_int,
            increase_limit_only: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE11max_handlesEv"]
        pub fn max_handles() -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9strenvdupEPKc"]
        pub fn strenvdup(str: *const libc::c_char) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6strendEPKc"]
        pub fn strend(s: *const libc::c_char) -> *const libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6strnewEPKc"]
        pub fn strnew(s: *const libc::c_char) -> *mut libc::c_char;
    }
    pub use crate::full_ops_0::ACE::strdelete;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7strndupEPKcm"]
        pub fn strndup_u823226a82bf0fd9a(
            str: *const libc::c_char,
            n: libc::c_ulong,
        ) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7strnnewEPKcm"]
        pub fn strnnew(str: *const libc::c_char, n: libc::c_ulong) -> *mut libc::c_char;
    }
    pub use crate::full_ops_0::ACE::isdotdir;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6strendEPKw"]
        pub fn strend_u1645597eee4d5ce1(s: *const libc::wchar_t) -> *const libc::wchar_t;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6strnewEPKw"]
        pub fn strnew_ufad401caff9461f0(s: *const libc::wchar_t) -> *mut libc::wchar_t;
    }
    pub use crate::full_ops_0::ACE::strdelete_u98ec8357f60b4ba4;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7strndupEPKwm"]
        pub fn strndup_ud668670afb78138e(
            str: *const libc::wchar_t,
            n: libc::c_ulong,
        ) -> *mut libc::wchar_t;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7strnnewEPKwm"]
        pub fn strnnew_u08b3cba2a6796603(
            str: *const libc::wchar_t,
            n: libc::c_ulong,
        ) -> *mut libc::wchar_t;
    }
    pub use crate::full_ops_0::ACE::isdotdir_u174d8f412e1ce4b7;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8execnameEPKc"]
        pub fn execname(pathname: *const libc::c_char) -> *const libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8basenameEPKcc"]
        pub fn basename_u206a02824429514d(
            pathname: *const libc::c_char,
            delim: libc::c_char,
        ) -> *const libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7dirnameEPKcc"]
        pub fn dirname(
            pathname: *const libc::c_char,
            delim: libc::c_char,
        ) -> *const libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9timestampERK14ACE_Time_ValuePcmb"]
        pub fn timestamp(
            time_value: *const super::ACE_Time_Value,
            date_and_time: *mut libc::c_char,
            time_len: libc::c_ulong,
            return_pointer_to_first_digit: bool,
        ) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9timestampEPcmb"]
        pub fn timestamp_uedcad69f71820d6a(
            date_and_time: *mut libc::c_char,
            time_len: libc::c_ulong,
            return_pointer_to_first_digit: bool,
        ) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE4forkEPKci"]
        pub fn fork_u4fb8921c2c83ccf9(
            program_name: *const libc::c_char,
            avoid_zombies: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9daemonizeEPKcbS1_"]
        pub fn daemonize(
            pathname: *const libc::c_char,
            close_all_handles: bool,
            program_name: *const libc::c_char,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE17round_to_pagesizeEm"]
        pub fn round_to_pagesize(len: libc::c_ulong) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE31round_to_allocation_granularityEm"]
        pub fn round_to_allocation_granularity(len: libc::c_ulong) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE14format_hexdumpEPKcmPcm"]
        pub fn format_hexdump(
            buffer: *const libc::c_char,
            size: libc::c_ulong,
            obuf: *mut libc::c_char,
            obuf_sz: libc::c_ulong,
        ) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8hash_pjwEPKc"]
        pub fn hash_pjw(str: *const libc::c_char) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8hash_pjwEPKcm"]
        pub fn hash_pjw_u1df7e2c40de66984(
            str: *const libc::c_char,
            len: libc::c_ulong,
        ) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8hash_pjwEPKw"]
        pub fn hash_pjw_ue0653afd76c2ae8d(str: *const libc::wchar_t) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8hash_pjwEPKwm"]
        pub fn hash_pjw_uc7d0e36c0401b650(
            str: *const libc::wchar_t,
            len: libc::c_ulong,
        ) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9crc_ccittEPKc"]
        pub fn crc_ccitt_u4c460c26d9ccad87(str: *const libc::c_char) -> libc::c_ushort;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9crc_ccittEPKvmt"]
        pub fn crc_ccitt_u2a057a3f51526d89(
            buf: *const libc::c_void,
            len: libc::c_ulong,
            crc: libc::c_ushort,
        ) -> libc::c_ushort;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9crc_ccittEPK5iovecit"]
        pub fn crc_ccitt(
            iov: *const super::iovec,
            len: libc::c_int,
            crc: libc::c_ushort,
        ) -> libc::c_ushort;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE5crc32EPKc"]
        pub fn crc32_u0e5e0cf2e3dddc4e(str: *const libc::c_char) -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE5crc32EPKvmj"]
        pub fn crc32_ub39069a03a8eec8e(
            buf: *const libc::c_void,
            len: libc::c_ulong,
            crc: libc::c_uint,
        ) -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE5crc32EPK5iovecij"]
        pub fn crc32(
            iov: *const super::iovec,
            len: libc::c_int,
            crc: libc::c_uint,
        ) -> libc::c_uint;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE3gcdEmm"]
        pub fn gcd(x: libc::c_ulong, y: libc::c_ulong) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE18minimum_frame_sizeEmm"]
        pub fn minimum_frame_size(
            period1: libc::c_ulong,
            period2: libc::c_ulong,
        ) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8is_primeEmmm"]
        pub fn is_prime(
            n: libc::c_ulong,
            min_factor: libc::c_ulong,
            max_factor: libc::c_ulong,
        ) -> libc::c_ulong;
    }
    pub use crate::full_ops_0::ACE::map_errno;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE10sock_errorEi"]
        pub fn sock_error(error: libc::c_int) -> *const libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE13is_sock_errorEi"]
        pub fn is_sock_error(error: libc::c_int) -> bool;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE14process_activeEi"]
        pub fn process_active(pid: libc::c_int) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE17terminate_processEi"]
        pub fn terminate_process(pid: libc::c_int) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE::unique_name;
    pub use crate::full_ops_0::ACE::log2_ua1e99811554d7513;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE10nibble2hexEj"]
        pub fn nibble2hex(n: libc::c_uint) -> libc::c_char;
    }
    pub use crate::full_ops_0::ACE::hex2byte;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE5debugEv"]
        pub fn debug() -> bool;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE5debugEb"]
        pub fn debug_u300ab704d1c44c4a(onoff: bool);
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6selectEiP14ACE_Handle_SetS1_S1_PK14ACE_Time_Value"]
        pub fn select_u68dbe54b28affe83(
            width: libc::c_int,
            readfds: *mut super::ACE_Handle_Set,
            writefds: *mut super::ACE_Handle_Set,
            exceptfds: *mut super::ACE_Handle_Set,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6selectEiR14ACE_Handle_SetPK14ACE_Time_Value"]
        pub fn select_u00462dd3223da577(
            width: libc::c_int,
            readfds: *mut super::ACE_Handle_Set,
            timeout: *const super::ACE_Time_Value,
        ) -> libc::c_int;
    }
    pub use crate::full_ops_0::ACE::handle_read_ready;
    pub use crate::full_ops_0::ACE::handle_write_ready;
    pub use crate::full_ops_0::ACE::handle_exception_ready;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE12handle_readyEiPK14ACE_Time_Valuebbb"]
        pub fn handle_ready(
            handle: libc::c_int,
            timeout: *const super::ACE_Time_Value,
            read_ready: bool,
            write_ready: bool,
            exception_ready: bool,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE20enter_recv_timedwaitEiPK14ACE_Time_ValueRi"]
        pub fn enter_recv_timedwait(
            handle: libc::c_int,
            timeout: *const super::ACE_Time_Value,
            val: *mut libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE20enter_send_timedwaitEiPK14ACE_Time_ValueRi"]
        pub fn enter_send_timedwait(
            handle: libc::c_int,
            timeout: *const super::ACE_Time_Value,
            val: *mut libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE32record_and_set_non_blocking_modeEiRi"]
        pub fn record_and_set_non_blocking_mode(
            handle: libc::c_int,
            val: *mut libc::c_int,
        );
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE25restore_non_blocking_modeEii"]
        pub fn restore_non_blocking_mode(handle: libc::c_int, val: libc::c_int);
    }
    pub use crate::full_ops_0::ACE::recv_i;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8recv_n_iEiPvmiPm"]
        pub fn recv_n_i_ube713064e95e6c1e(
            handle: libc::c_int,
            buf: *mut libc::c_void,
            len: libc::c_ulong,
            flags: libc::c_int,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8recv_n_iEiPvmiPK14ACE_Time_ValuePm"]
        pub fn recv_n_i_u611e7f8bb985d036(
            handle: libc::c_int,
            buf: *mut libc::c_void,
            len: libc::c_ulong,
            flags: libc::c_int,
            timeout: *const super::ACE_Time_Value,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8recv_n_iEiPvmPm"]
        pub fn recv_n_i_u657193012b74eb3a(
            handle: libc::c_int,
            buf: *mut libc::c_void,
            len: libc::c_ulong,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8recv_n_iEiPvmPK14ACE_Time_ValuePm"]
        pub fn recv_n_i(
            handle: libc::c_int,
            buf: *mut libc::c_void,
            len: libc::c_ulong,
            timeout: *const super::ACE_Time_Value,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9recvv_n_iEiP5ioveciPm"]
        pub fn recvv_n_i_ue6b6b92f3b58e3e4(
            handle: libc::c_int,
            iov: *mut super::iovec,
            iovcnt: libc::c_int,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9recvv_n_iEiP5ioveciPK14ACE_Time_ValuePm"]
        pub fn recvv_n_i(
            handle: libc::c_int,
            iov: *mut super::iovec,
            iovcnt: libc::c_int,
            timeout: *const super::ACE_Time_Value,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE::send_i;
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8send_n_iEiPKvmiPm"]
        pub fn send_n_i_u29b9c88dde4b736b(
            handle: libc::c_int,
            buf: *const libc::c_void,
            len: libc::c_ulong,
            flags: libc::c_int,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8send_n_iEiPKvmiPK14ACE_Time_ValuePm"]
        pub fn send_n_i_uca7d9e5057923519(
            handle: libc::c_int,
            buf: *const libc::c_void,
            len: libc::c_ulong,
            flags: libc::c_int,
            timeout: *const super::ACE_Time_Value,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8send_n_iEiPKvmPm"]
        pub fn send_n_i_u9fd33f238b140983(
            handle: libc::c_int,
            buf: *const libc::c_void,
            len: libc::c_ulong,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8send_n_iEiPKvmPK14ACE_Time_ValuePm"]
        pub fn send_n_i(
            handle: libc::c_int,
            buf: *const libc::c_void,
            len: libc::c_ulong,
            timeout: *const super::ACE_Time_Value,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9sendv_n_iEiPK5ioveciPm"]
        pub fn sendv_n_i_ua83fa69ef4917cf5(
            handle: libc::c_int,
            iov: *const super::iovec,
            iovcnt: libc::c_int,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9sendv_n_iEiPK5ioveciPK14ACE_Time_ValuePm"]
        pub fn sendv_n_i(
            handle: libc::c_int,
            iov: *const super::iovec,
            iovcnt: libc::c_int,
            timeout: *const super::ACE_Time_Value,
            bytes_transferred: *mut libc::c_ulong,
        ) -> libc::c_long;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE9bind_portEiji"]
        pub fn bind_port(
            handle: libc::c_int,
            ip_addr: libc::c_uint,
            address_family: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE14get_bcast_addrERjPKcji"]
        pub fn get_bcast_addr(
            bcast_addr: *mut libc::c_uint,
            hostname: *const libc::c_char,
            host_addr: libc::c_uint,
            handle: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE8get_fqdnERK13ACE_INET_AddrPcm"]
        pub fn get_fqdn(
            addr: *const super::ACE_INET_Addr,
            hostname: *mut libc::c_char,
            len: libc::c_ulong,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE17get_ip_interfacesERmRP13ACE_INET_Addr"]
        pub fn get_ip_interfaces(
            count: *mut libc::c_ulong,
            addr_array: *mut *mut super::ACE_INET_Addr,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE16count_interfacesEiRm"]
        pub fn count_interfaces(
            handle: libc::c_int,
            how_many: *mut libc::c_ulong,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE10get_handleEv"]
        pub fn get_handle() -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE12ipv4_enabledEv"]
        pub fn ipv4_enabled() -> bool;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE12ipv6_enabledEv"]
        pub fn ipv6_enabled() -> libc::c_int;
    }
}
extern "C-unwind" {
    pub fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, ...) -> libc::c_int;
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
    pub use crate::full_ops_0::ACE_Utils::Fast_Comparator_unsigned_long__unsigned_long_;
    pub use crate::full_ops_0::ACE_Utils::Truncator_long__unsigned_long_;
    pub use crate::CDR_Size_38::ACE_Utils::Truncator_unsigned_long__unsigned_int_;
    pub use crate::full_ops_0::ACE_Utils::truncator;
    pub use crate::full_ops_0::ACE_Utils::truncate_cast___long__ub591475ebc843689;
    pub type truncator_482 = Truncator_unsigned_long__unsigned_int_;
    pub use crate::CDR_Size_38::ACE_Utils::truncate_cast___unsigned_long__ue27a31e3ddf7f306;
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
extern "C-unwind" {
    pub fn uname(__name: *mut utsname) -> libc::c_int;
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
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Intrusive_List_ACE_Cleanup_Info_Node_ {
    pub head_: *mut ACE_Cleanup_Info_Node,
    pub tail_: *mut ACE_Cleanup_Info_Node,
}
pub mod __gnu_debug {}
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
#[doc = "* @class ACE_SOCK\n *\n * @brief An abstract class that forms the basis for more specific\n * classes, such as ACE_SOCK_Acceptor and ACE_SOCK_Stream.\n * Do not instantiate this class.\n *\n * This class provides functions that are common to all of the\n * <ACE_SOCK_*> classes. ACE_SOCK provides the ability to get\n * and set socket options, get the local and remote addresses,\n * and open and close a socket handle."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_SOCK {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_IPC_SAP>,
}
impl Drop for ACE_SOCK {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {}
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_u007594face7b2543(__this: *mut ACE_SOCK) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
extern "C-unwind" {
    pub fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
}
extern "C-unwind" {
    pub fn if_indextoname(
        __ifindex: libc::c_uint,
        __ifname: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn if_nameindex() -> *mut if_nameindex;
}
extern "C-unwind" {
    pub fn if_freenameindex(__ptr: *mut if_nameindex);
}
extern "C-unwind" {
    pub fn setpwent();
}
extern "C-unwind" {
    pub fn endpwent();
}
extern "C-unwind" {
    pub fn getpwent() -> *mut passwd;
}
extern "C-unwind" {
    pub fn fgetpwent(__stream: *mut _IO_FILE) -> *mut passwd;
}
extern "C-unwind" {
    pub fn putpwent(__p: *const passwd, __f: *mut _IO_FILE) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpwuid(__uid: libc::c_uint) -> *mut passwd;
}
extern "C-unwind" {
    pub fn getpwnam(__name: *const libc::c_char) -> *mut passwd;
}
extern "C-unwind" {
    pub fn getpwent_r(
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: libc::c_ulong,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpwuid_r(
        __uid: libc::c_uint,
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: libc::c_ulong,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpwnam_r(
        __name: *const libc::c_char,
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: libc::c_ulong,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn fgetpwent_r(
        __stream: *mut _IO_FILE,
        __resultbuf: *mut passwd,
        __buffer: *mut libc::c_char,
        __buflen: libc::c_ulong,
        __result: *mut *mut passwd,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn getpw(__uid: libc::c_uint, __buffer: *mut libc::c_char) -> libc::c_int;
}
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
extern "C-unwind" {
    pub fn ace_thread_adapter(args: *mut libc::c_void) -> *mut libc::c_void;
}
#[repr(C)]
pub struct ACE_Service_Gestalt {
    pub _opaque: [u8; 1],
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Base_Thread_Adapter_init_log_msg_hook_: Option<
        unsafe extern "C-unwind" fn(*mut ACE_OS_Log_Msg_Attributes),
    >;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Base_Thread_Adapter_inherit_log_msg_hook_: Option<
        unsafe extern "C-unwind" fn(
            *mut ACE_OS_Thread_Descriptor,
            *mut ACE_OS_Log_Msg_Attributes,
        ),
    >;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Base_Thread_Adapter_close_log_msg_hook_: Option<
        unsafe extern "C-unwind" fn(),
    >;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Base_Thread_Adapter_sync_log_msg_hook_: Option<
        unsafe extern "C-unwind" fn(*const libc::c_char),
    >;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Base_Thread_Adapter_thr_desc_log_msg_hook_: Option<
        unsafe extern "C-unwind" fn() -> *mut ACE_OS_Thread_Descriptor,
    >;
}
#[doc = "* @class ACE_Base_Thread_Adapter\n *\n * @brief Base class for all the Thread_Adapters.\n *\n * Converts a C++ function into a function that can be\n * called from a thread creation routine\n * (e.g., pthread_create() or _beginthreadex()) that expects an\n * extern \"C\" entry point.  This class also makes it possible to\n * transparently provide hooks to register a thread with an\n * ACE_Thread_Manager.\n * This class is used in ACE_OS::thr_create().  In general, the\n * thread that creates an object of this class is different from\n * the thread that calls @c invoke() on this object.  Therefore,\n * the @c invoke() method is responsible for deleting itself."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Base_Thread_Adapter {
    pub vptr: *const (),
    pub user_func_: Option<
        unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub arg_: *mut libc::c_void,
    pub entry_point_: Option<
        unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    pub thr_desc_: *mut ACE_OS_Thread_Descriptor,
    pub log_msg_attributes_: ACE_OS_Log_Msg_Attributes,
    pub ctx_: *mut ACE_Service_Gestalt,
    pub flags_: libc::c_long,
}
extern "C-unwind" {
    pub fn ftok(__pathname: *const libc::c_char, __proj_id: libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn semctl(
        __semid: libc::c_int,
        __semnum: libc::c_int,
        __cmd: libc::c_int,
        ...
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn semget(
        __key: libc::c_int,
        __nsems: libc::c_int,
        __semflg: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn semop(
        __semid: libc::c_int,
        __sops: *mut sembuf,
        __nsops: libc::c_ulong,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn semtimedop(
        __semid: libc::c_int,
        __sops: *mut sembuf,
        __nsops: libc::c_ulong,
        __timeout: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_init(
        __sem: *mut sem_t,
        __pshared: libc::c_int,
        __value: libc::c_uint,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_destroy(__sem: *mut sem_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_open(
        __name: *const libc::c_char,
        __oflag: libc::c_int,
        ...
    ) -> *mut sem_t;
}
extern "C-unwind" {
    pub fn sem_close(__sem: *mut sem_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_unlink(__name: *const libc::c_char) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_wait(__sem: *mut sem_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_timedwait(__sem: *mut sem_t, __abstime: *const timespec) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_clockwait(
        __sem: *mut sem_t,
        clock: libc::c_int,
        __abstime: *const timespec,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_trywait(__sem: *mut sem_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_post(__sem: *mut sem_t) -> libc::c_int;
}
extern "C-unwind" {
    pub fn sem_getvalue(__sem: *mut sem_t, __sval: *mut libc::c_int) -> libc::c_int;
}
extern "C-unwind" {
    pub fn ACE_MUTEX_LOCK_CLEANUP_ADAPTER_NAME(args: *mut libc::c_void);
}
#[doc = "* @class ACE_OS_Thread_Mutex_Guard\n *\n * This data structure is meant to be used within an ACE_OS\n * function.  It performs automatic aquisition and release of\n * an ACE_thread_mutex_t.\n *\n * If an object of this class is instantiated before ACE_Object_Manager is\n * initialized, it will not do anything. This is because this class is\n * used only with the ACE_OS_GUARD macro which is passing a reference to\n * one of the preallocated Object Manager locks. If the object manager\n * hasn't been initialized yet, the lock reference is bogus. This is an\n * acceptable tradeoff since in cases where the lock reference is bogus,\n * there isn't multithreaded access. Please see detailed comments in\n * Object_Manager.h for further information.\n *\n * For internal use only by ACE_OS."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_OS_Thread_Mutex_Guard {
    pub lock_: *mut pthread_mutex_t,
    pub owner_: libc::c_int,
}
pub unsafe extern "C-unwind" fn __xtu__ZN25ACE_OS_Thread_Mutex_GuardC1ER15pthread_mutex_t(
    __this: *mut ACE_OS_Thread_Mutex_Guard,
    __a0: *mut pthread_mutex_t,
) {
    ACE_OS_Thread_Mutex_Guard::new_at(__this, __a0)
}
impl Drop for ACE_OS_Thread_Mutex_Guard {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {
                <ACE_OS_Thread_Mutex_Guard>::release(
                    (__this) as *mut ACE_OS_Thread_Mutex_Guard,
                );
            }
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_u2613bb34aee8dddc(
    __this: *mut ACE_OS_Thread_Mutex_Guard,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[doc = "* @class ACE_OS_Recursive_Thread_Mutex_Guard\n *\n * @brief For internal use only by ACE_OS.\n *\n * This data structure is meant to be used within an ACE_OS\n * function.  It performs automatic aquisition and release of\n * an ACE_recursive_thread_mutex_t.\n *\n * If an object of this class is instantiated before ACE_Object_Manager is\n * initialized, it will not do anything. This is because this class is\n * used only with the ACE_TSS_GUARD macro which is passing a reference to\n * one of the preallocated Object Manager locks. If the object manager\n * hasn't been initialized yet, the lock reference is bogus. This is an\n * acceptable tradeoff since in cases where the lock reference is bogus,\n * there isn't multithreaded access. Please see detailed comments in\n * Object_Manager.h for further information."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_OS_Recursive_Thread_Mutex_Guard {
    pub lock_: *mut pthread_mutex_t,
    pub owner_: libc::c_int,
}
pub unsafe extern "C-unwind" fn __xtu__ZN35ACE_OS_Recursive_Thread_Mutex_GuardC1ER15pthread_mutex_t(
    __this: *mut ACE_OS_Recursive_Thread_Mutex_Guard,
    __a0: *mut pthread_mutex_t,
) {
    ACE_OS_Recursive_Thread_Mutex_Guard::new_at(__this, __a0)
}
impl Drop for ACE_OS_Recursive_Thread_Mutex_Guard {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {
                <ACE_OS_Recursive_Thread_Mutex_Guard>::release(
                    (__this) as *mut ACE_OS_Recursive_Thread_Mutex_Guard,
                );
            }
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_ufb9035b851c2f5e9(
    __this: *mut ACE_OS_Recursive_Thread_Mutex_Guard,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
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
#[doc = "* @class ACE_SOCK_IO\n *\n * @brief Defines the methods for the ACE socket wrapper I/O routines\n * described below.\n *\n * If @a timeout == 0, then the call behaves as a normal\n * send/recv call, i.e., for blocking sockets, the call will\n * block until action is possible; for non-blocking sockets,\n * -1 will be returned with errno == EWOULDBLOCK if no action is\n * immediately possible.\n * If @a timeout != 0, the call will wait until the relative time\n * specified in *@a timeout elapses.\n * Errors are reported by -1 and 0 return values.  If the\n * operation times out, -1 is returned with @c errno == ETIME.\n * If it succeeds the number of bytes transferred is returned.\n * Methods with the extra @a flags argument will always result in\n * @c send getting called. Methods without the extra @a flags\n * argument will result in @c send getting called on Win32\n * platforms, and @c write getting called on non-Win32 platforms."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_SOCK_IO {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_SOCK>,
}
pub unsafe extern "C-unwind" fn __xtu__ZN11ACE_SOCK_IOC1Ev(__this: *mut ACE_SOCK_IO) {
    ACE_SOCK_IO::new_at(__this)
}
impl Drop for ACE_SOCK_IO {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {}
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_udd2e6519082ac6d8(__this: *mut ACE_SOCK_IO) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[doc = "* @class ACE_INET_Addr\n *\n * @brief Defines a C++ wrapper facade for the Internet domain address\n * family format.\n *\n * ACE_INET_Addr can hold all of the IP addresses assigned to a single name.\n * By default it refers only to the first, if there is more than one. The\n * next() method can make the others available in turn."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_INET_Addr {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Addr>,
    pub inet_addr_: ip46,
    pub inet_addrs_: crate::__cxx_std::Vector<ip46>,
    pub inet_addrs_iter_: *mut ip46,
}
pub type ACE_SOCK_Stream_PEER_ADDR = ACE_INET_Addr;
#[doc = "* @class ACE_SOCK_Stream\n *\n * @brief Defines the methods in the ACE_SOCK_Stream abstraction.\n *\n * This adds additional wrapper methods atop the ACE_SOCK_IO\n * class.\n *\n * @sa ACE_SOCK_IO"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_SOCK_Stream {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_SOCK_IO>,
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_SOCK_StreamC1Ev(
    __this: *mut ACE_SOCK_Stream,
) {
    ACE_SOCK_Stream::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_SOCK_StreamC1Ei(
    __this: *mut ACE_SOCK_Stream,
    __a0: libc::c_int,
) {
    ACE_SOCK_Stream::new_at_ub6e5d8b47c21da88(__this, __a0)
}
impl Drop for ACE_SOCK_Stream {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {}
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_uf400883832fba5ac(
    __this: *mut ACE_SOCK_Stream,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
pub type ACE_SOCK_Connector_PEER_ADDR = ACE_INET_Addr;
pub type ACE_SOCK_Connector_PEER_STREAM = ACE_SOCK_Stream;
#[doc = "* @class ACE_SOCK_Connector\n *\n * @brief Defines a factory that actively connects to a remote IP\n * address and TCP port, creating a new @c ACE_SOCK_Stream object.\n *\n * The @c ACE_SOCK_Connector doesn't have a socket of its own,\n * i.e., it simply \"borrows\" the one from the @c ACE_SOCK_Stream\n * that's being connected.  The reason for this is that the\n * underlying socket API doesn't use a factory socket to connect\n * data mode sockets.  Therefore, there's no need to inherit\n * @c ACE_SOCK_Connector from @c ACE_SOCK.  A nice side-effect of\n * this is that @c ACE_SOCK_Connector objects do not store state so\n * they can be used reentrantly in multithreaded programs."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_SOCK_Connector {}
pub unsafe extern "C-unwind" fn __xtu__ZN18ACE_SOCK_ConnectorC1Ev(
    __this: *mut ACE_SOCK_Connector,
) {
    ACE_SOCK_Connector::new_at(__this)
}
impl Drop for ACE_SOCK_Connector {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {}
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_u18b2c8aa9314a617(
    __this: *mut ACE_SOCK_Connector,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
pub type ACE_LOG_MSG_IPC_STREAM = ACE_SOCK_Stream;
pub type ACE_LOG_MSG_IPC_CONNECTOR = ACE_SOCK_Connector;
pub type ACE_LOG_MSG_IPC_ADDR = ACE_INET_Addr;
#[doc = "Defines the interfaces for ACE_Log_Msg backend.\n/**\n * Implement an ACE_Log_Msg_Backend that logs to a remote logging\n * process."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Log_Msg_IPC {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Log_Msg_Backend>,
    pub message_queue_: ::core::mem::ManuallyDrop<ACE_SOCK_Stream>,
}
#[export_name = "_ZN15ACE_Log_Msg_IPCC1Ev"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_Log_Msg_IPCC1Ev(
    __this: *mut ACE_Log_Msg_IPC,
) {
    ACE_Log_Msg_IPC::new_at(__this)
}
impl Drop for ACE_Log_Msg_IPC {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {
                    {
                        let _ = {
                            let __obj: *mut ACE_Log_Msg_Backend = (__this)
                                as *mut ACE_Log_Msg_Backend;
                            let __vt: *const __Vtbl_udbddea67410264fa = *(__obj
                                as *const *const __Vtbl_udbddea67410264fa);
                            ((*__vt).vfn_ubf24d44d4595cf9b)(__obj)
                        };
                    };
                }
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.message_queue_);
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
#[export_name = "_ZN15ACE_Log_Msg_IPC4openEPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_Log_Msg_IPC4openEPKc(
    __this: *mut ACE_Log_Msg_IPC,
    logger_key: *const libc::c_char,
) -> libc::c_int {
    unsafe { ACE_Log_Msg_IPC::open(__this, logger_key) }
}
#[export_name = "_ZN15ACE_Log_Msg_IPC5resetEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_Log_Msg_IPC5resetEv(
    __this: *mut ACE_Log_Msg_IPC,
) -> libc::c_int {
    unsafe { ACE_Log_Msg_IPC::reset(__this) }
}
#[export_name = "_ZN15ACE_Log_Msg_IPC5closeEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_Log_Msg_IPC5closeEv(
    __this: *mut ACE_Log_Msg_IPC,
) -> libc::c_int {
    unsafe { ACE_Log_Msg_IPC::close(__this) }
}
#[export_name = "_ZN15ACE_Log_Msg_IPC3logER14ACE_Log_Record"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_Log_Msg_IPC3logER14ACE_Log_Record(
    __this: *mut ACE_Log_Msg_IPC,
    log_record: *mut ACE_Log_Record,
) -> libc::c_long {
    unsafe { ACE_Log_Msg_IPC::log(__this, log_record) }
}
#[repr(C)]
pub struct ACE_Log_Category_TSS {
    pub _opaque: [u8; 1],
}
/**Helper class to transfer the contents from one input CDR to
  /// another without requiring any extra memory allocations, data
  /// copies or too many temporaries.*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Transfer_Contents {
    pub rhs_: *mut ACE_InputCDR,
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_InputCDR17Transfer_ContentsC1ERS_(
    __this: *mut Transfer_Contents,
    __a0: *mut ACE_InputCDR,
) {
    Transfer_Contents::new_at(__this, __a0)
}
#[doc = "* @class ACE_InputCDR\n *\n * @brief A CDR stream for demarshalling CDR-encoded data.\n *\n * This class is based on the the CORBA spec for Java (98-02-29),\n * java class omg.org.CORBA.portable.InputStream.  It diverts in a\n * few ways:\n * @li Operations to retrieve basic types take parameters by\n *     reference.\n * @li Operations taking arrays don't have offsets, because in C++\n *     it is easier to describe an array starting from x+offset.\n * @li Operations return an error status, because exceptions are\n *     not widely available in C++ (yet)."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_InputCDR {
    pub vptr: *const (),
    pub start_: ::core::mem::ManuallyDrop<ACE_Message_Block>,
    pub do_byte_swap_: bool,
    pub good_bit_: bool,
    pub major_version_: libc::c_uchar,
    pub minor_version_: libc::c_uchar,
    pub char_translator_: *mut ACE_Char_Codeset_Translator,
    pub wchar_translator_: *mut ACE_WChar_Codeset_Translator,
}
impl Drop for ACE_InputCDR {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {}
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.start_);
        }
    }
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_OutputCDR_wchar_maxbytes_: libc::c_ulong;
}
#[doc = "* @class ACE_OutputCDR\n *\n * @brief A CDR stream for marshalling data, most often for transmission to\n * another system which may or may not have the same byte order.\n *\n * This class is based on the the CORBA spec for Java (98-02-29),\n * java class omg.org.CORBA.portable.OutputStream.  It diverts in\n * a few ways:\n * @li Operations taking arrays don't have offsets, because in C++\n *     it is easier to describe an array starting from x+offset.\n * @li Operations return an error status, because exceptions are\n *     not widely available in C++ (yet)."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_OutputCDR {
    pub start_: ::core::mem::ManuallyDrop<ACE_Message_Block>,
    pub current_: *mut ACE_Message_Block,
    pub current_alignment_: libc::c_ulong,
    pub current_is_writable_: bool,
    pub do_byte_swap_: bool,
    pub good_bit_: bool,
    pub memcpy_tradeoff_: libc::c_ulong,
    pub major_version_: libc::c_uchar,
    pub minor_version_: libc::c_uchar,
    pub char_translator_: *mut ACE_Char_Codeset_Translator,
    pub wchar_translator_: *mut ACE_WChar_Codeset_Translator,
}
impl Drop for ACE_OutputCDR {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {
                    if ((((!(<ACE_Message_Block>::cont(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                            ACE_Message_Block > ().cast_mut())
                        )) as *const ACE_Message_Block,
                    ))
                        .is_null()) as libc::c_int) as libc::c_int) != 0)
                    {
                        <ACE_Message_Block>::release_ubc51e64ee0ea988c(
                            <ACE_Message_Block>::cont(
                                (::core::ptr::addr_of!(
                                    (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                                    ACE_Message_Block > ().cast_mut())
                                )) as *const ACE_Message_Block,
                            ),
                        );
                        <ACE_Message_Block>::cont_u9515391441f35afa(
                            (::core::ptr::addr_of_mut!(
                                (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                                ACE_Message_Block > ().cast_mut())
                            )) as *mut ACE_Message_Block,
                            ((0) as *mut ACE_Message_Block),
                        );
                    }
                    (*__this).current_ = ((0) as *mut ACE_Message_Block);
                }
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.start_);
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_u4c0b68d17222a381(__this: *mut ACE_OutputCDR) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
extern "C-unwind" {
    #[link_name = "_ZrsR12ACE_InputCDRR14ACE_Log_Record"]
    pub fn operator_shr_u21390954517cc944(
        cdr: *mut ACE_InputCDR,
        log_record: *mut ACE_Log_Record,
    ) -> libc::c_int;
}
extern "C-unwind" {
    #[link_name = "_ZlsR13ACE_OutputCDRRK14ACE_Log_Record"]
    pub fn operator_shl_ub3e53c4872f84b62(
        cdr: *mut ACE_OutputCDR,
        log_record: *const ACE_Log_Record,
    ) -> libc::c_int;
}
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
#[doc = "* @class ACE_CDR\n *\n * @brief Keep constants and some routines common to both Output and\n * Input CDR streams."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_CDR {}
extern "C-unwind" {
    #[link_name = "_ZlsRSoRKN7ACE_CDR5FixedE"]
    pub fn operator_shl(
        lhs: *mut crate::__cxx_std::Ostream,
        rhs: *const Fixed,
    ) -> *mut crate::__cxx_std::Ostream;
}
extern "C-unwind" {
    #[link_name = "_ZrsRSiRN7ACE_CDR5FixedE"]
    pub fn operator_shr(
        lhs: *mut crate::__cxx_std::Istream,
        rhs: *mut Fixed,
    ) -> *mut crate::__cxx_std::Istream;
}
#[repr(C)]
pub struct ACE_String_Base_char_ {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_String_Base_wchar_t_ {
    pub _opaque: [u8; 1],
}
pub type ACE_CString = ACE_String_Base_char_;
pub type ACE_WString = ACE_String_Base_wchar_t_;
pub type ACE_TString = ACE_String_Base_char_;
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
#[doc = "* @class ACE_Char_Codeset_Translator\n *\n * @brief Codeset translation routines common to both Output and Input\n * CDR streams.\n *\n * This class is a base class for defining codeset translation\n * routines to handle the character set translations required by\n * both CDR Input streams and CDR Output streams.\n *\n * Translators are reference counted. This allows for stateful as well\n * as stateless translators. Stateless translators will be allocated\n * once whereas CDR Streams own their own copy of a stateful translator."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Char_Codeset_Translator {
    pub vptr: *const (),
}
#[doc = "* @class ACE_WChar_Codeset_Translator\n *\n * @brief Codeset translation routines common to both Output and Input\n * CDR streams.\n *\n * This class is a base class for defining codeset translation\n * routines to handle the character set translations required by\n * both CDR Input streams and CDR Output streams."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_WChar_Codeset_Translator {
    pub vptr: *const (),
}
extern "C-unwind" {
    #[link_name = "_ZlsR13ACE_OutputCDRRK15ACE_String_BaseIcE"]
    pub fn operator_shl_u41c499ef4c35bf83(
        os: *mut ACE_OutputCDR,
        x: *const ACE_String_Base_char_,
    ) -> bool;
}
extern "C-unwind" {
    #[link_name = "_ZrsR12ACE_InputCDRR15ACE_String_BaseIcE"]
    pub fn operator_shr_ue4a62b3567ee8189(
        is: *mut ACE_InputCDR,
        x: *mut ACE_String_Base_char_,
    ) -> bool;
}
pub unsafe extern "C-unwind" fn operator_shl_u718eae5145ac24db(
    mut os: *mut ACE_OutputCDR,
    mut x: libc::c_char,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_char(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u71580e51457d858b(
    mut os: *mut ACE_OutputCDR,
    mut x: libc::c_short,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_short(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u70eb8e5145218d2b(
    mut os: *mut ACE_OutputCDR,
    mut x: libc::c_ushort,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_ushort(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u716cb651458f4c0d(
    mut os: *mut ACE_OutputCDR,
    mut x: libc::c_int,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_long(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u70ffb65145327a2d(
    mut os: *mut ACE_OutputCDR,
    mut x: libc::c_uint,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_ulong(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u71627e5145869860(
    mut os: *mut ACE_OutputCDR,
    mut x: libc::c_long,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_longlong(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                ::core::ptr::addr_of!(x),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u70f57e514529c680(
    mut os: *mut ACE_OutputCDR,
    mut x: libc::c_ulong,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_ulonglong(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                ::core::ptr::addr_of!(x),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u717dae51459db1a8(
    mut os: *mut ACE_OutputCDR,
    mut x: crate::__f80::F80,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_longdouble(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                ::core::ptr::addr_of!(x),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u71177e514546ace6(
    mut os: *mut ACE_OutputCDR,
    mut x: libc::c_float,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_float(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u7110ae514540dfc8(
    mut os: *mut ACE_OutputCDR,
    mut x: libc::c_double,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_double(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                ::core::ptr::addr_of!(x),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_uc4f1f34cdb9a301a(
    mut os: *mut ACE_OutputCDR,
    mut x: *const Fixed,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_fixed(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                ::core::ptr::addr_of!((* x)),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u2ebfb4c91998fada(
    mut os: *mut ACE_OutputCDR,
    mut x: *const libc::c_char,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_string(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u2ee87cc919bba0c6(
    mut os: *mut ACE_OutputCDR,
    mut x: *const libc::wchar_t,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_wstring(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_uc408c09c0c22b9ae(
    mut os: *mut ACE_OutputCDR,
    mut x: from_std_string,
) -> bool {
    unsafe {
        {
            let mut len: libc::c_uint = ((*x.val_).size() as libc::c_uint);
            <ACE_OutputCDR>::write_string_u17e4de4e62d5fde9(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                len,
                (((*x.val_).c_str()) as *const libc::c_char),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                (((((<ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ) as libc::c_int) != 0)
                    && ((((((((!(((x.bound_) != 0)) as libc::c_int) as libc::c_int) != 0)
                        || (((((len as libc::c_uint)) <= (((x.bound_) as libc::c_uint)))
                            as libc::c_int as libc::c_int) != 0)) as libc::c_int))
                        as libc::c_int) != 0)) as libc::c_int)),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_udc7b2e2f094af1a1(
    mut os: *mut ACE_OutputCDR,
    mut x: from_std_wstring,
) -> bool {
    unsafe {
        {
            let mut len: libc::c_uint = ((*x.val_).size() as libc::c_uint);
            <ACE_OutputCDR>::write_wstring_u5bf3306947ee070a(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                len,
                (((*x.val_).c_str()) as *const libc::wchar_t),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                (((((<ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ) as libc::c_int) != 0)
                    && ((((((((!(((x.bound_) != 0)) as libc::c_int) as libc::c_int) != 0)
                        || (((((len as libc::c_uint)) <= (((x.bound_) as libc::c_uint)))
                            as libc::c_int as libc::c_int) != 0)) as libc::c_int))
                        as libc::c_int) != 0)) as libc::c_int)),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_uf7ae59c2866498d7(
    mut os: *mut ACE_OutputCDR,
    mut x: *const crate::__cxx_std::String,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_string_ud46834b86c3389c8(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                ::core::ptr::addr_of!((* x)),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_uda89877db870c643(
    mut os: *mut ACE_OutputCDR,
    mut x: *const crate::__cxx_std::WString,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_wstring_u3015186591267ebf(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                ::core::ptr::addr_of!((* x)),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_ue13a06333e680a27(
    mut os: *mut ACE_OutputCDR,
    mut x: from_boolean,
) -> bool {
    unsafe {
        {
            {
                let _ = <ACE_OutputCDR>::write_boolean(
                    (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                    x.val_,
                );
            };
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_ub8d6812b45533a1b(
    mut os: *mut ACE_OutputCDR,
    mut x: from_char,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_char(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x.val_,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_uc8846fd8ea282888(
    mut os: *mut ACE_OutputCDR,
    mut x: from_wchar,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_wchar(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x.val_,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u013d78562c5fa002(
    mut os: *mut ACE_OutputCDR,
    mut x: from_octet,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_octet(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x.val_,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u17e1239cc4d02bcc(
    mut os: *mut ACE_OutputCDR,
    mut x: from_string,
) -> bool {
    unsafe {
        {
            let mut len: libc::c_uint = ((0) as libc::c_uint);
            if ((((!(x.val_).is_null()) as libc::c_int) as libc::c_int) != 0) {
                len = (ACE_OS::strlen_u07dd12a225364fa6(
                    ((x.val_) as *const libc::c_char),
                ) as libc::c_uint);
            }
            <ACE_OutputCDR>::write_string_u17e4de4e62d5fde9(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                len,
                ((x.val_) as *const libc::c_char),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                (((((<ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ) as libc::c_int) != 0)
                    && ((((((((!(((x.bound_) != 0)) as libc::c_int) as libc::c_int) != 0)
                        || (((((len as libc::c_uint)) <= (((x.bound_) as libc::c_uint)))
                            as libc::c_int as libc::c_int) != 0)) as libc::c_int))
                        as libc::c_int) != 0)) as libc::c_int)),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u8d4b87ff032451cf(
    mut os: *mut ACE_OutputCDR,
    mut x: from_wstring,
) -> bool {
    unsafe {
        {
            let mut len: libc::c_uint = ((0) as libc::c_uint);
            if ((((!(x.val_).is_null()) as libc::c_int) as libc::c_int) != 0) {
                len = (ACE_OS::strlen_u07b44aa22513a9ba(
                    ((x.val_) as *const libc::wchar_t),
                ) as libc::c_uint);
            }
            <ACE_OutputCDR>::write_wstring_u5bf3306947ee070a(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                len,
                ((x.val_) as *const libc::wchar_t),
            );
            return crate::__cxx_std::__Truthy::__truthy(
                (((((<ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ) as libc::c_int) != 0)
                    && ((((((((!(((x.bound_) != 0)) as libc::c_int) as libc::c_int) != 0)
                        || (((((len as libc::c_uint)) <= (((x.bound_) as libc::c_uint)))
                            as libc::c_int as libc::c_int) != 0)) as libc::c_int))
                        as libc::c_int) != 0)) as libc::c_int)),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_ub005d9267d9b57cf(
    mut os: *mut ACE_OutputCDR,
    mut x: from_uint8,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_uint8(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x.val_,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shl_u173165c6f5bd7c3a(
    mut os: *mut ACE_OutputCDR,
    mut x: from_int8,
) -> bool {
    unsafe {
        {
            <ACE_OutputCDR>::write_int8(
                (::core::ptr::addr_of_mut!((* os))) as *mut ACE_OutputCDR,
                x.val_,
            );
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_OutputCDR>::good_bit(
                    (::core::ptr::addr_of!((* os))) as *const ACE_OutputCDR,
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9b60b2d8d7aa9948(
    mut is: *mut ACE_InputCDR,
    mut x: *mut libc::c_char,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_char(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9b2a92d8d77cd378(
    mut is: *mut ACE_InputCDR,
    mut x: *mut libc::c_short,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_short(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9b9712d8d7d8cbd8(
    mut is: *mut ACE_InputCDR,
    mut x: *mut libc::c_ushort,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_ushort(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9b4c92d8d799b9de(
    mut is: *mut ACE_InputCDR,
    mut x: *mut libc::c_int,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_long(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9bb912d8d7f5b23e(
    mut is: *mut ACE_InputCDR,
    mut x: *mut libc::c_uint,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_ulong(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9b5682d8d7a1f333(
    mut is: *mut ACE_InputCDR,
    mut x: *mut libc::c_long,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_longlong(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9bc382d8d7fec513(
    mut is: *mut ACE_InputCDR,
    mut x: *mut libc::c_ulong,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_ulonglong(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9b71b2d8d7b90c7b(
    mut is: *mut ACE_InputCDR,
    mut x: *mut crate::__f80::F80,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_longdouble(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9bd7aad8d80fb215(
    mut is: *mut ACE_InputCDR,
    mut x: *mut libc::c_float,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_float(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9bdeb2d8d815de5b(
    mut is: *mut ACE_InputCDR,
    mut x: *mut libc::c_double,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_double(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u05e476d4e0a1298c(
    mut is: *mut ACE_InputCDR,
    mut x: *mut Fixed,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_fixed(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u4e1f4075aee1665e(
    mut is: *mut ACE_InputCDR,
    mut x: *mut *mut libc::c_char,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_string(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u4df63875aebe53b2(
    mut is: *mut ACE_InputCDR,
    mut x: *mut *mut libc::wchar_t,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_wstring(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u9aa3c1d958194ef1(
    mut is: *mut ACE_InputCDR,
    mut x: *mut crate::__cxx_std::String,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_string_ued7770fc0812e3c8(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_uee0a0932fd8b134d(
    mut is: *mut ACE_InputCDR,
    mut x: *mut crate::__cxx_std::WString,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_wstring_u8ac5af3ed766eb27(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_uec3228264fccf9be(
    mut is: *mut ACE_InputCDR,
    mut x: to_boolean,
) -> bool {
    unsafe {
        {
            return crate::__cxx_std::__Truthy::__truthy(
                <ACE_InputCDR>::read_boolean(
                    (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                    ::core::ptr::addr_of_mut!((* x.ref_)),
                ),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u489cc477eba0a8f0(
    mut is: *mut ACE_InputCDR,
    mut x: to_char,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_char(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x.ref_)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u2b3095d233bb6069(
    mut is: *mut ACE_InputCDR,
    mut x: to_wchar,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_wchar(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x.ref_)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u8439b7b4ef9b39ab(
    mut is: *mut ACE_InputCDR,
    mut x: to_octet,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_octet(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x.ref_)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u7b720912efab3aa7(
    mut is: *mut ACE_InputCDR,
    mut x: to_string,
) -> bool {
    unsafe {
        {
            return (((((((((((<ACE_InputCDR>::read_string(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ((::core::ptr::addr_of!((* x.val_))) as *mut *mut libc::c_char),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                && ((((((((!(((x.bound_) != 0)) as libc::c_int) as libc::c_int) != 0)
                    || (((((ACE_OS::strlen_u07dd12a225364fa6(
                        (((*x.val_)) as *const libc::c_char),
                    ) as libc::c_ulong)) <= (((x.bound_) as libc::c_ulong)))
                        as libc::c_int as libc::c_int) != 0)) as libc::c_int))
                    as libc::c_int) != 0)) as libc::c_int)) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u4cb2a9f8d89589f2(
    mut is: *mut ACE_InputCDR,
    mut x: to_wstring,
) -> bool {
    unsafe {
        {
            return (((((((((((<ACE_InputCDR>::read_wstring(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ((::core::ptr::addr_of!((* x.val_))) as *mut *mut libc::wchar_t),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                && ((((((((!(((x.bound_) != 0)) as libc::c_int) as libc::c_int) != 0)
                    || (((((ACE_OS::strlen_u07b44aa22513a9ba(
                        (((*x.val_)) as *const libc::wchar_t),
                    ) as libc::c_ulong)) <= (((x.bound_) as libc::c_ulong)))
                        as libc::c_int as libc::c_int) != 0)) as libc::c_int))
                    as libc::c_int) != 0)) as libc::c_int)) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_u713e95760ebe87b9(
    mut is: *mut ACE_InputCDR,
    mut x: to_std_string,
) -> bool {
    unsafe {
        {
            return (((((((((((<ACE_InputCDR>::read_string_ued7770fc0812e3c8(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x.val_)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                && ((((((((!(((x.bound_) != 0)) as libc::c_int) as libc::c_int) != 0)
                    || (((((((*x.val_).size() as libc::c_uint) as libc::c_uint))
                        <= (((x.bound_) as libc::c_uint))) as libc::c_int as libc::c_int)
                        != 0)) as libc::c_int)) as libc::c_int) != 0)) as libc::c_int))
                as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_uff17d925cdff48d4(
    mut is: *mut ACE_InputCDR,
    mut x: to_std_wstring,
) -> bool {
    unsafe {
        {
            return (((((((((((<ACE_InputCDR>::read_wstring_u8ac5af3ed766eb27(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x.val_)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                && ((((((((!(((x.bound_) != 0)) as libc::c_int) as libc::c_int) != 0)
                    || (((((((*x.val_).size() as libc::c_uint) as libc::c_uint))
                        <= (((x.bound_) as libc::c_uint))) as libc::c_int as libc::c_int)
                        != 0)) as libc::c_int)) as libc::c_int) != 0)) as libc::c_int))
                as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_uabbe29dac215c636(
    mut is: *mut ACE_InputCDR,
    mut x: to_uint8,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_uint8(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x.ref_)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
pub unsafe extern "C-unwind" fn operator_shr_ude5a90f7d2110e05(
    mut is: *mut ACE_InputCDR,
    mut x: to_int8,
) -> bool {
    unsafe {
        {
            return ((((((<ACE_InputCDR>::read_int8(
                (::core::ptr::addr_of_mut!((* is))) as *mut ACE_InputCDR,
                ::core::ptr::addr_of_mut!((* x.ref_)),
            ) as libc::c_int) != 0)
                && ((<ACE_InputCDR>::good_bit(
                    (::core::ptr::addr_of!((* is))) as *const ACE_InputCDR,
                ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
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
impl ACE_Log_Record {
    #[doc = "* Create a Log_Record and set its priority, time stamp, and\n   * process id."]
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_RecordC1Ev"]
            fn __ext(__this: *mut ACE_Log_Record);
        }
        __ext(__this as *mut ACE_Log_Record)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    pub unsafe fn new_at_u7a41f48999333dc0(
        __this: *mut Self,
        mut __a0: libc::c_uint,
        mut __a1: libc::c_long,
        mut __a2: libc::c_long,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_RecordC1E16ACE_Log_Priorityll"]
            fn __ext(
                __this: *mut ACE_Log_Record,
                __a0: libc::c_uint,
                __a1: libc::c_long,
                __a2: libc::c_long,
            );
        }
        __ext(__this as *mut ACE_Log_Record, __a0, __a1, __a2)
    }
    pub unsafe fn new_u7a41f48999333dc0(
        mut __a0: libc::c_uint,
        mut __a1: libc::c_long,
        mut __a2: libc::c_long,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u7a41f48999333dc0(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    pub unsafe fn new_at_u5a37408ef8ac50e7(
        __this: *mut Self,
        mut __a0: libc::c_uint,
        mut __a1: *const ACE_Time_Value,
        mut __a2: libc::c_long,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_RecordC1E16ACE_Log_PriorityRK14ACE_Time_Valuel"]
            fn __ext(
                __this: *mut ACE_Log_Record,
                __a0: libc::c_uint,
                __a1: *const ACE_Time_Value,
                __a2: libc::c_long,
            );
        }
        __ext(__this as *mut ACE_Log_Record, __a0, __a1, __a2)
    }
    pub unsafe fn new_u5a37408ef8ac50e7(
        mut __a0: libc::c_uint,
        mut __a1: *const ACE_Time_Value,
        mut __a2: libc::c_long,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u5a37408ef8ac50e7(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    /**Write the contents of the logging record to the appropriate
  /// FILE if the corresponding type is enabled.*/
    pub unsafe fn print(
        __this: *mut Self,
        mut host_name: *const libc::c_char,
        mut verbose_flag: libc::c_ulong,
        mut fp: *mut _IO_FILE,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_Record5printEPKcmP8_IO_FILE"]
            fn __ext(
                __this: *mut ACE_Log_Record,
                host_name: *const libc::c_char,
                verbose_flag: libc::c_ulong,
                fp: *mut _IO_FILE,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Log_Record, host_name, verbose_flag, fp)
    }
    /**Write the contents of the logging record to the appropriate
  /// @a stream if the corresponding type is enabled.*/
    pub unsafe fn print_ufd36a7167f11a180(
        __this: *mut Self,
        mut host_name: *const libc::c_char,
        mut verbose_flag: libc::c_ulong,
        mut stream: *mut crate::__cxx_std::Ostream,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_Record5printEPKcmRSo"]
            fn __ext(
                __this: *mut ACE_Log_Record,
                host_name: *const libc::c_char,
                verbose_flag: libc::c_ulong,
                stream: *mut crate::__cxx_std::Ostream,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Log_Record, host_name, verbose_flag, stream)
    }
    pub unsafe fn format_msg(
        __this: *mut Self,
        mut host_name: *const libc::c_char,
        mut verbose_flag: libc::c_ulong,
        mut verbose_msg: *mut libc::c_char,
        mut verbose_msg_size: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_Record10format_msgEPKcmPcm"]
            fn __ext(
                __this: *mut ACE_Log_Record,
                host_name: *const libc::c_char,
                verbose_flag: libc::c_ulong,
                verbose_msg: *mut libc::c_char,
                verbose_msg_size: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_Log_Record,
            host_name,
            verbose_flag,
            verbose_msg,
            verbose_msg_size,
        )
    }
    #[doc = "* Returns a character array with the string form of the\n   * ACE_Log_Priority parameter.  This is used for the verbose\n   * printing format."]
    pub unsafe fn priority_name(mut p: libc::c_uint) -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_Record13priority_nameE16ACE_Log_Priority"]
            fn __ext(p: libc::c_uint) -> *const libc::c_char;
        }
        __ext(p)
    }
    ///IMPORTANT: @a name must be a statically allocated const ACE_TCHAR*
    pub unsafe fn priority_name_uca8ae485b15eb414(
        mut p: libc::c_uint,
        mut name: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_Record13priority_nameE16ACE_Log_PriorityPKc"]
            fn __ext(p: libc::c_uint, name: *const libc::c_char);
        }
        __ext(p, name)
    }
    ///Get the type of the Log_Record.
    pub unsafe fn r#type(__this: *const Self) -> libc::c_uint {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).type_) as libc::c_uint);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the type of the Log_Record.
    pub unsafe fn type_u77bb4f12aefc84b7(__this: *mut Self, mut t: libc::c_uint) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).type_ = t;
            }
            ()
        }
    }
    ///Get the category of the Log_Record.
    pub unsafe fn category(__this: *const Self) -> *mut ACE_Log_Category_TSS {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).category_) as *mut ACE_Log_Category_TSS);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the category of the Log_Record.
    pub unsafe fn category_u470eb2bd4fedcc23(
        __this: *mut Self,
        mut t: *mut ACE_Log_Category_TSS,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).category_ = t;
            }
            ()
        }
    }
    #[doc = "* Get the priority of the Log_Record <type_>.  This is computed\n   * as the base 2 logarithm of <type_> (which must be a power of 2,\n   * as defined by the enums in ACE_Log_Priority)."]
    pub unsafe fn priority(__this: *const Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Log_Record8priorityEv"]
            fn __ext(__this: *const ACE_Log_Record) -> libc::c_ulong;
        }
        __ext(__this as *const ACE_Log_Record)
    }
    /**Set the priority of the Log_Record <type_> (which must be a
  /// power of 2, as defined by the enums in ACE_Log_Priority).*/
    pub unsafe fn priority_ua8f07a45b6f49c7e(__this: *mut Self, mut num: libc::c_ulong) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_Record8priorityEm"]
            fn __ext(__this: *mut ACE_Log_Record, num: libc::c_ulong);
        }
        __ext(__this as *mut ACE_Log_Record, num)
    }
    /**Get the total length of the Log_Record, which includes the
  /// size of the various data member fields.*/
    pub unsafe fn length(__this: *const Self) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((*__this).length_ as libc::c_long);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Set the total length of the Log_Record, which needs to account for
  /// the size of the various data member fields.*/
    pub unsafe fn length_uc628bb8eb78e871e(__this: *mut Self, mut l: libc::c_long) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).length_ = (((l as libc::c_uint)) as libc::c_int);
            }
            ()
        }
    }
    ///Get the time stamp of the Log_Record.
    pub unsafe fn time_stamp(__this: *const Self) -> ACE_Time_Value {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Time_Value>::new_ub59bcc88eaedf2a6(
                    (((*__this).secs_) as libc::c_long),
                    ((((*__this).usecs_ as libc::c_long)) as libc::c_long),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the time stamp of the Log_Record.
    pub unsafe fn time_stamp_uad72df21fab83cfe(
        __this: *mut Self,
        mut ts: *const ACE_Time_Value,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).secs_ = <ACE_Time_Value>::sec(
                    (::core::ptr::addr_of!((* ts))) as *const ACE_Time_Value,
                );
                (*__this).usecs_ = (<ACE_Time_Value>::usec(
                    (::core::ptr::addr_of!((* ts))) as *const ACE_Time_Value,
                ) as libc::c_uint);
            }
            ()
        }
    }
    ///Get the process id of the Log_Record.
    pub unsafe fn pid(__this: *const Self) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((*__this).pid_ as libc::c_long);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the process id of the Log_Record.
    pub unsafe fn pid_ud5567264d4dd1d7d(__this: *mut Self, mut p: libc::c_long) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).pid_ = (p as libc::c_uint);
            }
            ()
        }
    }
    ///Get the message data of the Log_Record.
    pub unsafe fn msg_data(__this: *const Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).msg_data_) as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Set the message data of the record. If @a data is longer than the
  /// current msg_data_ buffer, a new msg_data_ buffer is allocated to
  /// fit. If such a reallocation faisl, this method returns -1, else 0.*/
    pub unsafe fn msg_data_u6c82741b0f816e66(
        __this: *mut Self,
        mut data: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_Record8msg_dataEPKc"]
            fn __ext(
                __this: *mut ACE_Log_Record,
                data: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Log_Record, data)
    }
    /**Get the size of the message data of the Log_Record, including
  /// a byte for the NUL.*/
    pub unsafe fn msg_data_len(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((ACE_OS::strlen_u07dd12a225364fa6(
                    (((*__this).msg_data_) as *const libc::c_char),
                )) as libc::c_ulong))
                    .wrapping_add((1) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Log_Record4dumpEv"]
            fn __ext(__this: *const ACE_Log_Record);
        }
        __ext(__this as *const ACE_Log_Record)
    }
    ///Round up to the alignment restrictions.
    pub unsafe fn round_up(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_Record8round_upEv"]
            fn __ext(__this: *mut ACE_Log_Record);
        }
        __ext(__this as *mut ACE_Log_Record)
    }
    ///disallow copying...
    pub unsafe fn new_at_u3c38dd1a7ee60a1a(
        __this: *mut Self,
        mut __a0: *const ACE_Log_Record,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_RecordC1ERKS_"]
            fn __ext(__this: *mut ACE_Log_Record, __a0: *const ACE_Log_Record);
        }
        __ext(__this as *mut ACE_Log_Record, __a0)
    }
    pub unsafe fn new_u3c38dd1a7ee60a1a(mut __a0: *const ACE_Log_Record) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u3c38dd1a7ee60a1a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: *const ACE_Log_Record,
    ) -> *mut ACE_Log_Record {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Log_RecordaSERKS_"]
            fn __ext(
                __this: *mut ACE_Log_Record,
                rhs: *const ACE_Log_Record,
            ) -> *mut ACE_Log_Record;
        }
        __ext(__this as *mut ACE_Log_Record, rhs)
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
impl ACE_SOCK {
    ///Wrapper around the @c setsockopt system call.
    pub unsafe fn set_option(
        __this: *const Self,
        mut level: libc::c_int,
        mut option: libc::c_int,
        mut optval: *mut libc::c_void,
        mut optlen: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::setsockopt_u491187844361ffa3(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    level,
                    option,
                    (((optval as *mut libc::c_char)) as *const libc::c_char),
                    optlen,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Wrapper around the @c getsockopt system call.
    pub unsafe fn get_option(
        __this: *const Self,
        mut level: libc::c_int,
        mut option: libc::c_int,
        mut optval: *mut libc::c_void,
        mut optlen: *mut libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::getsockopt_ue319435db01d3776(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    level,
                    option,
                    (optval as *mut libc::c_char),
                    optlen,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Close the socket.\n   * This method also sets the object's handle value to ACE_INVALID_HANDLE.\n   *\n   * @return The result of closing the socket; 0 if the handle value\n   *         was already ACE_INVALID_HANDLE."]
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN8ACE_SOCK5closeEv"]
            fn __ext(__this: *mut ACE_SOCK) -> libc::c_int;
        }
        __ext(__this as *mut ACE_SOCK)
    }
    /**Return the local endpoint address in the referenced ACE_Addr.
  /// Returns 0 if successful, else -1.*/
    pub unsafe fn get_local_addr(
        __this: *const Self,
        mut _anon_0: *mut ACE_Addr,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK8ACE_SOCK14get_local_addrER8ACE_Addr"]
            fn __ext(__this: *const ACE_SOCK, _anon_0: *mut ACE_Addr) -> libc::c_int;
        }
        __ext(__this as *const ACE_SOCK, _anon_0)
    }
    #[doc = "* Return the address of the remotely connected peer (if there is\n   * one), in the referenced ACE_Addr. Returns 0 if successful, else\n   * -1."]
    pub unsafe fn get_remote_addr(
        __this: *const Self,
        mut _anon_0: *mut ACE_Addr,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK8ACE_SOCK15get_remote_addrER8ACE_Addr"]
            fn __ext(__this: *const ACE_SOCK, _anon_0: *mut ACE_Addr) -> libc::c_int;
        }
        __ext(__this as *const ACE_SOCK, _anon_0)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK8ACE_SOCK4dumpEv"]
            fn __ext(__this: *const ACE_SOCK);
        }
        __ext(__this as *const ACE_SOCK)
    }
    ///Wrapper around the BSD-style @c socket system call (no QoS).
    pub unsafe fn open(
        __this: *mut Self,
        mut r#type: libc::c_int,
        mut protocol_family: libc::c_int,
        mut protocol: libc::c_int,
        mut reuse_addr: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN8ACE_SOCK4openEiiii"]
            fn __ext(
                __this: *mut ACE_SOCK,
                r#type: libc::c_int,
                protocol_family: libc::c_int,
                protocol: libc::c_int,
                reuse_addr: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_SOCK, r#type, protocol_family, protocol, reuse_addr)
    }
    ///Wrapper around the QoS-enabled @c WSASocket function.
    pub unsafe fn open_ua8dc1292ba9f7772(
        __this: *mut Self,
        mut r#type: libc::c_int,
        mut protocol_family: libc::c_int,
        mut protocol: libc::c_int,
        mut protocolinfo: *mut ACE_Protocol_Info,
        mut g: libc::c_ulong,
        mut flags: libc::c_ulong,
        mut reuse_addr: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN8ACE_SOCK4openEiiiP17ACE_Protocol_Infommi"]
            fn __ext(
                __this: *mut ACE_SOCK,
                r#type: libc::c_int,
                protocol_family: libc::c_int,
                protocol: libc::c_int,
                protocolinfo: *mut ACE_Protocol_Info,
                g: libc::c_ulong,
                flags: libc::c_ulong,
                reuse_addr: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_SOCK,
            r#type,
            protocol_family,
            protocol,
            protocolinfo,
            g,
            flags,
            reuse_addr,
        )
    }
    /**Constructor with arguments to call the BSD-style @c socket system
  /// call (no QoS).*/
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: libc::c_int,
        mut __a1: libc::c_int,
        mut __a2: libc::c_int,
        mut __a3: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN8ACE_SOCKC1Eiiii"]
            fn __ext(
                __this: *mut ACE_SOCK,
                __a0: libc::c_int,
                __a1: libc::c_int,
                __a2: libc::c_int,
                __a3: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_SOCK, __a0, __a1, __a2, __a3)
    }
    pub unsafe fn new(
        mut __a0: libc::c_int,
        mut __a1: libc::c_int,
        mut __a2: libc::c_int,
        mut __a3: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1, __a2, __a3);
        __obj
    }
    /**Constructor with arguments to call the QoS-enabled @c WSASocket
  /// function.*/
    pub unsafe fn new_at_u29b3d6504f3a1a8a(
        __this: *mut Self,
        mut __a0: libc::c_int,
        mut __a1: libc::c_int,
        mut __a2: libc::c_int,
        mut __a3: *mut ACE_Protocol_Info,
        mut __a4: libc::c_ulong,
        mut __a5: libc::c_ulong,
        mut __a6: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN8ACE_SOCKC1EiiiP17ACE_Protocol_Infommi"]
            fn __ext(
                __this: *mut ACE_SOCK,
                __a0: libc::c_int,
                __a1: libc::c_int,
                __a2: libc::c_int,
                __a3: *mut ACE_Protocol_Info,
                __a4: libc::c_ulong,
                __a5: libc::c_ulong,
                __a6: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_SOCK, __a0, __a1, __a2, __a3, __a4, __a5, __a6)
    }
    pub unsafe fn new_u29b3d6504f3a1a8a(
        mut __a0: libc::c_int,
        mut __a1: libc::c_int,
        mut __a2: libc::c_int,
        mut __a3: *mut ACE_Protocol_Info,
        mut __a4: libc::c_ulong,
        mut __a5: libc::c_ulong,
        mut __a6: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u29b3d6504f3a1a8a(
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
    /**Default constructor is protected to prevent instances of this class
  /// from being defined.*/
    pub unsafe fn new_at_u5fc888c9b9ff78ab(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN8ACE_SOCKC1Ev"]
            fn __ext(__this: *mut ACE_SOCK);
        }
        __ext(__this as *mut ACE_SOCK)
    }
    pub unsafe fn new_u5fc888c9b9ff78ab() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u5fc888c9b9ff78ab(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
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
impl ACE_Base_Thread_Adapter {
    /**Accessor for the C entry point function to the OS thread creation
  /// routine.*/
    pub unsafe fn entry_point(
        __this: *mut Self,
    ) -> Option<unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void> {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).entry_point_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Invoke the close_log_msg_hook, if it is present
    pub unsafe fn close_log_msg() {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Base_Thread_Adapter13close_log_msgEv"]
            fn __ext();
        }
        __ext()
    }
    ///Invoke the sync_log_msg_hook, if it is present
    pub unsafe fn sync_log_msg(mut prog_name: *const libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Base_Thread_Adapter12sync_log_msgEPKc"]
            fn __ext(prog_name: *const libc::c_char);
        }
        __ext(prog_name)
    }
    ///Invoke the thr_desc_log_msg_hook, if it is present
    pub unsafe fn thr_desc_log_msg() -> *mut ACE_OS_Thread_Descriptor {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Base_Thread_Adapter16thr_desc_log_msgEv"]
            fn __ext() -> *mut ACE_OS_Thread_Descriptor;
        }
        __ext()
    }
    ///Constructor.
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        mut __a1: *mut libc::c_void,
        mut __a2: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        mut __a3: *mut ACE_OS_Thread_Descriptor,
        mut __a4: libc::c_long,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Base_Thread_AdapterC1EPFPvS0_ES0_S2_P24ACE_OS_Thread_Descriptorl"]
            fn __ext(
                __this: *mut ACE_Base_Thread_Adapter,
                __a0: Option<
                    unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
                >,
                __a1: *mut libc::c_void,
                __a2: Option<
                    unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
                >,
                __a3: *mut ACE_OS_Thread_Descriptor,
                __a4: libc::c_long,
            );
        }
        __ext(__this as *mut ACE_Base_Thread_Adapter, __a0, __a1, __a2, __a3, __a4)
    }
    pub unsafe fn new(
        mut __a0: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        mut __a1: *mut libc::c_void,
        mut __a2: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        mut __a3: *mut ACE_OS_Thread_Descriptor,
        mut __a4: libc::c_long,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1, __a2, __a3, __a4);
        __obj
    }
    /**Inherit the logging features if the parent thread has an
  /// ACE_Log_Msg.*/
    pub unsafe fn inherit_log_msg(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Base_Thread_Adapter15inherit_log_msgEv"]
            fn __ext(__this: *mut ACE_Base_Thread_Adapter);
        }
        __ext(__this as *mut ACE_Base_Thread_Adapter)
    }
    ///Set the Log_Msg hooks
    pub unsafe fn set_log_msg_hooks(
        mut init_hook: Option<
            unsafe extern "C-unwind" fn(*mut ACE_OS_Log_Msg_Attributes),
        >,
        mut inherit_hook: Option<
            unsafe extern "C-unwind" fn(
                *mut ACE_OS_Thread_Descriptor,
                *mut ACE_OS_Log_Msg_Attributes,
            ),
        >,
        mut close_hook: Option<unsafe extern "C-unwind" fn()>,
        mut sync_hook: Option<unsafe extern "C-unwind" fn(*const libc::c_char)>,
        mut thr_desc_hook: Option<
            unsafe extern "C-unwind" fn() -> *mut ACE_OS_Thread_Descriptor,
        >,
    ) {
        unsafe {
            {
                ACE_Base_Thread_Adapter_init_log_msg_hook_ = init_hook;
                ACE_Base_Thread_Adapter_inherit_log_msg_hook_ = inherit_hook;
                ACE_Base_Thread_Adapter_close_log_msg_hook_ = close_hook;
                ACE_Base_Thread_Adapter_sync_log_msg_hook_ = sync_hook;
                ACE_Base_Thread_Adapter_thr_desc_log_msg_hook_ = thr_desc_hook;
            }
            ()
        }
    }
}
impl ACE_OS_Thread_Mutex_Guard {
    ///Implicitly and automatically acquire the lock.
    pub unsafe fn new_at(__this: *mut Self, mut m: *mut pthread_mutex_t) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).lock_),
                ::core::ptr::addr_of_mut!((* m)),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).owner_),
                (-((1) as libc::c_int)),
            );
            {
                if (((!(((<ACE_OS_Object_Manager>::starting_up()) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    <ACE_OS_Thread_Mutex_Guard>::acquire(
                        (__this) as *mut ACE_OS_Thread_Mutex_Guard,
                    );
                }
            }
            ()
        }
    }
    pub unsafe fn new(mut __a0: *mut pthread_mutex_t) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Explicitly acquire the lock.
    pub unsafe fn acquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __v = ACE_OS::thread_mutex_lock(
                        ::core::ptr::addr_of_mut!((* (* __this).lock_))
                            as *mut pthread_mutex_t,
                    );
                    (*__this).owner_ = __v;
                    __v
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Explicitly release the lock.
    pub unsafe fn release(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).owner_ as libc::c_int))
                    == ((((-((1) as libc::c_int))) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return 0;
                } else {
                    (*__this).owner_ = (-((1) as libc::c_int));
                    return ACE_OS::thread_mutex_unlock(
                        ::core::ptr::addr_of_mut!((* (* __this).lock_))
                            as *mut pthread_mutex_t,
                    );
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_OS_Thread_Mutex_Guard,
    ) -> *mut ACE_OS_Thread_Mutex_Guard {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_OS_Thread_Mutex_GuardaSERKS_"]
            fn __ext(
                __this: *mut ACE_OS_Thread_Mutex_Guard,
                _anon_0: *const ACE_OS_Thread_Mutex_Guard,
            ) -> *mut ACE_OS_Thread_Mutex_Guard;
        }
        __ext(__this as *mut ACE_OS_Thread_Mutex_Guard, _anon_0)
    }
    pub unsafe fn new_at_u69ba662ed8bb3140(
        __this: *mut Self,
        mut __a0: *const ACE_OS_Thread_Mutex_Guard,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_OS_Thread_Mutex_GuardC1ERKS_"]
            fn __ext(
                __this: *mut ACE_OS_Thread_Mutex_Guard,
                __a0: *const ACE_OS_Thread_Mutex_Guard,
            );
        }
        __ext(__this as *mut ACE_OS_Thread_Mutex_Guard, __a0)
    }
    pub unsafe fn new_u69ba662ed8bb3140(
        mut __a0: *const ACE_OS_Thread_Mutex_Guard,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u69ba662ed8bb3140(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_OS_Recursive_Thread_Mutex_Guard {
    ///Implicitly and automatically acquire the lock.
    pub unsafe fn new_at(__this: *mut Self, mut m: *mut pthread_mutex_t) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).lock_),
                ::core::ptr::addr_of_mut!((* m)),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).owner_),
                (-((1) as libc::c_int)),
            );
            {
                if (((!(((<ACE_OS_Object_Manager>::starting_up()) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    <ACE_OS_Recursive_Thread_Mutex_Guard>::acquire(
                        (__this) as *mut ACE_OS_Recursive_Thread_Mutex_Guard,
                    );
                }
            }
            ()
        }
    }
    pub unsafe fn new(mut __a0: *mut pthread_mutex_t) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Explicitly acquire the lock.
    pub unsafe fn acquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __v = ACE_OS::recursive_mutex_lock(
                        ::core::ptr::addr_of_mut!((* (* __this).lock_))
                            as *mut pthread_mutex_t,
                    );
                    (*__this).owner_ = __v;
                    __v
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Explicitly release the lock.
    pub unsafe fn release(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).owner_ as libc::c_int))
                    == ((((-((1) as libc::c_int))) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return 0;
                } else {
                    (*__this).owner_ = (-((1) as libc::c_int));
                    return ACE_OS::recursive_mutex_unlock(
                        ::core::ptr::addr_of_mut!((* (* __this).lock_))
                            as *mut pthread_mutex_t,
                    );
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_OS_Recursive_Thread_Mutex_Guard,
    ) -> *mut ACE_OS_Recursive_Thread_Mutex_Guard {
        extern "C-unwind" {
            #[link_name = "_ZN35ACE_OS_Recursive_Thread_Mutex_GuardaSERKS_"]
            fn __ext(
                __this: *mut ACE_OS_Recursive_Thread_Mutex_Guard,
                _anon_0: *const ACE_OS_Recursive_Thread_Mutex_Guard,
            ) -> *mut ACE_OS_Recursive_Thread_Mutex_Guard;
        }
        __ext(__this as *mut ACE_OS_Recursive_Thread_Mutex_Guard, _anon_0)
    }
    pub unsafe fn new_at_u645a3f2208c120f2(
        __this: *mut Self,
        mut __a0: *const ACE_OS_Recursive_Thread_Mutex_Guard,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN35ACE_OS_Recursive_Thread_Mutex_GuardC1ERKS_"]
            fn __ext(
                __this: *mut ACE_OS_Recursive_Thread_Mutex_Guard,
                __a0: *const ACE_OS_Recursive_Thread_Mutex_Guard,
            );
        }
        __ext(__this as *mut ACE_OS_Recursive_Thread_Mutex_Guard, __a0)
    }
    pub unsafe fn new_u645a3f2208c120f2(
        mut __a0: *const ACE_OS_Recursive_Thread_Mutex_Guard,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u645a3f2208c120f2(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_SOCK_IO {
    ///Constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_SOCK>::new_at_u5fc888c9b9ff78ab(
                ::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_SOCK>(),
            );
            {}
            ()
        }
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Recv an @a n byte buffer from the connected socket.
    pub unsafe fn recv(
        __this: *const Self,
        mut buf: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::recv_ubc58a1994981cdc8(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((* __this)
                            .__base_0) .cast:: < ACE_SOCK > ().cast_mut()).__base_0)
                            .cast:: < ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    buf,
                    len,
                    flags,
                    timeout,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Recv an @a n byte buffer from the connected socket.
    pub unsafe fn recv_uc64db67f1646fc60(
        __this: *const Self,
        mut buf: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::recv_ufcaf818c3c730c74(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((* __this)
                            .__base_0) .cast:: < ACE_SOCK > ().cast_mut()).__base_0)
                            .cast:: < ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    buf,
                    len,
                    timeout,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Recv an <iovec> of size @a n from the connected socket.
    pub unsafe fn recvv(
        __this: *const Self,
        mut iov: *mut iovec,
        mut n: libc::c_int,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::recvv(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((* __this)
                            .__base_0) .cast:: < ACE_SOCK > ().cast_mut()).__base_0)
                            .cast:: < ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    ((iov) as *mut iovec),
                    n,
                    timeout,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Allows a client to read from a socket without having to provide a\n   * buffer to read.  This method determines how much data is in the\n   * socket, allocates a buffer of this size, reads in the data, and\n   * returns the number of bytes read.  The caller is responsible for\n   * deleting the member in the <iov_base> field of @a io_vec using\n   * delete [] io_vec->iov_base."]
    pub unsafe fn recvv_u24170279959bdfe8(
        __this: *const Self,
        mut io_vec: *mut iovec,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZNK11ACE_SOCK_IO5recvvEP5iovecPK14ACE_Time_Value"]
            fn __ext(
                __this: *const ACE_SOCK_IO,
                io_vec: *mut iovec,
                timeout: *const ACE_Time_Value,
            ) -> libc::c_long;
        }
        __ext(__this as *const ACE_SOCK_IO, io_vec, timeout)
    }
    ///Recv @a n bytes via Win32 @c ReadFile using overlapped I/O.
    pub unsafe fn recv_u914536f0d48a8966(
        __this: *const Self,
        mut buf: *mut libc::c_void,
        mut n: libc::c_ulong,
        mut overlapped: *mut ACE_OVERLAPPED,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::read_uacaa68bd37a20f7b(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((* __this)
                            .__base_0) .cast:: < ACE_SOCK > ().cast_mut()).__base_0)
                            .cast:: < ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    (((buf as *mut libc::c_char)) as *mut libc::c_void),
                    n,
                    overlapped,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Send an @a n byte buffer to the connected socket.
    pub unsafe fn send(
        __this: *const Self,
        mut buf: *const libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::send_ue0793508d1a7ed9b(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((* __this)
                            .__base_0) .cast:: < ACE_SOCK > ().cast_mut()).__base_0)
                            .cast:: < ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    buf,
                    len,
                    flags,
                    timeout,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Send an @a n byte buffer to the connected socket.
    pub unsafe fn send_u872ae246b06a2943(
        __this: *const Self,
        mut buf: *const libc::c_void,
        mut len: libc::c_ulong,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::send_u8622bdbd94726127(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((* __this)
                            .__base_0) .cast:: < ACE_SOCK > ().cast_mut()).__base_0)
                            .cast:: < ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    buf,
                    len,
                    timeout,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Send an @c iovec of size @a n to the connected socket.
    pub unsafe fn sendv(
        __this: *const Self,
        mut iov: *const iovec,
        mut n: libc::c_int,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::sendv(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((* __this)
                            .__base_0) .cast:: < ACE_SOCK > ().cast_mut()).__base_0)
                            .cast:: < ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    ((iov) as *const iovec),
                    n,
                    timeout,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Send @a n bytes via Win32 <WriteFile> using overlapped I/O.
    pub unsafe fn send_uea07755ad2066453(
        __this: *const Self,
        mut buf: *const libc::c_void,
        mut n: libc::c_ulong,
        mut overlapped: *mut ACE_OVERLAPPED,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::write_uad61deb925fc5dbb(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((* __this)
                            .__base_0) .cast:: < ACE_SOCK > ().cast_mut()).__base_0)
                            .cast:: < ACE_IPC_SAP > ().cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    buf,
                    n,
                    overlapped,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK11ACE_SOCK_IO4dumpEv"]
            fn __ext(__this: *const ACE_SOCK_IO);
        }
        __ext(__this as *const ACE_SOCK_IO)
    }
}
impl ACE_INET_Addr {
    ///Default constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1Ev"]
            fn __ext(__this: *mut ACE_INET_Addr);
        }
        __ext(__this as *mut ACE_INET_Addr)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Copy constructor.
    pub unsafe fn new_at_ufdc256446431ae0e(
        __this: *mut Self,
        mut __a0: *const ACE_INET_Addr,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1ERKS_"]
            fn __ext(__this: *mut ACE_INET_Addr, __a0: *const ACE_INET_Addr);
        }
        __ext(__this as *mut ACE_INET_Addr, __a0)
    }
    pub unsafe fn new_ufdc256446431ae0e(mut __a0: *const ACE_INET_Addr) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ufdc256446431ae0e(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Creates an ACE_INET_Addr from a sockaddr_in structure.
    pub unsafe fn new_at_u7d57fae8a46ef0f3(
        __this: *mut Self,
        mut __a0: *const sockaddr_in,
        mut __a1: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1EPK11sockaddr_ini"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: *const sockaddr_in,
                __a1: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1)
    }
    pub unsafe fn new_u7d57fae8a46ef0f3(
        mut __a0: *const sockaddr_in,
        mut __a1: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u7d57fae8a46ef0f3(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Creates an ACE_INET_Addr from a @a port_number and the remote
  /// @a host_name. The port number is assumed to be in host byte order.
  /// To set a port already in network byte order, please @see set().
  /// Use address_family to select IPv6 (PF_INET6) vs. IPv4 (PF_INET).*/
    pub unsafe fn new_at_u85c760a46d886990(
        __this: *mut Self,
        mut __a0: libc::c_ushort,
        mut __a1: *const libc::c_char,
        mut __a2: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1EtPKci"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: libc::c_ushort,
                __a1: *const libc::c_char,
                __a2: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1, __a2)
    }
    pub unsafe fn new_u85c760a46d886990(
        mut __a0: libc::c_ushort,
        mut __a1: *const libc::c_char,
        mut __a2: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u85c760a46d886990(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    #[doc = "* Initializes an ACE_INET_Addr from the @a address, which can be\n   * \"ip-addr:port-number\" (e.g., \"tango.cs.wustl.edu:1234\"),\n   * \"ip-addr:port-name\" (e.g., \"tango.cs.wustl.edu:telnet\"),\n   * \"ip-number:port-number\" (e.g., \"128.252.166.57:1234\"),\n   * \"ip-number:port-name\" (e.g., \"128.252.166.57:telnet\"),\n   * \"[ipv6-number]:port-number (e.g, \"[2001:db8::57]:1234\") or\n   * \"[ipv6-number]:port-name (e.g, \"[2001:db8::57]:telnet\").\n   * If there is no ':' in the @a address it is assumed to be a port number,\n   * with the IP address being INADDR_ANY."]
    pub unsafe fn new_at_u1f600faeb3053862(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1EPKci"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: *const libc::c_char,
                __a1: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1)
    }
    pub unsafe fn new_u1f600faeb3053862(
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u1f600faeb3053862(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    #[doc = "* Creates an ACE_INET_Addr from a @a port_number and an Internet\n   * @a ip_addr.  This method assumes that @a port_number and @a ip_addr\n   * are in host byte order. If you have addressing information in\n   * network byte order, @see set()."]
    pub unsafe fn new_at_uccf807117aa945b3(
        __this: *mut Self,
        mut __a0: libc::c_ushort,
        mut __a1: libc::c_uint,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1Etj"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: libc::c_ushort,
                __a1: libc::c_uint,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1)
    }
    pub unsafe fn new_uccf807117aa945b3(
        mut __a0: libc::c_ushort,
        mut __a1: libc::c_uint,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_uccf807117aa945b3(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Uses getservbyname() to create an ACE_INET_Addr from a
  /// @a port_name, the remote @a host_name, and the @a protocol.*/
    pub unsafe fn new_at_ucdb04bf5cb5195d6(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
        mut __a2: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1EPKcS1_S1_"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: *const libc::c_char,
                __a1: *const libc::c_char,
                __a2: *const libc::c_char,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1, __a2)
    }
    pub unsafe fn new_ucdb04bf5cb5195d6(
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
        mut __a2: *const libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ucdb04bf5cb5195d6(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    #[doc = "* Uses getservbyname() to create an ACE_INET_Addr from a\n   * @a port_name, an Internet @a ip_addr, and the @a protocol.  This\n   * method assumes that @a ip_addr is in host byte order."]
    pub unsafe fn new_at_ub3fdd154b4ca85f3(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_uint,
        mut __a2: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1EPKcjS1_"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: *const libc::c_char,
                __a1: libc::c_uint,
                __a2: *const libc::c_char,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1, __a2)
    }
    pub unsafe fn new_ub3fdd154b4ca85f3(
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_uint,
        mut __a2: *const libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ub3fdd154b4ca85f3(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    pub unsafe fn new_at_udd5b903f2a757c84(
        __this: *mut Self,
        mut __a0: libc::c_ushort,
        mut __a1: *const libc::wchar_t,
        mut __a2: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1EtPKwi"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: libc::c_ushort,
                __a1: *const libc::wchar_t,
                __a2: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1, __a2)
    }
    pub unsafe fn new_udd5b903f2a757c84(
        mut __a0: libc::c_ushort,
        mut __a1: *const libc::wchar_t,
        mut __a2: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_udd5b903f2a757c84(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    pub unsafe fn new_at_uca351f4be2ae3ee6(
        __this: *mut Self,
        mut __a0: *const libc::wchar_t,
        mut __a1: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1EPKwi"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: *const libc::wchar_t,
                __a1: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1)
    }
    pub unsafe fn new_uca351f4be2ae3ee6(
        mut __a0: *const libc::wchar_t,
        mut __a1: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_uca351f4be2ae3ee6(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    pub unsafe fn new_at_uf0a9b5a7ab65abaa(
        __this: *mut Self,
        mut __a0: *const libc::wchar_t,
        mut __a1: *const libc::wchar_t,
        mut __a2: *const libc::wchar_t,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1EPKwS1_S1_"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: *const libc::wchar_t,
                __a1: *const libc::wchar_t,
                __a2: *const libc::wchar_t,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1, __a2)
    }
    pub unsafe fn new_uf0a9b5a7ab65abaa(
        mut __a0: *const libc::wchar_t,
        mut __a1: *const libc::wchar_t,
        mut __a2: *const libc::wchar_t,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_uf0a9b5a7ab65abaa(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    pub unsafe fn new_at_u881c7b3be891e7af(
        __this: *mut Self,
        mut __a0: *const libc::wchar_t,
        mut __a1: libc::c_uint,
        mut __a2: *const libc::wchar_t,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddrC1EPKwjS1_"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                __a0: *const libc::wchar_t,
                __a1: libc::c_uint,
                __a2: *const libc::wchar_t,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, __a0, __a1, __a2)
    }
    pub unsafe fn new_u881c7b3be891e7af(
        mut __a0: *const libc::wchar_t,
        mut __a1: libc::c_uint,
        mut __a2: *const libc::wchar_t,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u881c7b3be891e7af(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    /**Assignment. In a more well-ordered world, member-wise assignment would
  /// work fine. However, because of the class design feature that all of the
  /// acceptor/connector-type classes that can be used in the
  /// Acceptor-Connector framework take ACE_Addr objects instead of the
  /// addressing class matching the family in use. The mechanism used to
  /// enable this substitution to the more-appropriate class is
  /// ACE_sap_any_cast, which casts the ACE_Addr to the more-specific class.
  /// In this case, casting an ACE_Addr to ACE_INET_Addr then copying it.
  /// Since adding multiple address support to ACE_INET_Addr, that cast-copy
  /// operation ends up, in the member-wise case, copying a bogus vector
  /// and doing lots of random damage. Thus, this operator is used to make
  /// life ordered in this common scenario.*/
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: *const ACE_INET_Addr,
    ) -> *mut ACE_INET_Addr {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_AddraSERKS_"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                rhs: *const ACE_INET_Addr,
            ) -> *mut ACE_INET_Addr;
        }
        __ext(__this as *mut ACE_INET_Addr, rhs)
    }
    ///Initializes from another ACE_INET_Addr.
    pub unsafe fn set(
        __this: *mut Self,
        mut _anon_0: *const ACE_INET_Addr,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr3setERKS_"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                _anon_0: *const ACE_INET_Addr,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_INET_Addr, _anon_0)
    }
    #[doc = "* Initializes an ACE_INET_Addr from a @a port_number and the\n   * remote @a host_name.  If @a encode is non-zero then @a port_number is\n   * converted into network byte order, otherwise it is assumed to be\n   * in network byte order already and are passed straight through.\n   * address_family can be used to select IPv4/IPv6 if the OS has\n   * IPv6 capability (ACE_HAS_IPV6 is defined). To specify IPv6, use\n   * the value AF_INET6. To specify IPv4, use AF_INET."]
    pub unsafe fn set_u7beffee0f8836764(
        __this: *mut Self,
        mut port_number: libc::c_ushort,
        mut host_name: *const libc::c_char,
        mut encode: libc::c_int,
        mut address_family: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr3setEtPKcii"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                port_number: libc::c_ushort,
                host_name: *const libc::c_char,
                encode: libc::c_int,
                address_family: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_INET_Addr,
            port_number,
            host_name,
            encode,
            address_family,
        )
    }
    #[doc = "* Initializes an ACE_INET_Addr from a @a port_number and an Internet\n   * @a ip_addr.  If @a encode is non-zero then the port number and IP address\n   * are converted into network byte order, otherwise they are assumed to be\n   * in network byte order already and are passed straight through.\n   *\n   * If @a map is non-zero and IPv6 support has been compiled in,\n   * then this address will be set to the IPv4-mapped IPv6 address of it."]
    pub unsafe fn set_u25969953d4ee6f07(
        __this: *mut Self,
        mut port_number: libc::c_ushort,
        mut ip_addr: libc::c_uint,
        mut encode: libc::c_int,
        mut map: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr3setEtjii"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                port_number: libc::c_ushort,
                ip_addr: libc::c_uint,
                encode: libc::c_int,
                map: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_INET_Addr, port_number, ip_addr, encode, map)
    }
    /**Uses getservbyname() to initialize an ACE_INET_Addr from a
  /// @a port_name, the remote @a host_name, and the @a protocol.*/
    pub unsafe fn set_u95cc26a19650f362(
        __this: *mut Self,
        mut port_name: *const libc::c_char,
        mut host_name: *const libc::c_char,
        mut protocol: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr3setEPKcS1_S1_"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                port_name: *const libc::c_char,
                host_name: *const libc::c_char,
                protocol: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_INET_Addr, port_name, host_name, protocol)
    }
    #[doc = "* Uses getservbyname() to initialize an ACE_INET_Addr from a\n   * @a port_name, an @a ip_addr, and the @a protocol.  This assumes that\n   * @a ip_addr is already in network byte order."]
    pub unsafe fn set_u96435504a79ea087(
        __this: *mut Self,
        mut port_name: *const libc::c_char,
        mut ip_addr: libc::c_uint,
        mut protocol: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr3setEPKcjS1_"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                port_name: *const libc::c_char,
                ip_addr: libc::c_uint,
                protocol: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_INET_Addr, port_name, ip_addr, protocol)
    }
    #[doc = "* Initializes an ACE_INET_Addr from the @a address, which can be\n   * \"ip-addr:port-number\" (e.g., \"tango.cs.wustl.edu:1234\"),\n   * \"ip-addr:port-name\" (e.g., \"tango.cs.wustl.edu:telnet\"),\n   * \"ip-number:port-number\" (e.g., \"128.252.166.57:1234\"),\n   * \"ip-number:port-name\" (e.g., \"128.252.166.57:telnet\"),\n   * \"[ipv6-number]:port-number (e.g, \"[2001:db8::57]:1234\") or\n   * \"[ipv6-number]:port-name (e.g, \"[2001:db8::57]:telnet\").\n   * If there is no ':' in the @a address it is assumed to be a port number,\n   * with the IP address being INADDR_ANY."]
    pub unsafe fn set_u1c01826cbf0f2d4e(
        __this: *mut Self,
        mut addr: *const libc::c_char,
        mut address_family: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr3setEPKci"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                addr: *const libc::c_char,
                address_family: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_INET_Addr, addr, address_family)
    }
    ///Creates an ACE_INET_Addr from a sockaddr_in structure.
    pub unsafe fn set_u463e6dadeed4b31f(
        __this: *mut Self,
        mut _anon_0: *const sockaddr_in,
        mut len: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr3setEPK11sockaddr_ini"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                _anon_0: *const sockaddr_in,
                len: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_INET_Addr, _anon_0, len)
    }
    pub unsafe fn set_u13276f10427e88b0(
        __this: *mut Self,
        mut port_number: libc::c_ushort,
        mut host_name: *const libc::wchar_t,
        mut encode: libc::c_int,
        mut address_family: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_INET_Addr>::set_u7beffee0f8836764(
                    (__this) as *mut ACE_INET_Addr,
                    port_number,
                    ((<ACE_Wide_To_Ascii>::char_rep(
                        (((&mut ({
                            let mut __recv: ACE_Wide_To_Ascii = (<ACE_Wide_To_Ascii>::new(
                                ((host_name) as *const libc::wchar_t),
                            ));
                            __recv
                        })) as *mut ACE_Wide_To_Ascii)) as *mut ACE_Wide_To_Ascii,
                    )) as *const libc::c_char),
                    encode,
                    address_family,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn set_ua2365d5667a6f0ae(
        __this: *mut Self,
        mut port_name: *const libc::wchar_t,
        mut host_name: *const libc::wchar_t,
        mut protocol: *const libc::wchar_t,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_INET_Addr>::set_u95cc26a19650f362(
                    (__this) as *mut ACE_INET_Addr,
                    ((<ACE_Wide_To_Ascii>::char_rep(
                        (((&mut ({
                            let mut __recv: ACE_Wide_To_Ascii = (<ACE_Wide_To_Ascii>::new(
                                ((port_name) as *const libc::wchar_t),
                            ));
                            __recv
                        })) as *mut ACE_Wide_To_Ascii)) as *mut ACE_Wide_To_Ascii,
                    )) as *const libc::c_char),
                    ((<ACE_Wide_To_Ascii>::char_rep(
                        (((&mut ({
                            let mut __recv: ACE_Wide_To_Ascii = (<ACE_Wide_To_Ascii>::new(
                                ((host_name) as *const libc::wchar_t),
                            ));
                            __recv
                        })) as *mut ACE_Wide_To_Ascii)) as *mut ACE_Wide_To_Ascii,
                    )) as *const libc::c_char),
                    ((<ACE_Wide_To_Ascii>::char_rep(
                        (((&mut ({
                            let mut __recv: ACE_Wide_To_Ascii = (<ACE_Wide_To_Ascii>::new(
                                ((protocol) as *const libc::wchar_t),
                            ));
                            __recv
                        })) as *mut ACE_Wide_To_Ascii)) as *mut ACE_Wide_To_Ascii,
                    )) as *const libc::c_char),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn set_ueb968ca7a589a5ab(
        __this: *mut Self,
        mut port_name: *const libc::wchar_t,
        mut ip_addr: libc::c_uint,
        mut protocol: *const libc::wchar_t,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_INET_Addr>::set_u96435504a79ea087(
                    (__this) as *mut ACE_INET_Addr,
                    ((<ACE_Wide_To_Ascii>::char_rep(
                        (((&mut ({
                            let mut __recv: ACE_Wide_To_Ascii = (<ACE_Wide_To_Ascii>::new(
                                ((port_name) as *const libc::wchar_t),
                            ));
                            __recv
                        })) as *mut ACE_Wide_To_Ascii)) as *mut ACE_Wide_To_Ascii,
                    )) as *const libc::c_char),
                    ip_addr,
                    ((<ACE_Wide_To_Ascii>::char_rep(
                        (((&mut ({
                            let mut __recv: ACE_Wide_To_Ascii = (<ACE_Wide_To_Ascii>::new(
                                ((protocol) as *const libc::wchar_t),
                            ));
                            __recv
                        })) as *mut ACE_Wide_To_Ascii)) as *mut ACE_Wide_To_Ascii,
                    )) as *const libc::c_char),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn set_u9aef31c60198a34a(
        __this: *mut Self,
        mut addr: *const libc::wchar_t,
        mut address_family: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_INET_Addr>::set_u1c01826cbf0f2d4e(
                    (__this) as *mut ACE_INET_Addr,
                    ((<ACE_Wide_To_Ascii>::char_rep(
                        (((&mut ({
                            let mut __recv: ACE_Wide_To_Ascii = (<ACE_Wide_To_Ascii>::new(
                                ((addr) as *const libc::wchar_t),
                            ));
                            __recv
                        })) as *mut ACE_Wide_To_Ascii)) as *mut ACE_Wide_To_Ascii,
                    )) as *const libc::c_char),
                    address_family,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return a pointer to the underlying network address.
    pub unsafe fn get_addr(__this: *const Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr8get_addrEv"]
            fn __ext(__this: *const ACE_INET_Addr) -> *mut libc::c_void;
        }
        __ext(__this as *const ACE_INET_Addr)
    }
    pub unsafe fn get_addr_size(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((::core::mem::size_of::<sockaddr_in>() as libc::c_ulong)
                    as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set a pointer to the address.
    pub unsafe fn set_addr(
        __this: *mut Self,
        mut _anon_0: *const libc::c_void,
        mut len: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr8set_addrEPKvi"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                _anon_0: *const libc::c_void,
                len: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, _anon_0, len)
    }
    ///Set a pointer to the address.
    pub unsafe fn set_addr_ud6313bc69c6faed5(
        __this: *mut Self,
        mut _anon_0: *const libc::c_void,
        mut len: libc::c_int,
        mut map: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr8set_addrEPKvii"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                _anon_0: *const libc::c_void,
                len: libc::c_int,
                map: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, _anon_0, len, map)
    }
    #[doc = "* Transform the current ACE_INET_Addr address into string format.\n   * If @a ipaddr_format is true this produces \"ip-number:port-number\"\n   * (e.g., \"128.252.166.57:1234\" or \"[2001:db8::57]:1234\"), whereas\n   * if @a ipaddr_format is false this produces \"ip-name:port-number\" (e.g.,\n   * \"tango.cs.wustl.edu:1234\").  Returns -1 if the @a size of the\n   * @a buffer is too small, else 0."]
    pub unsafe fn addr_to_string(
        __this: *const Self,
        mut buffer: *mut libc::c_char,
        mut size: libc::c_ulong,
        mut ipaddr_format: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr14addr_to_stringEPcmi"]
            fn __ext(
                __this: *const ACE_INET_Addr,
                buffer: *mut libc::c_char,
                size: libc::c_ulong,
                ipaddr_format: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *const ACE_INET_Addr, buffer, size, ipaddr_format)
    }
    #[doc = "* Initializes an ACE_INET_Addr from the @a address, which can be\n   * \"ip-addr:port-number\" (e.g., \"tango.cs.wustl.edu:1234\"),\n   * \"ip-addr:port-name\" (e.g., \"tango.cs.wustl.edu:telnet\"),\n   * \"ip-number:port-number\" (e.g., \"128.252.166.57:1234\"),\n   * \"ip-number:port-name\" (e.g., \"128.252.166.57:telnet\"),\n   * \"[ipv6-number]:port-number (e.g, \"[2001:db8::57]:1234\") or\n   * \"[ipv6-number]:port-name (e.g, \"[2001:db8::57]:telnet\").\n   * If there is no ':' in the @a address it is assumed to be a port number,\n   * with the IP address being INADDR_ANY."]
    pub unsafe fn string_to_addr(
        __this: *mut Self,
        mut address: *const libc::c_char,
        mut address_family: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr14string_to_addrEPKci"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                address: *const libc::c_char,
                address_family: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_INET_Addr, address, address_family)
    }
    #[doc = "* Sets the port number without affecting the host name.  If\n   * @a encode is enabled then @a port_number is converted into network\n   * byte order, otherwise it is assumed to be in network byte order\n   * already and are passed straight through."]
    pub unsafe fn set_port_number(
        __this: *mut Self,
        mut _anon_0: libc::c_ushort,
        mut encode: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr15set_port_numberEti"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                _anon_0: libc::c_ushort,
                encode: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_INET_Addr, _anon_0, encode)
    }
    #[doc = "* Sets the address without affecting the port number.  If\n   * @a encode is enabled then @a ip_addr is converted into network\n   * byte order, otherwise it is assumed to be in network byte order\n   * already and are passed straight through.  The size of the address\n   * is specified in the @a len parameter.\n   * If @a map is non-zero, IPv6 support has been compiled in, and\n   * @a ip_addr is an IPv4 address, then this address is set to the IPv4-mapped\n   * IPv6 address of it."]
    pub unsafe fn set_address(
        __this: *mut Self,
        mut ip_addr: *const libc::c_char,
        mut len: libc::c_int,
        mut encode: libc::c_int,
        mut map: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr11set_addressEPKciii"]
            fn __ext(
                __this: *mut ACE_INET_Addr,
                ip_addr: *const libc::c_char,
                len: libc::c_int,
                encode: libc::c_int,
                map: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_INET_Addr, ip_addr, len, encode, map)
    }
    ///Return the port number, converting it into host byte-order.
    pub unsafe fn get_port_number(__this: *const Self) -> libc::c_ushort {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((__bswap_16(
                    (((*__this).inet_addr_.in4_.sin_port) as libc::c_ushort),
                )) as libc::c_ushort);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Return the character representation of the name of the host,\n   * storing it in the @a hostname (which is assumed to be\n   * @a hostnamelen bytes long).  This version is reentrant.  If\n   * @a hostnamelen is greater than 0 then @a hostname will be\n   * NUL-terminated even if -1 is returned."]
    pub unsafe fn get_host_name(
        __this: *const Self,
        mut hostname: *mut libc::c_char,
        mut hostnamelen: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr13get_host_nameEPcm"]
            fn __ext(
                __this: *const ACE_INET_Addr,
                hostname: *mut libc::c_char,
                hostnamelen: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *const ACE_INET_Addr, hostname, hostnamelen)
    }
    pub unsafe fn get_host_name_u44c430077e6123ac(
        __this: *const Self,
        mut hostname: *mut libc::wchar_t,
        mut hostnamelen: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr13get_host_nameEPwm"]
            fn __ext(
                __this: *const ACE_INET_Addr,
                hostname: *mut libc::wchar_t,
                hostnamelen: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *const ACE_INET_Addr, hostname, hostnamelen)
    }
    #[doc = "* Return the character representation of the hostname.  This\n   * version is non-reentrant since it returns a pointer to a static\n   * data area.  You should therefore either (1) do a \"deep copy\" of\n   * the address returned by get_host_name(), e.g., using strdup() or\n   * (2) use the \"reentrant\" version of get_host_name() described\n   * above."]
    pub unsafe fn get_host_name_u9e4db10baaeaecab(
        __this: *const Self,
    ) -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr13get_host_nameEv"]
            fn __ext(__this: *const ACE_INET_Addr) -> *const libc::c_char;
        }
        __ext(__this as *const ACE_INET_Addr)
    }
    #[doc = "* Return the \"dotted decimal\" Internet address representation of\n   * the hostname storing it in the @a addr (which is assumed to be\n   * @a addr_size bytes long).  This version is reentrant."]
    pub unsafe fn get_host_addr(
        __this: *const Self,
        mut addr: *mut libc::c_char,
        mut addr_size: libc::c_int,
    ) -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr13get_host_addrEPci"]
            fn __ext(
                __this: *const ACE_INET_Addr,
                addr: *mut libc::c_char,
                addr_size: libc::c_int,
            ) -> *const libc::c_char;
        }
        __ext(__this as *const ACE_INET_Addr, addr, addr_size)
    }
    #[doc = "* Return the \"dotted decimal\" Internet address representation of\n   * the hostname.  This version is non-reentrant since it returns a\n   * pointer to a static data area.  You should therefore either\n   * (1) do a \"deep copy\" of the address returned by get_host_addr(), e.g.,\n   * using strdup() or (2) use the \"reentrant\" version of\n   * get_host_addr() described above."]
    pub unsafe fn get_host_addr_u74ff37b8fb08070b(
        __this: *const Self,
    ) -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr13get_host_addrEv"]
            fn __ext(__this: *const ACE_INET_Addr) -> *const libc::c_char;
        }
        __ext(__this as *const ACE_INET_Addr)
    }
    /**Return the 4-byte IP address, converting it into host byte
  /// order.*/
    pub unsafe fn get_ip_address(__this: *const Self) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr14get_ip_addressEv"]
            fn __ext(__this: *const ACE_INET_Addr) -> libc::c_uint;
        }
        __ext(__this as *const ACE_INET_Addr)
    }
    ///Return @c true if the IP address is INADDR_ANY or IN6ADDR_ANY.
    pub unsafe fn is_any(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((((((*__this).inet_addr_.in4_.sin_addr.s_addr as libc::c_uint))
                    == (((((0 as libc::c_uint))) as libc::c_uint))) as libc::c_int)
                    as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return @c true if the IP address is IPv4/IPv6 loopback address.
    pub unsafe fn is_loopback(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((((((((((<ACE_INET_Addr>::get_ip_address(
                    (__this) as *const ACE_INET_Addr,
                )) as libc::c_uint)) & ((0xff000000u32) as libc::c_uint))
                    as libc::c_uint))
                    == (((((((((2130706433 as libc::c_uint))) as libc::c_uint))
                        & ((0xff000000u32) as libc::c_uint))) as libc::c_uint)))
                    as libc::c_int) as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return @c true if the IP address is IPv4/IPv6 multicast address.
    pub unsafe fn is_multicast(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((((((((((*(((::core::ptr::addr_of!(
                    (* __this).inet_addr_.in4_.sin_addr.s_addr
                ) as *const libc::c_uint as *const libc::c_void)
                    as *const libc::c_uchar)))) as libc::c_int))
                    & ((0xf0) as libc::c_int)) as libc::c_int))
                    == (((0xe0) as libc::c_int))) as libc::c_int as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Returns @c true if @c this is less than @a rhs.  In this context,\n   * \"less than\" is defined in terms of IP address and TCP port\n   * number.  This operator makes it possible to use @c ACE_INET_Addrs\n   * in STL maps."]
    pub unsafe fn operator_lt(
        __this: *const Self,
        mut rhs: *const ACE_INET_Addr,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((((((((<ACE_INET_Addr>::get_ip_address(
                    (__this) as *const ACE_INET_Addr,
                ) as libc::c_uint))
                    < (((<ACE_INET_Addr>::get_ip_address(
                        (::core::ptr::addr_of!((* rhs))) as *const ACE_INET_Addr,
                    )) as libc::c_uint))) as libc::c_int as libc::c_int) != 0)
                    || ((((((((((<ACE_INET_Addr>::get_ip_address(
                        (__this) as *const ACE_INET_Addr,
                    ) as libc::c_uint))
                        == (((<ACE_INET_Addr>::get_ip_address(
                            (::core::ptr::addr_of!((* rhs))) as *const ACE_INET_Addr,
                        )) as libc::c_uint))) as libc::c_int as libc::c_int) != 0)
                        && (((((<ACE_INET_Addr>::get_port_number(
                            (__this) as *const ACE_INET_Addr,
                        ) as libc::c_int as libc::c_ushort))
                            < (((<ACE_INET_Addr>::get_port_number(
                                (::core::ptr::addr_of!((* rhs))) as *const ACE_INET_Addr,
                            )) as libc::c_int as libc::c_ushort))) as libc::c_int
                            as libc::c_int) != 0)) as libc::c_int)) as libc::c_int)
                        != 0)) as libc::c_int) as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Compare two addresses for equality.  The addresses are considered
  /// equal if they contain the same IP address and port number.*/
    pub unsafe fn operator_eq(
        __this: *const Self,
        mut SAP: *const ACE_INET_Addr,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_AddreqERKS_"]
            fn __ext(__this: *const ACE_INET_Addr, SAP: *const ACE_INET_Addr) -> bool;
        }
        __ext(__this as *const ACE_INET_Addr, SAP)
    }
    ///Compare two addresses for inequality.
    pub unsafe fn operator_ne(
        __this: *const Self,
        mut SAP: *const ACE_INET_Addr,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_AddrneERKS_"]
            fn __ext(__this: *const ACE_INET_Addr, SAP: *const ACE_INET_Addr) -> bool;
        }
        __ext(__this as *const ACE_INET_Addr, SAP)
    }
    /**A variation of the equality operator, this method only compares the
  /// IP address and ignores the port number.*/
    pub unsafe fn is_ip_equal(
        __this: *const Self,
        mut SAP: *const ACE_INET_Addr,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr11is_ip_equalERKS_"]
            fn __ext(__this: *const ACE_INET_Addr, SAP: *const ACE_INET_Addr) -> bool;
        }
        __ext(__this as *const ACE_INET_Addr, SAP)
    }
    ///Computes and returns hash value.
    pub unsafe fn hash(__this: *const Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr4hashEv"]
            fn __ext(__this: *const ACE_INET_Addr) -> libc::c_ulong;
        }
        __ext(__this as *const ACE_INET_Addr)
    }
    /**If there is another address to examine, move to it and return true;
  /// else return false.*/
    pub unsafe fn next(__this: *mut Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr4nextEv"]
            fn __ext(__this: *mut ACE_INET_Addr) -> bool;
        }
        __ext(__this as *mut ACE_INET_Addr)
    }
    ///Reset the set of address so they can be scanned again using next().
    pub unsafe fn reset(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_INET_Addr5resetEv"]
            fn __ext(__this: *mut ACE_INET_Addr);
        }
        __ext(__this as *mut ACE_INET_Addr)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr4dumpEv"]
            fn __ext(__this: *const ACE_INET_Addr);
        }
        __ext(__this as *const ACE_INET_Addr)
    }
    ///Insure that @a hostname is properly null-terminated.
    pub unsafe fn get_host_name_i(
        __this: *const Self,
        mut hostname: *mut libc::c_char,
        mut hostnamelen: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_INET_Addr15get_host_name_iEPcm"]
            fn __ext(
                __this: *const ACE_INET_Addr,
                hostname: *mut libc::c_char,
                hostnamelen: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *const ACE_INET_Addr, hostname, hostnamelen)
    }
    pub unsafe fn ip_addr_pointer(__this: *const Self) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((::core::ptr::addr_of!((* __this).inet_addr_.in4_.sin_addr)
                    as *const in_addr as *mut in_addr)) as *mut libc::c_void);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn ip_addr_size(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (::core::mem::size_of::<libc::c_uint>() as libc::c_ulong
                    as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn determine_type(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return 2;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Initialize underlying inet_addr_ to default values
    pub unsafe fn reset_i(__this: *mut Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                ACE_OS::memset_u2b5dfc47d301370a(
                    ((::core::ptr::addr_of_mut!((* __this).inet_addr_) as *mut ip46)
                        as *mut libc::c_void),
                    0,
                    ((::core::mem::size_of::<ip46>() as libc::c_ulong) as libc::c_ulong),
                );
                if (((((<ACE_Addr>::get_type(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).__base_0) .cast:: < ACE_Addr
                        > ().cast_mut())
                    )) as *const ACE_Addr,
                ) as libc::c_int)) == (((2) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    (*__this).inet_addr_.in4_.sin_family = ((2) as libc::c_ushort);
                }
                (*__this).inet_addrs_.clear();
                ((*__this).inet_addrs_iter_) = (*__this).inet_addrs_.end_ptr();
            }
            ()
        }
    }
}
impl ACE_SOCK_Stream {
    ///Constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_SOCK_IO>::new_at(
                ::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_SOCK_IO>(),
            );
            {}
            ()
        }
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Constructor (sets the underlying ACE_HANDLE with @a h).
    pub unsafe fn new_at_ub6e5d8b47c21da88(__this: *mut Self, mut h: libc::c_int) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_SOCK_IO>::new_at(
                ::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_SOCK_IO>(),
            );
            {
                <ACE_IPC_SAP>::set_handle(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                        ::core::ptr::addr_of!((* __this).__base_0) .cast:: < ACE_SOCK_IO
                        > ().cast_mut()).__base_0) .cast:: < ACE_SOCK > ().cast_mut())
                        .__base_0) .cast:: < ACE_IPC_SAP > ().cast_mut())
                    )) as *mut ACE_IPC_SAP,
                    h,
                );
            }
            ()
        }
    }
    pub unsafe fn new_ub6e5d8b47c21da88(mut __a0: libc::c_int) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ub6e5d8b47c21da88(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Try to recv exactly @a len bytes into @a buf from the connected socket.
    pub unsafe fn recv_n(
        __this: *const Self,
        mut buf: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
        mut timeout: *const ACE_Time_Value,
        mut bytes_transferred: *mut libc::c_ulong,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::recv_n_u94a488480e849282(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                            ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                            ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                            .cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    buf,
                    len,
                    flags,
                    timeout,
                    bytes_transferred,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Try to recv exactly @a len bytes into @a buf from the connected socket.
    pub unsafe fn recv_n_u4d3e8de03f9a0b30(
        __this: *const Self,
        mut buf: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut timeout: *const ACE_Time_Value,
        mut bytes_transferred: *mut libc::c_ulong,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::recv_n_uf7085c287ac65f5e(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                            ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                            ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                            .cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    buf,
                    len,
                    timeout,
                    bytes_transferred,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Receive an @c iovec of size @a iovcnt from the connected socket.
    pub unsafe fn recvv_n(
        __this: *const Self,
        mut iov: *mut iovec,
        mut n: libc::c_int,
        mut timeout: *const ACE_Time_Value,
        mut bytes_transferred: *mut libc::c_ulong,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::recvv_n(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                            ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                            ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                            .cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    ((iov) as *mut iovec),
                    n,
                    timeout,
                    bytes_transferred,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Try to send exactly @a len bytes from @a buf to the connection socket.
    pub unsafe fn send_n(
        __this: *const Self,
        mut buf: *const libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
        mut timeout: *const ACE_Time_Value,
        mut bytes_transferred: *mut libc::c_ulong,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::send_n_ucbbbc5a02ac043c9(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                            ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                            ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                            .cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    buf,
                    len,
                    flags,
                    timeout,
                    bytes_transferred,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Try to send exactly @a len bytes from @a buf to the connected socket.
    pub unsafe fn send_n_ubb6804fc1c1f9cef(
        __this: *const Self,
        mut buf: *const libc::c_void,
        mut len: libc::c_ulong,
        mut timeout: *const ACE_Time_Value,
        mut bytes_transferred: *mut libc::c_ulong,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::send_n_ufa3c0d7f3dea0191(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                            ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                            ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                            .cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    buf,
                    len,
                    timeout,
                    bytes_transferred,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Send all the message blocks chained through their @c next and
  /// @c cont pointers.  This call uses the underlying OS gather-write
  /// operation to reduce the domain-crossing penalty.*/
    pub unsafe fn send_n_ua41ccd9d69599d36(
        __this: *const Self,
        mut message_block: *const ACE_Message_Block,
        mut timeout: *const ACE_Time_Value,
        mut bytes_transferred: *mut libc::c_ulong,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::send_n(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                            ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                            ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                            .cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    message_block,
                    timeout,
                    bytes_transferred,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Send an @c iovec of size @a iovcnt to the connected socket.
    pub unsafe fn sendv_n(
        __this: *const Self,
        mut iov: *const iovec,
        mut n: libc::c_int,
        mut timeout: *const ACE_Time_Value,
        mut bytes_transferred: *mut libc::c_ulong,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::sendv_n(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                            ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                            ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                            .cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    ((iov) as *const iovec),
                    n,
                    timeout,
                    bytes_transferred,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn send_urg(
        __this: *const Self,
        mut ptr: *const libc::c_void,
        mut len: libc::c_ulong,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::send_ue0793508d1a7ed9b(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                            ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                            ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                            .cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    ptr,
                    len,
                    (((1 as libc::c_int)) as libc::c_int),
                    timeout,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn recv_urg(
        __this: *const Self,
        mut ptr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::recv_ubc58a1994981cdc8(
                    <ACE_IPC_SAP>::get_handle(
                        (::core::ptr::addr_of!(
                            (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                            ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                            ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                            ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                            .cast_mut())
                        )) as *const ACE_IPC_SAP,
                    ),
                    ptr,
                    len,
                    (((1 as libc::c_int)) as libc::c_int),
                    timeout,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Close down the reader.
    pub unsafe fn close_reader(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((<ACE_IPC_SAP>::get_handle(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                        ::core::ptr::addr_of!((* __this).__base_0) .cast:: < ACE_SOCK_IO
                        > ().cast_mut()).__base_0) .cast:: < ACE_SOCK > ().cast_mut())
                        .__base_0) .cast:: < ACE_IPC_SAP > ().cast_mut())
                    )) as *const ACE_IPC_SAP,
                ) as libc::c_int)) != ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return ACE_OS::shutdown_u7313014062aea7b1(
                        <ACE_IPC_SAP>::get_handle(
                            (::core::ptr::addr_of!(
                                (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                                ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                                ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                                ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                                .cast_mut())
                            )) as *const ACE_IPC_SAP,
                        ),
                        (((0 as libc::c_int)) as libc::c_int),
                    );
                } else {
                    return 0;
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Close down the writer.
    pub unsafe fn close_writer(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((<ACE_IPC_SAP>::get_handle(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                        ::core::ptr::addr_of!((* __this).__base_0) .cast:: < ACE_SOCK_IO
                        > ().cast_mut()).__base_0) .cast:: < ACE_SOCK > ().cast_mut())
                        .__base_0) .cast:: < ACE_IPC_SAP > ().cast_mut())
                    )) as *const ACE_IPC_SAP,
                ) as libc::c_int)) != ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return ACE_OS::shutdown_u7313014062aea7b1(
                        <ACE_IPC_SAP>::get_handle(
                            (::core::ptr::addr_of!(
                                (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                                ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                                ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK >
                                ().cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ()
                                .cast_mut())
                            )) as *const ACE_IPC_SAP,
                        ),
                        (((1 as libc::c_int)) as libc::c_int),
                    );
                } else {
                    return 0;
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Close down the socket (we need this to make things work correctly\n   * on Win32, which requires use to do a close_writer() before doing\n   * the close to avoid losing data)."]
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_SOCK_Stream5closeEv"]
            fn __ext(__this: *mut ACE_SOCK_Stream) -> libc::c_int;
        }
        __ext(__this as *mut ACE_SOCK_Stream)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK15ACE_SOCK_Stream4dumpEv"]
            fn __ext(__this: *const ACE_SOCK_Stream);
        }
        __ext(__this as *const ACE_SOCK_Stream)
    }
}
impl ACE_SOCK_Connector {
    ///Default constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            {}
            ()
        }
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    #[doc = "* Actively connect to a peer, producing a connected @c ACE_SOCK_Stream\n   * object if the connection succeeds.\n   *\n   * @param new_stream  The @c ACE_SOCK_Stream object that will be connected\n   *                    to the peer.\n   * @param remote_sap  The address that we are trying to connect to.\n   *                    The protocol family of @c remote_sap is used for\n   *                    the connected socket. That is, if @c remote_sap\n   *                    contains an IPv6 address, a socket with family\n   *                    PF_INET6 will be used, else it will be PF_INET.\n   * @param timeout     Pointer to an @c ACE_Time_Value object with amount\n   *                    of time to wait to connect. If the pointer is 0\n   *                    then the call blocks until the connection attempt\n   *                    is complete, whether it succeeds or fails.  If\n   *                    *timeout == {0, 0} then the connection is done\n   *                    using nonblocking mode.  In this case, if the\n   *                    connection can't be made immediately, this method\n   *                    returns -1 and errno == EWOULDBLOCK.\n   *                    If *timeout > {0, 0} then this is the maximum amount\n   *                    of time to wait before timing out; if the specified\n   *                    amount of time passes before the connection is made,\n   *                    this method returns -1 and errno == ETIME. Note\n   *                    the difference between this case and when a blocking\n   *                    connect is attempted that TCP times out - in the latter\n   *                    case, errno will be ETIMEDOUT.\n   * @param local_sap   (optional) The local address to bind to.  If it's\n   *                    the default value of @c ACE_Addr::sap_any then the\n   *                    OS will choose an unused port.\n   * @param reuse_addr  (optional) If the value is 1, the local address\n   *                    (@c local_sap) is reused, even if it hasn't been\n   *                    cleaned up yet.\n   * @param flags       Ignored.\n   * @param perms       Ignored.\n   * @param protocol    (optional) If value is 0, default SOCK_STREAM\n   *                    protocol is selected by kernel (typically TCP)."]
    pub unsafe fn new_at_ue24892fcc11674d1(
        __this: *mut Self,
        mut __a0: *mut ACE_SOCK_Stream,
        mut __a1: *const ACE_Addr,
        mut __a2: *const ACE_Time_Value,
        mut __a3: *const ACE_Addr,
        mut __a4: libc::c_int,
        mut __a5: libc::c_int,
        mut __a6: libc::c_int,
        mut __a7: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_SOCK_ConnectorC1ER15ACE_SOCK_StreamRK8ACE_AddrPK14ACE_Time_ValueS4_iiii"]
            fn __ext(
                __this: *mut ACE_SOCK_Connector,
                __a0: *mut ACE_SOCK_Stream,
                __a1: *const ACE_Addr,
                __a2: *const ACE_Time_Value,
                __a3: *const ACE_Addr,
                __a4: libc::c_int,
                __a5: libc::c_int,
                __a6: libc::c_int,
                __a7: libc::c_int,
            );
        }
        __ext(
            __this as *mut ACE_SOCK_Connector,
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
            __a7,
        )
    }
    pub unsafe fn new_ue24892fcc11674d1(
        mut __a0: *mut ACE_SOCK_Stream,
        mut __a1: *const ACE_Addr,
        mut __a2: *const ACE_Time_Value,
        mut __a3: *const ACE_Addr,
        mut __a4: libc::c_int,
        mut __a5: libc::c_int,
        mut __a6: libc::c_int,
        mut __a7: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ue24892fcc11674d1(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
            __a7,
        );
        __obj
    }
    #[doc = "* Actively connect to a peer, producing a connected @c ACE_SOCK_Stream\n   * object if the connection succeeds.\n   *\n   * @param new_stream  The @c ACE_SOCK_Stream object that will be connected\n   *                    to the peer.\n   * @param remote_sap  The address that we are trying to connect to.\n   *                    The protocol family of @c remote_sap is used for\n   *                    the connected socket. That is, if @c remote_sap\n   *                    contains an IPv6 address, a socket with family\n   *                    PF_INET6 will be used, else it will be PF_INET.\n   * @param timeout     Pointer to an @c ACE_Time_Value object with amount\n   *                    of time to wait to connect. If the pointer is 0\n   *                    then the call blocks until the connection attempt\n   *                    is complete, whether it succeeds or fails.  If\n   *                    *timeout == {0, 0} then the connection is done\n   *                    using nonblocking mode.  In this case, if the\n   *                    connection can't be made immediately, this method\n   *                    returns -1 and errno == EWOULDBLOCK.\n   *                    If *timeout > {0, 0} then this is the maximum amount\n   *                    of time to wait before timing out; if the specified\n   *                    amount of time passes before the connection is made,\n   *                    this method returns -1 and errno == ETIME. Note\n   *                    the difference between this case and when a blocking\n   *                    connect is attempted that TCP times out - in the latter\n   *                    case, errno will be ETIMEDOUT.\n   * @param local_sap   (optional) The local address to bind to.  If it's\n   *                    the default value of @c ACE_Addr::sap_any then the\n   *                    OS will choose an unused port.\n   * @param reuse_addr  (optional) If the value is 1, the local address\n   *                    (@c local_sap) is reused, even if it hasn't been\n   *                    cleaned up yet.\n   * @param flags       Ignored.\n   * @param perms       Ignored.\n   * @param protocol    (optional) If value is 0, default SOCK_STREAM\n   *                    protocol is selected by kernel (typically TCP).\n   *\n   * @return            Returns 0 if the connection succeeds. If it fails,\n   *                    -1 is returned and errno contains a specific error\n   *                    code."]
    pub unsafe fn connect(
        __this: *mut Self,
        mut new_stream: *mut ACE_SOCK_Stream,
        mut remote_sap: *const ACE_Addr,
        mut timeout: *const ACE_Time_Value,
        mut local_sap: *const ACE_Addr,
        mut reuse_addr: libc::c_int,
        mut flags: libc::c_int,
        mut perms: libc::c_int,
        mut protocol: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_SOCK_Connector7connectER15ACE_SOCK_StreamRK8ACE_AddrPK14ACE_Time_ValueS4_iiii"]
            fn __ext(
                __this: *mut ACE_SOCK_Connector,
                new_stream: *mut ACE_SOCK_Stream,
                remote_sap: *const ACE_Addr,
                timeout: *const ACE_Time_Value,
                local_sap: *const ACE_Addr,
                reuse_addr: libc::c_int,
                flags: libc::c_int,
                perms: libc::c_int,
                protocol: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_SOCK_Connector,
            new_stream,
            remote_sap,
            timeout,
            local_sap,
            reuse_addr,
            flags,
            perms,
            protocol,
        )
    }
    #[doc = "* Actively connect to a peer, producing a connected @c ACE_SOCK_Stream\n   * object if the connection succeeds.\n   *\n   * @param new_stream  The @c ACE_SOCK_Stream object that will be connected\n   *                    to the peer.\n   * @param remote_sap  The address that we are trying to connect to.\n   *                    The protocol family of @c remote_sap is used for\n   *                    the connected socket. That is, if @c remote_sap\n   *                    contains an IPv6 address, a socket with family\n   *                    PF_INET6 will be used, else it will be PF_INET.\n   * @param qos_params  Contains QoS parameters that are passed to the\n   *                    IntServ (RSVP) and DiffServ protocols.\n   *                    @see ACE_QoS_Params.\n   * @param timeout     Pointer to an @c ACE_Time_Value object with amount\n   *                    of time to wait to connect. If the pointer is 0\n   *                    then the call blocks until the connection attempt\n   *                    is complete, whether it succeeds or fails.  If\n   *                    *timeout == {0, 0} then the connection is done\n   *                    using nonblocking mode.  In this case, if the\n   *                    connection can't be made immediately, this method\n   *                    returns -1 and errno == EWOULDBLOCK.\n   *                    If *timeout > {0, 0} then this is the maximum amount\n   *                    of time to wait before timing out; if the specified\n   *                    amount of time passes before the connection is made,\n   *                    this method returns -1 and errno == ETIME. Note\n   *                    the difference between this case and when a blocking\n   *                    connect is attempted that TCP times out - in the latter\n   *                    case, errno will be ETIMEDOUT.\n   * @param local_sap   (optional) The local address to bind to.  If it's\n   *                    the default value of @c ACE_Addr::sap_any then the\n   *                    OS will choose an unused port.\n   * @param reuse_addr  (optional) If the value is 1, the local address\n   *                    (@c local_sap) is reused, even if it hasn't been\n   *                    cleaned up yet.\n   * @param flags       Ignored.\n   * @param perms       Ignored.\n   *\n   * @return            Returns 0 if the connection succeeds. If it fails,\n   *                    -1 is returned and errno contains a specific error\n   *                    code."]
    pub unsafe fn connect_ub2074e6d75d82102(
        __this: *mut Self,
        mut new_stream: *mut ACE_SOCK_Stream,
        mut remote_sap: *const ACE_Addr,
        mut qos_params: ACE_QoS_Params,
        mut timeout: *const ACE_Time_Value,
        mut local_sap: *const ACE_Addr,
        mut protocolinfo: *mut ACE_Protocol_Info,
        mut g: libc::c_ulong,
        mut flags: libc::c_ulong,
        mut reuse_addr: libc::c_int,
        mut perms: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_SOCK_Connector7connectER15ACE_SOCK_StreamRK8ACE_Addr14ACE_QoS_ParamsPK14ACE_Time_ValueS4_P17ACE_Protocol_Infommii"]
            fn __ext(
                __this: *mut ACE_SOCK_Connector,
                new_stream: *mut ACE_SOCK_Stream,
                remote_sap: *const ACE_Addr,
                qos_params: ACE_QoS_Params,
                timeout: *const ACE_Time_Value,
                local_sap: *const ACE_Addr,
                protocolinfo: *mut ACE_Protocol_Info,
                g: libc::c_ulong,
                flags: libc::c_ulong,
                reuse_addr: libc::c_int,
                perms: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_SOCK_Connector,
            new_stream,
            remote_sap,
            qos_params,
            timeout,
            local_sap,
            protocolinfo,
            g,
            flags,
            reuse_addr,
            perms,
        )
    }
    #[doc = "* Try to complete a nonblocking connection that was begun by a\n   * previous call to connect with a {0, 0} ACE_Time_Value timeout.\n   * @see connect().\n   *\n   * @param new_stream  The @c ACE_SOCK_Stream object that will be connected\n   *                    to the peer.\n   * @param remote_sap  If non-0, it points to the @c ACE_INET_Addr object\n   *                    that will contain the address of the connected peer.\n   * @param timeout     Same values and return value possibilities as for\n   *                    connect(). @see connect()."]
    pub unsafe fn complete(
        __this: *mut Self,
        mut new_stream: *mut ACE_SOCK_Stream,
        mut remote_sap: *mut ACE_Addr,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_SOCK_Connector8completeER15ACE_SOCK_StreamP8ACE_AddrPK14ACE_Time_Value"]
            fn __ext(
                __this: *mut ACE_SOCK_Connector,
                new_stream: *mut ACE_SOCK_Stream,
                remote_sap: *mut ACE_Addr,
                timeout: *const ACE_Time_Value,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_SOCK_Connector, new_stream, remote_sap, timeout)
    }
    ///Resets any event associations on this handle
    pub unsafe fn reset_new_handle(__this: *mut Self, mut handle: libc::c_int) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let _ = (handle);
                };
                return crate::__cxx_std::__Truthy::__truthy(false);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_SOCK_Connector4dumpEv"]
            fn __ext(__this: *const ACE_SOCK_Connector);
        }
        __ext(__this as *const ACE_SOCK_Connector)
    }
    /**Perform operations that ensure the socket is opened using
  /// BSD-style semantics (no QoS).*/
    pub unsafe fn shared_open(
        __this: *mut Self,
        mut new_stream: *mut ACE_SOCK_Stream,
        mut protocol_family: libc::c_int,
        mut protocol: libc::c_int,
        mut reuse_addr: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_SOCK_Connector11shared_openER15ACE_SOCK_Streamiii"]
            fn __ext(
                __this: *mut ACE_SOCK_Connector,
                new_stream: *mut ACE_SOCK_Stream,
                protocol_family: libc::c_int,
                protocol: libc::c_int,
                reuse_addr: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_SOCK_Connector,
            new_stream,
            protocol_family,
            protocol,
            reuse_addr,
        )
    }
    /**Perform operations that ensure the socket is opened using
  /// QoS-enabled semantics.*/
    pub unsafe fn shared_open_u9e80e8f49f0e633f(
        __this: *mut Self,
        mut new_stream: *mut ACE_SOCK_Stream,
        mut protocol_family: libc::c_int,
        mut protocol: libc::c_int,
        mut protocolinfo: *mut ACE_Protocol_Info,
        mut g: libc::c_ulong,
        mut flags: libc::c_ulong,
        mut reuse_addr: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_SOCK_Connector11shared_openER15ACE_SOCK_StreamiiP17ACE_Protocol_Infommi"]
            fn __ext(
                __this: *mut ACE_SOCK_Connector,
                new_stream: *mut ACE_SOCK_Stream,
                protocol_family: libc::c_int,
                protocol: libc::c_int,
                protocolinfo: *mut ACE_Protocol_Info,
                g: libc::c_ulong,
                flags: libc::c_ulong,
                reuse_addr: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_SOCK_Connector,
            new_stream,
            protocol_family,
            protocol,
            protocolinfo,
            g,
            flags,
            reuse_addr,
        )
    }
    ///Perform operations that must be called before <ACE_OS::connect>.
    pub unsafe fn shared_connect_start(
        __this: *mut Self,
        mut new_stream: *mut ACE_SOCK_Stream,
        mut timeout: *const ACE_Time_Value,
        mut local_sap: *const ACE_Addr,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_SOCK_Connector20shared_connect_startER15ACE_SOCK_StreamPK14ACE_Time_ValueRK8ACE_Addr"]
            fn __ext(
                __this: *mut ACE_SOCK_Connector,
                new_stream: *mut ACE_SOCK_Stream,
                timeout: *const ACE_Time_Value,
                local_sap: *const ACE_Addr,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_SOCK_Connector, new_stream, timeout, local_sap)
    }
    ///Perform operations that must be called after <ACE_OS::connect>.
    pub unsafe fn shared_connect_finish(
        __this: *mut Self,
        mut new_stream: *mut ACE_SOCK_Stream,
        mut timeout: *const ACE_Time_Value,
        mut result: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_SOCK_Connector21shared_connect_finishER15ACE_SOCK_StreamPK14ACE_Time_Valuei"]
            fn __ext(
                __this: *mut ACE_SOCK_Connector,
                new_stream: *mut ACE_SOCK_Stream,
                timeout: *const ACE_Time_Value,
                result: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_SOCK_Connector, new_stream, timeout, result)
    }
}
impl ACE_Log_Msg_IPC {
    ///Constructor
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            {
                let __this: *mut ACE_Log_Msg_Backend = ::core::ptr::addr_of_mut!(
                    (* __this).__base_0
                )
                    .cast::<ACE_Log_Msg_Backend>();
                *(__this as *mut *const ()) = &__VTBL_udbddea67410264fa
                    as *const __Vtbl_udbddea67410264fa as *const ();
            }
            *(__this as *mut *const ()) = &__VTBL_uae6401f90b61767c
                as *const __Vtbl_uae6401f90b61767c as *const ();
            <ACE_SOCK_Stream>::new_at(
                ::core::ptr::addr_of_mut!((* __this).message_queue_)
                    .cast::<ACE_SOCK_Stream>(),
            );
            {}
            ()
        }
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Open a new connection
    pub unsafe fn open(
        __this: *mut Self,
        mut logger_key: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut con: ACE_SOCK_Connector = ::core::mem::MaybeUninit::<
                    ACE_SOCK_Connector,
                >::zeroed()
                    .assume_init();
                <ACE_SOCK_Connector>::new_at(
                    (::core::ptr::addr_of_mut!(con)) as *mut ACE_SOCK_Connector,
                );
                return <ACE_SOCK_Connector>::connect(
                    (::core::ptr::addr_of_mut!(con)) as *mut ACE_SOCK_Connector,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).message_queue_) .cast:: <
                        ACE_SOCK_Stream > ().cast_mut())
                    ),
                    (&((<ACE_INET_Addr>::new_u1f600faeb3053862(logger_key, 0))).__base_0
                        as *const ::core::mem::ManuallyDrop<ACE_Addr>)
                        .cast::<ACE_Addr>(),
                    ((0) as *const ACE_Time_Value),
                    ::core::ptr::addr_of!(ACE_Addr_sap_any),
                    0,
                    0,
                    0,
                    0,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Reset the backend.  When changing the logging destination the\n   * backend may need to properly disconnect from the remote logging\n   * daemon and reclaim some local resources.  But we try to reduce\n   * the number of local allocations/deallocations."]
    pub unsafe fn reset(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((<ACE_IPC_SAP>::get_handle(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* ::core::ptr::addr_of!((*
                        ::core::ptr::addr_of!((* (::core::ptr::addr_of!((*
                        ::core::ptr::addr_of!((* __this).message_queue_) .cast:: <
                        ACE_SOCK_Stream > ().cast_mut())))).__base_0) .cast:: <
                        ACE_SOCK_IO > ().cast_mut()).__base_0) .cast:: < ACE_SOCK > ()
                        .cast_mut()).__base_0) .cast:: < ACE_IPC_SAP > ().cast_mut())
                    )) as *const ACE_IPC_SAP,
                ) as libc::c_int)) != ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return {
                        let __obj: *mut ACE_Log_Msg_Backend = (__this)
                            as *mut ACE_Log_Msg_Backend;
                        let __vt: *const __Vtbl_udbddea67410264fa = *(__obj
                            as *const *const __Vtbl_udbddea67410264fa);
                        ((*__vt).vfn_ubf24d44d4595cf9b)(__obj)
                    };
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_SOCK_Stream>::close(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).message_queue_) .cast:: <
                        ACE_SOCK_Stream > ().cast_mut())
                    )) as *mut ACE_SOCK_Stream,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn log(
        __this: *mut Self,
        mut log_record: *mut ACE_Log_Record,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut max_payload_size: libc::c_ulong = ((((((((((((((((((((4)
                    as libc::c_int))
                    .wrapping_add((4) as libc::c_int)) as libc::c_int))
                    .wrapping_add((12) as libc::c_int)) as libc::c_int))
                    .wrapping_add((4) as libc::c_int)) as libc::c_int))
                    .wrapping_add((4) as libc::c_int)) as libc::c_ulong))
                    .wrapping_add(
                        (<ACE_Log_Record>::msg_data_len(
                            (::core::ptr::addr_of!((* log_record)))
                                as *const ACE_Log_Record,
                        )) as libc::c_ulong,
                    )) as libc::c_ulong))
                    .wrapping_add(((8 as libc::c_int)) as libc::c_ulong))
                    as libc::c_ulong);
                let mut payload: ACE_OutputCDR = ::core::mem::MaybeUninit::<
                    ACE_OutputCDR,
                >::zeroed()
                    .assume_init();
                <ACE_OutputCDR>::new_at(
                    (::core::ptr::addr_of_mut!(payload)) as *mut ACE_OutputCDR,
                    ((max_payload_size) as libc::c_ulong),
                    ((BYTE_ORDER_NATIVE) as libc::c_int),
                    ((0) as *mut ACE_Allocator),
                    ((0) as *mut ACE_Allocator),
                    ((0) as *mut ACE_Allocator),
                    ((256) as libc::c_ulong),
                    ((1) as libc::c_uchar),
                    ((2) as libc::c_uchar),
                );
                if (((!((((operator_shl_ub3e53c4872f84b62(
                    ::core::ptr::addr_of_mut!(payload),
                    ::core::ptr::addr_of!((* log_record)),
                ))) != 0)) as libc::c_int) as libc::c_int) != 0)
                {
                    return (((-((1) as libc::c_int))) as libc::c_long);
                }
                let mut length: libc::c_uint = ((ACE_Utils::truncate_cast___unsigned_long__ue27a31e3ddf7f306(
                    ((<ACE_OutputCDR>::total_length(
                        (::core::ptr::addr_of!(payload)) as *const ACE_OutputCDR,
                    )) as libc::c_ulong),
                )) as libc::c_uint);
                let mut header: ACE_OutputCDR = ::core::mem::MaybeUninit::<
                    ACE_OutputCDR,
                >::zeroed()
                    .assume_init();
                <ACE_OutputCDR>::new_at(
                    (::core::ptr::addr_of_mut!(header)) as *mut ACE_OutputCDR,
                    ((((((8 as libc::c_int)) as libc::c_int))
                        .wrapping_add((8) as libc::c_int)) as libc::c_ulong),
                    ((BYTE_ORDER_NATIVE) as libc::c_int),
                    ((0) as *mut ACE_Allocator),
                    ((0) as *mut ACE_Allocator),
                    ((0) as *mut ACE_Allocator),
                    ((256) as libc::c_ulong),
                    ((1) as libc::c_uchar),
                    ((2) as libc::c_uchar),
                );
                if (((!((((operator_shl_ue13a06333e680a27(
                    ::core::ptr::addr_of_mut!(header),
                    (<from_boolean>::new(((1) != 0))),
                )) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                {
                    return (((-((1) as libc::c_int))) as libc::c_long);
                }
                if (((!((((operator_shl_u70ffb65145327a2d(
                    ::core::ptr::addr_of_mut!(header),
                    (length as libc::c_uint),
                )) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                {
                    return (((-((1) as libc::c_int))) as libc::c_long);
                }
                let mut iov: [iovec; 2usize] = unsafe {
                    ::core::mem::MaybeUninit::<[iovec; 2usize]>::zeroed().assume_init()
                };
                (iov)[(0) as usize].iov_base = ((<ACE_Message_Block>::rd_ptr(
                    (<ACE_OutputCDR>::begin(
                        (::core::ptr::addr_of!(header)) as *const ACE_OutputCDR,
                    )) as *const ACE_Message_Block,
                )) as *mut libc::c_void);
                (iov)[(0) as usize].iov_len = ((8) as libc::c_ulong);
                (iov)[(1) as usize].iov_base = ((<ACE_Message_Block>::rd_ptr(
                    (<ACE_OutputCDR>::begin(
                        (::core::ptr::addr_of!(payload)) as *const ACE_OutputCDR,
                    )) as *const ACE_Message_Block,
                )) as *mut libc::c_void);
                (iov)[(1) as usize].iov_len = ((length) as libc::c_ulong);
                return <ACE_SOCK_Stream>::sendv_n(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).message_queue_) .cast:: <
                        ACE_SOCK_Stream > ().cast_mut())
                    )) as *const ACE_SOCK_Stream,
                    ((iov).as_ptr() as *const iovec),
                    2,
                    ((0) as *const ACE_Time_Value),
                    ((0) as *mut libc::c_ulong),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_InputCDR {
    #[doc = "* Create an input stream from an arbitrary buffer.  The buffer must\n   * be properly aligned because this constructor will *not* work if\n   * the buffer is aligned unproperly.See ACE_ptr_align_binary() for\n   * instructions on how to align a pointer properly and use\n   * ACE_CDR::MAX_ALIGNMENT for the correct alignment."]
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: libc::c_uchar,
        mut __a4: libc::c_uchar,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRC1EPKcmihh"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                __a0: *const libc::c_char,
                __a1: libc::c_ulong,
                __a2: libc::c_int,
                __a3: libc::c_uchar,
                __a4: libc::c_uchar,
            );
        }
        __ext(__this as *mut ACE_InputCDR, __a0, __a1, __a2, __a3, __a4)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: libc::c_uchar,
        mut __a4: libc::c_uchar,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1, __a2, __a3, __a4);
        __obj
    }
    /**Create an empty input stream. The caller is responsible for
  /// putting the right data and providing the right alignment.*/
    pub unsafe fn new_at_ufac38467db8bcb9b(
        __this: *mut Self,
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: libc::c_uchar,
        mut __a3: libc::c_uchar,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRC1Emihh"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                __a0: libc::c_ulong,
                __a1: libc::c_int,
                __a2: libc::c_uchar,
                __a3: libc::c_uchar,
            );
        }
        __ext(__this as *mut ACE_InputCDR, __a0, __a1, __a2, __a3)
    }
    pub unsafe fn new_ufac38467db8bcb9b(
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: libc::c_uchar,
        mut __a3: libc::c_uchar,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ufac38467db8bcb9b(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
        );
        __obj
    }
    #[doc = "Create an input stream from an ACE_Message_Block\n  /**\n   * The alignment of the @a data block is carried into the new\n   * ACE_InputCDR object. This constructor either increments the\n   * @a data reference count, or copies the data (if it's a compound\n   * message block) so the caller can release the block immediately\n   * upon return."]
    pub unsafe fn new_at_u241ed9d1affd74c9(
        __this: *mut Self,
        mut __a0: *const ACE_Message_Block,
        mut __a1: libc::c_int,
        mut __a2: libc::c_uchar,
        mut __a3: libc::c_uchar,
        mut __a4: *mut ACE_Lock,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRC1EPK17ACE_Message_BlockihhP8ACE_Lock"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                __a0: *const ACE_Message_Block,
                __a1: libc::c_int,
                __a2: libc::c_uchar,
                __a3: libc::c_uchar,
                __a4: *mut ACE_Lock,
            );
        }
        __ext(__this as *mut ACE_InputCDR, __a0, __a1, __a2, __a3, __a4)
    }
    pub unsafe fn new_u241ed9d1affd74c9(
        mut __a0: *const ACE_Message_Block,
        mut __a1: libc::c_int,
        mut __a2: libc::c_uchar,
        mut __a3: libc::c_uchar,
        mut __a4: *mut ACE_Lock,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u241ed9d1affd74c9(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
        );
        __obj
    }
    /**Create an input stream from an ACE_Data_Block. The @a flag
  /// indicates whether the @a data can be deleted by the CDR stream
  /// or not*/
    pub unsafe fn new_at_ud0fc97bcba0855d7(
        __this: *mut Self,
        mut __a0: *mut ACE_Data_Block,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: libc::c_uchar,
        mut __a4: libc::c_uchar,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRC1EP14ACE_Data_Blockmihh"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                __a0: *mut ACE_Data_Block,
                __a1: libc::c_ulong,
                __a2: libc::c_int,
                __a3: libc::c_uchar,
                __a4: libc::c_uchar,
            );
        }
        __ext(__this as *mut ACE_InputCDR, __a0, __a1, __a2, __a3, __a4)
    }
    pub unsafe fn new_ud0fc97bcba0855d7(
        mut __a0: *mut ACE_Data_Block,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: libc::c_uchar,
        mut __a4: libc::c_uchar,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ud0fc97bcba0855d7(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
        );
        __obj
    }
    /**Create an input stream from an ACE_Data_Block. It also sets the
  /// read and write pointers at the desired positions. This would be
  /// helpful if the applications desires to create a new CDR stream
  /// from a semi-processed datablock.*/
    pub unsafe fn new_at_u0c903443185e52c7(
        __this: *mut Self,
        mut __a0: *mut ACE_Data_Block,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_ulong,
        mut __a3: libc::c_ulong,
        mut __a4: libc::c_int,
        mut __a5: libc::c_uchar,
        mut __a6: libc::c_uchar,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRC1EP14ACE_Data_Blockmmmihh"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                __a0: *mut ACE_Data_Block,
                __a1: libc::c_ulong,
                __a2: libc::c_ulong,
                __a3: libc::c_ulong,
                __a4: libc::c_int,
                __a5: libc::c_uchar,
                __a6: libc::c_uchar,
            );
        }
        __ext(__this as *mut ACE_InputCDR, __a0, __a1, __a2, __a3, __a4, __a5, __a6)
    }
    pub unsafe fn new_u0c903443185e52c7(
        mut __a0: *mut ACE_Data_Block,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_ulong,
        mut __a3: libc::c_ulong,
        mut __a4: libc::c_int,
        mut __a5: libc::c_uchar,
        mut __a6: libc::c_uchar,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u0c903443185e52c7(
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
    #[doc = "* These make a copy of the current stream state, but do not copy\n   * the internal buffer, so the same stream can be read multiple\n   * times efficiently."]
    pub unsafe fn new_at_u2d3980f6dc00e190(
        __this: *mut Self,
        mut __a0: *const ACE_InputCDR,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRC1ERKS_"]
            fn __ext(__this: *mut ACE_InputCDR, __a0: *const ACE_InputCDR);
        }
        __ext(__this as *mut ACE_InputCDR, __a0)
    }
    pub unsafe fn new_u2d3980f6dc00e190(mut __a0: *const ACE_InputCDR) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u2d3980f6dc00e190(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: *const ACE_InputCDR,
    ) -> *mut ACE_InputCDR {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRaSERKS_"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                rhs: *const ACE_InputCDR,
            ) -> *mut ACE_InputCDR;
        }
        __ext(__this as *mut ACE_InputCDR, rhs)
    }
    /**When interpreting indirected TypeCodes it is useful to make a
  /// "copy" of the stream starting in the new position.*/
    pub unsafe fn new_at_u2afe0b5c26267235(
        __this: *mut Self,
        mut __a0: *const ACE_InputCDR,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRC1ERKS_mi"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                __a0: *const ACE_InputCDR,
                __a1: libc::c_ulong,
                __a2: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_InputCDR, __a0, __a1, __a2)
    }
    pub unsafe fn new_u2afe0b5c26267235(
        mut __a0: *const ACE_InputCDR,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u2afe0b5c26267235(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    /**This creates an encapsulated stream, the first byte must be (per
  /// the spec) the byte order of the encapsulation.*/
    pub unsafe fn new_at_u3d7c8f9fc87e7415(
        __this: *mut Self,
        mut __a0: *const ACE_InputCDR,
        mut __a1: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRC1ERKS_m"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                __a0: *const ACE_InputCDR,
                __a1: libc::c_ulong,
            );
        }
        __ext(__this as *mut ACE_InputCDR, __a0, __a1)
    }
    pub unsafe fn new_u3d7c8f9fc87e7415(
        mut __a0: *const ACE_InputCDR,
        mut __a1: libc::c_ulong,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u3d7c8f9fc87e7415(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Create an input CDR from an output CDR.
    pub unsafe fn new_at_u55373837f476ee84(
        __this: *mut Self,
        mut __a0: *const ACE_OutputCDR,
        mut __a1: *mut ACE_Allocator,
        mut __a2: *mut ACE_Allocator,
        mut __a3: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDRC1ERK13ACE_OutputCDRP13ACE_AllocatorS4_S4_"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                __a0: *const ACE_OutputCDR,
                __a1: *mut ACE_Allocator,
                __a2: *mut ACE_Allocator,
                __a3: *mut ACE_Allocator,
            );
        }
        __ext(__this as *mut ACE_InputCDR, __a0, __a1, __a2, __a3)
    }
    pub unsafe fn new_u55373837f476ee84(
        mut __a0: *const ACE_OutputCDR,
        mut __a1: *mut ACE_Allocator,
        mut __a2: *mut ACE_Allocator,
        mut __a3: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u55373837f476ee84(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
        );
        __obj
    }
    pub unsafe fn read_boolean(__this: *mut Self, mut x: *mut bool) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut tmp: libc::c_uchar = ((0) as libc::c_uchar);
                {
                    let _ = <ACE_InputCDR>::read_octet(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(tmp),
                    );
                };
                (*x) = ((if ((tmp) != 0) { true } else { false }) as bool);
                return crate::__cxx_std::__Truthy::__truthy((*__this).good_bit_);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_char(__this: *mut Self, mut x: *mut libc::c_char) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).char_translator_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    let mut temp: *mut libc::c_void = ((::core::ptr::addr_of_mut!((* x))
                        as *mut libc::c_char) as *mut libc::c_void);
                    return crate::__cxx_std::__Truthy::__truthy(
                        <ACE_InputCDR>::read_1(
                            (__this) as *mut ACE_InputCDR,
                            (temp as *mut libc::c_uchar),
                        ),
                    );
                }
                return crate::__cxx_std::__Truthy::__truthy({
                    let __obj: *mut ACE_Char_Codeset_Translator = ((*__this)
                        .char_translator_) as *mut ACE_Char_Codeset_Translator;
                    let __vt: *const __Vtbl_ue1bfefa7873110ae = *(__obj
                        as *const *const __Vtbl_ue1bfefa7873110ae);
                    ((*__vt)
                        .vfn_u5c6599352240b84d)(
                        __obj,
                        __this,
                        ::core::ptr::addr_of_mut!((* x)),
                    )
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_wchar(__this: *mut Self, mut x: *mut libc::wchar_t) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR10read_wcharERw"]
            fn __ext(__this: *mut ACE_InputCDR, x: *mut libc::wchar_t) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_octet(__this: *mut Self, mut x: *mut libc::c_uchar) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_1(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!((* x)) as *mut libc::c_uchar,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_short(__this: *mut Self, mut x: *mut libc::c_short) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: *mut libc::c_void = ((::core::ptr::addr_of_mut!((* x))
                    as *mut libc::c_short) as *mut libc::c_void);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_2(
                        (__this) as *mut ACE_InputCDR,
                        (temp as *mut libc::c_ushort),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_ushort(__this: *mut Self, mut x: *mut libc::c_ushort) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_2(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!((* x)) as *mut libc::c_ushort,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_long(__this: *mut Self, mut x: *mut libc::c_int) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: *mut libc::c_void = ((::core::ptr::addr_of_mut!((* x))
                    as *mut libc::c_int) as *mut libc::c_void);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_4(
                        (__this) as *mut ACE_InputCDR,
                        (temp as *mut libc::c_uint),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_ulong(__this: *mut Self, mut x: *mut libc::c_uint) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_4(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!((* x)) as *mut libc::c_uint,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_longlong(__this: *mut Self, mut x: *mut libc::c_long) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: *mut libc::c_void = ((::core::ptr::addr_of_mut!((* x))
                    as *mut libc::c_long) as *mut libc::c_void);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_8(
                        (__this) as *mut ACE_InputCDR,
                        (temp as *mut libc::c_ulong),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_ulonglong(__this: *mut Self, mut x: *mut libc::c_ulong) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_8(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!((* x)) as *mut libc::c_ulong,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_float(__this: *mut Self, mut x: *mut libc::c_float) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: *mut libc::c_void = ((::core::ptr::addr_of_mut!((* x))
                    as *mut libc::c_float) as *mut libc::c_void);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_4(
                        (__this) as *mut ACE_InputCDR,
                        (temp as *mut libc::c_uint),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_double(__this: *mut Self, mut x: *mut libc::c_double) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: *mut libc::c_void = ((::core::ptr::addr_of_mut!((* x))
                    as *mut libc::c_double) as *mut libc::c_void);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_8(
                        (__this) as *mut ACE_InputCDR,
                        (temp as *mut libc::c_ulong),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_longdouble(
        __this: *mut Self,
        mut x: *mut crate::__f80::F80,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_16(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!((* x)) as *mut crate::__f80::F80,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_fixed(__this: *mut Self, mut x: *mut Fixed) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut a: [libc::c_uchar; 16usize] = unsafe { ::core::mem::zeroed() };
                {
                    let mut i: libc::c_int = 0;
                    'for_0: loop {
                        if !((((((i as libc::c_int)) < (((16) as libc::c_int)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break;
                        }
                        'cont_0: loop {
                            {
                                {
                                    if (((!(((<ACE_InputCDR>::read_1(
                                        (__this) as *mut ACE_InputCDR,
                                        (((a).as_ptr() as *mut libc::c_uchar))
                                            .wrapping_offset((i) as isize),
                                    ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int)
                                        != 0)
                                    {
                                        return crate::__cxx_std::__Truthy::__truthy(
                                            ((false) as bool),
                                        );
                                    }
                                    let mut low: libc::c_uint = ((((((a)[(i) as usize])
                                        as libc::c_int)) & ((0xf) as libc::c_int)) as libc::c_uint);
                                    if (((((((((low as libc::c_uint))
                                        == (((Fixed_POSITIVE) as libc::c_uint))) as libc::c_int
                                        as libc::c_int) != 0)
                                        || (((((low as libc::c_uint))
                                            == (((Fixed_NEGATIVE) as libc::c_uint))) as libc::c_int
                                            as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                                    {
                                        {
                                            let __v = <Fixed>::from_octets(
                                                ((a).as_ptr() as *const libc::c_uchar),
                                                (((i) as libc::c_int)).wrapping_add((1) as libc::c_int),
                                                ((0) as libc::c_uint),
                                            );
                                            let __asg_p = ::core::ptr::addr_of_mut!((* x));
                                            *__asg_p = __v;
                                            __asg_p
                                        };
                                        return crate::__cxx_std::__Truthy::__truthy(
                                            ((true) as bool),
                                        );
                                    }
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                        {
                            let __lv = &mut (i);
                            *__lv = (*__lv).wrapping_add(1);
                            *__lv
                        };
                    }
                }
                return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_int8(__this: *mut Self, mut x: *mut libc::c_schar) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_1(
                        (__this) as *mut ACE_InputCDR,
                        (::core::ptr::addr_of_mut!((* x)) as *mut libc::c_schar
                            as *mut libc::c_uchar),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_uint8(__this: *mut Self, mut x: *mut libc::c_uchar) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_1(
                        (__this) as *mut ACE_InputCDR,
                        (::core::ptr::addr_of_mut!((* x)) as *mut libc::c_uchar
                            as *mut libc::c_uchar),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_string(__this: *mut Self, mut x: *mut *mut libc::c_char) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR11read_stringERPc"]
            fn __ext(__this: *mut ACE_InputCDR, x: *mut *mut libc::c_char) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_string_u34b00bd9d71b8282(
        __this: *mut Self,
        mut x: *mut ACE_String_Base_char_,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR11read_stringER15ACE_String_BaseIcE"]
            fn __ext(__this: *mut ACE_InputCDR, x: *mut ACE_String_Base_char_) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_wstring(
        __this: *mut Self,
        mut x: *mut *mut libc::wchar_t,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR12read_wstringERPw"]
            fn __ext(__this: *mut ACE_InputCDR, x: *mut *mut libc::wchar_t) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_string_ued7770fc0812e3c8(
        __this: *mut Self,
        mut x: *mut crate::__cxx_std::String,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR11read_stringERNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                x: *mut crate::__cxx_std::String,
            ) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_wstring_u8ac5af3ed766eb27(
        __this: *mut Self,
        mut x: *mut crate::__cxx_std::WString,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR12read_wstringERNSt7__cxx1112basic_stringIwSt11char_traitsIwESaIwEEE"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                x: *mut crate::__cxx_std::WString,
            ) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_boolean_array(
        __this: *mut Self,
        mut x: *mut bool,
        mut length: libc::c_uint,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR18read_boolean_arrayEPbj"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                x: *mut bool,
                length: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x, length)
    }
    pub unsafe fn read_char_array(
        __this: *mut Self,
        mut x: *mut libc::c_char,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((length as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                if ((((((*__this).char_translator_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    return crate::__cxx_std::__Truthy::__truthy(
                        <ACE_InputCDR>::read_array(
                            (__this) as *mut ACE_InputCDR,
                            ((x) as *mut libc::c_void),
                            (((1 as libc::c_int)) as libc::c_ulong),
                            (((1 as libc::c_int)) as libc::c_ulong),
                            length,
                        ),
                    );
                }
                return crate::__cxx_std::__Truthy::__truthy({
                    let __obj: *mut ACE_Char_Codeset_Translator = ((*__this)
                        .char_translator_) as *mut ACE_Char_Codeset_Translator;
                    let __vt: *const __Vtbl_ue1bfefa7873110ae = *(__obj
                        as *const *const __Vtbl_ue1bfefa7873110ae);
                    ((*__vt).vfn_u8702df99cfecd82b)(__obj, __this, x, length)
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_wchar_array(
        __this: *mut Self,
        mut x: *mut libc::wchar_t,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_ulong))
                    .wrapping_mul((ACE_OutputCDR_wchar_maxbytes_) as libc::c_ulong)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                if ((((!((*__this).wchar_translator_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    return crate::__cxx_std::__Truthy::__truthy({
                        let __obj: *mut ACE_WChar_Codeset_Translator = ((*__this)
                            .wchar_translator_) as *mut ACE_WChar_Codeset_Translator;
                        let __vt: *const __Vtbl_u7f71c32ff7e5c9bb = *(__obj
                            as *const *const __Vtbl_u7f71c32ff7e5c9bb);
                        ((*__vt).vfn_uc4b8c672331a5a94)(__obj, __this, x, length)
                    });
                }
                if (((((ACE_OutputCDR_wchar_maxbytes_ as libc::c_ulong))
                    != (((4) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    return crate::__cxx_std::__Truthy::__truthy(
                        <ACE_InputCDR>::read_wchar_array_i(
                            (__this) as *mut ACE_InputCDR,
                            x,
                            length,
                        ),
                    );
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        ((4) as libc::c_ulong),
                        ((if (((((4 as libc::c_ulong)) == (((2) as libc::c_ulong)))
                            as libc::c_int as libc::c_int) != 0)
                        {
                            (2 as libc::c_int)
                        } else {
                            (4 as libc::c_int)
                        }) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_octet_array(
        __this: *mut Self,
        mut x: *mut libc::c_uchar,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((1 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_short_array(
        __this: *mut Self,
        mut x: *mut libc::c_short,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((2 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((2 as libc::c_int)) as libc::c_ulong),
                        (((2 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_ushort_array(
        __this: *mut Self,
        mut x: *mut libc::c_ushort,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((2 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((2 as libc::c_int)) as libc::c_ulong),
                        (((2 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_long_array(
        __this: *mut Self,
        mut x: *mut libc::c_int,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((4 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_ulong_array(
        __this: *mut Self,
        mut x: *mut libc::c_uint,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((4 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_longlong_array(
        __this: *mut Self,
        mut x: *mut libc::c_long,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((8 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_ulonglong_array(
        __this: *mut Self,
        mut x: *mut libc::c_ulong,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((8 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_float_array(
        __this: *mut Self,
        mut x: *mut libc::c_float,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((4 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_double_array(
        __this: *mut Self,
        mut x: *mut libc::c_double,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((8 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_longdouble_array(
        __this: *mut Self,
        mut x: *mut crate::__f80::F80,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((16 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((16 as libc::c_int)) as libc::c_ulong),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_int8_array(
        __this: *mut Self,
        mut x: *mut libc::c_schar,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((1 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_uint8_array(
        __this: *mut Self,
        mut x: *mut libc::c_uchar,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((((length) as libc::c_uint))
                    .wrapping_mul(((1 as libc::c_int)) as libc::c_uint)
                    as libc::c_ulong))
                    > (((<ACE_InputCDR>::length((__this) as *const ACE_InputCDR))
                        as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).good_bit_ = false;
                    return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (__this) as *mut ACE_InputCDR,
                        ((x) as *mut libc::c_void),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_boolean(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((((((<ACE_InputCDR>::skip_octet((__this) as *mut ACE_InputCDR)
                    as libc::c_int) != 0) && (((*__this).good_bit_ as libc::c_int) != 0))
                    as libc::c_int) as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_char(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::skip_octet((__this) as *mut ACE_InputCDR),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_wchar(__this: *mut Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR10skip_wcharEv"]
            fn __ext(__this: *mut ACE_InputCDR) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR)
    }
    pub unsafe fn skip_octet(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_uchar = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_1(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x) as *mut libc::c_uchar,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_short(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::skip_ushort((__this) as *mut ACE_InputCDR),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_ushort(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_ushort = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_2(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x) as *mut libc::c_ushort,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_long(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::skip_ulong((__this) as *mut ACE_InputCDR),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_ulong(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_uint = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_4(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x) as *mut libc::c_uint,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_longlong(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::skip_ulonglong((__this) as *mut ACE_InputCDR),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_ulonglong(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_ulong = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_8(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x) as *mut libc::c_ulong,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_float(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::skip_ulong((__this) as *mut ACE_InputCDR),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_double(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::skip_ulonglong((__this) as *mut ACE_InputCDR),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_longdouble(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: crate::__f80::F80 = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_16(
                        (__this) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x) as *mut crate::__f80::F80,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn skip_fixed(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let mut i: libc::c_int = 0;
                    'for_0: loop {
                        if !((((((i as libc::c_int)) < (((16) as libc::c_int)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break;
                        }
                        'cont_0: loop {
                            {
                                {
                                    let mut x: libc::c_uchar = unsafe { ::core::mem::zeroed() };
                                    if (((!(((<ACE_InputCDR>::read_1(
                                        (__this) as *mut ACE_InputCDR,
                                        ::core::ptr::addr_of_mut!(x) as *mut libc::c_uchar,
                                    ) as libc::c_int) != 0)) as libc::c_int) as libc::c_int)
                                        != 0)
                                    {
                                        return crate::__cxx_std::__Truthy::__truthy(
                                            ((false) as bool),
                                        );
                                    }
                                    let mut low: libc::c_uint = (((((x) as libc::c_int))
                                        & ((0xf) as libc::c_int)) as libc::c_uint);
                                    if (((((((((low as libc::c_uint))
                                        == (((0xc) as libc::c_uint))) as libc::c_int as libc::c_int)
                                        != 0)
                                        || (((((low as libc::c_uint)) == (((0xd) as libc::c_uint)))
                                            as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                                        as libc::c_int) != 0)
                                    {
                                        return crate::__cxx_std::__Truthy::__truthy(
                                            ((true) as bool),
                                        );
                                    }
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                        {
                            let __lv = &mut (i);
                            *__lv = (*__lv).wrapping_add(1);
                            *__lv
                        };
                    }
                }
                return crate::__cxx_std::__Truthy::__truthy(((false) as bool));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* The next field must be a string, this method skips it. It is\n   * useful in parsing a TypeCode.\n   * @return @c false on failure and @c true on success."]
    pub unsafe fn skip_wstring(__this: *mut Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR12skip_wstringEv"]
            fn __ext(__this: *mut ACE_InputCDR) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR)
    }
    pub unsafe fn skip_string(__this: *mut Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR11skip_stringEv"]
            fn __ext(__this: *mut ACE_InputCDR) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR)
    }
    #[doc = "Skip @a n bytes in the CDR stream.\n  /**\n   * @return @c false on failure and @c true on success."]
    pub unsafe fn skip_bytes(__this: *mut Self, mut n: libc::c_ulong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR10skip_bytesEm"]
            fn __ext(__this: *mut ACE_InputCDR, n: libc::c_ulong) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, n)
    }
    ///returns @c false if a problem has been detected.
    pub unsafe fn good_bit(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    (((*__this).good_bit_) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* @return The start of the message block chain for this CDR\n   *         stream.\n   *\n   * @note In the current implementation the chain has length 1, but\n   *       we are planning to change that."]
    pub unsafe fn start(__this: *const Self) -> *const ACE_Message_Block {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of!(
                    (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                    ACE_Message_Block > ().cast_mut())
                ) as *const ACE_Message_Block;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Grow the internal buffer, reset @c rd_ptr to the first byte in\n   * the new buffer that is properly aligned, and set @c wr_ptr to @c\n   * rd_ptr @c + @c newsize"]
    pub unsafe fn grow(__this: *mut Self, mut newsize: libc::c_ulong) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR4growEm"]
            fn __ext(__this: *mut ACE_InputCDR, newsize: libc::c_ulong) -> libc::c_int;
        }
        __ext(__this as *mut ACE_InputCDR, newsize)
    }
    #[doc = "* After reading and partially parsing the contents the user can\n   * detect a change in the byte order, this method will let him/her\n   * change it."]
    pub unsafe fn reset_byte_order(__this: *mut Self, mut byte_order: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).do_byte_swap_ = ((((((byte_order as libc::c_int))
                    != (((1) as libc::c_int))) as libc::c_int) as libc::c_int) != 0);
            }
            ()
        }
    }
    /**Re-initialize the CDR stream, copying the contents of the chain
  /// of message_blocks starting from @a data.*/
    pub unsafe fn reset(
        __this: *mut Self,
        mut data: *const ACE_Message_Block,
        mut byte_order: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR5resetEPK17ACE_Message_Blocki"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                data: *const ACE_Message_Block,
                byte_order: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_InputCDR, data, byte_order)
    }
    ///Steal the contents from the current CDR.
    pub unsafe fn steal_contents(__this: *mut Self) -> *mut ACE_Message_Block {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR14steal_contentsEv"]
            fn __ext(__this: *mut ACE_InputCDR) -> *mut ACE_Message_Block;
        }
        __ext(__this as *mut ACE_InputCDR)
    }
    /**Steal the contents of @a cdr and make a shallow copy into this
  /// stream.*/
    pub unsafe fn steal_from(__this: *mut Self, mut cdr: *mut ACE_InputCDR) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR10steal_fromERS_"]
            fn __ext(__this: *mut ACE_InputCDR, cdr: *mut ACE_InputCDR);
        }
        __ext(__this as *mut ACE_InputCDR, cdr)
    }
    #[doc = "Exchange data blocks with the caller of this method. The read\n  /// and write pointers are also exchanged.\n  /**\n   * @note We now do only with the start_ message block."]
    pub unsafe fn exchange_data_blocks(__this: *mut Self, mut cdr: *mut ACE_InputCDR) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR20exchange_data_blocksERS_"]
            fn __ext(__this: *mut ACE_InputCDR, cdr: *mut ACE_InputCDR);
        }
        __ext(__this as *mut ACE_InputCDR, cdr)
    }
    #[doc = "Copy the data portion from the @a cdr to this cdr and return the\n  /// data content (ie. the ACE_Data_Block) from this CDR to the\n  /// caller.\n  /**\n   * @note The caller is responsible for managing the memory of the\n   *       returned ACE_Data_Block."]
    pub unsafe fn clone_from(
        __this: *mut Self,
        mut cdr: *mut ACE_InputCDR,
    ) -> *mut ACE_Data_Block {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR10clone_fromERS_"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                cdr: *mut ACE_InputCDR,
            ) -> *mut ACE_Data_Block;
        }
        __ext(__this as *mut ACE_InputCDR, cdr)
    }
    /**Re-initialize the CDR stream, forgetting about the old contents
  /// of the stream and allocating a new buffer (from the allocators).*/
    pub unsafe fn reset_contents(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR14reset_contentsEv"]
            fn __ext(__this: *mut ACE_InputCDR);
        }
        __ext(__this as *mut ACE_InputCDR)
    }
    ///Returns the current position for the @c rd_ptr.
    pub unsafe fn rd_ptr(__this: *mut Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Message_Block>::rd_ptr(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                        ACE_Message_Block > ().cast_mut())
                    )) as *const ACE_Message_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Returns the current position for the @c wr_ptr.
    pub unsafe fn wr_ptr(__this: *mut Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Message_Block>::wr_ptr(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                        ACE_Message_Block > ().cast_mut())
                    )) as *const ACE_Message_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return how many bytes are left in the stream.
    pub unsafe fn length(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Message_Block>::length(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                        ACE_Message_Block > ().cast_mut())
                    )) as *const ACE_Message_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Utility function to allow the user more flexibility.\n   * Skips up to the nearest @a alignment-byte boundary.\n   * Argument MUST be a power of 2.\n   *\n   * @return 0 on success and -1 on failure."]
    pub unsafe fn align_read_ptr(
        __this: *mut Self,
        mut alignment: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut buf: *mut libc::c_char = ACE_ptr_align_binary(
                    ((<ACE_InputCDR>::rd_ptr((__this) as *mut ACE_InputCDR))
                        as *const libc::c_char),
                    ((alignment) as libc::c_ulong),
                );
                if ((((((buf) as *const u8))
                    <= (((<ACE_InputCDR>::wr_ptr((__this) as *mut ACE_InputCDR))
                        as *const u8))) as libc::c_int as libc::c_int) != 0)
                {
                    <ACE_Message_Block>::rd_ptr_u4c9504a2c1e343b2(
                        (::core::ptr::addr_of_mut!(
                            (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                            ACE_Message_Block > ().cast_mut())
                        )) as *mut ACE_Message_Block,
                        buf,
                    );
                    return 0;
                }
                (*__this).good_bit_ = false;
                return (-((1) as libc::c_int));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**If @c true then this stream is writing in non-native byte order.
  /// This is only meaningful if ACE_ENABLE_SWAP_ON_WRITE is defined.*/
    pub unsafe fn do_byte_swap(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    (((*__this).do_byte_swap_) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**If @c do_byte_swap() returns @c false, this returns
  /// ACE_CDR_BYTE_ORDER else it returns !ACE_CDR_BYTE_ORDER.*/
    pub unsafe fn byte_order(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return if ((<ACE_InputCDR>::do_byte_swap((__this) as *const ACE_InputCDR)
                    as libc::c_int) != 0)
                {
                    (((!(((1) != 0)) as libc::c_int)) as libc::c_int)
                } else {
                    1
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Access the codeset translators. They can be nil!
    pub unsafe fn char_translator(
        __this: *const Self,
    ) -> *mut ACE_Char_Codeset_Translator {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).char_translator_)
                    as *mut ACE_Char_Codeset_Translator);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn wchar_translator(
        __this: *const Self,
    ) -> *mut ACE_WChar_Codeset_Translator {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).wchar_translator_)
                    as *mut ACE_WChar_Codeset_Translator);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the codeset translators.
    pub unsafe fn char_translator_u38956bfbb06a88fb(
        __this: *mut Self,
        mut ctran: *mut ACE_Char_Codeset_Translator,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).char_translator_ = ctran;
            }
            ()
        }
    }
    pub unsafe fn wchar_translator_u93bf06563e8951e1(
        __this: *mut Self,
        mut wctran: *mut ACE_WChar_Codeset_Translator,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).wchar_translator_ = wctran;
            }
            ()
        }
    }
    #[doc = "* Returns (in @a buf) the next position in the buffer aligned to\n   * @a size.  It advances the Message_Block @c rd_ptr past the data\n   * (i.e., @c buf @c + @c size).  Sets the good_bit to @c false and\n   * returns a -1 on failure."]
    pub unsafe fn adjust(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut buf: *mut *mut libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_InputCDR>::adjust_uf41c158b9318ca40(
                    (__this) as *mut ACE_InputCDR,
                    size,
                    size,
                    ::core::ptr::addr_of_mut!((* buf)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**As above, but now the size and alignment requirements may be
  /// different.*/
    pub unsafe fn adjust_uf41c158b9318ca40(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut buf: *mut *mut libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*buf) = ACE_ptr_align_binary(
                    ((<ACE_InputCDR>::rd_ptr((__this) as *mut ACE_InputCDR))
                        as *const libc::c_char),
                    ((align) as libc::c_ulong),
                );
                let mut end: *mut libc::c_char = ((((*buf))
                    .wrapping_offset((size) as isize)) as *mut libc::c_char);
                if ((((((end) as *const u8))
                    <= (((<ACE_InputCDR>::wr_ptr((__this) as *mut ACE_InputCDR))
                        as *const u8))) as libc::c_int as libc::c_int) != 0)
                {
                    <ACE_Message_Block>::rd_ptr_u4c9504a2c1e343b2(
                        (::core::ptr::addr_of_mut!(
                            (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                            ACE_Message_Block > ().cast_mut())
                        )) as *mut ACE_Message_Block,
                        ((end) as *mut libc::c_char),
                    );
                    return 0;
                }
                (*__this).good_bit_ = false;
                return (-((1) as libc::c_int));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the underlying GIOP version..
    pub unsafe fn set_version(
        __this: *mut Self,
        mut major: libc::c_uchar,
        mut minor: libc::c_uchar,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).major_version_ = major;
                (*__this).minor_version_ = minor;
            }
            ()
        }
    }
    ///Set the underlying GIOP version..
    pub unsafe fn get_version(
        __this: *mut Self,
        mut major: *mut libc::c_uchar,
        mut minor: *mut libc::c_uchar,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*major) = (*__this).major_version_;
                (*minor) = (*__this).minor_version_;
            }
            ()
        }
    }
    pub unsafe fn read_1(__this: *mut Self, mut x: *mut libc::c_uchar) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR6read_1EPh"]
            fn __ext(__this: *mut ACE_InputCDR, x: *mut libc::c_uchar) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_2(__this: *mut Self, mut x: *mut libc::c_ushort) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR6read_2EPt"]
            fn __ext(__this: *mut ACE_InputCDR, x: *mut libc::c_ushort) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_4(__this: *mut Self, mut x: *mut libc::c_uint) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR6read_4EPj"]
            fn __ext(__this: *mut ACE_InputCDR, x: *mut libc::c_uint) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_8(__this: *mut Self, mut x: *mut libc::c_ulong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR6read_8EPm"]
            fn __ext(__this: *mut ACE_InputCDR, x: *mut libc::c_ulong) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    pub unsafe fn read_16(__this: *mut Self, mut x: *mut crate::__f80::F80) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR7read_16EPe"]
            fn __ext(__this: *mut ACE_InputCDR, x: *mut crate::__f80::F80) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x)
    }
    #[doc = "* Read an array of @a length elements, each of @a size bytes and the\n   * start aligned at a multiple of @a align. The elements are assumed\n   * to be packed with the right alignment restrictions.  It is mostly\n   * designed for buffers of the basic types.\n   *\n   * This operation uses @c memcpy; as explained above it is expected\n   * that using assignment is faster that @c memcpy for one element,\n   * but for several elements @c memcpy should be more efficient, it\n   * could be interesting to find the break even point and optimize\n   * for that case, but that would be too platform dependent."]
    pub unsafe fn read_array(
        __this: *mut Self,
        mut x: *mut libc::c_void,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut length: libc::c_uint,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR10read_arrayEPvmmj"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                x: *mut libc::c_void,
                size: libc::c_ulong,
                align: libc::c_ulong,
                length: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x, size, align, length)
    }
    #[doc = "* On those occasions when the native codeset for wchar is smaller than\n   * the size of a wchar_t, such as using UTF-16 with a 4-byte wchar_t, a\n   * special form of reading the array is needed. Actually, this should be\n   * a default translator."]
    pub unsafe fn read_wchar_array_i(
        __this: *mut Self,
        mut x: *mut libc::wchar_t,
        mut length: libc::c_uint,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_InputCDR18read_wchar_array_iEPwj"]
            fn __ext(
                __this: *mut ACE_InputCDR,
                x: *mut libc::wchar_t,
                length: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *mut ACE_InputCDR, x, length)
    }
    ///Move the rd_ptr ahead by @a offset bytes.
    pub unsafe fn rd_ptr_u13a150ad10071bc7(
        __this: *mut Self,
        mut offset: libc::c_ulong,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Message_Block>::rd_ptr_u0d0bd23428e552e7(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                        ACE_Message_Block > ().cast_mut())
                    )) as *mut ACE_Message_Block,
                    offset,
                );
            }
            ()
        }
    }
    ///Points to the continuation field of the current message block.
    pub unsafe fn end(__this: *mut Self) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Message_Block>::end(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                        ACE_Message_Block > ().cast_mut())
                    )) as *const ACE_Message_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_OutputCDR {
    #[doc = "* Default constructor; allows one to set byte ordering, allocators, and\n   * tuning information.\n   *\n   * @param size        Causes constructor to preallocate @a size bytes; if\n   *                    @a size is 0 it allocates the default size.\n   *\n   * @param byte_order  The byte order that data will have within this\n   *                    object. Unless otherwise specified, the byte order\n   *                    will be the order native to the hardware this is\n   *                    executed on. To force the marshalled data to have\n   *                    a specific order, specify one of the values defined\n   *                    in ACE_CDR::Byte_Order.\n   *                    @note The @c ACE_ENABLE_SWAP_ON_WRITE config macro\n   *                    must be set for any local byte swapping to occur\n   *                    as data is inserted into an ACE_OutputCDR object."]
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Allocator,
        mut __a3: *mut ACE_Allocator,
        mut __a4: *mut ACE_Allocator,
        mut __a5: libc::c_ulong,
        mut __a6: libc::c_uchar,
        mut __a7: libc::c_uchar,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDRC1EmiP13ACE_AllocatorS1_S1_mhh"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                __a0: libc::c_ulong,
                __a1: libc::c_int,
                __a2: *mut ACE_Allocator,
                __a3: *mut ACE_Allocator,
                __a4: *mut ACE_Allocator,
                __a5: libc::c_ulong,
                __a6: libc::c_uchar,
                __a7: libc::c_uchar,
            );
        }
        __ext(
            __this as *mut ACE_OutputCDR,
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
            __a7,
        )
    }
    pub unsafe fn new(
        mut __a0: libc::c_ulong,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Allocator,
        mut __a3: *mut ACE_Allocator,
        mut __a4: *mut ACE_Allocator,
        mut __a5: libc::c_ulong,
        mut __a6: libc::c_uchar,
        mut __a7: libc::c_uchar,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
            __a7,
        );
        __obj
    }
    #[doc = "Build a CDR stream with an initial buffer, it will *not* remove\n  /// @a data, since it did not allocated it.  It's important to be careful\n  /// with the alignment of @a data.\n  /**\n   * Create an output stream from an arbitrary buffer, care must be\n   * exercised with alignment, because this constructor will align if\n   * needed.  In this case @a data will not point to the start of the\n   * output stream. @c begin()->rd_ptr() points to the start of the\n   * output stream.  See @c ACE_ptr_align_binary() to properly align a\n   * pointer and use ACE_CDR::MAX_ALIGNMENT for the correct alignment."]
    pub unsafe fn new_at_u2dca6b51a5c89238(
        __this: *mut Self,
        mut __a0: *mut libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: *mut ACE_Allocator,
        mut __a4: *mut ACE_Allocator,
        mut __a5: *mut ACE_Allocator,
        mut __a6: libc::c_ulong,
        mut __a7: libc::c_uchar,
        mut __a8: libc::c_uchar,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDRC1EPcmiP13ACE_AllocatorS2_S2_mhh"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                __a0: *mut libc::c_char,
                __a1: libc::c_ulong,
                __a2: libc::c_int,
                __a3: *mut ACE_Allocator,
                __a4: *mut ACE_Allocator,
                __a5: *mut ACE_Allocator,
                __a6: libc::c_ulong,
                __a7: libc::c_uchar,
                __a8: libc::c_uchar,
            );
        }
        __ext(
            __this as *mut ACE_OutputCDR,
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
            __a6,
            __a7,
            __a8,
        )
    }
    pub unsafe fn new_u2dca6b51a5c89238(
        mut __a0: *mut libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: *mut ACE_Allocator,
        mut __a4: *mut ACE_Allocator,
        mut __a5: *mut ACE_Allocator,
        mut __a6: libc::c_ulong,
        mut __a7: libc::c_uchar,
        mut __a8: libc::c_uchar,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u2dca6b51a5c89238(
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
        );
        __obj
    }
    #[doc = "Build a CDR stream with an initial data block, it will *not* remove\n  /// @a data_block, since it did not allocated it.  It's important to be\n  /// careful with the alignment of @a data_block.\n  /**\n   * Create an output stream from an arbitrary data block, care must be\n   * exercised with alignment, because this constructor will align if\n   * needed.  In this case @a data_block will not point to the\n   * start of the output stream. begin()->rd_ptr() points to the start\n   * off the output stream.  See ACE_ptr_align_binary() to properly align a\n   * pointer and use ACE_CDR::MAX_ALIGNMENT for the correct alignment."]
    pub unsafe fn new_at_u0a52cbba3022bdd1(
        __this: *mut Self,
        mut __a0: *mut ACE_Data_Block,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Allocator,
        mut __a3: libc::c_ulong,
        mut __a4: libc::c_uchar,
        mut __a5: libc::c_uchar,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDRC1EP14ACE_Data_BlockiP13ACE_Allocatormhh"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                __a0: *mut ACE_Data_Block,
                __a1: libc::c_int,
                __a2: *mut ACE_Allocator,
                __a3: libc::c_ulong,
                __a4: libc::c_uchar,
                __a5: libc::c_uchar,
            );
        }
        __ext(__this as *mut ACE_OutputCDR, __a0, __a1, __a2, __a3, __a4, __a5)
    }
    pub unsafe fn new_u0a52cbba3022bdd1(
        mut __a0: *mut ACE_Data_Block,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Allocator,
        mut __a3: libc::c_ulong,
        mut __a4: libc::c_uchar,
        mut __a5: libc::c_uchar,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u0a52cbba3022bdd1(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
            __a5,
        );
        __obj
    }
    /**Build a CDR stream with an initial Message_Block chain, it will
  /// *not* remove @a data, since it did not allocate it.*/
    pub unsafe fn new_at_u6c5baa43461f7301(
        __this: *mut Self,
        mut __a0: *mut ACE_Message_Block,
        mut __a1: libc::c_int,
        mut __a2: libc::c_ulong,
        mut __a3: libc::c_uchar,
        mut __a4: libc::c_uchar,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDRC1EP17ACE_Message_Blockimhh"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                __a0: *mut ACE_Message_Block,
                __a1: libc::c_int,
                __a2: libc::c_ulong,
                __a3: libc::c_uchar,
                __a4: libc::c_uchar,
            );
        }
        __ext(__this as *mut ACE_OutputCDR, __a0, __a1, __a2, __a3, __a4)
    }
    pub unsafe fn new_u6c5baa43461f7301(
        mut __a0: *mut ACE_Message_Block,
        mut __a1: libc::c_int,
        mut __a2: libc::c_ulong,
        mut __a3: libc::c_uchar,
        mut __a4: libc::c_uchar,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u6c5baa43461f7301(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
            __a4,
        );
        __obj
    }
    #[doc = "* @{ @name Write operations\n   * Return 0 on failure and 1 on success."]
    pub unsafe fn write_boolean(__this: *mut Self, mut x: bool) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_octet(
                        (__this) as *mut ACE_OutputCDR,
                        if ((x as libc::c_int) != 0) {
                            (1 as libc::c_uchar)
                        } else {
                            (0 as libc::c_uchar)
                        },
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_char(__this: *mut Self, mut x: libc::c_char) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).char_translator_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    let mut temp: libc::c_uchar = (x as libc::c_uchar);
                    return crate::__cxx_std::__Truthy::__truthy(
                        <ACE_OutputCDR>::write_1(
                            (__this) as *mut ACE_OutputCDR,
                            ((::core::ptr::addr_of_mut!(temp) as *mut libc::c_uchar)
                                as *const libc::c_uchar),
                        ),
                    );
                }
                return crate::__cxx_std::__Truthy::__truthy({
                    let __obj: *mut ACE_Char_Codeset_Translator = ((*__this)
                        .char_translator_) as *mut ACE_Char_Codeset_Translator;
                    let __vt: *const __Vtbl_ue1bfefa7873110ae = *(__obj
                        as *const *const __Vtbl_ue1bfefa7873110ae);
                    ((*__vt).vfn_u61069d17dd479073)(__obj, __this, x)
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_wchar(__this: *mut Self, mut x: libc::wchar_t) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR11write_wcharEw"]
            fn __ext(__this: *mut ACE_OutputCDR, x: libc::wchar_t) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x)
    }
    pub unsafe fn write_octet(__this: *mut Self, mut x: libc::c_uchar) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_1(
                        (__this) as *mut ACE_OutputCDR,
                        ((::core::ptr::addr_of_mut!(x) as *mut libc::c_uchar)
                            as *const libc::c_uchar),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_short(__this: *mut Self, mut x: libc::c_short) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: libc::c_ushort = (x as libc::c_ushort);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_2(
                        (__this) as *mut ACE_OutputCDR,
                        ((::core::ptr::addr_of_mut!(temp) as *mut libc::c_ushort)
                            as *const libc::c_ushort),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_ushort(__this: *mut Self, mut x: libc::c_ushort) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_2(
                        (__this) as *mut ACE_OutputCDR,
                        ((::core::ptr::addr_of_mut!(x) as *mut libc::c_ushort)
                            as *const libc::c_ushort),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_long(__this: *mut Self, mut x: libc::c_int) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: libc::c_uint = (x as libc::c_uint);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_4(
                        (__this) as *mut ACE_OutputCDR,
                        ((::core::ptr::addr_of_mut!(temp) as *mut libc::c_uint)
                            as *const libc::c_uint),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_ulong(__this: *mut Self, mut x: libc::c_uint) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_4(
                        (__this) as *mut ACE_OutputCDR,
                        ((::core::ptr::addr_of_mut!(x) as *mut libc::c_uint)
                            as *const libc::c_uint),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_longlong(__this: *mut Self, mut x: *const libc::c_long) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: *const libc::c_void = ((::core::ptr::addr_of!((* x))
                    as *const libc::c_long) as *const libc::c_void);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_8(
                        (__this) as *mut ACE_OutputCDR,
                        (temp as *const libc::c_ulong),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_ulonglong(
        __this: *mut Self,
        mut x: *const libc::c_ulong,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_8(
                        (__this) as *mut ACE_OutputCDR,
                        ::core::ptr::addr_of!((* x)) as *const libc::c_ulong,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_float(__this: *mut Self, mut x: libc::c_float) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: *const libc::c_void = ((::core::ptr::addr_of_mut!(x)
                    as *mut libc::c_float) as *const libc::c_void);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_4(
                        (__this) as *mut ACE_OutputCDR,
                        (temp as *const libc::c_uint),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_double(__this: *mut Self, mut x: *const libc::c_double) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: *const libc::c_void = ((::core::ptr::addr_of!((* x))
                    as *const libc::c_double) as *const libc::c_void);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_8(
                        (__this) as *mut ACE_OutputCDR,
                        (temp as *const libc::c_ulong),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_longdouble(
        __this: *mut Self,
        mut x: *const crate::__f80::F80,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_16(
                        (__this) as *mut ACE_OutputCDR,
                        ::core::ptr::addr_of!((* x)) as *const crate::__f80::F80,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_fixed(__this: *mut Self, mut x: *const Fixed) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut n: libc::c_int = unsafe { ::core::mem::zeroed() };
                let mut arr: *const libc::c_uchar = ((<Fixed>::to_octets(
                    (::core::ptr::addr_of!((* x))) as *const Fixed,
                    ::core::ptr::addr_of_mut!(n),
                )) as *const libc::c_uchar);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((arr) as *const libc::c_void),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        (n as libc::c_uint),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_int8(__this: *mut Self, mut x: libc::c_schar) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_1(
                        (__this) as *mut ACE_OutputCDR,
                        (((::core::ptr::addr_of_mut!(x) as *mut libc::c_schar
                            as *mut libc::c_uchar)) as *const libc::c_uchar),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_uint8(__this: *mut Self, mut x: libc::c_uchar) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_1(
                        (__this) as *mut ACE_OutputCDR,
                        (((::core::ptr::addr_of_mut!(x) as *mut libc::c_uchar
                            as *mut libc::c_uchar)) as *const libc::c_uchar),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///For string we offer methods that accept a precomputed length.
    pub unsafe fn write_string(__this: *mut Self, mut x: *const libc::c_char) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (!(x).is_null()) {
                    let mut len: libc::c_uint = (((ACE_OS::strlen_u07dd12a225364fa6(
                        ((x) as *const libc::c_char),
                    ) as libc::c_uint)) as libc::c_uint);
                    return crate::__cxx_std::__Truthy::__truthy(
                        <ACE_OutputCDR>::write_string_u17e4de4e62d5fde9(
                            (__this) as *mut ACE_OutputCDR,
                            ((len) as libc::c_uint),
                            x,
                        ),
                    );
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_string_u17e4de4e62d5fde9(
                        (__this) as *mut ACE_OutputCDR,
                        ((0) as libc::c_uint),
                        ((0) as *const libc::c_char),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_string_u17e4de4e62d5fde9(
        __this: *mut Self,
        mut len: libc::c_uint,
        mut x: *const libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR12write_stringEjPKc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                len: libc::c_uint,
                x: *const libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, len, x)
    }
    pub unsafe fn write_string_uf7ff1951453c6482(
        __this: *mut Self,
        mut x: *const ACE_String_Base_char_,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR12write_stringERK15ACE_String_BaseIcE"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: *const ACE_String_Base_char_,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x)
    }
    pub unsafe fn write_wstring(__this: *mut Self, mut x: *const libc::wchar_t) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (!(x).is_null()) {
                    let mut len: libc::c_uint = (((ACE_OS::strlen_u07b44aa22513a9ba(
                        ((x) as *const libc::wchar_t),
                    ) as libc::c_uint)) as libc::c_uint);
                    return crate::__cxx_std::__Truthy::__truthy(
                        <ACE_OutputCDR>::write_wstring_u5bf3306947ee070a(
                            (__this) as *mut ACE_OutputCDR,
                            ((len) as libc::c_uint),
                            x,
                        ),
                    );
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_wstring_u5bf3306947ee070a(
                        (__this) as *mut ACE_OutputCDR,
                        ((0) as libc::c_uint),
                        ((0) as *const libc::wchar_t),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_wstring_u5bf3306947ee070a(
        __this: *mut Self,
        mut length: libc::c_uint,
        mut x: *const libc::wchar_t,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR13write_wstringEjPKw"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                length: libc::c_uint,
                x: *const libc::wchar_t,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, length, x)
    }
    pub unsafe fn write_string_ud46834b86c3389c8(
        __this: *mut Self,
        mut x: *const crate::__cxx_std::String,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut len: libc::c_uint = ((((*x).size() as libc::c_uint))
                    as libc::c_uint);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_string_u17e4de4e62d5fde9(
                        (__this) as *mut ACE_OutputCDR,
                        ((len) as libc::c_uint),
                        ((if ((((((*x).empty()) as libc::c_int)) as libc::c_int) != 0) {
                            ((0) as *const libc::c_char)
                        } else {
                            (*x).c_str()
                        }) as *const libc::c_char),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_wstring_u3015186591267ebf(
        __this: *mut Self,
        mut x: *const crate::__cxx_std::WString,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut len: libc::c_uint = ((((*x).size() as libc::c_uint))
                    as libc::c_uint);
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_wstring_u5bf3306947ee070a(
                        (__this) as *mut ACE_OutputCDR,
                        ((len) as libc::c_uint),
                        ((if ((((((*x).empty()) as libc::c_int)) as libc::c_int) != 0) {
                            ((0) as *const libc::wchar_t)
                        } else {
                            (*x).c_str()
                        }) as *const libc::wchar_t),
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_boolean_array(
        __this: *mut Self,
        mut x: *const bool,
        mut length: libc::c_uint,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR19write_boolean_arrayEPKbj"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: *const bool,
                length: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, length)
    }
    pub unsafe fn write_char_array(
        __this: *mut Self,
        mut x: *const libc::c_char,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).char_translator_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    return crate::__cxx_std::__Truthy::__truthy(
                        <ACE_OutputCDR>::write_array(
                            (__this) as *mut ACE_OutputCDR,
                            ((x) as *const libc::c_void),
                            (((1 as libc::c_int)) as libc::c_ulong),
                            (((1 as libc::c_int)) as libc::c_ulong),
                            length,
                        ),
                    );
                }
                return crate::__cxx_std::__Truthy::__truthy({
                    let __obj: *mut ACE_Char_Codeset_Translator = ((*__this)
                        .char_translator_) as *mut ACE_Char_Codeset_Translator;
                    let __vt: *const __Vtbl_ue1bfefa7873110ae = *(__obj
                        as *const *const __Vtbl_ue1bfefa7873110ae);
                    ((*__vt).vfn_u21eb314fe72cbce4)(__obj, __this, x, length)
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_wchar_array(
        __this: *mut Self,
        mut x: *const libc::wchar_t,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (!((*__this).wchar_translator_).is_null()) {
                    return crate::__cxx_std::__Truthy::__truthy({
                        let __obj: *mut ACE_WChar_Codeset_Translator = ((*__this)
                            .wchar_translator_) as *mut ACE_WChar_Codeset_Translator;
                        let __vt: *const __Vtbl_u7f71c32ff7e5c9bb = *(__obj
                            as *const *const __Vtbl_u7f71c32ff7e5c9bb);
                        ((*__vt).vfn_u239c561f8bfd7f81)(__obj, __this, x, length)
                    });
                }
                if (((((ACE_OutputCDR_wchar_maxbytes_ as libc::c_ulong))
                    == (((0) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    ((*(__errno_location()))) = 13;
                    return crate::__cxx_std::__Truthy::__truthy(
                        ({
                            let __v = false;
                            (*__this).good_bit_ = __v;
                            __v
                        }),
                    );
                }
                if (((((ACE_OutputCDR_wchar_maxbytes_ as libc::c_ulong))
                    == (((4) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    return crate::__cxx_std::__Truthy::__truthy(
                        <ACE_OutputCDR>::write_array(
                            (__this) as *mut ACE_OutputCDR,
                            ((x) as *const libc::c_void),
                            ((4) as libc::c_ulong),
                            ((if (((((4 as libc::c_ulong)) == (((2) as libc::c_ulong)))
                                as libc::c_int as libc::c_int) != 0)
                            {
                                (2 as libc::c_int)
                            } else {
                                (4 as libc::c_int)
                            }) as libc::c_ulong),
                            length,
                        ),
                    );
                }
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_wchar_array_i(
                        (__this) as *mut ACE_OutputCDR,
                        x,
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_octet_array(
        __this: *mut Self,
        mut x: *const libc::c_uchar,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_short_array(
        __this: *mut Self,
        mut x: *const libc::c_short,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((2 as libc::c_int)) as libc::c_ulong),
                        (((2 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_ushort_array(
        __this: *mut Self,
        mut x: *const libc::c_ushort,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((2 as libc::c_int)) as libc::c_ulong),
                        (((2 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_long_array(
        __this: *mut Self,
        mut x: *const libc::c_int,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_ulong_array(
        __this: *mut Self,
        mut x: *const libc::c_uint,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_longlong_array(
        __this: *mut Self,
        mut x: *const libc::c_long,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_ulonglong_array(
        __this: *mut Self,
        mut x: *const libc::c_ulong,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_float_array(
        __this: *mut Self,
        mut x: *const libc::c_float,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        (((4 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_double_array(
        __this: *mut Self,
        mut x: *const libc::c_double,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_longdouble_array(
        __this: *mut Self,
        mut x: *const crate::__f80::F80,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((16 as libc::c_int)) as libc::c_ulong),
                        (((8 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_int8_array(
        __this: *mut Self,
        mut x: *const libc::c_schar,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_uint8_array(
        __this: *mut Self,
        mut x: *const libc::c_uchar,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_void),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        (((1 as libc::c_int)) as libc::c_ulong),
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Write an octet array contained inside a MB, this can be optimized
  /// to minimize copies.*/
    pub unsafe fn write_octet_array_mb(
        __this: *mut Self,
        mut mb: *const ACE_Message_Block,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR20write_octet_array_mbEPK17ACE_Message_Block"]
            fn __ext(__this: *mut ACE_OutputCDR, mb: *const ACE_Message_Block) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, mb)
    }
    #[doc = "* Write a placeholder into the stream. The placeholder's pointer\n   * is returned so it may later be passed as the @a loc argument to\n   * replace ().\n   * These methods align the stream's write pointer properly prior to\n   * writing the placeholder.\n   *\n   * @retval Pointer to the placeholder; 0 if there is not enough space\n   *         in the stream and memory could not be allocated."]
    pub unsafe fn write_long_placeholder(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR22write_long_placeholderEv"]
            fn __ext(__this: *mut ACE_OutputCDR) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_OutputCDR)
    }
    pub unsafe fn write_short_placeholder(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR23write_short_placeholderEv"]
            fn __ext(__this: *mut ACE_OutputCDR) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_OutputCDR)
    }
    pub unsafe fn write_boolean_placeholder(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR25write_boolean_placeholderEv"]
            fn __ext(__this: *mut ACE_OutputCDR) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_OutputCDR)
    }
    pub unsafe fn write_char_placeholder(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR22write_char_placeholderEv"]
            fn __ext(__this: *mut ACE_OutputCDR) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_OutputCDR)
    }
    pub unsafe fn write_longlong_placeholder(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR26write_longlong_placeholderEv"]
            fn __ext(__this: *mut ACE_OutputCDR) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_OutputCDR)
    }
    pub unsafe fn write_octet_placeholder(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR23write_octet_placeholderEv"]
            fn __ext(__this: *mut ACE_OutputCDR) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_OutputCDR)
    }
    pub unsafe fn write_float_placeholder(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR23write_float_placeholderEv"]
            fn __ext(__this: *mut ACE_OutputCDR) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_OutputCDR)
    }
    pub unsafe fn write_double_placeholder(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR24write_double_placeholderEv"]
            fn __ext(__this: *mut ACE_OutputCDR) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_OutputCDR)
    }
    #[doc = "* Writes a new value into a specific location. This is commonly\n   * used to update a prior \"placeholder\" location in the stream.\n   * The specified location is assumed to have proper CDR alignment for the\n   * type to insert. This requirement is satisfied by using one of the\n   * placeholder-writing methods to align the stream for the anticipated\n   * value and obtain the correct location.\n   * Treatment of @a x with respect to byte swapping is the same as for when\n   * any value is inserted.\n   *\n   * @param x   The value to insert into the specified location.\n   * @param loc The location at which to insert @a x. @a loc must be a valid\n   *            position within the stream's current set of message blocks.\n   *\n   * @sa write_long_placeholder(), write_short_placeholder ()"]
    pub unsafe fn replace(
        __this: *mut Self,
        mut x: libc::c_int,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEiPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_int,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_u42cfb31d8c1522f7(
        __this: *mut Self,
        mut x: libc::c_uint,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEjPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_uint,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_u4a8b47617d040a2d(
        __this: *mut Self,
        mut x: libc::c_short,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEsPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_short,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_ue062dffd7fa7b10d(
        __this: *mut Self,
        mut x: libc::c_ushort,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEtPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_ushort,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_ucdaaefdf446adf92(
        __this: *mut Self,
        mut x: bool,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEbPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: bool,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_u13c6ba095089889d(
        __this: *mut Self,
        mut x: libc::c_char,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEcPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_char,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_ue325242580a784d8(
        __this: *mut Self,
        mut x: libc::c_long,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceElPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_long,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_u228c3cbf0dda41b8(
        __this: *mut Self,
        mut x: libc::c_ulong,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEmPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_ulong,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_ua99dd2a5532c55fd(
        __this: *mut Self,
        mut x: libc::c_uchar,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEhPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_uchar,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_u9e7a4c8a23b647ee(
        __this: *mut Self,
        mut x: libc::c_float,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEfPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_float,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn replace_ufdc9a82838b662e0(
        __this: *mut Self,
        mut x: libc::c_double,
        mut loc: *mut libc::c_char,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7replaceEdPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: libc::c_double,
                loc: *mut libc::c_char,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, loc)
    }
    pub unsafe fn append_boolean(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: bool = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_boolean(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_boolean(
                            (__this) as *mut ACE_OutputCDR,
                            x,
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_char(__this: *mut Self, mut stream: *mut ACE_InputCDR) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_char = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_char(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_char((__this) as *mut ACE_OutputCDR, x))
                            as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_wchar(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::wchar_t = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_wchar(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_wchar(
                            (__this) as *mut ACE_OutputCDR,
                            x,
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_octet(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_uchar = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_octet(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_octet(
                            (__this) as *mut ACE_OutputCDR,
                            x,
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_short(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_short = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_short(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_short(
                            (__this) as *mut ACE_OutputCDR,
                            x,
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_ushort(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_ushort = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_ushort(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_ushort(
                            (__this) as *mut ACE_OutputCDR,
                            x,
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_long(__this: *mut Self, mut stream: *mut ACE_InputCDR) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_int = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_long(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_long((__this) as *mut ACE_OutputCDR, x))
                            as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_ulong(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_uint = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_ulong(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_ulong(
                            (__this) as *mut ACE_OutputCDR,
                            x,
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_longlong(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_long = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_longlong(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_longlong(
                            (__this) as *mut ACE_OutputCDR,
                            ::core::ptr::addr_of!(x),
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_ulonglong(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_ulong = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_ulonglong(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_ulonglong(
                            (__this) as *mut ACE_OutputCDR,
                            ::core::ptr::addr_of!(x),
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_float(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_float = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_float(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_float(
                            (__this) as *mut ACE_OutputCDR,
                            x,
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_double(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: libc::c_double = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_double(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_double(
                            (__this) as *mut ACE_OutputCDR,
                            ::core::ptr::addr_of!(x),
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_longdouble(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: crate::__f80::F80 = unsafe { ::core::mem::zeroed() };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_longdouble(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_longdouble(
                            (__this) as *mut ACE_OutputCDR,
                            ::core::ptr::addr_of!(x),
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_fixed(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: Fixed = unsafe {
                    ::core::mem::MaybeUninit::<Fixed>::zeroed().assume_init()
                };
                return crate::__cxx_std::__Truthy::__truthy(
                    ((if ((<ACE_InputCDR>::read_fixed(
                        (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                        ::core::ptr::addr_of_mut!(x),
                    ) as libc::c_int) != 0)
                    {
                        ((<ACE_OutputCDR>::write_fixed(
                            (__this) as *mut ACE_OutputCDR,
                            ::core::ptr::addr_of!(x),
                        )) as bool)
                    } else {
                        false
                    }) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_wstring(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: *mut libc::wchar_t = ((0) as *mut libc::wchar_t);
                let mut flag: bool = (((if ((<ACE_InputCDR>::read_wstring(
                    (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                    ::core::ptr::addr_of_mut!(x),
                ) as libc::c_int) != 0)
                {
                    ((<ACE_OutputCDR>::write_wstring(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::wchar_t),
                    )) as bool)
                } else {
                    false
                })) as bool);
                {
                    let __data = x as *mut libc::wchar_t;
                    if !__data.is_null() {
                        ::libc::free(__data as *mut libc::c_void);
                    }
                };
                return crate::__cxx_std::__Truthy::__truthy(((flag) as bool));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn append_string(
        __this: *mut Self,
        mut stream: *mut ACE_InputCDR,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut x: *mut libc::c_char = ((0) as *mut libc::c_char);
                let mut flag: bool = (((if ((<ACE_InputCDR>::read_string(
                    (::core::ptr::addr_of_mut!((* stream))) as *mut ACE_InputCDR,
                    ::core::ptr::addr_of_mut!(x),
                ) as libc::c_int) != 0)
                {
                    ((<ACE_OutputCDR>::write_string(
                        (__this) as *mut ACE_OutputCDR,
                        ((x) as *const libc::c_char),
                    )) as bool)
                } else {
                    false
                })) as bool);
                {
                    let __data = x as *mut libc::c_char;
                    if !__data.is_null() {
                        ::libc::free(__data as *mut libc::c_void);
                    }
                };
                return crate::__cxx_std::__Truthy::__truthy(((flag) as bool));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "Returns @c false if an error has occurred.\n  /**\n   * @note The only expected error is to run out of memory."]
    pub unsafe fn good_bit(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    (((*__this).good_bit_) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Reuse the CDR stream to write on the old buffer.
    pub unsafe fn reset(__this: *mut Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).current_ = ::core::ptr::addr_of_mut!(
                    (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                    ACE_Message_Block > ().cast_mut())
                ) as *mut ACE_Message_Block;
                (*__this).current_is_writable_ = true;
                <ACE_CDR>::mb_align(
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                        ACE_Message_Block > ().cast_mut())
                    ) as *mut ACE_Message_Block,
                );
                (*__this).current_alignment_ = ((0) as libc::c_ulong);
                let mut cont: *mut ACE_Message_Block = ((<ACE_Message_Block>::cont(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                        ACE_Message_Block > ().cast_mut())
                    )) as *const ACE_Message_Block,
                )) as *mut ACE_Message_Block);
                if (!(cont).is_null()) {
                    <ACE_Message_Block>::release_ubc51e64ee0ea988c(
                        ((cont) as *mut ACE_Message_Block),
                    );
                    <ACE_Message_Block>::cont_u9515391441f35afa(
                        (::core::ptr::addr_of_mut!(
                            (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                            ACE_Message_Block > ().cast_mut())
                        )) as *mut ACE_Message_Block,
                        ((0) as *mut ACE_Message_Block),
                    );
                }
            }
            ()
        }
    }
    ///Add the length of each message block in the chain.
    pub unsafe fn total_length(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_CDR>::total_length(
                    <ACE_OutputCDR>::begin((__this) as *const ACE_OutputCDR),
                    <ACE_OutputCDR>::end((__this) as *const ACE_OutputCDR),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Return the start of the message block chain for this CDR stream.\n   * @note The complete CDR stream is represented by a chain of\n   * message blocks."]
    pub unsafe fn begin(__this: *const Self) -> *const ACE_Message_Block {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of!(
                    (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                    ACE_Message_Block > ().cast_mut())
                ) as *const ACE_Message_Block;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the last message in the chain that is is use.
    pub unsafe fn end(__this: *const Self) -> *const ACE_Message_Block {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (<ACE_Message_Block>::cont(
                    ((*__this).current_) as *const ACE_Message_Block,
                )) as *const ACE_Message_Block;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the <current_> message block in chain.
    pub unsafe fn current(__this: *const Self) -> *const ACE_Message_Block {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((*__this).current_) as *const ACE_Message_Block;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "Replace the message block chain with a single message block.\n  /**\n   * Upon successful completion, there will be a single message block\n   * containing the data from the complete message block chain.\n   *\n   * @note The only expected error is to run out of memory."]
    pub unsafe fn consolidate(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR11consolidateEv"]
            fn __ext(__this: *mut ACE_OutputCDR) -> libc::c_int;
        }
        __ext(__this as *mut ACE_OutputCDR)
    }
    #[doc = "* Access the underlying buffer (read only).  @note This\n   * method only returns a pointer to the first block in the\n   * chain."]
    pub unsafe fn buffer(__this: *const Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((<ACE_Message_Block>::rd_ptr(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                        ACE_Message_Block > ().cast_mut())
                    )) as *const ACE_Message_Block,
                )) as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Return the size of first message block in the block chain. @note This\n   * method only returns information about the first block in the\n   * chain."]
    pub unsafe fn length(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Message_Block>::length(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).start_) .cast:: <
                        ACE_Message_Block > ().cast_mut())
                    )) as *const ACE_Message_Block,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Utility function to allow the user more flexibility.\n   * Pads the stream up to the nearest @a alignment byte boundary.\n   * Argument MUST be a power of 2.\n   * Returns 0 on success and -1 on failure."]
    pub unsafe fn align_write_ptr(
        __this: *mut Self,
        mut alignment: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut dummy: *mut libc::c_char = unsafe { ::core::mem::zeroed() };
                return <ACE_OutputCDR>::adjust_ubc71eaf6ad051f92(
                    (__this) as *mut ACE_OutputCDR,
                    ((0) as libc::c_ulong),
                    alignment,
                    ::core::ptr::addr_of_mut!(dummy),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Access the codeset translators. They can be null!
    pub unsafe fn char_translator(
        __this: *const Self,
    ) -> *mut ACE_Char_Codeset_Translator {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).char_translator_)
                    as *mut ACE_Char_Codeset_Translator);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn wchar_translator(
        __this: *const Self,
    ) -> *mut ACE_WChar_Codeset_Translator {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).wchar_translator_)
                    as *mut ACE_WChar_Codeset_Translator);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the char codeset translator.
    pub unsafe fn char_translator_uf4bae95772f0d161(
        __this: *mut Self,
        mut ctran: *mut ACE_Char_Codeset_Translator,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).char_translator_ = ctran;
            }
            ()
        }
    }
    ///Set the wchar codeset translator.
    pub unsafe fn wchar_translator_u49f90b51093a6777(
        __this: *mut Self,
        mut wctran: *mut ACE_WChar_Codeset_Translator,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).wchar_translator_ = wctran;
            }
            ()
        }
    }
    /**set the global size of serialized wchars. This may be different
  /// than the size of a wchar_t.*/
    pub unsafe fn wchar_maxbytes(mut max_bytes: libc::c_ulong) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR14wchar_maxbytesEm"]
            fn __ext(max_bytes: libc::c_ulong);
        }
        __ext(max_bytes)
    }
    ///access the serialized size of wchars.
    pub unsafe fn wchar_maxbytes_u47850143c0df4111() -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR14wchar_maxbytesEv"]
            fn __ext() -> libc::c_ulong;
        }
        __ext()
    }
    #[doc = "* Return alignment of the wr_ptr(), with respect to the start of\n   * the CDR stream.  This is not the same as the alignment of\n   * current->wr_ptr()!"]
    pub unsafe fn current_alignment(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).current_alignment_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn current_alignment_uffaef2f9fd22bedd(
        __this: *mut Self,
        mut current_alignment: libc::c_ulong,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).current_alignment_ = current_alignment;
            }
            ()
        }
    }
    #[doc = "* Returns (in @a buf) the next position in the buffer aligned to\n   * @a size, it advances the Message_Block wr_ptr past the data\n   * (i.e., @a buf + @a size). If necessary it grows the Message_Block\n   * buffer.  Sets the good_bit to false and returns a -1 on failure."]
    pub unsafe fn adjust(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut buf: *mut *mut libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_OutputCDR>::adjust_ubc71eaf6ad051f92(
                    (__this) as *mut ACE_OutputCDR,
                    size,
                    size,
                    ::core::ptr::addr_of_mut!((* buf)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**As above, but now the size and alignment requirements may be
  /// different.*/
    pub unsafe fn adjust_ubc71eaf6ad051f92(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut buf: *mut *mut libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((!((((*__this).current_is_writable_ as libc::c_int) != 0))
                    as libc::c_int) as libc::c_int) != 0)
                {
                    return <ACE_OutputCDR>::grow_and_adjust(
                        (__this) as *mut ACE_OutputCDR,
                        size,
                        align,
                        ::core::ptr::addr_of_mut!((* buf)),
                    );
                }
                let mut offset: libc::c_ulong = (((((ACE_align_binary(
                    (((*__this).current_alignment_) as libc::c_ulong),
                    ((align) as libc::c_ulong),
                )) as libc::c_ulong))
                    .wrapping_sub(((*__this).current_alignment_) as libc::c_ulong))
                    as libc::c_ulong);
                (*buf) = (<ACE_Message_Block>::wr_ptr(
                    ((*__this).current_) as *const ACE_Message_Block,
                ))
                    .wrapping_offset((offset) as isize);
                let mut end: *mut libc::c_char = ((((*buf))
                    .wrapping_offset((size) as isize)) as *mut libc::c_char);
                if ((((((((((end) as *const u8))
                    <= (((<ACE_Message_Block>::end(
                        ((*__this).current_) as *const ACE_Message_Block,
                    )) as *const u8))) as libc::c_int as libc::c_int) != 0)
                    && ((((((end) as *const u8)) >= ((((*buf)) as *const u8)))
                        as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    {
                        let __lv = ::core::ptr::addr_of_mut!(
                            (* __this).current_alignment_
                        );
                        unsafe {
                            *__lv = ((((*__lv)) as libc::c_ulong))
                                .wrapping_add(
                                    ((((offset) as libc::c_ulong))
                                        .wrapping_add((size) as libc::c_ulong)) as libc::c_ulong,
                                );
                            *__lv
                        }
                    };
                    <ACE_Message_Block>::wr_ptr_u16d0e11bb2cda475(
                        ((*__this).current_) as *mut ACE_Message_Block,
                        ((end) as *mut libc::c_char),
                    );
                    return 0;
                }
                return <ACE_OutputCDR>::grow_and_adjust(
                    (__this) as *mut ACE_OutputCDR,
                    size,
                    align,
                    ::core::ptr::addr_of_mut!((* buf)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Returns true if this stream is writing in non-native byte order
  /// and false otherwise. For example, it would be true if either
  /// ACE_ENABLE_SWAP_ON_WRITE is defined or a specific byte order was
  /// specified for this stream.*/
    pub unsafe fn do_byte_swap(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    (((*__this).do_byte_swap_) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Returns the byte order this stream is marshaling data in. Will be one
  /// of the values in ACE_CDR::Byte_Order.*/
    pub unsafe fn byte_order(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((<ACE_OutputCDR>::do_byte_swap((__this) as *const ACE_OutputCDR)
                    as libc::c_int) != 0)
                {
                    return (((!(((1) != 0)) as libc::c_int)) as libc::c_int);
                } else {
                    return 1;
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**For use by a gateway, which creates the output stream for the
  /// reply to the client in its native byte order, but which must
  /// send the reply in the byte order of the target's reply to the
  /// gateway.*/
    pub unsafe fn reset_byte_order(__this: *mut Self, mut byte_order: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).do_byte_swap_ = ((((((byte_order as libc::c_int))
                    != (((1) as libc::c_int))) as libc::c_int) as libc::c_int) != 0);
            }
            ()
        }
    }
    ///Set GIOP version info
    pub unsafe fn set_version(
        __this: *mut Self,
        mut major: libc::c_uchar,
        mut minor: libc::c_uchar,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).major_version_ = major;
                (*__this).minor_version_ = minor;
            }
            ()
        }
    }
    ///Set the underlying GIOP version..
    pub unsafe fn get_version(
        __this: *mut Self,
        mut major: *mut libc::c_uchar,
        mut minor: *mut libc::c_uchar,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*major) = (*__this).major_version_;
                (*minor) = (*__this).minor_version_;
            }
            ()
        }
    }
    pub unsafe fn find(
        __this: *mut Self,
        mut loc: *mut libc::c_char,
    ) -> *mut ACE_Message_Block {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR4findEPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                loc: *mut libc::c_char,
            ) -> *mut ACE_Message_Block;
        }
        __ext(__this as *mut ACE_OutputCDR, loc)
    }
    ///disallow copying...
    pub unsafe fn new_at_u09a3fe3b66a1b55e(
        __this: *mut Self,
        mut __a0: *const ACE_OutputCDR,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDRC1ERKS_"]
            fn __ext(__this: *mut ACE_OutputCDR, __a0: *const ACE_OutputCDR);
        }
        __ext(__this as *mut ACE_OutputCDR, __a0)
    }
    pub unsafe fn new_u09a3fe3b66a1b55e(mut __a0: *const ACE_OutputCDR) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u09a3fe3b66a1b55e(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: *const ACE_OutputCDR,
    ) -> *mut ACE_OutputCDR {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDRaSERKS_"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                rhs: *const ACE_OutputCDR,
            ) -> *mut ACE_OutputCDR;
        }
        __ext(__this as *mut ACE_OutputCDR, rhs)
    }
    pub unsafe fn write_1(__this: *mut Self, mut x: *const libc::c_uchar) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7write_1EPKh"]
            fn __ext(__this: *mut ACE_OutputCDR, x: *const libc::c_uchar) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x)
    }
    pub unsafe fn write_2(__this: *mut Self, mut x: *const libc::c_ushort) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7write_2EPKt"]
            fn __ext(__this: *mut ACE_OutputCDR, x: *const libc::c_ushort) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x)
    }
    pub unsafe fn write_4(__this: *mut Self, mut x: *const libc::c_uint) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7write_4EPKj"]
            fn __ext(__this: *mut ACE_OutputCDR, x: *const libc::c_uint) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x)
    }
    pub unsafe fn write_8(__this: *mut Self, mut x: *const libc::c_ulong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR7write_8EPKm"]
            fn __ext(__this: *mut ACE_OutputCDR, x: *const libc::c_ulong) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x)
    }
    pub unsafe fn write_16(__this: *mut Self, mut x: *const crate::__f80::F80) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR8write_16EPKe"]
            fn __ext(__this: *mut ACE_OutputCDR, x: *const crate::__f80::F80) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x)
    }
    #[doc = "* write an array of @a length elements, each of @a size bytes and the\n   * start aligned at a multiple of @a align. The elements are assumed\n   * to be packed with the right alignment restrictions.  It is mostly\n   * designed for buffers of the basic types.\n   *\n   * This operation uses @c memcpy; as explained above it is expected\n   * that using assignment is faster that @c memcpy for one element,\n   * but for several elements @c memcpy should be more efficient, it\n   * could be interesting to find the break even point and optimize\n   * for that case, but that would be too platform dependent."]
    pub unsafe fn write_array(
        __this: *mut Self,
        mut x: *const libc::c_void,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut length: libc::c_uint,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR11write_arrayEPKvmmj"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: *const libc::c_void,
                size: libc::c_ulong,
                align: libc::c_ulong,
                length: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, size, align, length)
    }
    pub unsafe fn write_wchar_array_i(
        __this: *mut Self,
        mut x: *const libc::wchar_t,
        mut length: libc::c_uint,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR19write_wchar_array_iEPKwj"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                x: *const libc::wchar_t,
                length: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *mut ACE_OutputCDR, x, length)
    }
    #[doc = "* Grow the CDR stream. When it returns @a buf contains a pointer to\n   * memory in the CDR stream, with at least @a size bytes ahead of it\n   * and aligned to an @a align boundary. It moved the <wr_ptr> to <buf\n   * + size>."]
    pub unsafe fn grow_and_adjust(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut buf: *mut *mut libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_OutputCDR15grow_and_adjustEmmRPc"]
            fn __ext(
                __this: *mut ACE_OutputCDR,
                size: libc::c_ulong,
                align: libc::c_ulong,
                buf: *mut *mut libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_OutputCDR, size, align, buf)
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
        mut length: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_CDR12swap_2_arrayEPKcPcm"]
            fn __ext(
                orig: *const libc::c_char,
                target: *mut libc::c_char,
                length: libc::c_ulong,
            );
        }
        __ext(orig, target, length)
    }
    pub unsafe fn swap_4_array(
        mut orig: *const libc::c_char,
        mut target: *mut libc::c_char,
        mut length: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_CDR12swap_4_arrayEPKcPcm"]
            fn __ext(
                orig: *const libc::c_char,
                target: *mut libc::c_char,
                length: libc::c_ulong,
            );
        }
        __ext(orig, target, length)
    }
    pub unsafe fn swap_8_array(
        mut orig: *const libc::c_char,
        mut target: *mut libc::c_char,
        mut length: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_CDR12swap_8_arrayEPKcPcm"]
            fn __ext(
                orig: *const libc::c_char,
                target: *mut libc::c_char,
                length: libc::c_ulong,
            );
        }
        __ext(orig, target, length)
    }
    pub unsafe fn swap_16_array(
        mut orig: *const libc::c_char,
        mut target: *mut libc::c_char,
        mut length: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_CDR13swap_16_arrayEPKcPcm"]
            fn __ext(
                orig: *const libc::c_char,
                target: *mut libc::c_char,
                length: libc::c_ulong,
            );
        }
        __ext(orig, target, length)
    }
    /**Align the message block to ACE_CDR::MAX_ALIGNMENT,
  /// set by the CORBA spec at 8 bytes.*/
    pub unsafe fn mb_align(mut mb: *mut ACE_Message_Block) {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_CDR8mb_alignEP17ACE_Message_Block"]
            fn __ext(mb: *mut ACE_Message_Block);
        }
        __ext(mb)
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
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_CDR4growEP17ACE_Message_Blockm"]
            fn __ext(mb: *mut ACE_Message_Block, minsize: libc::c_ulong) -> libc::c_int;
        }
        __ext(mb, minsize)
    }
    #[doc = "* Copy a message block chain into a single message block,\n   * preserving the alignment of the first message block of the\n   * original stream, not the following message blocks.\n   * @retval -1 Failure\n   * @retval 0 Success."]
    pub unsafe fn consolidate(
        mut dst: *mut ACE_Message_Block,
        mut src: *const ACE_Message_Block,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_CDR11consolidateEP17ACE_Message_BlockPKS0_"]
            fn __ext(
                dst: *mut ACE_Message_Block,
                src: *const ACE_Message_Block,
            ) -> libc::c_int;
        }
        __ext(dst, src)
    }
    pub unsafe fn total_length(
        mut begin: *const ACE_Message_Block,
        mut end: *const ACE_Message_Block,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_CDR12total_lengthEPK17ACE_Message_BlockS2_"]
            fn __ext(
                begin: *const ACE_Message_Block,
                end: *const ACE_Message_Block,
            ) -> libc::c_ulong;
        }
        __ext(begin, end)
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
impl ACE_Char_Codeset_Translator {
    /**Read a std::string from the stream, including the length, converting
  /// the characters from the stream codeset to the native codeset
  /// (provide non-optimized default implementation)*/
    pub unsafe fn read_string_u149f56048e9130dd(
        __this: *mut Self,
        mut _anon_0: *mut ACE_InputCDR,
        mut _anon_1: *mut crate::__cxx_std::String,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN27ACE_Char_Codeset_Translator11read_stringER12ACE_InputCDRRNSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEE"]
            fn __ext(
                __this: *mut ACE_Char_Codeset_Translator,
                _anon_0: *mut ACE_InputCDR,
                _anon_1: *mut crate::__cxx_std::String,
            ) -> bool;
        }
        __ext(__this as *mut ACE_Char_Codeset_Translator, _anon_0, _anon_1)
    }
    /**Children have access to low-level routines because they cannot
  /// use read_char or something similar (it would recurse).*/
    pub unsafe fn read_1(
        __this: *mut Self,
        mut input: *mut ACE_InputCDR,
        mut x: *mut libc::c_uchar,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_1(
                        (::core::ptr::addr_of_mut!((* input))) as *mut ACE_InputCDR,
                        x,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_1(
        __this: *mut Self,
        mut output: *mut ACE_OutputCDR,
        mut x: *const libc::c_uchar,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_1(
                        (::core::ptr::addr_of_mut!((* output))) as *mut ACE_OutputCDR,
                        x,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Efficiently read @a length elements of size @a size each from
  /// @a input into @a x; the data must be aligned to @a align.*/
    pub unsafe fn read_array(
        __this: *mut Self,
        mut r#in: *mut ACE_InputCDR,
        mut x: *mut libc::c_void,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (::core::ptr::addr_of_mut!((* r#in))) as *mut ACE_InputCDR,
                        x,
                        size,
                        align,
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Efficiently write @a length elements of size @a size from <x> into\n   * <output>. Before inserting the elements enough padding is added\n   * to ensure that the elements will be aligned to <align> in the\n   * stream."]
    pub unsafe fn write_array(
        __this: *mut Self,
        mut out: *mut ACE_OutputCDR,
        mut x: *const libc::c_void,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (::core::ptr::addr_of_mut!((* out))) as *mut ACE_OutputCDR,
                        x,
                        size,
                        align,
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Exposes the stream implementation of <adjust>, this is useful in\n   * many cases to minimize memory allocations during marshaling.\n   * On success @a buf will contain a contiguous area in the CDR stream\n   * that can hold @a size bytes aligned to @a align.\n   * Results"]
    pub unsafe fn adjust(
        __this: *mut Self,
        mut out: *mut ACE_OutputCDR,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut buf: *mut *mut libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_OutputCDR>::adjust_ubc71eaf6ad051f92(
                    (::core::ptr::addr_of_mut!((* out))) as *mut ACE_OutputCDR,
                    size,
                    align,
                    ::core::ptr::addr_of_mut!((* buf)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Used by derived classes to set errors in the CDR stream.
    pub unsafe fn good_bit(
        __this: *mut Self,
        mut out: *mut ACE_OutputCDR,
        mut bit: bool,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*out).good_bit_ = bit;
            }
            ()
        }
    }
    ///Obtain the CDR Stream's major & minor version values.
    pub unsafe fn major_version(
        __this: *mut Self,
        mut input: *mut ACE_InputCDR,
    ) -> libc::c_uchar {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*input).major_version_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn minor_version(
        __this: *mut Self,
        mut input: *mut ACE_InputCDR,
    ) -> libc::c_uchar {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*input).minor_version_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn major_version_ubdba4af94797da83(
        __this: *mut Self,
        mut output: *mut ACE_OutputCDR,
    ) -> libc::c_uchar {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*output).major_version_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn minor_version_ud57f5bfc473e14a7(
        __this: *mut Self,
        mut output: *mut ACE_OutputCDR,
    ) -> libc::c_uchar {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*output).minor_version_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_WChar_Codeset_Translator {
    /**Read a std::wstring from the stream, including the length, converting
  /// the characters from the stream codeset to the native codeset
  /// (provide non-optimized default implementation)*/
    pub unsafe fn read_wstring_u94b79f713f9e771a(
        __this: *mut Self,
        mut _anon_0: *mut ACE_InputCDR,
        mut _anon_1: *mut crate::__cxx_std::WString,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN28ACE_WChar_Codeset_Translator12read_wstringER12ACE_InputCDRRNSt7__cxx1112basic_stringIwSt11char_traitsIwESaIwEEE"]
            fn __ext(
                __this: *mut ACE_WChar_Codeset_Translator,
                _anon_0: *mut ACE_InputCDR,
                _anon_1: *mut crate::__cxx_std::WString,
            ) -> bool;
        }
        __ext(__this as *mut ACE_WChar_Codeset_Translator, _anon_0, _anon_1)
    }
    /**Children have access to low-level routines because they cannot
  /// use read_char or something similar (it would recurse).*/
    pub unsafe fn read_1(
        __this: *mut Self,
        mut input: *mut ACE_InputCDR,
        mut x: *mut libc::c_uchar,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_1(
                        (::core::ptr::addr_of_mut!((* input))) as *mut ACE_InputCDR,
                        x,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_2(
        __this: *mut Self,
        mut input: *mut ACE_InputCDR,
        mut x: *mut libc::c_ushort,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_2(
                        (::core::ptr::addr_of_mut!((* input))) as *mut ACE_InputCDR,
                        x,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn read_4(
        __this: *mut Self,
        mut input: *mut ACE_InputCDR,
        mut x: *mut libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_4(
                        (::core::ptr::addr_of_mut!((* input))) as *mut ACE_InputCDR,
                        x,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_1(
        __this: *mut Self,
        mut output: *mut ACE_OutputCDR,
        mut x: *const libc::c_uchar,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_1(
                        (::core::ptr::addr_of_mut!((* output))) as *mut ACE_OutputCDR,
                        x,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_2(
        __this: *mut Self,
        mut output: *mut ACE_OutputCDR,
        mut x: *const libc::c_ushort,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_2(
                        (::core::ptr::addr_of_mut!((* output))) as *mut ACE_OutputCDR,
                        x,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn write_4(
        __this: *mut Self,
        mut output: *mut ACE_OutputCDR,
        mut x: *const libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_4(
                        (::core::ptr::addr_of_mut!((* output))) as *mut ACE_OutputCDR,
                        x,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Efficiently read @a length elements of size @a size each from
  /// @a input into @a x; the data must be aligned to @a align.*/
    pub unsafe fn read_array(
        __this: *mut Self,
        mut r#in: *mut ACE_InputCDR,
        mut x: *mut libc::c_void,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_InputCDR>::read_array(
                        (::core::ptr::addr_of_mut!((* r#in))) as *mut ACE_InputCDR,
                        x,
                        size,
                        align,
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Efficiently write @a length elements of size @a size from @a x into\n   * @a output. Before inserting the elements enough padding is added\n   * to ensure that the elements will be aligned to @a align in the\n   * stream."]
    pub unsafe fn write_array(
        __this: *mut Self,
        mut out: *mut ACE_OutputCDR,
        mut x: *const libc::c_void,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut length: libc::c_uint,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    <ACE_OutputCDR>::write_array(
                        (::core::ptr::addr_of_mut!((* out))) as *mut ACE_OutputCDR,
                        x,
                        size,
                        align,
                        length,
                    ),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Exposes the stream implementation of @a adjust, this is useful in\n   * many cases to minimize memory allocations during marshaling.\n   * On success @a buf will contain a contiguous area in the CDR stream\n   * that can hold @a size bytes aligned to @a align.\n   * Results"]
    pub unsafe fn adjust(
        __this: *mut Self,
        mut out: *mut ACE_OutputCDR,
        mut size: libc::c_ulong,
        mut align: libc::c_ulong,
        mut buf: *mut *mut libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_OutputCDR>::adjust_ubc71eaf6ad051f92(
                    (::core::ptr::addr_of_mut!((* out))) as *mut ACE_OutputCDR,
                    size,
                    align,
                    ::core::ptr::addr_of_mut!((* buf)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Used by derived classes to set errors in the CDR stream.
    pub unsafe fn good_bit(
        __this: *mut Self,
        mut out: *mut ACE_OutputCDR,
        mut bit: bool,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*out).good_bit_ = bit;
            }
            ()
        }
    }
    ///Obtain the CDR Stream's major & minor version values.
    pub unsafe fn major_version(
        __this: *mut Self,
        mut input: *mut ACE_InputCDR,
    ) -> libc::c_uchar {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*input).major_version_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn minor_version(
        __this: *mut Self,
        mut input: *mut ACE_InputCDR,
    ) -> libc::c_uchar {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*input).minor_version_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn major_version_u45e2a7024c7ee433(
        __this: *mut Self,
        mut output: *mut ACE_OutputCDR,
    ) -> libc::c_uchar {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*output).major_version_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn minor_version_u63bdea0aebb16917(
        __this: *mut Self,
        mut output: *mut ACE_OutputCDR,
    ) -> libc::c_uchar {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*output).minor_version_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl Transfer_Contents {
    pub unsafe fn new_at(__this: *mut Self, mut rhs: *mut ACE_InputCDR) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).rhs_),
                ::core::ptr::addr_of_mut!((* rhs)),
            );
            {}
            ()
        }
    }
    pub unsafe fn new(mut __a0: *mut ACE_InputCDR) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
pub unsafe fn __vthunk_oued873b4a9e3519e9_iued873b4a9e3519e9(
    __this: *mut ACE_Log_Msg_Backend,
    p0: *mut ACE_Log_Record,
) -> libc::c_long {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_udbddea67410264fa {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Log_Msg_Backend),
    pub vfn_ue9cbb6d7ae11c030: unsafe fn(
        *mut ACE_Log_Msg_Backend,
        *const libc::c_char,
    ) -> libc::c_int,
    pub vfn_u26314720d1e50526: unsafe fn(*mut ACE_Log_Msg_Backend) -> libc::c_int,
    pub vfn_ubf24d44d4595cf9b: unsafe fn(*mut ACE_Log_Msg_Backend) -> libc::c_int,
    pub vfn_ued873b4a9e3519e9: unsafe fn(
        *mut ACE_Log_Msg_Backend,
        *mut ACE_Log_Record,
    ) -> libc::c_long,
}
pub static __VTBL_udbddea67410264fa: __Vtbl_udbddea67410264fa = __Vtbl_udbddea67410264fa {
    __type_info: &__TYPEINFO_19ACE_Log_Msg_Backend,
    __vdtor: __vdtor_udbddea67410264fa,
    vfn_ue9cbb6d7ae11c030: __vthunk_oue9cbb6d7ae11c030_iue9cbb6d7ae11c030,
    vfn_u26314720d1e50526: __vthunk_ou26314720d1e50526_iu26314720d1e50526,
    vfn_ubf24d44d4595cf9b: __vthunk_oubf24d44d4595cf9b_iubf24d44d4595cf9b,
    vfn_ued873b4a9e3519e9: __vthunk_oued873b4a9e3519e9_iued873b4a9e3519e9,
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
pub unsafe fn __vdtor_uf9c6177bf0b37713(__this: *mut ACE_Base_Thread_Adapter) {
    let _ = Box::from_raw(__this as *mut ACE_Base_Thread_Adapter);
}
pub unsafe fn __vthunk_ou11d7950624b3f9a1_iu11d7950624b3f9a1(
    __this: *mut ACE_Base_Thread_Adapter,
) -> *mut libc::c_void {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_uf9c6177bf0b37713 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Base_Thread_Adapter),
    pub vfn_u11d7950624b3f9a1: unsafe fn(
        *mut ACE_Base_Thread_Adapter,
    ) -> *mut libc::c_void,
}
pub static __VTBL_uf9c6177bf0b37713: __Vtbl_uf9c6177bf0b37713 = __Vtbl_uf9c6177bf0b37713 {
    __type_info: &__TYPEINFO_23ACE_Base_Thread_Adapter,
    __vdtor: __vdtor_uf9c6177bf0b37713,
    vfn_u11d7950624b3f9a1: __vthunk_ou11d7950624b3f9a1_iu11d7950624b3f9a1,
};
pub unsafe fn __vdtor_ued8c6ee26a6e6919(__this: *mut ACE_Addr) {
    let _ = Box::from_raw(__this as *mut ACE_INET_Addr);
}
pub unsafe fn __vthunk_ou7c9ccf8921bc010e_iueade8b323a2ffe30(
    __this: *mut ACE_Addr,
) -> *mut libc::c_void {
    <ACE_INET_Addr>::get_addr((__this as *mut ACE_INET_Addr))
}
pub unsafe fn __vthunk_ou0a9b8523b8a85ef5_iue2ea8d5bbd9dafa3(
    __this: *mut ACE_Addr,
    p0: *const libc::c_void,
    p1: libc::c_int,
) {
    <ACE_INET_Addr>::set_addr((__this as *mut ACE_INET_Addr), p0, p1)
}
pub unsafe fn __vthunk_ou4fd7596860a1760c_iuc46dbcdd1f343976(
    __this: *mut ACE_Addr,
) -> libc::c_ulong {
    <ACE_INET_Addr>::hash((__this as *mut ACE_INET_Addr))
}
pub unsafe fn __vthunk_oud6313bc69c6faed5_iud6313bc69c6faed5(
    __this: *mut ACE_INET_Addr,
    p0: *const libc::c_void,
    p1: libc::c_int,
    p2: libc::c_int,
) {
    <ACE_INET_Addr>::set_addr_ud6313bc69c6faed5(
        (__this as *mut ACE_INET_Addr),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou6a425f759686dd38_iu6a425f759686dd38(
    __this: *mut ACE_INET_Addr,
    p0: *mut libc::c_char,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_INET_Addr>::addr_to_string((__this as *mut ACE_INET_Addr), p0, p1, p2)
}
pub unsafe fn __vthunk_ou87cf85c4f5cb9bbb_iu87cf85c4f5cb9bbb(
    __this: *mut ACE_INET_Addr,
    p0: *const libc::c_char,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_INET_Addr>::string_to_addr((__this as *mut ACE_INET_Addr), p0, p1)
}
#[repr(C)]
pub struct __Vtbl_ued8c6ee26a6e6919 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Addr),
    pub vfn_ueade8b323a2ffe30: unsafe fn(*mut ACE_Addr) -> *mut libc::c_void,
    pub vfn_ue2ea8d5bbd9dafa3: unsafe fn(
        *mut ACE_Addr,
        *const libc::c_void,
        libc::c_int,
    ),
    pub vfn_uc46dbcdd1f343976: unsafe fn(*mut ACE_Addr) -> libc::c_ulong,
    pub vfn_ud6313bc69c6faed5: unsafe fn(
        *mut ACE_INET_Addr,
        *const libc::c_void,
        libc::c_int,
        libc::c_int,
    ),
    pub vfn_u6a425f759686dd38: unsafe fn(
        *mut ACE_INET_Addr,
        *mut libc::c_char,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u87cf85c4f5cb9bbb: unsafe fn(
        *mut ACE_INET_Addr,
        *const libc::c_char,
        libc::c_int,
    ) -> libc::c_int,
}
pub static __VTBL_ued8c6ee26a6e6919: __Vtbl_ued8c6ee26a6e6919 = __Vtbl_ued8c6ee26a6e6919 {
    __type_info: &__TYPEINFO_13ACE_INET_Addr,
    __vdtor: __vdtor_ued8c6ee26a6e6919,
    vfn_ueade8b323a2ffe30: __vthunk_ou7c9ccf8921bc010e_iueade8b323a2ffe30,
    vfn_ue2ea8d5bbd9dafa3: __vthunk_ou0a9b8523b8a85ef5_iue2ea8d5bbd9dafa3,
    vfn_uc46dbcdd1f343976: __vthunk_ou4fd7596860a1760c_iuc46dbcdd1f343976,
    vfn_ud6313bc69c6faed5: __vthunk_oud6313bc69c6faed5_iud6313bc69c6faed5,
    vfn_u6a425f759686dd38: __vthunk_ou6a425f759686dd38_iu6a425f759686dd38,
    vfn_u87cf85c4f5cb9bbb: __vthunk_ou87cf85c4f5cb9bbb_iu87cf85c4f5cb9bbb,
};
pub unsafe fn __vdtor_uae6401f90b61767c(__this: *mut ACE_Log_Msg_Backend) {
    let _ = Box::from_raw(__this as *mut ACE_Log_Msg_IPC);
}
pub unsafe fn __vthunk_ouc41a41bc8ce5ac30_iue9cbb6d7ae11c030(
    __this: *mut ACE_Log_Msg_Backend,
    p0: *const libc::c_char,
) -> libc::c_int {
    <ACE_Log_Msg_IPC>::open((__this as *mut ACE_Log_Msg_IPC), p0)
}
pub unsafe fn __vthunk_oud002b650bd70c926_iu26314720d1e50526(
    __this: *mut ACE_Log_Msg_Backend,
) -> libc::c_int {
    <ACE_Log_Msg_IPC>::reset((__this as *mut ACE_Log_Msg_IPC))
}
pub unsafe fn __vthunk_ou68f6437d3121939b_iubf24d44d4595cf9b(
    __this: *mut ACE_Log_Msg_Backend,
) -> libc::c_int {
    <ACE_Log_Msg_IPC>::close((__this as *mut ACE_Log_Msg_IPC))
}
pub unsafe fn __vthunk_ou573fe8f6675c7de9_iued873b4a9e3519e9(
    __this: *mut ACE_Log_Msg_Backend,
    p0: *mut ACE_Log_Record,
) -> libc::c_long {
    <ACE_Log_Msg_IPC>::log((__this as *mut ACE_Log_Msg_IPC), p0)
}
#[repr(C)]
pub struct __Vtbl_uae6401f90b61767c {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Log_Msg_Backend),
    pub vfn_ue9cbb6d7ae11c030: unsafe fn(
        *mut ACE_Log_Msg_Backend,
        *const libc::c_char,
    ) -> libc::c_int,
    pub vfn_u26314720d1e50526: unsafe fn(*mut ACE_Log_Msg_Backend) -> libc::c_int,
    pub vfn_ubf24d44d4595cf9b: unsafe fn(*mut ACE_Log_Msg_Backend) -> libc::c_int,
    pub vfn_ued873b4a9e3519e9: unsafe fn(
        *mut ACE_Log_Msg_Backend,
        *mut ACE_Log_Record,
    ) -> libc::c_long,
}
pub static __VTBL_uae6401f90b61767c: __Vtbl_uae6401f90b61767c = __Vtbl_uae6401f90b61767c {
    __type_info: &__TYPEINFO_15ACE_Log_Msg_IPC,
    __vdtor: __vdtor_uae6401f90b61767c,
    vfn_ue9cbb6d7ae11c030: __vthunk_ouc41a41bc8ce5ac30_iue9cbb6d7ae11c030,
    vfn_u26314720d1e50526: __vthunk_oud002b650bd70c926_iu26314720d1e50526,
    vfn_ubf24d44d4595cf9b: __vthunk_ou68f6437d3121939b_iubf24d44d4595cf9b,
    vfn_ued873b4a9e3519e9: __vthunk_ou573fe8f6675c7de9_iued873b4a9e3519e9,
};
pub unsafe fn __vdtor_ub182f02a4e33516c(__this: *mut ACE_InputCDR) {
    let _ = Box::from_raw(__this as *mut ACE_InputCDR);
}
#[repr(C)]
pub struct __Vtbl_ub182f02a4e33516c {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_InputCDR),
}
pub static __VTBL_ub182f02a4e33516c: __Vtbl_ub182f02a4e33516c = __Vtbl_ub182f02a4e33516c {
    __type_info: &__TYPEINFO_12ACE_InputCDR,
    __vdtor: __vdtor_ub182f02a4e33516c,
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
pub unsafe fn __vdtor_ue1bfefa7873110ae(__this: *mut ACE_Char_Codeset_Translator) {
    let _ = Box::from_raw(__this as *mut ACE_Char_Codeset_Translator);
}
pub unsafe fn __vthunk_ou5c6599352240b84d_iu5c6599352240b84d(
    __this: *mut ACE_Char_Codeset_Translator,
    p0: *mut ACE_InputCDR,
    p1: *mut libc::c_char,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oud0b313606cefd622_iud0b313606cefd622(
    __this: *mut ACE_Char_Codeset_Translator,
    p0: *mut ACE_InputCDR,
    p1: *mut *mut libc::c_char,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou149f56048e9130dd_iu149f56048e9130dd(
    __this: *mut ACE_Char_Codeset_Translator,
    p0: *mut ACE_InputCDR,
    p1: *mut crate::__cxx_std::String,
) -> bool {
    <ACE_Char_Codeset_Translator>::read_string_u149f56048e9130dd(
        (__this as *mut ACE_Char_Codeset_Translator),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ou8702df99cfecd82b_iu8702df99cfecd82b(
    __this: *mut ACE_Char_Codeset_Translator,
    p0: *mut ACE_InputCDR,
    p1: *mut libc::c_char,
    p2: libc::c_uint,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou61069d17dd479073_iu61069d17dd479073(
    __this: *mut ACE_Char_Codeset_Translator,
    p0: *mut ACE_OutputCDR,
    p1: libc::c_char,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou9455d85558a51633_iu9455d85558a51633(
    __this: *mut ACE_Char_Codeset_Translator,
    p0: *mut ACE_OutputCDR,
    p1: libc::c_uint,
    p2: *const libc::c_char,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou21eb314fe72cbce4_iu21eb314fe72cbce4(
    __this: *mut ACE_Char_Codeset_Translator,
    p0: *mut ACE_OutputCDR,
    p1: *const libc::c_char,
    p2: libc::c_uint,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oubbe3c6a798993407_iubbe3c6a798993407(
    __this: *mut ACE_Char_Codeset_Translator,
) -> libc::c_uint {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oub0d37f7f0e8d8e3d_iub0d37f7f0e8d8e3d(
    __this: *mut ACE_Char_Codeset_Translator,
) -> libc::c_uint {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_ue1bfefa7873110ae {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Char_Codeset_Translator),
    pub vfn_u5c6599352240b84d: unsafe fn(
        *mut ACE_Char_Codeset_Translator,
        *mut ACE_InputCDR,
        *mut libc::c_char,
    ) -> bool,
    pub vfn_ud0b313606cefd622: unsafe fn(
        *mut ACE_Char_Codeset_Translator,
        *mut ACE_InputCDR,
        *mut *mut libc::c_char,
    ) -> bool,
    pub vfn_u149f56048e9130dd: unsafe fn(
        *mut ACE_Char_Codeset_Translator,
        *mut ACE_InputCDR,
        *mut crate::__cxx_std::String,
    ) -> bool,
    pub vfn_u8702df99cfecd82b: unsafe fn(
        *mut ACE_Char_Codeset_Translator,
        *mut ACE_InputCDR,
        *mut libc::c_char,
        libc::c_uint,
    ) -> bool,
    pub vfn_u61069d17dd479073: unsafe fn(
        *mut ACE_Char_Codeset_Translator,
        *mut ACE_OutputCDR,
        libc::c_char,
    ) -> bool,
    pub vfn_u9455d85558a51633: unsafe fn(
        *mut ACE_Char_Codeset_Translator,
        *mut ACE_OutputCDR,
        libc::c_uint,
        *const libc::c_char,
    ) -> bool,
    pub vfn_u21eb314fe72cbce4: unsafe fn(
        *mut ACE_Char_Codeset_Translator,
        *mut ACE_OutputCDR,
        *const libc::c_char,
        libc::c_uint,
    ) -> bool,
    pub vfn_ubbe3c6a798993407: unsafe fn(
        *mut ACE_Char_Codeset_Translator,
    ) -> libc::c_uint,
    pub vfn_ub0d37f7f0e8d8e3d: unsafe fn(
        *mut ACE_Char_Codeset_Translator,
    ) -> libc::c_uint,
}
pub static __VTBL_ue1bfefa7873110ae: __Vtbl_ue1bfefa7873110ae = __Vtbl_ue1bfefa7873110ae {
    __type_info: &__TYPEINFO_27ACE_Char_Codeset_Translator,
    __vdtor: __vdtor_ue1bfefa7873110ae,
    vfn_u5c6599352240b84d: __vthunk_ou5c6599352240b84d_iu5c6599352240b84d,
    vfn_ud0b313606cefd622: __vthunk_oud0b313606cefd622_iud0b313606cefd622,
    vfn_u149f56048e9130dd: __vthunk_ou149f56048e9130dd_iu149f56048e9130dd,
    vfn_u8702df99cfecd82b: __vthunk_ou8702df99cfecd82b_iu8702df99cfecd82b,
    vfn_u61069d17dd479073: __vthunk_ou61069d17dd479073_iu61069d17dd479073,
    vfn_u9455d85558a51633: __vthunk_ou9455d85558a51633_iu9455d85558a51633,
    vfn_u21eb314fe72cbce4: __vthunk_ou21eb314fe72cbce4_iu21eb314fe72cbce4,
    vfn_ubbe3c6a798993407: __vthunk_oubbe3c6a798993407_iubbe3c6a798993407,
    vfn_ub0d37f7f0e8d8e3d: __vthunk_oub0d37f7f0e8d8e3d_iub0d37f7f0e8d8e3d,
};
pub unsafe fn __vdtor_u7f71c32ff7e5c9bb(__this: *mut ACE_WChar_Codeset_Translator) {
    let _ = Box::from_raw(__this as *mut ACE_WChar_Codeset_Translator);
}
pub unsafe fn __vthunk_ou640effbe06ca84c2_iu640effbe06ca84c2(
    __this: *mut ACE_WChar_Codeset_Translator,
    p0: *mut ACE_InputCDR,
    p1: *mut libc::wchar_t,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou99464aefd8bf3b83_iu99464aefd8bf3b83(
    __this: *mut ACE_WChar_Codeset_Translator,
    p0: *mut ACE_InputCDR,
    p1: *mut *mut libc::wchar_t,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou94b79f713f9e771a_iu94b79f713f9e771a(
    __this: *mut ACE_WChar_Codeset_Translator,
    p0: *mut ACE_InputCDR,
    p1: *mut crate::__cxx_std::WString,
) -> bool {
    <ACE_WChar_Codeset_Translator>::read_wstring_u94b79f713f9e771a(
        (__this as *mut ACE_WChar_Codeset_Translator),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ouc4b8c672331a5a94_iuc4b8c672331a5a94(
    __this: *mut ACE_WChar_Codeset_Translator,
    p0: *mut ACE_InputCDR,
    p1: *mut libc::wchar_t,
    p2: libc::c_uint,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou0372aeecb10f2296_iu0372aeecb10f2296(
    __this: *mut ACE_WChar_Codeset_Translator,
    p0: *mut ACE_OutputCDR,
    p1: libc::wchar_t,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou87d7635fe14273fa_iu87d7635fe14273fa(
    __this: *mut ACE_WChar_Codeset_Translator,
    p0: *mut ACE_OutputCDR,
    p1: libc::c_uint,
    p2: *const libc::wchar_t,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou239c561f8bfd7f81_iu239c561f8bfd7f81(
    __this: *mut ACE_WChar_Codeset_Translator,
    p0: *mut ACE_OutputCDR,
    p1: *const libc::wchar_t,
    p2: libc::c_uint,
) -> bool {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouf5e94c8176a15b97_iuf5e94c8176a15b97(
    __this: *mut ACE_WChar_Codeset_Translator,
) -> libc::c_uint {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouc04b444ceb25aead_iuc04b444ceb25aead(
    __this: *mut ACE_WChar_Codeset_Translator,
) -> libc::c_uint {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_u7f71c32ff7e5c9bb {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_WChar_Codeset_Translator),
    pub vfn_u640effbe06ca84c2: unsafe fn(
        *mut ACE_WChar_Codeset_Translator,
        *mut ACE_InputCDR,
        *mut libc::wchar_t,
    ) -> bool,
    pub vfn_u99464aefd8bf3b83: unsafe fn(
        *mut ACE_WChar_Codeset_Translator,
        *mut ACE_InputCDR,
        *mut *mut libc::wchar_t,
    ) -> bool,
    pub vfn_u94b79f713f9e771a: unsafe fn(
        *mut ACE_WChar_Codeset_Translator,
        *mut ACE_InputCDR,
        *mut crate::__cxx_std::WString,
    ) -> bool,
    pub vfn_uc4b8c672331a5a94: unsafe fn(
        *mut ACE_WChar_Codeset_Translator,
        *mut ACE_InputCDR,
        *mut libc::wchar_t,
        libc::c_uint,
    ) -> bool,
    pub vfn_u0372aeecb10f2296: unsafe fn(
        *mut ACE_WChar_Codeset_Translator,
        *mut ACE_OutputCDR,
        libc::wchar_t,
    ) -> bool,
    pub vfn_u87d7635fe14273fa: unsafe fn(
        *mut ACE_WChar_Codeset_Translator,
        *mut ACE_OutputCDR,
        libc::c_uint,
        *const libc::wchar_t,
    ) -> bool,
    pub vfn_u239c561f8bfd7f81: unsafe fn(
        *mut ACE_WChar_Codeset_Translator,
        *mut ACE_OutputCDR,
        *const libc::wchar_t,
        libc::c_uint,
    ) -> bool,
    pub vfn_uf5e94c8176a15b97: unsafe fn(
        *mut ACE_WChar_Codeset_Translator,
    ) -> libc::c_uint,
    pub vfn_uc04b444ceb25aead: unsafe fn(
        *mut ACE_WChar_Codeset_Translator,
    ) -> libc::c_uint,
}
pub static __VTBL_u7f71c32ff7e5c9bb: __Vtbl_u7f71c32ff7e5c9bb = __Vtbl_u7f71c32ff7e5c9bb {
    __type_info: &__TYPEINFO_28ACE_WChar_Codeset_Translator,
    __vdtor: __vdtor_u7f71c32ff7e5c9bb,
    vfn_u640effbe06ca84c2: __vthunk_ou640effbe06ca84c2_iu640effbe06ca84c2,
    vfn_u99464aefd8bf3b83: __vthunk_ou99464aefd8bf3b83_iu99464aefd8bf3b83,
    vfn_u94b79f713f9e771a: __vthunk_ou94b79f713f9e771a_iu94b79f713f9e771a,
    vfn_uc4b8c672331a5a94: __vthunk_ouc4b8c672331a5a94_iuc4b8c672331a5a94,
    vfn_u0372aeecb10f2296: __vthunk_ou0372aeecb10f2296_iu0372aeecb10f2296,
    vfn_u87d7635fe14273fa: __vthunk_ou87d7635fe14273fa_iu87d7635fe14273fa,
    vfn_u239c561f8bfd7f81: __vthunk_ou239c561f8bfd7f81_iu239c561f8bfd7f81,
    vfn_uf5e94c8176a15b97: __vthunk_ouf5e94c8176a15b97_iuf5e94c8176a15b97,
    vfn_uc04b444ceb25aead: __vthunk_ouc04b444ceb25aead_iuc04b444ceb25aead,
};
