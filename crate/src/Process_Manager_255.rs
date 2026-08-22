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
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Handle_Set_nbits_: [libc::c_char; 256usize];
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
    pub use crate::full_ops_0::ACE_OS::wait_udebb2ca6ef98a3fa;
    pub use crate::full_ops_0::ACE_OS::wait_u9d999b984b9ff79a;
    pub use crate::full_ops_0::ACE_OS::waitpid_ua99c1cff38bad46d;
    pub use crate::full_ops_0::ACE_OS::ACE_HRTimer_Op;
    pub const ACE_HRTIMER_START: ACE_HRTimer_Op = 0 as ACE_HRTimer_Op;
    pub const ACE_HRTIMER_INCR: ACE_HRTimer_Op = 1 as ACE_HRTimer_Op;
    pub const ACE_HRTIMER_STOP: ACE_HRTimer_Op = 2 as ACE_HRTimer_Op;
    pub const ACE_HRTIMER_GETTIME: ACE_HRTimer_Op = 65535 as ACE_HRTimer_Op;
    pub use crate::full_ops_0::ACE_OS::asctime_uf8502e7a2e96e737;
    pub use crate::full_ops_0::ACE_OS::asctime_r_ue1639a94476df356;
    pub use crate::full_ops_0::ACE_OS::clock_gettime_ud3037344305752f7;
    pub use crate::full_ops_0::ACE_OS::clock_settime_ufa866b37a86e6d9c;
    pub use crate::full_ops_0::ACE_OS::ctime_u7209e9bc5e02bb5b;
    pub use crate::full_ops_0::ACE_OS::ctime_r_u81fdbec9675eeb84;
    pub use crate::full_ops_0::ACE_OS::difftime_u7379ee088bf95ffd;
    pub use crate::full_ops_0::ACE_OS::gethrtime;
    pub use crate::full_ops_0::ACE_OS::gmtime_ued5a8cc5ef3123f8;
    pub use crate::full_ops_0::ACE_OS::gmtime_r_udd1d8ff82c02e938;
    pub use crate::full_ops_0::ACE_OS::localtime_ud9bfad8c2ebc8529;
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS11localtime_rEPKlP2tm"]
        pub fn localtime_r_u00ea56666aff6d11(
            clock: *const libc::c_long,
            res: *mut super::tm,
        ) -> *mut super::tm;
    }
    extern "C-unwind" {
        #[link_name = "_ZN6ACE_OS6mktimeEP2tm"]
        pub fn mktime_ua30f0b21ba95e4f3(timeptr: *mut super::tm) -> libc::c_long;
    }
    pub use crate::full_ops_0::ACE_OS::nanosleep_ue6919a8c01f74bcb;
    pub use crate::full_ops_0::ACE_OS::strftime_uc75f7a0b22241bcd;
    pub use crate::full_ops_0::ACE_OS::strptime_u8121d5ca334a950e;
    pub use crate::full_ops_0::ACE_OS::time_udaedd75f32971343;
    pub use crate::full_ops_0::ACE_OS::timezone;
    pub use crate::full_ops_0::ACE_OS::tzset_ueecba752fce1d04d;
}
#[doc = "* @class ACE_Thread_Mutex\n *\n * @brief ACE_Thread_Mutex wrapper (only valid for threads in the same\n * process).\n *\n * This implementation is optimized for locking threads that are\n * in the same process.  It maps to <CRITICAL_SECTION>s on NT\n * and <ACE_mutex_t> with <type> set to <USYNC_THREAD> on UNIX.\n * ACE_Thread_Mutex is recursive on some platforms (like\n * Win32). However, on most platforms (like Solaris) it is not\n * recursive.  To be totally safe and portable, developers\n * should use ACE_Recursive_Thread_Mutex when they need a\n * recursive mutex."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Thread_Mutex {
    pub lock_: pthread_mutex_t,
    pub removed_: bool,
}
impl Drop for ACE_Thread_Mutex {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u2944902dcb34c4d7"]
                fn __ext(__this: *mut ACE_Thread_Mutex);
            }
            __ext(self as *mut Self);
        }
    }
}
#[repr(C)]
pub struct ACE_Process_Mutex {
    pub _opaque: [u8; 1],
}
#[doc = "* @class ACE_Recursive_Thread_Mutex\n *\n * @brief Implement a C++ wrapper that allows nested acquisition and\n * release of a mutex that occurs in the same thread."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Recursive_Thread_Mutex {
    pub lock_: pthread_mutex_t,
    pub removed_: bool,
}
impl Drop for ACE_Recursive_Thread_Mutex {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_ub41d302ca1e4f26a"]
                fn __ext(__this: *mut ACE_Recursive_Thread_Mutex);
            }
            __ext(self as *mut Self);
        }
    }
}
#[doc = "* @class ACE_RW_Thread_Mutex\n *\n * @brief Wrapper for readers/writer locks that exist within a process."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_RW_Thread_Mutex {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_RW_Mutex>,
}
impl Drop for ACE_RW_Thread_Mutex {
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
pub unsafe extern "C-unwind" fn __acedtor_u6e998b886c0cf6fd(
    __this: *mut ACE_RW_Thread_Mutex,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[repr(C)]
pub struct ACE_Thread_Semaphore {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Condition_ACE_Thread_Mutex_ {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Condition_ACE_Recursive_Thread_Mutex_ {
    pub _opaque: [u8; 1],
}
pub type ACE_Condition_Thread_Mutex = ACE_Condition_ACE_Thread_Mutex_;
pub type ACE_Condition_Recursive_Thread_Mutex = ACE_Condition_ACE_Recursive_Thread_Mutex_;
pub type ACE_MT_SYNCH_MUTEX = ACE_Thread_Mutex;
pub type ACE_MT_SYNCH_PROCESS_MUTEX = ACE_Process_Mutex;
pub type ACE_MT_SYNCH_RECURSIVE_MUTEX = ACE_Recursive_Thread_Mutex;
pub type ACE_MT_SYNCH_RW_MUTEX = ACE_RW_Thread_Mutex;
pub type ACE_MT_SYNCH_CONDITION = ACE_Condition_ACE_Thread_Mutex_;
pub type ACE_MT_SYNCH_RECURSIVE_CONDITION = ACE_Condition_ACE_Recursive_Thread_Mutex_;
pub type ACE_MT_SYNCH_SEMAPHORE = ACE_Thread_Semaphore;
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
#[repr(C)]
pub struct ACE_Log_Msg_Callback {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Thread_Descriptor {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Log_Record {
    pub _opaque: [u8; 1],
}
#[doc = "* @class ACE_Log_Category_TSS\n *\n * @brief The thread specific object for a ACE_Log_Categy object.\n *\n * @see ACE_Log_Categy"]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Log_Category_TSS {
    pub category_: *mut ACE_Log_Category,
    pub logger_: *mut ACE_Log_Msg,
    pub priority_mask_: libc::c_ulong,
}
/**This pointer is 0 if we are not reference counting (the user has not
  /// passed "true" for the delete_ostream argument to msg_ostream).
  /// If we are reference counting, this points to a shared count that will
  /// be deleted when it reaches zero.  Since we want optional but shared
  /// ownership neither std::auto_ptr nor ACE_Strong_Bound_Ptr have the right
  /// semantics.  *Bound_Ptr also doesn't take advantage of Atomic_Op.*/
pub type ACE_Log_Msg_Atomic_ULong = ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_;
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Log_Msg_process_priority_mask_: libc::c_ulong;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Log_Msg_program_name_: *const libc::c_char;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Log_Msg_local_host_: *const libc::c_char;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Log_Msg_flags_: libc::c_ulong;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Log_Msg_msg_off_: libc::c_long;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Log_Msg_instance_count_: libc::c_int;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Log_Msg_default_priority_mask_: libc::c_ulong;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Log_Msg_key_created_: bool;
}
#[doc = "* @class ACE_Log_Msg\n *\n * @brief Provides a variable length argument message logging\n * abstraction.\n *\n * This class is very flexible since it allows formatted error\n * messages to be printed in a thread-safe manner to various\n * locations, such as stderr, cerr, a distributed logger, etc.  The\n * current message is also kept in a thread-specific storage location\n * (threads spawned using ACE_Thread_Manager automatically get an\n * ACE_Log_Msg object that inherits the spawning thread's settings),\n * which can be used to communicate errors between framework methods\n * and callers.  A message is logged by the log() method, only if the\n * message priority is currently enabled.  Moreover, only the current\n * log message is stored here -- it will be overwritten by the\n * subsequent call to log().\n *\n * The ACE_Log_Msg class uses two priority masks to control its\n * logging behavior.  The @c priority_mask_ object attribute is\n * thread- specific and specifies the priority levels logged by the\n * thread.  The @c process_priority_mask_ class attribute is not\n * thread-specific and specifies the priority levels that will be\n * logged by all threads in the process.  By default, all levels are\n * disabled for @c priority_mask_ and all levels are enabled for @c\n * process_priority_mask_ (i.e. the process-wide mask controls the\n * settings, and each instance can expand on it if desired).  Both\n * priority masks can be modified using the priority_mask() method of\n * this class."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Log_Msg {
    pub status_: libc::c_int,
    pub errnum_: libc::c_int,
    pub linenum_: libc::c_int,
    pub file_: [libc::c_char; 4097usize],
    pub msg_: *mut libc::c_char,
    pub restart_: bool,
    pub ostream_: *mut crate::__cxx_std::Ostream,
    pub ostream_refcount_: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_,
    pub msg_callback_: *mut ACE_Log_Msg_Callback,
    pub trace_depth_: libc::c_int,
    pub trace_active_: bool,
    pub tracing_enabled_: bool,
    pub thr_desc_: *mut ACE_Thread_Descriptor,
    pub priority_mask_: libc::c_ulong,
    pub timestamp_: libc::c_int,
    pub conditional_values_: _unnamed_struct_at__build_ace_full_src_ACE_ace_Log_Msg_h_738_3_,
}
impl Drop for ACE_Log_Msg {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u6c712f2129200c9d"]
                fn __ext(__this: *mut ACE_Log_Msg);
            }
            __ext(self as *mut Self);
        }
    }
}
extern "C-unwind" {
    #[link_name = "_Z15ACE_TSS_cleanupPv"]
    pub fn ACE_TSS_cleanup(ptr: *mut libc::c_void);
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
pub mod ACE {
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
    pub use crate::full_ops_0::ACE_Utils::Truncator_unsigned_long__long_;
    pub use crate::full_ops_0::ACE_Utils::truncator;
    pub use crate::full_ops_0::ACE_Utils::truncate_cast___long__ub591475ebc843689;
    pub type truncator_289 = Truncator_unsigned_long__long_;
    pub use crate::full_ops_0::ACE_Utils::truncate_cast___unsigned_long__u503964095c07d269;
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
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Object_Manager_preallocated_object: [*mut libc::c_void; 11usize];
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Object_Manager_preallocated_array: [*mut libc::c_void; 1usize];
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Object_Manager_instance_: *mut ACE_Object_Manager;
}
#[doc = "* @class ACE_Object_Manager\n *\n * @brief Manager for ACE library services and singleton cleanup.\n *\n * The ACE_Object_Manager manages cleanup of objects, typically\n * singletons, at program termination.  In addition to managing\n * the cleanup of the ACE library, it provides an interface for\n * application to register objects to be cleaned up.\n * This class also shuts down ACE library services, so that they\n * can reclaim their storage, at program termination.  It works\n * by creating a static instance whose destructor gets called\n * along with those of all other static objects.  Hooks are\n * provided for application code to register objects and arrays\n * for cleanup, e.g., destruction.  The order of such cleanup\n * calls is in the reverse order of registration, i.e., that\n * last object/array to register gets cleaned up first.\n * The ACE_Object_Manager API includes ACE_Managed_Object.  That\n * class is contained in a separate file because it is a\n * template class, and some compilers require that template and\n * non-template class definitions appear in separate files.\n * Please see ace/Managed_Object.h for a description of that\n * part of the API.  In summary, ACE_Managed_Object provides two\n * adapters, the ACE_Cleanup_Adapter and ACE_Managed_Object\n * template classes for adapting objects of any type to be\n * easily managed by the ACE_Object_Manager.  There are several\n * mechanisms for adapting objects and arrays for cleanup at\n * program termination, in roughly increasing order of ease-of-use:\n * 1) Derive the object's class from ACE_Cleanup.\n * 2) Allow the ACE_Object_Manager to both dynamically allocate\n * and deallocate the object.\n * 3) Provide an <ACE_CLEANUP_FUNC> cleanup hook for the object or\n * array.\n * 4) Allow the ACE_Object_Manager to both preallocate the object\n * or array, either statically in global data or dynamically on\n * the heap, when its singleton instance is construction.\n *\n * There are also several mechanisms for registering objects and\n * arrays for cleanup.  In decreasing order of flexibility and\n * complexity (with the exception of the last mechanism):\n *\n * 1) ACE_Object_Manager::at_exit (void *object,\n * ACE_CLEANUP_FUNC cleanup_hook,\n * void *param);\n * can be used to register any object or array for any\n * cleanup activity at program termination.\n * 2) ACE_Object_Manager::at_exit (ACE_Cleanup *object,\n * void *param = 0);\n * can be used to register an ACE_Cleanup object\n * for any cleanup activity at program termination.\n * The final mechanism is not general purpose, but can only\n * be used to allocate objects and arrays at program startup:\n * 3) ACE_Managed_Object::get_preallocated_object\n * (ACE_Object_Manager::Preallocated_Object id);\n * and\n * ACE_Managed_Object::get_preallocated_array\n * (ACE_Object_Manager::Preallocated_Array id);\n * can only be used to allocate objects at program startup,\n * either in global data or on the heap (selected at compile\n * time).  These are intended to replace static locks, etc.\n * Instead of creating a static ACE_Object_Manager instance, one\n * can alternatively be created on the stack of the main program\n * thread.  It is created just after entry to ::main (int, char\n * *[]), and before any existing code in that function is\n * executed.  To enable this alternative, add #define\n * ACE_HAS_NONSTATIC_OBJECT_MANAGER before including the platform\n * specific config-* file in ace/config.h prior to\n * building the ACE library and your applications.  This #define\n * is enabled in some config files that are supplied with ACE.\n *\n * To ensure a static object manager is used, #undef\n * ACE_HAS_NONSTATIC_OBJECT_MANAGER *after* including the platform\n * specific config-* file.\n * Note that the ACE_Object_Manager _must_ be created before\n * any threads are spawned by the program.\n * If ACE_HAS_NONSTATIC_OBJECT_MANAGER is not #defined, the ACE\n * library creates a static, singleton ACE_Object_Manager instance.\n * The instance is placed in global program data, and constructed\n * via a static object constructor.  If ACE_HAS_NONSTATIC_OBJECT_MANAGER\n * is #defined, the ACE_Object_Manager instance is created on the stack\n * of the main program thread, as noted above.\n *\n * With ACE_HAS_NONSTATIC_OBJECT_MANAGER enabled, the ACE\n * library has no static objects that require destruction.\n * However, there are two drawbacks to using it:\n * 1) main (int, char *[]) must be declared with arguments, even\n * if they're not used.  All of ACE is converted to this, so\n * just applications have to be concerned with it.\n * 2) If there any static objects that depend on those that are\n * cleaned up by the Object_Manager, they'll get cleaned up too\n * late.  The ACE tests do not violate this requirement.\n * However, applications may have trouble with it.\n * NOTE on the use of <::exit> -- <::exit> does not destroy\n * automatic objects.  Therefore, if\n * ACE_HAS_NONSTATIC_OBJECT_MANAGER is enabled, the\n * ACE_Object_Manager instance will *not* be destroyed if\n * <::exit> is called!  However, <ACE_OS::exit> will properly\n * destroy the ACE_Object_Manager.  It is highly recommended\n * that <ACE_OS::exit> be used instead of <::exit>.\n *\n * However, <::exit> and <ACE_OS::exit> are tricky to use\n * properly, especially in multithread programs.  It is much\n * safer to throw an exception (or simulate that effect) that\n * will be caught by <main> instead of calling exit.  Then,\n * <main> can perform any necessary application-specific cleanup\n * and return the status value.  In addition, it's usually best\n * to avoid calling <::exit> and <ACE_OS::exit> from threads\n * other than the main thread.  Thanks to Jeff Greif\n * <jmg@trivida.com> for pointing out that <::exit> doesn't\n * destroy automatic objects, and for developing the\n * recommendations in this paragraph.\n *\n * Instead of creating a static ACE_Object_Manager, or letting\n * ACE create it on the stack of <main> for you, another\n * alternative is to #define\n * ACE_DOESNT_INSTANTIATE_NONSTATIC_OBJECT_MANAGER.  With that\n * #define, the application must create the ACE_Object_Manager.\n * The recommended way is to call <ACE::init> at the start of\n * the program, and call <ACE::fini> at the end.  Alternatively,\n * the application could explicity construct an\n * ACE_Object_Manager."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Object_Manager {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Object_Manager_Base>,
    pub exit_info_: ::core::mem::ManuallyDrop<ACE_OS_Exit_Info>,
    pub preallocations_: *mut ACE_Object_Manager_Preallocations,
    pub ace_service_config_sig_handler_: *mut ACE_Sig_Adapter,
    pub internal_lock_: *mut ACE_Recursive_Thread_Mutex,
    pub singleton_null_lock_: *mut ACE_Cleanup_Adapter_ACE_Null_Mutex_,
    pub singleton_recursive_lock_: *mut ACE_Cleanup_Adapter_ACE_Recursive_Thread_Mutex_,
}
#[repr(C)]
pub struct ACE_OS_Object_Manager_Manager {
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
#[doc = "* @class ACE_Log_Category\n *\n * @brief Provides a categorized message logging\n * abstraction.\n *\n * This class added another level of abstraction to\n * @c ACE_Log_Msg to separate log messages into different\n * categories. Logs in different categories can be independently\n * enabled or disabled. However, they will all be affected by the\n * priority_mask setting in ACE_Log_Msg. That is to say, if a\n * given priority level is disabled using @c ACE_Log_Msg::priority_mask(),\n * all messages of that priority level logged via any @c ACE_Log_Category\n * object would also be disabled regardless of the @c priority_mask\n * setting in the  @c ACE_Log_Category object.\n *\n * Each category can have a name which\n * is fixed at construction. The name is not used for\n * formatting the messages. However, it can be used by a\n * message backend object for identification and reformat\n * accordingly.\n *\n * To log a message into a category. Create a new @c ACE_Log_Category\n * and then use @c per_thr_obj() for logging. For example,\n *\n * \\code{.cpp}\n *  ACE_Log_Category test_category(\"Test\");\n *  test_category.per_thr_obj()->log(LM_DEBUG, \"Log into the Test category.\");\n *\n *  // set the process wide priority mask\n *  test_category.priority_mask(LM_DEBUG|LM_ERROR);\n *\n *  // set the thread specific priority mask\n *  test_category.per_thr_obj()->priority_mask(LM_DEBUG);\n * \\endcode"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Log_Category {
    pub name_: *const libc::c_char,
    pub id_: libc::c_uint,
    pub priority_mask_: libc::c_ulong,
    pub keylock_: ::core::mem::ManuallyDrop<ACE_Thread_Mutex>,
    pub key_: libc::c_uint,
}
impl Drop for ACE_Log_Category {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u52845f472dabf67c"]
                fn __ext(__this: *mut ACE_Log_Category);
            }
            __ext(self as *mut Self);
        }
    }
}
#[doc = "* @class ACE_Process_Options\n *\n * @brief Process Options\n *\n * This class controls the options passed to <CreateProcess> (or <fork>\n * and <exec>).\n * Notice that on Windows CE, creating a process merely means\n * instantiating a new process.  You can't set the handles (since\n * there's no stdin, stdout and stderr,) specify process/thread\n * options, set environment,...  So, basically, this class only\n * set the command line and nothing else.\n * Notice that on UNIX platforms, if the <setenv> is used, the\n * <spawn> is using the <execve> system call. It means that the\n * <command_line> should include a full path to the program file\n * (<execve> does not search the PATH).  If <setenv> is not used\n * then, the <spawn> is using the <execvp> which searches for the\n * program file in the PATH variable."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Process_Options {
    pub inherit_environment_: bool,
    pub creation_flags_: libc::c_ulong,
    pub avoid_zombies_: libc::c_int,
    pub stdin_: libc::c_int,
    pub stdout_: libc::c_int,
    pub stderr_: libc::c_int,
    pub ruid_: libc::c_uint,
    pub euid_: libc::c_uint,
    pub rgid_: libc::c_uint,
    pub egid_: libc::c_uint,
    pub handle_inheritance_: bool,
    pub set_handles_called_: libc::c_int,
    pub environment_buf_index_: libc::c_ulong,
    pub environment_argv_index_: libc::c_ulong,
    pub environment_buf_: *mut libc::c_char,
    pub environment_buf_len_: libc::c_ulong,
    pub environment_argv_: *mut *mut libc::c_char,
    pub max_environment_args_: libc::c_ulong,
    pub max_environ_argv_index_: libc::c_ulong,
    pub working_directory_: [libc::c_char; 4097usize],
    pub command_line_argv_calculated_: bool,
    pub command_line_buf_: *mut libc::c_char,
    pub command_line_copy_: *mut libc::c_char,
    pub command_line_buf_len_: libc::c_ulong,
    pub max_command_line_args_: libc::c_ulong,
    pub command_line_argv_: *mut *mut libc::c_char,
    pub process_group_: libc::c_int,
    pub handles_passed_: ACE_Handle_Set,
    pub dup_handles_: ACE_Handle_Set,
    pub process_name_: [libc::c_char; 4097usize],
    pub use_unicode_environment_: bool,
}
impl Drop for ACE_Process_Options {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u9f898fe84bb87f35"]
                fn __ext(__this: *mut ACE_Process_Options);
            }
            __ext(self as *mut Self);
        }
    }
}
#[doc = "* @class ACE_Process\n *\n * @brief A portable encapsulation for creating and managing new processes.\n *\n * ACE_Process provides a convenient way to:\n *  - Spawn child processes, with convenient hooks for pre- and post-spawn\n *    actions\n *  - Check if a spawned process is still running\n *  - Kill a spawned child process\n *  - Wait for a spawned child process to exit.\n *\n * @see ACE_Process_Options because it is used to\n * pass options when spawning child processes.\n *\n * @see ACE_Process_Manager for additional ways to manage spawned\n * processes."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Process {
    pub vptr: *const (),
    pub child_id_: libc::c_int,
    pub exit_code_: libc::c_int,
    pub handles_passed_: ACE_Handle_Set,
    pub dup_handles_: ACE_Handle_Set,
}
#[doc = "* @class ACE_Managed_Process\n *\n * @brief A process easily managed by ACE_Process_Manager.\n *\n * @arg ACE_Managed_Process is just an @arg ACE_Process with an\n * @arg unmanage() method that deletes the instance.\n * This class is only valid for use as a dynamically-allocated object!"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Managed_Process {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Process>,
}
#[repr(C)]
pub struct ACE_Message_Block {
    pub _opaque: [u8; 1],
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
    pub fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, ...) -> libc::c_int;
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Guard_ACE_Recursive_Thread_Mutex_ {
    pub lock_: *mut ACE_Recursive_Thread_Mutex,
    pub owner_: libc::c_int,
}
pub unsafe extern "C-unwind" fn __xtu__ZN9ACE_GuardI26ACE_Recursive_Thread_MutexEC1EPS0_(
    __this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
    __a0: *mut ACE_Recursive_Thread_Mutex,
) {
    ACE_Guard_ACE_Recursive_Thread_Mutex_::new_at_s32ddbf42ad61ad7d(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN9ACE_GuardI26ACE_Recursive_Thread_MutexEC1ERS0_(
    __this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
    __a0: *mut ACE_Recursive_Thread_Mutex,
) {
    ACE_Guard_ACE_Recursive_Thread_Mutex_::new_at_sd3d970b1b01b243f(__this, __a0)
}
impl Drop for ACE_Guard_ACE_Recursive_Thread_Mutex_ {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::release(
                    (__this) as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                );
            }
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_u9910894ba935e451(
    __this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[doc = "* @class ACE_RW_Mutex\n *\n * @brief Wrapper for readers/writer locks.\n *\n * These are most useful for applications that have many more\n * parallel readers than writers..."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_RW_Mutex {
    pub lock_: pthread_rwlock_t,
    pub removed_: bool,
}
impl Drop for ACE_RW_Mutex {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u869c09e87589f1b0"]
                fn __ext(__this: *mut ACE_RW_Mutex);
            }
            __ext(self as *mut Self);
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_GCC_int_ {
    pub value_: libc::c_int,
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIiEC1Ev(
    __this: *mut ACE_Atomic_Op_GCC_int_,
) {
    ACE_Atomic_Op_GCC_int_::new_at_sb2afbc14dc15fb76(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIiEC1Ei(
    __this: *mut ACE_Atomic_Op_GCC_int_,
    __a0: libc::c_int,
) {
    ACE_Atomic_Op_GCC_int_::new_at_sf76c32f2735f2713(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIiEC1ERKS0_(
    __this: *mut ACE_Atomic_Op_GCC_int_,
    __a0: *const ACE_Atomic_Op_GCC_int_,
) {
    ACE_Atomic_Op_GCC_int_::new_at_s3fb4016ff2eb03e7(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_GCC_unsigned_int_ {
    pub value_: libc::c_uint,
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIjEC1Ev(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
) {
    ACE_Atomic_Op_GCC_unsigned_int_::new_at_sb2af9c14dc15c516(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIjEC1ERKS0_(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
    __a0: *const ACE_Atomic_Op_GCC_unsigned_int_,
) {
    ACE_Atomic_Op_GCC_unsigned_int_::new_at_sffc62675d9b85727(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIjEC1Ej(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
    __a0: libc::c_uint,
) {
    ACE_Atomic_Op_GCC_unsigned_int_::new_at_sb1d76ea393b6b1ee(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_GCC_long_ {
    pub value_: libc::c_long,
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIlEC1Ev(
    __this: *mut ACE_Atomic_Op_GCC_long_,
) {
    ACE_Atomic_Op_GCC_long_::new_at_sb2afc114dc1603f5(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIlEC1El(
    __this: *mut ACE_Atomic_Op_GCC_long_,
    __a0: libc::c_long,
) {
    ACE_Atomic_Op_GCC_long_::new_at_sef6195b741b75f97(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIlEC1ERKS0_(
    __this: *mut ACE_Atomic_Op_GCC_long_,
    __a0: *const ACE_Atomic_Op_GCC_long_,
) {
    ACE_Atomic_Op_GCC_long_::new_at_s06c4fe3ad40cd3fb(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_GCC_unsigned_long_ {
    pub value_: libc::c_ulong,
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCImEC1Ev(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
) {
    ACE_Atomic_Op_GCC_unsigned_long_::new_at_sb2afa114dc15cd95(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCImEC1Em(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
    __a0: libc::c_ulong,
) {
    ACE_Atomic_Op_GCC_unsigned_long_::new_at_s801634c3aabc0cfe(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCImEC1ERKS0_(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
    __a0: *const ACE_Atomic_Op_GCC_unsigned_long_,
) {
    ACE_Atomic_Op_GCC_unsigned_long_::new_at_scf5b98a64f65a4fb(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_GCC_long_long_ {
    pub value_: libc::c_longlong,
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIxEC1Ev(
    __this: *mut ACE_Atomic_Op_GCC_long_long_,
) {
    ACE_Atomic_Op_GCC_long_long_::new_at_sb2afba14dc15f810(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIxEC1Ex(
    __this: *mut ACE_Atomic_Op_GCC_long_long_,
    __a0: libc::c_longlong,
) {
    ACE_Atomic_Op_GCC_long_long_::new_at_s8ac76eb38ea25f2e(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIxEC1ERKS0_(
    __this: *mut ACE_Atomic_Op_GCC_long_long_,
    __a0: *const ACE_Atomic_Op_GCC_long_long_,
) {
    ACE_Atomic_Op_GCC_long_long_::new_at_s64b54c651f3e4c5f(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_GCC_unsigned_long_long_ {
    pub value_: libc::c_ulonglong,
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIyEC1Ev(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
) {
    ACE_Atomic_Op_GCC_unsigned_long_long_::new_at_sb2af9a14dc15c1b0(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIyEC1Ey(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
    __a0: libc::c_ulonglong,
) {
    ACE_Atomic_Op_GCC_unsigned_long_long_::new_at_se5ec53498d612b13(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIyEC1ERKS0_(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
    __a0: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
) {
    ACE_Atomic_Op_GCC_unsigned_long_long_::new_at_sb631fd26c277959f(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_GCC_short_ {
    pub value_: libc::c_short,
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIsEC1Ev(
    __this: *mut ACE_Atomic_Op_GCC_short_,
) {
    ACE_Atomic_Op_GCC_short_::new_at_sb2afb214dc15ea78(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIsEC1Es(
    __this: *mut ACE_Atomic_Op_GCC_short_,
    __a0: libc::c_short,
) {
    ACE_Atomic_Op_GCC_short_::new_at_s946653175f1c528b(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIsEC1ERKS0_(
    __this: *mut ACE_Atomic_Op_GCC_short_,
    __a0: *const ACE_Atomic_Op_GCC_short_,
) {
    ACE_Atomic_Op_GCC_short_::new_at_s42f6630c2645225f(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_GCC_unsigned_short_ {
    pub value_: libc::c_ushort,
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCItEC1Ev(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
) {
    ACE_Atomic_Op_GCC_unsigned_short_::new_at_sb2af9214dc15b418(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCItEC1Et(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
    __a0: libc::c_ushort,
) {
    ACE_Atomic_Op_GCC_unsigned_short_::new_at_s878cc61f3e019dca(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCItEC1ERKS0_(
    __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
    __a0: *const ACE_Atomic_Op_GCC_unsigned_short_,
) {
    ACE_Atomic_Op_GCC_unsigned_short_::new_at_s71cb7029d33cb79f(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_GCC_bool_ {
    pub value_: bool,
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIbEC1Ev(
    __this: *mut ACE_Atomic_Op_GCC_bool_,
) {
    ACE_Atomic_Op_GCC_bool_::new_at_sb2afa314dc15d0fb(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIbEC1Eb(
    __this: *mut ACE_Atomic_Op_GCC_bool_,
    __a0: bool,
) {
    ACE_Atomic_Op_GCC_bool_::new_at_sc5ccefaa3980372a(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN17ACE_Atomic_Op_GCCIbEC1ERKS0_(
    __this: *mut ACE_Atomic_Op_GCC_bool_,
    __a0: *const ACE_Atomic_Op_GCC_bool_,
) {
    ACE_Atomic_Op_GCC_bool_::new_at_sa2856b7b2f8711f3(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_ACE_Thread_Mutex__int_ {
    pub __base_0: ACE_Atomic_Op_GCC_int_,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexiEC1Ev(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__int_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__int_::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexiEC1Ei(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__int_,
    __a0: libc::c_int,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__int_::new_at_u52259338b7dfec84(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexiEC1ERKS1_(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__int_,
    __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__int_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__int_::new_at_u39a5191e90dd80e9(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_ {
    pub __base_0: ACE_Atomic_Op_GCC_unsigned_int_,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexjEC1Ev(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexjEC1Ej(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_,
    __a0: libc::c_uint,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_::new_at_ufefa9eb56a8f4964(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexjEC1ERKS1_(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_,
    __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_::new_at_u4525c24972278e49(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_ACE_Thread_Mutex__long_ {
    pub __base_0: ACE_Atomic_Op_GCC_long_,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexlEC1Ev(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__long_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__long_::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexlEC1El(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__long_,
    __a0: libc::c_long,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__long_::new_at_u3fc757b24fef6c3f(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexlEC1ERKS1_(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__long_,
    __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__long_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__long_::new_at_u0822bd45516f190a(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_ {
    pub __base_0: ACE_Atomic_Op_GCC_unsigned_long_,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexmEC1Ev(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexmEC1Em(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_,
    __a0: libc::c_ulong,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_::new_at_u8abafd19dfaf07df(
        __this,
        __a0,
    )
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexmEC1ERKS1_(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_,
    __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_::new_at_ueb6cc6249a36382a(
        __this,
        __a0,
    )
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_ACE_Thread_Mutex__long_long_ {
    pub __base_0: ACE_Atomic_Op_GCC_long_long_,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexxEC1Ev(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__long_long_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__long_long_::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexxEC1Ex(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__long_long_,
    __a0: libc::c_longlong,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__long_long_::new_at_u796ec9126c443db6(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexxEC1ERKS1_(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__long_long_,
    __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__long_long_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__long_long_::new_at_u1994d2c095adb793(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_ {
    pub __base_0: ACE_Atomic_Op_GCC_unsigned_long_long_,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexyEC1Ev(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexyEC1Ey(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_,
    __a0: libc::c_ulonglong,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_::new_at_ud3a33d3ab7354316(
        __this,
        __a0,
    )
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexyEC1ERKS1_(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_,
    __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_::new_at_u85cc3b07e52f9b73(
        __this,
        __a0,
    )
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_ACE_Thread_Mutex__short_ {
    pub __base_0: ACE_Atomic_Op_GCC_short_,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexsEC1Ev(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__short_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__short_::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexsEC1Es(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__short_,
    __a0: libc::c_short,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__short_::new_at_u8af4ddea024eb19e(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexsEC1ERKS1_(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__short_,
    __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__short_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__short_::new_at_uef9241b52e0b8deb(__this, __a0)
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_ {
    pub __base_0: ACE_Atomic_Op_GCC_unsigned_short_,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutextEC1Ev(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutextEC1Et(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_,
    __a0: libc::c_ushort,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_::new_at_u11010cccb631acfe(
        __this,
        __a0,
    )
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutextEC1ERKS1_(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_,
    __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_::new_at_u137675b7e074ff4b(
        __this,
        __a0,
    )
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Atomic_Op_ACE_Thread_Mutex__bool_ {
    pub __base_0: ACE_Atomic_Op_GCC_bool_,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexbEC1Ev(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__bool_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__bool_::new_at(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexbEC1Eb(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__bool_,
    __a0: bool,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__bool_::new_at_u9396a416a0964f25(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Atomic_OpI16ACE_Thread_MutexbEC1ERKS1_(
    __this: *mut ACE_Atomic_Op_ACE_Thread_Mutex__bool_,
    __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__bool_,
) {
    ACE_Atomic_Op_ACE_Thread_Mutex__bool_::new_at_ua42208eb4017b900(__this, __a0)
}
#[doc = "* You can specify a hook function to event-handling methods that will\n   * be called after each iteration of event handling.  If the hook function\n   * returns a non-zero value, the event loop will immediately resume\n   * waiting for the next event(s) to process without checking the error\n   * status of the just-completed iteration of event handling or the\n   * end-of-loop indication. If the hook function returns 0, the event\n   * handling error status and the end-of-loop indication will be checked\n   * as normal, just as if there is no hook function specified."]
pub type ACE_Reactor_REACTOR_EVENT_HOOK = Option<
    unsafe extern "C-unwind" fn(*mut ACE_Reactor) -> libc::c_int,
>;
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Reactor_reactor_: *mut ACE_Reactor;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Reactor_delete_reactor_: bool;
}
#[doc = "* @class ACE_Reactor\n *\n * @brief This class forwards all methods to its delegation/implementation class, e.g.,\n * ACE_Select_Reactor or ACE_WFMO_Reactor."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Reactor {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Reactor_Timer_Interface>,
    pub implementation_: *mut ACE_Reactor_Impl,
    pub delete_implementation_: bool,
}
#[repr(C)]
pub struct ACE_Thread_Manager {
    pub _opaque: [u8; 1],
}
///Typedef for implementation of reference counting.
pub type ACE_Event_Handler_Atomic_Reference_Count = ACE_Atomic_Op_ACE_Thread_Mutex__long_;
#[doc = "* @class ACE_Event_Handler\n *\n * @brief\n * Provides an abstract interface for handling various types of\n * I/O, timer, and signal events.\n *\n * Subclasses read/write input/output on an I/O descriptor,\n * handle an exception raised on an I/O descriptor, handle a\n * timer's expiration, or handle a signal."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Event_Handler {
    pub vptr: *const (),
    pub reference_count_: ACE_Atomic_Op_ACE_Thread_Mutex__long_,
    pub priority_: libc::c_int,
    pub reactor_: *mut ACE_Reactor,
    pub reference_counting_policy_: ::core::mem::ManuallyDrop<Reference_Counting_Policy>,
}
#[doc = "* @class ACE_Event_Handler_var\n *\n * @brief Auto pointer like class for Event Handlers.\n *\n * Used to manage lifecycle of handlers. This class calls\n * ACE_Event_Handler::remove_reference() in its destructor."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Event_Handler_var {
    pub ptr_: *mut ACE_Event_Handler,
}
impl Drop for ACE_Event_Handler_var {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_ubaaade5edda792fc"]
                fn __ext(__this: *mut ACE_Event_Handler_var);
            }
            __ext(self as *mut Self);
        }
    }
}
#[doc = "* @class ACE_Notification_Buffer\n *\n * @brief Simple wrapper for passing ACE_Event_Handler *s and\n * ACE_Reactor_Masks between threads."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Notification_Buffer {
    pub eh_: *mut ACE_Event_Handler,
    pub mask_: libc::c_ulong,
}
impl Drop for ACE_Notification_Buffer {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {}
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_u70c7d2df3655f7df(
    __this: *mut ACE_Notification_Buffer,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[doc = "* @struct Process_Descriptor\n   *\n   * @internal This struct is for internal use only by ACE_Process_Manager.\n   *\n   * @brief Information describing each process that's controlled by an\n   * ACE_Process_Manager."]
#[repr(C)]
#[derive(Clone)]
pub struct Process_Descriptor {
    pub process_: *mut ACE_Process,
    pub exit_notify_: *mut ACE_Event_Handler,
}
#[export_name = "_ZN19ACE_Process_Manager18Process_DescriptorC1Ev"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager18Process_DescriptorC1Ev(
    __this: *mut Process_Descriptor,
) {
    Process_Descriptor::new_at(__this)
}
impl Drop for Process_Descriptor {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {}
            ()
        }
    }
}
#[export_name = "__acedtor_uc3412bfde8007f00"]
pub unsafe extern "C-unwind" fn __acedtor_uc3412bfde8007f00(
    __this: *mut Process_Descriptor,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[export_name = "_ZNK19ACE_Process_Manager18Process_Descriptor4dumpEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK19ACE_Process_Manager18Process_Descriptor4dumpEv(
    __this: *const Process_Descriptor,
) {
    unsafe { Process_Descriptor::dump(__this) }
}
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut ACE_Process_Manager_instance_: *mut ACE_Process_Manager = ((0)
    as *mut ACE_Process_Manager);
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut ACE_Process_Manager_delete_instance_: bool = false;
#[doc = "* @class ACE_Process_Manager\n *\n * @brief Manages a group of processes.\n *\n * This class allows applications to control groups of processes,\n * similar to the way ACE_Thread_Manager controls groups of\n * threads.  Naturally, it doesn't work at all on platforms, such\n * as VxWorks or pSoS, that don't support multiple processes.\n * There are two main ways of using ACE_Process_Manager,\n * depending on how involved you wish to be with the termination\n * of managed processes.  If you want processes to simply\n * go away when they're finished, register the ACE_Process_Manager with\n * an ACE_Reactor that can handle notifications of child process exit:\n * @code\n * ACE_Process_Manager mgr;\n * // ...\n * mgr.open (100, ACE_Reactor::instance ());\n * @endcode\n * In this usage scenario, the ACE_Process_Manager will clean up after any\n * processes that it spawns.  (On Unix, this means executing a\n * wait(2) to collect the exit status and avoid zombie\n * processes; on Win32, it means closing the process and thread\n * HANDLEs that are created when CreateProcess is called.)\n *\n * @note When you register a ACE_Process_Manager with a\n * ACE_Reactor, the reactor's notification pipe is used to help reap the\n * available process exit statuses.  Therefore, you must not use a\n * reactor whose notify pipe has been disabled.  Here's the\n * sequence of steps used to reap the exit statuses in this case:\n * -# The ACE_Process_Manager registers a signal handler for\n *    SIGCHLD.\n * -# The SIGCHLD handler, when invoked, uses the ACE_Reactor's\n *    notify() method to inform the ACE_Reactor to wake up.\n * -# The ACE_Reactor calls the ACE_Process_Manager's\n *    handle_input() method; this happens synchronously, not in\n *    signal context.\n * -# The handle_input() method collects all available exit\n *    statuses.\n *\n * If, on the other hand you want to wait \"in line\" to handle the\n * terminated process cleanup code, call one of the wait functions\n * whenever there might be managed processes that have exited.\n *\n * Note that in either case, ACE_Process_Manager allows you to\n * register an ACE_Event_Handler to be called when a specific\n * spawned process exits, or when any process without a specific\n * ACE_Event_Handler exits.  When a process exits, the\n * appropriate ACE_Event_Handler's handle_input() method is called; the\n * ACE_HANDLE passed is either the process's HANDLE (on Windows),\n * or its pid cast to an ACE_HANDLE (on POSIX).\n * It is also possible to call the wait() functions even when the\n * ACE_Process_Manager is registered with a reactor.\n *\n * @note Be aware that the wait functions are \"sloppy\" on Unix,\n * because there's no good way to wait for a subset of the\n * children of a process.  The wait functions may end up\n * collecting the exit status of a process that's not managed by\n * the ACE_Process_Manager whose wait() you invoked.  It's best to\n * only use a single ACE_Process_Manager, and to create all\n * subprocesses by calling that manager's spawn() method."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Process_Manager {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Event_Handler>,
    pub process_table_: *mut Process_Descriptor,
    pub max_process_table_size_: libc::c_ulong,
    pub current_count_: libc::c_ulong,
    pub default_exit_handler_: *mut ACE_Event_Handler,
    pub lock_: ::core::mem::ManuallyDrop<ACE_Recursive_Thread_Mutex>,
}
#[export_name = "_ZN19ACE_Process_ManagerC1EmP11ACE_Reactor"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_ManagerC1EmP11ACE_Reactor(
    __this: *mut ACE_Process_Manager,
    __a0: libc::c_ulong,
    __a1: *mut ACE_Reactor,
) {
    ACE_Process_Manager::new_at(__this, __a0, __a1)
}
#[export_name = "_ZN19ACE_Process_Manager4openEmP11ACE_Reactor"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager4openEmP11ACE_Reactor(
    __this: *mut ACE_Process_Manager,
    size: libc::c_ulong,
    r: *mut ACE_Reactor,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::open(__this, size, r) }
}
#[export_name = "_ZN19ACE_Process_Manager5closeEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager5closeEv(
    __this: *mut ACE_Process_Manager,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::close(__this) }
}
impl Drop for ACE_Process_Manager {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {
                    <ACE_Process_Manager>::close((__this) as *mut ACE_Process_Manager);
                }
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.lock_);
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
#[export_name = "_ZN19ACE_Process_Manager8instanceEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager8instanceEv() -> *mut ACE_Process_Manager {
    unsafe { ACE_Process_Manager::instance() }
}
#[export_name = "_ZN19ACE_Process_Manager8instanceEPS_"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager8instanceEPS_(
    tm: *mut ACE_Process_Manager,
) -> *mut ACE_Process_Manager {
    unsafe { ACE_Process_Manager::instance_ud8939d75278aebda(tm) }
}
#[export_name = "_ZN19ACE_Process_Manager15close_singletonEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager15close_singletonEv() {
    unsafe { ACE_Process_Manager::close_singleton() }
}
#[export_name = "_ZN19ACE_Process_Manager7cleanupEPvS0_"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager7cleanupEPvS0_(
    _anon_0: *mut libc::c_void,
    _anon_1: *mut libc::c_void,
) {
    unsafe { ACE_Process_Manager::cleanup(_anon_0, _anon_1) }
}
#[export_name = "_ZN19ACE_Process_Manager5spawnEP11ACE_ProcessR19ACE_Process_OptionsP17ACE_Event_Handler"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager5spawnEP11ACE_ProcessR19ACE_Process_OptionsP17ACE_Event_Handler(
    __this: *mut ACE_Process_Manager,
    process: *mut ACE_Process,
    options: *mut ACE_Process_Options,
    event_handler: *mut ACE_Event_Handler,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::spawn(__this, process, options, event_handler) }
}
#[export_name = "_ZN19ACE_Process_Manager5spawnER19ACE_Process_OptionsP17ACE_Event_Handler"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager5spawnER19ACE_Process_OptionsP17ACE_Event_Handler(
    __this: *mut ACE_Process_Manager,
    options: *mut ACE_Process_Options,
    event_handler: *mut ACE_Event_Handler,
) -> libc::c_int {
    unsafe {
        ACE_Process_Manager::spawn_u723e6092e06de6bf(__this, options, event_handler)
    }
}
#[export_name = "_ZN19ACE_Process_Manager7spawn_nEmR19ACE_Process_OptionsPiP17ACE_Event_Handler"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager7spawn_nEmR19ACE_Process_OptionsPiP17ACE_Event_Handler(
    __this: *mut ACE_Process_Manager,
    n: libc::c_ulong,
    options: *mut ACE_Process_Options,
    child_pids: *mut libc::c_int,
    event_handler: *mut ACE_Event_Handler,
) -> libc::c_int {
    unsafe {
        ACE_Process_Manager::spawn_n(__this, n, options, child_pids, event_handler)
    }
}
#[export_name = "_ZN19ACE_Process_Manager9terminateEi"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager9terminateEi(
    __this: *mut ACE_Process_Manager,
    pid: libc::c_int,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::terminate(__this, pid) }
}
#[export_name = "_ZN19ACE_Process_Manager9terminateEii"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager9terminateEii(
    __this: *mut ACE_Process_Manager,
    pid: libc::c_int,
    sig: libc::c_int,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::terminate_u78afa949f27d6dc6(__this, pid, sig) }
}
#[export_name = "_ZN19ACE_Process_Manager4waitERK14ACE_Time_Value"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager4waitERK14ACE_Time_Value(
    __this: *mut ACE_Process_Manager,
    timeout: *const ACE_Time_Value,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::wait(__this, timeout) }
}
#[export_name = "_ZN19ACE_Process_Manager4waitEiRK14ACE_Time_ValuePi"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager4waitEiRK14ACE_Time_ValuePi(
    __this: *mut ACE_Process_Manager,
    pid: libc::c_int,
    timeout: *const ACE_Time_Value,
    status: *mut libc::c_int,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::wait_ud310f8de9a6d598c(__this, pid, timeout, status) }
}
#[export_name = "_ZN19ACE_Process_Manager4waitEiPi"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager4waitEiPi(
    __this: *mut ACE_Process_Manager,
    pid: libc::c_int,
    status: *mut libc::c_int,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::wait_u04054cf0e91bd50a(__this, pid, status) }
}
#[export_name = "_ZN19ACE_Process_Manager16register_handlerEP17ACE_Event_Handleri"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager16register_handlerEP17ACE_Event_Handleri(
    __this: *mut ACE_Process_Manager,
    eh: *mut ACE_Event_Handler,
    pid: libc::c_int,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::register_handler(__this, eh, pid) }
}
#[export_name = "_ZN19ACE_Process_Manager6removeEi"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager6removeEi(
    __this: *mut ACE_Process_Manager,
    pid: libc::c_int,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::remove(__this, pid) }
}
#[export_name = "_ZN19ACE_Process_Manager13set_schedulerERK16ACE_Sched_Paramsi"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager13set_schedulerERK16ACE_Sched_Paramsi(
    __this: *mut ACE_Process_Manager,
    params: *const ACE_Sched_Params,
    pid: libc::c_int,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::set_scheduler(__this, params, pid) }
}
#[export_name = "_ZN19ACE_Process_Manager17set_scheduler_allERK16ACE_Sched_Params"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager17set_scheduler_allERK16ACE_Sched_Params(
    __this: *mut ACE_Process_Manager,
    params: *const ACE_Sched_Params,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::set_scheduler_all(__this, params) }
}
#[export_name = "_ZNK19ACE_Process_Manager4dumpEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK19ACE_Process_Manager4dumpEv(
    __this: *const ACE_Process_Manager,
) {
    unsafe { ACE_Process_Manager::dump(__this) }
}
#[export_name = "_ZN19ACE_Process_Manager12handle_inputEi"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager12handle_inputEi(
    __this: *mut ACE_Process_Manager,
    _anon_0: libc::c_int,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::handle_input(__this, _anon_0) }
}
#[export_name = "_ZN19ACE_Process_Manager12handle_closeEim"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager12handle_closeEim(
    __this: *mut ACE_Process_Manager,
    _anon_0: libc::c_int,
    close_mask: libc::c_ulong,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::handle_close(__this, _anon_0, close_mask) }
}
#[export_name = "_ZN19ACE_Process_Manager13handle_signalEiP9siginfo_tP10ucontext_t"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager13handle_signalEiP9siginfo_tP10ucontext_t(
    __this: *mut ACE_Process_Manager,
    _anon_0: libc::c_int,
    si: *mut siginfo_t,
    _anon_2: *mut ucontext_t,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::handle_signal(__this, _anon_0, si, _anon_2) }
}
#[export_name = "_ZN19ACE_Process_Manager6resizeEm"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager6resizeEm(
    __this: *mut ACE_Process_Manager,
    size: libc::c_ulong,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::resize(__this, size) }
}
#[export_name = "_ZN19ACE_Process_Manager9find_procEi"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager9find_procEi(
    __this: *mut ACE_Process_Manager,
    pid: libc::c_int,
) -> libc::c_long {
    unsafe { ACE_Process_Manager::find_proc(__this, pid) }
}
#[export_name = "_ZN19ACE_Process_Manager11insert_procEP11ACE_ProcessP17ACE_Event_Handler"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager11insert_procEP11ACE_ProcessP17ACE_Event_Handler(
    __this: *mut ACE_Process_Manager,
    proc: *mut ACE_Process,
    event_handler: *mut ACE_Event_Handler,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::insert_proc(__this, proc, event_handler) }
}
#[export_name = "_ZN19ACE_Process_Manager11append_procEP11ACE_ProcessP17ACE_Event_Handler"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager11append_procEP11ACE_ProcessP17ACE_Event_Handler(
    __this: *mut ACE_Process_Manager,
    proc: *mut ACE_Process,
    event_handler: *mut ACE_Event_Handler,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::append_proc(__this, proc, event_handler) }
}
#[export_name = "_ZN19ACE_Process_Manager11remove_procEm"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager11remove_procEm(
    __this: *mut ACE_Process_Manager,
    i: libc::c_ulong,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::remove_proc(__this, i) }
}
#[export_name = "_ZN19ACE_Process_Manager19notify_proc_handlerEmi"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ACE_Process_Manager19notify_proc_handlerEmi(
    __this: *mut ACE_Process_Manager,
    i: libc::c_ulong,
    exit_code: libc::c_int,
) -> libc::c_int {
    unsafe { ACE_Process_Manager::notify_proc_handler(__this, i, exit_code) }
}
#[doc = "* @class ACE_Sig_Set\n *\n * @brief Provide a C++ wrapper for the C sigset_t interface.\n *\n * Handle signals via a more elegant C++ interface (e.g.,\n * doesn't require the use of global variables or global\n * functions in an application)."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Sig_Set {
    pub sigset_: __sigset_t,
}
pub unsafe extern "C-unwind" fn __xtu__ZN11ACE_Sig_SetC1EP10__sigset_t(
    __this: *mut ACE_Sig_Set,
    __a0: *mut __sigset_t,
) {
    ACE_Sig_Set::new_at(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN11ACE_Sig_SetC1EPS_(
    __this: *mut ACE_Sig_Set,
    __a0: *mut ACE_Sig_Set,
) {
    ACE_Sig_Set::new_at_u74fa6d1f47e29b13(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN11ACE_Sig_SetC1Ei(
    __this: *mut ACE_Sig_Set,
    __a0: libc::c_int,
) {
    ACE_Sig_Set::new_at_ud60a876b80356237(__this, __a0)
}
impl Drop for ACE_Sig_Set {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u6c78bee3fdbb2127"]
                fn __ext(__this: *mut ACE_Sig_Set);
            }
            __ext(self as *mut Self);
        }
    }
}
#[doc = "* @class ACE_Sig_Action\n *\n * @brief C++ wrapper facade for the @c sigaction struct."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Sig_Action {
    pub sa_: sigaction,
}
impl Drop for ACE_Sig_Action {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u2f1f30f4f5bd2233"]
                fn __ext(__this: *mut ACE_Sig_Action);
            }
            __ext(self as *mut Self);
        }
    }
}
#[doc = "* @class ACE_Sig_Guard\n *\n * @brief Hold signals in MASK for duration of a C++ statement block.\n * Note that a \"0\" for mask causes all signals to be held."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Sig_Guard {
    pub omask_: ::core::mem::ManuallyDrop<ACE_Sig_Set>,
    pub condition_: bool,
}
pub unsafe extern "C-unwind" fn __xtu__ZN13ACE_Sig_GuardC1EP11ACE_Sig_Setb(
    __this: *mut ACE_Sig_Guard,
    __a0: *mut ACE_Sig_Set,
    __a1: bool,
) {
    ACE_Sig_Guard::new_at(__this, __a0, __a1)
}
impl Drop for ACE_Sig_Guard {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_ue2d33af8f6a80e7a"]
                fn __ext(__this: *mut ACE_Sig_Guard);
            }
            __ext(self as *mut Self);
        }
    }
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Trace_nesting_indent_: libc::c_int;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Trace_enable_tracing_: bool;
}
#[doc = "* @class ACE_Trace\n *\n * @brief A C++ trace facility that keeps track of which methods are\n * entered and exited.\n *\n * This class uses C++ constructors and destructors to automate\n * the ACE_Trace nesting.  In addition, thread-specific storage\n * is used to enable multiple threads to work correctly."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Trace {
    pub name_: *const libc::c_char,
}
impl Drop for ACE_Trace {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u5438f1796abb9d58"]
                fn __ext(__this: *mut ACE_Trace);
            }
            __ext(self as *mut Self);
        }
    }
}
#[repr(C)]
pub struct ACE_Object_Manager_Preallocations {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Sig_Adapter {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Mutex {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Cleanup_Adapter_ACE_Recursive_Thread_Mutex_ {
    pub _opaque: [u8; 1],
}
#[doc = "* @class ACE_Static_Object_Lock\n *\n * @brief Provide an interface to access a global lock.\n *\n * This class is used to serialize the creation of static\n * singleton objects.  It really isn't needed any more, because\n * anyone can access ACE_STATIC_OBJECT_LOCK directly.  But, it\n * is retained for backward compatibility."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Static_Object_Lock {}
#[repr(C)]
pub struct ACE_Service_Object {
    pub _opaque: [u8; 1],
}
extern "C-unwind" {
    pub fn _make_ACE_Service_Manager(
        _anon_0: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut ACE_Service_Object;
}
#[repr(C)]
pub struct ACE_Abstract_Timer_Queue_ACE_Event_Handler___ {
    pub _opaque: [u8; 1],
}
pub type ACE_Timer_Queue = ACE_Abstract_Timer_Queue_ACE_Event_Handler___;
#[repr(C)]
pub struct ACE_Sig_Handler {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Time_Value_T_ACE_HR_Time_Policy_ {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Time_Value>,
    pub time_policy_: ACE_HR_Time_Policy,
}
impl Clone for ACE_Time_Value_T_ACE_HR_Time_Policy_ {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { Self::new_s9f868f6e6111a33b(self as *const Self) }
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEC1Ev(
    __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
) {
    ACE_Time_Value_T_ACE_HR_Time_Policy_::new_at_s45a5d1d1a3b95282(__this)
}
pub unsafe extern "C-unwind" fn __xtu__ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEC1Ell(
    __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
    __a0: libc::c_long,
    __a1: libc::c_long,
) {
    ACE_Time_Value_T_ACE_HR_Time_Policy_::new_at_s57a0e694bb633ba2(__this, __a0, __a1)
}
pub unsafe extern "C-unwind" fn __xtu__ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEC1ERK7timeval(
    __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
    __a0: *const timeval,
) {
    ACE_Time_Value_T_ACE_HR_Time_Policy_::new_at_scf346aaab86c6e8a(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEC1ERK8timespec(
    __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
    __a0: *const timespec,
) {
    ACE_Time_Value_T_ACE_HR_Time_Policy_::new_at_sdfbb0bed66ce6a8c(__this, __a0)
}
impl Drop for ACE_Time_Value_T_ACE_HR_Time_Policy_ {
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
pub unsafe extern "C-unwind" fn __xtu__ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEC1ERK14ACE_Time_ValueRKS0_(
    __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
    __a0: *const ACE_Time_Value,
    __a1: *const ACE_HR_Time_Policy,
) {
    ACE_Time_Value_T_ACE_HR_Time_Policy_::new_at_s1cc0ae66d2e0ee48(__this, __a0, __a1)
}
pub unsafe extern "C-unwind" fn __xtu__ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEC1ERKS1_(
    __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
    __a0: *const ACE_Time_Value_T_ACE_HR_Time_Policy_,
) {
    ACE_Time_Value_T_ACE_HR_Time_Policy_::new_at_s9f868f6e6111a33b(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZNK16ACE_Time_Value_TI18ACE_HR_Time_PolicyE9duplicateEv(
    __this: *const ACE_Time_Value_T_ACE_HR_Time_Policy_,
) -> *mut ACE_Time_Value {
    unsafe { ACE_Time_Value_T_ACE_HR_Time_Policy_::duplicate(__this) }
}
pub unsafe extern "C-unwind" fn __xtu__ZNK16ACE_Time_Value_TI18ACE_HR_Time_PolicyE3nowEv(
    __this: *const ACE_Time_Value_T_ACE_HR_Time_Policy_,
) -> ACE_Time_Value {
    unsafe { ACE_Time_Value_T_ACE_HR_Time_Policy_::now(__this) }
}
pub unsafe extern "C-unwind" fn __xtu__ZNK16ACE_Time_Value_TI18ACE_HR_Time_PolicyE16to_absolute_timeEv(
    __this: *const ACE_Time_Value_T_ACE_HR_Time_Policy_,
) -> ACE_Time_Value {
    unsafe { ACE_Time_Value_T_ACE_HR_Time_Policy_::to_absolute_time(__this) }
}
pub unsafe extern "C-unwind" fn __xtu__ZNK16ACE_Time_Value_TI18ACE_HR_Time_PolicyE16to_relative_timeEv(
    __this: *const ACE_Time_Value_T_ACE_HR_Time_Policy_,
) -> ACE_Time_Value {
    unsafe { ACE_Time_Value_T_ACE_HR_Time_Policy_::to_relative_time(__this) }
}
#[doc = "* @class ACE_HR_Time_Policy\n *\n * @brief Implement a time policy based on the ACE Highres timer."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_HR_Time_Policy {}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_High_Res_Timer_global_scale_factor_: libc::c_uint;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_High_Res_Timer_global_scale_factor_status_: libc::c_int;
}
#[doc = "* @class ACE_High_Res_Timer\n *\n * @brief A high resolution timer class wrapper that encapsulates\n * OS-specific high-resolution timers, such as those found on\n * Solaris, AIX, Win32/Pentium, and VxWorks.\n *\n * Most of the member functions don't return values.  The only\n * reason that one would fail is if high-resolution time isn't\n * supported on the platform.  To avoid impacting performance\n * and complicating the interface, in that case,\n * <ACE_OS::gettimeofday> is used instead.\n * The global scale factor is required for platforms that have\n * high-resolution timers that return units other than\n * microseconds, such as clock ticks.  It is represented as a\n * static u_long, can only be accessed through static methods,\n * and is used by all instances of High Res Timer.  The member\n * functions that return or print times use the global scale\n * factor.  They divide the \"time\" that they get from\n * <ACE_OS::gethrtime> by global_scale_factor_ to obtain the\n * time in microseconds.  Its units are therefore 1/microsecond.\n * On Windows the global_scale_factor_ units are 1/millisecond.\n * There's a macro <ACE_HR_SCALE_CONVERSION> which gives the\n * units/second.  Because it's possible that the units/second\n * changes in the future, it's recommended to use it instead\n * of a \"hard coded\" solution.\n * Dependent on the platform and used class members, there's a\n * maximum elapsed period before overflow (which is not checked).\n * Look at the documentation with some members functions.\n * On some (most?) implementations it's not recommended to measure\n * \"long\" timeperiods, because the error's can accumulate fast.\n * This is probably not a problem profiling code, but could be\n * on if the high resolution timer class is used to initiate\n * actions after a \"long\" timeout.\n * On Solaris, a scale factor of 1000 should be used because its\n * high-resolution timer returns nanoseconds.  However, on Intel\n * platforms, we use RDTSC which returns the number of clock\n * ticks since system boot.  For a 200MHz cpu, each clock tick\n * is 1/200 of a microsecond; the global_scale_factor_ should\n * therefore be 200 or 200000 if it's in unit/millisecond.\n * On Windows ::QueryPerformanceCounter() is used, which can be a\n * different implementation depending on the used windows HAL\n * (Hardware Abstraction Layer).  On some it uses the PC \"timer chip\"\n * while it uses RDTSC on others.\n * @note The elapsed time calculations in the print methods use\n * ACE_hrtime_t values.  Those methods do _not_ check for overflow!\n * @note Gabe <begeddov@proaxis.com> raises this issue regarding\n * <ACE_OS::gethrtime>: on multi-processors, the processor that\n * you query for your @c timer.stop() value might not be the one\n * you queried for @c timer.start().  Its not clear how much\n * divergence there would be, if any.\n * This issue is not mentioned in the Solaris 2.5.1 gethrtime\n * man page.\n * A RDTSC NOTE: RDTSC is the Intel Pentium read-time stamp counter\n * and is actualy a 64 bit clock cycle counter, which is increased\n * with every cycle.  It has a low overhead and can be read within\n * 16 (pentium) or 32 (pentium II,III,...) cycles, but it doesn't\n * serialize the processor, which could give wrong timings when\n * profiling very short code fragments.\n * Problematic is that some power sensitive devices\n * (laptops for example, but probably also embedded devices),\n * do change the cycle rate while running.\n * Some pentiums can run on (at least) two clock frequency's.\n * Another problem arises with multiprocessor computers, there\n * are reports that the different RDTSC's are not always kept\n * in sync.\n * A windows \"timer chip\" NOTE: (8254-compatible real-time clock)\n * When ::QueryPerformanceCounter() uses the 8254 it has a\n * frequency off about 1.193 Mhz (or sometimes 3.579 Mhz?) and\n * reading it requires some time (several thousand cycles)."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_High_Res_Timer {
    pub start_: libc::c_ulong,
    pub end_: libc::c_ulong,
    pub total_: libc::c_ulong,
    pub start_incr_: libc::c_ulong,
}
impl Drop for ACE_High_Res_Timer {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {}
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_u8111749324abfcb8(
    __this: *mut ACE_High_Res_Timer,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[doc = "* @class ACE_Reactor_Notify\n *\n * @internal This class is for ACE internal use only.\n *\n * @brief Abstract class for unblocking an ACE_Reactor_Impl from its\n * event loop."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Reactor_Notify {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Event_Handler>,
}
impl Drop for ACE_Reactor_Notify {
    fn drop(&mut self) {
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
pub mod __cxxabiv1 {}
pub(crate) unsafe extern "C-unwind" fn sigchld_nop_u6eab7901cba1c161(
    mut _anon_0: libc::c_int,
    mut _anon_1: *mut siginfo_t,
    mut _anon_2: *mut ucontext_t,
) {
    unsafe {
        {
            return;
        }
        ()
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
    pub fn __builtin_va_start(_anon_0: ::core::ffi::VaList<'_>, ...);
}
extern "C-unwind" {
    pub fn __builtin_va_end(_anon_0: ::core::ffi::VaList<'_>);
}
extern "C-unwind" {
    pub fn __builtin_memcpy(
        _anon_0: *mut libc::c_void,
        _anon_1: *const libc::c_void,
        _anon_2: libc::c_ulong,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn __builtin_strlen(_anon_0: *const libc::c_char) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __builtin_strcmp(
        _anon_0: *const libc::c_char,
        _anon_1: *const libc::c_char,
    ) -> libc::c_int;
}
impl ACE_Thread_Mutex {
    ///Constructor.
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *mut pthread_mutexattr_t,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Thread_MutexC1EPKcP19pthread_mutexattr_t"]
            fn __ext(
                __this: *mut ACE_Thread_Mutex,
                __a0: *const libc::c_char,
                __a1: *mut pthread_mutexattr_t,
            );
        }
        __ext(__this as *mut ACE_Thread_Mutex, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: *mut pthread_mutexattr_t,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    #[doc = "* Explicitly destroy the mutex.  Note that only one thread should\n   * call this method since it doesn't protect against race\n   * conditions."]
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut result: libc::c_int = 0;
                if (((!((((*__this).removed_ as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    (*__this).removed_ = true;
                    result = ACE_OS::thread_mutex_destroy(
                        ::core::ptr::addr_of_mut!((* __this).lock_)
                            as *mut pthread_mutex_t,
                    );
                }
                return result;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Acquire lock ownership (wait on queue if necessary).
    pub unsafe fn acquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::thread_mutex_lock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Block the thread until we acquire the mutex or until @a tv times\n   * out, in which case -1 is returned with @c errno == @c ETIME.  Note\n   * that @a tv is assumed to be in \"absolute\" rather than \"relative\"\n   * time.  The value of @a tv is updated upon return to show the\n   * actual (absolute) acquisition time."]
    pub unsafe fn acquire_ufe375f1dc7d7f248(
        __this: *mut Self,
        mut tv: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::thread_mutex_lock_u60ab414c30bff129(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                    ::core::ptr::addr_of!((* tv)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* If @a tv == 0 the call acquire() directly.  Otherwise, Block the\n   * thread until we acquire the mutex or until @a tv times out, in\n   * which case -1 is returned with @c errno == @c ETIME.  Note that\n   * @a tv is assumed to be in \"absolute\" rather than \"relative\" time.\n   * The value of @a tv is updated upon return to show the actual\n   * (absolute) acquisition time."]
    pub unsafe fn acquire_u9431b8dec6deac34(
        __this: *mut Self,
        mut tv: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::thread_mutex_lock_u00620e1377f06cf5(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                    (tv) as *const ACE_Time_Value,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Conditionally acquire lock (i.e., don't wait on queue).  Returns\n   * -1 on failure.  If we \"failed\" because someone else already had\n   * the lock, @c errno is set to @c EBUSY."]
    pub unsafe fn tryacquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::thread_mutex_trylock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Release lock and unblock a thread at head of queue.
    pub unsafe fn release(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::thread_mutex_unlock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Acquire mutex ownership.  This calls acquire() and is only here\n   * to make the ACE_Thread_Mutex interface consistent with the\n   * other synchronization APIs."]
    pub unsafe fn acquire_read(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::thread_mutex_lock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Acquire mutex ownership.  This calls acquire() and is only here\n   * to make the ACE_Thread_Mutex interface consistent with the\n   * other synchronization APIs."]
    pub unsafe fn acquire_write(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::thread_mutex_lock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Conditionally acquire mutex (i.e., won't block).  This calls\n   * tryacquire() and is only here to make the ACE_Thread_Mutex\n   * interface consistent with the other synchronization APIs.\n   * Returns -1 on failure.  If we \"failed\" because someone else\n   * already had the lock, @c errno is set to @c EBUSY."]
    pub unsafe fn tryacquire_read(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::thread_mutex_trylock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Conditionally acquire mutex (i.e., won't block).  This calls\n   * tryacquire() and is only here to make the ACE_Thread_Mutex\n   * interface consistent with the other synchronization APIs.\n   * Returns -1 on failure.  If we \"failed\" because someone else\n   * already had the lock, @c errno is set to @c EBUSY."]
    pub unsafe fn tryacquire_write(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::thread_mutex_trylock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* This is only here to make the ACE_Thread_Mutex interface\n   * consistent with the other synchronization APIs.  Assumes the\n   * caller has already acquired the mutex using one of the above\n   * calls, and returns 0 (success) always."]
    pub unsafe fn tryacquire_write_upgrade(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the underlying mutex.
    pub unsafe fn lock(__this: *const Self) -> *const pthread_mutex_t {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of!((* __this).lock_);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn lock_uc8df6ef02603782a(__this: *mut Self) -> *mut pthread_mutex_t {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of_mut!((* __this).lock_);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK16ACE_Thread_Mutex4dumpEv"]
            fn __ext(__this: *const ACE_Thread_Mutex);
        }
        __ext(__this as *const ACE_Thread_Mutex)
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Thread_Mutex,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Thread_MutexaSERKS_"]
            fn __ext(__this: *mut ACE_Thread_Mutex, _anon_0: *const ACE_Thread_Mutex);
        }
        __ext(__this as *mut ACE_Thread_Mutex, _anon_0)
    }
    pub unsafe fn new_at_u31bd3c546db061f2(
        __this: *mut Self,
        mut __a0: *const ACE_Thread_Mutex,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Thread_MutexC1ERKS_"]
            fn __ext(__this: *mut ACE_Thread_Mutex, __a0: *const ACE_Thread_Mutex);
        }
        __ext(__this as *mut ACE_Thread_Mutex, __a0)
    }
    pub unsafe fn new_u31bd3c546db061f2(mut __a0: *const ACE_Thread_Mutex) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u31bd3c546db061f2(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_Recursive_Thread_Mutex {
    ///Initialize a recursive mutex.
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *mut pthread_mutexattr_t,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN26ACE_Recursive_Thread_MutexC1EPKcP19pthread_mutexattr_t"]
            fn __ext(
                __this: *mut ACE_Recursive_Thread_Mutex,
                __a0: *const libc::c_char,
                __a1: *mut pthread_mutexattr_t,
            );
        }
        __ext(__this as *mut ACE_Recursive_Thread_Mutex, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: *mut pthread_mutexattr_t,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    #[doc = "* Implicitly release a recursive mutex.  Note that only one thread\n   * should call this method since it doesn't protect against race\n   * conditions."]
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN26ACE_Recursive_Thread_Mutex6removeEv"]
            fn __ext(__this: *mut ACE_Recursive_Thread_Mutex) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Recursive_Thread_Mutex)
    }
    #[doc = "* Acquire a recursive mutex (will increment the nesting level and\n   * not deadmutex if the owner of the mutex calls this method more\n   * than once)."]
    pub unsafe fn acquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::recursive_mutex_lock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Block the thread until we acquire the mutex or until @a tv times\n   * out, in which case -1 is returned with @c errno == @c ETIME.  Note\n   * that @a tv is assumed to be in \"absolute\" rather than \"relative\"\n   * time.  The value of @a tv is updated upon return to show the\n   * actual (absolute) acquisition time."]
    pub unsafe fn acquire_u8121ae102d8a5810(
        __this: *mut Self,
        mut tv: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::recursive_mutex_lock_ub8a8d00bad55dd79(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                    ::core::ptr::addr_of!((* tv)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* If @a tv == 0 the call acquire() directly.  Otherwise, Block the\n   * thread until we acquire the mutex or until @a tv times out, in\n   * which case -1 is returned with @c errno == @c ETIME.  Note that\n   * <*tv> is assumed to be in \"absolute\" rather than \"relative\" time.\n   * The value of <*tv> is updated upon return to show the actual\n   * (absolute) acquisition time."]
    pub unsafe fn acquire_u3a48565ec0e1150c(
        __this: *mut Self,
        mut tv: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::recursive_mutex_lock_ub73fefaba0bdcd05(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                    (tv) as *const ACE_Time_Value,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Conditionally acquire a recursive mutex (i.e., won't block).\n   * Returns -1 on failure.  If we \"failed\" because someone else\n   * already had the lock, @c errno is set to @c EBUSY."]
    pub unsafe fn tryacquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::recursive_mutex_trylock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Acquire mutex ownership.  This calls acquire() and is only\n   * here to make the ACE_Recusive_Thread_Mutex interface consistent\n   * with the other synchronization APIs."]
    pub unsafe fn acquire_read(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Recursive_Thread_Mutex>::acquire(
                    (__this) as *mut ACE_Recursive_Thread_Mutex,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Acquire mutex ownership.  This calls acquire() and is only\n   * here to make the ACE_Recusive_Thread_Mutex interface consistent\n   * with the other synchronization APIs."]
    pub unsafe fn acquire_write(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Recursive_Thread_Mutex>::acquire(
                    (__this) as *mut ACE_Recursive_Thread_Mutex,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Conditionally acquire mutex (i.e., won't block).  This calls\n   * tryacquire() and is only here to make the\n   * ACE_Recusive_Thread_Mutex interface consistent with the other\n   * synchronization APIs.  Returns -1 on failure.  If we \"failed\"\n   * because someone else already had the lock, @c errno is set to\n   * @c EBUSY."]
    pub unsafe fn tryacquire_read(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Recursive_Thread_Mutex>::tryacquire(
                    (__this) as *mut ACE_Recursive_Thread_Mutex,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Conditionally acquire mutex (i.e., won't block).  This calls\n   * tryacquire() and is only here to make the\n   * ACE_Recusive_Thread_Mutex interface consistent with the other\n   * synchronization APIs.  Returns -1 on failure.  If we \"failed\"\n   * because someone else already had the lock, @c errno is set to\n   * @c EBUSY."]
    pub unsafe fn tryacquire_write(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Recursive_Thread_Mutex>::tryacquire(
                    (__this) as *mut ACE_Recursive_Thread_Mutex,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* This is only here to make the ACE_Recursive_Thread_Mutex\n   * interface consistent with the other synchronization APIs.\n   * Assumes the caller has already acquired the mutex using one of\n   * the above calls, and returns 0 (success) always."]
    pub unsafe fn tryacquire_write_upgrade(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Releases a recursive mutex (will not release mutex until all the\n   * nesting level drops to 0, which means the mutex is no longer\n   * held)."]
    pub unsafe fn release(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::recursive_mutex_unlock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_mutex_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the id of the thread that currently owns the mutex.
    pub unsafe fn get_thread_id(__this: *mut Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN26ACE_Recursive_Thread_Mutex13get_thread_idEv"]
            fn __ext(__this: *mut ACE_Recursive_Thread_Mutex) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Recursive_Thread_Mutex)
    }
    #[doc = "* Return the nesting level of the recursion.  When a thread has\n   * acquired the mutex for the first time, the nesting level == 1.\n   * The nesting level is incremented every time the thread acquires\n   * the mutex recursively.  Note that if the ACE_HAS_RECURSIVE_MUTEXES\n   * macro is enabled then this method may return -1 on platforms that\n   * do not expose the internal count."]
    pub unsafe fn get_nesting_level(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN26ACE_Recursive_Thread_Mutex17get_nesting_levelEv"]
            fn __ext(__this: *mut ACE_Recursive_Thread_Mutex) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Recursive_Thread_Mutex)
    }
    ///Returns a reference to the recursive mutex;
    pub unsafe fn lock(__this: *mut Self) -> *mut pthread_mutex_t {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of_mut!((* __this).lock_);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Returns a reference to the recursive mutex's internal mutex;
    pub unsafe fn get_nesting_mutex(__this: *mut Self) -> *mut pthread_mutex_t {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of_mut!((* __this).lock_);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK26ACE_Recursive_Thread_Mutex4dumpEv"]
            fn __ext(__this: *const ACE_Recursive_Thread_Mutex);
        }
        __ext(__this as *const ACE_Recursive_Thread_Mutex)
    }
    pub unsafe fn set_thread_id(__this: *mut Self, mut t: libc::c_ulong) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let _ = (t);
                };
            }
            ()
        }
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Recursive_Thread_Mutex,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN26ACE_Recursive_Thread_MutexaSERKS_"]
            fn __ext(
                __this: *mut ACE_Recursive_Thread_Mutex,
                _anon_0: *const ACE_Recursive_Thread_Mutex,
            );
        }
        __ext(__this as *mut ACE_Recursive_Thread_Mutex, _anon_0)
    }
    pub unsafe fn new_at_ue3cf3ccec10e54b4(
        __this: *mut Self,
        mut __a0: *const ACE_Recursive_Thread_Mutex,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN26ACE_Recursive_Thread_MutexC1ERKS_"]
            fn __ext(
                __this: *mut ACE_Recursive_Thread_Mutex,
                __a0: *const ACE_Recursive_Thread_Mutex,
            );
        }
        __ext(__this as *mut ACE_Recursive_Thread_Mutex, __a0)
    }
    pub unsafe fn new_ue3cf3ccec10e54b4(
        mut __a0: *const ACE_Recursive_Thread_Mutex,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ue3cf3ccec10e54b4(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_RW_Thread_Mutex {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *mut libc::c_void,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_RW_Thread_MutexC1EPKcPv"]
            fn __ext(
                __this: *mut ACE_RW_Thread_Mutex,
                __a0: *const libc::c_char,
                __a1: *mut libc::c_void,
            );
        }
        __ext(__this as *mut ACE_RW_Thread_Mutex, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: *mut libc::c_void,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    #[doc = "* Conditionally upgrade a read lock to a write lock.  This only\n   * works if there are no other readers present, in which case the\n   * method returns 0.  Otherwise, the method returns -1 and sets\n   * @c errno to @c EBUSY.  Note that the caller of this method *must*\n   * already possess this lock as a read lock (but this condition is\n   * not checked by the current implementation)."]
    pub unsafe fn tryacquire_write_upgrade(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::rw_trywrlock_upgrade(
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).__base_0) .cast:: <
                        ACE_RW_Mutex > ().cast_mut()).lock_
                    ) as *mut pthread_rwlock_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK19ACE_RW_Thread_Mutex4dumpEv"]
            fn __ext(__this: *const ACE_RW_Thread_Mutex);
        }
        __ext(__this as *const ACE_RW_Thread_Mutex)
    }
}
impl ACE_Log_Category_TSS {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *mut ACE_Log_Category,
        mut __a1: *mut ACE_Log_Msg,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_Log_Category_TSSC1EP16ACE_Log_CategoryP11ACE_Log_Msg"]
            fn __ext(
                __this: *mut ACE_Log_Category_TSS,
                __a0: *mut ACE_Log_Category,
                __a1: *mut ACE_Log_Msg,
            );
        }
        __ext(__this as *mut ACE_Log_Category_TSS, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *mut ACE_Log_Category,
        mut __a1: *mut ACE_Log_Msg,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    pub unsafe fn name(__this: *const Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*(*__this).category_).name_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn id(__this: *const Self) -> libc::c_uint {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*(*__this).category_).id_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn logger(__this: *mut Self) -> *mut ACE_Log_Msg {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).logger_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the current ACE_Log_Priority mask.
    pub unsafe fn priority_mask(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).priority_mask_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the ACE_Log_Priority mask, returns original mask.
    pub unsafe fn priority_mask_u42f45a1071cf0c71(
        __this: *mut Self,
        mut n_mask: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut o_mask: libc::c_ulong = (*__this).priority_mask_;
                (*__this).priority_mask_ = n_mask;
                return o_mask;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return true if the requested priority is enabled.
    pub unsafe fn log_priority_enabled(
        __this: *mut Self,
        mut log_priority: libc::c_uint,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((((((((((((((*__this).priority_mask_) as libc::c_ulong))
                    | ((<ACE_Log_Category>::priority_mask(
                        ((*__this).category_) as *const ACE_Log_Category,
                    )) as libc::c_ulong))) as libc::c_ulong))
                    & ((((log_priority as libc::c_ulong))) as libc::c_ulong))
                    as libc::c_ulong)) != (((0) as libc::c_ulong))) as libc::c_int))
                    as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Set the line number, file name, operational status, error number,\n   * restart flag, ostream, and the callback object.  This combines\n   * all the other set methods into a single method."]
    pub unsafe fn set(
        __this: *mut Self,
        mut file: *const libc::c_char,
        mut line: libc::c_int,
        mut op_status: libc::c_int,
        mut errnum: libc::c_int,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Log_Msg>::set(
                    ((*__this).logger_) as *mut ACE_Log_Msg,
                    file,
                    line,
                    op_status,
                    errnum,
                    <ACE_Log_Msg>::restart_uc5a430d422835657(
                        ((*__this).logger_) as *const ACE_Log_Msg,
                    ),
                    <ACE_Log_Msg>::msg_ostream_ueaab56de14069bdb(
                        ((*__this).logger_) as *const ACE_Log_Msg,
                    ),
                    <ACE_Log_Msg>::msg_callback_u67a211100f0e3e05(
                        ((*__this).logger_) as *const ACE_Log_Msg,
                    ),
                );
            }
            ()
        }
    }
    /**These values are only actually set if the requested priority is
  /// enabled.*/
    pub unsafe fn conditional_set(
        __this: *mut Self,
        mut file: *const libc::c_char,
        mut line: libc::c_int,
        mut op_status: libc::c_int,
        mut errnum: libc::c_int,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Log_Msg>::conditional_set(
                    ((*__this).logger_) as *mut ACE_Log_Msg,
                    file,
                    line,
                    op_status,
                    errnum,
                );
            }
            ()
        }
    }
    pub unsafe extern "C-unwind" fn log(
        __this: *mut Self,
        mut priority: libc::c_uint,
        mut format_str: *const libc::c_char,
        mut __args: ...
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut argp: ::core::ffi::VaList<'_> = unsafe {
                    ::core::mem::MaybeUninit::<::core::ffi::VaList<'_>>::zeroed()
                        .assume_init()
                };
                argp = __args.clone();
                let mut result: libc::c_long = ((<ACE_Log_Category_TSS>::log_u646c7c6997ae181a(
                    (__this) as *mut ACE_Log_Category_TSS,
                    format_str,
                    priority,
                    argp,
                )) as libc::c_long);
                ();
                return ((result) as libc::c_long);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe extern "C-unwind" fn log_u212294331acb46cd(
        __this: *mut Self,
        mut priority: libc::c_uint,
        mut format_str: *const libc::wchar_t,
        mut __args: ...
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut argp: ::core::ffi::VaList<'_> = unsafe {
                    ::core::mem::MaybeUninit::<::core::ffi::VaList<'_>>::zeroed()
                        .assume_init()
                };
                argp = __args.clone();
                let mut result: libc::c_long = ((<ACE_Log_Category_TSS>::log_u646c7c6997ae181a(
                    (__this) as *mut ACE_Log_Category_TSS,
                    ((<ACE_Wide_To_Ascii>::char_rep(
                        (((&mut ({
                            let mut __recv: ACE_Wide_To_Ascii = (<ACE_Wide_To_Ascii>::new(
                                ((format_str) as *const libc::wchar_t),
                            ));
                            __recv
                        })) as *mut ACE_Wide_To_Ascii)) as *mut ACE_Wide_To_Ascii,
                    )) as *const libc::c_char),
                    priority,
                    argp,
                )) as libc::c_long);
                ();
                return ((result) as libc::c_long);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* An alternative logging mechanism that makes it possible to\n * integrate variable argument lists from other logging mechanisms\n * into the ACE mechanism."]
    pub unsafe fn log_u646c7c6997ae181a(
        __this: *mut Self,
        mut format: *const libc::c_char,
        mut priority: libc::c_uint,
        mut argp: ::core::ffi::VaList<'_>,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((<ACE_Log_Category_TSS>::log_priority_enabled(
                    (__this) as *mut ACE_Log_Category_TSS,
                    priority,
                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return ((0) as libc::c_long);
                }
                return <ACE_Log_Msg>::log_udba2b3215d5c24e0(
                    ((*__this).logger_) as *mut ACE_Log_Msg,
                    format,
                    priority,
                    argp,
                    __this,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn log_u720732fe358f1aed(
        __this: *mut Self,
        mut log_record: *mut ACE_Log_Record,
        mut suppress_stderr: libc::c_int,
    ) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Log_Msg>::log_udbac5b16bf739507(
                    ((*__this).logger_) as *mut ACE_Log_Msg,
                    ::core::ptr::addr_of_mut!((* log_record)),
                    suppress_stderr,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Method to log hex dump.  This is useful for debugging.  Calls\n   * log() to do the actual print, but formats first to make the chars\n   * printable."]
    pub unsafe fn log_hexdump(
        __this: *mut Self,
        mut priority: libc::c_uint,
        mut buffer: *const libc::c_char,
        mut size: libc::c_ulong,
        mut text: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((<ACE_Log_Category_TSS>::log_priority_enabled(
                    (__this) as *mut ACE_Log_Category_TSS,
                    priority,
                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return 0;
                }
                return <ACE_Log_Msg>::log_hexdump(
                    ((*__this).logger_) as *mut ACE_Log_Msg,
                    priority,
                    buffer,
                    size,
                    text,
                    __this,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Log_Msg {
    ///Returns a pointer to the Singleton.
    pub unsafe fn instance() -> *mut ACE_Log_Msg {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg8instanceEv"]
            fn __ext() -> *mut ACE_Log_Msg;
        }
        __ext()
    }
    ///Returns last error.
    pub unsafe fn last_error_adapter() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg18last_error_adapterEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    ///Returns non-null if an ACE_Log_Msg exists for the calling thread.
    pub unsafe fn exists() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg6existsEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    ///Returns the current program name used for logging.
    pub unsafe fn program_name() -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg12program_nameEv"]
            fn __ext() -> *const libc::c_char;
        }
        __ext()
    }
    /**Clears the flag from the default priority mask used to
  /// initialize ACE_Log_Msg instances.*/
    pub unsafe fn disable_debug_messages(mut priority: libc::c_uint) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg22disable_debug_messagesE16ACE_Log_Priority"]
            fn __ext(priority: libc::c_uint);
        }
        __ext(priority)
    }
    /**Sets the flag in the default priority mask used to initialize
  /// ACE_Log_Msg instances.*/
    pub unsafe fn enable_debug_messages(mut priority: libc::c_uint) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg21enable_debug_messagesE16ACE_Log_Priority"]
            fn __ext(priority: libc::c_uint);
        }
        __ext(priority)
    }
    ///Initialize logger.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_MsgC1Ev"]
            fn __ext(__this: *mut ACE_Log_Msg);
        }
        __ext(__this as *mut ACE_Log_Msg)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    #[doc = "Initialize the ACE logging facility.\n  /**\n   * Initialize the ACE logging facility. Supplies the program name\n   * that is available to each logging message call. Default arguments\n   * set up logging to STDERR only.\n   *\n   * @param prog_name      The name of the calling program.\n   * @param options_flags  A bitwise-or of options flags used to set the\n   *                       initial behavior and logging sink(s). (see the\n   *                       enum above for the valid values).\n   * @param logger_key     The name of ACE_FIFO rendezvous point where the\n   *                       local client logger daemon is listening for logging\n   *                       messages if the LOGGER bit is set in the @a flags\n   *                       argument. If the SYSLOG bit is set in @a flags,\n   *                       @a logger_key is the source/program name specified\n   *                       in the syslog facility (UNIX/Linux) or the Windows\n   *                       event log (Windows). In the SYSLOG case, if\n   *                       @a logger_key is 0, @a prog_name is used."]
    pub unsafe fn open(
        __this: *mut Self,
        mut prog_name: *const libc::c_char,
        mut options_flags: libc::c_ulong,
        mut logger_key: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg4openEPKcmS1_"]
            fn __ext(
                __this: *mut ACE_Log_Msg,
                prog_name: *const libc::c_char,
                options_flags: libc::c_ulong,
                logger_key: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Log_Msg, prog_name, options_flags, logger_key)
    }
    ///* Enable the bits in the logger's options flags.
    pub unsafe fn set_flags(__this: *mut Self, mut f: libc::c_ulong) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg9set_flagsEm"]
            fn __ext(__this: *mut ACE_Log_Msg, f: libc::c_ulong);
        }
        __ext(__this as *mut ACE_Log_Msg, f)
    }
    ///* Disable the bits in the logger's options flags.
    pub unsafe fn clr_flags(__this: *mut Self, mut f: libc::c_ulong) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg9clr_flagsEm"]
            fn __ext(__this: *mut ACE_Log_Msg, f: libc::c_ulong);
        }
        __ext(__this as *mut ACE_Log_Msg, f)
    }
    ///* Return the bits in the logger's options flags.
    pub unsafe fn flags(__this: *mut Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg5flagsEv"]
            fn __ext(__this: *mut ACE_Log_Msg) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Log_Msg)
    }
    ///Acquire the internal lock.
    pub unsafe fn acquire(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg7acquireEv"]
            fn __ext(__this: *mut ACE_Log_Msg) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Log_Msg)
    }
    ///Release the internal lock.
    pub unsafe fn release(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg7releaseEv"]
            fn __ext(__this: *mut ACE_Log_Msg) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Log_Msg)
    }
    /**Call after doing a @c fork() to resynchronize the process id and
  /// @c program_name_ variables.*/
    pub unsafe fn sync(__this: *mut Self, mut program_name: *const libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg4syncEPKc"]
            fn __ext(__this: *mut ACE_Log_Msg, program_name: *const libc::c_char);
        }
        __ext(__this as *mut ACE_Log_Msg, program_name)
    }
    /**Set the result of the operation status (by convention, -1 means
  /// error).*/
    pub unsafe fn op_status(__this: *mut Self, mut status: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).status_ = status;
            }
            ()
        }
    }
    /**Get the result of the operation status (by convention, -1 means
  /// error).*/
    pub unsafe fn op_status_ufed6916af7f9abaa(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).status_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Set the value of the errnum (by convention this corresponds to
  /// errno).*/
    pub unsafe fn errnum(__this: *mut Self, mut e: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).errnum_ = e;
            }
            ()
        }
    }
    /**Get the value of the errnum (by convention this corresponds to
  /// errno).*/
    pub unsafe fn errnum_ube5ddfb14fa3543d(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).errnum_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the line number where an error occurred.
    pub unsafe fn linenum(__this: *mut Self, mut l: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).linenum_ = l;
            }
            ()
        }
    }
    ///Get the line number where an error occurred.
    pub unsafe fn linenum_u40d7065b119b310a(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).linenum_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the file name where an error occurred.
    pub unsafe fn file(__this: *mut Self, mut s: *const libc::c_char) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                ACE_OS::strsncpy(
                    (((*__this).file_).as_mut_ptr() as *mut libc::c_char),
                    s,
                    ((::core::mem::size_of::<[libc::c_char; 4097usize]>()
                        as libc::c_ulong) as libc::c_ulong),
                );
            }
            ()
        }
    }
    ///Get the file name where an error occurred.
    pub unsafe fn file_ud74acec8863ebe7b(__this: *mut Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).file_).as_ptr() as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the message that describes what type of error occurred.
    pub unsafe fn msg(__this: *mut Self, mut m: *const libc::c_char) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                ACE_OS::strsncpy(
                    (((*__this).msg_) as *mut libc::c_char),
                    ((m) as *const libc::c_char),
                    (((((((((((((4) as libc::c_int)).wrapping_mul((1024) as libc::c_int))
                        as libc::c_int))
                        .wrapping_add((1) as libc::c_int))) as libc::c_ulong))
                        / ((1) as libc::c_ulong))) as libc::c_ulong),
                );
            }
            ()
        }
    }
    ///Get the message that describes what type of error occurred.
    pub unsafe fn msg_u3684c07b6c08da72(__this: *mut Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((((*__this).msg_)
                    .wrapping_offset((ACE_Log_Msg_msg_off_) as isize))
                    as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Set the field that indicates whether interrupted calls should be
  /// restarted.*/
    pub unsafe fn restart(__this: *mut Self, mut r: bool) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).restart_ = r;
            }
            ()
        }
    }
    /**Get the field that indicates whether interrupted calls should be
  /// restarted.*/
    pub unsafe fn restart_uc5a430d422835657(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    (((*__this).restart_) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Update the ostream without overwriting the delete_ostream_ flag.
    pub unsafe fn msg_ostream(__this: *mut Self, mut m: *mut crate::__cxx_std::Ostream) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).ostream_ = m;
            }
            ()
        }
    }
    #[doc = "* delete_stream == true, forces Log_Msg.h to delete the stream in\n   * its own ~dtor (assumes control of the stream)\n   * use only with proper ostream (eg: fstream), not (cout, cerr)"]
    pub unsafe fn msg_ostream_ub3f0a7fa05058697(
        __this: *mut Self,
        mut _anon_0: *mut crate::__cxx_std::Ostream,
        mut delete_ostream: bool,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg11msg_ostreamEPSob"]
            fn __ext(
                __this: *mut ACE_Log_Msg,
                _anon_0: *mut crate::__cxx_std::Ostream,
                delete_ostream: bool,
            );
        }
        __ext(__this as *mut ACE_Log_Msg, _anon_0, delete_ostream)
    }
    ///Get the ostream that is used to print error messages.
    pub unsafe fn msg_ostream_ueaab56de14069bdb(
        __this: *const Self,
    ) -> *mut crate::__cxx_std::Ostream {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).ostream_) as *mut crate::__cxx_std::Ostream);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Set a new callback object and return the existing callback to\n   * allow \"chaining\".  Note that ACE_Log_Msg_Callback objects are not\n   * inherited when spawning a new thread, so you'll need to reset\n   * them in each thread."]
    pub unsafe fn msg_callback(
        __this: *mut Self,
        mut c: *mut ACE_Log_Msg_Callback,
    ) -> *mut ACE_Log_Msg_Callback {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut old: *mut ACE_Log_Msg_Callback = (*__this).msg_callback_;
                (*__this).msg_callback_ = c;
                return old;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn msg_callback_u67a211100f0e3e05(
        __this: *const Self,
    ) -> *mut ACE_Log_Msg_Callback {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).msg_callback_) as *mut ACE_Log_Msg_Callback);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Set a new backend object and return the existing backend to\n   * allow \"chaining\". Note that as opposed to ACE_Log_Msg_Callback,\n   * ACE_Log_Msg_Backend is a per-process entity.\n   *\n   * @note Be aware that because of the current architecture there is\n   * no guarantee that open (), reset () and close () will be called\n   * on a backend object."]
    pub unsafe fn msg_backend(
        mut b: *mut ACE_Log_Msg_Backend,
    ) -> *mut ACE_Log_Msg_Backend {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg11msg_backendEP19ACE_Log_Msg_Backend"]
            fn __ext(b: *mut ACE_Log_Msg_Backend) -> *mut ACE_Log_Msg_Backend;
        }
        __ext(b)
    }
    pub unsafe fn msg_backend_u6cae5ebc38e54170() -> *mut ACE_Log_Msg_Backend {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg11msg_backendEv"]
            fn __ext() -> *mut ACE_Log_Msg_Backend;
        }
        __ext()
    }
    ///Nesting depth increment.
    pub unsafe fn inc(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __lv = &mut ((*__this).trace_depth_);
                    let __r = *__lv;
                    *__lv = (*__lv).wrapping_add(1);
                    __r
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Nesting depth decrement.
    pub unsafe fn dec(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return if ((((((*__this).trace_depth_ as libc::c_int))
                    == (((0) as libc::c_int))) as libc::c_int as libc::c_int) != 0)
                {
                    0
                } else {
                    {
                        let __lv = &mut ((*__this).trace_depth_);
                        *__lv = (*__lv).wrapping_sub(1);
                        *__lv
                    }
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get trace depth.
    pub unsafe fn trace_depth(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).trace_depth_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set trace depth.
    pub unsafe fn trace_depth_u181f6714d28b47bc(
        __this: *mut Self,
        mut depth: libc::c_int,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).trace_depth_ = depth;
            }
            ()
        }
    }
    ///Get trace active status.
    pub unsafe fn trace_active(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    (((*__this).trace_active_) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set  trace active status.
    pub unsafe fn trace_active_u3597e8daa2350886(__this: *mut Self, mut value: bool) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).trace_active_ = value;
            }
            ()
        }
    }
    ///Get the TSS thread descriptor.
    pub unsafe fn thr_desc(__this: *const Self) -> *mut ACE_Thread_Descriptor {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).thr_desc_) as *mut ACE_Thread_Descriptor);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Set the TSS thread descriptor.  This method will call\n   * td->acquire_release to block execution until this call\n   * return."]
    pub unsafe fn thr_desc_u595350604b3da0c1(
        __this: *mut Self,
        mut td: *mut ACE_Thread_Descriptor,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg8thr_descEP21ACE_Thread_Descriptor"]
            fn __ext(__this: *mut ACE_Log_Msg, td: *mut ACE_Thread_Descriptor);
        }
        __ext(__this as *mut ACE_Log_Msg, td)
    }
    ///Stop tracing status on a per-thread basis...
    pub unsafe fn stop_tracing(__this: *mut Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).tracing_enabled_ = false;
            }
            ()
        }
    }
    ///Start tracing status on a per-thread basis...
    pub unsafe fn start_tracing(__this: *mut Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).tracing_enabled_ = true;
            }
            ()
        }
    }
    ///Query tracing status on a per-thread basis...
    pub unsafe fn tracing_enabled(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    (((*__this).tracing_enabled_) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the current ACE_Log_Priority mask.
    pub unsafe fn priority_mask(
        __this: *mut Self,
        mut mask_type: libc::c_uint,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return if (((((mask_type as libc::c_uint))
                    == (((THREAD) as libc::c_uint))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).priority_mask_
                } else {
                    ACE_Log_Msg_process_priority_mask_
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the ACE_Log_Priority mask, returns original mask.
    pub unsafe fn priority_mask_u7614f3efc4edccc4(
        __this: *mut Self,
        mut _anon_0: libc::c_ulong,
        mut _anon_1: libc::c_uint,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg13priority_maskEmNS_9MASK_TYPEE"]
            fn __ext(
                __this: *mut ACE_Log_Msg,
                _anon_0: libc::c_ulong,
                _anon_1: libc::c_uint,
            ) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Log_Msg, _anon_0, _anon_1)
    }
    ///Return true if the requested priority is enabled.
    pub unsafe fn log_priority_enabled(
        __this: *mut Self,
        mut log_priority: libc::c_uint,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((((((((((((((*__this).priority_mask_) as libc::c_ulong))
                    | ((ACE_Log_Msg_process_priority_mask_) as libc::c_ulong)))
                    as libc::c_ulong))
                    & ((((log_priority as libc::c_ulong))) as libc::c_ulong))
                    as libc::c_ulong)) != (((0) as libc::c_ulong))) as libc::c_int))
                    as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Optimize reading of the pid (avoids a system call if the value is
  /// cached...).*/
    pub unsafe fn getpid(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::getpid_ucc9c2c9e176d9ca8();
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the name of the local host.
    pub unsafe fn local_host(__this: *const Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_Log_Msg_local_host_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the name of the local host.
    pub unsafe fn local_host_u1f4fbd1a9b85b9d4(
        __this: *mut Self,
        mut _anon_0: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg10local_hostEPKc"]
            fn __ext(__this: *mut ACE_Log_Msg, _anon_0: *const libc::c_char);
        }
        __ext(__this as *mut ACE_Log_Msg, _anon_0)
    }
    #[doc = "* Set the line number, file name, operational status, error number,\n   * restart flag, ostream, and the callback object.  This combines\n   * all the other set methods into a single method."]
    pub unsafe fn set(
        __this: *mut Self,
        mut file: *const libc::c_char,
        mut line: libc::c_int,
        mut op_status: libc::c_int,
        mut errnum: libc::c_int,
        mut restart: bool,
        mut os: *mut crate::__cxx_std::Ostream,
        mut c: *mut ACE_Log_Msg_Callback,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg3setEPKciiibPSoP20ACE_Log_Msg_Callback"]
            fn __ext(
                __this: *mut ACE_Log_Msg,
                file: *const libc::c_char,
                line: libc::c_int,
                op_status: libc::c_int,
                errnum: libc::c_int,
                restart: bool,
                os: *mut crate::__cxx_std::Ostream,
                c: *mut ACE_Log_Msg_Callback,
            );
        }
        __ext(__this as *mut ACE_Log_Msg, file, line, op_status, errnum, restart, os, c)
    }
    /**These values are only actually set if the requested priority is
  /// enabled.*/
    pub unsafe fn conditional_set(
        __this: *mut Self,
        mut file: *const libc::c_char,
        mut line: libc::c_int,
        mut op_status: libc::c_int,
        mut errnum: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg15conditional_setEPKciii"]
            fn __ext(
                __this: *mut ACE_Log_Msg,
                file: *const libc::c_char,
                line: libc::c_int,
                op_status: libc::c_int,
                errnum: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Log_Msg, file, line, op_status, errnum)
    }
    #[doc = "* An alternative logging mechanism that makes it possible to\n   * integrate variable argument lists from other logging mechanisms\n   * into the ACE mechanism."]
    pub unsafe fn log_udba2b3215d5c24e0(
        __this: *mut Self,
        mut format: *const libc::c_char,
        mut priority: libc::c_uint,
        mut argp: ::core::ffi::VaList<'_>,
        mut category: *mut ACE_Log_Category_TSS,
    ) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg3logEPKc16ACE_Log_PriorityP13__va_list_tagP20ACE_Log_Category_TSS"]
            fn __ext(
                __this: *mut ACE_Log_Msg,
                format: *const libc::c_char,
                priority: libc::c_uint,
                argp: ::core::ffi::VaList<'_>,
                category: *mut ACE_Log_Category_TSS,
            ) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Log_Msg, format, priority, argp, category)
    }
    /**Log a custom built log record to the currently enabled logging
  /// sinks.*/
    pub unsafe fn log_udbac5b16bf739507(
        __this: *mut Self,
        mut log_record: *mut ACE_Log_Record,
        mut suppress_stderr: libc::c_int,
    ) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg3logER14ACE_Log_Recordi"]
            fn __ext(
                __this: *mut ACE_Log_Msg,
                log_record: *mut ACE_Log_Record,
                suppress_stderr: libc::c_int,
            ) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Log_Msg, log_record, suppress_stderr)
    }
    #[doc = "* Method to log hex dump.  This is useful for debugging.  Calls\n   * log() to do the actual print, but formats first to make the chars\n   * printable."]
    pub unsafe fn log_hexdump(
        __this: *mut Self,
        mut log_priority: libc::c_uint,
        mut buffer: *const libc::c_char,
        mut size: libc::c_ulong,
        mut text: *const libc::c_char,
        mut category: *mut ACE_Log_Category_TSS,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg11log_hexdumpE16ACE_Log_PriorityPKcmS2_P20ACE_Log_Category_TSS"]
            fn __ext(
                __this: *mut ACE_Log_Msg,
                log_priority: libc::c_uint,
                buffer: *const libc::c_char,
                size: libc::c_ulong,
                text: *const libc::c_char,
                category: *mut ACE_Log_Category_TSS,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Log_Msg, log_priority, buffer, size, text, category)
    }
    #[doc = "* Init hook, create a Log_Msg_Attribute object, initialize its\n   * attributes from the TSS Log_Msg and save the object in the\n   * @a attributes argument"]
    pub unsafe fn init_hook(mut attributes: *mut ACE_OS_Log_Msg_Attributes) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg9init_hookER25ACE_OS_Log_Msg_Attributes"]
            fn __ext(attributes: *mut ACE_OS_Log_Msg_Attributes);
        }
        __ext(attributes)
    }
    #[doc = "* Inherit hook, the @a attributes field is a ACE_OS_Log_Msg_Attributes\n   * object, invoke the inherit_log_msg() method on it, then destroy\n   * it and set the @a attribute argument to 0."]
    pub unsafe fn inherit_hook(
        mut thr_desc: *mut ACE_OS_Thread_Descriptor,
        mut attributes: *mut ACE_OS_Log_Msg_Attributes,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg12inherit_hookEP24ACE_OS_Thread_DescriptorR25ACE_OS_Log_Msg_Attributes"]
            fn __ext(
                thr_desc: *mut ACE_OS_Thread_Descriptor,
                attributes: *mut ACE_OS_Log_Msg_Attributes,
            );
        }
        __ext(thr_desc, attributes)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK11ACE_Log_Msg4dumpEv"]
            fn __ext(__this: *const ACE_Log_Msg);
        }
        __ext(__this as *const ACE_Log_Msg)
    }
    pub unsafe fn cleanup_ostream(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg15cleanup_ostreamEv"]
            fn __ext(__this: *mut ACE_Log_Msg);
        }
        __ext(__this as *mut ACE_Log_Msg)
    }
    ///For cleanup, at program termination.
    pub unsafe fn close() {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg5closeEv"]
            fn __ext();
        }
        __ext()
    }
    ///Decouple the OS layer from the ACE_Log_Msg layer.
    pub unsafe fn sync_hook(mut prg_name: *const libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg9sync_hookEPKc"]
            fn __ext(prg_name: *const libc::c_char);
        }
        __ext(prg_name)
    }
    ///Return the TSS singleton thread descriptor
    pub unsafe fn thr_desc_hook() -> *mut ACE_OS_Thread_Descriptor {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_Msg13thr_desc_hookEv"]
            fn __ext() -> *mut ACE_OS_Thread_Descriptor;
        }
        __ext()
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Log_Msg,
    ) -> *mut ACE_Log_Msg {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_MsgaSERKS_"]
            fn __ext(
                __this: *mut ACE_Log_Msg,
                _anon_0: *const ACE_Log_Msg,
            ) -> *mut ACE_Log_Msg;
        }
        __ext(__this as *mut ACE_Log_Msg, _anon_0)
    }
    pub unsafe fn new_at_u206f479abc154a2e(
        __this: *mut Self,
        mut __a0: *const ACE_Log_Msg,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Log_MsgC1ERKS_"]
            fn __ext(__this: *mut ACE_Log_Msg, __a0: *const ACE_Log_Msg);
        }
        __ext(__this as *mut ACE_Log_Msg, __a0)
    }
    pub unsafe fn new_u206f479abc154a2e(mut __a0: *const ACE_Log_Msg) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u206f479abc154a2e(::core::ptr::addr_of_mut!(__obj), __a0);
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
impl ACE_Object_Manager {
    #[doc = "* Explicitly initialize (construct the singleton instance of) the\n   * ACE_Object_Manager.  Returns 0 on success, -1 on failure, and 1\n   * if it had already been called."]
    pub unsafe fn init(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager4initEv"]
            fn __ext(__this: *mut ACE_Object_Manager) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Object_Manager)
    }
    #[doc = "* Explicitly destroy the singleton instance of the\n   * ACE_Object_Manager.  Returns 0 on success, -1 on failure, and 1\n   * if it had already been called."]
    pub unsafe fn fini(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager4finiEv"]
            fn __ext(__this: *mut ACE_Object_Manager) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Object_Manager)
    }
    #[doc = "* Returns 1 before the ACE_Object_Manager has been constructed.\n   * This flag can be used to determine if the program is constructing\n   * static objects.  If no static object spawns any threads, the\n   * program will be single-threaded when this flag returns 1.  (Note\n   * that the program still might construct some static objects when\n   * this flag returns 0, if ACE_HAS_NONSTATIC_OBJECT_MANAGER is not\n   * defined.)"]
    pub unsafe fn starting_up() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager11starting_upEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    #[doc = "* Returns 1 after the ACE_Object_Manager has been destroyed.  This\n   * flag can be used to determine if the program is in the midst of\n   * destroying static objects.  (Note that the program might destroy\n   * some static objects before this flag can return 1, if\n   * ACE_HAS_NONSTATIC_OBJECT_MANAGER is not defined.)"]
    pub unsafe fn shutting_down() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager13shutting_downEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    #[doc = "* Register an ACE_Cleanup object for cleanup at process\n   * termination.  The object is deleted via the\n   * <ace_cleanup_destroyer>.  If you need more flexibility, see the\n   * @c other at_exit method below.  For OS's that do not have\n   * processes, cleanup takes place at the end of <main>.  Returns 0\n   * on success.  On failure, returns -1 and sets errno to: EAGAIN if\n   * shutting down, ENOMEM if insufficient virtual memory, or EEXIST\n   * if the object (or array) had already been registered."]
    pub unsafe fn at_exit(
        mut object: *mut ACE_Cleanup,
        mut param: *mut libc::c_void,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            {
                return <ACE_Object_Manager>::at_exit_i(
                    (<ACE_Object_Manager>::instance()) as *mut ACE_Object_Manager,
                    ((object) as *mut libc::c_void),
                    unsafe {
                        ::core::mem::transmute::<
                            unsafe extern "C-unwind" fn(
                                *mut ACE_Cleanup,
                                *mut libc::c_void,
                            ),
                            Option<
                                unsafe extern "C-unwind" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ),
                            >,
                        >(
                            (ace_cleanup_destroyer)
                                as unsafe extern "C-unwind" fn(
                                    *mut ACE_Cleanup,
                                    *mut libc::c_void,
                                ),
                        )
                    },
                    param,
                    name,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Register an object (or array) for cleanup at process termination.\n   * \"cleanup_hook\" points to a (global, or static member) function\n   * that is called for the object or array when it to be destroyed.\n   * It may perform any necessary cleanup specific for that object or\n   * its class.  \"param\" is passed as the second parameter to the\n   * @a cleanup_hook function; the first parameter is the object (or\n   * array) to be destroyed.  @a cleanup_hook, for example, may delete\n   * the object (or array).  For OS's that do not have processes, this\n   * function is the same as <at_thread_exit>.  Returns 0 on success.\n   * On failure, returns -1 and sets errno to: EAGAIN if shutting\n   * down, ENOMEM if insufficient virtual memory, or EEXIST if the\n   * object (or array) had already been registered."]
    pub unsafe fn at_exit_ud30f528992f3d733(
        mut object: *mut libc::c_void,
        mut cleanup_hook: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void, *mut libc::c_void),
        >,
        mut param: *mut libc::c_void,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            {
                return <ACE_Object_Manager>::at_exit_i(
                    (<ACE_Object_Manager>::instance()) as *mut ACE_Object_Manager,
                    object,
                    cleanup_hook,
                    param,
                    name,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn remove_at_exit(mut object: *mut libc::c_void) -> libc::c_int {
        unsafe {
            {
                return <ACE_Object_Manager>::remove_at_exit_i(
                    (<ACE_Object_Manager>::instance()) as *mut ACE_Object_Manager,
                    object,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* @deprecated\n   * Accesses a default signal set used, for example,\n   * in ACE_Sig_Guard methods.\n   * Deprecated: use ACE_Object_Manager::default_mask () instead."]
    pub unsafe fn default_mask() -> *mut ACE_Sig_Set {
        unsafe {
            {
                return (<ACE_OS_Object_Manager>::default_mask() as *mut ACE_Sig_Set);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Register an object or array for deletion at program termination.
  /// See description of static version above for return values.*/
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
            #[link_name = "_ZN18ACE_Object_Manager9at_exit_iEPvPFvS0_S0_ES0_PKc"]
            fn __ext(
                __this: *mut ACE_Object_Manager,
                object: *mut libc::c_void,
                cleanup_hook: Option<
                    unsafe extern "C-unwind" fn(*mut libc::c_void, *mut libc::c_void),
                >,
                param: *mut libc::c_void,
                name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Object_Manager, object, cleanup_hook, param, name)
    }
    /**Remove an object for deletion at program termination.
  /// See description of static version above for return values.*/
    pub unsafe fn remove_at_exit_i(
        __this: *mut Self,
        mut object: *mut libc::c_void,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager16remove_at_exit_iEPv"]
            fn __ext(
                __this: *mut ACE_Object_Manager,
                object: *mut libc::c_void,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Object_Manager, object)
    }
    #[doc = "* Accesses an ACE_Null_Mutex to be used for construction of\n   * ACE_Singletons.  Returns 0, and the lock in the argument, on\n   * success; returns -1 on failure."]
    pub unsafe fn get_singleton_lock(
        mut _anon_0: *mut *mut ACE_Null_Mutex,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager18get_singleton_lockERP14ACE_Null_Mutex"]
            fn __ext(_anon_0: *mut *mut ACE_Null_Mutex) -> libc::c_int;
        }
        __ext(_anon_0)
    }
    #[doc = "* Accesses a non-recursive ACE_Thread_Mutex to be used for\n   * construction of ACE_Singletons.  Returns 0, and the lock in the\n   * argument, on success; returns -1 on failure."]
    pub unsafe fn get_singleton_lock_ucad184b1c11997ce(
        mut _anon_0: *mut *mut ACE_Thread_Mutex,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager18get_singleton_lockERP16ACE_Thread_Mutex"]
            fn __ext(_anon_0: *mut *mut ACE_Thread_Mutex) -> libc::c_int;
        }
        __ext(_anon_0)
    }
    #[doc = "* Accesses a non-recursive ACE_Mutex to be used for construction\n   * of ACE_Singletons.  Returns 0, and the lock in the argument, on\n   * success; returns -1 on failure."]
    pub unsafe fn get_singleton_lock_u5d000c32322ab315(
        mut _anon_0: *mut *mut ACE_Mutex,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager18get_singleton_lockERP9ACE_Mutex"]
            fn __ext(_anon_0: *mut *mut ACE_Mutex) -> libc::c_int;
        }
        __ext(_anon_0)
    }
    #[doc = "* Accesses a recursive ACE_Recursive_Thread_Mutex to be used for\n   * construction of ACE_Singletons.  Returns 0, and the lock in the\n   * argument, on success; returns -1 on failure."]
    pub unsafe fn get_singleton_lock_u4275b6060d6b60bf(
        mut _anon_0: *mut *mut ACE_Recursive_Thread_Mutex,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager18get_singleton_lockERP26ACE_Recursive_Thread_Mutex"]
            fn __ext(_anon_0: *mut *mut ACE_Recursive_Thread_Mutex) -> libc::c_int;
        }
        __ext(_anon_0)
    }
    #[doc = "* Accesses a readers/writer ACE_RW_Thread_Mutex to be used for\n   * construction of ACE_Singletons.  Returns 0, and the lock in the\n   * argument, on success; returns -1 on failure."]
    pub unsafe fn get_singleton_lock_uce60b00472ad1b0a(
        mut _anon_0: *mut *mut ACE_RW_Thread_Mutex,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager18get_singleton_lockERP19ACE_RW_Thread_Mutex"]
            fn __ext(_anon_0: *mut *mut ACE_RW_Thread_Mutex) -> libc::c_int;
        }
        __ext(_anon_0)
    }
    #[doc = "* Accessor to singleton instance.  Because static member functions\n   * are provided in the interface, this should not be public.  However,\n   * it is public so that ACE_Managed_Object<TYPE> can access it."]
    pub unsafe fn instance() -> *mut ACE_Object_Manager {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_Manager8instanceEv"]
            fn __ext() -> *mut ACE_Object_Manager;
        }
        __ext()
    }
    /**Application code should not use these explicitly, so they're
  /// hidden here.  They're public so that the ACE_Object_Manager can
  /// be constructed/destructed in <main> with
  /// ACE_HAS_NONSTATIC_OBJECT_MANAGER.*/
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_ManagerC1Ev"]
            fn __ext(__this: *mut ACE_Object_Manager);
        }
        __ext(__this as *mut ACE_Object_Manager)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Disallow copying by not implementing the following . . .
    pub unsafe fn new_at_u951884c3ee739b5c(
        __this: *mut Self,
        mut __a0: *const ACE_Object_Manager,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_ManagerC1ERKS_"]
            fn __ext(__this: *mut ACE_Object_Manager, __a0: *const ACE_Object_Manager);
        }
        __ext(__this as *mut ACE_Object_Manager, __a0)
    }
    pub unsafe fn new_u951884c3ee739b5c(mut __a0: *const ACE_Object_Manager) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u951884c3ee739b5c(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Object_Manager,
    ) -> *mut ACE_Object_Manager {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Object_ManageraSERKS_"]
            fn __ext(
                __this: *mut ACE_Object_Manager,
                _anon_0: *const ACE_Object_Manager,
            ) -> *mut ACE_Object_Manager;
        }
        __ext(__this as *mut ACE_Object_Manager, _anon_0)
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
impl ACE_Log_Category {
    #[doc = "* Initialize the logger with a name.\n   *\n   * Notice that ACE_Log_Category does not\n   * deep copy the passed \\a name; therefore,\n   * you must keep the lifetime of \\a name\n   * longer than the newly create ACE_Log_Category\n   * object. The rational for the design is to avoid\n   * static initialization problem when the ACE_Log_Category\n   * is created in static storage."]
    pub unsafe fn new_at(__this: *mut Self, mut __a0: *const libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Log_CategoryC1EPKc"]
            fn __ext(__this: *mut ACE_Log_Category, __a0: *const libc::c_char);
        }
        __ext(__this as *mut ACE_Log_Category, __a0)
    }
    pub unsafe fn new(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn id(__this: *const Self) -> libc::c_uint {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).id_) as libc::c_uint);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn name(__this: *const Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).name_) as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn per_thr_obj(__this: *mut Self) -> *mut ACE_Log_Category_TSS {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Log_Category11per_thr_objEv"]
            fn __ext(__this: *mut ACE_Log_Category) -> *mut ACE_Log_Category_TSS;
        }
        __ext(__this as *mut ACE_Log_Category)
    }
    ///Get the process  ACE_Log_Priority mask.
    pub unsafe fn priority_mask(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).priority_mask_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the process ACE_Log_Priority mask, returns original mask.
    pub unsafe fn priority_mask_u44879e5ec1b59335(
        __this: *mut Self,
        mut n_mask: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut o_mask: libc::c_ulong = (*__this).priority_mask_;
                (*__this).priority_mask_ = n_mask;
                return o_mask;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn ace_lib() -> *mut ACE_Log_Category {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Log_Category7ace_libEv"]
            fn __ext() -> *mut ACE_Log_Category;
        }
        __ext()
    }
    pub unsafe fn new_at_ucd64e11dfb568270(
        __this: *mut Self,
        mut __a0: *const ACE_Log_Category,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Log_CategoryC1ERKS_"]
            fn __ext(__this: *mut ACE_Log_Category, __a0: *const ACE_Log_Category);
        }
        __ext(__this as *mut ACE_Log_Category, __a0)
    }
    pub unsafe fn new_ucd64e11dfb568270(mut __a0: *const ACE_Log_Category) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ucd64e11dfb568270(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Log_Category,
    ) -> *mut ACE_Log_Category {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Log_CategoryaSERKS_"]
            fn __ext(
                __this: *mut ACE_Log_Category,
                _anon_0: *const ACE_Log_Category,
            ) -> *mut ACE_Log_Category;
        }
        __ext(__this as *mut ACE_Log_Category, _anon_0)
    }
}
impl ACE_Process_Options {
    #[doc = "* If @a inherit_environment == true, the new process will inherit the\n   * environment of the current process.  @a command_line_buf_len is the\n   * max strlen for command-line arguments."]
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: bool,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_ulong,
        mut __a3: libc::c_ulong,
        mut __a4: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_OptionsC1Ebmmmm"]
            fn __ext(
                __this: *mut ACE_Process_Options,
                __a0: bool,
                __a1: libc::c_ulong,
                __a2: libc::c_ulong,
                __a3: libc::c_ulong,
                __a4: libc::c_ulong,
            );
        }
        __ext(__this as *mut ACE_Process_Options, __a0, __a1, __a2, __a3, __a4)
    }
    pub unsafe fn new(
        mut __a0: bool,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_ulong,
        mut __a3: libc::c_ulong,
        mut __a4: libc::c_ulong,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1, __a2, __a3, __a4);
        __obj
    }
    #[doc = "* Set the standard handles of the new process to the respective\n   * handles.  If you want to affect a subset of the handles, make\n   * sure to set the others to ACE_INVALID_HANDLE.\n   *\n   * @note Any handle passed as ACE_INVALID_HANDLE will be changed to\n   * a duplicate of the current associated handle. For example, passing\n   * ACE_INVALID_HANDLE for @a std_in will cause ACE_STDIN to be\n   * duplicated and set in this object.\n   *\n   * @note Windows: The implementation of set_handles() uses DuplicateHandle\n   *       on Windows. DuplicateHandle cannot be used to pass a socket handle\n   *       on Windows. Socket handles require an alternate mechanism to pass;\n   *       see http://msdn.microsoft.com/en-us/library/ms741565(v=VS.85).aspx\n   *\n   * @return 0 on success, -1 on failure."]
    pub unsafe fn set_handles(
        __this: *mut Self,
        mut std_in: libc::c_int,
        mut std_out: libc::c_int,
        mut std_err: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_Options11set_handlesEiii"]
            fn __ext(
                __this: *mut ACE_Process_Options,
                std_in: libc::c_int,
                std_out: libc::c_int,
                std_err: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Process_Options, std_in, std_out, std_err)
    }
    ///Release the standard handles previously set with set_handles;
    pub unsafe fn release_handles(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_Options15release_handlesEv"]
            fn __ext(__this: *mut ACE_Process_Options);
        }
        __ext(__this as *mut ACE_Process_Options)
    }
    ///Same as above with argv format.  @a envp must be null terminated.
    pub unsafe fn setenv_u272ea02094c11226(
        __this: *mut Self,
        mut envp: *mut *mut libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_Options6setenvEPPc"]
            fn __ext(
                __this: *mut ACE_Process_Options,
                envp: *mut *mut libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Process_Options, envp)
    }
    /**Set the working directory for the process.  strlen of @a wd must
  /// be <= MAXPATHLEN.*/
    pub unsafe fn working_directory(__this: *mut Self, mut wd: *const libc::c_char) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                ACE_OS::strcpy_u08e8184bcebbac89(
                    (((*__this).working_directory_).as_mut_ptr() as *mut libc::c_char),
                    wd,
                );
            }
            ()
        }
    }
    ///wchar_t version of working_directory
    pub unsafe fn working_directory_u2aecbdc9c31dd18b(
        __this: *mut Self,
        mut wd: *const libc::wchar_t,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                ACE_OS::strcpy_u08e8184bcebbac89(
                    (((*__this).working_directory_).as_mut_ptr() as *mut libc::c_char),
                    ((<ACE_Wide_To_Ascii>::char_rep(
                        (((&mut ({
                            let mut __recv: ACE_Wide_To_Ascii = (<ACE_Wide_To_Ascii>::new(
                                wd,
                            ));
                            __recv
                        })) as *mut ACE_Wide_To_Ascii)) as *mut ACE_Wide_To_Ascii,
                    )) as *const libc::c_char),
                );
            }
            ()
        }
    }
    ///Same as above in argv format.  @a argv must be null terminated.
    pub unsafe fn command_line_u0cd738baa4082b1b(
        __this: *mut Self,
        mut argv: *const *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_Options12command_lineEPKPKc"]
            fn __ext(
                __this: *mut ACE_Process_Options,
                argv: *const *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Process_Options, argv)
    }
    #[doc = "* Specify the full path or relative path, or just the executable\n   * name for the process. If this is set, then @a name will be used to\n   * create the process instead of argv[0] set in the command\n   * line. This is here so that you can supply something other than\n   * executable name as argv[0]."]
    pub unsafe fn process_name(__this: *mut Self, mut p: *const libc::c_char) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                ACE_OS::strcpy_u08e8184bcebbac89(
                    (((*__this).process_name_).as_mut_ptr() as *mut libc::c_char),
                    ((p) as *const libc::c_char),
                );
            }
            ()
        }
    }
    /**Return the process_name.  If the <process_name(name)> set
  /// method is not called, this method will return argv[0].*/
    pub unsafe fn process_name_u4c7c375f150565ca(
        __this: *mut Self,
    ) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*((((*__this).process_name_).as_ptr() as *mut libc::c_char)
                    .wrapping_offset((0) as isize))) as libc::c_int as libc::c_char))
                    == (((0 as libc::c_char) as libc::c_int as libc::c_char)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    <ACE_Process_Options>::process_name(
                        (__this) as *mut ACE_Process_Options,
                        (((*(<ACE_Process_Options>::command_line_argv(
                            (__this) as *mut ACE_Process_Options,
                        ))
                            .wrapping_offset((0) as isize))) as *const libc::c_char),
                    );
                }
                return (((*__this).process_name_).as_ptr() as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the creation flags.
    pub unsafe fn creation_flags(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).creation_flags_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Set the creation flags to affect how a new process is spawned.\n   * The only ACE-defined flag is @c NO_EXEC which prevents the new process\n   * from executing a new program image; this is a simple POSIX fork().\n   * The @c NO_EXEC option has no affect on Windows; on other platforms where\n   * a POSIX fork is not possible, specifying @c NO_EXEC will cause\n   * ACE_Process::spawn() to fail.\n   *\n   * On Windows, the value of creation_flags is passed to the @c CreateProcess\n   * system call as the value of the @c dwCreationFlags parameter."]
    pub unsafe fn creation_flags_uba6d83645c63110b(
        __this: *mut Self,
        mut cf: libc::c_ulong,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).creation_flags_ = cf;
            }
            ()
        }
    }
    ///Current working directory.  Returns "" if nothing has been set.
    pub unsafe fn working_directory_uf98831c8bacc074c(
        __this: *mut Self,
    ) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*((((*__this).working_directory_).as_ptr() as *mut libc::c_char)
                    .wrapping_offset((0) as isize))) as libc::c_int as libc::c_char))
                    == (((0 as libc::c_char) as libc::c_int as libc::c_char)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return ((0) as *mut libc::c_char);
                } else {
                    return (((*__this).working_directory_).as_mut_ptr()
                        as *mut libc::c_char);
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Buffer of command-line options.  Returns a pointer to a buffer that
  /// contains the list of command line options.  Prior to a call to
  /// command_line_argv(), this is a single string of space separated
  /// arguments independent of which form of command_line() was used to
  /// create it.  After a call to command_line_argv(), this is a list of
  /// strings each terminated by '\0'.  [Note: spawn() will call
  /// command_line_argv().]  The total length of all these strings is the
  /// same as the single string in the prior case and can be obtained by
  /// providing max_len. @arg max_len, if non-zero, provides a location
  /// into which the total length of the command line buffer is returned.*/
    pub unsafe fn command_line_buf(
        __this: *mut Self,
        mut max_lenp: *mut libc::c_ulong,
    ) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((!(max_lenp).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    (*(max_lenp)) = (*__this).command_line_buf_len_;
                }
                return (*__this).command_line_buf_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* argv-style command-line options.  Parses and modifies the string\n   * created from <command_line_>.  All spaces not in quotes (\"\" or\n   * '') are replaced with null (\\0) bytes.  An argv array is built\n   * and returned with each entry pointing to the start of\n   * null-terminated string.  Returns { 0 } if nothing has been set."]
    pub unsafe fn command_line_argv(__this: *mut Self) -> *const *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_Options17command_line_argvEv"]
            fn __ext(__this: *mut ACE_Process_Options) -> *const *mut libc::c_char;
        }
        __ext(__this as *mut ACE_Process_Options)
    }
    #[doc = "* Null-terminated buffer of null terminated strings.  Each string\n   * is an environment assignment \"VARIABLE=value\".  This buffer\n   * should end with two null characters."]
    pub unsafe fn env_buf(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_Options7env_bufEv"]
            fn __ext(__this: *mut ACE_Process_Options) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_Process_Options)
    }
    /**Get the process group.  On UNIX, these methods are used by the
  /// ACE_Process_Manager to manage groups of processes.*/
    pub unsafe fn getgroup(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).process_group_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Set the process group.  On UNIX, these methods are used by the
  /// ACE_Process_Manager to manage groups of processes.*/
    pub unsafe fn setgroup(__this: *mut Self, mut pgrp: libc::c_int) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut old: libc::c_int = (*__this).process_group_;
                (*__this).process_group_ = pgrp;
                return old;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Allows disabling of handle inheritance, default is TRUE.
  ///
  /// @remarks @b Windows: the handle_inheritance value is passed as the
  /// bInheritHandles value to the CreateProcess() system function. Therefore,
  /// if you redirect standard input, output, or error via
  /// ACE_Process_Options::set_handles() you must not call
  /// handle_inheritance(false). Doing so will prevent the duplicated handles
  /// from surviving in the created process.*/
    pub unsafe fn handle_inheritance(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).handle_inheritance_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn handle_inheritance_u72b0f29424a10ad6(
        __this: *mut Self,
        mut hi: libc::c_int,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).handle_inheritance_ = ((hi) != 0);
            }
            ()
        }
    }
    #[doc = "Cause the specified handle to be passed to a child process\n  /// when it runs a new program image.\n  /**\n   * The specified handle value will be included in the spawned\n   * process's command line as @arg +H @arg handle, if a new\n   * program is spawned (always on Win32; else if NO_EXEC is not\n   * set in creation flags).  The passed handle value will be\n   * duplicated if on Win32 less capable than NT.\n   * @return 0 if success, -1 if failure."]
    pub unsafe fn pass_handle(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_Options11pass_handleEi"]
            fn __ext(
                __this: *mut ACE_Process_Options,
                _anon_0: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Process_Options, _anon_0)
    }
    #[doc = "Get a copy of the handles the ACE_Process_Options duplicated\n  /// for the spawned process.\n  /**\n   * Any handles created through duplication of those passed into\n   * @arg pass_handle are returned in @arg set.\n   * @return 0 if there were no handles to return; 1 if there were."]
    pub unsafe fn dup_handles(
        __this: *const Self,
        mut set: *mut ACE_Handle_Set,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK19ACE_Process_Options11dup_handlesER14ACE_Handle_Set"]
            fn __ext(
                __this: *const ACE_Process_Options,
                set: *mut ACE_Handle_Set,
            ) -> libc::c_int;
        }
        __ext(__this as *const ACE_Process_Options, set)
    }
    #[doc = "Get a copy of the handles passed to the spawned process. This\n  /// will be the set of handles previously passed to @arg pass_handle().\n  /**\n   * Any handles previously passed to @arg pass_handle are returned\n   * in @arg set.\n   * @return 0 if there were no handles to return; 1 if there were."]
    pub unsafe fn passed_handles(
        __this: *const Self,
        mut set: *mut ACE_Handle_Set,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK19ACE_Process_Options14passed_handlesER14ACE_Handle_Set"]
            fn __ext(
                __this: *const ACE_Process_Options,
                set: *mut ACE_Handle_Set,
            ) -> libc::c_int;
        }
        __ext(__this as *const ACE_Process_Options, set)
    }
    ///Set value for avoid_zombies (has no real effect except on *nix).
    pub unsafe fn avoid_zombies(__this: *mut Self, mut avoid_zombies: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).avoid_zombies_ = avoid_zombies;
            }
            ()
        }
    }
    ///Get current value for avoid_zombies.
    pub unsafe fn avoid_zombies_uce94ebb5b3607812(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).avoid_zombies_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Enable the use of a Unicode environment.  This only makes sense
  /// for Win32 when ACE_USES_WCHAR is not defined.*/
    pub unsafe fn enable_unicode_environment(__this: *mut Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).use_unicode_environment_ = true;
            }
            ()
        }
    }
    ///Disable the use of a Unicode environment.
    pub unsafe fn disable_unicode_environment(__this: *mut Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).use_unicode_environment_ = false;
            }
            ()
        }
    }
    ///Return the unicode environment status
    pub unsafe fn use_unicode_environment(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    (((*__this).use_unicode_environment_) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///argv-style array of environment settings.
    pub unsafe fn env_argv(__this: *mut Self) -> *const *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_Options8env_argvEv"]
            fn __ext(__this: *mut ACE_Process_Options) -> *const *mut libc::c_char;
        }
        __ext(__this as *mut ACE_Process_Options)
    }
    pub unsafe fn get_stdin(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).stdin_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn get_stdout(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).stdout_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn get_stderr(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).stderr_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn setreugid(
        __this: *mut Self,
        mut user: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ent: *mut passwd = ACE_OS::getpwnam_u0e28381801657da9(
                    ((user) as *const libc::c_char),
                );
                if ((((!(ent).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    (*__this).euid_ = (((*ent).pw_uid) as libc::c_uint);
                    (*__this).ruid_ = (((*ent).pw_uid) as libc::c_uint);
                    (*__this).egid_ = (((*ent).pw_gid) as libc::c_uint);
                    (*__this).rgid_ = (((*ent).pw_gid) as libc::c_uint);
                    return 0;
                } else {
                    return (-((1) as libc::c_int));
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn setruid(__this: *mut Self, mut id: libc::c_uint) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).ruid_ = id;
            }
            ()
        }
    }
    pub unsafe fn seteuid(__this: *mut Self, mut id: libc::c_uint) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).euid_ = id;
            }
            ()
        }
    }
    pub unsafe fn setrgid(__this: *mut Self, mut id: libc::c_uint) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).rgid_ = id;
            }
            ()
        }
    }
    pub unsafe fn setegid(__this: *mut Self, mut id: libc::c_uint) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).egid_ = id;
            }
            ()
        }
    }
    pub unsafe fn getruid(__this: *const Self) -> libc::c_uint {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).ruid_) as libc::c_uint);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn geteuid(__this: *const Self) -> libc::c_uint {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).euid_) as libc::c_uint);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn getrgid(__this: *const Self) -> libc::c_uint {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).rgid_) as libc::c_uint);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn getegid(__this: *const Self) -> libc::c_uint {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).egid_) as libc::c_uint);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///* Get the inherit_environment flag.
    pub unsafe fn inherit_environment(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    (((*__this).inherit_environment_) as bool),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///* Set the inherit_environment flag.
    pub unsafe fn inherit_environment_u3baf8afa5687c389(
        __this: *mut Self,
        mut nv: bool,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).inherit_environment_ = nv;
            }
            ()
        }
    }
    /**Add @a assignment to environment_buf_ and adjust
  /// environment_argv_.  @a len is the strlen of @a assignment.*/
    pub unsafe fn setenv_i(
        __this: *mut Self,
        mut assignment: *mut libc::c_char,
        mut len: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Process_Options8setenv_iEPcm"]
            fn __ext(
                __this: *mut ACE_Process_Options,
                assignment: *mut libc::c_char,
                len: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Process_Options, assignment, len)
    }
}
impl ACE_Process {
    ///Default construction.  Use ACE_Process::spawn() to start a process.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_ProcessC1Ev"]
            fn __ext(__this: *mut ACE_Process);
        }
        __ext(__this as *mut ACE_Process)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    #[doc = "* Called back from spawn() just before spawning the child.  If this\n   * returns non-zero, the spawn is aborted (and returns ACE_INVALID_PID).\n   * The default returns zero."]
    pub unsafe fn prepare(
        __this: *mut Self,
        mut options: *mut ACE_Process_Options,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Process7prepareER19ACE_Process_Options"]
            fn __ext(
                __this: *mut ACE_Process,
                options: *mut ACE_Process_Options,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Process, options)
    }
    #[doc = "* Launch a new process as described by @a options.\n   *\n   * @retval -1 on failure; check @c errno for error code.\n   * @retval 1 on success if the option @c avoid_zombies is set.\n   * @retval other the process id of the newly spawned child.\n   *\n   * @note The return value 1 may be changed in future versions of ACE to be\n   * the process id of the child will be returned regardless of the\n   * @c avoid_zombies option.\n   *\n   * @note On UNIX platforms, spawn() uses the execvp() system call if\n   * ACE_Process_Options::inherit_environment() returns true (which is the\n   * default) and execve() if not. Since execve() does not search PATH, the\n   * ACE_Process_Options::command_line() should include a full path to the\n   * program file."]
    pub unsafe fn spawn(
        __this: *mut Self,
        mut options: *mut ACE_Process_Options,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Process5spawnER19ACE_Process_Options"]
            fn __ext(
                __this: *mut ACE_Process,
                options: *mut ACE_Process_Options,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Process, options)
    }
    /**Called back from spawn() in the parent's context just after forking,
  /// if the fork succeeds.  The default simply returns.*/
    pub unsafe fn parent(__this: *mut Self, mut child: libc::c_int) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Process6parentEi"]
            fn __ext(__this: *mut ACE_Process, child: libc::c_int);
        }
        __ext(__this as *mut ACE_Process, child)
    }
    #[doc = "* Called back from spawn() in the child's context just after forking.  The\n   * default does nothing.\n   *\n   * @note This function is *not* called on Windows\n   * because the process-creation scheme does not allow it."]
    pub unsafe fn child(__this: *mut Self, mut parent: libc::c_int) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Process5childEi"]
            fn __ext(__this: *mut ACE_Process, parent: libc::c_int);
        }
        __ext(__this as *mut ACE_Process, parent)
    }
    /**Called by a ACE_Process_Manager that is removing this object from
  /// its table of managed processes. Default is to do nothing.*/
    pub unsafe fn unmanage(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Process8unmanageEv"]
            fn __ext(__this: *mut ACE_Process);
        }
        __ext(__this as *mut ACE_Process)
    }
    #[doc = "* Wait for a previously spawned process to exit.\n   *\n   * @arg status Points to a location to receive the exit status of the\n   *      spawned process. Ignored if the value is 0.\n   * @arg wait_options If @c WNOHANG then return 0 and don't block if the\n   *      child process hasn't exited yet.\n   *\n   * @retval -1 the wait operation failed; consult @c errno for details.\n   * @retval other the child process id is returned on success."]
    pub unsafe fn wait(
        __this: *mut Self,
        mut status: *mut libc::c_int,
        mut wait_options: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut retv: libc::c_int = ACE_OS::wait_u9d999b984b9ff79a(
                    <ACE_Process>::getpid((__this) as *const ACE_Process),
                    ::core::ptr::addr_of_mut!((* __this).exit_code_) as *mut libc::c_int,
                    wait_options,
                    ((0) as libc::c_int),
                );
                if ((((!(status).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    (*(status)) = (*__this).exit_code_;
                }
                return retv;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Timed wait for a previously spawned process to exit.\n   *\n   * @arg tv A relative amount of time to wait for the process to exit.\n   * @arg status Points to a location to receive the exit status of the\n   *      spawned process. Ignored if the value is 0.\n   *\n   * @retval 0 the specified time period elapsed before the process exited.\n   * @retval -1 the wait operation failed; consult @c errno for details.\n   * @retval other the child process id is returned on success.\n   *\n   * @note On UNIX platforms this function uses @c ualarm(), i.e., it\n   * overwrites any existing alarm.  In addition, it steals all\n   * @c SIGCHLD signals during the timeout period, which will break another\n   * ACE_Process_Manager in the same process that's expecting\n   * @c SIGCHLD to kick off process reaping."]
    pub unsafe fn wait_u9c844e55a2eff714(
        __this: *mut Self,
        mut tv: *const ACE_Time_Value,
        mut status: *mut libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Process4waitERK14ACE_Time_ValuePi"]
            fn __ext(
                __this: *mut ACE_Process,
                tv: *const ACE_Time_Value,
                status: *mut libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Process, tv, status)
    }
    /**Send the process a signal.  This only has an effect on operating
  /// systems that support signals, such as UNIX/POSIX.*/
    pub unsafe fn kill(__this: *mut Self, mut signum: libc::c_int) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((<ACE_Process>::getpid((__this) as *const ACE_Process)
                    as libc::c_int)) != ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return ACE_OS::kill_udafa4deef137b5a3(
                        <ACE_Process>::getpid((__this) as *const ACE_Process),
                        signum,
                    );
                } else {
                    return (-((1) as libc::c_int));
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Terminate the process abruptly using ACE::terminate_process().\n   * This call doesn't give the process a chance to cleanup, so use it\n   * with caution."]
    pub unsafe fn terminate(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((<ACE_Process>::getpid((__this) as *const ACE_Process)
                    as libc::c_int)) != ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return ACE::terminate_process(
                        <ACE_Process>::getpid((__this) as *const ACE_Process),
                    );
                } else {
                    return (-((1) as libc::c_int));
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the process id of the new child process.
    pub unsafe fn getpid(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).child_id_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the handle of the process, if it has one.
    pub unsafe fn gethandle(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((*__this).child_id_ as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return 1 if running; 0 otherwise.
    pub unsafe fn running(__this: *const Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK11ACE_Process7runningEv"]
            fn __ext(__this: *const ACE_Process) -> libc::c_int;
        }
        __ext(__this as *const ACE_Process)
    }
    /**Return the process's exit code.  This method returns the raw
  /// exit status returned from system APIs (such as @c wait() or
  /// @c waitpid() ).  This value is system dependent.*/
    pub unsafe fn exit_code(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).exit_code_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Return the process's return value.  This method returns the
  /// actual return value that a child process returns or exits with.*/
    pub unsafe fn return_value(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((((((((((*__this).exit_code_)) as libc::c_int))
                    & ((65280) as libc::c_int))) as libc::c_int))
                    .wrapping_shr((8) as u32));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Close all the handles in the set obtained from the
  /// @a ACE_Process_Options::dup_handles object used to spawn
  /// the process.*/
    pub unsafe fn close_dup_handles(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Process17close_dup_handlesEv"]
            fn __ext(__this: *mut ACE_Process);
        }
        __ext(__this as *mut ACE_Process)
    }
    /**Close all the passed handles in the set obtained from the
  /// ACE_Process_Options object used to spawn the process.*/
    pub unsafe fn close_passed_handles(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Process20close_passed_handlesEv"]
            fn __ext(__this: *mut ACE_Process);
        }
        __ext(__this as *mut ACE_Process)
    }
    pub unsafe fn new_at_u23ab8d1c10286a38(
        __this: *mut Self,
        mut __a0: *const ACE_Process,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_ProcessC1ERKS_"]
            fn __ext(__this: *mut ACE_Process, __a0: *const ACE_Process);
        }
        __ext(__this as *mut ACE_Process, __a0)
    }
    pub unsafe fn new_u23ab8d1c10286a38(mut __a0: *const ACE_Process) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u23ab8d1c10286a38(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(__this: *mut Self, mut _anon_0: *const ACE_Process) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_ProcessaSERKS_"]
            fn __ext(__this: *mut ACE_Process, _anon_0: *const ACE_Process);
        }
        __ext(__this as *mut ACE_Process, _anon_0)
    }
    /**Set this process's exit code.  ACE_Process_Manager uses this
  /// method to set the exit code after successfully waiting for
  /// this process to exit.*/
    pub unsafe fn exit_code_uc86b462655ee0a17(__this: *mut Self, mut code: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).exit_code_ = code;
            }
            ()
        }
    }
}
impl ACE_Managed_Process {
    ///Cleanup by deleting @c this.
    pub unsafe fn unmanage(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Managed_Process8unmanageEv"]
            fn __ext(__this: *mut ACE_Managed_Process);
        }
        __ext(__this as *mut ACE_Managed_Process)
    }
}
impl ACE_RW_Mutex {
    ///Initialize a readers/writer lock.
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: libc::c_int,
        mut __a1: *const libc::c_char,
        mut __a2: *mut libc::c_void,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_RW_MutexC1EiPKcPv"]
            fn __ext(
                __this: *mut ACE_RW_Mutex,
                __a0: libc::c_int,
                __a1: *const libc::c_char,
                __a2: *mut libc::c_void,
            );
        }
        __ext(__this as *mut ACE_RW_Mutex, __a0, __a1, __a2)
    }
    pub unsafe fn new(
        mut __a0: libc::c_int,
        mut __a1: *const libc::c_char,
        mut __a2: *mut libc::c_void,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1, __a2);
        __obj
    }
    #[doc = "* Explicitly destroy a readers/writer lock.  Note that only one\n   * thread should call this method since it doesn't protect against\n   * race conditions."]
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut result: libc::c_int = 0;
                if (((!((((*__this).removed_ as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    (*__this).removed_ = true;
                    result = ACE_OS::rwlock_destroy(
                        ::core::ptr::addr_of_mut!((* __this).lock_)
                            as *mut pthread_rwlock_t,
                    );
                }
                return result;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Acquire a read lock, but block if a writer hold the lock.
    pub unsafe fn acquire_read(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::rw_rdlock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_rwlock_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Acquire a write lock, but block if any readers or a
  /// writer hold the lock.*/
    pub unsafe fn acquire_write(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::rw_wrlock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_rwlock_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Conditionally acquire a read lock (i.e., won't block).  Returns\n   * -1 on failure.  If we \"failed\" because someone else already had\n   * the lock, @c errno is set to @c EBUSY."]
    pub unsafe fn tryacquire_read(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::rw_tryrdlock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_rwlock_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Conditionally acquire a write lock (i.e., won't block).
    pub unsafe fn tryacquire_write(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::rw_trywrlock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_rwlock_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Conditionally upgrade a read lock to a write lock.  This only\n   * works if there are no other readers present, in which case the\n   * method returns 0.  Otherwise, the method returns -1 and sets\n   * @c errno to @c EBUSY.  Note that the caller of this method *must*\n   * already possess this lock as a read lock (but this condition is\n   * not checked by the current implementation)."]
    pub unsafe fn tryacquire_write_upgrade(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::rw_trywrlock_upgrade(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_rwlock_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Note, for interface uniformity with other synchronization\n   * wrappers we include the <acquire> method.  This is implemented as\n   * a write-lock to safe..."]
    pub unsafe fn acquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::rw_wrlock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_rwlock_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Note, for interface uniformity with other synchronization\n   * wrappers we include the tryacquire() method.  This is implemented\n   * as a write-lock to be safe...  Returns -1 on failure.  If we\n   * \"failed\" because someone else already had the lock, @c errno is\n   * set to @c EBUSY."]
    pub unsafe fn tryacquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_RW_Mutex>::tryacquire_write((__this) as *mut ACE_RW_Mutex);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Unlock a readers/writer lock.
    pub unsafe fn release(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::rw_unlock(
                    ::core::ptr::addr_of_mut!((* __this).lock_) as *mut pthread_rwlock_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the underlying lock.
    pub unsafe fn lock(__this: *const Self) -> *const pthread_rwlock_t {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of!((* __this).lock_);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK12ACE_RW_Mutex4dumpEv"]
            fn __ext(__this: *const ACE_RW_Mutex);
        }
        __ext(__this as *const ACE_RW_Mutex)
    }
    pub unsafe fn operator_assign(__this: *mut Self, mut _anon_0: *const ACE_RW_Mutex) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_RW_MutexaSERKS_"]
            fn __ext(__this: *mut ACE_RW_Mutex, _anon_0: *const ACE_RW_Mutex);
        }
        __ext(__this as *mut ACE_RW_Mutex, _anon_0)
    }
    pub unsafe fn new_at_u46eae7555b4891d0(
        __this: *mut Self,
        mut __a0: *const ACE_RW_Mutex,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_RW_MutexC1ERKS_"]
            fn __ext(__this: *mut ACE_RW_Mutex, __a0: *const ACE_RW_Mutex);
        }
        __ext(__this as *mut ACE_RW_Mutex, __a0)
    }
    pub unsafe fn new_u46eae7555b4891d0(mut __a0: *const ACE_RW_Mutex) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u46eae7555b4891d0(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_Atomic_Op_ACE_Thread_Mutex__int_ {
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_int_>::new_at_sb2afbc14dc15fb76(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_int_,
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
    pub unsafe fn new_at_u52259338b7dfec84(__this: *mut Self, mut c: libc::c_int) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_int_>::new_at_sf76c32f2735f2713(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_int_,
                c,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u52259338b7dfec84(mut __a0: libc::c_int) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u52259338b7dfec84(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_u39a5191e90dd80e9(
        __this: *mut Self,
        mut c: *const ACE_Atomic_Op_ACE_Thread_Mutex__int_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_int_>::new_at_s3fb4016ff2eb03e7(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_int_,
                ::core::ptr::addr_of!(((* c)).__base_0) as *const ACE_Atomic_Op_GCC_int_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u39a5191e90dd80e9(
        mut __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__int_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u39a5191e90dd80e9(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: libc::c_int,
    ) -> *mut ACE_Atomic_Op_ACE_Thread_Mutex__int_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Atomic_Op_GCC_int_>::operator_assign_sa6183fb5824b891a(
                    (::core::ptr::addr_of_mut!((* __this).__base_0))
                        as *mut ACE_Atomic_Op_GCC_int_,
                    rhs,
                );
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_GCC_int_ {
    ///Atomically pre-increment @c value_.
    pub unsafe fn operator_inc_s8079f1940e04efea(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiEppEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_int_) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_)
    }
    ///Atomically post-increment @c value_.
    pub unsafe fn operator_inc_scec30cab4fbe9357(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiEppEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_int_,
                _anon_0: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_, _anon_0)
    }
    ///Atomically increment @c value_ by rhs.
    pub unsafe fn operator_add_assign(
        __this: *mut Self,
        mut rhs: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiEpLEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_int_,
                rhs: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_, rhs)
    }
    ///Atomically pre-decrement @c value_.
    pub unsafe fn operator_dec_s8079f1940e04efea(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiEmmEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_int_) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_)
    }
    ///Atomically post-decrement @c value_.
    pub unsafe fn operator_dec_scec30cab4fbe9357(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiEmmEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_int_,
                _anon_0: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_, _anon_0)
    }
    ///Atomically decrement @c value_ by rhs.
    pub unsafe fn operator_sub_assign(
        __this: *mut Self,
        mut rhs: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiEmIEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_int_,
                rhs: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_eq(__this: *const Self, mut rhs: libc::c_int) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIiEeqEi"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_int_, rhs: libc::c_int) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_int_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_ne(__this: *const Self, mut rhs: libc::c_int) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIiEneEi"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_int_, rhs: libc::c_int) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_int_, rhs)
    }
    ///Atomically check if @c value_ greater than or equal to rhs.
    pub unsafe fn operator_ge(__this: *const Self, mut rhs: libc::c_int) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIiEgeEi"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_int_, rhs: libc::c_int) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_int_, rhs)
    }
    ///Atomically check if @c value_ greater than rhs.
    pub unsafe fn operator_gt(__this: *const Self, mut rhs: libc::c_int) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIiEgtEi"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_int_, rhs: libc::c_int) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_int_, rhs)
    }
    ///Atomically check if @c value_ less than or equal to rhs.
    pub unsafe fn operator_le(__this: *const Self, mut rhs: libc::c_int) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIiEleEi"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_int_, rhs: libc::c_int) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_int_, rhs)
    }
    ///Atomically check if @c value_ less than rhs.
    pub unsafe fn operator_lt(__this: *const Self, mut rhs: libc::c_int) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIiEltEi"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_int_, rhs: libc::c_int) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_int_, rhs)
    }
    ///Exchange value with @a newval.
    pub unsafe fn exchange(__this: *mut Self, mut newval: libc::c_int) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiE8exchangeEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_int_,
                newval: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_, newval)
    }
    ///Explicitly return @c value_.
    pub unsafe fn value(__this: *const Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIiE5valueEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_int_) -> libc::c_int;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_int_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIiE4dumpEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_int_);
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_int_)
    }
    ///Explicitly return @c value_ (by reference).
    pub unsafe fn value_i(__this: *mut Self) -> *mut libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiE7value_iEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_int_) -> *mut libc::c_int;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_)
    }
    ///Atomically assign <rhs> to @c value_.
    pub unsafe fn operator_assign_s7da516af3658cc2d(
        __this: *mut Self,
        mut rhs: *mut ACE_Atomic_Op_GCC_int_,
    ) -> *mut ACE_Atomic_Op_GCC_int_ {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiEaSERKS0_"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_int_,
                rhs: *mut ACE_Atomic_Op_GCC_int_,
            ) -> *mut ACE_Atomic_Op_GCC_int_;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_, rhs)
    }
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIiE5mutexEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_int_) -> *mut ACE_Thread_Mutex;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_int_)
    }
    ///Initialize @c value_ to 0.
    pub unsafe fn new_at_sb2afbc14dc15fb76(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((0) as libc::c_int),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb2afbc14dc15fb76() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb2afbc14dc15fb76(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Initialize @c value_ to c.
    pub unsafe fn new_at_sf76c32f2735f2713(__this: *mut Self, mut c: libc::c_int) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((c) as libc::c_int),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sf76c32f2735f2713(mut __a0: libc::c_int) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sf76c32f2735f2713(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Manage copying...
    pub unsafe fn new_at_s3fb4016ff2eb03e7(
        __this: *mut Self,
        mut rhs: *const ACE_Atomic_Op_GCC_int_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((unsafe {
                    ::core::ptr::read_volatile(::core::ptr::addr_of!((* rhs).value_))
                }) as libc::c_int),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s3fb4016ff2eb03e7(
        mut __a0: *const ACE_Atomic_Op_GCC_int_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s3fb4016ff2eb03e7(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Atomically assign rhs to @c value_.
    pub unsafe fn operator_assign_sa6183fb5824b891a(
        __this: *mut Self,
        mut rhs: libc::c_int,
    ) -> *mut ACE_Atomic_Op_GCC_int_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    (unsafe {
                        &*((::core::ptr::addr_of_mut!((* __this).value_)
                            as *mut libc::c_int)
                            as *const ::core::sync::atomic::AtomicI32)
                    })
                        .store(
                            (((rhs) as libc::c_int)) as i32,
                            ::core::sync::atomic::Ordering::Release,
                        );
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_ {
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_int_>::new_at_sb2af9c14dc15c516(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_int_,
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
    pub unsafe fn new_at_ufefa9eb56a8f4964(__this: *mut Self, mut c: libc::c_uint) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_int_>::new_at_sb1d76ea393b6b1ee(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_int_,
                c,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_ufefa9eb56a8f4964(mut __a0: libc::c_uint) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ufefa9eb56a8f4964(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_u4525c24972278e49(
        __this: *mut Self,
        mut c: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_int_>::new_at_sffc62675d9b85727(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_int_,
                ::core::ptr::addr_of!(((* c)).__base_0)
                    as *const ACE_Atomic_Op_GCC_unsigned_int_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u4525c24972278e49(
        mut __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u4525c24972278e49(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: libc::c_uint,
    ) -> *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_int_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Atomic_Op_GCC_unsigned_int_>::operator_assign_s98c244cb4408b555(
                    (::core::ptr::addr_of_mut!((* __this).__base_0))
                        as *mut ACE_Atomic_Op_GCC_unsigned_int_,
                    rhs,
                );
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_GCC_unsigned_int_ {
    ///Atomically pre-increment @c value_.
    pub unsafe fn operator_inc_sd3df00105d874b6b(__this: *mut Self) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjEppEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_unsigned_int_) -> libc::c_uint;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_)
    }
    ///Atomically post-increment @c value_.
    pub unsafe fn operator_inc_sbaa26e1b39b18202(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjEppEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
                _anon_0: libc::c_int,
            ) -> libc::c_uint;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_, _anon_0)
    }
    ///Atomically increment @c value_ by rhs.
    pub unsafe fn operator_add_assign(
        __this: *mut Self,
        mut rhs: libc::c_uint,
    ) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjEpLEj"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
                rhs: libc::c_uint,
            ) -> libc::c_uint;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_, rhs)
    }
    ///Atomically pre-decrement @c value_.
    pub unsafe fn operator_dec_sd3df00105d874b6b(__this: *mut Self) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjEmmEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_unsigned_int_) -> libc::c_uint;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_)
    }
    ///Atomically post-decrement @c value_.
    pub unsafe fn operator_dec_sbaa26e1b39b18202(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjEmmEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
                _anon_0: libc::c_int,
            ) -> libc::c_uint;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_, _anon_0)
    }
    ///Atomically decrement @c value_ by rhs.
    pub unsafe fn operator_sub_assign(
        __this: *mut Self,
        mut rhs: libc::c_uint,
    ) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjEmIEj"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
                rhs: libc::c_uint,
            ) -> libc::c_uint;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_eq(__this: *const Self, mut rhs: libc::c_uint) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIjEeqEj"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_int_,
                rhs: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_int_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_ne(__this: *const Self, mut rhs: libc::c_uint) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIjEneEj"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_int_,
                rhs: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_int_, rhs)
    }
    ///Atomically check if @c value_ greater than or equal to rhs.
    pub unsafe fn operator_ge(__this: *const Self, mut rhs: libc::c_uint) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIjEgeEj"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_int_,
                rhs: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_int_, rhs)
    }
    ///Atomically check if @c value_ greater than rhs.
    pub unsafe fn operator_gt(__this: *const Self, mut rhs: libc::c_uint) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIjEgtEj"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_int_,
                rhs: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_int_, rhs)
    }
    ///Atomically check if @c value_ less than or equal to rhs.
    pub unsafe fn operator_le(__this: *const Self, mut rhs: libc::c_uint) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIjEleEj"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_int_,
                rhs: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_int_, rhs)
    }
    ///Atomically check if @c value_ less than rhs.
    pub unsafe fn operator_lt(__this: *const Self, mut rhs: libc::c_uint) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIjEltEj"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_int_,
                rhs: libc::c_uint,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_int_, rhs)
    }
    ///Exchange value with @a newval.
    pub unsafe fn exchange(__this: *mut Self, mut newval: libc::c_uint) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjE8exchangeEj"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
                newval: libc::c_uint,
            ) -> libc::c_uint;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_, newval)
    }
    ///Explicitly return @c value_.
    pub unsafe fn value(__this: *const Self) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIjE5valueEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_unsigned_int_) -> libc::c_uint;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_int_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIjE4dumpEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_unsigned_int_);
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_int_)
    }
    ///Explicitly return @c value_ (by reference).
    pub unsafe fn value_i(__this: *mut Self) -> *mut libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjE7value_iEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_unsigned_int_) -> *mut libc::c_uint;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_)
    }
    ///Atomically assign <rhs> to @c value_.
    pub unsafe fn operator_assign_sd4b07985d4d038cd(
        __this: *mut Self,
        mut rhs: *mut ACE_Atomic_Op_GCC_unsigned_int_,
    ) -> *mut ACE_Atomic_Op_GCC_unsigned_int_ {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjEaSERKS0_"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
                rhs: *mut ACE_Atomic_Op_GCC_unsigned_int_,
            ) -> *mut ACE_Atomic_Op_GCC_unsigned_int_;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_, rhs)
    }
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIjE5mutexEv"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_int_,
            ) -> *mut ACE_Thread_Mutex;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_int_)
    }
    ///Initialize @c value_ to 0.
    pub unsafe fn new_at_sb2af9c14dc15c516(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((0) as libc::c_uint),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb2af9c14dc15c516() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb2af9c14dc15c516(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Manage copying...
    pub unsafe fn new_at_sffc62675d9b85727(
        __this: *mut Self,
        mut rhs: *const ACE_Atomic_Op_GCC_unsigned_int_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((unsafe {
                    ::core::ptr::read_volatile(::core::ptr::addr_of!((* rhs).value_))
                }) as libc::c_uint),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sffc62675d9b85727(
        mut __a0: *const ACE_Atomic_Op_GCC_unsigned_int_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sffc62675d9b85727(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Initialize @c value_ to c.
    pub unsafe fn new_at_sb1d76ea393b6b1ee(__this: *mut Self, mut c: libc::c_uint) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((c) as libc::c_uint),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb1d76ea393b6b1ee(mut __a0: libc::c_uint) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb1d76ea393b6b1ee(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Atomically assign rhs to @c value_.
    pub unsafe fn operator_assign_s98c244cb4408b555(
        __this: *mut Self,
        mut rhs: libc::c_uint,
    ) -> *mut ACE_Atomic_Op_GCC_unsigned_int_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    (unsafe {
                        &*((::core::ptr::addr_of_mut!((* __this).value_)
                            as *mut libc::c_uint)
                            as *const ::core::sync::atomic::AtomicU32)
                    })
                        .store(
                            (((rhs) as libc::c_uint)) as u32,
                            ::core::sync::atomic::Ordering::Release,
                        );
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_ACE_Thread_Mutex__long_ {
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_long_>::new_at_sb2afc114dc1603f5(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_long_,
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
    pub unsafe fn new_at_u3fc757b24fef6c3f(__this: *mut Self, mut c: libc::c_long) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_long_>::new_at_sef6195b741b75f97(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_long_,
                c,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u3fc757b24fef6c3f(mut __a0: libc::c_long) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u3fc757b24fef6c3f(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_u0822bd45516f190a(
        __this: *mut Self,
        mut c: *const ACE_Atomic_Op_ACE_Thread_Mutex__long_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_long_>::new_at_s06c4fe3ad40cd3fb(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_long_,
                ::core::ptr::addr_of!(((* c)).__base_0) as *const ACE_Atomic_Op_GCC_long_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u0822bd45516f190a(
        mut __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__long_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u0822bd45516f190a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: libc::c_long,
    ) -> *mut ACE_Atomic_Op_ACE_Thread_Mutex__long_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Atomic_Op_GCC_long_>::operator_assign_s5aa91e1f83c07555(
                    (::core::ptr::addr_of_mut!((* __this).__base_0))
                        as *mut ACE_Atomic_Op_GCC_long_,
                    rhs,
                );
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_GCC_long_ {
    ///Atomically pre-increment @c value_.
    pub unsafe fn operator_inc_s6da53671d6b482ca(__this: *mut Self) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlEppEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_long_) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_)
    }
    ///Atomically post-increment @c value_.
    pub unsafe fn operator_inc_s6d88ed27c94676eb(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlEppEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_,
                _anon_0: libc::c_int,
            ) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_, _anon_0)
    }
    ///Atomically increment @c value_ by rhs.
    pub unsafe fn operator_add_assign(
        __this: *mut Self,
        mut rhs: libc::c_long,
    ) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlEpLEl"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_,
                rhs: libc::c_long,
            ) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_, rhs)
    }
    ///Atomically pre-decrement @c value_.
    pub unsafe fn operator_dec_s6da53671d6b482ca(__this: *mut Self) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlEmmEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_long_) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_)
    }
    ///Atomically post-decrement @c value_.
    pub unsafe fn operator_dec_s6d88ed27c94676eb(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlEmmEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_,
                _anon_0: libc::c_int,
            ) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_, _anon_0)
    }
    ///Atomically decrement @c value_ by rhs.
    pub unsafe fn operator_sub_assign(
        __this: *mut Self,
        mut rhs: libc::c_long,
    ) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlEmIEl"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_,
                rhs: libc::c_long,
            ) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_eq(__this: *const Self, mut rhs: libc::c_long) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIlEeqEl"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_, rhs: libc::c_long) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_ne(__this: *const Self, mut rhs: libc::c_long) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIlEneEl"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_, rhs: libc::c_long) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_, rhs)
    }
    ///Atomically check if @c value_ greater than or equal to rhs.
    pub unsafe fn operator_ge(__this: *const Self, mut rhs: libc::c_long) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIlEgeEl"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_, rhs: libc::c_long) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_, rhs)
    }
    ///Atomically check if @c value_ greater than rhs.
    pub unsafe fn operator_gt(__this: *const Self, mut rhs: libc::c_long) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIlEgtEl"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_, rhs: libc::c_long) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_, rhs)
    }
    ///Atomically check if @c value_ less than or equal to rhs.
    pub unsafe fn operator_le(__this: *const Self, mut rhs: libc::c_long) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIlEleEl"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_, rhs: libc::c_long) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_, rhs)
    }
    ///Atomically check if @c value_ less than rhs.
    pub unsafe fn operator_lt(__this: *const Self, mut rhs: libc::c_long) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIlEltEl"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_, rhs: libc::c_long) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_, rhs)
    }
    ///Exchange value with @a newval.
    pub unsafe fn exchange(__this: *mut Self, mut newval: libc::c_long) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlE8exchangeEl"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_,
                newval: libc::c_long,
            ) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_, newval)
    }
    ///Explicitly return @c value_.
    pub unsafe fn value(__this: *const Self) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIlE5valueEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_) -> libc::c_long;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIlE4dumpEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_);
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_)
    }
    ///Explicitly return @c value_ (by reference).
    pub unsafe fn value_i(__this: *mut Self) -> *mut libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlE7value_iEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_long_) -> *mut libc::c_long;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_)
    }
    ///Atomically assign <rhs> to @c value_.
    pub unsafe fn operator_assign_s4aee3a57f777891e(
        __this: *mut Self,
        mut rhs: *mut ACE_Atomic_Op_GCC_long_long_,
    ) -> *mut ACE_Atomic_Op_GCC_long_long_ {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlEaSERKS0_"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_,
                rhs: *mut ACE_Atomic_Op_GCC_long_long_,
            ) -> *mut ACE_Atomic_Op_GCC_long_long_;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_, rhs)
    }
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIlE5mutexEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_long_) -> *mut ACE_Thread_Mutex;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_)
    }
    ///Initialize @c value_ to 0.
    pub unsafe fn new_at_sb2afc114dc1603f5(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((0) as libc::c_long),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb2afc114dc1603f5() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb2afc114dc1603f5(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Initialize @c value_ to c.
    pub unsafe fn new_at_sef6195b741b75f97(__this: *mut Self, mut c: libc::c_long) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((c) as libc::c_long),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sef6195b741b75f97(mut __a0: libc::c_long) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sef6195b741b75f97(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Manage copying...
    pub unsafe fn new_at_s06c4fe3ad40cd3fb(
        __this: *mut Self,
        mut rhs: *const ACE_Atomic_Op_GCC_long_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((unsafe {
                    ::core::ptr::read_volatile(::core::ptr::addr_of!((* rhs).value_))
                }) as libc::c_long),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s06c4fe3ad40cd3fb(
        mut __a0: *const ACE_Atomic_Op_GCC_long_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s06c4fe3ad40cd3fb(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Atomically assign rhs to @c value_.
    pub unsafe fn operator_assign_s5aa91e1f83c07555(
        __this: *mut Self,
        mut rhs: libc::c_long,
    ) -> *mut ACE_Atomic_Op_GCC_long_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    (unsafe {
                        &*((::core::ptr::addr_of_mut!((* __this).value_)
                            as *mut libc::c_long)
                            as *const ::core::sync::atomic::AtomicI64)
                    })
                        .store(
                            (((rhs) as libc::c_long)) as i64,
                            ::core::sync::atomic::Ordering::Release,
                        );
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_ {
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_long_>::new_at_sb2afa114dc15cd95(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_long_,
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
    pub unsafe fn new_at_u8abafd19dfaf07df(__this: *mut Self, mut c: libc::c_ulong) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_long_>::new_at_s801634c3aabc0cfe(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_long_,
                c,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u8abafd19dfaf07df(mut __a0: libc::c_ulong) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u8abafd19dfaf07df(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_ueb6cc6249a36382a(
        __this: *mut Self,
        mut c: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_long_>::new_at_scf5b98a64f65a4fb(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_long_,
                ::core::ptr::addr_of!(((* c)).__base_0)
                    as *const ACE_Atomic_Op_GCC_unsigned_long_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_ueb6cc6249a36382a(
        mut __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ueb6cc6249a36382a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: libc::c_ulong,
    ) -> *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Atomic_Op_GCC_unsigned_long_>::operator_assign_se8d4e6e2b543c0a6(
                    (::core::ptr::addr_of_mut!((* __this).__base_0))
                        as *mut ACE_Atomic_Op_GCC_unsigned_long_,
                    rhs,
                );
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_GCC_unsigned_long_ {
    ///Atomically pre-increment @c value_.
    pub unsafe fn operator_inc_s47a179809ada7b0f(__this: *mut Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImEppEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_unsigned_long_) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_)
    }
    ///Atomically post-increment @c value_.
    pub unsafe fn operator_inc_sf832a7bdb99cff02(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImEppEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
                _anon_0: libc::c_int,
            ) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_, _anon_0)
    }
    ///Atomically increment @c value_ by rhs.
    pub unsafe fn operator_add_assign(
        __this: *mut Self,
        mut rhs: libc::c_ulong,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImEpLEm"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
                rhs: libc::c_ulong,
            ) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_, rhs)
    }
    ///Atomically pre-decrement @c value_.
    pub unsafe fn operator_dec_s47a179809ada7b0f(__this: *mut Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImEmmEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_unsigned_long_) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_)
    }
    ///Atomically post-decrement @c value_.
    pub unsafe fn operator_dec_sf832a7bdb99cff02(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImEmmEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
                _anon_0: libc::c_int,
            ) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_, _anon_0)
    }
    ///Atomically decrement @c value_ by rhs.
    pub unsafe fn operator_sub_assign(
        __this: *mut Self,
        mut rhs: libc::c_ulong,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImEmIEm"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
                rhs: libc::c_ulong,
            ) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_eq(__this: *const Self, mut rhs: libc::c_ulong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCImEeqEm"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_,
                rhs: libc::c_ulong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_ne(__this: *const Self, mut rhs: libc::c_ulong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCImEneEm"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_,
                rhs: libc::c_ulong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_, rhs)
    }
    ///Atomically check if @c value_ greater than or equal to rhs.
    pub unsafe fn operator_ge(__this: *const Self, mut rhs: libc::c_ulong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCImEgeEm"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_,
                rhs: libc::c_ulong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_, rhs)
    }
    ///Atomically check if @c value_ greater than rhs.
    pub unsafe fn operator_gt(__this: *const Self, mut rhs: libc::c_ulong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCImEgtEm"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_,
                rhs: libc::c_ulong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_, rhs)
    }
    ///Atomically check if @c value_ less than or equal to rhs.
    pub unsafe fn operator_le(__this: *const Self, mut rhs: libc::c_ulong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCImEleEm"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_,
                rhs: libc::c_ulong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_, rhs)
    }
    ///Atomically check if @c value_ less than rhs.
    pub unsafe fn operator_lt(__this: *const Self, mut rhs: libc::c_ulong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCImEltEm"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_,
                rhs: libc::c_ulong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_, rhs)
    }
    ///Exchange value with @a newval.
    pub unsafe fn exchange(
        __this: *mut Self,
        mut newval: libc::c_ulong,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImE8exchangeEm"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
                newval: libc::c_ulong,
            ) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_, newval)
    }
    ///Explicitly return @c value_.
    pub unsafe fn value(__this: *const Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCImE5valueEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_unsigned_long_) -> libc::c_ulong;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCImE4dumpEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_unsigned_long_);
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_)
    }
    ///Explicitly return @c value_ (by reference).
    pub unsafe fn value_i(__this: *mut Self) -> *mut libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImE7value_iEv"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
            ) -> *mut libc::c_ulong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_)
    }
    ///Atomically assign <rhs> to @c value_.
    pub unsafe fn operator_assign_sd3fcfd79df0abc3e(
        __this: *mut Self,
        mut rhs: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
    ) -> *mut ACE_Atomic_Op_GCC_unsigned_long_long_ {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImEaSERKS0_"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
                rhs: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
            ) -> *mut ACE_Atomic_Op_GCC_unsigned_long_long_;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_, rhs)
    }
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCImE5mutexEv"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_,
            ) -> *mut ACE_Thread_Mutex;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_)
    }
    ///Initialize @c value_ to 0.
    pub unsafe fn new_at_sb2afa114dc15cd95(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((0) as libc::c_ulong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb2afa114dc15cd95() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb2afa114dc15cd95(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Initialize @c value_ to c.
    pub unsafe fn new_at_s801634c3aabc0cfe(__this: *mut Self, mut c: libc::c_ulong) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((c) as libc::c_ulong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s801634c3aabc0cfe(mut __a0: libc::c_ulong) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s801634c3aabc0cfe(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Manage copying...
    pub unsafe fn new_at_scf5b98a64f65a4fb(
        __this: *mut Self,
        mut rhs: *const ACE_Atomic_Op_GCC_unsigned_long_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((unsafe {
                    ::core::ptr::read_volatile(::core::ptr::addr_of!((* rhs).value_))
                }) as libc::c_ulong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_scf5b98a64f65a4fb(
        mut __a0: *const ACE_Atomic_Op_GCC_unsigned_long_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_scf5b98a64f65a4fb(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Atomically assign rhs to @c value_.
    pub unsafe fn operator_assign_se8d4e6e2b543c0a6(
        __this: *mut Self,
        mut rhs: libc::c_ulong,
    ) -> *mut ACE_Atomic_Op_GCC_unsigned_long_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    (unsafe {
                        &*((::core::ptr::addr_of_mut!((* __this).value_)
                            as *mut libc::c_ulong)
                            as *const ::core::sync::atomic::AtomicU64)
                    })
                        .store(
                            (((rhs) as libc::c_ulong)) as u64,
                            ::core::sync::atomic::Ordering::Release,
                        );
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_ACE_Thread_Mutex__long_long_ {
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_long_long_>::new_at_sb2afba14dc15f810(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_long_long_,
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
    pub unsafe fn new_at_u796ec9126c443db6(__this: *mut Self, mut c: libc::c_longlong) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_long_long_>::new_at_s8ac76eb38ea25f2e(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_long_long_,
                c,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u796ec9126c443db6(mut __a0: libc::c_longlong) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u796ec9126c443db6(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_u1994d2c095adb793(
        __this: *mut Self,
        mut c: *const ACE_Atomic_Op_ACE_Thread_Mutex__long_long_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_long_long_>::new_at_s64b54c651f3e4c5f(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_long_long_,
                ::core::ptr::addr_of!(((* c)).__base_0)
                    as *const ACE_Atomic_Op_GCC_long_long_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u1994d2c095adb793(
        mut __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__long_long_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u1994d2c095adb793(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: libc::c_longlong,
    ) -> *mut ACE_Atomic_Op_ACE_Thread_Mutex__long_long_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Atomic_Op_GCC_long_long_>::operator_assign_s841d310ddcf20e8f(
                    (::core::ptr::addr_of_mut!((* __this).__base_0))
                        as *mut ACE_Atomic_Op_GCC_long_long_,
                    rhs,
                );
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_GCC_long_long_ {
    ///Atomically pre-increment @c value_.
    pub unsafe fn operator_inc_sc97478295f3ef95b(__this: *mut Self) -> libc::c_longlong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxEppEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_long_long_) -> libc::c_longlong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_)
    }
    ///Atomically post-increment @c value_.
    pub unsafe fn operator_inc_s4e24900ee6c55bc2(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_longlong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxEppEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_long_,
                _anon_0: libc::c_int,
            ) -> libc::c_longlong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_, _anon_0)
    }
    ///Atomically increment @c value_ by rhs.
    pub unsafe fn operator_add_assign(
        __this: *mut Self,
        mut rhs: libc::c_longlong,
    ) -> libc::c_longlong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxEpLEx"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_long_,
                rhs: libc::c_longlong,
            ) -> libc::c_longlong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_, rhs)
    }
    ///Atomically pre-decrement @c value_.
    pub unsafe fn operator_dec_sc97478295f3ef95b(__this: *mut Self) -> libc::c_longlong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxEmmEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_long_long_) -> libc::c_longlong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_)
    }
    ///Atomically post-decrement @c value_.
    pub unsafe fn operator_dec_s4e24900ee6c55bc2(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_longlong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxEmmEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_long_,
                _anon_0: libc::c_int,
            ) -> libc::c_longlong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_, _anon_0)
    }
    ///Atomically decrement @c value_ by rhs.
    pub unsafe fn operator_sub_assign(
        __this: *mut Self,
        mut rhs: libc::c_longlong,
    ) -> libc::c_longlong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxEmIEx"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_long_,
                rhs: libc::c_longlong,
            ) -> libc::c_longlong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_eq(__this: *const Self, mut rhs: libc::c_longlong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIxEeqEx"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_long_long_,
                rhs: libc::c_longlong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_long_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_ne(__this: *const Self, mut rhs: libc::c_longlong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIxEneEx"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_long_long_,
                rhs: libc::c_longlong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_long_, rhs)
    }
    ///Atomically check if @c value_ greater than or equal to rhs.
    pub unsafe fn operator_ge(__this: *const Self, mut rhs: libc::c_longlong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIxEgeEx"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_long_long_,
                rhs: libc::c_longlong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_long_, rhs)
    }
    ///Atomically check if @c value_ greater than rhs.
    pub unsafe fn operator_gt(__this: *const Self, mut rhs: libc::c_longlong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIxEgtEx"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_long_long_,
                rhs: libc::c_longlong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_long_, rhs)
    }
    ///Atomically check if @c value_ less than or equal to rhs.
    pub unsafe fn operator_le(__this: *const Self, mut rhs: libc::c_longlong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIxEleEx"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_long_long_,
                rhs: libc::c_longlong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_long_, rhs)
    }
    ///Atomically check if @c value_ less than rhs.
    pub unsafe fn operator_lt(__this: *const Self, mut rhs: libc::c_longlong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIxEltEx"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_long_long_,
                rhs: libc::c_longlong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_long_, rhs)
    }
    ///Exchange value with @a newval.
    pub unsafe fn exchange(
        __this: *mut Self,
        mut newval: libc::c_longlong,
    ) -> libc::c_longlong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxE8exchangeEx"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_long_,
                newval: libc::c_longlong,
            ) -> libc::c_longlong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_, newval)
    }
    ///Explicitly return @c value_.
    pub unsafe fn value(__this: *const Self) -> libc::c_longlong {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIxE5valueEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_long_) -> libc::c_longlong;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_long_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIxE4dumpEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_long_long_);
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_long_long_)
    }
    ///Explicitly return @c value_ (by reference).
    pub unsafe fn value_i(__this: *mut Self) -> *mut libc::c_longlong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxE7value_iEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_long_long_) -> *mut libc::c_longlong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_)
    }
    ///Atomically assign <rhs> to @c value_.
    pub unsafe fn operator_assign_s14de97bf6bf5a27b(
        __this: *mut Self,
        mut rhs: *mut ACE_Atomic_Op_GCC_long_long_,
    ) -> *mut ACE_Atomic_Op_GCC_long_long_ {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxEaSERKS0_"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_long_long_,
                rhs: *mut ACE_Atomic_Op_GCC_long_long_,
            ) -> *mut ACE_Atomic_Op_GCC_long_long_;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_, rhs)
    }
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIxE5mutexEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_long_long_) -> *mut ACE_Thread_Mutex;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_long_long_)
    }
    ///Initialize @c value_ to 0.
    pub unsafe fn new_at_sb2afba14dc15f810(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((0) as libc::c_longlong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb2afba14dc15f810() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb2afba14dc15f810(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Initialize @c value_ to c.
    pub unsafe fn new_at_s8ac76eb38ea25f2e(__this: *mut Self, mut c: libc::c_longlong) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((c) as libc::c_longlong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s8ac76eb38ea25f2e(mut __a0: libc::c_longlong) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s8ac76eb38ea25f2e(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Manage copying...
    pub unsafe fn new_at_s64b54c651f3e4c5f(
        __this: *mut Self,
        mut rhs: *const ACE_Atomic_Op_GCC_long_long_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((unsafe {
                    ::core::ptr::read_volatile(::core::ptr::addr_of!((* rhs).value_))
                }) as libc::c_longlong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s64b54c651f3e4c5f(
        mut __a0: *const ACE_Atomic_Op_GCC_long_long_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s64b54c651f3e4c5f(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Atomically assign rhs to @c value_.
    pub unsafe fn operator_assign_s841d310ddcf20e8f(
        __this: *mut Self,
        mut rhs: libc::c_longlong,
    ) -> *mut ACE_Atomic_Op_GCC_long_long_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    (unsafe {
                        &*((::core::ptr::addr_of_mut!((* __this).value_)
                            as *mut libc::c_longlong)
                            as *const ::core::sync::atomic::AtomicI64)
                    })
                        .store(
                            (((rhs) as libc::c_longlong)) as i64,
                            ::core::sync::atomic::Ordering::Release,
                        );
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_ {
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_long_long_>::new_at_sb2af9a14dc15c1b0(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
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
    pub unsafe fn new_at_ud3a33d3ab7354316(__this: *mut Self, mut c: libc::c_ulonglong) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_long_long_>::new_at_se5ec53498d612b13(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
                c,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_ud3a33d3ab7354316(mut __a0: libc::c_ulonglong) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ud3a33d3ab7354316(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_u85cc3b07e52f9b73(
        __this: *mut Self,
        mut c: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_long_long_>::new_at_sb631fd26c277959f(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
                ::core::ptr::addr_of!(((* c)).__base_0)
                    as *const ACE_Atomic_Op_GCC_unsigned_long_long_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u85cc3b07e52f9b73(
        mut __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u85cc3b07e52f9b73(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: libc::c_ulonglong,
    ) -> *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_long_long_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Atomic_Op_GCC_unsigned_long_long_>::operator_assign_s2bdbc7bf6e8d3b54(
                    (::core::ptr::addr_of_mut!((* __this).__base_0))
                        as *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
                    rhs,
                );
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_GCC_unsigned_long_long_ {
    ///Atomically pre-increment @c value_.
    pub unsafe fn operator_inc_s31ae7ea9b386b452(
        __this: *mut Self,
    ) -> libc::c_ulonglong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyEppEv"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
            ) -> libc::c_ulonglong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_)
    }
    ///Atomically post-increment @c value_.
    pub unsafe fn operator_inc_s9b00a5261c2cdb17(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_ulonglong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyEppEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
                _anon_0: libc::c_int,
            ) -> libc::c_ulonglong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_, _anon_0)
    }
    ///Atomically increment @c value_ by rhs.
    pub unsafe fn operator_add_assign(
        __this: *mut Self,
        mut rhs: libc::c_ulonglong,
    ) -> libc::c_ulonglong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyEpLEy"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
                rhs: libc::c_ulonglong,
            ) -> libc::c_ulonglong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_, rhs)
    }
    ///Atomically pre-decrement @c value_.
    pub unsafe fn operator_dec_s31ae7ea9b386b452(
        __this: *mut Self,
    ) -> libc::c_ulonglong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyEmmEv"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
            ) -> libc::c_ulonglong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_)
    }
    ///Atomically post-decrement @c value_.
    pub unsafe fn operator_dec_s9b00a5261c2cdb17(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_ulonglong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyEmmEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
                _anon_0: libc::c_int,
            ) -> libc::c_ulonglong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_, _anon_0)
    }
    ///Atomically decrement @c value_ by rhs.
    pub unsafe fn operator_sub_assign(
        __this: *mut Self,
        mut rhs: libc::c_ulonglong,
    ) -> libc::c_ulonglong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyEmIEy"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
                rhs: libc::c_ulonglong,
            ) -> libc::c_ulonglong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_eq(__this: *const Self, mut rhs: libc::c_ulonglong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIyEeqEy"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
                rhs: libc::c_ulonglong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_long_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_ne(__this: *const Self, mut rhs: libc::c_ulonglong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIyEneEy"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
                rhs: libc::c_ulonglong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_long_, rhs)
    }
    ///Atomically check if @c value_ greater than or equal to rhs.
    pub unsafe fn operator_ge(__this: *const Self, mut rhs: libc::c_ulonglong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIyEgeEy"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
                rhs: libc::c_ulonglong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_long_, rhs)
    }
    ///Atomically check if @c value_ greater than rhs.
    pub unsafe fn operator_gt(__this: *const Self, mut rhs: libc::c_ulonglong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIyEgtEy"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
                rhs: libc::c_ulonglong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_long_, rhs)
    }
    ///Atomically check if @c value_ less than or equal to rhs.
    pub unsafe fn operator_le(__this: *const Self, mut rhs: libc::c_ulonglong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIyEleEy"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
                rhs: libc::c_ulonglong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_long_, rhs)
    }
    ///Atomically check if @c value_ less than rhs.
    pub unsafe fn operator_lt(__this: *const Self, mut rhs: libc::c_ulonglong) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIyEltEy"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
                rhs: libc::c_ulonglong,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_long_, rhs)
    }
    ///Exchange value with @a newval.
    pub unsafe fn exchange(
        __this: *mut Self,
        mut newval: libc::c_ulonglong,
    ) -> libc::c_ulonglong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyE8exchangeEy"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
                newval: libc::c_ulonglong,
            ) -> libc::c_ulonglong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_, newval)
    }
    ///Explicitly return @c value_.
    pub unsafe fn value(__this: *const Self) -> libc::c_ulonglong {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIyE5valueEv"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
            ) -> libc::c_ulonglong;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_long_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIyE4dumpEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_unsigned_long_long_);
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_long_long_)
    }
    ///Explicitly return @c value_ (by reference).
    pub unsafe fn value_i(__this: *mut Self) -> *mut libc::c_ulonglong {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyE7value_iEv"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
            ) -> *mut libc::c_ulonglong;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_)
    }
    ///Atomically assign <rhs> to @c value_.
    pub unsafe fn operator_assign_s06f6cf7c2a5dc49b(
        __this: *mut Self,
        mut rhs: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
    ) -> *mut ACE_Atomic_Op_GCC_unsigned_long_long_ {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyEaSERKS0_"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
                rhs: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
            ) -> *mut ACE_Atomic_Op_GCC_unsigned_long_long_;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_, rhs)
    }
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIyE5mutexEv"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_long_long_,
            ) -> *mut ACE_Thread_Mutex;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_long_long_)
    }
    ///Initialize @c value_ to 0.
    pub unsafe fn new_at_sb2af9a14dc15c1b0(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((0) as libc::c_ulonglong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb2af9a14dc15c1b0() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb2af9a14dc15c1b0(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Initialize @c value_ to c.
    pub unsafe fn new_at_se5ec53498d612b13(__this: *mut Self, mut c: libc::c_ulonglong) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((c) as libc::c_ulonglong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_se5ec53498d612b13(mut __a0: libc::c_ulonglong) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_se5ec53498d612b13(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Manage copying...
    pub unsafe fn new_at_sb631fd26c277959f(
        __this: *mut Self,
        mut rhs: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((unsafe {
                    ::core::ptr::read_volatile(::core::ptr::addr_of!((* rhs).value_))
                }) as libc::c_ulonglong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb631fd26c277959f(
        mut __a0: *const ACE_Atomic_Op_GCC_unsigned_long_long_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb631fd26c277959f(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Atomically assign rhs to @c value_.
    pub unsafe fn operator_assign_s2bdbc7bf6e8d3b54(
        __this: *mut Self,
        mut rhs: libc::c_ulonglong,
    ) -> *mut ACE_Atomic_Op_GCC_unsigned_long_long_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    (unsafe {
                        &*((::core::ptr::addr_of_mut!((* __this).value_)
                            as *mut libc::c_ulonglong)
                            as *const ::core::sync::atomic::AtomicU64)
                    })
                        .store(
                            (((rhs) as libc::c_ulonglong)) as u64,
                            ::core::sync::atomic::Ordering::Release,
                        );
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_ACE_Thread_Mutex__short_ {
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_short_>::new_at_sb2afb214dc15ea78(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_short_,
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
    pub unsafe fn new_at_u8af4ddea024eb19e(__this: *mut Self, mut c: libc::c_short) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_short_>::new_at_s946653175f1c528b(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_short_,
                c,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u8af4ddea024eb19e(mut __a0: libc::c_short) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u8af4ddea024eb19e(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_uef9241b52e0b8deb(
        __this: *mut Self,
        mut c: *const ACE_Atomic_Op_ACE_Thread_Mutex__short_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_short_>::new_at_s42f6630c2645225f(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_short_,
                ::core::ptr::addr_of!(((* c)).__base_0)
                    as *const ACE_Atomic_Op_GCC_short_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_uef9241b52e0b8deb(
        mut __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__short_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_uef9241b52e0b8deb(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: libc::c_short,
    ) -> *mut ACE_Atomic_Op_ACE_Thread_Mutex__short_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Atomic_Op_GCC_short_>::operator_assign_sf3742ebfc8f13684(
                    (::core::ptr::addr_of_mut!((* __this).__base_0))
                        as *mut ACE_Atomic_Op_GCC_short_,
                    rhs,
                );
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_GCC_short_ {
    ///Atomically pre-increment @c value_.
    pub unsafe fn operator_inc_sdd127c87d39931c2(__this: *mut Self) -> libc::c_short {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsEppEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_short_) -> libc::c_short;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_)
    }
    ///Atomically post-increment @c value_.
    pub unsafe fn operator_inc_sd4bda2ddce0ef237(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_short {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsEppEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_short_,
                _anon_0: libc::c_int,
            ) -> libc::c_short;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_, _anon_0)
    }
    ///Atomically increment @c value_ by rhs.
    pub unsafe fn operator_add_assign(
        __this: *mut Self,
        mut rhs: libc::c_short,
    ) -> libc::c_short {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsEpLEs"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_short_,
                rhs: libc::c_short,
            ) -> libc::c_short;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_, rhs)
    }
    ///Atomically pre-decrement @c value_.
    pub unsafe fn operator_dec_sdd127c87d39931c2(__this: *mut Self) -> libc::c_short {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsEmmEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_short_) -> libc::c_short;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_)
    }
    ///Atomically post-decrement @c value_.
    pub unsafe fn operator_dec_sd4bda2ddce0ef237(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_short {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsEmmEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_short_,
                _anon_0: libc::c_int,
            ) -> libc::c_short;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_, _anon_0)
    }
    ///Atomically decrement @c value_ by rhs.
    pub unsafe fn operator_sub_assign(
        __this: *mut Self,
        mut rhs: libc::c_short,
    ) -> libc::c_short {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsEmIEs"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_short_,
                rhs: libc::c_short,
            ) -> libc::c_short;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_eq(__this: *const Self, mut rhs: libc::c_short) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIsEeqEs"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_short_,
                rhs: libc::c_short,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_short_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_ne(__this: *const Self, mut rhs: libc::c_short) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIsEneEs"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_short_,
                rhs: libc::c_short,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_short_, rhs)
    }
    ///Atomically check if @c value_ greater than or equal to rhs.
    pub unsafe fn operator_ge(__this: *const Self, mut rhs: libc::c_short) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIsEgeEs"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_short_,
                rhs: libc::c_short,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_short_, rhs)
    }
    ///Atomically check if @c value_ greater than rhs.
    pub unsafe fn operator_gt(__this: *const Self, mut rhs: libc::c_short) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIsEgtEs"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_short_,
                rhs: libc::c_short,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_short_, rhs)
    }
    ///Atomically check if @c value_ less than or equal to rhs.
    pub unsafe fn operator_le(__this: *const Self, mut rhs: libc::c_short) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIsEleEs"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_short_,
                rhs: libc::c_short,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_short_, rhs)
    }
    ///Atomically check if @c value_ less than rhs.
    pub unsafe fn operator_lt(__this: *const Self, mut rhs: libc::c_short) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIsEltEs"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_short_,
                rhs: libc::c_short,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_short_, rhs)
    }
    ///Exchange value with @a newval.
    pub unsafe fn exchange(
        __this: *mut Self,
        mut newval: libc::c_short,
    ) -> libc::c_short {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsE8exchangeEs"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_short_,
                newval: libc::c_short,
            ) -> libc::c_short;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_, newval)
    }
    ///Explicitly return @c value_.
    pub unsafe fn value(__this: *const Self) -> libc::c_short {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIsE5valueEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_short_) -> libc::c_short;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_short_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIsE4dumpEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_short_);
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_short_)
    }
    ///Explicitly return @c value_ (by reference).
    pub unsafe fn value_i(__this: *mut Self) -> *mut libc::c_short {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsE7value_iEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_short_) -> *mut libc::c_short;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_)
    }
    ///Atomically assign <rhs> to @c value_.
    pub unsafe fn operator_assign_s7ccb8ed3b44a20e3(
        __this: *mut Self,
        mut rhs: *mut ACE_Atomic_Op_GCC_short_,
    ) -> *mut ACE_Atomic_Op_GCC_short_ {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsEaSERKS0_"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_short_,
                rhs: *mut ACE_Atomic_Op_GCC_short_,
            ) -> *mut ACE_Atomic_Op_GCC_short_;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_, rhs)
    }
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIsE5mutexEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_short_) -> *mut ACE_Thread_Mutex;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_short_)
    }
    ///Initialize @c value_ to 0.
    pub unsafe fn new_at_sb2afb214dc15ea78(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((0) as libc::c_short),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb2afb214dc15ea78() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb2afb214dc15ea78(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Initialize @c value_ to c.
    pub unsafe fn new_at_s946653175f1c528b(__this: *mut Self, mut c: libc::c_short) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((c) as libc::c_short),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s946653175f1c528b(mut __a0: libc::c_short) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s946653175f1c528b(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Manage copying...
    pub unsafe fn new_at_s42f6630c2645225f(
        __this: *mut Self,
        mut rhs: *const ACE_Atomic_Op_GCC_short_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((unsafe {
                    ::core::ptr::read_volatile(::core::ptr::addr_of!((* rhs).value_))
                }) as libc::c_short),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s42f6630c2645225f(
        mut __a0: *const ACE_Atomic_Op_GCC_short_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s42f6630c2645225f(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Atomically assign rhs to @c value_.
    pub unsafe fn operator_assign_sf3742ebfc8f13684(
        __this: *mut Self,
        mut rhs: libc::c_short,
    ) -> *mut ACE_Atomic_Op_GCC_short_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    (unsafe {
                        &*((::core::ptr::addr_of_mut!((* __this).value_)
                            as *mut libc::c_short)
                            as *const ::core::sync::atomic::AtomicI16)
                    })
                        .store(
                            (((rhs) as libc::c_short)) as i16,
                            ::core::sync::atomic::Ordering::Release,
                        );
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_ {
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_short_>::new_at_sb2af9214dc15b418(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_short_,
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
    pub unsafe fn new_at_u11010cccb631acfe(__this: *mut Self, mut c: libc::c_ushort) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_short_>::new_at_s878cc61f3e019dca(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_short_,
                c,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u11010cccb631acfe(mut __a0: libc::c_ushort) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u11010cccb631acfe(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_u137675b7e074ff4b(
        __this: *mut Self,
        mut c: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_unsigned_short_>::new_at_s71cb7029d33cb79f(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_unsigned_short_,
                ::core::ptr::addr_of!(((* c)).__base_0)
                    as *const ACE_Atomic_Op_GCC_unsigned_short_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u137675b7e074ff4b(
        mut __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u137675b7e074ff4b(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: libc::c_ushort,
    ) -> *mut ACE_Atomic_Op_ACE_Thread_Mutex__unsigned_short_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Atomic_Op_GCC_unsigned_short_>::operator_assign_s69c5926379e651fb(
                    (::core::ptr::addr_of_mut!((* __this).__base_0))
                        as *mut ACE_Atomic_Op_GCC_unsigned_short_,
                    rhs,
                );
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_GCC_unsigned_short_ {
    ///Atomically pre-increment @c value_.
    pub unsafe fn operator_inc_s2c263b2e3114be97(__this: *mut Self) -> libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItEppEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_unsigned_short_) -> libc::c_ushort;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_)
    }
    ///Atomically post-increment @c value_.
    pub unsafe fn operator_inc_sc2b3a31e08ab6fc6(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItEppEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
                _anon_0: libc::c_int,
            ) -> libc::c_ushort;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_, _anon_0)
    }
    ///Atomically increment @c value_ by rhs.
    pub unsafe fn operator_add_assign(
        __this: *mut Self,
        mut rhs: libc::c_ushort,
    ) -> libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItEpLEt"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
                rhs: libc::c_ushort,
            ) -> libc::c_ushort;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_, rhs)
    }
    ///Atomically pre-decrement @c value_.
    pub unsafe fn operator_dec_s2c263b2e3114be97(__this: *mut Self) -> libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItEmmEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_unsigned_short_) -> libc::c_ushort;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_)
    }
    ///Atomically post-decrement @c value_.
    pub unsafe fn operator_dec_sc2b3a31e08ab6fc6(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItEmmEi"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
                _anon_0: libc::c_int,
            ) -> libc::c_ushort;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_, _anon_0)
    }
    ///Atomically decrement @c value_ by rhs.
    pub unsafe fn operator_sub_assign(
        __this: *mut Self,
        mut rhs: libc::c_ushort,
    ) -> libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItEmIEt"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
                rhs: libc::c_ushort,
            ) -> libc::c_ushort;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_eq(__this: *const Self, mut rhs: libc::c_ushort) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCItEeqEt"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_short_,
                rhs: libc::c_ushort,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_short_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_ne(__this: *const Self, mut rhs: libc::c_ushort) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCItEneEt"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_short_,
                rhs: libc::c_ushort,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_short_, rhs)
    }
    ///Atomically check if @c value_ greater than or equal to rhs.
    pub unsafe fn operator_ge(__this: *const Self, mut rhs: libc::c_ushort) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCItEgeEt"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_short_,
                rhs: libc::c_ushort,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_short_, rhs)
    }
    ///Atomically check if @c value_ greater than rhs.
    pub unsafe fn operator_gt(__this: *const Self, mut rhs: libc::c_ushort) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCItEgtEt"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_short_,
                rhs: libc::c_ushort,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_short_, rhs)
    }
    ///Atomically check if @c value_ less than or equal to rhs.
    pub unsafe fn operator_le(__this: *const Self, mut rhs: libc::c_ushort) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCItEleEt"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_short_,
                rhs: libc::c_ushort,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_short_, rhs)
    }
    ///Atomically check if @c value_ less than rhs.
    pub unsafe fn operator_lt(__this: *const Self, mut rhs: libc::c_ushort) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCItEltEt"]
            fn __ext(
                __this: *const ACE_Atomic_Op_GCC_unsigned_short_,
                rhs: libc::c_ushort,
            ) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_short_, rhs)
    }
    ///Exchange value with @a newval.
    pub unsafe fn exchange(
        __this: *mut Self,
        mut newval: libc::c_ushort,
    ) -> libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItE8exchangeEt"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
                newval: libc::c_ushort,
            ) -> libc::c_ushort;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_, newval)
    }
    ///Explicitly return @c value_.
    pub unsafe fn value(__this: *const Self) -> libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCItE5valueEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_unsigned_short_) -> libc::c_ushort;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_short_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCItE4dumpEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_unsigned_short_);
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_unsigned_short_)
    }
    ///Explicitly return @c value_ (by reference).
    pub unsafe fn value_i(__this: *mut Self) -> *mut libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItE7value_iEv"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
            ) -> *mut libc::c_ushort;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_)
    }
    ///Atomically assign <rhs> to @c value_.
    pub unsafe fn operator_assign_sbb72890663718103(
        __this: *mut Self,
        mut rhs: *mut ACE_Atomic_Op_GCC_unsigned_short_,
    ) -> *mut ACE_Atomic_Op_GCC_unsigned_short_ {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItEaSERKS0_"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
                rhs: *mut ACE_Atomic_Op_GCC_unsigned_short_,
            ) -> *mut ACE_Atomic_Op_GCC_unsigned_short_;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_, rhs)
    }
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCItE5mutexEv"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_unsigned_short_,
            ) -> *mut ACE_Thread_Mutex;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_unsigned_short_)
    }
    ///Initialize @c value_ to 0.
    pub unsafe fn new_at_sb2af9214dc15b418(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((0) as libc::c_ushort),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sb2af9214dc15b418() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb2af9214dc15b418(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Initialize @c value_ to c.
    pub unsafe fn new_at_s878cc61f3e019dca(__this: *mut Self, mut c: libc::c_ushort) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((c) as libc::c_ushort),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s878cc61f3e019dca(mut __a0: libc::c_ushort) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s878cc61f3e019dca(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Manage copying...
    pub unsafe fn new_at_s71cb7029d33cb79f(
        __this: *mut Self,
        mut rhs: *const ACE_Atomic_Op_GCC_unsigned_short_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((unsafe {
                    ::core::ptr::read_volatile(::core::ptr::addr_of!((* rhs).value_))
                }) as libc::c_ushort),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s71cb7029d33cb79f(
        mut __a0: *const ACE_Atomic_Op_GCC_unsigned_short_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s71cb7029d33cb79f(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Atomically assign rhs to @c value_.
    pub unsafe fn operator_assign_s69c5926379e651fb(
        __this: *mut Self,
        mut rhs: libc::c_ushort,
    ) -> *mut ACE_Atomic_Op_GCC_unsigned_short_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    (unsafe {
                        &*((::core::ptr::addr_of_mut!((* __this).value_)
                            as *mut libc::c_ushort)
                            as *const ::core::sync::atomic::AtomicU16)
                    })
                        .store(
                            (((rhs) as libc::c_ushort)) as u16,
                            ::core::sync::atomic::Ordering::Release,
                        );
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_ACE_Thread_Mutex__bool_ {
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_bool_>::new_at_sb2afa314dc15d0fb(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_bool_,
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
    pub unsafe fn new_at_u9396a416a0964f25(__this: *mut Self, mut c: bool) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_bool_>::new_at_sc5ccefaa3980372a(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_bool_,
                c,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u9396a416a0964f25(mut __a0: bool) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u9396a416a0964f25(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_ua42208eb4017b900(
        __this: *mut Self,
        mut c: *const ACE_Atomic_Op_ACE_Thread_Mutex__bool_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Atomic_Op_GCC_bool_>::new_at_sa2856b7b2f8711f3(
                (::core::ptr::addr_of_mut!((* __this).__base_0))
                    as *mut ACE_Atomic_Op_GCC_bool_,
                ::core::ptr::addr_of!(((* c)).__base_0) as *const ACE_Atomic_Op_GCC_bool_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_ua42208eb4017b900(
        mut __a0: *const ACE_Atomic_Op_ACE_Thread_Mutex__bool_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ua42208eb4017b900(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: bool,
    ) -> *mut ACE_Atomic_Op_ACE_Thread_Mutex__bool_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Atomic_Op_GCC_bool_>::operator_assign_s29041e5426cb27f8(
                    (::core::ptr::addr_of_mut!((* __this).__base_0))
                        as *mut ACE_Atomic_Op_GCC_bool_,
                    rhs,
                );
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Atomic_Op_GCC_bool_ {
    ///Atomically pre-increment @c value_.
    pub unsafe fn operator_inc_sc5d0d1aa3983ed07(__this: *mut Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbEppEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_bool_) -> bool;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_)
    }
    ///Atomically post-increment @c value_.
    pub unsafe fn operator_inc_sa32b53d4231e4e1e(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbEppEi"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_bool_, _anon_0: libc::c_int) -> bool;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_, _anon_0)
    }
    ///Atomically increment @c value_ by rhs.
    pub unsafe fn operator_add_assign(__this: *mut Self, mut rhs: bool) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbEpLEb"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_bool_, rhs: bool) -> bool;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_, rhs)
    }
    ///Atomically pre-decrement @c value_.
    pub unsafe fn operator_dec_sc5d0d1aa3983ed07(__this: *mut Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbEmmEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_bool_) -> bool;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_)
    }
    ///Atomically post-decrement @c value_.
    pub unsafe fn operator_dec_sa32b53d4231e4e1e(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbEmmEi"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_bool_, _anon_0: libc::c_int) -> bool;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_, _anon_0)
    }
    ///Atomically decrement @c value_ by rhs.
    pub unsafe fn operator_sub_assign(__this: *mut Self, mut rhs: bool) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbEmIEb"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_bool_, rhs: bool) -> bool;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_eq(__this: *const Self, mut rhs: bool) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIbEeqEb"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_bool_, rhs: bool) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_bool_, rhs)
    }
    ///Atomically compare @c value_ with rhs.
    pub unsafe fn operator_ne(__this: *const Self, mut rhs: bool) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIbEneEb"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_bool_, rhs: bool) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_bool_, rhs)
    }
    ///Atomically check if @c value_ greater than or equal to rhs.
    pub unsafe fn operator_ge(__this: *const Self, mut rhs: bool) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIbEgeEb"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_bool_, rhs: bool) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_bool_, rhs)
    }
    ///Atomically check if @c value_ greater than rhs.
    pub unsafe fn operator_gt(__this: *const Self, mut rhs: bool) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIbEgtEb"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_bool_, rhs: bool) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_bool_, rhs)
    }
    ///Atomically check if @c value_ less than or equal to rhs.
    pub unsafe fn operator_le(__this: *const Self, mut rhs: bool) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIbEleEb"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_bool_, rhs: bool) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_bool_, rhs)
    }
    ///Atomically check if @c value_ less than rhs.
    pub unsafe fn operator_lt(__this: *const Self, mut rhs: bool) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIbEltEb"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_bool_, rhs: bool) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_bool_, rhs)
    }
    ///Exchange value with @a newval.
    pub unsafe fn exchange(__this: *mut Self, mut newval: bool) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbE8exchangeEb"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_bool_, newval: bool) -> bool;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_, newval)
    }
    ///Explicitly return @c value_.
    pub unsafe fn value(__this: *const Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIbE5valueEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_bool_) -> bool;
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_bool_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Atomic_Op_GCCIbE4dumpEv"]
            fn __ext(__this: *const ACE_Atomic_Op_GCC_bool_);
        }
        __ext(__this as *const ACE_Atomic_Op_GCC_bool_)
    }
    ///Explicitly return @c value_ (by reference).
    pub unsafe fn value_i(__this: *mut Self) -> *mut bool {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbE7value_iEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_bool_) -> *mut bool;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_)
    }
    ///Atomically assign <rhs> to @c value_.
    pub unsafe fn operator_assign_s2e92a7773f0b5016(
        __this: *mut Self,
        mut rhs: *mut ACE_Atomic_Op_GCC_bool_,
    ) -> *mut ACE_Atomic_Op_GCC_bool_ {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbEaSERKS0_"]
            fn __ext(
                __this: *mut ACE_Atomic_Op_GCC_bool_,
                rhs: *mut ACE_Atomic_Op_GCC_bool_,
            ) -> *mut ACE_Atomic_Op_GCC_bool_;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_, rhs)
    }
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Atomic_Op_GCCIbE5mutexEv"]
            fn __ext(__this: *mut ACE_Atomic_Op_GCC_bool_) -> *mut ACE_Thread_Mutex;
        }
        __ext(__this as *mut ACE_Atomic_Op_GCC_bool_)
    }
    ///Initialize @c value_ to 0.
    pub unsafe fn new_at_sb2afa314dc15d0fb(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).value_), ((0) != 0));
            {}
            ()
        }
    }
    pub unsafe fn new_sb2afa314dc15d0fb() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb2afa314dc15d0fb(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Initialize @c value_ to c.
    pub unsafe fn new_at_sc5ccefaa3980372a(__this: *mut Self, mut c: bool) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((c) as bool),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sc5ccefaa3980372a(mut __a0: bool) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sc5ccefaa3980372a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Manage copying...
    pub unsafe fn new_at_sa2856b7b2f8711f3(
        __this: *mut Self,
        mut rhs: *const ACE_Atomic_Op_GCC_bool_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).value_),
                ((unsafe {
                    ::core::ptr::read_volatile(::core::ptr::addr_of!((* rhs).value_))
                }) as bool),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sa2856b7b2f8711f3(
        mut __a0: *const ACE_Atomic_Op_GCC_bool_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sa2856b7b2f8711f3(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Atomically assign rhs to @c value_.
    pub unsafe fn operator_assign_s29041e5426cb27f8(
        __this: *mut Self,
        mut rhs: bool,
    ) -> *mut ACE_Atomic_Op_GCC_bool_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    (unsafe {
                        &*((::core::ptr::addr_of_mut!((* __this).value_) as *mut bool)
                            as *const ::core::sync::atomic::AtomicBool)
                    })
                        .store(((rhs) as bool), ::core::sync::atomic::Ordering::Release);
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Reactor {
    ///Get pointer to a process-wide ACE_Reactor.
    pub unsafe fn instance() -> *mut ACE_Reactor {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor8instanceEv"]
            fn __ext() -> *mut ACE_Reactor;
        }
        __ext()
    }
    #[doc = "* Set pointer to a process-wide ACE_Reactor and return existing\n   * pointer.  If @a delete_reactor == true then we'll delete the Reactor\n   * at destruction time."]
    pub unsafe fn instance_u03e55b1200a0ddf6(
        mut _anon_0: *mut ACE_Reactor,
        mut delete_reactor: bool,
    ) -> *mut ACE_Reactor {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor8instanceEPS_b"]
            fn __ext(
                _anon_0: *mut ACE_Reactor,
                delete_reactor: bool,
            ) -> *mut ACE_Reactor;
        }
        __ext(_anon_0, delete_reactor)
    }
    ///Delete the dynamically allocated Singleton
    pub unsafe fn close_singleton() {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor15close_singletonEv"]
            fn __ext();
        }
        __ext()
    }
    ///Name of the dll in which the dll lives.
    pub unsafe fn dll_name() -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor8dll_nameEv"]
            fn __ext() -> *const libc::c_char;
        }
        __ext()
    }
    ///Name of the component--ACE_Reactor in this case.
    pub unsafe fn name() -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor4nameEv"]
            fn __ext() -> *const libc::c_char;
        }
        __ext()
    }
    #[doc = "* Run the event loop until the\n   * ACE_Reactor::handle_events()/ACE_Reactor::alertable_handle_events()\n   * method returns -1 or the end_event_loop() method is invoked.\n   * Note that this method can only be used by the singleton\n   * ACE_Reactor::instance().  Thus, to run another reactor use\n   * ACE_Reactor::run_reactor_event_loop().\n   *\n   * @deprecated Use ACE_Reactor::instance()->run_reactor_event_loop() instead"]
    pub unsafe fn run_event_loop() -> libc::c_int {
        unsafe {
            {
                let mut r: *mut ACE_Reactor = <ACE_Reactor>::instance();
                if (((((r).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return (-((1) as libc::c_int));
                }
                return <ACE_Reactor>::run_reactor_event_loop(
                    (r) as *mut ACE_Reactor,
                    Some({
                        extern "C-unwind" {
                            #[link_name = "_ZN11ACE_Reactor21check_reconfigurationEPS_"]
                            fn __fp(a0: *mut ACE_Reactor) -> libc::c_int;
                        }
                        __fp
                            as unsafe extern "C-unwind" fn(
                                *mut ACE_Reactor,
                            ) -> libc::c_int
                    }),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn run_alertable_event_loop() -> libc::c_int {
        unsafe {
            {
                let mut r: *mut ACE_Reactor = <ACE_Reactor>::instance();
                if (((((r).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return (-((1) as libc::c_int));
                }
                return <ACE_Reactor>::run_alertable_reactor_event_loop(
                    (r) as *mut ACE_Reactor,
                    Some({
                        extern "C-unwind" {
                            #[link_name = "_ZN11ACE_Reactor21check_reconfigurationEPS_"]
                            fn __fp(a0: *mut ACE_Reactor) -> libc::c_int;
                        }
                        __fp
                            as unsafe extern "C-unwind" fn(
                                *mut ACE_Reactor,
                            ) -> libc::c_int
                    }),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Run the event loop until the ACE_Reactor::handle_events() or\n   * <ACE_Reactor::alertable_handle_events> methods returns -1, the\n   * end_event_loop() method is invoked, or the ACE_Time_Value\n   * expires.  Note that this method can only be used by the singleton\n   * ACE_Reactor::instance().  Thus, to run another reactor use\n   * <ACE_Reactor::run_reactor_event_loop>.\n   *\n   * @deprecated Use ACE_Reactor::instance()->run_reactor_event_loop() instead"]
    pub unsafe fn run_event_loop_u41ef1c2de7a49a0a(
        mut tv: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            {
                let mut r: *mut ACE_Reactor = <ACE_Reactor>::instance();
                if (((((r).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return (-((1) as libc::c_int));
                }
                return <ACE_Reactor>::run_reactor_event_loop_u983bed3e9ebc1d83(
                    (r) as *mut ACE_Reactor,
                    ::core::ptr::addr_of_mut!((* tv)),
                    Some({
                        extern "C-unwind" {
                            #[link_name = "_ZN11ACE_Reactor21check_reconfigurationEPS_"]
                            fn __fp(a0: *mut ACE_Reactor) -> libc::c_int;
                        }
                        __fp
                            as unsafe extern "C-unwind" fn(
                                *mut ACE_Reactor,
                            ) -> libc::c_int
                    }),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn run_alertable_event_loop_ub0e1ed87729365a9(
        mut tv: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            {
                let mut r: *mut ACE_Reactor = <ACE_Reactor>::instance();
                if (((((r).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return (-((1) as libc::c_int));
                }
                return <ACE_Reactor>::run_alertable_reactor_event_loop_u8f7b349329cd6000(
                    (r) as *mut ACE_Reactor,
                    ::core::ptr::addr_of_mut!((* tv)),
                    Some({
                        extern "C-unwind" {
                            #[link_name = "_ZN11ACE_Reactor21check_reconfigurationEPS_"]
                            fn __fp(a0: *mut ACE_Reactor) -> libc::c_int;
                        }
                        __fp
                            as unsafe extern "C-unwind" fn(
                                *mut ACE_Reactor,
                            ) -> libc::c_int
                    }),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Instruct the ACE_Reactor::instance() to terminate its event loop\n   * and notifies the ACE_Reactor::instance() so that it can wake up\n   * and close down gracefully.  Note that this method can only be\n   * used by the singleton ACE_Reactor::instance().  Thus, to\n   * terminate another reactor, use\n   * <ACE_Reactor::end_reactor_event_loop>.\n   *\n   * @deprecated Use ACE_Reactor::instance()->end_reactor_event_loop() instead"]
    pub unsafe fn end_event_loop() -> libc::c_int {
        unsafe {
            {
                <ACE_Reactor>::end_reactor_event_loop(
                    (<ACE_Reactor>::instance()) as *mut ACE_Reactor,
                );
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Report if the ACE_Reactor::instance()'s event loop is finished.\n   * Note that this method can only be used by the singleton\n   * ACE_Reactor::instance().  Thus, to check another reactor use\n   * <ACE_Reactor::reactor_event_loop_done>.\n   *\n   * @deprecated Use ACE_Reactor::instance()->reactor_event_loop_done() instead"]
    pub unsafe fn event_loop_done() -> libc::c_int {
        unsafe {
            {
                return <ACE_Reactor>::reactor_event_loop_done(
                    (<ACE_Reactor>::instance()) as *mut ACE_Reactor,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Resets the ACE_Reactor::end_event_loop_  static so that the\n   * run_event_loop() method can be restarted.  Note that this method\n   * can only be used by the singleton ACE_Reactor::instance().  Thus,\n   * to reset another reactor use ACE_Reactor::reset_reactor_event_loop().\n   *\n   * @deprecated Use ACE_Reactor::instance()->reset_reactor_event_loop()\n   * instead"]
    pub unsafe fn reset_event_loop() {
        unsafe {
            {
                <ACE_Reactor>::reset_reactor_event_loop(
                    (<ACE_Reactor>::instance()) as *mut ACE_Reactor,
                );
            }
            ()
        }
    }
    #[doc = "* The singleton reactor is used by the ACE_Service_Config.\n   * Therefore, we must check for the reconfiguration request and\n   * handle it after handling an event."]
    pub unsafe fn check_reconfiguration(mut _anon_0: *mut ACE_Reactor) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor21check_reconfigurationEPS_"]
            fn __ext(_anon_0: *mut ACE_Reactor) -> libc::c_int;
        }
        __ext(_anon_0)
    }
    #[doc = "* Run the event loop until the ACE_Reactor::handle_events() or\n   * ACE_Reactor::alertable_handle_events() method returns -1 or\n   * the end_reactor_event_loop() method is invoked."]
    pub unsafe fn run_reactor_event_loop(
        __this: *mut Self,
        mut _anon_0: Option<unsafe extern "C-unwind" fn(*mut ACE_Reactor) -> libc::c_int>,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor22run_reactor_event_loopEPFiPS_E"]
            fn __ext(
                __this: *mut ACE_Reactor,
                _anon_0: Option<
                    unsafe extern "C-unwind" fn(*mut ACE_Reactor) -> libc::c_int,
                >,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, _anon_0)
    }
    pub unsafe fn run_alertable_reactor_event_loop(
        __this: *mut Self,
        mut _anon_0: Option<unsafe extern "C-unwind" fn(*mut ACE_Reactor) -> libc::c_int>,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor32run_alertable_reactor_event_loopEPFiPS_E"]
            fn __ext(
                __this: *mut ACE_Reactor,
                _anon_0: Option<
                    unsafe extern "C-unwind" fn(*mut ACE_Reactor) -> libc::c_int,
                >,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, _anon_0)
    }
    #[doc = "* Run the event loop until the ACE_Reactor::handle_events() or\n   * ACE_Reactor::alertable_handle_events() method returns -1, the\n   * end_reactor_event_loop() method is invoked, or the ACE_Time_Value\n   * expires while the underlying event demultiplexer is waiting for\n   * events.\n   * Note that it is possible for events to continuously be available,\n   * avoiding the need to wait for events. In this situation the timeout\n   * value will not have an opportunity to expire until the next time\n   * the underlying event demultiplexer waits for events."]
    pub unsafe fn run_reactor_event_loop_u983bed3e9ebc1d83(
        __this: *mut Self,
        mut tv: *mut ACE_Time_Value,
        mut _anon_1: Option<unsafe extern "C-unwind" fn(*mut ACE_Reactor) -> libc::c_int>,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor22run_reactor_event_loopER14ACE_Time_ValuePFiPS_E"]
            fn __ext(
                __this: *mut ACE_Reactor,
                tv: *mut ACE_Time_Value,
                _anon_1: Option<
                    unsafe extern "C-unwind" fn(*mut ACE_Reactor) -> libc::c_int,
                >,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, tv, _anon_1)
    }
    pub unsafe fn run_alertable_reactor_event_loop_u8f7b349329cd6000(
        __this: *mut Self,
        mut tv: *mut ACE_Time_Value,
        mut _anon_1: Option<unsafe extern "C-unwind" fn(*mut ACE_Reactor) -> libc::c_int>,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor32run_alertable_reactor_event_loopER14ACE_Time_ValuePFiPS_E"]
            fn __ext(
                __this: *mut ACE_Reactor,
                tv: *mut ACE_Time_Value,
                _anon_1: Option<
                    unsafe extern "C-unwind" fn(*mut ACE_Reactor) -> libc::c_int,
                >,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, tv, _anon_1)
    }
    #[doc = "* Instruct the Reactor to terminate its event loop and notifies the\n   * Reactor so that it can wake up and deactivate\n   * itself. Deactivating the Reactor would allow the Reactor to be\n   * shutdown gracefully. Internally the Reactor calls deactivate ()\n   * on the underlying implementation.\n   * Any queued notifications remain queued on return from this method.\n   * If the event loop is restarted in the future, the notifications\n   * will be dispatched then. If the reactor is closed or deleted without\n   * further dispatching, the notifications will be lost."]
    pub unsafe fn end_reactor_event_loop(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __obj: *mut ACE_Reactor_Impl = ((*__this).implementation_)
                        as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u23fc623b1c3b8165)(__obj, 1)
                };
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Indicate if the Reactor's event loop has been ended.
    pub unsafe fn reactor_event_loop_done(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = ((*__this).implementation_)
                        as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u436e43ca94274a59)(__obj)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Resets the ACE_Reactor::end_event_loop_ static so that the
  /// run_event_loop() method can be restarted.*/
    pub unsafe fn reset_reactor_event_loop(__this: *mut Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __obj: *mut ACE_Reactor_Impl = ((*__this).implementation_)
                        as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u23fc623b1c3b8165)(__obj, 0)
                };
            }
            ()
        }
    }
    #[doc = "* Create the Reactor using @a implementation.  The flag\n   * @a delete_implementation tells the Reactor whether or not to\n   * delete the @a implementation on destruction."]
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *mut ACE_Reactor_Impl,
        mut __a1: bool,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_ReactorC1EP16ACE_Reactor_Implb"]
            fn __ext(__this: *mut ACE_Reactor, __a0: *mut ACE_Reactor_Impl, __a1: bool);
        }
        __ext(__this as *mut ACE_Reactor, __a0, __a1)
    }
    pub unsafe fn new(mut __a0: *mut ACE_Reactor_Impl, mut __a1: bool) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    #[doc = "* Initialize the ACE_Reactor to manage @a max_number_of_handles.\n   * If @a restart is false then the ACE_Reactor's handle_events()\n   * method will be restarted automatically when @c EINTR occurs.  If\n   * @a signal_handler or @a timer_queue are non-0 they are used as the\n   * signal handler and timer queue, respectively."]
    pub unsafe fn open(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut restart: bool,
        mut signal_handler: *mut ACE_Sig_Handler,
        mut timer_queue: *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_ubcec569a70adf5c2)(
                        __obj,
                        size,
                        restart,
                        signal_handler,
                        timer_queue,
                        0,
                        ((0) as *mut ACE_Reactor_Notify),
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Use a user specified signal handler instead.
    pub unsafe fn set_sig_handler(
        __this: *mut Self,
        mut signal_handler: *mut ACE_Sig_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u34cd077df5b3f6ae)(__obj, signal_handler)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set a user-specified timer queue.
    pub unsafe fn timer_queue(
        __this: *mut Self,
        mut tq: *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u1259ba200bddddef)(__obj, tq)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the current ACE_Timer_Queue.
    pub unsafe fn timer_queue_u706b88adc7f2fc29(
        __this: *const Self,
    ) -> *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_uac657357bdffab37)(__obj)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Close down and release all resources.
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u11c88cedb3abbab3)(__obj)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Returns non-zero if there are I/O events \"ready\" for dispatching,\n   * but does not actually dispatch the event handlers.  By default,\n   * don't block while checking this, i.e., \"poll\"."]
    pub unsafe fn work_pending(
        __this: *mut Self,
        mut max_wait_time: *const ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_ucda8bd73b913aae2)(
                        __obj,
                        ::core::ptr::addr_of!((* max_wait_time)),
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* This event loop driver blocks for up to @a max_wait_time before\n   * returning.  It will return earlier if events occur.  Note that\n   * @a max_wait_time can be 0, in which case this method blocks\n   * indefinitely until events occur.\n   *\n   * @a max_wait_time is decremented to reflect how much time this call\n   * took.  For instance, if a time value of 3 seconds is passed to\n   * handle_events and an event occurs after 2 seconds,\n   * @a max_wait_time will equal 1 second.  This can be used if an\n   * application wishes to handle events for some fixed amount of\n   * time.\n   *\n   * Returns the total number of timers and I/O ACE_Event_Handlers\n   * that were dispatched, 0 if the @a max_wait_time elapsed without\n   * dispatching any handlers, or -1 if an error occurs.\n   *\n   * The only difference between alertable_handle_events() and\n   * handle_events() is that in the alertable case, the eventloop will\n   * return when the system queues an I/O completion routine or an\n   * Asynchronous Procedure Call."]
    pub unsafe fn handle_events(
        __this: *mut Self,
        mut max_wait_time: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_ueb3555a06630494a)(__obj, max_wait_time)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn alertable_handle_events(
        __this: *mut Self,
        mut max_wait_time: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_uf86867003a3dcb25)(__obj, max_wait_time)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* This method is just like the one above, except the\n   * @a max_wait_time value is a reference and can therefore never be\n   * NULL.\n   *\n   * The only difference between alertable_handle_events() and\n   * handle_events() is that in the alertable case, the eventloop will\n   * return when the system queues an I/O completion routine or an\n   * Asynchronous Procedure Call."]
    pub unsafe fn handle_events_u9d0d65d6eef329f4(
        __this: *mut Self,
        mut max_wait_time: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_udb4a5c2c3ef2df76)(
                        __obj,
                        ::core::ptr::addr_of_mut!((* max_wait_time)),
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn alertable_handle_events_uec633d7701100443(
        __this: *mut Self,
        mut max_wait_time: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_uc9758e13f2d4e5e1)(
                        __obj,
                        ::core::ptr::addr_of_mut!((* max_wait_time)),
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Register handler for I/O events.\n   *\n   * A handler can be associated with multiple handles. A handle\n   * cannot be associated with multiple handlers.\n   *\n   * The handle will come from ACE_Event_Handler::get_handle().\n   *\n   * Reactor will call ACE_Event_Handler::add_reference() for a new\n   * handler/handle pair.\n   *\n   * If this handler/handle pair has already been registered, any new\n   * masks specified will be added. In this case,\n   * ACE_Event_Handler::add_reference() will not be called.\n   *\n   * If the registered handler is currently suspended, it will remain\n   * suspended.  When the handler is resumed, it will have the\n   * existing masks plus any masks added through this call. Handlers\n   * do not have partial suspensions."]
    pub unsafe fn register_handler(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
        mut mask: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor16register_handlerEP17ACE_Event_Handlerm"]
            fn __ext(
                __this: *mut ACE_Reactor,
                event_handler: *mut ACE_Event_Handler,
                mask: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, event_handler, mask)
    }
    #[doc = "* Register handler for I/O events.\n   *\n   * Same as register_handler(ACE_Event_Handler*,ACE_Reactor_Mask),\n   * except handle is explicitly specified."]
    pub unsafe fn register_handler_uaf2cc7107110a9fd(
        __this: *mut Self,
        mut io_handle: libc::c_int,
        mut event_handler: *mut ACE_Event_Handler,
        mut mask: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor16register_handlerEiP17ACE_Event_Handlerm"]
            fn __ext(
                __this: *mut ACE_Reactor,
                io_handle: libc::c_int,
                event_handler: *mut ACE_Event_Handler,
                mask: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, io_handle, event_handler, mask)
    }
    #[doc = "* Register handler for I/O events.\n   *\n   * Similar to\n   * register_handler(ACE_HANDLE,ACE_Event_Handler*,ACE_Reactor_Mask),\n   * except that the user gets to specify the event handle that will\n   * be used for this registration.  This only applies to Reactors\n   * that use event handles for I/O registrations."]
    pub unsafe fn register_handler_u4af593dc003c5889(
        __this: *mut Self,
        mut event_handle: libc::c_int,
        mut io_handle: libc::c_int,
        mut event_handler: *mut ACE_Event_Handler,
        mut mask: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor16register_handlerEiiP17ACE_Event_Handlerm"]
            fn __ext(
                __this: *mut ACE_Reactor,
                event_handle: libc::c_int,
                io_handle: libc::c_int,
                event_handler: *mut ACE_Event_Handler,
                mask: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, event_handle, io_handle, event_handler, mask)
    }
    #[doc = "* Register handler for multiple I/O events.\n   *\n   * Shorthand for calling\n   * register_handler(ACE_HANDLE,ACE_Event_Handler*,ACE_Reactor_Mask),\n   * multiple times for the same @a event_handler and @a masks but\n   * different @a handles."]
    pub unsafe fn register_handler_u36bd3ab61e17806b(
        __this: *mut Self,
        mut handles: *const ACE_Handle_Set,
        mut event_handler: *mut ACE_Event_Handler,
        mut masks: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor16register_handlerERK14ACE_Handle_SetP17ACE_Event_Handlerm"]
            fn __ext(
                __this: *mut ACE_Reactor,
                handles: *const ACE_Handle_Set,
                event_handler: *mut ACE_Event_Handler,
                masks: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, handles, event_handler, masks)
    }
    #[doc = "* Register handler for signals.\n   *\n   * Register @a new_sh to handle the signal @a signum using the\n   * @a new_disp.  Returns the @a old_sh that was previously registered\n   * (if any), along with the @a old_disp of the signal handler.\n   *\n   * Reactor will call ACE_Event_Handler::add_reference() on @a new_sh\n   * and ACE_Event_Handler::remove_reference() on @a old_sh."]
    pub unsafe fn register_handler_ubdaebacc7f8d596e(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut new_sh: *mut ACE_Event_Handler,
        mut new_disp: *mut ACE_Sig_Action,
        mut old_sh: *mut *mut ACE_Event_Handler,
        mut old_disp: *mut ACE_Sig_Action,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_uebbb7291ea242e70)(
                        __obj,
                        signum,
                        new_sh,
                        new_disp,
                        old_sh,
                        old_disp,
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Register handler for multiple signals.\n   *\n   * Shorthand for calling\n   * register_handler(int,ACE_Event_Handler*,ACE_Sig_Action*,ACE_Event_Handler**,ACE_Sig_Action*)\n   * multiple times for the same @a event_handler and @a sig_action but\n   * different <signals>."]
    pub unsafe fn register_handler_u6b90fc56da169ad5(
        __this: *mut Self,
        mut sigset: *const ACE_Sig_Set,
        mut new_sh: *mut ACE_Event_Handler,
        mut new_disp: *mut ACE_Sig_Action,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_uf0baf69df97ee5bb)(
                        __obj,
                        ::core::ptr::addr_of!((* sigset)),
                        new_sh,
                        new_disp,
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Remove @a masks from @a handle registration.\n   *\n   * For I/O handles, @a masks are removed from the Reactor.  Unless\n   * @a masks includes ACE_Event_Handler::DONT_CALL,\n   * ACE_Event_Handler::handle_close() will be called with the @a masks\n   * that have been removed.  If all masks have been removed,\n   * ACE_Event_Handler::remove_reference() will be called.\n   *\n   * For OS handles, the @a handle is removed from the Reactor.  Unless\n   * @a masks includes ACE_Event_Handler::DONT_CALL,\n   * ACE_Event_Handler::handle_close() will be called with\n   * ACE_Event_Handler::NULL_MASK.\n   * ACE_Event_Handler::remove_reference() will also be called."]
    pub unsafe fn remove_handler(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut mask: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_udac36f861cf0a8f3)(__obj, handle, mask)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Remove @a masks from @a event_handler registration.\n   *\n   * Same as remove_handler(ACE_HANDLE,ACE_Reactor_Mask), except\n   * @a handle comes from ACE_Event_Handler::get_handle()."]
    pub unsafe fn remove_handler_udff48cd4d7d1d15c(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
        mut mask: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_uaf0dc9c8418b9132)(__obj, event_handler, mask)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Remove @a masks from multiple <handle> registrations.\n   *\n   * Shorthand for calling remove_handler(ACE_HANDLE,ACE_Reactor_Mask)\n   * multiple times for the same @a masks but different @a handles."]
    pub unsafe fn remove_handler_ua83495ecc720ff6d(
        __this: *mut Self,
        mut handle_set: *const ACE_Handle_Set,
        mut mask: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_uc14a6d6420dea3b3)(
                        __obj,
                        ::core::ptr::addr_of!((* handle_set)),
                        mask,
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Remove signal handler registration.\n   *\n   * Remove the ACE_Event_Handler currently associated with @a signum.\n   * Install the new disposition (if given) and return the previous\n   * disposition (if desired by the caller).\n   *\n   * Note that the registered handler's ACE_Event_Handler::handle_close ()\n   * callback will be called to indicate the signal handler has been removed.\n   * Unlike with I/O handles, there is no way to prevent this callback. The\n   * handle_close() callback can check the passed mask for the value\n   * ACE_Event_Handler::SIGNAL_MASK to tell when the callback is the result\n   * of a signal handler removal."]
    pub unsafe fn remove_handler_u37c581ec4c0aaac3(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut new_disp: *mut ACE_Sig_Action,
        mut old_disp: *mut ACE_Sig_Action,
        mut sigkey: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_uc75cd90734e55231)(
                        __obj,
                        signum,
                        new_disp,
                        old_disp,
                        sigkey,
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Remove multiple signal handler registrations.\n   *\n   * Shorthand for calling\n   * remove_handler(int,ACE_Sig_Action*,ACE_Sig_Action*,int) multiple\n   * times for every signal in @a sigset."]
    pub unsafe fn remove_handler_ub18d7a4490b36909(
        __this: *mut Self,
        mut sigset: *const ACE_Sig_Set,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_ud3adb6a76edadf4b)(__obj, ::core::ptr::addr_of!((* sigset)))
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///* Suspend @a handle temporarily.
    pub unsafe fn suspend_handler(
        __this: *mut Self,
        mut handle: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_udf3e1116f7643292)(__obj, handle)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Suspend @a event_handler temporarily.\n   *\n   * Handle is obtained from ACE_Event_Handler::get_handle()."]
    pub unsafe fn suspend_handler_u79c6c7c76cfe4f05(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u2ab95efe2aaafd93)(__obj, event_handler)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Suspend @a handles temporarily.\n   *\n   * Shorthand for calling suspend_handler(ACE_HANDLE) with multiple\n   * @a handles."]
    pub unsafe fn suspend_handler_u110b3caad1469a2c(
        __this: *mut Self,
        mut handles: *const ACE_Handle_Set,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_u466ed1ff499b0d62)(
                        __obj,
                        ::core::ptr::addr_of!((* handles)),
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///* Suspend all registered handles temporarily.
    pub unsafe fn suspend_handlers(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u02fd627e956944c7)(__obj)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///* Resume @a handle.
    pub unsafe fn resume_handler(
        __this: *mut Self,
        mut handle: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_udc0eba225fcd4a87)(__obj, handle)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Resume @a event_handler.\n   *\n   * Handle is obtained from ACE_Event_Handler::get_handle()."]
    pub unsafe fn resume_handler_u2386b103cd196b7e(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u4b62ea5905f5d054)(__obj, event_handler)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Resume @a handles.\n   *\n   * Shorthand for calling resume_handler(ACE_HANDLE) with multiple\n   * @a handles."]
    pub unsafe fn resume_handler_u8b4b44d1f5951137(
        __this: *mut Self,
        mut handles: *const ACE_Handle_Set,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_u8b4ef2f95c2e8dbd)(
                        __obj,
                        ::core::ptr::addr_of!((* handles)),
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///* Resume all registered handles.
    pub unsafe fn resume_handlers(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_uadbacf5f6b488bd0)(__obj)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Does the reactor allow the application to resume the handle on
  /// its own ie. can it pass on the control of handle resumption to
  /// the application. A positive value indicates that the handlers
  /// are application resumable. A value of 0 indicates otherwise.*/
    pub unsafe fn resumable_handler(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_ufe6ad1e266584666)(__obj)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Schedule a timer event.\n   *\n   * Schedule a timer event that will expire after an @a delay amount\n   * of time.  The return value of this method, a timer_id value,\n   * uniquely identifies the @a event_handler in the ACE_Reactor's\n   * internal list of timers.  This timer_id value can be used to\n   * cancel the timer with the cancel_timer() call.\n   *\n   * Reactor will call ACE_Event_Handler::add_reference() on the\n   * handler.  After the timeout occurs and\n   * ACE_Event_Handler::handle_timeout() has completed, the handler\n   * will be implicitly removed from the Reactor and\n   * ACE_Event_Handler::remove_reference() will be called.\n   *\n   * @see cancel_timer()\n   * @see reset_timer_interval()\n   *\n   * @param event_handler Event handler to schedule on reactor. The handler's\n   *                      handle_timeout() method will be called when this\n   *                      scheduled timer expires.\n   * @param arg           Argument passed to the handle_timeout() method of\n   *                      event_handler.\n   * @param delay         Time interval after which the timer will expire.\n   * @param interval      Time interval for which the timer will be\n   *                      automatically rescheduled if the handle_timeout()\n   *                      callback does not return a value less than 0.\n   *\n   * @retval              timer id, on success. The id can be used to\n   *                      cancel or reschedule this timer.\n   * @retval              -1 on failure, with errno set."]
    pub unsafe fn schedule_timer(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
        mut arg: *const libc::c_void,
        mut delay: *const ACE_Time_Value,
        mut interval: *const ACE_Time_Value,
    ) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor14schedule_timerEP17ACE_Event_HandlerPKvRK14ACE_Time_ValueS6_"]
            fn __ext(
                __this: *mut ACE_Reactor,
                event_handler: *mut ACE_Event_Handler,
                arg: *const libc::c_void,
                delay: *const ACE_Time_Value,
                interval: *const ACE_Time_Value,
            ) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Reactor, event_handler, arg, delay, interval)
    }
    #[doc = "* Reset recurring timer interval.\n   *\n   * Resets the interval of the timer represented by @a timer_id to\n   * @a interval, which is specified in relative time to the current\n   * gettimeofday().  If @a interval is equal to\n   * ACE_Time_Value::zero, the timer will become a non-rescheduling\n   * timer.  Returns 0 if successful, -1 if not.\n   *\n   * This change will not take effect until the next timeout."]
    pub unsafe fn reset_timer_interval(
        __this: *mut Self,
        mut timer_id: libc::c_long,
        mut interval: *const ACE_Time_Value,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor20reset_timer_intervalElRK14ACE_Time_Value"]
            fn __ext(
                __this: *mut ACE_Reactor,
                timer_id: libc::c_long,
                interval: *const ACE_Time_Value,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, timer_id, interval)
    }
    #[doc = "* Cancel timer.\n   *\n   * Cancel timer associated with @a timer_id that was returned from\n   * the schedule_timer() method.  If arg is non-NULL then it will be\n   * set to point to the ``magic cookie'' argument passed in when the\n   * handler was registered.  This makes it possible to free up the\n   * memory and avoid memory leaks.  Returns 1 if cancellation\n   * succeeded and 0 if the @a timer_id wasn't found.\n   *\n   * On successful cancellation, ACE_Event_Handler::handle_close()\n   * will be called with ACE_Event_Handler::TIMER_MASK.\n   * ACE_Event_Handler::remove_reference() will also be called."]
    pub unsafe fn cancel_timer(
        __this: *mut Self,
        mut timer_id: libc::c_long,
        mut arg: *mut *const libc::c_void,
        mut dont_call_handle_close: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor12cancel_timerElPPKvi"]
            fn __ext(
                __this: *mut ACE_Reactor,
                timer_id: libc::c_long,
                arg: *mut *const libc::c_void,
                dont_call_handle_close: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, timer_id, arg, dont_call_handle_close)
    }
    #[doc = "* Cancel all timers associated with event handler.\n   *\n   * Shorthand for calling cancel_timer(long,const void **,int)\n   * multiple times for all timer associated with @a event_handler.\n   *\n   * ACE_Event_Handler::handle_close() will be called with\n   * ACE_Event_Handler::TIMER_MASK only once irrespective of the\n   * number of timers associated with the event handler.\n   * ACE_Event_Handler::remove_reference() will also be called once\n   * for every timer associated with the event handler.\n   *\n   * In case this operation is called with a nil event_handler\n   * it returns with 0 as the number of handlers cancelled.\n   *\n   * Returns number of handlers cancelled."]
    pub unsafe fn cancel_timer_ue8bc061cd761dc66(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
        mut dont_call_handle_close: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor12cancel_timerEP17ACE_Event_Handleri"]
            fn __ext(
                __this: *mut ACE_Reactor,
                event_handler: *mut ACE_Event_Handler,
                dont_call_handle_close: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, event_handler, dont_call_handle_close)
    }
    /**Add @a masks_to_be_added to the @a event_handler's entry.
  /// @a event_handler must already have been registered.
  /// Note that this call does not cause the Reactor to re-examine
  /// its set of handlers - the new masks will be noticed the next
  /// time the Reactor waits for activity. If there is no other
  /// activity expected, or you need immediate re-examination of the
  /// wait masks, either call ACE_Reactor::notify after this call, or
  /// use ACE_Reactor::register_handler instead.*/
    pub unsafe fn schedule_wakeup(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
        mut masks_to_be_added: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor15schedule_wakeupEP17ACE_Event_Handlerm"]
            fn __ext(
                __this: *mut ACE_Reactor,
                event_handler: *mut ACE_Event_Handler,
                masks_to_be_added: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, event_handler, masks_to_be_added)
    }
    /**Add @a masks_to_be_added to the @a handle's entry.  <event_handler>
  /// associated with @a handle must already have been registered.
  /// Note that this call does not cause the Reactor to re-examine
  /// its set of handlers - the new masks will be noticed the next
  /// time the Reactor waits for activity. If there is no other
  /// activity expected, or you need immediate re-examination of
  /// the wait masks, either call ACE_Reactor::notify after this call,
  /// or use ACE_Reactor::register_handler instead.*/
    pub unsafe fn schedule_wakeup_u7e0c328a3f4b194d(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut masks_to_be_added: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u3a4948b251e007ab)(__obj, handle, masks_to_be_added)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Clear @a masks_to_be_cleared from the @a event_handler's entry.
  /// Note that this call does not cause the Reactor to re-examine
  /// its set of handlers - the new masks will be noticed the next
  /// time the Reactor waits for activity. If there is no other
  /// activity expected, or you need immediate re-examination of
  /// the wait masks, either call ACE_Reactor::notify after this
  /// call, or use ACE_Reactor::register_handler instead.*/
    pub unsafe fn cancel_wakeup(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
        mut masks_to_be_cleared: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_uaef99343c0b53cf3)(
                        __obj,
                        event_handler,
                        masks_to_be_cleared,
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Clear @a masks_to_be_cleared from the @a handle's entry.
  /// Note that this call does not cause the Reactor to re-examine
  /// its set of handlers - the new masks will be noticed the next
  /// time the Reactor waits for activity. If there is no other
  /// activity expected, or you need immediate re-examination of
  /// the wait masks, either call ACE_Reactor::notify after this
  /// call, or use ACE_Reactor::register_handler instead.*/
    pub unsafe fn cancel_wakeup_u49131cf1b0901386(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut masks_to_be_cleared: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u70d2d1718da0b0a8)(__obj, handle, masks_to_be_cleared)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Dispatch user specified events.\n   *\n   * Handler will be dispatched irrespective of whether it is\n   * registered, not registered, or suspended in the Reactor.\n   *\n   * If user specified event is successfully queued,\n   * ACE_Event_Handler::add_reference() will be called.  After the\n   * notify occurs and the upcall to the handler completes, the\n   * handler will be implicitly removed from the Reactor and\n   * ACE_Event_Handler::remove_reference() will be called.  No other\n   * upcall reference counting is done.\n   *\n   * For I/O or OS events, the upcall is invoked with an\n   * ACE_INVALID_HANDLE.\n   *\n   * For timer events, the upcall is invoked with a null ACT.\n   *\n   * @param event_handler: IN - Handler on which the event will be\n   * dispatched.\n   * @param masks: IN - Events to be dispatched - multiple events can\n   * be OR'd together.\n   * @param timeout: INOUT - Relative time up to which to wait for\n   * user specified event to be queued.  If tv is 0, wait\n   * indefinitely.  When the call returns, tv has the time remaining\n   * after the call completes."]
    pub unsafe fn notify(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
        mut masks: libc::c_ulong,
        mut timeout: *mut ACE_Time_Value,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Reactor6notifyEP17ACE_Event_HandlermP14ACE_Time_Value"]
            fn __ext(
                __this: *mut ACE_Reactor,
                event_handler: *mut ACE_Event_Handler,
                masks: libc::c_ulong,
                timeout: *mut ACE_Time_Value,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Reactor, event_handler, masks, timeout)
    }
    #[doc = "* Set the maximum number of times that ACE_Reactor will\n   * iterate and dispatch the ACE_Event_Handlers that are passed in\n   * via the notify queue before breaking out of its\n   * ACE_Message_Queue::dequeue() loop.  By default, this is set to\n   * -1, which means \"iterate until the queue is empty.\"  Setting this\n   * to a value like \"1 or 2\" will increase \"fairness\" (and thus\n   * prevent starvation) at the expense of slightly higher dispatching\n   * overhead."]
    pub unsafe fn max_notify_iterations(__this: *mut Self, mut iterations: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u324d3bbce1889e52)(__obj, iterations)
                };
            }
            ()
        }
    }
    #[doc = "* Get the maximum number of times that the ACE_Reactor will\n   * iterate and dispatch the ACE_Event_Handler's that are passed in\n   * via the notify queue before breaking out of its\n   * ACE_Message_Queue::dequeue() loop."]
    pub unsafe fn max_notify_iterations_u6d29c9549ee55598(
        __this: *mut Self,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u5d120e242195d36e)(__obj)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Purge any notifications pending in this reactor for the specified\n   * ACE_Event_Handler object. If @a eh == 0, all notifications for\n   * all handlers are removed (but not any notifications posted just\n   * to wake up the reactor itself). Returns the number of\n   * notifications purged.  Returns -1 on error.\n   *\n   * After the purging occurs, the handler will be implicitly removed\n   * from the Reactor and ACE_Event_Handler::remove_reference() will\n   * be called."]
    pub unsafe fn purge_pending_notifications(
        __this: *mut Self,
        mut eh: *mut ACE_Event_Handler,
        mut mask: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u9b86516632b01291)(__obj, eh, mask)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Return the Event_Handler associated with @a handle.  Return 0 if\n   * @a handle is not registered.\n   *\n   * Reactor will call ACE_Event_Handler::add_reference() on the\n   * handler before returning it."]
    pub unsafe fn find_handler(
        __this: *mut Self,
        mut handle: libc::c_int,
    ) -> *mut ACE_Event_Handler {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u495d4b8c1fe69e57)(__obj, handle)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Check to see if @a handle is associated with a valid Event_Handler\n   * bound to @a mask.  Return the @c event_handler associated with this\n   * @a handler if @a event_handler != 0.\n   *\n   * Reactor will call ACE_Event_Handler::add_reference() on the\n   * handler before returning it if @a event_handler != 0."]
    pub unsafe fn handler(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut mask: libc::c_ulong,
        mut event_handler: *mut *mut ACE_Event_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u17856e9c20782909)(__obj, handle, mask, event_handler)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Check to see if @a signum is associated with a valid Event_Handler\n   * bound to a signal.  Return the @a event_handler associated with\n   * this @c handler if @a event_handler != 0."]
    pub unsafe fn handler_u5e78c7b17ceecfb2(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut event_handler: *mut *mut ACE_Event_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u02a50aad3bea1efc)(__obj, signum, event_handler)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Returns true if Reactor has been successfully initialized, else
  /// false.*/
    pub unsafe fn initialized(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (({
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_ubf7b5dbfbdbd9de1)(__obj)
                }) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Returns the current size of the Reactor's internal descriptor
  /// table.*/
    pub unsafe fn size(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_ub8a3988b3562f107)(__obj)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Returns a reference to the Reactor's internal lock.
    pub unsafe fn lock(__this: *mut Self) -> *mut ACE_Lock {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ({
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_ucda5680deed6f816)(__obj)
                }) as *mut ACE_Lock;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Wake up all threads in waiting in the event loop
    pub unsafe fn wakeup_all_threads(__this: *mut Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u995d18386c3ca1a6)(__obj)
                };
            }
            ()
        }
    }
    ///Transfers ownership of Reactor to the @a new_owner.
    pub unsafe fn owner(
        __this: *mut Self,
        mut new_owner: libc::c_ulong,
        mut old_owner: *mut libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u61a984cbe2966a4e)(__obj, new_owner, old_owner)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the ID of the "owner" thread.
    pub unsafe fn owner_u13a6434b91b5f6e3(
        __this: *mut Self,
        mut owner: *mut libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u95f3298d130aaa99)(__obj, owner)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set position of the owner thread.
    pub unsafe fn requeue_position(__this: *mut Self, mut position: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u5d59f9127796d40d)(__obj, position)
                };
            }
            ()
        }
    }
    ///Get position of the owner thread.
    pub unsafe fn requeue_position_u34f4788a775e7bc7(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_ub3766cf129b99a3d)(__obj)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the existing restart value.
    pub unsafe fn restart(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy({
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u7b8714cbca14692e)(__obj)
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set a new value for restart and return the original value.
    pub unsafe fn restart_uab7a1a791d8bac4f(__this: *mut Self, mut r: bool) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy({
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u1782c7fc493b0fe5)(__obj, r)
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**GET/SET/ADD/CLR the dispatch mask "bit" bound with the
  /// @a event_handler and @a mask.*/
    pub unsafe fn mask_ops(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
        mut mask: libc::c_ulong,
        mut ops: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u54940c38c361ab8a)(__obj, event_handler, mask, ops)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**GET/SET/ADD/CLR the dispatch MASK "bit" bound with the @a handle
  /// and @a mask.*/
    pub unsafe fn mask_ops_ufda807de2364380d(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut mask: libc::c_ulong,
        mut ops: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u213f3877c01f842b)(__obj, handle, mask, ops)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**GET/SET/ADD/CLR the ready "bit" bound with the @a event_handler
  /// and @a mask.*/
    pub unsafe fn ready_ops(
        __this: *mut Self,
        mut event_handler: *mut ACE_Event_Handler,
        mut mask: libc::c_ulong,
        mut ops: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u3f6fdf50bbadffcd)(__obj, event_handler, mask, ops)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///GET/SET/ADD/CLR the ready "bit" bound with the @a handle and @a mask.
    pub unsafe fn ready_ops_u0cb3b885d3d40c78(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut mask: libc::c_ulong,
        mut ops: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_u22e60c7be962e926)(__obj, handle, mask, ops)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the implementation class
    pub unsafe fn implementation(__this: *const Self) -> *mut ACE_Reactor_Impl {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).implementation_) as *mut ACE_Reactor_Impl);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Returns 0, if the size of the current message has been put in\n   * @a size returns -1, if not.  ACE_HANDLE allows the reactor to\n   * check if the caller is valid.  Used for CLASSIX Reactor\n   * implementation."]
    pub unsafe fn current_info(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut size: *mut libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt)
                        .vfn_u6a7b69fc514d715c)(
                        __obj,
                        handle,
                        ::core::ptr::addr_of_mut!((* size)),
                    )
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Return true if we any event associations were made by the reactor
  /// for the handles that it waits on, false otherwise.*/
    pub unsafe fn uses_event_associations(__this: *mut Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy({
                    let __obj: *mut ACE_Reactor_Impl = (<ACE_Reactor>::implementation(
                        (__this) as *const ACE_Reactor,
                    )) as *mut ACE_Reactor_Impl;
                    let __vt: *const __Vtbl_u7168bc4535358c82 = *(__obj
                        as *const *const __Vtbl_u7168bc4535358c82);
                    ((*__vt).vfn_ub5a322e104b295a3)(__obj)
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of the object.
    pub unsafe fn dump(__this: *const Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {}
            ()
        }
    }
    ///Set the implementation class.
    pub unsafe fn implementation_uf481170c464bfb3e(
        __this: *mut Self,
        mut r#impl: *mut ACE_Reactor_Impl,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).implementation_ = r#impl;
            }
            ()
        }
    }
    ///Deny access since member-wise won't work...
    pub unsafe fn new_at_u951994d5dd1d32a2(
        __this: *mut Self,
        mut __a0: *const ACE_Reactor,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_ReactorC1ERKS_"]
            fn __ext(__this: *mut ACE_Reactor, __a0: *const ACE_Reactor);
        }
        __ext(__this as *mut ACE_Reactor, __a0)
    }
    pub unsafe fn new_u951994d5dd1d32a2(mut __a0: *const ACE_Reactor) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u951994d5dd1d32a2(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Reactor,
    ) -> *mut ACE_Reactor {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_ReactoraSERKS_"]
            fn __ext(
                __this: *mut ACE_Reactor,
                _anon_0: *const ACE_Reactor,
            ) -> *mut ACE_Reactor;
        }
        __ext(__this as *mut ACE_Reactor, _anon_0)
    }
}
impl ACE_Event_Handler {
    ///Get the I/O handle.
    pub unsafe fn get_handle(__this: *const Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Event_Handler10get_handleEv"]
            fn __ext(__this: *const ACE_Event_Handler) -> libc::c_int;
        }
        __ext(__this as *const ACE_Event_Handler)
    }
    ///Set the I/O handle.
    pub unsafe fn set_handle(__this: *mut Self, mut _anon_0: libc::c_int) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler10set_handleEi"]
            fn __ext(__this: *mut ACE_Event_Handler, _anon_0: libc::c_int);
        }
        __ext(__this as *mut ACE_Event_Handler, _anon_0)
    }
    /**Get the priority of the Event_Handler.
  /// @note Priorities run from MIN_PRIORITY (which is the "lowest priority")
  /// to MAX_PRIORITY (which is the "highest priority").*/
    pub unsafe fn priority(__this: *const Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Event_Handler8priorityEv"]
            fn __ext(__this: *const ACE_Event_Handler) -> libc::c_int;
        }
        __ext(__this as *const ACE_Event_Handler)
    }
    ///Set the priority of the Event_Handler.
    pub unsafe fn priority_u8c5dfd1fdaa209ab(
        __this: *mut Self,
        mut priority: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler8priorityEi"]
            fn __ext(__this: *mut ACE_Event_Handler, priority: libc::c_int);
        }
        __ext(__this as *mut ACE_Event_Handler, priority)
    }
    ///Called when input events occur (e.g., connection or data).
    pub unsafe fn handle_input(__this: *mut Self, mut fd: libc::c_int) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler12handle_inputEi"]
            fn __ext(__this: *mut ACE_Event_Handler, fd: libc::c_int) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler, fd)
    }
    /**Called when output events are possible (e.g., when flow control
  /// abates or non-blocking connection completes).*/
    pub unsafe fn handle_output(__this: *mut Self, mut fd: libc::c_int) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler13handle_outputEi"]
            fn __ext(__this: *mut ACE_Event_Handler, fd: libc::c_int) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler, fd)
    }
    ///Called when an exceptional events occur (e.g., SIGURG).
    pub unsafe fn handle_exception(
        __this: *mut Self,
        mut fd: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler16handle_exceptionEi"]
            fn __ext(__this: *mut ACE_Event_Handler, fd: libc::c_int) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler, fd)
    }
    #[doc = "* Called when timer expires.  @a current_time represents the current\n   * time that the Event_Handler was selected for timeout\n   * dispatching and @a act is the asynchronous completion token that\n   * was passed in when <schedule_timer> was invoked."]
    pub unsafe fn handle_timeout(
        __this: *mut Self,
        mut current_time: *const ACE_Time_Value,
        mut act: *const libc::c_void,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler14handle_timeoutERK14ACE_Time_ValuePKv"]
            fn __ext(
                __this: *mut ACE_Event_Handler,
                current_time: *const ACE_Time_Value,
                act: *const libc::c_void,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler, current_time, act)
    }
    ///Called when a process exits.
    pub unsafe fn handle_exit(
        __this: *mut Self,
        mut _anon_0: *mut ACE_Process,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler11handle_exitEP11ACE_Process"]
            fn __ext(
                __this: *mut ACE_Event_Handler,
                _anon_0: *mut ACE_Process,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler, _anon_0)
    }
    /**Called when a handle_*() method returns -1 or when the
  /// remove_handler() method is called on an ACE_Reactor.  The
  /// @a close_mask indicates which event has triggered the
  /// handle_close() method callback on a particular @a handle.*/
    pub unsafe fn handle_close(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut close_mask: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler12handle_closeEim"]
            fn __ext(
                __this: *mut ACE_Event_Handler,
                handle: libc::c_int,
                close_mask: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler, handle, close_mask)
    }
    /**Called when object is signaled by OS (either via UNIX signals or
  /// when a Win32 object becomes signaled).*/
    pub unsafe fn handle_signal(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut _anon_1: *mut siginfo_t,
        mut _anon_2: *mut ucontext_t,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler13handle_signalEiP9siginfo_tP10ucontext_t"]
            fn __ext(
                __this: *mut ACE_Event_Handler,
                signum: libc::c_int,
                _anon_1: *mut siginfo_t,
                _anon_2: *mut ucontext_t,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler, signum, _anon_1, _anon_2)
    }
    #[doc = "* Called to figure out whether the handler needs to resumed by the\n   * reactor or the application can take care of it. The default\n   * value of 0 would be returned which would allow the reactor to\n   * take care of resumption of the handler. The application can\n   * return a value more than zero and decide to resume the handler\n   * themselves.\n   *\n   * @note This method has an affect only when used with the\n   * ACE_Dev_Poll_Reactor (and then, only on Linux) or the ACE_TP_Reactor."]
    pub unsafe fn resume_handler(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler14resume_handlerEv"]
            fn __ext(__this: *mut ACE_Event_Handler) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler)
    }
    pub unsafe fn handle_qos(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler10handle_qosEi"]
            fn __ext(
                __this: *mut ACE_Event_Handler,
                _anon_0: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler, _anon_0)
    }
    pub unsafe fn handle_group_qos(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler16handle_group_qosEi"]
            fn __ext(
                __this: *mut ACE_Event_Handler,
                _anon_0: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Event_Handler, _anon_0)
    }
    ///Set the event demultiplexors.
    pub unsafe fn reactor(__this: *mut Self, mut reactor: *mut ACE_Reactor) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler7reactorEP11ACE_Reactor"]
            fn __ext(__this: *mut ACE_Event_Handler, reactor: *mut ACE_Reactor);
        }
        __ext(__this as *mut ACE_Event_Handler, reactor)
    }
    ///Get the event demultiplexors.
    pub unsafe fn reactor_ub75675325861507c(__this: *const Self) -> *mut ACE_Reactor {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Event_Handler7reactorEv"]
            fn __ext(__this: *const ACE_Event_Handler) -> *mut ACE_Reactor;
        }
        __ext(__this as *const ACE_Event_Handler)
    }
    ///Get only the reactor's timer related interface.
    pub unsafe fn reactor_timer_interface(
        __this: *const Self,
    ) -> *mut ACE_Reactor_Timer_Interface {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Event_Handler23reactor_timer_interfaceEv"]
            fn __ext(
                __this: *const ACE_Event_Handler,
            ) -> *mut ACE_Reactor_Timer_Interface;
        }
        __ext(__this as *const ACE_Event_Handler)
    }
    #[doc = "* Used to read from non-socket ACE_HANDLEs in our own thread to\n   * work around Win32 limitations that don't allow us to <select> on\n   * non-sockets (such as ACE_STDIN).  This is commonly used in\n   * situations where the Reactor is used to demultiplex read events\n   * on ACE_STDIN on UNIX.  Note that @a event_handler must be a\n   * subclass of ACE_Event_Handler.  If the get_handle() method of\n   * this event handler returns ACE_INVALID_HANDLE we default to\n   * reading from ACE_STDIN."]
    pub unsafe fn read_adapter(
        mut event_handler: *mut libc::c_void,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler12read_adapterEPv"]
            fn __ext(event_handler: *mut libc::c_void) -> *mut libc::c_void;
        }
        __ext(event_handler)
    }
    #[doc = "* Abstracts away from the differences between Win32 and ACE with\n   * respect to reading from ACE_STDIN, which is non-<select>'able on\n   * Win32."]
    pub unsafe fn register_stdin_handler(
        mut eh: *mut ACE_Event_Handler,
        mut reactor: *mut ACE_Reactor,
        mut thr_mgr: *mut ACE_Thread_Manager,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler22register_stdin_handlerEPS_P11ACE_ReactorP18ACE_Thread_Manageri"]
            fn __ext(
                eh: *mut ACE_Event_Handler,
                reactor: *mut ACE_Reactor,
                thr_mgr: *mut ACE_Thread_Manager,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(eh, reactor, thr_mgr, flags)
    }
    ///Performs the inverse of the register_stdin_handler() method.
    pub unsafe fn remove_stdin_handler(
        mut reactor: *mut ACE_Reactor,
        mut thr_mgr: *mut ACE_Thread_Manager,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler20remove_stdin_handlerEP11ACE_ReactorP18ACE_Thread_Manager"]
            fn __ext(
                reactor: *mut ACE_Reactor,
                thr_mgr: *mut ACE_Thread_Manager,
            ) -> libc::c_int;
        }
        __ext(reactor, thr_mgr)
    }
    #[doc = "Increment reference count on the handler.\n  /**\n   * This method is called when the handler is registered with the\n   * Reactor and when the Reactor makes an upcall on the handler.\n   * Reference count is 1 when the handler is created.\n   *\n   * @return Current reference count."]
    pub unsafe fn add_reference(__this: *mut Self) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler13add_referenceEv"]
            fn __ext(__this: *mut ACE_Event_Handler) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Event_Handler)
    }
    #[doc = "Decrement reference count on the handler.\n  /**\n   * This method is called when the handler is removed from the\n   * Reactor and when an upcall made on the handler by the Reactor\n   * completes.  Handler is deleted when the reference count reaches\n   * 0.\n   *\n   * @return Current reference count."]
    pub unsafe fn remove_reference(__this: *mut Self) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler16remove_referenceEv"]
            fn __ext(__this: *mut ACE_Event_Handler) -> libc::c_long;
        }
        __ext(__this as *mut ACE_Event_Handler)
    }
    ///Current Reference_Counting_Policy.
    pub unsafe fn reference_counting_policy(
        __this: *mut Self,
    ) -> *mut Reference_Counting_Policy {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_Handler25reference_counting_policyEv"]
            fn __ext(__this: *mut ACE_Event_Handler) -> *mut Reference_Counting_Policy;
        }
        __ext(__this as *mut ACE_Event_Handler)
    }
    ///Force ACE_Event_Handler to be an abstract base class.
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *mut ACE_Reactor,
        mut __a1: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Event_HandlerC1EP11ACE_Reactori"]
            fn __ext(
                __this: *mut ACE_Event_Handler,
                __a0: *mut ACE_Reactor,
                __a1: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Event_Handler, __a0, __a1)
    }
    pub unsafe fn new(mut __a0: *mut ACE_Reactor, mut __a1: libc::c_int) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
}
impl ACE_Event_Handler_var {
    ///Default constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_Event_Handler_varC1Ev"]
            fn __ext(__this: *mut ACE_Event_Handler_var);
        }
        __ext(__this as *mut ACE_Event_Handler_var)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Construct with a handler.
    pub unsafe fn new_at_ued1a07363602f0bd(
        __this: *mut Self,
        mut __a0: *mut ACE_Event_Handler,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_Event_Handler_varC1EP17ACE_Event_Handler"]
            fn __ext(__this: *mut ACE_Event_Handler_var, __a0: *mut ACE_Event_Handler);
        }
        __ext(__this as *mut ACE_Event_Handler_var, __a0)
    }
    pub unsafe fn new_ued1a07363602f0bd(mut __a0: *mut ACE_Event_Handler) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ued1a07363602f0bd(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Copy constructor.
    pub unsafe fn new_at_u1846162ad699b028(
        __this: *mut Self,
        mut __a0: *const ACE_Event_Handler_var,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_Event_Handler_varC1ERKS_"]
            fn __ext(
                __this: *mut ACE_Event_Handler_var,
                __a0: *const ACE_Event_Handler_var,
            );
        }
        __ext(__this as *mut ACE_Event_Handler_var, __a0)
    }
    pub unsafe fn new_u1846162ad699b028(mut __a0: *const ACE_Event_Handler_var) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u1846162ad699b028(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Assignment to a handler.
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut p: *mut ACE_Event_Handler,
    ) -> *mut ACE_Event_Handler_var {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_Event_Handler_varaSEP17ACE_Event_Handler"]
            fn __ext(
                __this: *mut ACE_Event_Handler_var,
                p: *mut ACE_Event_Handler,
            ) -> *mut ACE_Event_Handler_var;
        }
        __ext(__this as *mut ACE_Event_Handler_var, p)
    }
    ///Assignment to a ACE_Event_Handler_var.
    pub unsafe fn operator_assign_uffe189ba2e82d75e(
        __this: *mut Self,
        mut b: *const ACE_Event_Handler_var,
    ) -> *mut ACE_Event_Handler_var {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_Event_Handler_varaSERKS_"]
            fn __ext(
                __this: *mut ACE_Event_Handler_var,
                b: *const ACE_Event_Handler_var,
            ) -> *mut ACE_Event_Handler_var;
        }
        __ext(__this as *mut ACE_Event_Handler_var, b)
    }
    ///Overloaded "->".
    pub unsafe fn operator_arrow(__this: *const Self) -> *mut ACE_Event_Handler {
        extern "C-unwind" {
            #[link_name = "_ZNK21ACE_Event_Handler_varptEv"]
            fn __ext(__this: *const ACE_Event_Handler_var) -> *mut ACE_Event_Handler;
        }
        __ext(__this as *const ACE_Event_Handler_var)
    }
    ///Access the handler.
    pub unsafe fn handler(__this: *const Self) -> *mut ACE_Event_Handler {
        extern "C-unwind" {
            #[link_name = "_ZNK21ACE_Event_Handler_var7handlerEv"]
            fn __ext(__this: *const ACE_Event_Handler_var) -> *mut ACE_Event_Handler;
        }
        __ext(__this as *const ACE_Event_Handler_var)
    }
    ///Release the handler.
    pub unsafe fn release(__this: *mut Self) -> *mut ACE_Event_Handler {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_Event_Handler_var7releaseEv"]
            fn __ext(__this: *mut ACE_Event_Handler_var) -> *mut ACE_Event_Handler;
        }
        __ext(__this as *mut ACE_Event_Handler_var)
    }
    ///Reset the handler.
    pub unsafe fn reset(__this: *mut Self, mut p: *mut ACE_Event_Handler) {
        extern "C-unwind" {
            #[link_name = "_ZN21ACE_Event_Handler_var5resetEP17ACE_Event_Handler"]
            fn __ext(__this: *mut ACE_Event_Handler_var, p: *mut ACE_Event_Handler);
        }
        __ext(__this as *mut ACE_Event_Handler_var, p)
    }
    ///Bool operator to check if the ACE_Event_Handler_var has a value
    pub unsafe fn operator_bool(__this: *const Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK21ACE_Event_Handler_varcvbEv"]
            fn __ext(__this: *const ACE_Event_Handler_var) -> bool;
        }
        __ext(__this as *const ACE_Event_Handler_var)
    }
}
impl ACE_Notification_Buffer {
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Notification_BufferC1Ev"]
            fn __ext(__this: *mut ACE_Notification_Buffer);
        }
        __ext(__this as *mut ACE_Notification_Buffer)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    pub unsafe fn new_at_u2c59014350f736d9(
        __this: *mut Self,
        mut __a0: *mut ACE_Event_Handler,
        mut __a1: libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN23ACE_Notification_BufferC1EP17ACE_Event_Handlerm"]
            fn __ext(
                __this: *mut ACE_Notification_Buffer,
                __a0: *mut ACE_Event_Handler,
                __a1: libc::c_ulong,
            );
        }
        __ext(__this as *mut ACE_Notification_Buffer, __a0, __a1)
    }
    pub unsafe fn new_u2c59014350f736d9(
        mut __a0: *mut ACE_Event_Handler,
        mut __a1: libc::c_ulong,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u2c59014350f736d9(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
}
impl ACE_Process_Manager {
    #[doc = "* Initialize an ACE_Process_Manager with a table containing up to\n   * @a size processes.  This table resizes itself automatically as\n   * needed.  If a @a reactor is provided, this\n   * ACE_Process_Manager uses it to notify an application when a\n   * process it controls exits.  By default, however, we don't use an\n   * ACE_Reactor."]
    pub unsafe fn new_at(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut r: *mut ACE_Reactor,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Event_Handler>::new_at(
                (::core::ptr::addr_of_mut!((* __this).__base_0)
                    .cast::<ACE_Event_Handler>()) as *mut ACE_Event_Handler,
                ((0) as *mut ACE_Reactor),
                (((0 as libc::c_int)) as libc::c_int),
            );
            *(__this as *mut *const ()) = &__VTBL_u3baeb18df2f9ff7c
                as *const __Vtbl_u3baeb18df2f9ff7c as *const ();
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).process_table_),
                ((0) as *mut Process_Descriptor),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).max_process_table_size_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).current_count_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).default_exit_handler_),
                ((0) as *mut ACE_Event_Handler),
            );
            <ACE_Recursive_Thread_Mutex>::new_at(
                (::core::ptr::addr_of_mut!((* __this).lock_)
                    .cast::<ACE_Recursive_Thread_Mutex>())
                    as *mut ACE_Recursive_Thread_Mutex,
                ((0) as *const libc::c_char),
                ((0) as *mut pthread_mutexattr_t),
            );
            {
                if (((((<ACE_Process_Manager>::open(
                    (__this) as *mut ACE_Process_Manager,
                    size,
                    r,
                ) as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    'dowhile_0: loop {
                        'cont_0: loop {
                            {
                                {
                                    let mut __ace_error: libc::c_int = ((<ACE_Log_Msg>::last_error_adapter())
                                        as libc::c_int);
                                    let mut ace___: *mut ACE_Log_Category_TSS = <ACE_Log_Category>::per_thr_obj(
                                        (::core::ptr::addr_of_mut!(
                                            (* < ACE_Log_Category > ::ace_lib())
                                        )) as *mut ACE_Log_Category,
                                    );
                                    if (((((ace___).is_null()) as libc::c_int) as libc::c_int)
                                        != 0)
                                    {
                                        break 'dowhile_0;
                                    }
                                    <ACE_Log_Category_TSS>::conditional_set(
                                        (ace___) as *mut ACE_Log_Category_TSS,
                                        ((b"/build/ace_full/src/ACE/ace/Process_Manager.cpp\0"
                                            .as_ptr() as *const libc::c_char) as *const libc::c_char),
                                        248,
                                        (-((1) as libc::c_int)),
                                        ((__ace_error) as libc::c_int),
                                    );
                                    <ACE_Log_Category_TSS>::log(
                                        (ace___) as *mut ACE_Log_Category_TSS,
                                        LM_ERROR,
                                        ((b"%p\n\0".as_ptr() as *const libc::c_char)
                                            as *const libc::c_char),
                                        b"ACE_Process_Manager\0".as_ptr() as *const libc::c_char,
                                    );
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                        if !(((0) != 0)) {
                            break 'dowhile_0;
                        }
                    }
                }
            }
            ()
        }
    }
    pub unsafe fn new(mut __a0: libc::c_ulong, mut __a1: *mut ACE_Reactor) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    #[doc = "* Initialize an ACE_Process_Manager with a table containing up to\n   * @a size processes.  This table resizes itself automatically as\n   * needed.  If a @a reactor is provided, this\n   * ACE_Process_Manager uses it to notify an application when a\n   * process it controls exits.  By default, however, we don't use an\n   * ACE_Reactor."]
    pub unsafe fn open(
        __this: *mut Self,
        mut size: libc::c_ulong,
        mut r: *mut ACE_Reactor,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (!(r).is_null()) {
                    {
                        let __obj: *mut ACE_Event_Handler = (__this)
                            as *mut ACE_Event_Handler;
                        let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                            as *const *const __Vtbl_ud05ba0669c8bb3a2);
                        ((*__vt).vfn_u5cb133098a439c05)(__obj, r)
                    };
                    if (((((<ACE_Reactor>::register_handler_ubdaebacc7f8d596e(
                        (r) as *mut ACE_Reactor,
                        17,
                        ((__this) as *mut ACE_Event_Handler),
                        ((0) as *mut ACE_Sig_Action),
                        ((0) as *mut *mut ACE_Event_Handler),
                        ((0) as *mut ACE_Sig_Action),
                    ) as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        return (-((1) as libc::c_int));
                    }
                }
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                if ((((((*__this).max_process_table_size_ as libc::c_ulong))
                    < (((size) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    <ACE_Process_Manager>::resize(
                        (__this) as *mut ACE_Process_Manager,
                        size,
                    );
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Release all resources.  Do not wait for processes to exit.
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((!({
                    let __obj: *mut ACE_Event_Handler = (__this)
                        as *mut ACE_Event_Handler;
                    let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                        as *const *const __Vtbl_ud05ba0669c8bb3a2);
                    ((*__vt).vfn_ub75675325861507c)(__obj)
                })
                    .is_null()) as libc::c_int) as libc::c_int) != 0)
                {
                    <ACE_Reactor>::remove_handler_u37c581ec4c0aaac3(
                        ({
                            let __obj: *mut ACE_Event_Handler = (__this)
                                as *mut ACE_Event_Handler;
                            let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                                as *const *const __Vtbl_ud05ba0669c8bb3a2);
                            ((*__vt).vfn_ub75675325861507c)(__obj)
                        }) as *mut ACE_Reactor,
                        17,
                        (0 as *mut ACE_Sig_Action),
                        ((0) as *mut ACE_Sig_Action),
                        (-((1) as libc::c_int)),
                    );
                    {
                        let __obj: *mut ACE_Event_Handler = (__this)
                            as *mut ACE_Event_Handler;
                        let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                            as *const *const __Vtbl_ud05ba0669c8bb3a2);
                        ((*__vt).vfn_u5cb133098a439c05)(__obj, ((0) as *mut ACE_Reactor))
                    };
                }
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                if ((((!((*__this).process_table_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    'while_0: loop {
                        if !(((((((*__this).current_count_ as libc::c_ulong))
                            > (((0) as libc::c_ulong))) as libc::c_int as libc::c_int)
                            != 0))
                        {
                            break 'while_0;
                        }
                        'cont_0: loop {
                            {
                                <ACE_Process_Manager>::remove_proc(
                                    (__this) as *mut ACE_Process_Manager,
                                    ((0) as libc::c_ulong),
                                );
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                    }
                    {
                        let __data = (*__this).process_table_ as *mut Process_Descriptor;
                        if !__data.is_null() {
                            let __off = ::core::alloc::Layout::new::<usize>()
                                .extend(
                                    ::core::alloc::Layout::array::<Process_Descriptor>(1)
                                        .unwrap(),
                                )
                                .unwrap()
                                .1;
                            let __base = (__data as *mut u8).sub(__off);
                            let __count = *(__base as *mut usize);
                            let mut __i: usize = 0;
                            while __i < __count {
                                ::core::ptr::drop_in_place(__data.add(__i));
                                __i += 1;
                            }
                            ::libc::free(__base as *mut libc::c_void);
                        }
                    };
                    (*__this).process_table_ = ((0) as *mut Process_Descriptor);
                    (*__this).max_process_table_size_ = ((0) as libc::c_ulong);
                    (*__this).current_count_ = ((0) as libc::c_ulong);
                }
                if ((((!((*__this).default_exit_handler_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    {
                        let __obj: *mut ACE_Event_Handler = ((*__this)
                            .default_exit_handler_) as *mut ACE_Event_Handler;
                        let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                            as *const *const __Vtbl_ud05ba0669c8bb3a2);
                        ((*__vt)
                            .vfn_ua1174916b4160f19)(
                            __obj,
                            (((-((1) as libc::c_int))) as libc::c_int),
                            ((0) as libc::c_ulong),
                        )
                    };
                }
                (*__this).default_exit_handler_ = ((0) as *mut ACE_Event_Handler);
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get pointer to a process-wide ACE_Process_Manager.
    pub unsafe fn instance() -> *mut ACE_Process_Manager {
        unsafe {
            {
                if (((((ACE_Process_Manager_instance_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                        ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    >::zeroed()
                        .assume_init();
                    <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                        (::core::ptr::addr_of_mut!(ace_mon))
                            as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                        <ACE_Static_Object_Lock>::instance(),
                    );
                    if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                        (::core::ptr::addr_of!(ace_mon))
                            as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)
                    {} else {
                        return ((0) as *mut ACE_Process_Manager);
                    };
                    if (((((ACE_Process_Manager_instance_).is_null()) as libc::c_int)
                        as libc::c_int) != 0)
                    {
                        'dowhile_0: loop {
                            'cont_0: loop {
                                {
                                    {
                                        ACE_Process_Manager_instance_ = {
                                            let __new: *mut ACE_Process_Manager = Box::into_raw(
                                                Box::new(
                                                    ::core::mem::MaybeUninit::<ACE_Process_Manager>::zeroed()
                                                        .assume_init(),
                                                ),
                                            );
                                            <ACE_Process_Manager>::new_at(
                                                (__new) as *mut ACE_Process_Manager,
                                                (((100 as libc::c_int)) as libc::c_ulong),
                                                ((0) as *mut ACE_Reactor),
                                            );
                                            __new
                                        };
                                        if (((((ACE_Process_Manager_instance_).is_null())
                                            as libc::c_int) as libc::c_int) != 0)
                                        {
                                            ((*(__errno_location()))) = 12;
                                            return ((0) as *mut ACE_Process_Manager);
                                        }
                                    }
                                }
                                #[allow(unreachable_code)] break 'cont_0;
                            }
                            if !(((0) != 0)) {
                                break 'dowhile_0;
                            }
                        }
                        ACE_Process_Manager_delete_instance_ = true;
                        <ACE_Object_Manager>::at_exit_ud30f528992f3d733(
                            ((ACE_Process_Manager_instance_) as *mut libc::c_void),
                            Some({
                                unsafe extern "C-unwind" fn __shim(
                                    a0: *mut libc::c_void,
                                    a1: *mut libc::c_void,
                                ) {
                                    unsafe { ACE_Process_Manager::cleanup(a0, a1) }
                                }
                                __shim
                                    as unsafe extern "C-unwind" fn(
                                        *mut libc::c_void,
                                        *mut libc::c_void,
                                    )
                            }),
                            ((0) as *mut libc::c_void),
                            unsafe {
                                (*((&__TYPEINFO_19ACE_Process_Manager
                                    as *const crate::__cxx_std::TypeInfo)))
                                    .name()
                            },
                        );
                    }
                }
                return ACE_Process_Manager_instance_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Set pointer to a process-wide ACE_Process_Manager and return
  /// existing pointer.*/
    pub unsafe fn instance_ud8939d75278aebda(
        mut tm: *mut ACE_Process_Manager,
    ) -> *mut ACE_Process_Manager {
        unsafe {
            {
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    <ACE_Static_Object_Lock>::instance(),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return ((0) as *mut ACE_Process_Manager);
                };
                let mut t: *mut ACE_Process_Manager = ACE_Process_Manager_instance_;
                ACE_Process_Manager_delete_instance_ = false;
                <ACE_Object_Manager>::at_exit_ud30f528992f3d733(
                    ((ACE_Process_Manager_instance_) as *mut libc::c_void),
                    Some({
                        unsafe extern "C-unwind" fn __shim(
                            a0: *mut libc::c_void,
                            a1: *mut libc::c_void,
                        ) {
                            unsafe { ACE_Process_Manager::cleanup(a0, a1) }
                        }
                        __shim
                            as unsafe extern "C-unwind" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            )
                    }),
                    ((0) as *mut libc::c_void),
                    unsafe {
                        (*({
                            let __obj = (t) as *const u8;
                            let __vt = *(__obj as *const *const u8);
                            *(__vt as *const *const crate::__cxx_std::TypeInfo)
                        }))
                            .name()
                    },
                );
                ACE_Process_Manager_instance_ = tm;
                return t;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Delete the dynamically allocated singleton.
    pub unsafe fn close_singleton() {
        unsafe {
            {
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    <ACE_Static_Object_Lock>::instance(),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return;
                };
                if ((ACE_Process_Manager_delete_instance_ as libc::c_int) != 0) {
                    {
                        let __p = ACE_Process_Manager_instance_;
                        if !__p.is_null() {
                            let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__p
                                as *const *const __Vtbl_ud05ba0669c8bb3a2);
                            ((*__vt).__vdtor)(__p as *mut ACE_Event_Handler);
                        }
                    };
                    ACE_Process_Manager_instance_ = ((0) as *mut ACE_Process_Manager);
                    ACE_Process_Manager_delete_instance_ = false;
                }
            }
            ()
        }
    }
    /**Cleanup method, used by the ACE_Object_Manager to destroy the
  /// singleton.*/
    pub unsafe fn cleanup(
        mut _anon_0: *mut libc::c_void,
        mut _anon_1: *mut libc::c_void,
    ) {
        unsafe {
            {
                <ACE_Process_Manager>::close_singleton();
            }
            ()
        }
    }
    #[doc = "* Create a new process with specified @a options.\n   * Register @a event_handler to be called back when the process exits.\n   * The @a proc object's ACE_Process::unmanage() method is called when\n   * the process is removed from ACE_Process_Manager.\n   *\n   * On success, returns the process id of the child that was created.\n   * On failure, returns ACE_INVALID_PID."]
    pub unsafe fn spawn(
        __this: *mut Self,
        mut process: *mut ACE_Process,
        mut options: *mut ACE_Process_Options,
        mut event_handler: *mut ACE_Event_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut pid: libc::c_int = (({
                    let __obj: *mut ACE_Process = (process) as *mut ACE_Process;
                    let __vt: *const __Vtbl_u75264ee698c7f168 = *(__obj
                        as *const *const __Vtbl_u75264ee698c7f168);
                    ((*__vt)
                        .vfn_u3bdf0a03b95f6b5a)(
                        __obj,
                        ::core::ptr::addr_of_mut!((* options)),
                    )
                }) as libc::c_int);
                if (((((((((pid as libc::c_int))
                    == ((((((-((1) as libc::c_int)) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                    || (((((pid as libc::c_int)) == (((0) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    return ((pid) as libc::c_int);
                }
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (((-((1) as libc::c_int))) as libc::c_int);
                };
                if (((((<ACE_Process_Manager>::append_proc(
                    (__this) as *mut ACE_Process_Manager,
                    process,
                    event_handler,
                ) as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (((-((1) as libc::c_int)) as libc::c_int));
                }
                return ((pid) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Create a new process with the specified @a options.\n   * Register @a event_handler to be called back when the process exits.\n   *\n   * On success, returns the process id of the child that was created.\n   * On failure, returns ACE_INVALID_PID."]
    pub unsafe fn spawn_u723e6092e06de6bf(
        __this: *mut Self,
        mut options: *mut ACE_Process_Options,
        mut event_handler: *mut ACE_Event_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut process: *mut ACE_Process = ((0) as *mut ACE_Process);
                'dowhile_0: loop {
                    'cont_0: loop {
                        {
                            {
                                process = (({
                                    let mut __new: ACE_Managed_Process = unsafe {
                                        {
                                            let mut __obj: ACE_Managed_Process = ::core::mem::MaybeUninit::<
                                                ACE_Managed_Process,
                                            >::zeroed()
                                                .assume_init();
                                            let __this: *mut ACE_Managed_Process = &mut __obj
                                                as *mut ACE_Managed_Process;
                                            ::core::ptr::write(
                                                ::core::ptr::addr_of_mut!((* __this).__base_0)
                                                    .cast::<ACE_Process>(),
                                                <ACE_Process>::new(),
                                            );
                                            *(__this as *mut *const ()) = &__VTBL_u5baae7ee749c5722
                                                as *const __Vtbl_u5baae7ee749c5722 as *const ();
                                            __obj
                                        }
                                    };
                                    Box::into_raw(Box::new(__new))
                                }) as *mut ACE_Process);
                                if (((((process).is_null()) as libc::c_int) as libc::c_int)
                                    != 0)
                                {
                                    ((*(__errno_location()))) = 12;
                                    return (((-((1) as libc::c_int)) as libc::c_int));
                                }
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                    if !(((0) != 0)) {
                        break 'dowhile_0;
                    }
                }
                let mut pid: libc::c_int = ((<ACE_Process_Manager>::spawn(
                    (__this) as *mut ACE_Process_Manager,
                    process,
                    ::core::ptr::addr_of_mut!((* options)),
                    event_handler,
                )) as libc::c_int);
                if (((((((((pid as libc::c_int))
                    == ((((((-((1) as libc::c_int)) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                    || (((((pid as libc::c_int)) == (((0) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    {
                        let __p = process;
                        if !__p.is_null() {
                            let __vt: *const __Vtbl_u75264ee698c7f168 = *(__p
                                as *const *const __Vtbl_u75264ee698c7f168);
                            ((*__vt).__vdtor)(__p as *mut ACE_Process);
                        }
                    };
                }
                return ((pid) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Create @a n new processes with the same @a options.\n   * If @a child_pids is non-0 it is expected to be an array of at least\n   * @a n pid_t, which are filled in with the process IDs of the spawned\n   * processes.\n   * Register @a event_handler to be called back when each process exits.\n   * Returns 0 on success and -1 on failure."]
    pub unsafe fn spawn_n(
        __this: *mut Self,
        mut n: libc::c_ulong,
        mut options: *mut ACE_Process_Options,
        mut child_pids: *mut libc::c_int,
        mut event_handler: *mut ACE_Event_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((!(child_pids).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    {
                        let mut i: libc::c_ulong = ((0) as libc::c_ulong);
                        'for_0: loop {
                            if !((((((i as libc::c_ulong)) < (((n) as libc::c_ulong)))
                                as libc::c_int as libc::c_int) != 0))
                            {
                                break;
                            }
                            'cont_0: loop {
                                {
                                    (*(child_pids).wrapping_offset((i) as isize)) = (((-((1)
                                        as libc::c_int)) as libc::c_int));
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
                }
                {
                    let mut i: libc::c_ulong = ((0) as libc::c_ulong);
                    'for_1: loop {
                        if !((((((i as libc::c_ulong)) < (((n) as libc::c_ulong)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break;
                        }
                        'cont_1: loop {
                            {
                                {
                                    let mut pid: libc::c_int = ((<ACE_Process_Manager>::spawn_u723e6092e06de6bf(
                                        (__this) as *mut ACE_Process_Manager,
                                        ::core::ptr::addr_of_mut!((* options)),
                                        event_handler,
                                    )) as libc::c_int);
                                    if (((((((((pid as libc::c_int))
                                        == ((((((-((1) as libc::c_int)) as libc::c_int)))
                                            as libc::c_int))) as libc::c_int as libc::c_int) != 0)
                                        || (((((pid as libc::c_int)) == (((0) as libc::c_int)))
                                            as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                                        as libc::c_int) != 0)
                                    {
                                        return ((pid) as libc::c_int);
                                    } else {
                                        if ((((!(child_pids).is_null()) as libc::c_int)
                                            as libc::c_int) != 0)
                                        {
                                            (*(child_pids).wrapping_offset((i) as isize)) = ((pid)
                                                as libc::c_int);
                                        }
                                    }
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_1;
                        }
                        {
                            let __lv = &mut (i);
                            let __r = *__lv;
                            *__lv = (*__lv).wrapping_add(1);
                            __r
                        };
                    }
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Abruptly terminate a single process with id @a pid using the\n   * ACE::terminate_process() method which works on both signal-capable\n   * systems and on Windows.\n   *\n   * @note This call is potentially dangerous to use since the process\n   * being terminated may not have a chance to cleanup before it shuts down.\n   * The process's entry is also not removed from this class's process\n   * table. Calling either wait() or remove() after terminate() is\n   * advisable.\n   *\n   * @retval 0 on success and -1 on failure."]
    pub unsafe fn terminate(__this: *mut Self, mut pid: libc::c_int) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                let mut i: libc::c_long = ((<ACE_Process_Manager>::find_proc(
                    (__this) as *mut ACE_Process_Manager,
                    pid,
                )) as libc::c_long);
                if (((((i as libc::c_long))
                    == ((((-((1) as libc::c_int))) as libc::c_long))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                return ACE::terminate_process(pid);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Sends the specified signal to the specified process.\n   *\n   * @note This only works on platforms that have signal capability. In\n   * particular, it doesn't work on Windows.\n   *\n   * @retval 0 on success and -1 on failure."]
    pub unsafe fn terminate_u78afa949f27d6dc6(
        __this: *mut Self,
        mut pid: libc::c_int,
        mut sig: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                let mut i: libc::c_long = ((<ACE_Process_Manager>::find_proc(
                    (__this) as *mut ACE_Process_Manager,
                    pid,
                )) as libc::c_long);
                if (((((i as libc::c_long))
                    == ((((-((1) as libc::c_int))) as libc::c_long))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                return ACE_OS::kill_udafa4deef137b5a3(pid, sig);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Block until there are no more child processes running that were\n   * spawned by this ACE_Process_Manager.  Unlike the wait() method\n   * below, this method does not require a signal handler or use of\n   * ACE_OS::sigwait() because it simply blocks synchronously waiting\n   * for all the children managed by this ACE_Process_Manager to\n   * exit.  Note that this does not return any status information\n   * about the success or failure of exiting child processes, although\n   * any registered exit handlers are called.\n   *\n   * @param timeout Relative time to wait for processes to terminate.\n   *\n   * @retval 0 on success; -1 on failure."]
    pub unsafe fn wait(
        __this: *mut Self,
        mut timeout: *const ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut until: ACE_Time_Value = ::core::mem::MaybeUninit::<
                    ACE_Time_Value,
                >::zeroed()
                    .assume_init();
                <ACE_Time_Value>::new_at_u3614bf99aa612c0c(
                    (::core::ptr::addr_of_mut!(until)) as *mut ACE_Time_Value,
                    ::core::ptr::addr_of!((* timeout)),
                );
                let mut remaining: ACE_Time_Value = ::core::mem::MaybeUninit::<
                    ACE_Time_Value,
                >::zeroed()
                    .assume_init();
                <ACE_Time_Value>::new_at_u3614bf99aa612c0c(
                    (::core::ptr::addr_of_mut!(remaining)) as *mut ACE_Time_Value,
                    ::core::ptr::addr_of!((* timeout)),
                );
                if ((operator_lt_uabf160308c113d02(
                    ::core::ptr::addr_of!(until),
                    ::core::ptr::addr_of!(ACE_Time_Value_max_time),
                ) as libc::c_int) != 0)
                {
                    let __addrof_tmp_0 = ACE_OS::gettimeofday_u3220bcbbceb90f45();
                    <ACE_Time_Value>::operator_add_assign(
                        (::core::ptr::addr_of_mut!(until)) as *mut ACE_Time_Value,
                        &__addrof_tmp_0 as *const ACE_Time_Value,
                    );
                }
                'while_0: loop {
                    if !(((((((*__this).current_count_ as libc::c_ulong))
                        > (((0) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0))
                    {
                        break 'while_0;
                    }
                    'cont_0: loop {
                        {
                            {
                                let mut pid: libc::c_int = ((<ACE_Process_Manager>::wait_ud310f8de9a6d598c(
                                    (__this) as *mut ACE_Process_Manager,
                                    ((0) as libc::c_int),
                                    ::core::ptr::addr_of!(remaining),
                                    ((0) as *mut libc::c_int),
                                )) as libc::c_int);
                                if (((((pid as libc::c_int))
                                    == ((((((-((1) as libc::c_int)) as libc::c_int)))
                                        as libc::c_int))) as libc::c_int as libc::c_int) != 0)
                                {
                                    return (-((1) as libc::c_int));
                                } else {
                                    if (((((pid as libc::c_int)) == (((0) as libc::c_int)))
                                        as libc::c_int as libc::c_int) != 0)
                                    {
                                        break 'while_0;
                                    }
                                }
                                let __addrof_tmp_1 = ACE_OS::gettimeofday_u3220bcbbceb90f45();
                                let __addrof_tmp_2 = operator_sub_u7824066738f89e79(
                                    ::core::ptr::addr_of!(until),
                                    &__addrof_tmp_1 as *const ACE_Time_Value,
                                );
                                <ACE_Time_Value>::operator_assign(
                                    (::core::ptr::addr_of_mut!(remaining))
                                        as *mut ACE_Time_Value,
                                    if ((operator_lt_uabf160308c113d02(
                                        ::core::ptr::addr_of!(until),
                                        ::core::ptr::addr_of!(ACE_Time_Value_max_time),
                                    ) as libc::c_int) != 0)
                                    {
                                        &__addrof_tmp_2 as *const ACE_Time_Value
                                    } else {
                                        &(<ACE_Time_Value>::new_u3614bf99aa612c0c(
                                            ::core::ptr::addr_of!(ACE_Time_Value_max_time),
                                        )) as *const ACE_Time_Value
                                    },
                                );
                                if ((operator_le_u4cb918f914889af1(
                                    ::core::ptr::addr_of!(remaining),
                                    ::core::ptr::addr_of!(ACE_Time_Value_zero),
                                ) as libc::c_int) != 0)
                                {
                                    break 'while_0;
                                }
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                }
                return ((*__this).current_count_ as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Wait up to @a timeout for a single specified process to terminate.\n   * If @a pid is 0, this method waits for any of the managed processes\n   * (but see the note concerning \"sloppy process cleanup on unix\").\n   * If @a pid != 0, waits for that process only.\n   *\n   * @param pid     Process ID\n   * @param timeout Relative time to wait for process to terminate\n   * @param status  Exit status of terminated process\n   *\n   * @retval The pid of the process which exited, 0\n   * if a timeout occurred, or ACE_INVALID_PID on error."]
    pub unsafe fn wait_ud310f8de9a6d598c(
        __this: *mut Self,
        mut pid: libc::c_int,
        mut timeout: *const ACE_Time_Value,
        mut status: *mut libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut local_stat: libc::c_int = ((0) as libc::c_int);
                if (((((status).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    status = ::core::ptr::addr_of_mut!(local_stat) as *mut libc::c_int;
                }
                (*(status)) = ((0) as libc::c_int);
                let mut idx: libc::c_long = (((-((1) as libc::c_int))) as libc::c_long);
                let mut proc: *mut ACE_Process = ((0) as *mut ACE_Process);
                {
                    let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                        ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    >::zeroed()
                        .assume_init();
                    <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                        (::core::ptr::addr_of_mut!(ace_mon))
                            as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                        ::core::ptr::addr_of_mut!(
                            (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                            ACE_Recursive_Thread_Mutex > ().cast_mut())
                        ),
                    );
                    if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                        (::core::ptr::addr_of!(ace_mon))
                            as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)
                    {} else {
                        return (((-((1) as libc::c_int))) as libc::c_int);
                    };
                    if (((((pid as libc::c_int)) != (((0) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        idx = <ACE_Process_Manager>::find_proc(
                            (__this) as *mut ACE_Process_Manager,
                            pid,
                        );
                        if (((((idx as libc::c_long))
                            == ((((-((1) as libc::c_int))) as libc::c_long)))
                            as libc::c_int as libc::c_int) != 0)
                        {
                            return (((-((1) as libc::c_int)) as libc::c_int));
                        } else {
                            proc = (*((*__this).process_table_)
                                .wrapping_offset((idx) as isize))
                                .process_;
                        }
                    }
                }
                if ((((!(proc).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    pid = <ACE_Process>::wait_u9c844e55a2eff714(
                        (proc) as *mut ACE_Process,
                        ::core::ptr::addr_of!((* timeout)),
                        status,
                    );
                } else {
                    let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                        ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    >::zeroed()
                        .assume_init();
                    <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                        (::core::ptr::addr_of_mut!(ace_mon))
                            as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                        ::core::ptr::addr_of_mut!(
                            (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                            ACE_Recursive_Thread_Mutex > ().cast_mut())
                        ),
                    );
                    if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                        (::core::ptr::addr_of!(ace_mon))
                            as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)
                    {} else {
                        return (((-((1) as libc::c_int))) as libc::c_int);
                    };
                    if ((operator_eq_u62c91a36bca5f714(
                        ::core::ptr::addr_of!((* timeout)),
                        ::core::ptr::addr_of!(ACE_Time_Value_max_time),
                    ) as libc::c_int) != 0)
                    {
                        pid = ACE_OS::waitpid_ua99c1cff38bad46d(
                            (((-((1) as libc::c_int))) as libc::c_int),
                            status,
                            0,
                            ((0) as libc::c_int),
                        );
                    } else {
                        if ((operator_eq_u62c91a36bca5f714(
                            ::core::ptr::addr_of!((* timeout)),
                            ::core::ptr::addr_of!(ACE_Time_Value_zero),
                        ) as libc::c_int) != 0)
                        {
                            pid = ACE_OS::waitpid_ua99c1cff38bad46d(
                                (((-((1) as libc::c_int))) as libc::c_int),
                                status,
                                1,
                                ((0) as libc::c_int),
                            );
                        } else {
                            let mut old_action: ACE_Sig_Action = ::core::mem::MaybeUninit::<
                                ACE_Sig_Action,
                            >::zeroed()
                                .assume_init();
                            <ACE_Sig_Action>::new_at(
                                (::core::ptr::addr_of_mut!(old_action))
                                    as *mut ACE_Sig_Action,
                            );
                            if ((((({
                                let __obj: *mut ACE_Event_Handler = (__this)
                                    as *mut ACE_Event_Handler;
                                let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                                    as *const *const __Vtbl_ud05ba0669c8bb3a2);
                                ((*__vt).vfn_ub75675325861507c)(__obj)
                            })
                                .is_null()) as libc::c_int) as libc::c_int) != 0)
                            {
                                let mut sigchld_nop_ptr: Option<
                                    unsafe extern "C-unwind" fn(
                                        libc::c_int,
                                        *mut siginfo_t,
                                        *mut ucontext_t,
                                    ),
                                > = Some(sigchld_nop_u6eab7901cba1c161 as _);
                                let mut do_sigchld: ACE_Sig_Action = ::core::mem::MaybeUninit::<
                                    ACE_Sig_Action,
                                >::zeroed()
                                    .assume_init();
                                <ACE_Sig_Action>::new_at_u892bf0cfe5150792(
                                    (::core::ptr::addr_of_mut!(do_sigchld))
                                        as *mut ACE_Sig_Action,
                                    unsafe {
                                        ::core::mem::transmute::<
                                            _,
                                            Option<unsafe extern "C-unwind" fn(libc::c_int)>,
                                        >(unsafe {
                                            ::core::mem::transmute::<
                                                _,
                                                *mut libc::c_void,
                                            >(sigchld_nop_ptr)
                                        })
                                    },
                                    ((0) as *mut __sigset_t),
                                    0,
                                );
                                <ACE_Sig_Action>::register_action(
                                    (::core::ptr::addr_of_mut!(do_sigchld))
                                        as *mut ACE_Sig_Action,
                                    17,
                                    ::core::ptr::addr_of_mut!(old_action) as *mut ACE_Sig_Action,
                                );
                            }
                            let mut tmo: ACE_Time_Value = ::core::mem::MaybeUninit::<
                                ACE_Time_Value,
                            >::zeroed()
                                .assume_init();
                            <ACE_Time_Value>::new_at_u3614bf99aa612c0c(
                                (::core::ptr::addr_of_mut!(tmo)) as *mut ACE_Time_Value,
                                ::core::ptr::addr_of!((* timeout)),
                            );
                            {
                                let mut time_left: ACE_Countdown_Time_T__ = ::core::mem::MaybeUninit::<
                                    ACE_Countdown_Time_T__,
                                >::zeroed()
                                    .assume_init();
                                <ACE_Countdown_Time_T__>::new_at_s6c7afe4405ab6dcb(
                                    (::core::ptr::addr_of_mut!(time_left))
                                        as *mut ACE_Countdown_Time_T__,
                                    ::core::ptr::addr_of_mut!(tmo) as *mut ACE_Time_Value,
                                    &(<ACE_FPointer_Time_Policy>::new())
                                        as *const ACE_FPointer_Time_Policy,
                                );
                                'for_0: loop {
                                    'cont_0: loop {
                                        {
                                            {
                                                pid = ACE_OS::waitpid_ua99c1cff38bad46d(
                                                    (((-((1) as libc::c_int))) as libc::c_int),
                                                    status,
                                                    1,
                                                    ((0) as libc::c_int),
                                                );
                                                if (((((((((pid as libc::c_int)) > (((0) as libc::c_int)))
                                                    as libc::c_int as libc::c_int) != 0)
                                                    || (((((pid as libc::c_int))
                                                        == ((((((-((1) as libc::c_int)) as libc::c_int)))
                                                            as libc::c_int))) as libc::c_int as libc::c_int) != 0))
                                                    as libc::c_int) as libc::c_int) != 0)
                                                {
                                                    break 'for_0;
                                                }
                                                if ((((((((((-((1) as libc::c_int)) as libc::c_int))
                                                    == (((ACE_OS::sleep_u1b7b7e36e28cf584(
                                                        ::core::ptr::addr_of!(tmo),
                                                    )) as libc::c_int))) as libc::c_int as libc::c_int) != 0)
                                                    && (((((((*(__errno_location()))) as libc::c_int))
                                                        == (((4) as libc::c_int))) as libc::c_int as libc::c_int)
                                                        != 0)) as libc::c_int) as libc::c_int) != 0)
                                                {
                                                    break 'cont_0;
                                                }
                                                pid = ((0) as libc::c_int);
                                                break 'for_0;
                                            }
                                        }
                                        #[allow(unreachable_code)] break 'cont_0;
                                    }
                                    <ACE_Countdown_Time_T__>::update(
                                        (::core::ptr::addr_of_mut!(time_left))
                                            as *mut ACE_Countdown_Time_T__,
                                    );
                                }
                            }
                            if ((((({
                                let __obj: *mut ACE_Event_Handler = (__this)
                                    as *mut ACE_Event_Handler;
                                let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                                    as *const *const __Vtbl_ud05ba0669c8bb3a2);
                                ((*__vt).vfn_ub75675325861507c)(__obj)
                            })
                                .is_null()) as libc::c_int) as libc::c_int) != 0)
                            {
                                <ACE_Sig_Action>::register_action(
                                    (::core::ptr::addr_of_mut!(old_action))
                                        as *mut ACE_Sig_Action,
                                    17,
                                    ((0) as *mut ACE_Sig_Action),
                                );
                            }
                        }
                    }
                }
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (((-((1) as libc::c_int))) as libc::c_int);
                };
                if (((((((((pid as libc::c_int))
                    != ((((((-((1) as libc::c_int)) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                    && (((((pid as libc::c_int)) != (((0) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    idx = <ACE_Process_Manager>::find_proc(
                        (__this) as *mut ACE_Process_Manager,
                        pid,
                    );
                    if (((((idx as libc::c_long))
                        == ((((-((1) as libc::c_int))) as libc::c_long))) as libc::c_int
                        as libc::c_int) != 0)
                    {
                        'dowhile_1: loop {
                            'cont_1: loop {
                                {
                                    {
                                        let mut __ace_error: libc::c_int = ((<ACE_Log_Msg>::last_error_adapter())
                                            as libc::c_int);
                                        let mut ace___: *mut ACE_Log_Category_TSS = <ACE_Log_Category>::per_thr_obj(
                                            (::core::ptr::addr_of_mut!(
                                                (* < ACE_Log_Category > ::ace_lib())
                                            )) as *mut ACE_Log_Category,
                                        );
                                        if (((((ace___).is_null()) as libc::c_int) as libc::c_int)
                                            != 0)
                                        {
                                            break 'dowhile_1;
                                        }
                                        <ACE_Log_Category_TSS>::conditional_set(
                                            (ace___) as *mut ACE_Log_Category_TSS,
                                            ((b"/build/ace_full/src/ACE/ace/Process_Manager.cpp\0"
                                                .as_ptr() as *const libc::c_char) as *const libc::c_char),
                                            937,
                                            0,
                                            ((__ace_error) as libc::c_int),
                                        );
                                        <ACE_Log_Category_TSS>::log(
                                            (ace___) as *mut ACE_Log_Category_TSS,
                                            LM_DEBUG,
                                            ((b"(%P|%t) oops, reaped unmanaged %d\n\0".as_ptr()
                                                as *const libc::c_char) as *const libc::c_char),
                                            pid,
                                        );
                                    }
                                }
                                #[allow(unreachable_code)] break 'cont_1;
                            }
                            if !(((0) != 0)) {
                                break 'dowhile_1;
                            }
                        }
                        return pid;
                    } else {
                        proc = (*((*__this).process_table_)
                            .wrapping_offset((idx) as isize))
                            .process_;
                    }
                    if ((((!(proc).is_null()) as libc::c_int) as libc::c_int) != 0) {
                        (if ((((((pid as libc::c_int))
                            == (((<ACE_Process>::getpid((proc) as *const ACE_Process))
                                as libc::c_int))) as libc::c_int) as libc::c_int) != 0)
                        {
                            ()
                        } else {
                            __ace_assert(
                                ((b"/build/ace_full/src/ACE/ace/Process_Manager.cpp\0"
                                    .as_ptr() as *const libc::c_char) as *const libc::c_char),
                                943,
                                ((b"pid == proc->getpid ()\0".as_ptr()
                                    as *const libc::c_char) as *const libc::c_char),
                            )
                        });
                    }
                    <ACE_Process_Manager>::notify_proc_handler(
                        (__this) as *mut ACE_Process_Manager,
                        ((idx) as libc::c_ulong),
                        (*(status)),
                    );
                    <ACE_Process_Manager>::remove(
                        (__this) as *mut ACE_Process_Manager,
                        pid,
                    );
                }
                return pid;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Wait indefinitely for a single, specified process to terminate.\n   * If @a pid is 0, waits for any of the managed processes (but see the\n   * note concerning \"sloppy process cleanup on unix\").\n   * If @a pid != 0, this method waits for that process only.\n   *\n   * @retval The pid of the process which exited, or\n   * ACE_INVALID_PID on error."]
    pub unsafe fn wait_u04054cf0e91bd50a(
        __this: *mut Self,
        mut pid: libc::c_int,
        mut status: *mut libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Process_Manager>::wait_ud310f8de9a6d598c(
                    (__this) as *mut ACE_Process_Manager,
                    pid,
                    ::core::ptr::addr_of!(ACE_Time_Value_max_time),
                    status,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Register an event handler to be called back when the specified\n   * process exits.  If @a pid == ACE_INVALID_PID this handler is called\n   * when any process with no specific handler exits.\n   *\n   * @warning In multithreaded applications, there is a race condition\n   * if a process exits between the time it is spawned and when its\n   * handler is registered.  To avoid this, register the handler at\n   * the time the process is spawned."]
    pub unsafe fn register_handler(
        __this: *mut Self,
        mut eh: *mut ACE_Event_Handler,
        mut pid: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                if (((((pid as libc::c_int))
                    == ((((((-((1) as libc::c_int)) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    if ((((!((*__this).default_exit_handler_).is_null()) as libc::c_int)
                        as libc::c_int) != 0)
                    {
                        {
                            let __obj: *mut ACE_Event_Handler = ((*__this)
                                .default_exit_handler_) as *mut ACE_Event_Handler;
                            let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                                as *const *const __Vtbl_ud05ba0669c8bb3a2);
                            ((*__vt)
                                .vfn_ua1174916b4160f19)(
                                __obj,
                                (((-((1) as libc::c_int))) as libc::c_int),
                                ((0) as libc::c_ulong),
                            )
                        };
                    }
                    (*__this).default_exit_handler_ = eh;
                    return 0;
                }
                let mut i: libc::c_long = ((<ACE_Process_Manager>::find_proc(
                    (__this) as *mut ACE_Process_Manager,
                    pid,
                )) as libc::c_long);
                if (((((i as libc::c_long))
                    == ((((-((1) as libc::c_int))) as libc::c_long))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    ((*(__errno_location()))) = 22;
                    return (-((1) as libc::c_int));
                }
                let mut proc_desc: *mut Process_Descriptor = ::core::ptr::addr_of_mut!(
                    (* ((* __this).process_table_).wrapping_offset((i) as isize))
                );
                if ((((!((*proc_desc).exit_notify_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    {
                        let __obj: *mut ACE_Event_Handler = ((*proc_desc).exit_notify_)
                            as *mut ACE_Event_Handler;
                        let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                            as *const *const __Vtbl_ud05ba0669c8bb3a2);
                        ((*__vt)
                            .vfn_ua1174916b4160f19)(
                            __obj,
                            (((-((1) as libc::c_int))) as libc::c_int),
                            ((0) as libc::c_ulong),
                        )
                    };
                }
                (*proc_desc).exit_notify_ = eh;
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Remove process @a pid from the ACE_Process_Manager's internal records.\n   * This is called automatically by the wait() method if the waited process\n   * exits. This method can also be called after calling terminate() if\n   * there's no need to wait() for the terminated process."]
    pub unsafe fn remove(__this: *mut Self, mut pid: libc::c_int) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                let mut i: libc::c_long = ((<ACE_Process_Manager>::find_proc(
                    (__this) as *mut ACE_Process_Manager,
                    pid,
                )) as libc::c_long);
                if (((((i as libc::c_long))
                    != ((((-((1) as libc::c_int))) as libc::c_long))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return <ACE_Process_Manager>::remove_proc(
                        (__this) as *mut ACE_Process_Manager,
                        ((i) as libc::c_ulong),
                    );
                }
                return (-((1) as libc::c_int));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the number of managed processes.
    pub unsafe fn managed(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).current_count_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Sets the scheduling parameters for process identified by @a pid by\n   * passing @a params, @a pid to ACE_OS::sched_params().\n   *\n   * @retval 0 on success, -1 on failure, and ACE_INVALID_PID when the\n   * specified @a pid is not managed by this ACE_Process_Manager."]
    pub unsafe fn set_scheduler(
        __this: *mut Self,
        mut params: *const ACE_Sched_Params,
        mut pid: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                let mut i: libc::c_long = ((<ACE_Process_Manager>::find_proc(
                    (__this) as *mut ACE_Process_Manager,
                    pid,
                )) as libc::c_long);
                if (((((i as libc::c_long))
                    == ((((-((1) as libc::c_int))) as libc::c_long))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return (((((-((1) as libc::c_int)) as libc::c_int))) as libc::c_int);
                }
                return ACE_OS::sched_params(
                    ::core::ptr::addr_of!((* params)),
                    ((pid) as libc::c_long),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Sets the scheduling parameters for all the processes managed by\n   * this ACE_Process_Manager by passing @a params to\n   * ACE_OS::sched_params().\n   *\n   * @retval 0 on success, -1 on failure."]
    pub unsafe fn set_scheduler_all(
        __this: *mut Self,
        mut params: *const ACE_Sched_Params,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Recursive_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Recursive_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::new_at_sd3d970b1b01b243f(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                    ::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    ),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                {
                    let mut i: libc::c_ulong = ((0) as libc::c_ulong);
                    'for_0: loop {
                        if !((((((i as libc::c_ulong))
                            < ((((*__this).current_count_) as libc::c_ulong)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break;
                        }
                        'cont_0: loop {
                            {
                                {
                                    let mut pid: libc::c_int = ((<ACE_Process>::getpid(
                                        ((*((*__this).process_table_).wrapping_offset((i) as isize))
                                            .process_) as *const ACE_Process,
                                    )) as libc::c_int);
                                    if (((((ACE_OS::sched_params(
                                        ::core::ptr::addr_of!((* params)),
                                        ((pid) as libc::c_long),
                                    ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                                        as libc::c_int) != 0)
                                    {
                                        return (-((1) as libc::c_int));
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
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {}
            ()
        }
    }
    ///Collect one (or more, on unix) process exit status.
    pub unsafe fn handle_input(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut pid: libc::c_int = unsafe { ::core::mem::zeroed() };
                'dowhile_0: loop {
                    'cont_0: loop {
                        {
                            pid = <ACE_Process_Manager>::wait_ud310f8de9a6d598c(
                                (__this) as *mut ACE_Process_Manager,
                                ((0) as libc::c_int),
                                ::core::ptr::addr_of!(ACE_Time_Value_zero),
                                ((0) as *mut libc::c_int),
                            );
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                    if !((((((((((pid as libc::c_int)) != (((0) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)
                        && (((((pid as libc::c_int))
                            != ((((((-((1) as libc::c_int)) as libc::c_int)))
                                as libc::c_int))) as libc::c_int as libc::c_int) != 0))
                        as libc::c_int) as libc::c_int) != 0))
                    {
                        break 'dowhile_0;
                    }
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**If registered with a reactor for SIGCHLD and the reactor closes, this
  /// will get called to notify.*/
    pub unsafe fn handle_close(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
        mut close_mask: libc::c_ulong,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((close_mask as libc::c_ulong))
                    == ((((256 as libc::c_int)) as libc::c_ulong))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    {
                        let __obj: *mut ACE_Event_Handler = (__this)
                            as *mut ACE_Event_Handler;
                        let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                            as *const *const __Vtbl_ud05ba0669c8bb3a2);
                        ((*__vt).vfn_u5cb133098a439c05)(__obj, ((0) as *mut ACE_Reactor))
                    };
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* On Unix, this routine is called asynchronously when a SIGCHLD is\n   * received.  We just tweak the reactor so that it'll call back our\n   * <handle_input> function, which allows us to handle Process exits\n   * synchronously.\n   *\n   * On Win32, this routine is called synchronously, and is passed the\n   * HANDLE of the Process that exited, so we can do all our work here"]
    pub unsafe fn handle_signal(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
        mut si: *mut siginfo_t,
        mut _anon_2: *mut ucontext_t,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let _ = (si);
                };
                return <ACE_Reactor>::notify(
                    ({
                        let __obj: *mut ACE_Event_Handler = (__this)
                            as *mut ACE_Event_Handler;
                        let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                            as *const *const __Vtbl_ud05ba0669c8bb3a2);
                        ((*__vt).vfn_ub75675325861507c)(__obj)
                    }) as *mut ACE_Reactor,
                    ((__this) as *mut ACE_Event_Handler),
                    (((1 as libc::c_int)) as libc::c_ulong),
                    ((0) as *mut ACE_Time_Value),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Resize the pool of Process_Descriptors.
    pub unsafe fn resize(__this: *mut Self, mut size: libc::c_ulong) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((size as libc::c_ulong))
                    <= ((((*__this).max_process_table_size_) as libc::c_ulong)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return 0;
                }
                let mut temp: *mut Process_Descriptor = ((0) as *mut Process_Descriptor);
                'dowhile_0: loop {
                    'cont_0: loop {
                        {
                            {
                                temp = {
                                    let __count: usize = (size) as usize;
                                    let __arr = ::core::alloc::Layout::array::<
                                        Process_Descriptor,
                                    >(__count)
                                        .unwrap();
                                    let (__layout, __off) = ::core::alloc::Layout::new::<
                                        usize,
                                    >()
                                        .extend(__arr)
                                        .unwrap();
                                    let __layout = __layout.pad_to_align();
                                    let __base = ::libc::malloc(__layout.size().max(1))
                                        as *mut u8;
                                    if __base.is_null() {
                                        ::std::alloc::handle_alloc_error(__layout);
                                    }
                                    *(__base as *mut usize) = __count;
                                    let __data = __base.add(__off) as *mut Process_Descriptor;
                                    let mut __i: usize = 0;
                                    while __i < __count {
                                        ::core::ptr::write(
                                            __data.add(__i),
                                            <Process_Descriptor>::new(),
                                        );
                                        __i += 1;
                                    }
                                    __data
                                };
                                if (((((temp).is_null()) as libc::c_int) as libc::c_int)
                                    != 0)
                                {
                                    ((*(__errno_location()))) = 12;
                                    return (-((1) as libc::c_int));
                                }
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                    if !(((0) != 0)) {
                        break 'dowhile_0;
                    }
                }
                {
                    let mut i: libc::c_ulong = ((0) as libc::c_ulong);
                    'for_1: loop {
                        if !((((((i as libc::c_ulong))
                            < ((((*__this).current_count_) as libc::c_ulong)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break;
                        }
                        'cont_1: loop {
                            {
                                {
                                    let __v = ((*((*__this).process_table_)
                                        .wrapping_offset((i) as isize)))
                                        .clone();
                                    let __asg_p = ::core::ptr::addr_of_mut!(
                                        (* (temp).wrapping_offset((i) as isize))
                                    );
                                    *__asg_p = __v;
                                    __asg_p
                                };
                            }
                            #[allow(unreachable_code)] break 'cont_1;
                        }
                        {
                            let __lv = &mut (i);
                            let __r = *__lv;
                            *__lv = (*__lv).wrapping_add(1);
                            __r
                        };
                    }
                }
                (*__this).max_process_table_size_ = size;
                {
                    let __data = (*__this).process_table_ as *mut Process_Descriptor;
                    if !__data.is_null() {
                        let __off = ::core::alloc::Layout::new::<usize>()
                            .extend(
                                ::core::alloc::Layout::array::<Process_Descriptor>(1)
                                    .unwrap(),
                            )
                            .unwrap()
                            .1;
                        let __base = (__data as *mut u8).sub(__off);
                        let __count = *(__base as *mut usize);
                        let mut __i: usize = 0;
                        while __i < __count {
                            ::core::ptr::drop_in_place(__data.add(__i));
                            __i += 1;
                        }
                        ::libc::free(__base as *mut libc::c_void);
                    }
                };
                (*__this).process_table_ = temp;
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Locate the index of the table slot occupied by @a process_id.
  /// Returns -1 if @a process_id is not in the @c process_table_*/
    pub unsafe fn find_proc(__this: *mut Self, mut pid: libc::c_int) -> libc::c_long {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let mut i: libc::c_ulong = ((0) as libc::c_ulong);
                    'for_0: loop {
                        if !((((((i as libc::c_ulong))
                            < ((((*__this).current_count_) as libc::c_ulong)))
                            as libc::c_int as libc::c_int) != 0))
                        {
                            break;
                        }
                        'cont_0: loop {
                            {
                                {
                                    if (((((pid as libc::c_int))
                                        == (((<ACE_Process>::getpid(
                                            ((*((*__this).process_table_).wrapping_offset((i) as isize))
                                                .process_) as *const ACE_Process,
                                        )) as libc::c_int))) as libc::c_int as libc::c_int) != 0)
                                    {
                                        return ((ACE_Utils::truncate_cast___unsigned_long__u503964095c07d269(
                                            ((i) as libc::c_ulong),
                                        )) as libc::c_long);
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
                return (((-((1) as libc::c_int))) as libc::c_long);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Insert a process in the table (checks for duplicates).  Omitting
  /// the process handle won't work on Win32...
  /// Register @a event_handler to be called back when the process exits.*/
    pub unsafe fn insert_proc(
        __this: *mut Self,
        mut proc: *mut ACE_Process,
        mut event_handler: *mut ACE_Event_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((<ACE_Process_Manager>::find_proc(
                    (__this) as *mut ACE_Process_Manager,
                    <ACE_Process>::getpid((proc) as *const ACE_Process),
                ) as libc::c_long)) != ((((-((1) as libc::c_int))) as libc::c_long)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                return <ACE_Process_Manager>::append_proc(
                    (__this) as *mut ACE_Process_Manager,
                    proc,
                    event_handler,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Append information about a process, i.e., its <process_id> in the\n   * @c process_table_.  Each entry is added at the end, growing the\n   * table if necessary.\n   * Register @a event_handler to be called back when the process exits."]
    pub unsafe fn append_proc(
        __this: *mut Self,
        mut proc: *mut ACE_Process,
        mut event_handler: *mut ACE_Event_Handler,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).current_count_ as libc::c_ulong))
                    >= ((((*__this).max_process_table_size_) as libc::c_ulong)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    let mut new_size: libc::c_ulong = ((((*__this)
                        .max_process_table_size_) as libc::c_ulong))
                        .wrapping_mul((2) as libc::c_ulong);
                    if (((((new_size as libc::c_ulong)) == (((0) as libc::c_ulong)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        new_size = (((100 as libc::c_int)) as libc::c_ulong);
                    }
                    if (((((<ACE_Process_Manager>::resize(
                        (__this) as *mut ACE_Process_Manager,
                        new_size,
                    ) as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        return (-((1) as libc::c_int));
                    }
                }
                let mut proc_desc: *mut Process_Descriptor = ::core::ptr::addr_of_mut!(
                    (* ((* __this).process_table_).wrapping_offset(((* __this)
                    .current_count_) as isize))
                );
                (*proc_desc).process_ = proc;
                (*proc_desc).exit_notify_ = event_handler;
                {
                    let __lv = &mut ((*__this).current_count_);
                    *__lv = (*__lv).wrapping_add(1);
                    *__lv
                };
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Actually removes the process at index @a n from the table.  This method
  /// must be called with locks held.*/
    pub unsafe fn remove_proc(__this: *mut Self, mut i: libc::c_ulong) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((!((*((*__this).process_table_).wrapping_offset((i) as isize))
                    .exit_notify_)
                    .is_null()) as libc::c_int) as libc::c_int) != 0)
                {
                    {
                        let __obj: *mut ACE_Event_Handler = ((*((*__this).process_table_)
                            .wrapping_offset((i) as isize))
                            .exit_notify_) as *mut ACE_Event_Handler;
                        let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                            as *const *const __Vtbl_ud05ba0669c8bb3a2);
                        ((*__vt)
                            .vfn_ua1174916b4160f19)(
                            __obj,
                            <ACE_Process>::gethandle(
                                ((*((*__this).process_table_).wrapping_offset((i) as isize))
                                    .process_) as *const ACE_Process,
                            ),
                            ((0) as libc::c_ulong),
                        )
                    };
                    (*((*__this).process_table_).wrapping_offset((i) as isize))
                        .exit_notify_ = ((0) as *mut ACE_Event_Handler);
                }
                {
                    let __obj: *mut ACE_Process = ((*((*__this).process_table_)
                        .wrapping_offset((i) as isize))
                        .process_) as *mut ACE_Process;
                    let __vt: *const __Vtbl_u75264ee698c7f168 = *(__obj
                        as *const *const __Vtbl_u75264ee698c7f168);
                    ((*__vt).vfn_u9963a2b3ddfce4c7)(__obj)
                };
                (*((*__this).process_table_).wrapping_offset((i) as isize)).process_ = ((0)
                    as *mut ACE_Process);
                {
                    let __lv = &mut ((*__this).current_count_);
                    let __r = *__lv;
                    *__lv = (*__lv).wrapping_sub(1);
                    __r
                };
                if ((((((*__this).current_count_ as libc::c_ulong))
                    > (((0) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    {
                        let __v = ((*((*__this).process_table_)
                            .wrapping_offset(((*__this).current_count_) as isize)))
                            .clone();
                        let __asg_p = ::core::ptr::addr_of_mut!(
                            (* ((* __this).process_table_).wrapping_offset((i) as isize))
                        );
                        *__asg_p = __v;
                        __asg_p
                    };
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**If there's a specific handler for the Process at index @a n in the
  /// table, or there's a default handler, call it.*/
    pub unsafe fn notify_proc_handler(
        __this: *mut Self,
        mut i: libc::c_ulong,
        mut exit_code: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((i as libc::c_ulong))
                    < ((((*__this).current_count_) as libc::c_ulong))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    let mut proc_desc: *mut Process_Descriptor = ::core::ptr::addr_of_mut!(
                        (* ((* __this).process_table_).wrapping_offset((i) as isize))
                    );
                    <ACE_Process>::exit_code_uc86b462655ee0a17(
                        ((*proc_desc).process_) as *mut ACE_Process,
                        exit_code,
                    );
                    if ((((!((*proc_desc).exit_notify_).is_null()) as libc::c_int)
                        as libc::c_int) != 0)
                    {
                        {
                            let __obj: *mut ACE_Event_Handler = ((*proc_desc)
                                .exit_notify_) as *mut ACE_Event_Handler;
                            let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                                as *const *const __Vtbl_ud05ba0669c8bb3a2);
                            ((*__vt).vfn_u140f83bfe0d335d9)(__obj, (*proc_desc).process_)
                        };
                    } else {
                        if ((((((((!((*__this).default_exit_handler_).is_null())
                            as libc::c_int) as libc::c_int) != 0)
                            && ((((({
                                let __obj: *mut ACE_Event_Handler = ((*__this)
                                    .default_exit_handler_) as *mut ACE_Event_Handler;
                                let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                                    as *const *const __Vtbl_ud05ba0669c8bb3a2);
                                ((*__vt)
                                    .vfn_u140f83bfe0d335d9)(__obj, (*proc_desc).process_)
                            } as libc::c_int)) < (((0) as libc::c_int))) as libc::c_int
                                as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                        {
                            {
                                let __obj: *mut ACE_Event_Handler = ((*__this)
                                    .default_exit_handler_) as *mut ACE_Event_Handler;
                                let __vt: *const __Vtbl_ud05ba0669c8bb3a2 = *(__obj
                                    as *const *const __Vtbl_ud05ba0669c8bb3a2);
                                ((*__vt)
                                    .vfn_ua1174916b4160f19)(
                                    __obj,
                                    (((-((1) as libc::c_int))) as libc::c_int),
                                    ((0) as libc::c_ulong),
                                )
                            };
                            (*__this).default_exit_handler_ = ((0)
                                as *mut ACE_Event_Handler);
                        }
                    }
                    return 1;
                } else {
                    'dowhile_0: loop {
                        'cont_0: loop {
                            {
                                {
                                    let mut __ace_error: libc::c_int = ((<ACE_Log_Msg>::last_error_adapter())
                                        as libc::c_int);
                                    let mut ace___: *mut ACE_Log_Category_TSS = <ACE_Log_Category>::per_thr_obj(
                                        (::core::ptr::addr_of_mut!(
                                            (* < ACE_Log_Category > ::ace_lib())
                                        )) as *mut ACE_Log_Category,
                                    );
                                    if (((((ace___).is_null()) as libc::c_int) as libc::c_int)
                                        != 0)
                                    {
                                        break 'dowhile_0;
                                    }
                                    <ACE_Log_Category_TSS>::conditional_set(
                                        (ace___) as *mut ACE_Log_Category_TSS,
                                        ((b"/build/ace_full/src/ACE/ace/Process_Manager.cpp\0"
                                            .as_ptr() as *const libc::c_char) as *const libc::c_char),
                                        981,
                                        0,
                                        ((__ace_error) as libc::c_int),
                                    );
                                    <ACE_Log_Category_TSS>::log(
                                        (ace___) as *mut ACE_Log_Category_TSS,
                                        LM_DEBUG,
                                        ((b"(%P:%t|%T) ACE_Process_Manager::notify_proc_handler: unknown/unmanaged process reaped\n\0"
                                            .as_ptr() as *const libc::c_char) as *const libc::c_char),
                                    );
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                        if !(((0) != 0)) {
                            break 'dowhile_0;
                        }
                    }
                    return 0;
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl Process_Descriptor {
    ///Default ctor/dtor.
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).process_),
                ((0) as *mut ACE_Process),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).exit_notify_),
                ((0) as *mut ACE_Event_Handler),
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
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {}
            ()
        }
    }
}
impl ACE_Sig_Set {
    /**Initialize <sigset_> with @a sigset.  If @a sigset == 0 then fill
  /// the set.*/
    pub unsafe fn new_at(__this: *mut Self, mut ss: *mut __sigset_t) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            {
                if (((((ss).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    ACE_OS::sigfillset_ubda4b03e91b23d38(
                        ::core::ptr::addr_of_mut!((* __this).sigset_) as *mut __sigset_t,
                    );
                } else {
                    {
                        let __v = (*(ss));
                        let __asg_p = ::core::ptr::addr_of_mut!((* __this).sigset_);
                        *__asg_p = __v;
                        __asg_p
                    };
                }
            }
            ()
        }
    }
    pub unsafe fn new(mut __a0: *mut __sigset_t) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    /**Initialize <sigset_> with @a sigset.  If @a sigset == 0 then fill
  /// the set.*/
    pub unsafe fn new_at_u74fa6d1f47e29b13(__this: *mut Self, mut ss: *mut ACE_Sig_Set) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            {
                if (((((ss).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    ACE_OS::sigfillset_ubda4b03e91b23d38(
                        ::core::ptr::addr_of_mut!((* __this).sigset_) as *mut __sigset_t,
                    );
                } else {
                    {
                        let __v = (*ss).sigset_;
                        let __asg_p = ::core::ptr::addr_of_mut!((* __this).sigset_);
                        *__asg_p = __v;
                        __asg_p
                    };
                }
            }
            ()
        }
    }
    pub unsafe fn new_u74fa6d1f47e29b13(mut __a0: *mut ACE_Sig_Set) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u74fa6d1f47e29b13(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    /**If @a fill == 0 then initialize the <sigset_> to be empty, else
  /// full.*/
    pub unsafe fn new_at_ud60a876b80356237(__this: *mut Self, mut fill: libc::c_int) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            {
                if ((fill) != 0) {
                    ACE_OS::sigfillset_ubda4b03e91b23d38(
                        ::core::ptr::addr_of_mut!((* __this).sigset_) as *mut __sigset_t,
                    );
                } else {
                    ACE_OS::sigemptyset_udf9bdeb276d35426(
                        ::core::ptr::addr_of_mut!((* __this).sigset_) as *mut __sigset_t,
                    );
                }
            }
            ()
        }
    }
    pub unsafe fn new_ud60a876b80356237(mut __a0: libc::c_int) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ud60a876b80356237(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Create a set that excludes all signals defined by the system.
    pub unsafe fn empty_set(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::sigemptyset_udf9bdeb276d35426(
                    ::core::ptr::addr_of_mut!((* __this).sigset_) as *mut __sigset_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Create a set that includes all signals defined by the system.
    pub unsafe fn fill_set(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::sigfillset_ubda4b03e91b23d38(
                    ::core::ptr::addr_of_mut!((* __this).sigset_) as *mut __sigset_t,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Adds the individual signal specified by @a signo to the set.
    pub unsafe fn sig_add(__this: *mut Self, mut signo: libc::c_int) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::sigaddset_u87d182dd58bf700c(
                    ::core::ptr::addr_of_mut!((* __this).sigset_) as *mut __sigset_t,
                    signo,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Deletes the individual signal specified by @a signo from the set.
    pub unsafe fn sig_del(__this: *mut Self, mut signo: libc::c_int) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::sigdelset_u44b27dd4262df326(
                    ::core::ptr::addr_of_mut!((* __this).sigset_) as *mut __sigset_t,
                    signo,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Checks whether the signal specified by @a signo is in the set.
    pub unsafe fn is_member(__this: *const Self, mut signo: libc::c_int) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::sigismember_u8594864a428d04c3(
                    (::core::ptr::addr_of!((* __this).sigset_) as *const __sigset_t
                        as *mut __sigset_t),
                    signo,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Returns a pointer to the underlying @c sigset_t.
    pub unsafe fn operator___sigset_t__(__this: *mut Self) -> *mut __sigset_t {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of_mut!((* __this).sigset_) as *mut __sigset_t;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Returns a copy of the underlying @c sigset_t.
    pub unsafe fn sigset(__this: *const Self) -> __sigset_t {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).sigset_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK11ACE_Sig_Set4dumpEv"]
            fn __ext(__this: *const ACE_Sig_Set);
        }
        __ext(__this as *const ACE_Sig_Set)
    }
}
impl ACE_Sig_Action {
    ///Default constructor.  Initializes everything to 0.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Sig_ActionC1Ev"]
            fn __ext(__this: *mut ACE_Sig_Action);
        }
        __ext(__this as *mut ACE_Sig_Action)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    /**Assigns the various fields of a @c sigaction struct but doesn't
  /// register for signal handling via the @c sigaction function.*/
    pub unsafe fn new_at_u892bf0cfe5150792(
        __this: *mut Self,
        mut __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a1: *mut __sigset_t,
        mut __a2: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Sig_ActionC1EPFviEP10__sigset_ti"]
            fn __ext(
                __this: *mut ACE_Sig_Action,
                __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
                __a1: *mut __sigset_t,
                __a2: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Sig_Action, __a0, __a1, __a2)
    }
    pub unsafe fn new_u892bf0cfe5150792(
        mut __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a1: *mut __sigset_t,
        mut __a2: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u892bf0cfe5150792(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    /**Assigns the various fields of a @c sigaction struct but doesn't
  /// register for signal handling via the @c sigaction function.*/
    pub unsafe fn new_at_u8b56cd1c456e16e0(
        __this: *mut Self,
        mut __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a1: *const ACE_Sig_Set,
        mut __a2: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Sig_ActionC1EPFviERK11ACE_Sig_Seti"]
            fn __ext(
                __this: *mut ACE_Sig_Action,
                __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
                __a1: *const ACE_Sig_Set,
                __a2: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Sig_Action, __a0, __a1, __a2)
    }
    pub unsafe fn new_u8b56cd1c456e16e0(
        mut __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a1: *const ACE_Sig_Set,
        mut __a2: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u8b56cd1c456e16e0(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    #[doc = "* Assigns the various fields of a @c sigaction struct and registers\n   * the @a handler to process signal @a signum via the @c sigaction\n   * function."]
    pub unsafe fn new_at_u86502fabbb2d72c2(
        __this: *mut Self,
        mut __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a1: libc::c_int,
        mut __a2: *mut __sigset_t,
        mut __a3: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Sig_ActionC1EPFviEiP10__sigset_ti"]
            fn __ext(
                __this: *mut ACE_Sig_Action,
                __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
                __a1: libc::c_int,
                __a2: *mut __sigset_t,
                __a3: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Sig_Action, __a0, __a1, __a2, __a3)
    }
    pub unsafe fn new_u86502fabbb2d72c2(
        mut __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a1: libc::c_int,
        mut __a2: *mut __sigset_t,
        mut __a3: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u86502fabbb2d72c2(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
        );
        __obj
    }
    #[doc = "* Assigns the various fields of a @c sigaction struct and registers\n   * the @a handler to process signal @a signum via the @c sigaction\n   * function."]
    pub unsafe fn new_at_ub54cb9768e84bbd0(
        __this: *mut Self,
        mut __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a1: libc::c_int,
        mut __a2: *const ACE_Sig_Set,
        mut __a3: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Sig_ActionC1EPFviEiRK11ACE_Sig_Seti"]
            fn __ext(
                __this: *mut ACE_Sig_Action,
                __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
                __a1: libc::c_int,
                __a2: *const ACE_Sig_Set,
                __a3: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Sig_Action, __a0, __a1, __a2, __a3)
    }
    pub unsafe fn new_ub54cb9768e84bbd0(
        mut __a0: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a1: libc::c_int,
        mut __a2: *const ACE_Sig_Set,
        mut __a3: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ub54cb9768e84bbd0(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
        );
        __obj
    }
    #[doc = "* Assigns the various fields of a @c sigaction struct and registers\n   * the @a handler to process all @a signalss via the @c sigaction\n   * function."]
    pub unsafe fn new_at_u9835d3d7c2ae36e9(
        __this: *mut Self,
        mut __a0: *const ACE_Sig_Set,
        mut __a1: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a2: *const ACE_Sig_Set,
        mut __a3: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Sig_ActionC1ERK11ACE_Sig_SetPFviES2_i"]
            fn __ext(
                __this: *mut ACE_Sig_Action,
                __a0: *const ACE_Sig_Set,
                __a1: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
                __a2: *const ACE_Sig_Set,
                __a3: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Sig_Action, __a0, __a1, __a2, __a3)
    }
    pub unsafe fn new_u9835d3d7c2ae36e9(
        mut __a0: *const ACE_Sig_Set,
        mut __a1: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a2: *const ACE_Sig_Set,
        mut __a3: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u9835d3d7c2ae36e9(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
        );
        __obj
    }
    #[doc = "* Assigns the various fields of a @c sigaction struct and registers\n   * the @a handler to process all @a signalss via the @c sigaction\n   * function."]
    pub unsafe fn new_at_u22c0e21ba8c3dfc5(
        __this: *mut Self,
        mut __a0: *const ACE_Sig_Set,
        mut __a1: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a2: *mut __sigset_t,
        mut __a3: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Sig_ActionC1ERK11ACE_Sig_SetPFviEP10__sigset_ti"]
            fn __ext(
                __this: *mut ACE_Sig_Action,
                __a0: *const ACE_Sig_Set,
                __a1: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
                __a2: *mut __sigset_t,
                __a3: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Sig_Action, __a0, __a1, __a2, __a3)
    }
    pub unsafe fn new_u22c0e21ba8c3dfc5(
        mut __a0: *const ACE_Sig_Set,
        mut __a1: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
        mut __a2: *mut __sigset_t,
        mut __a3: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u22c0e21ba8c3dfc5(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
        );
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Sig_Action,
    ) -> *mut ACE_Sig_Action {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Sig_ActionaSERKS_"]
            fn __ext(
                __this: *mut ACE_Sig_Action,
                _anon_0: *const ACE_Sig_Action,
            ) -> *mut ACE_Sig_Action;
        }
        __ext(__this as *mut ACE_Sig_Action, _anon_0)
    }
    pub unsafe fn operator_assign_u0957da55363d9352(
        __this: *mut Self,
        mut _anon_0: *mut ACE_Sig_Action,
    ) -> *mut ACE_Sig_Action {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Sig_ActionaSEOS_"]
            fn __ext(
                __this: *mut ACE_Sig_Action,
                _anon_0: *mut ACE_Sig_Action,
            ) -> *mut ACE_Sig_Action;
        }
        __ext(__this as *mut ACE_Sig_Action, _anon_0)
    }
    /**Register @c this as the current disposition and store old
  /// disposition into @a oaction if it is non-NULL.*/
    pub unsafe fn register_action(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut oaction: *mut ACE_Sig_Action,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut sa: *mut sigaction = if (((((oaction).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    ((0) as *mut sigaction)
                } else {
                    <ACE_Sig_Action>::get((oaction) as *mut ACE_Sig_Action)
                };
                return ACE_OS::sigaction_uf038829092b02270(
                    signum,
                    (::core::ptr::addr_of_mut!((* __this).sa_) as *mut sigaction)
                        as *const sigaction,
                    ((sa) as *mut sigaction),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Assign the value of @a oaction to @c this and make it become the
  /// new signal disposition.*/
    pub unsafe fn restore_action(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut oaction: *mut ACE_Sig_Action,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __v = (*(<ACE_Sig_Action>::get(
                        (::core::ptr::addr_of_mut!((* oaction))) as *mut ACE_Sig_Action,
                    )));
                    let __asg_p = ::core::ptr::addr_of_mut!((* __this).sa_);
                    *__asg_p = __v;
                    __asg_p
                };
                return ACE_OS::sigaction_uf038829092b02270(
                    signum,
                    (::core::ptr::addr_of_mut!((* __this).sa_) as *mut sigaction)
                        as *const sigaction,
                    ((0) as *mut sigaction),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Retrieve the current disposition into @c this.
    pub unsafe fn retrieve_action(
        __this: *mut Self,
        mut signum: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::sigaction_uf038829092b02270(
                    signum,
                    ((0) as *const sigaction),
                    ((::core::ptr::addr_of_mut!((* __this).sa_) as *mut sigaction)
                        as *mut sigaction),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set current signal action.
    pub unsafe fn set(__this: *mut Self, mut sa: *mut sigaction) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __v = (*(sa));
                    let __asg_p = ::core::ptr::addr_of_mut!((* __this).sa_);
                    *__asg_p = __v;
                    __asg_p
                };
            }
            ()
        }
    }
    ///Get current signal action.
    pub unsafe fn get(__this: *mut Self) -> *mut sigaction {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of_mut!((* __this).sa_) as *mut sigaction;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn operator_sigaction__(__this: *mut Self) -> *mut sigaction {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of_mut!((* __this).sa_) as *mut sigaction;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set current signal flags.
    pub unsafe fn flags(__this: *mut Self, mut flags: libc::c_int) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).sa_.sa_flags = flags;
            }
            ()
        }
    }
    ///Get current signal flags.
    pub unsafe fn flags_uc3802ff705244b96(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).sa_.sa_flags;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set current signal mask.
    pub unsafe fn mask(__this: *mut Self, mut ss: *mut __sigset_t) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((!(ss).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    {
                        let __v = (*(ss));
                        let __asg_p = ::core::ptr::addr_of_mut!((* __this).sa_.sa_mask);
                        *__asg_p = __v;
                        __asg_p
                    };
                }
            }
            ()
        }
    }
    pub unsafe fn mask_u67ace4a0f02083d3(__this: *mut Self, mut ss: *mut ACE_Sig_Set) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __v = <ACE_Sig_Set>::sigset(
                        (::core::ptr::addr_of!((* ss))) as *const ACE_Sig_Set,
                    );
                    let __asg_p = ::core::ptr::addr_of_mut!((* __this).sa_.sa_mask);
                    *__asg_p = __v;
                    __asg_p
                };
            }
            ()
        }
    }
    ///Get current signal mask.
    pub unsafe fn mask_u44631ec3c629c47f(__this: *mut Self) -> *mut __sigset_t {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((::core::ptr::addr_of_mut!((* __this).sa_.sa_mask)
                    as *mut __sigset_t) as *mut __sigset_t);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set current signal handler (pointer to function).
    pub unsafe fn handler(
        __this: *mut Self,
        mut handler_arg: Option<unsafe extern "C-unwind" fn(libc::c_int)>,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).sa_.__sigaction_handler.sa_handler = unsafe {
                    ::core::mem::transmute::<
                        _,
                        Option<unsafe extern "C-unwind" fn(libc::c_int)>,
                    >(handler_arg)
                };
            }
            ()
        }
    }
    ///Get current signal handler (pointer to function).
    pub unsafe fn handler_u5d5a568d02c4ccdf(
        __this: *mut Self,
    ) -> Option<unsafe extern "C-unwind" fn(libc::c_int)> {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return unsafe {
                    ::core::mem::transmute::<
                        _,
                        Option<unsafe extern "C-unwind" fn(libc::c_int)>,
                    >((*__this).sa_.__sigaction_handler.sa_handler)
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Sig_Action4dumpEv"]
            fn __ext(__this: *const ACE_Sig_Action);
        }
        __ext(__this as *const ACE_Sig_Action)
    }
}
impl ACE_Sig_Guard {
    /**This is kind of conditional Guard, needed when guard should be
  /// activated only when a specific condition met. When condition ==
  /// true (default), Guard is activated*/
    pub unsafe fn new_at(
        __this: *mut Self,
        mut mask: *mut ACE_Sig_Set,
        mut condition: bool,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Sig_Set>::new_at_ud60a876b80356237(
                (::core::ptr::addr_of_mut!((* __this).omask_).cast::<ACE_Sig_Set>())
                    as *mut ACE_Sig_Set,
                0,
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).condition_),
                condition,
            );
            {
                if (((!((((*__this).condition_ as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    return;
                }
                if (((((mask).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    ACE_OS::thr_sigsetmask(
                        0,
                        (<ACE_OS_Object_Manager>::default_mask()) as *const __sigset_t,
                        (<ACE_Sig_Set>::operator___sigset_t__(
                            (::core::ptr::addr_of_mut!(
                                (* ::core::ptr::addr_of!((* __this).omask_) .cast:: <
                                ACE_Sig_Set > ().cast_mut())
                            )) as *mut ACE_Sig_Set,
                        ) as *mut __sigset_t),
                    );
                } else {
                    ACE_OS::thr_sigsetmask(
                        0,
                        ((<ACE_Sig_Set>::operator___sigset_t__(
                            (mask) as *mut ACE_Sig_Set,
                        ) as *mut __sigset_t)) as *const __sigset_t,
                        (<ACE_Sig_Set>::operator___sigset_t__(
                            (::core::ptr::addr_of_mut!(
                                (* ::core::ptr::addr_of!((* __this).omask_) .cast:: <
                                ACE_Sig_Set > ().cast_mut())
                            )) as *mut ACE_Sig_Set,
                        ) as *mut __sigset_t),
                    );
                }
            }
            ()
        }
    }
    pub unsafe fn new(mut __a0: *mut ACE_Sig_Set, mut __a1: bool) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_Sig_Guard4dumpEv"]
            fn __ext(__this: *const ACE_Sig_Guard);
        }
        __ext(__this as *const ACE_Sig_Guard)
    }
}
impl ACE_Trace {
    /**Perform the first part of the trace, which prints out the string
  /// N, the LINE, and the ACE_FILE as the function is entered.*/
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_int,
        mut __a2: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_TraceC1EPKciS1_"]
            fn __ext(
                __this: *mut ACE_Trace,
                __a0: *const libc::c_char,
                __a1: libc::c_int,
                __a2: *const libc::c_char,
            );
        }
        __ext(__this as *mut ACE_Trace, __a0, __a1, __a2)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_int,
        mut __a2: *const libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1, __a2);
        __obj
    }
    ///Determine if tracing is enabled or not
    pub unsafe fn is_tracing() -> bool {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_Trace10is_tracingEv"]
            fn __ext() -> bool;
        }
        __ext()
    }
    ///Enable the tracing facility.
    pub unsafe fn start_tracing() {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_Trace13start_tracingEv"]
            fn __ext();
        }
        __ext()
    }
    ///Disable the tracing facility.
    pub unsafe fn stop_tracing() {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_Trace12stop_tracingEv"]
            fn __ext();
        }
        __ext()
    }
    ///Change the nesting indentation level.
    pub unsafe fn set_nesting_indent(mut indent: libc::c_int) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_Trace18set_nesting_indentEi"]
            fn __ext(indent: libc::c_int);
        }
        __ext(indent)
    }
    ///Get the nesting indentation level.
    pub unsafe fn get_nesting_indent() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_Trace18get_nesting_indentEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK9ACE_Trace4dumpEv"]
            fn __ext(__this: *const ACE_Trace);
        }
        __ext(__this as *const ACE_Trace)
    }
}
impl ACE_Static_Object_Lock {
    ///Static lock access point.
    pub unsafe fn instance() -> *mut ACE_Recursive_Thread_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Static_Object_Lock8instanceEv"]
            fn __ext() -> *mut ACE_Recursive_Thread_Mutex;
        }
        __ext()
    }
    /**For use only by ACE_Object_Manager to clean up lock if it
  /// what dynamically allocated.*/
    pub unsafe fn cleanup_lock() {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Static_Object_Lock12cleanup_lockEv"]
            fn __ext();
        }
        __ext()
    }
}
impl ACE_HR_Time_Policy {
    ///Return the current time according to this policy
    pub unsafe fn operator_call(
        __this: *const Self,
    ) -> ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let __addrof_tmp_0 = <ACE_High_Res_Timer>::gettimeofday_hr();
                return (<ACE_Time_Value_T_ACE_HR_Time_Policy_>::new_s1cc0ae66d2e0ee48(
                    &__addrof_tmp_0 as *const ACE_Time_Value,
                    &(unsafe {
                        ::core::mem::MaybeUninit::<ACE_HR_Time_Policy>::zeroed()
                            .assume_init()
                    }) as *const ACE_HR_Time_Policy,
                ));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Noop. Just here to satisfy backwards compatibility demands.
    pub unsafe fn set_gettimeofday(
        __this: *mut Self,
        mut _anon_0: Option<unsafe extern "C-unwind" fn() -> ACE_Time_Value>,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {}
            ()
        }
    }
}
impl ACE_Time_Value_T_ACE_HR_Time_Policy_ {
    ///Default Constructor.
    pub unsafe fn new_at_s45a5d1d1a3b95282(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Time_Value>::new_at(
                ::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_Time_Value>(),
            );
            *(__this as *mut *const ()) = &__VTBL_u45a5d1d1a3b95282
                as *const __Vtbl_u45a5d1d1a3b95282 as *const ();
            {}
            ()
        }
    }
    pub unsafe fn new_s45a5d1d1a3b95282() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s45a5d1d1a3b95282(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Constructor.
    pub unsafe fn new_at_s57a0e694bb633ba2(
        __this: *mut Self,
        mut sec: libc::c_long,
        mut usec: libc::c_long,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Time_Value>::new_at_ub59bcc88eaedf2a6(
                (::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_Time_Value>())
                    as *mut ACE_Time_Value,
                sec,
                usec,
            );
            *(__this as *mut *const ()) = &__VTBL_u45a5d1d1a3b95282
                as *const __Vtbl_u45a5d1d1a3b95282 as *const ();
            {}
            ()
        }
    }
    pub unsafe fn new_s57a0e694bb633ba2(
        mut __a0: libc::c_long,
        mut __a1: libc::c_long,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s57a0e694bb633ba2(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Construct the ACE_Time_Value from a timeval.
    pub unsafe fn new_at_scf346aaab86c6e8a(__this: *mut Self, mut t: *const timeval) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Time_Value>::new_at_u0731a1c677ac70f5(
                (::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_Time_Value>())
                    as *mut ACE_Time_Value,
                ::core::ptr::addr_of!((* t)),
            );
            *(__this as *mut *const ()) = &__VTBL_u45a5d1d1a3b95282
                as *const __Vtbl_u45a5d1d1a3b95282 as *const ();
            {}
            ()
        }
    }
    pub unsafe fn new_scf346aaab86c6e8a(mut __a0: *const timeval) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_scf346aaab86c6e8a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Construct the ACE_Time_Value object from a timespec_t.
    pub unsafe fn new_at_sdfbb0bed66ce6a8c(__this: *mut Self, mut t: *const timespec) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Time_Value>::new_at_ua91b8b3861bcdc6f(
                (::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_Time_Value>())
                    as *mut ACE_Time_Value,
                ::core::ptr::addr_of!((* t)),
            );
            *(__this as *mut *const ()) = &__VTBL_u45a5d1d1a3b95282
                as *const __Vtbl_u45a5d1d1a3b95282 as *const ();
            {}
            ()
        }
    }
    pub unsafe fn new_sdfbb0bed66ce6a8c(mut __a0: *const timespec) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sdfbb0bed66ce6a8c(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Add @a tv to this.
    pub unsafe fn operator_add_assign_sa5a7d811a712c7b0(
        __this: *mut Self,
        mut tv: *const ACE_Time_Value,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEpLERK14ACE_Time_Value"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
                tv: *const ACE_Time_Value,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_, tv)
    }
    ///Add @a tv to this.
    pub unsafe fn operator_add_assign_s19f53dc453113eb7(
        __this: *mut Self,
        mut tv: libc::c_long,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEpLEl"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
                tv: libc::c_long,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_, tv)
    }
    ///Assign @ tv to this
    pub unsafe fn operator_assign_sf23995c34bfde059(
        __this: *mut Self,
        mut tv: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEaSERKS1_"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
                tv: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_, tv)
    }
    ///Assign @ tv to this
    pub unsafe fn operator_assign_sa5a7d811a712c7b0(
        __this: *mut Self,
        mut tv: *const ACE_Time_Value,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEaSERK14ACE_Time_Value"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
                tv: *const ACE_Time_Value,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_, tv)
    }
    ///Assign @ tv to this
    pub unsafe fn operator_assign_s19f53dc453113eb7(
        __this: *mut Self,
        mut tv: libc::c_long,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEaSEl"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
                tv: libc::c_long,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_, tv)
    }
    ///Subtract @a tv to this.
    pub unsafe fn operator_sub_assign_sa5a7d811a712c7b0(
        __this: *mut Self,
        mut tv: *const ACE_Time_Value,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEmIERK14ACE_Time_Value"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
                tv: *const ACE_Time_Value,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_, tv)
    }
    ///Subtract @a tv to this.
    pub unsafe fn operator_sub_assign_s19f53dc453113eb7(
        __this: *mut Self,
        mut tv: libc::c_long,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEmIEl"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
                tv: libc::c_long,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_, tv)
    }
    /**\brief Multiply the time value by the @a d factor.
    \note The result of the operator is valid for results from range
    < (ACE_INT32_MIN, -999999), (ACE_INT32_MAX, 999999) >. Result
    outside this range are saturated to a limit.*/
    pub unsafe fn operator_mul_assign(
        __this: *mut Self,
        mut d: libc::c_double,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEmLEd"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
                d: libc::c_double,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_, d)
    }
    #[doc = "Increment microseconds as prefix.\n  /**\n   * @note The only reason this is here is to allow the use of ACE_Atomic_Op\n   * with ACE_Time_Value."]
    pub unsafe fn operator_inc_s98a4019ba35042ed(
        __this: *mut Self,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEppEv"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_)
    }
    #[doc = "Decrement microseconds as prefix.\n  /**\n   * @note The only reason this is here is to allow the use of ACE_Atomic_Op\n   * with ACE_Time_Value."]
    pub unsafe fn operator_dec_s98a4019ba35042ed(
        __this: *mut Self,
    ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Time_Value_TI18ACE_HR_Time_PolicyEmmEv"]
            fn __ext(
                __this: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
            ) -> *mut ACE_Time_Value_T_ACE_HR_Time_Policy_;
        }
        __ext(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_)
    }
    ///Construct from ACE_Time_Value
    pub unsafe fn new_at_s1cc0ae66d2e0ee48(
        __this: *mut Self,
        mut tv: *const ACE_Time_Value,
        mut tp: *const ACE_HR_Time_Policy,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_Time_Value>(),
                ::core::ptr::read(::core::ptr::addr_of!(((* tv)))),
            );
            *(__this as *mut *const ()) = &__VTBL_u45a5d1d1a3b95282
                as *const __Vtbl_u45a5d1d1a3b95282 as *const ();
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).time_policy_),
                (*tp),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s1cc0ae66d2e0ee48(
        mut __a0: *const ACE_Time_Value,
        mut __a1: *const ACE_HR_Time_Policy,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s1cc0ae66d2e0ee48(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Construct from ACE_Time_Value_T<TIME_POLICY>
    pub unsafe fn new_at_s9f868f6e6111a33b(
        __this: *mut Self,
        mut tv: *const ACE_Time_Value_T_ACE_HR_Time_Policy_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_Time_Value>(),
                ::core::ptr::read(
                    ::core::ptr::addr_of!(((* tv)).__base_0).cast::<ACE_Time_Value>(),
                ),
            );
            *(__this as *mut *const ()) = &__VTBL_u45a5d1d1a3b95282
                as *const __Vtbl_u45a5d1d1a3b95282 as *const ();
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).time_policy_),
                (*tv).time_policy_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s9f868f6e6111a33b(
        mut __a0: *const ACE_Time_Value_T_ACE_HR_Time_Policy_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s9f868f6e6111a33b(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    #[doc = "Duplicates this time value (incl. time policy).\n  /**\n   * @return  Dynamically allocated time value copy.\n   *\n   * @note    The caller is responsible for freeing the copy when it's not needed\n   *          anymore."]
    pub unsafe fn duplicate(__this: *const Self) -> *mut ACE_Time_Value {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut tmp: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ = ((0)
                    as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_);
                'dowhile_0: loop {
                    'cont_0: loop {
                        {
                            {
                                tmp = {
                                    let __new: *mut ACE_Time_Value_T_ACE_HR_Time_Policy_ = Box::into_raw(
                                        Box::new(
                                            ::core::mem::MaybeUninit::<
                                                ACE_Time_Value_T_ACE_HR_Time_Policy_,
                                            >::zeroed()
                                                .assume_init(),
                                        ),
                                    );
                                    <ACE_Time_Value_T_ACE_HR_Time_Policy_>::new_at_s9f868f6e6111a33b(
                                        (__new) as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_,
                                        __this,
                                    );
                                    __new
                                };
                                if (((((tmp).is_null()) as libc::c_int) as libc::c_int)
                                    != 0)
                                {
                                    ((*(__errno_location()))) = 12;
                                    return ((0) as *mut ACE_Time_Value);
                                }
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                    if !(((0) != 0)) {
                        break 'dowhile_0;
                    }
                }
                return ((tmp) as *mut ACE_Time_Value);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "Get current time of day according to time policy.\n  /**\n   * @return  Time value representing current time of day according to time policy."]
    pub unsafe fn now(__this: *const Self) -> ACE_Time_Value {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Time_Value>::new_ud5d373334e09ec0f(
                    (&mut (<ACE_HR_Time_Policy>::operator_call(
                        (::core::ptr::addr_of!((* __this).time_policy_))
                            as *const ACE_HR_Time_Policy,
                    ))
                        .__base_0 as *mut ::core::mem::ManuallyDrop<ACE_Time_Value>)
                        .cast::<ACE_Time_Value>(),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "Converts relative time value to absolute time value based on current time of day.\n  /**\n   * @return  Absolute time value.\n   *\n   * @note    This method uses it's time_policy_ member to get the current\n   *          time of day.\n   *          The developer is responsible for making sure this is a relative\n   *          time value.\n   *          Note that the returned time value has no notion of the time policy\n   *          on which it is based anymore."]
    pub unsafe fn to_absolute_time(__this: *const Self) -> ACE_Time_Value {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return operator_add_u6912887df3d67ceb(
                    ::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((((* (__this)))).__base_0) .cast:: <
                        ACE_Time_Value > ().cast_mut())
                    ) as *const ACE_Time_Value,
                    (&(<ACE_HR_Time_Policy>::operator_call(
                        (::core::ptr::addr_of!((* __this).time_policy_))
                            as *const ACE_HR_Time_Policy,
                    ))
                        .__base_0 as *const ::core::mem::ManuallyDrop<ACE_Time_Value>)
                        .cast::<ACE_Time_Value>(),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "Converts absolute time value to time value relative to current time of day.\n  /**\n   * @return  Relative time value.\n   *\n   * @note    This method uses it's time_policy_ member to get the current\n   *          time of day.\n   *          The developer is responsible for making sure this is an absolute\n   *          time value compatible with the active time policy.\n   *          Note that the returned time value has no notion of the time policy\n   *          on which it is based anymore."]
    pub unsafe fn to_relative_time(__this: *const Self) -> ACE_Time_Value {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return operator_sub_u7824066738f89e79(
                    ::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((((* (__this)))).__base_0) .cast:: <
                        ACE_Time_Value > ().cast_mut())
                    ) as *const ACE_Time_Value,
                    (&(<ACE_HR_Time_Policy>::operator_call(
                        (::core::ptr::addr_of!((* __this).time_policy_))
                            as *const ACE_HR_Time_Policy,
                    ))
                        .__base_0 as *const ::core::mem::ManuallyDrop<ACE_Time_Value>)
                        .cast::<ACE_Time_Value>(),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_High_Res_Timer {
    #[doc = "* global_scale_factor_ is set to @a gsf.  All High_Res_Timers use\n   * global_scale_factor_.  This allows applications to set the scale\n   * factor just once for all High_Res_Timers.  Check\n   *  High_Res_Timer.cpp for the default global_scale_factors for\n   * several platforms.  For many platforms (e.g., Solaris), the\n   * global_scale_factor_ is set to 1000 so that <scale_factor> need\n   * not be set.  Careful, a <scale_factor> of 0 will cause division\n   * by zero exceptions.\n   * Depending on the platform its units are 1/microsecond or\n   * 1/millisecond. Use @c ACE_HR_SCALE_CONVERSION inside calculations\n   * instead a hardcoded value."]
    pub unsafe fn global_scale_factor(mut gsf: libc::c_uint) {
        unsafe {
            {
                ACE_High_Res_Timer_global_scale_factor_ = ((gsf) as libc::c_uint);
            }
            ()
        }
    }
    ///Returns the global_scale_factor.
    pub unsafe fn global_scale_factor_ubcde1b6c85276b84() -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_High_Res_Timer19global_scale_factorEv"]
            fn __ext() -> libc::c_uint;
        }
        __ext()
    }
    #[doc = "* Sets the global_scale_factor to the value in the @a env\n   * environment variable.  Returns 0 on success, -1 on failure.\n   * @note If @a env points to string \"0\" (value zero), this call will fail.\n   * This is basically a no-op on CE because there is no concept of\n   * environment variable on CE."]
    pub unsafe fn get_env_global_scale_factor(
        mut env: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_High_Res_Timer27get_env_global_scale_factorEPKc"]
            fn __ext(env: *const libc::c_char) -> libc::c_int;
        }
        __ext(env)
    }
    #[doc = "* Set (and return, for info) the global scale factor by sleeping\n   * for @a usec and counting the number of intervening clock cycles.\n   * Average over @a iterations of @a usec each.  On some platforms,\n   * such as Pentiums, this is called automatically during the first\n   * ACE_High_Res_Timer construction with the default parameter\n   * values.  An application can override that by calling calibrate\n   * with any desired parameter values _prior_ to constructing the\n   * first ACE_High_Res_Timer instance.\n   * Beware for platforms that can change the cycle rate on the fly."]
    pub unsafe fn calibrate(
        mut usec: libc::c_uint,
        mut iterations: libc::c_uint,
    ) -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_High_Res_Timer9calibrateEjj"]
            fn __ext(usec: libc::c_uint, iterations: libc::c_uint) -> libc::c_uint;
        }
        __ext(usec, iterations)
    }
    ///Initialize the timer.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_High_Res_TimerC1Ev"]
            fn __ext(__this: *mut ACE_High_Res_Timer);
        }
        __ext(__this as *mut ACE_High_Res_Timer)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    ///Reinitialize the timer.
    pub unsafe fn reset(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_High_Res_Timer5resetEv"]
            fn __ext(__this: *mut ACE_High_Res_Timer);
        }
        __ext(__this as *mut ACE_High_Res_Timer)
    }
    ///Start timing.
    pub unsafe fn start(__this: *mut Self, mut op: libc::c_uint) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).start_ = <ACE_High_Res_Timer>::gettime(op);
            }
            ()
        }
    }
    ///Stop timing.
    pub unsafe fn stop(__this: *mut Self, mut op: libc::c_uint) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).end_ = <ACE_High_Res_Timer>::gettime(op);
            }
            ()
        }
    }
    ///Set @a tv to the number of microseconds elapsed.
    pub unsafe fn elapsed_time(__this: *const Self, mut tv: *mut ACE_Time_Value) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_High_Res_Timer12elapsed_timeER14ACE_Time_Value"]
            fn __ext(__this: *const ACE_High_Res_Timer, tv: *mut ACE_Time_Value);
        }
        __ext(__this as *const ACE_High_Res_Timer, tv)
    }
    #[doc = "Set @a nanoseconds to the number of nanoseconds elapsed.\n  /**\n   *  Will overflow when measuring more than 194 day's."]
    pub unsafe fn elapsed_time_u0ae118857bf6b975(
        __this: *const Self,
        mut nanoseconds: *mut libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_High_Res_Timer12elapsed_timeERm"]
            fn __ext(__this: *const ACE_High_Res_Timer, nanoseconds: *mut libc::c_ulong);
        }
        __ext(__this as *const ACE_High_Res_Timer, nanoseconds)
    }
    /**Returns the elapsed (stop - start) time in a struct timespec
  /// (sec, nsec).*/
    pub unsafe fn elapsed_time_uefa7a55ba73008f6(
        __this: *const Self,
        mut _anon_0: *mut timespec,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_High_Res_Timer12elapsed_timeER8timespec"]
            fn __ext(__this: *const ACE_High_Res_Timer, _anon_0: *mut timespec);
        }
        __ext(__this as *const ACE_High_Res_Timer, _anon_0)
    }
    #[doc = "Sets @a usecs to the elapsed (stop - start) time in microseconds.\n  /**\n   *  Will overflow on windows when measuring more than appox. 2^^54 ticks.\n   *  Is still more than 48 days with a 4 Ghz counter."]
    pub unsafe fn elapsed_microseconds(
        __this: *const Self,
        mut usecs: *mut libc::c_ulong,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut elapsed: libc::c_ulong = <ACE_High_Res_Timer>::elapsed_hrtime(
                    (*__this).end_,
                    (*__this).start_,
                );
                (*usecs) = (((((elapsed) as libc::c_ulong))
                    / ((<ACE_High_Res_Timer>::global_scale_factor_ubcde1b6c85276b84())
                        as libc::c_ulong)) as libc::c_ulong);
            }
            ()
        }
    }
    ///Start incremental timing.
    pub unsafe fn start_incr(__this: *mut Self, mut op: libc::c_uint) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).start_incr_ = <ACE_High_Res_Timer>::gettime(op);
            }
            ()
        }
    }
    ///Stop incremental timing.
    pub unsafe fn stop_incr(__this: *mut Self, mut op: libc::c_uint) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __lv = ::core::ptr::addr_of_mut!((* __this).total_);
                    unsafe {
                        *__lv = ((((*__lv)) as libc::c_ulong))
                            .wrapping_add(
                                (<ACE_High_Res_Timer>::elapsed_hrtime(
                                    ((<ACE_High_Res_Timer>::gettime(op)) as libc::c_ulong),
                                    (((*__this).start_incr_) as libc::c_ulong),
                                )) as libc::c_ulong,
                            );
                        *__lv
                    }
                };
            }
            ()
        }
    }
    /**Set @a tv to the number of microseconds elapsed between all calls
  /// to start_incr and stop_incr.*/
    pub unsafe fn elapsed_time_incr(__this: *const Self, mut tv: *mut ACE_Time_Value) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_High_Res_Timer17elapsed_time_incrER14ACE_Time_Value"]
            fn __ext(__this: *const ACE_High_Res_Timer, tv: *mut ACE_Time_Value);
        }
        __ext(__this as *const ACE_High_Res_Timer, tv)
    }
    /**Set @a nanoseconds to the number of nanoseconds elapsed between all calls
  /// to start_incr and stop_incr.*/
    pub unsafe fn elapsed_time_incr_u71012f5f981a9568(
        __this: *const Self,
        mut nanoseconds: *mut libc::c_ulong,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_High_Res_Timer17elapsed_time_incrERm"]
            fn __ext(__this: *const ACE_High_Res_Timer, nanoseconds: *mut libc::c_ulong);
        }
        __ext(__this as *const ACE_High_Res_Timer, nanoseconds)
    }
    /**Print total time.
  /// @note only use @c print_total if incremental timings had been used!*/
    pub unsafe fn print_total(
        __this: *const Self,
        mut message: *const libc::c_char,
        mut iterations: libc::c_int,
        mut handle: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_High_Res_Timer11print_totalEPKcii"]
            fn __ext(
                __this: *const ACE_High_Res_Timer,
                message: *const libc::c_char,
                iterations: libc::c_int,
                handle: libc::c_int,
            );
        }
        __ext(__this as *const ACE_High_Res_Timer, message, iterations, handle)
    }
    ///Print average time.
    pub unsafe fn print_ave(
        __this: *const Self,
        mut message: *const libc::c_char,
        mut iterations: libc::c_int,
        mut handle: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_High_Res_Timer9print_aveEPKcii"]
            fn __ext(
                __this: *const ACE_High_Res_Timer,
                message: *const libc::c_char,
                iterations: libc::c_int,
                handle: libc::c_int,
            );
        }
        __ext(__this as *const ACE_High_Res_Timer, message, iterations, handle)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_High_Res_Timer4dumpEv"]
            fn __ext(__this: *const ACE_High_Res_Timer);
        }
        __ext(__this as *const ACE_High_Res_Timer)
    }
    #[doc = "* Get the current \"time\" as the high resolution counter at this time.\n   * This is intended to be useful for supplying to a ACE_Timer_Queue\n   * as the gettimeofday function, thereby basing the timer calculations\n   * on the high res timer rather than wall clock time."]
    pub unsafe fn gettimeofday_hr() -> ACE_Time_Value {
        unsafe {
            {
                return <ACE_High_Res_Timer>::gettimeofday(
                    ((ACE_OS::ACE_HRTIMER_GETTIME) as libc::c_uint),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* @deprecated THIS FUNCTION IS DEPRECATED.  PLEASE USE\n   * <ACE_OS::gettimeofday> INSTEAD!  Calls <ACE_High_Res_Timer::hrtime_to_tv>\n   * passing <ACE_OS::gethrtime>.  This function can be used to parameterize\n   * objects such as <ACE_Timer_Queue::gettimeofday>.  If\n   * <global_scale_factor_> is not set, and we're on a platform that\n   * requires <global_scale_factor_> (e.g., Win32),\n   * ACE_OS::gettimeofday will be used instead of <ACE_OS::gethrtime>.\n   * This allows applications on Intel to use <High_Res_Timer> even\n   * when <global_scale_factor> is not set.  However, setting the\n   * <global_scale_factor_> appropriately will result in the finest\n   * resolution possible."]
    pub unsafe fn gettimeofday(mut op: libc::c_uint) -> ACE_Time_Value {
        unsafe {
            {
                let mut tv: ACE_Time_Value = ::core::mem::MaybeUninit::<
                    ACE_Time_Value,
                >::zeroed()
                    .assume_init();
                <ACE_Time_Value>::new_at(
                    (::core::ptr::addr_of_mut!(tv)) as *mut ACE_Time_Value,
                );
                <ACE_High_Res_Timer>::hrtime_to_tv(
                    ::core::ptr::addr_of_mut!(tv),
                    ((ACE_OS::gethrtime(((op) as libc::c_uint))) as libc::c_ulong),
                );
                return <ACE_Time_Value>::new_ud5d373334e09ec0f(
                    ::core::ptr::addr_of_mut!(tv),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Converts an @a hrt to @a tv using global_scale_factor_.
    pub unsafe fn hrtime_to_tv(mut tv: *mut ACE_Time_Value, mut hrt: libc::c_ulong) {
        unsafe {
            {
                <ACE_Time_Value>::sec_u74028f0493c718a1(
                    (::core::ptr::addr_of_mut!((* tv))) as *mut ACE_Time_Value,
                    ((((((((hrt) as libc::c_ulong))
                        / ((((ACE_ONE_SECOND_IN_USECS) as libc::c_uint))
                            as libc::c_ulong)) as libc::c_ulong))
                        / ((<ACE_High_Res_Timer>::global_scale_factor_ubcde1b6c85276b84())
                            as libc::c_ulong)) as libc::c_long),
                );
                let mut tmp: libc::c_ulong = (<ACE_Time_Value>::sec(
                    (::core::ptr::addr_of!((* tv))) as *const ACE_Time_Value,
                ) as libc::c_ulong);
                {
                    tmp = (((tmp) as libc::c_ulong))
                        .wrapping_mul(
                            (((((((ACE_ONE_SECOND_IN_USECS) as libc::c_uint))
                                as libc::c_uint))
                                .wrapping_mul(
                                    (<ACE_High_Res_Timer>::global_scale_factor_ubcde1b6c85276b84())
                                        as libc::c_uint,
                                ))) as libc::c_ulong,
                        );
                    tmp
                };
                <ACE_Time_Value>::usec_u12bf25f2e86a1ec6(
                    (::core::ptr::addr_of_mut!((* tv))) as *mut ACE_Time_Value,
                    (((((((((hrt) as libc::c_ulong))
                        .wrapping_sub((tmp) as libc::c_ulong))) as libc::c_ulong))
                        / ((<ACE_High_Res_Timer>::global_scale_factor_ubcde1b6c85276b84())
                            as libc::c_ulong)) as libc::c_long),
                );
            }
            ()
        }
    }
    #[doc = "* This is used to find out the Mhz of the machine for the scale\n   * factor.  If there are any problems getting it, we just return 1\n   * (the default)."]
    pub unsafe fn get_cpuinfo() -> libc::c_uint {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_High_Res_Timer11get_cpuinfoEv"]
            fn __ext() -> libc::c_uint;
        }
        __ext()
    }
    #[doc = "* For internal use: gets the high-resolution time using\n   * <ACE_OS::gethrtime>.  Except on platforms that require that the\n   * <global_scale_factor_> be set, such as ACE_WIN32, uses the\n   * low-resolution clock if the <global_scale_factor_> has not been\n   * set."]
    pub unsafe fn gettime(mut op: libc::c_uint) -> libc::c_ulong {
        unsafe {
            {
                return ACE_OS::gethrtime(((op) as libc::c_uint));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Calculate the difference between two ACE_hrtime_t values. It is assumed
  /// that the end time is later than start time, so if end is a smaller
  /// value, the time counter has wrapped around.*/
    pub unsafe fn elapsed_hrtime(
        mut end: libc::c_ulong,
        mut start: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            {
                if (((((end as libc::c_ulong)) > (((start) as libc::c_ulong)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (((end) as libc::c_ulong))
                        .wrapping_sub((start) as libc::c_ulong);
                }
                return ((((((((!(start))) as libc::c_ulong))
                    .wrapping_add((1) as libc::c_ulong)) as libc::c_ulong))
                    .wrapping_add((end) as libc::c_ulong));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Guard_ACE_Recursive_Thread_Mutex_ {
    /**Implicitly and automatically acquire (or try to acquire) the
  /// lock.  If @a block is non-0 then acquire() the ACE_LOCK, else
  /// tryacquire() it.*/
    pub unsafe fn new_at_sdae87dd38da0bfc6(
        __this: *mut Self,
        mut __a0: *mut ACE_Recursive_Thread_Mutex,
        mut __a1: bool,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI26ACE_Recursive_Thread_MutexEC1ERS0_b"]
            fn __ext(
                __this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                __a0: *mut ACE_Recursive_Thread_Mutex,
                __a1: bool,
            );
        }
        __ext(__this as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_, __a0, __a1)
    }
    pub unsafe fn new_sdae87dd38da0bfc6(
        mut __a0: *mut ACE_Recursive_Thread_Mutex,
        mut __a1: bool,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sdae87dd38da0bfc6(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Initialize the guard without implicitly acquiring the lock. The
  /// @a become_owner parameter indicates whether the guard should release
  /// the lock implicitly on destruction. The @a block parameter is
  /// ignored and is used here to disambiguate with the preceding
  /// constructor.*/
    pub unsafe fn new_at_sc3c1c82380cf9ca3(
        __this: *mut Self,
        mut __a0: *mut ACE_Recursive_Thread_Mutex,
        mut __a1: bool,
        mut __a2: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI26ACE_Recursive_Thread_MutexEC1ERS0_bi"]
            fn __ext(
                __this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                __a0: *mut ACE_Recursive_Thread_Mutex,
                __a1: bool,
                __a2: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_, __a0, __a1, __a2)
    }
    pub unsafe fn new_sc3c1c82380cf9ca3(
        mut __a0: *mut ACE_Recursive_Thread_Mutex,
        mut __a1: bool,
        mut __a2: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sc3c1c82380cf9ca3(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    ///Conditionally acquire the lock (i.e., won't block).
    pub unsafe fn tryacquire(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI26ACE_Recursive_Thread_MutexE10tryacquireEv"]
            fn __ext(__this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_)
    }
    /**Relinquish ownership of the lock so that it is not released
  /// implicitly in the destructor.*/
    pub unsafe fn disown(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI26ACE_Recursive_Thread_MutexE6disownEv"]
            fn __ext(__this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_);
        }
        __ext(__this as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_)
    }
    ///Explicitly remove the lock.
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI26ACE_Recursive_Thread_MutexE6removeEv"]
            fn __ext(__this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK9ACE_GuardI26ACE_Recursive_Thread_MutexE4dumpEv"]
            fn __ext(__this: *const ACE_Guard_ACE_Recursive_Thread_Mutex_);
        }
        __ext(__this as *const ACE_Guard_ACE_Recursive_Thread_Mutex_)
    }
    ///Helper, meant for subclass only.
    pub unsafe fn new_at_s32ddbf42ad61ad7d(
        __this: *mut Self,
        mut lock: *mut ACE_Recursive_Thread_Mutex,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).lock_),
                ((lock) as *mut ACE_Recursive_Thread_Mutex),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).owner_), 0);
            {}
            ()
        }
    }
    pub unsafe fn new_s32ddbf42ad61ad7d(
        mut __a0: *mut ACE_Recursive_Thread_Mutex,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s32ddbf42ad61ad7d(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI26ACE_Recursive_Thread_MutexEaSERKS1_"]
            fn __ext(
                __this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                _anon_0: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
            );
        }
        __ext(__this as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_, _anon_0)
    }
    pub unsafe fn new_at_s1d2389309febbef3(
        __this: *mut Self,
        mut __a0: *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI26ACE_Recursive_Thread_MutexEC1ERKS1_"]
            fn __ext(
                __this: *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                __a0: *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
            );
        }
        __ext(__this as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_, __a0)
    }
    pub unsafe fn new_s1d2389309febbef3(
        mut __a0: *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s1d2389309febbef3(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Explicitly acquire the lock.
    pub unsafe fn acquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __v = <ACE_Recursive_Thread_Mutex>::acquire(
                        ((*__this).lock_) as *mut ACE_Recursive_Thread_Mutex,
                    );
                    (*__this).owner_ = __v;
                    __v
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn new_at_sd3d970b1b01b243f(
        __this: *mut Self,
        mut l: *mut ACE_Recursive_Thread_Mutex,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).lock_),
                ::core::ptr::addr_of_mut!((* l)) as *mut ACE_Recursive_Thread_Mutex,
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).owner_), 0);
            {
                <ACE_Guard_ACE_Recursive_Thread_Mutex_>::acquire(
                    (__this) as *mut ACE_Guard_ACE_Recursive_Thread_Mutex_,
                );
            }
            ()
        }
    }
    pub unsafe fn new_sd3d970b1b01b243f(
        mut __a0: *mut ACE_Recursive_Thread_Mutex,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sd3d970b1b01b243f(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    /**true if locked, false if couldn't acquire the lock
  /// (errno will contain the reason for this).*/
    pub unsafe fn locked(__this: *const Self) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((((((*__this).owner_ as libc::c_int))
                    != ((((-((1) as libc::c_int))) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Explicitly release the lock, but only if it is held!
    pub unsafe fn release(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).owner_ as libc::c_int))
                    == ((((-((1) as libc::c_int))) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                } else {
                    (*__this).owner_ = (-((1) as libc::c_int));
                    return <ACE_Recursive_Thread_Mutex>::release(
                        ((*__this).lock_) as *mut ACE_Recursive_Thread_Mutex,
                    );
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
pub unsafe fn __vdtor_u278a8a4a3a17abbc(__this: *mut ACE_Object_Manager_Base) {
    let _ = Box::from_raw(__this as *mut ACE_Object_Manager);
}
pub unsafe fn __vthunk_ouff3e4d6aa5350b05_iu2ae16bb83518d3fd(
    __this: *mut ACE_Object_Manager_Base,
) -> libc::c_int {
    <ACE_Object_Manager>::init((__this as *mut ACE_Object_Manager))
}
pub unsafe fn __vthunk_ou0acc2efdb74b928b_iue800bfd5a4e8ffa3(
    __this: *mut ACE_Object_Manager_Base,
) -> libc::c_int {
    <ACE_Object_Manager>::fini((__this as *mut ACE_Object_Manager))
}
pub static __VTBL_u278a8a4a3a17abbc: __Vtbl_u278a8a4a3a17abbc = __Vtbl_u278a8a4a3a17abbc {
    __type_info: &__TYPEINFO_18ACE_Object_Manager,
    __vdtor: __vdtor_u278a8a4a3a17abbc,
    vfn_u2ae16bb83518d3fd: __vthunk_ouff3e4d6aa5350b05_iu2ae16bb83518d3fd,
    vfn_ue800bfd5a4e8ffa3: __vthunk_ou0acc2efdb74b928b_iue800bfd5a4e8ffa3,
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
pub unsafe fn __vdtor_u75264ee698c7f168(__this: *mut ACE_Process) {
    let _ = Box::from_raw(__this as *mut ACE_Process);
}
pub unsafe fn __vthunk_ou081801663cb7e0ca_iu081801663cb7e0ca(
    __this: *mut ACE_Process,
    p0: *mut ACE_Process_Options,
) -> libc::c_int {
    <ACE_Process>::prepare((__this as *mut ACE_Process), p0)
}
pub unsafe fn __vthunk_ou3bdf0a03b95f6b5a_iu3bdf0a03b95f6b5a(
    __this: *mut ACE_Process,
    p0: *mut ACE_Process_Options,
) -> libc::c_int {
    <ACE_Process>::spawn((__this as *mut ACE_Process), p0)
}
pub unsafe fn __vthunk_ouecaa3acc181bab5b_iuecaa3acc181bab5b(
    __this: *mut ACE_Process,
    p0: libc::c_int,
) {
    <ACE_Process>::parent((__this as *mut ACE_Process), p0)
}
pub unsafe fn __vthunk_oudbf99b92792d94dd_iudbf99b92792d94dd(
    __this: *mut ACE_Process,
    p0: libc::c_int,
) {
    <ACE_Process>::child((__this as *mut ACE_Process), p0)
}
pub unsafe fn __vthunk_ou9963a2b3ddfce4c7_iu9963a2b3ddfce4c7(__this: *mut ACE_Process) {
    <ACE_Process>::unmanage((__this as *mut ACE_Process))
}
#[repr(C)]
pub struct __Vtbl_u75264ee698c7f168 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Process),
    pub vfn_u081801663cb7e0ca: unsafe fn(
        *mut ACE_Process,
        *mut ACE_Process_Options,
    ) -> libc::c_int,
    pub vfn_u3bdf0a03b95f6b5a: unsafe fn(
        *mut ACE_Process,
        *mut ACE_Process_Options,
    ) -> libc::c_int,
    pub vfn_uecaa3acc181bab5b: unsafe fn(*mut ACE_Process, libc::c_int),
    pub vfn_udbf99b92792d94dd: unsafe fn(*mut ACE_Process, libc::c_int),
    pub vfn_u9963a2b3ddfce4c7: unsafe fn(*mut ACE_Process),
}
pub static __VTBL_u75264ee698c7f168: __Vtbl_u75264ee698c7f168 = __Vtbl_u75264ee698c7f168 {
    __type_info: &__TYPEINFO_11ACE_Process,
    __vdtor: __vdtor_u75264ee698c7f168,
    vfn_u081801663cb7e0ca: __vthunk_ou081801663cb7e0ca_iu081801663cb7e0ca,
    vfn_u3bdf0a03b95f6b5a: __vthunk_ou3bdf0a03b95f6b5a_iu3bdf0a03b95f6b5a,
    vfn_uecaa3acc181bab5b: __vthunk_ouecaa3acc181bab5b_iuecaa3acc181bab5b,
    vfn_udbf99b92792d94dd: __vthunk_oudbf99b92792d94dd_iudbf99b92792d94dd,
    vfn_u9963a2b3ddfce4c7: __vthunk_ou9963a2b3ddfce4c7_iu9963a2b3ddfce4c7,
};
pub unsafe fn __vdtor_u5baae7ee749c5722(__this: *mut ACE_Process) {
    let _ = Box::from_raw(__this as *mut ACE_Managed_Process);
}
pub unsafe fn __vthunk_oud2cc3f6007495957_iu9963a2b3ddfce4c7(__this: *mut ACE_Process) {
    <ACE_Managed_Process>::unmanage((__this as *mut ACE_Managed_Process))
}
#[repr(C)]
pub struct __Vtbl_u5baae7ee749c5722 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Process),
    pub vfn_u081801663cb7e0ca: unsafe fn(
        *mut ACE_Process,
        *mut ACE_Process_Options,
    ) -> libc::c_int,
    pub vfn_u3bdf0a03b95f6b5a: unsafe fn(
        *mut ACE_Process,
        *mut ACE_Process_Options,
    ) -> libc::c_int,
    pub vfn_uecaa3acc181bab5b: unsafe fn(*mut ACE_Process, libc::c_int),
    pub vfn_udbf99b92792d94dd: unsafe fn(*mut ACE_Process, libc::c_int),
    pub vfn_u9963a2b3ddfce4c7: unsafe fn(*mut ACE_Process),
}
pub static __VTBL_u5baae7ee749c5722: __Vtbl_u5baae7ee749c5722 = __Vtbl_u5baae7ee749c5722 {
    __type_info: &__TYPEINFO_19ACE_Managed_Process,
    __vdtor: __vdtor_u5baae7ee749c5722,
    vfn_u081801663cb7e0ca: __vthunk_ou081801663cb7e0ca_iu081801663cb7e0ca,
    vfn_u3bdf0a03b95f6b5a: __vthunk_ou3bdf0a03b95f6b5a_iu3bdf0a03b95f6b5a,
    vfn_uecaa3acc181bab5b: __vthunk_ouecaa3acc181bab5b_iuecaa3acc181bab5b,
    vfn_udbf99b92792d94dd: __vthunk_oudbf99b92792d94dd_iudbf99b92792d94dd,
    vfn_u9963a2b3ddfce4c7: __vthunk_oud2cc3f6007495957_iu9963a2b3ddfce4c7,
};
pub unsafe fn __vdtor_uce092e161042e5b3(__this: *mut ACE_Reactor_Timer_Interface) {
    let _ = Box::from_raw(__this as *mut ACE_Reactor);
}
pub unsafe fn __vthunk_ou9e884774820851e3_iu16610216518d6d87(
    __this: *mut ACE_Reactor_Timer_Interface,
    p0: *mut ACE_Event_Handler,
    p1: *const libc::c_void,
    p2: *const ACE_Time_Value,
    p3: *const ACE_Time_Value,
) -> libc::c_long {
    <ACE_Reactor>::schedule_timer((__this as *mut ACE_Reactor), p0, p1, p2, p3)
}
pub unsafe fn __vthunk_ouc7eae581205cb60d_iubbee5f06b8f63cb9(
    __this: *mut ACE_Reactor_Timer_Interface,
    p0: libc::c_long,
    p1: *const ACE_Time_Value,
) -> libc::c_int {
    <ACE_Reactor>::reset_timer_interval((__this as *mut ACE_Reactor), p0, p1)
}
pub unsafe fn __vthunk_ou14367c2f561665d8_iuac147cf795d58ccc(
    __this: *mut ACE_Reactor_Timer_Interface,
    p0: libc::c_long,
    p1: *mut *const libc::c_void,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_Reactor>::cancel_timer((__this as *mut ACE_Reactor), p0, p1, p2)
}
pub unsafe fn __vthunk_oue8bc061cd761dc66_iu5d76921f62203fe2(
    __this: *mut ACE_Reactor_Timer_Interface,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_Reactor>::cancel_timer_ue8bc061cd761dc66((__this as *mut ACE_Reactor), p0, p1)
}
#[repr(C)]
pub struct __Vtbl_uce092e161042e5b3 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Reactor_Timer_Interface),
    pub vfn_u16610216518d6d87: unsafe fn(
        *mut ACE_Reactor_Timer_Interface,
        *mut ACE_Event_Handler,
        *const libc::c_void,
        *const ACE_Time_Value,
        *const ACE_Time_Value,
    ) -> libc::c_long,
    pub vfn_ubbee5f06b8f63cb9: unsafe fn(
        *mut ACE_Reactor_Timer_Interface,
        libc::c_long,
        *const ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_uac147cf795d58ccc: unsafe fn(
        *mut ACE_Reactor_Timer_Interface,
        libc::c_long,
        *mut *const libc::c_void,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u5d76921f62203fe2: unsafe fn(
        *mut ACE_Reactor_Timer_Interface,
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
}
pub static __VTBL_uce092e161042e5b3: __Vtbl_uce092e161042e5b3 = __Vtbl_uce092e161042e5b3 {
    __type_info: &__TYPEINFO_11ACE_Reactor,
    __vdtor: __vdtor_uce092e161042e5b3,
    vfn_u16610216518d6d87: __vthunk_ou9e884774820851e3_iu16610216518d6d87,
    vfn_ubbee5f06b8f63cb9: __vthunk_ouc7eae581205cb60d_iubbee5f06b8f63cb9,
    vfn_uac147cf795d58ccc: __vthunk_ou14367c2f561665d8_iuac147cf795d58ccc,
    vfn_u5d76921f62203fe2: __vthunk_oue8bc061cd761dc66_iu5d76921f62203fe2,
};
pub unsafe fn __vthunk_ou16610216518d6d87_iu16610216518d6d87(
    __this: *mut ACE_Reactor_Timer_Interface,
    p0: *mut ACE_Event_Handler,
    p1: *const libc::c_void,
    p2: *const ACE_Time_Value,
    p3: *const ACE_Time_Value,
) -> libc::c_long {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou5d76921f62203fe2_iu5d76921f62203fe2(
    __this: *mut ACE_Reactor_Timer_Interface,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_u093ccbc4c8f491d5 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Reactor_Timer_Interface),
    pub vfn_u16610216518d6d87: unsafe fn(
        *mut ACE_Reactor_Timer_Interface,
        *mut ACE_Event_Handler,
        *const libc::c_void,
        *const ACE_Time_Value,
        *const ACE_Time_Value,
    ) -> libc::c_long,
    pub vfn_ubbee5f06b8f63cb9: unsafe fn(
        *mut ACE_Reactor_Timer_Interface,
        libc::c_long,
        *const ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_uac147cf795d58ccc: unsafe fn(
        *mut ACE_Reactor_Timer_Interface,
        libc::c_long,
        *mut *const libc::c_void,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u5d76921f62203fe2: unsafe fn(
        *mut ACE_Reactor_Timer_Interface,
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
}
pub static __VTBL_u093ccbc4c8f491d5: __Vtbl_u093ccbc4c8f491d5 = __Vtbl_u093ccbc4c8f491d5 {
    __type_info: &__TYPEINFO_27ACE_Reactor_Timer_Interface,
    __vdtor: __vdtor_u093ccbc4c8f491d5,
    vfn_u16610216518d6d87: __vthunk_ou16610216518d6d87_iu16610216518d6d87,
    vfn_ubbee5f06b8f63cb9: __vthunk_oubbee5f06b8f63cb9_iubbee5f06b8f63cb9,
    vfn_uac147cf795d58ccc: __vthunk_ouac147cf795d58ccc_iuac147cf795d58ccc,
    vfn_u5d76921f62203fe2: __vthunk_ou5d76921f62203fe2_iu5d76921f62203fe2,
};
pub unsafe fn __vdtor_ud05ba0669c8bb3a2(__this: *mut ACE_Event_Handler) {
    let _ = Box::from_raw(__this as *mut ACE_Event_Handler);
}
pub unsafe fn __vthunk_ou9c73e06ba19a4821_iu9c73e06ba19a4821(
    __this: *mut ACE_Event_Handler,
) -> libc::c_int {
    <ACE_Event_Handler>::get_handle((__this as *mut ACE_Event_Handler))
}
pub unsafe fn __vthunk_ou7d36445ae9d974fa_iu7d36445ae9d974fa(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
) {
    <ACE_Event_Handler>::set_handle((__this as *mut ACE_Event_Handler), p0)
}
pub unsafe fn __vthunk_ou5cd5a5fc8acd0342_iu5cd5a5fc8acd0342(
    __this: *mut ACE_Event_Handler,
) -> libc::c_int {
    <ACE_Event_Handler>::priority((__this as *mut ACE_Event_Handler))
}
pub unsafe fn __vthunk_ou8c5dfd1fdaa209ab_iu8c5dfd1fdaa209ab(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
) {
    <ACE_Event_Handler>::priority_u8c5dfd1fdaa209ab(
        (__this as *mut ACE_Event_Handler),
        p0,
    )
}
pub unsafe fn __vthunk_ou3ac298578aa09514_iu3ac298578aa09514(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_Event_Handler>::handle_input((__this as *mut ACE_Event_Handler), p0)
}
pub unsafe fn __vthunk_ouafe15b49ae15a941_iuafe15b49ae15a941(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_Event_Handler>::handle_output((__this as *mut ACE_Event_Handler), p0)
}
pub unsafe fn __vthunk_oua4412b5a365fe817_iua4412b5a365fe817(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_Event_Handler>::handle_exception((__this as *mut ACE_Event_Handler), p0)
}
pub unsafe fn __vthunk_ou0604b3232f6ba27b_iu0604b3232f6ba27b(
    __this: *mut ACE_Event_Handler,
    p0: *const ACE_Time_Value,
    p1: *const libc::c_void,
) -> libc::c_int {
    <ACE_Event_Handler>::handle_timeout((__this as *mut ACE_Event_Handler), p0, p1)
}
pub unsafe fn __vthunk_ou140f83bfe0d335d9_iu140f83bfe0d335d9(
    __this: *mut ACE_Event_Handler,
    p0: *mut ACE_Process,
) -> libc::c_int {
    <ACE_Event_Handler>::handle_exit((__this as *mut ACE_Event_Handler), p0)
}
pub unsafe fn __vthunk_oua1174916b4160f19_iua1174916b4160f19(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
    p1: libc::c_ulong,
) -> libc::c_int {
    <ACE_Event_Handler>::handle_close((__this as *mut ACE_Event_Handler), p0, p1)
}
pub unsafe fn __vthunk_ou3f872401b3199422_iu3f872401b3199422(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
    p1: *mut siginfo_t,
    p2: *mut ucontext_t,
) -> libc::c_int {
    <ACE_Event_Handler>::handle_signal((__this as *mut ACE_Event_Handler), p0, p1, p2)
}
pub unsafe fn __vthunk_ouc909131ec77881c7_iuc909131ec77881c7(
    __this: *mut ACE_Event_Handler,
) -> libc::c_int {
    <ACE_Event_Handler>::resume_handler((__this as *mut ACE_Event_Handler))
}
pub unsafe fn __vthunk_ouf69856450f20fddf_iuf69856450f20fddf(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_Event_Handler>::handle_qos((__this as *mut ACE_Event_Handler), p0)
}
pub unsafe fn __vthunk_ou366f25d08aaa768b_iu366f25d08aaa768b(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_Event_Handler>::handle_group_qos((__this as *mut ACE_Event_Handler), p0)
}
pub unsafe fn __vthunk_ou5cb133098a439c05_iu5cb133098a439c05(
    __this: *mut ACE_Event_Handler,
    p0: *mut ACE_Reactor,
) {
    <ACE_Event_Handler>::reactor((__this as *mut ACE_Event_Handler), p0)
}
pub unsafe fn __vthunk_oub75675325861507c_iub75675325861507c(
    __this: *mut ACE_Event_Handler,
) -> *mut ACE_Reactor {
    <ACE_Event_Handler>::reactor_ub75675325861507c((__this as *mut ACE_Event_Handler))
}
pub unsafe fn __vthunk_ou8ad488637ad422a6_iu8ad488637ad422a6(
    __this: *mut ACE_Event_Handler,
) -> *mut ACE_Reactor_Timer_Interface {
    <ACE_Event_Handler>::reactor_timer_interface((__this as *mut ACE_Event_Handler))
}
pub unsafe fn __vthunk_ou1c00fc150b810f94_iu1c00fc150b810f94(
    __this: *mut ACE_Event_Handler,
) -> libc::c_long {
    <ACE_Event_Handler>::add_reference((__this as *mut ACE_Event_Handler))
}
pub unsafe fn __vthunk_ou590ad03b86fd5891_iu590ad03b86fd5891(
    __this: *mut ACE_Event_Handler,
) -> libc::c_long {
    <ACE_Event_Handler>::remove_reference((__this as *mut ACE_Event_Handler))
}
#[repr(C)]
pub struct __Vtbl_ud05ba0669c8bb3a2 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Event_Handler),
    pub vfn_u9c73e06ba19a4821: unsafe fn(*mut ACE_Event_Handler) -> libc::c_int,
    pub vfn_u7d36445ae9d974fa: unsafe fn(*mut ACE_Event_Handler, libc::c_int),
    pub vfn_u5cd5a5fc8acd0342: unsafe fn(*mut ACE_Event_Handler) -> libc::c_int,
    pub vfn_u8c5dfd1fdaa209ab: unsafe fn(*mut ACE_Event_Handler, libc::c_int),
    pub vfn_u3ac298578aa09514: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uafe15b49ae15a941: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_ua4412b5a365fe817: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u0604b3232f6ba27b: unsafe fn(
        *mut ACE_Event_Handler,
        *const ACE_Time_Value,
        *const libc::c_void,
    ) -> libc::c_int,
    pub vfn_u140f83bfe0d335d9: unsafe fn(
        *mut ACE_Event_Handler,
        *mut ACE_Process,
    ) -> libc::c_int,
    pub vfn_ua1174916b4160f19: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u3f872401b3199422: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
        *mut siginfo_t,
        *mut ucontext_t,
    ) -> libc::c_int,
    pub vfn_uc909131ec77881c7: unsafe fn(*mut ACE_Event_Handler) -> libc::c_int,
    pub vfn_uf69856450f20fddf: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u366f25d08aaa768b: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u5cb133098a439c05: unsafe fn(*mut ACE_Event_Handler, *mut ACE_Reactor),
    pub vfn_ub75675325861507c: unsafe fn(*mut ACE_Event_Handler) -> *mut ACE_Reactor,
    pub vfn_u8ad488637ad422a6: unsafe fn(
        *mut ACE_Event_Handler,
    ) -> *mut ACE_Reactor_Timer_Interface,
    pub vfn_u1c00fc150b810f94: unsafe fn(*mut ACE_Event_Handler) -> libc::c_long,
    pub vfn_u590ad03b86fd5891: unsafe fn(*mut ACE_Event_Handler) -> libc::c_long,
}
pub static __VTBL_ud05ba0669c8bb3a2: __Vtbl_ud05ba0669c8bb3a2 = __Vtbl_ud05ba0669c8bb3a2 {
    __type_info: &__TYPEINFO_17ACE_Event_Handler,
    __vdtor: __vdtor_ud05ba0669c8bb3a2,
    vfn_u9c73e06ba19a4821: __vthunk_ou9c73e06ba19a4821_iu9c73e06ba19a4821,
    vfn_u7d36445ae9d974fa: __vthunk_ou7d36445ae9d974fa_iu7d36445ae9d974fa,
    vfn_u5cd5a5fc8acd0342: __vthunk_ou5cd5a5fc8acd0342_iu5cd5a5fc8acd0342,
    vfn_u8c5dfd1fdaa209ab: __vthunk_ou8c5dfd1fdaa209ab_iu8c5dfd1fdaa209ab,
    vfn_u3ac298578aa09514: __vthunk_ou3ac298578aa09514_iu3ac298578aa09514,
    vfn_uafe15b49ae15a941: __vthunk_ouafe15b49ae15a941_iuafe15b49ae15a941,
    vfn_ua4412b5a365fe817: __vthunk_oua4412b5a365fe817_iua4412b5a365fe817,
    vfn_u0604b3232f6ba27b: __vthunk_ou0604b3232f6ba27b_iu0604b3232f6ba27b,
    vfn_u140f83bfe0d335d9: __vthunk_ou140f83bfe0d335d9_iu140f83bfe0d335d9,
    vfn_ua1174916b4160f19: __vthunk_oua1174916b4160f19_iua1174916b4160f19,
    vfn_u3f872401b3199422: __vthunk_ou3f872401b3199422_iu3f872401b3199422,
    vfn_uc909131ec77881c7: __vthunk_ouc909131ec77881c7_iuc909131ec77881c7,
    vfn_uf69856450f20fddf: __vthunk_ouf69856450f20fddf_iuf69856450f20fddf,
    vfn_u366f25d08aaa768b: __vthunk_ou366f25d08aaa768b_iu366f25d08aaa768b,
    vfn_u5cb133098a439c05: __vthunk_ou5cb133098a439c05_iu5cb133098a439c05,
    vfn_ub75675325861507c: __vthunk_oub75675325861507c_iub75675325861507c,
    vfn_u8ad488637ad422a6: __vthunk_ou8ad488637ad422a6_iu8ad488637ad422a6,
    vfn_u1c00fc150b810f94: __vthunk_ou1c00fc150b810f94_iu1c00fc150b810f94,
    vfn_u590ad03b86fd5891: __vthunk_ou590ad03b86fd5891_iu590ad03b86fd5891,
};
pub unsafe fn __vdtor_u3baeb18df2f9ff7c(__this: *mut ACE_Event_Handler) {
    let _ = Box::from_raw(__this as *mut ACE_Process_Manager);
}
pub unsafe fn __vthunk_ou0d0a9599bbc178f0_iu3ac298578aa09514(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_Process_Manager>::handle_input((__this as *mut ACE_Process_Manager), p0)
}
pub unsafe fn __vthunk_ou4e9042e6c552562d_iua1174916b4160f19(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
    p1: libc::c_ulong,
) -> libc::c_int {
    <ACE_Process_Manager>::handle_close((__this as *mut ACE_Process_Manager), p0, p1)
}
pub unsafe fn __vthunk_ouc9dd99c769f46dd6_iu3f872401b3199422(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
    p1: *mut siginfo_t,
    p2: *mut ucontext_t,
) -> libc::c_int {
    <ACE_Process_Manager>::handle_signal(
        (__this as *mut ACE_Process_Manager),
        p0,
        p1,
        p2,
    )
}
#[repr(C)]
pub struct __Vtbl_u3baeb18df2f9ff7c {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Event_Handler),
    pub vfn_u9c73e06ba19a4821: unsafe fn(*mut ACE_Event_Handler) -> libc::c_int,
    pub vfn_u7d36445ae9d974fa: unsafe fn(*mut ACE_Event_Handler, libc::c_int),
    pub vfn_u5cd5a5fc8acd0342: unsafe fn(*mut ACE_Event_Handler) -> libc::c_int,
    pub vfn_u8c5dfd1fdaa209ab: unsafe fn(*mut ACE_Event_Handler, libc::c_int),
    pub vfn_u3ac298578aa09514: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uafe15b49ae15a941: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_ua4412b5a365fe817: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u0604b3232f6ba27b: unsafe fn(
        *mut ACE_Event_Handler,
        *const ACE_Time_Value,
        *const libc::c_void,
    ) -> libc::c_int,
    pub vfn_u140f83bfe0d335d9: unsafe fn(
        *mut ACE_Event_Handler,
        *mut ACE_Process,
    ) -> libc::c_int,
    pub vfn_ua1174916b4160f19: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u3f872401b3199422: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
        *mut siginfo_t,
        *mut ucontext_t,
    ) -> libc::c_int,
    pub vfn_uc909131ec77881c7: unsafe fn(*mut ACE_Event_Handler) -> libc::c_int,
    pub vfn_uf69856450f20fddf: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u366f25d08aaa768b: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u5cb133098a439c05: unsafe fn(*mut ACE_Event_Handler, *mut ACE_Reactor),
    pub vfn_ub75675325861507c: unsafe fn(*mut ACE_Event_Handler) -> *mut ACE_Reactor,
    pub vfn_u8ad488637ad422a6: unsafe fn(
        *mut ACE_Event_Handler,
    ) -> *mut ACE_Reactor_Timer_Interface,
    pub vfn_u1c00fc150b810f94: unsafe fn(*mut ACE_Event_Handler) -> libc::c_long,
    pub vfn_u590ad03b86fd5891: unsafe fn(*mut ACE_Event_Handler) -> libc::c_long,
}
pub static __VTBL_u3baeb18df2f9ff7c: __Vtbl_u3baeb18df2f9ff7c = __Vtbl_u3baeb18df2f9ff7c {
    __type_info: &__TYPEINFO_19ACE_Process_Manager,
    __vdtor: __vdtor_u3baeb18df2f9ff7c,
    vfn_u9c73e06ba19a4821: __vthunk_ou9c73e06ba19a4821_iu9c73e06ba19a4821,
    vfn_u7d36445ae9d974fa: __vthunk_ou7d36445ae9d974fa_iu7d36445ae9d974fa,
    vfn_u5cd5a5fc8acd0342: __vthunk_ou5cd5a5fc8acd0342_iu5cd5a5fc8acd0342,
    vfn_u8c5dfd1fdaa209ab: __vthunk_ou8c5dfd1fdaa209ab_iu8c5dfd1fdaa209ab,
    vfn_u3ac298578aa09514: __vthunk_ou0d0a9599bbc178f0_iu3ac298578aa09514,
    vfn_uafe15b49ae15a941: __vthunk_ouafe15b49ae15a941_iuafe15b49ae15a941,
    vfn_ua4412b5a365fe817: __vthunk_oua4412b5a365fe817_iua4412b5a365fe817,
    vfn_u0604b3232f6ba27b: __vthunk_ou0604b3232f6ba27b_iu0604b3232f6ba27b,
    vfn_u140f83bfe0d335d9: __vthunk_ou140f83bfe0d335d9_iu140f83bfe0d335d9,
    vfn_ua1174916b4160f19: __vthunk_ou4e9042e6c552562d_iua1174916b4160f19,
    vfn_u3f872401b3199422: __vthunk_ouc9dd99c769f46dd6_iu3f872401b3199422,
    vfn_uc909131ec77881c7: __vthunk_ouc909131ec77881c7_iuc909131ec77881c7,
    vfn_uf69856450f20fddf: __vthunk_ouf69856450f20fddf_iuf69856450f20fddf,
    vfn_u366f25d08aaa768b: __vthunk_ou366f25d08aaa768b_iu366f25d08aaa768b,
    vfn_u5cb133098a439c05: __vthunk_ou5cb133098a439c05_iu5cb133098a439c05,
    vfn_ub75675325861507c: __vthunk_oub75675325861507c_iub75675325861507c,
    vfn_u8ad488637ad422a6: __vthunk_ou8ad488637ad422a6_iu8ad488637ad422a6,
    vfn_u1c00fc150b810f94: __vthunk_ou1c00fc150b810f94_iu1c00fc150b810f94,
    vfn_u590ad03b86fd5891: __vthunk_ou590ad03b86fd5891_iu590ad03b86fd5891,
};
pub unsafe fn __vdtor_u33994cc85f1192b8(__this: *mut ACE_Cleanup) {
    let _ = Box::from_raw(
        __this as *mut ACE_Cleanup_Adapter_ACE_Recursive_Thread_Mutex_,
    );
}
pub static __VTBL_u33994cc85f1192b8: __Vtbl_u33994cc85f1192b8 = __Vtbl_u33994cc85f1192b8 {
    __type_info: &__TYPEINFO_19ACE_Cleanup_Adapter,
    __vdtor: __vdtor_u33994cc85f1192b8,
    vfn_ucaae14a381d74b6a: __vthunk_oucaae14a381d74b6a_iucaae14a381d74b6a,
};
pub unsafe fn __vthunk_oubcec569a70adf5c2_iubcec569a70adf5c2(
    __this: *mut ACE_Reactor_Impl,
    p0: libc::c_ulong,
    p1: bool,
    p2: *mut ACE_Sig_Handler,
    p3: *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___,
    p4: libc::c_int,
    p5: *mut ACE_Reactor_Notify,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou34cd077df5b3f6ae_iu34cd077df5b3f6ae(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Sig_Handler,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou1259ba200bddddef_iu1259ba200bddddef(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouac657357bdffab37_iuac657357bdffab37(
    __this: *mut ACE_Reactor_Impl,
) -> *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___ {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou421f838bc5307e2b_iu421f838bc5307e2b(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouace6981cff6fe71b_iuace6981cff6fe71b(
    __this: *mut ACE_Reactor_Impl,
    p0: libc::c_int,
    p1: *mut ACE_Event_Handler,
    p2: libc::c_ulong,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou9eb78727bd640fcb_iu9eb78727bd640fcb(
    __this: *mut ACE_Reactor_Impl,
    p0: libc::c_int,
    p1: libc::c_int,
    p2: *mut ACE_Event_Handler,
    p3: libc::c_ulong,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oub4e8b74bbca44329_iub4e8b74bbca44329(
    __this: *mut ACE_Reactor_Impl,
    p0: *const ACE_Handle_Set,
    p1: *mut ACE_Event_Handler,
    p2: libc::c_ulong,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouebbb7291ea242e70_iuebbb7291ea242e70(
    __this: *mut ACE_Reactor_Impl,
    p0: libc::c_int,
    p1: *mut ACE_Event_Handler,
    p2: *mut ACE_Sig_Action,
    p3: *mut *mut ACE_Event_Handler,
    p4: *mut ACE_Sig_Action,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouf0baf69df97ee5bb_iuf0baf69df97ee5bb(
    __this: *mut ACE_Reactor_Impl,
    p0: *const ACE_Sig_Set,
    p1: *mut ACE_Event_Handler,
    p2: *mut ACE_Sig_Action,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouaf0dc9c8418b9132_iuaf0dc9c8418b9132(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouc75cd90734e55231_iuc75cd90734e55231(
    __this: *mut ACE_Reactor_Impl,
    p0: libc::c_int,
    p1: *mut ACE_Sig_Action,
    p2: *mut ACE_Sig_Action,
    p3: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oud3adb6a76edadf4b_iud3adb6a76edadf4b(
    __this: *mut ACE_Reactor_Impl,
    p0: *const ACE_Sig_Set,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou2ab95efe2aaafd93_iu2ab95efe2aaafd93(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou4b62ea5905f5d054_iu4b62ea5905f5d054(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou0ed7a0590892f471_iu0ed7a0590892f471(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: *const libc::c_void,
    p2: *const ACE_Time_Value,
    p3: *const ACE_Time_Value,
) -> libc::c_long {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou6c03a6553a94f5f8_iu6c03a6553a94f5f8(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou35cce8b44d41241a_iu35cce8b44d41241a(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ouaef99343c0b53cf3_iuaef99343c0b53cf3(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou269a97af28385647_iu269a97af28385647(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
    p2: *mut ACE_Time_Value,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou9b86516632b01291_iu9b86516632b01291(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou495d4b8c1fe69e57_iu495d4b8c1fe69e57(
    __this: *mut ACE_Reactor_Impl,
    p0: libc::c_int,
) -> *mut ACE_Event_Handler {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou17856e9c20782909_iu17856e9c20782909(
    __this: *mut ACE_Reactor_Impl,
    p0: libc::c_int,
    p1: libc::c_ulong,
    p2: *mut *mut ACE_Event_Handler,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou02a50aad3bea1efc_iu02a50aad3bea1efc(
    __this: *mut ACE_Reactor_Impl,
    p0: libc::c_int,
    p1: *mut *mut ACE_Event_Handler,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou54940c38c361ab8a_iu54940c38c361ab8a(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou3f6fdf50bbadffcd_iu3f6fdf50bbadffcd(
    __this: *mut ACE_Reactor_Impl,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_u7168bc4535358c82 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Reactor_Impl),
    pub vfn_ubcec569a70adf5c2: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_ulong,
        bool,
        *mut ACE_Sig_Handler,
        *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___,
        libc::c_int,
        *mut ACE_Reactor_Notify,
    ) -> libc::c_int,
    pub vfn_u6a7b69fc514d715c: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        *mut libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u34cd077df5b3f6ae: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Sig_Handler,
    ) -> libc::c_int,
    pub vfn_u1259ba200bddddef: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___,
    ) -> libc::c_int,
    pub vfn_uac657357bdffab37: unsafe fn(
        *mut ACE_Reactor_Impl,
    ) -> *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___,
    pub vfn_u11c88cedb3abbab3: unsafe fn(*mut ACE_Reactor_Impl) -> libc::c_int,
    pub vfn_ucda8bd73b913aae2: unsafe fn(
        *mut ACE_Reactor_Impl,
        *const ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_ueb3555a06630494a: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_uf86867003a3dcb25: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_udb4a5c2c3ef2df76: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_uc9758e13f2d4e5e1: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_u436e43ca94274a59: unsafe fn(*mut ACE_Reactor_Impl) -> libc::c_int,
    pub vfn_u23fc623b1c3b8165: unsafe fn(*mut ACE_Reactor_Impl, libc::c_int),
    pub vfn_u421f838bc5307e2b: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_uace6981cff6fe71b: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        *mut ACE_Event_Handler,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u9eb78727bd640fcb: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        libc::c_int,
        *mut ACE_Event_Handler,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_ub4e8b74bbca44329: unsafe fn(
        *mut ACE_Reactor_Impl,
        *const ACE_Handle_Set,
        *mut ACE_Event_Handler,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_uebbb7291ea242e70: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        *mut ACE_Event_Handler,
        *mut ACE_Sig_Action,
        *mut *mut ACE_Event_Handler,
        *mut ACE_Sig_Action,
    ) -> libc::c_int,
    pub vfn_uf0baf69df97ee5bb: unsafe fn(
        *mut ACE_Reactor_Impl,
        *const ACE_Sig_Set,
        *mut ACE_Event_Handler,
        *mut ACE_Sig_Action,
    ) -> libc::c_int,
    pub vfn_uaf0dc9c8418b9132: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_udac36f861cf0a8f3: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_uc14a6d6420dea3b3: unsafe fn(
        *mut ACE_Reactor_Impl,
        *const ACE_Handle_Set,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_uc75cd90734e55231: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        *mut ACE_Sig_Action,
        *mut ACE_Sig_Action,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_ud3adb6a76edadf4b: unsafe fn(
        *mut ACE_Reactor_Impl,
        *const ACE_Sig_Set,
    ) -> libc::c_int,
    pub vfn_u2ab95efe2aaafd93: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
    ) -> libc::c_int,
    pub vfn_udf3e1116f7643292: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u466ed1ff499b0d62: unsafe fn(
        *mut ACE_Reactor_Impl,
        *const ACE_Handle_Set,
    ) -> libc::c_int,
    pub vfn_u02fd627e956944c7: unsafe fn(*mut ACE_Reactor_Impl) -> libc::c_int,
    pub vfn_u4b62ea5905f5d054: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
    ) -> libc::c_int,
    pub vfn_udc0eba225fcd4a87: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u8b4ef2f95c2e8dbd: unsafe fn(
        *mut ACE_Reactor_Impl,
        *const ACE_Handle_Set,
    ) -> libc::c_int,
    pub vfn_uadbacf5f6b488bd0: unsafe fn(*mut ACE_Reactor_Impl) -> libc::c_int,
    pub vfn_ufe6ad1e266584666: unsafe fn(*mut ACE_Reactor_Impl) -> libc::c_int,
    pub vfn_ub5a322e104b295a3: unsafe fn(*mut ACE_Reactor_Impl) -> bool,
    pub vfn_u0ed7a0590892f471: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        *const libc::c_void,
        *const ACE_Time_Value,
        *const ACE_Time_Value,
    ) -> libc::c_long,
    pub vfn_u5baab3aca5b8827f: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_long,
        *const ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_u6c03a6553a94f5f8: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_ub918df8b9f44606e: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_long,
        *mut *const libc::c_void,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u35cce8b44d41241a: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u3a4948b251e007ab: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_uaef99343c0b53cf3: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u70d2d1718da0b0a8: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u269a97af28385647: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        libc::c_ulong,
        *mut ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_u324d3bbce1889e52: unsafe fn(*mut ACE_Reactor_Impl, libc::c_int),
    pub vfn_u5d120e242195d36e: unsafe fn(*mut ACE_Reactor_Impl) -> libc::c_int,
    pub vfn_u9b86516632b01291: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u495d4b8c1fe69e57: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
    ) -> *mut ACE_Event_Handler,
    pub vfn_u17856e9c20782909: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        libc::c_ulong,
        *mut *mut ACE_Event_Handler,
    ) -> libc::c_int,
    pub vfn_u02a50aad3bea1efc: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        *mut *mut ACE_Event_Handler,
    ) -> libc::c_int,
    pub vfn_ubf7b5dbfbdbd9de1: unsafe fn(*mut ACE_Reactor_Impl) -> bool,
    pub vfn_ub8a3988b3562f107: unsafe fn(*mut ACE_Reactor_Impl) -> libc::c_ulong,
    pub vfn_ucda5680deed6f816: unsafe fn(*mut ACE_Reactor_Impl) -> *mut ACE_Lock,
    pub vfn_u995d18386c3ca1a6: unsafe fn(*mut ACE_Reactor_Impl),
    pub vfn_u61a984cbe2966a4e: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_ulong,
        *mut libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u95f3298d130aaa99: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u7b8714cbca14692e: unsafe fn(*mut ACE_Reactor_Impl) -> bool,
    pub vfn_u1782c7fc493b0fe5: unsafe fn(*mut ACE_Reactor_Impl, bool) -> bool,
    pub vfn_u5d59f9127796d40d: unsafe fn(*mut ACE_Reactor_Impl, libc::c_int),
    pub vfn_ub3766cf129b99a3d: unsafe fn(*mut ACE_Reactor_Impl) -> libc::c_int,
    pub vfn_u54940c38c361ab8a: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u213f3877c01f842b: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u3f6fdf50bbadffcd: unsafe fn(
        *mut ACE_Reactor_Impl,
        *mut ACE_Event_Handler,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u22e60c7be962e926: unsafe fn(
        *mut ACE_Reactor_Impl,
        libc::c_int,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u1540026ec95aae2c: unsafe fn(*mut ACE_Reactor_Impl),
}
pub static __VTBL_u7168bc4535358c82: __Vtbl_u7168bc4535358c82 = __Vtbl_u7168bc4535358c82 {
    __type_info: &__TYPEINFO_16ACE_Reactor_Impl,
    __vdtor: __vdtor_u7168bc4535358c82,
    vfn_ubcec569a70adf5c2: __vthunk_oubcec569a70adf5c2_iubcec569a70adf5c2,
    vfn_u6a7b69fc514d715c: __vthunk_ou6a7b69fc514d715c_iu6a7b69fc514d715c,
    vfn_u34cd077df5b3f6ae: __vthunk_ou34cd077df5b3f6ae_iu34cd077df5b3f6ae,
    vfn_u1259ba200bddddef: __vthunk_ou1259ba200bddddef_iu1259ba200bddddef,
    vfn_uac657357bdffab37: __vthunk_ouac657357bdffab37_iuac657357bdffab37,
    vfn_u11c88cedb3abbab3: __vthunk_ou11c88cedb3abbab3_iu11c88cedb3abbab3,
    vfn_ucda8bd73b913aae2: __vthunk_oucda8bd73b913aae2_iucda8bd73b913aae2,
    vfn_ueb3555a06630494a: __vthunk_oueb3555a06630494a_iueb3555a06630494a,
    vfn_uf86867003a3dcb25: __vthunk_ouf86867003a3dcb25_iuf86867003a3dcb25,
    vfn_udb4a5c2c3ef2df76: __vthunk_oudb4a5c2c3ef2df76_iudb4a5c2c3ef2df76,
    vfn_uc9758e13f2d4e5e1: __vthunk_ouc9758e13f2d4e5e1_iuc9758e13f2d4e5e1,
    vfn_u436e43ca94274a59: __vthunk_ou436e43ca94274a59_iu436e43ca94274a59,
    vfn_u23fc623b1c3b8165: __vthunk_ou23fc623b1c3b8165_iu23fc623b1c3b8165,
    vfn_u421f838bc5307e2b: __vthunk_ou421f838bc5307e2b_iu421f838bc5307e2b,
    vfn_uace6981cff6fe71b: __vthunk_ouace6981cff6fe71b_iuace6981cff6fe71b,
    vfn_u9eb78727bd640fcb: __vthunk_ou9eb78727bd640fcb_iu9eb78727bd640fcb,
    vfn_ub4e8b74bbca44329: __vthunk_oub4e8b74bbca44329_iub4e8b74bbca44329,
    vfn_uebbb7291ea242e70: __vthunk_ouebbb7291ea242e70_iuebbb7291ea242e70,
    vfn_uf0baf69df97ee5bb: __vthunk_ouf0baf69df97ee5bb_iuf0baf69df97ee5bb,
    vfn_uaf0dc9c8418b9132: __vthunk_ouaf0dc9c8418b9132_iuaf0dc9c8418b9132,
    vfn_udac36f861cf0a8f3: __vthunk_oudac36f861cf0a8f3_iudac36f861cf0a8f3,
    vfn_uc14a6d6420dea3b3: __vthunk_ouc14a6d6420dea3b3_iuc14a6d6420dea3b3,
    vfn_uc75cd90734e55231: __vthunk_ouc75cd90734e55231_iuc75cd90734e55231,
    vfn_ud3adb6a76edadf4b: __vthunk_oud3adb6a76edadf4b_iud3adb6a76edadf4b,
    vfn_u2ab95efe2aaafd93: __vthunk_ou2ab95efe2aaafd93_iu2ab95efe2aaafd93,
    vfn_udf3e1116f7643292: __vthunk_oudf3e1116f7643292_iudf3e1116f7643292,
    vfn_u466ed1ff499b0d62: __vthunk_ou466ed1ff499b0d62_iu466ed1ff499b0d62,
    vfn_u02fd627e956944c7: __vthunk_ou02fd627e956944c7_iu02fd627e956944c7,
    vfn_u4b62ea5905f5d054: __vthunk_ou4b62ea5905f5d054_iu4b62ea5905f5d054,
    vfn_udc0eba225fcd4a87: __vthunk_oudc0eba225fcd4a87_iudc0eba225fcd4a87,
    vfn_u8b4ef2f95c2e8dbd: __vthunk_ou8b4ef2f95c2e8dbd_iu8b4ef2f95c2e8dbd,
    vfn_uadbacf5f6b488bd0: __vthunk_ouadbacf5f6b488bd0_iuadbacf5f6b488bd0,
    vfn_ufe6ad1e266584666: __vthunk_oufe6ad1e266584666_iufe6ad1e266584666,
    vfn_ub5a322e104b295a3: __vthunk_oub5a322e104b295a3_iub5a322e104b295a3,
    vfn_u0ed7a0590892f471: __vthunk_ou0ed7a0590892f471_iu0ed7a0590892f471,
    vfn_u5baab3aca5b8827f: __vthunk_ou5baab3aca5b8827f_iu5baab3aca5b8827f,
    vfn_u6c03a6553a94f5f8: __vthunk_ou6c03a6553a94f5f8_iu6c03a6553a94f5f8,
    vfn_ub918df8b9f44606e: __vthunk_oub918df8b9f44606e_iub918df8b9f44606e,
    vfn_u35cce8b44d41241a: __vthunk_ou35cce8b44d41241a_iu35cce8b44d41241a,
    vfn_u3a4948b251e007ab: __vthunk_ou3a4948b251e007ab_iu3a4948b251e007ab,
    vfn_uaef99343c0b53cf3: __vthunk_ouaef99343c0b53cf3_iuaef99343c0b53cf3,
    vfn_u70d2d1718da0b0a8: __vthunk_ou70d2d1718da0b0a8_iu70d2d1718da0b0a8,
    vfn_u269a97af28385647: __vthunk_ou269a97af28385647_iu269a97af28385647,
    vfn_u324d3bbce1889e52: __vthunk_ou324d3bbce1889e52_iu324d3bbce1889e52,
    vfn_u5d120e242195d36e: __vthunk_ou5d120e242195d36e_iu5d120e242195d36e,
    vfn_u9b86516632b01291: __vthunk_ou9b86516632b01291_iu9b86516632b01291,
    vfn_u495d4b8c1fe69e57: __vthunk_ou495d4b8c1fe69e57_iu495d4b8c1fe69e57,
    vfn_u17856e9c20782909: __vthunk_ou17856e9c20782909_iu17856e9c20782909,
    vfn_u02a50aad3bea1efc: __vthunk_ou02a50aad3bea1efc_iu02a50aad3bea1efc,
    vfn_ubf7b5dbfbdbd9de1: __vthunk_oubf7b5dbfbdbd9de1_iubf7b5dbfbdbd9de1,
    vfn_ub8a3988b3562f107: __vthunk_oub8a3988b3562f107_iub8a3988b3562f107,
    vfn_ucda5680deed6f816: __vthunk_oucda5680deed6f816_iucda5680deed6f816,
    vfn_u995d18386c3ca1a6: __vthunk_ou995d18386c3ca1a6_iu995d18386c3ca1a6,
    vfn_u61a984cbe2966a4e: __vthunk_ou61a984cbe2966a4e_iu61a984cbe2966a4e,
    vfn_u95f3298d130aaa99: __vthunk_ou95f3298d130aaa99_iu95f3298d130aaa99,
    vfn_u7b8714cbca14692e: __vthunk_ou7b8714cbca14692e_iu7b8714cbca14692e,
    vfn_u1782c7fc493b0fe5: __vthunk_ou1782c7fc493b0fe5_iu1782c7fc493b0fe5,
    vfn_u5d59f9127796d40d: __vthunk_ou5d59f9127796d40d_iu5d59f9127796d40d,
    vfn_ub3766cf129b99a3d: __vthunk_oub3766cf129b99a3d_iub3766cf129b99a3d,
    vfn_u54940c38c361ab8a: __vthunk_ou54940c38c361ab8a_iu54940c38c361ab8a,
    vfn_u213f3877c01f842b: __vthunk_ou213f3877c01f842b_iu213f3877c01f842b,
    vfn_u3f6fdf50bbadffcd: __vthunk_ou3f6fdf50bbadffcd_iu3f6fdf50bbadffcd,
    vfn_u22e60c7be962e926: __vthunk_ou22e60c7be962e926_iu22e60c7be962e926,
    vfn_u1540026ec95aae2c: __vthunk_ou1540026ec95aae2c_iu1540026ec95aae2c,
};
pub unsafe fn __vdtor_u45a5d1d1a3b95282(__this: *mut ACE_Time_Value) {
    let _ = Box::from_raw(__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_);
}
pub unsafe fn __vthunk_ou060f4dfa320b8472_iuab959427fb1be952(
    __this: *mut ACE_Time_Value,
) -> ACE_Time_Value {
    <ACE_Time_Value_T_ACE_HR_Time_Policy_>::now(
        (__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_),
    )
}
pub unsafe fn __vthunk_ou6b0c2bdc6dc9e690_iu3fe790ad6cf423f0(
    __this: *mut ACE_Time_Value,
) -> ACE_Time_Value {
    <ACE_Time_Value_T_ACE_HR_Time_Policy_>::to_relative_time(
        (__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_),
    )
}
pub unsafe fn __vthunk_ou6c68d411ab7a714b_iuc093c9f86ab975ab(
    __this: *mut ACE_Time_Value,
) -> ACE_Time_Value {
    <ACE_Time_Value_T_ACE_HR_Time_Policy_>::to_absolute_time(
        (__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_),
    )
}
pub unsafe fn __vthunk_ouf34318776a11f38f_iu63f805d10a6c85ef(
    __this: *mut ACE_Time_Value,
) -> *mut ACE_Time_Value {
    <ACE_Time_Value_T_ACE_HR_Time_Policy_>::duplicate(
        (__this as *mut ACE_Time_Value_T_ACE_HR_Time_Policy_),
    )
}
pub static __VTBL_u45a5d1d1a3b95282: __Vtbl_u45a5d1d1a3b95282 = __Vtbl_u45a5d1d1a3b95282 {
    __type_info: &__TYPEINFO_16ACE_Time_Value_T,
    __vdtor: __vdtor_u45a5d1d1a3b95282,
    vfn_uab959427fb1be952: __vthunk_ou060f4dfa320b8472_iuab959427fb1be952,
    vfn_u3fe790ad6cf423f0: __vthunk_ou6b0c2bdc6dc9e690_iu3fe790ad6cf423f0,
    vfn_uc093c9f86ab975ab: __vthunk_ou6c68d411ab7a714b_iuc093c9f86ab975ab,
    vfn_u63f805d10a6c85ef: __vthunk_ouf34318776a11f38f_iu63f805d10a6c85ef,
};
pub unsafe fn __vdtor_ud248eb175b83721d(__this: *mut ACE_Event_Handler) {
    let _ = Box::from_raw(__this as *mut ACE_Reactor_Notify);
}
pub unsafe fn __vthunk_oube96b7ae489e3c55_iube96b7ae489e3c55(
    __this: *mut ACE_Reactor_Notify,
    p0: *mut ACE_Reactor_Impl,
    p1: *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___,
    p2: libc::c_int,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oudbd469aff18b2c3f_iudbd469aff18b2c3f(
    __this: *mut ACE_Reactor_Notify,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou9c077e1d35761163_iu9c077e1d35761163(
    __this: *mut ACE_Reactor_Notify,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
    p2: *mut ACE_Time_Value,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou6ccdd98a888c30b3_iu6ccdd98a888c30b3(
    __this: *mut ACE_Reactor_Notify,
    p0: *mut libc::c_int,
    p1: *mut ACE_Handle_Set,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oude929c5a15f036a7_iude929c5a15f036a7(
    __this: *mut ACE_Reactor_Notify,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou912354b9b93f5008_iu912354b9b93f5008(
    __this: *mut ACE_Reactor_Notify,
    p0: *mut ACE_Notification_Buffer,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou19c562da0700f375_iu19c562da0700f375(
    __this: *mut ACE_Reactor_Notify,
    p0: *mut ACE_Notification_Buffer,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou2a586c6e0631400e_iu2a586c6e0631400e(
    __this: *mut ACE_Reactor_Notify,
    p0: libc::c_int,
    p1: *mut ACE_Notification_Buffer,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oub6e70875c84bf506_iub6e70875c84bf506(
    __this: *mut ACE_Reactor_Notify,
    p0: libc::c_int,
) {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou1370e1a65e291ef2_iu1370e1a65e291ef2(
    __this: *mut ACE_Reactor_Notify,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_oucf084e0173088cc5_iucf084e0173088cc5(
    __this: *mut ACE_Reactor_Notify,
    p0: *mut ACE_Event_Handler,
    p1: libc::c_ulong,
) -> libc::c_int {
    panic!("pure virtual function called")
}
pub unsafe fn __vthunk_ou07330d006331c058_iu07330d006331c058(
    __this: *mut ACE_Reactor_Notify,
) {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_ud248eb175b83721d {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Event_Handler),
    pub vfn_u9c73e06ba19a4821: unsafe fn(*mut ACE_Event_Handler) -> libc::c_int,
    pub vfn_u7d36445ae9d974fa: unsafe fn(*mut ACE_Event_Handler, libc::c_int),
    pub vfn_u5cd5a5fc8acd0342: unsafe fn(*mut ACE_Event_Handler) -> libc::c_int,
    pub vfn_u8c5dfd1fdaa209ab: unsafe fn(*mut ACE_Event_Handler, libc::c_int),
    pub vfn_u3ac298578aa09514: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uafe15b49ae15a941: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_ua4412b5a365fe817: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u0604b3232f6ba27b: unsafe fn(
        *mut ACE_Event_Handler,
        *const ACE_Time_Value,
        *const libc::c_void,
    ) -> libc::c_int,
    pub vfn_u140f83bfe0d335d9: unsafe fn(
        *mut ACE_Event_Handler,
        *mut ACE_Process,
    ) -> libc::c_int,
    pub vfn_ua1174916b4160f19: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u3f872401b3199422: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
        *mut siginfo_t,
        *mut ucontext_t,
    ) -> libc::c_int,
    pub vfn_uc909131ec77881c7: unsafe fn(*mut ACE_Event_Handler) -> libc::c_int,
    pub vfn_uf69856450f20fddf: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u366f25d08aaa768b: unsafe fn(
        *mut ACE_Event_Handler,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u5cb133098a439c05: unsafe fn(*mut ACE_Event_Handler, *mut ACE_Reactor),
    pub vfn_ub75675325861507c: unsafe fn(*mut ACE_Event_Handler) -> *mut ACE_Reactor,
    pub vfn_u8ad488637ad422a6: unsafe fn(
        *mut ACE_Event_Handler,
    ) -> *mut ACE_Reactor_Timer_Interface,
    pub vfn_u1c00fc150b810f94: unsafe fn(*mut ACE_Event_Handler) -> libc::c_long,
    pub vfn_u590ad03b86fd5891: unsafe fn(*mut ACE_Event_Handler) -> libc::c_long,
    pub vfn_ube96b7ae489e3c55: unsafe fn(
        *mut ACE_Reactor_Notify,
        *mut ACE_Reactor_Impl,
        *mut ACE_Abstract_Timer_Queue_ACE_Event_Handler___,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_udbd469aff18b2c3f: unsafe fn(*mut ACE_Reactor_Notify) -> libc::c_int,
    pub vfn_u9c077e1d35761163: unsafe fn(
        *mut ACE_Reactor_Notify,
        *mut ACE_Event_Handler,
        libc::c_ulong,
        *mut ACE_Time_Value,
    ) -> libc::c_int,
    pub vfn_u6ccdd98a888c30b3: unsafe fn(
        *mut ACE_Reactor_Notify,
        *mut libc::c_int,
        *mut ACE_Handle_Set,
    ) -> libc::c_int,
    pub vfn_ude929c5a15f036a7: unsafe fn(*mut ACE_Reactor_Notify) -> libc::c_int,
    pub vfn_u912354b9b93f5008: unsafe fn(
        *mut ACE_Reactor_Notify,
        *mut ACE_Notification_Buffer,
    ) -> libc::c_int,
    pub vfn_u19c562da0700f375: unsafe fn(
        *mut ACE_Reactor_Notify,
        *mut ACE_Notification_Buffer,
    ) -> libc::c_int,
    pub vfn_u2a586c6e0631400e: unsafe fn(
        *mut ACE_Reactor_Notify,
        libc::c_int,
        *mut ACE_Notification_Buffer,
    ) -> libc::c_int,
    pub vfn_ub6e70875c84bf506: unsafe fn(*mut ACE_Reactor_Notify, libc::c_int),
    pub vfn_u1370e1a65e291ef2: unsafe fn(*mut ACE_Reactor_Notify) -> libc::c_int,
    pub vfn_ucf084e0173088cc5: unsafe fn(
        *mut ACE_Reactor_Notify,
        *mut ACE_Event_Handler,
        libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u07330d006331c058: unsafe fn(*mut ACE_Reactor_Notify),
}
pub static __VTBL_ud248eb175b83721d: __Vtbl_ud248eb175b83721d = __Vtbl_ud248eb175b83721d {
    __type_info: &__TYPEINFO_18ACE_Reactor_Notify,
    __vdtor: __vdtor_ud248eb175b83721d,
    vfn_u9c73e06ba19a4821: __vthunk_ou9c73e06ba19a4821_iu9c73e06ba19a4821,
    vfn_u7d36445ae9d974fa: __vthunk_ou7d36445ae9d974fa_iu7d36445ae9d974fa,
    vfn_u5cd5a5fc8acd0342: __vthunk_ou5cd5a5fc8acd0342_iu5cd5a5fc8acd0342,
    vfn_u8c5dfd1fdaa209ab: __vthunk_ou8c5dfd1fdaa209ab_iu8c5dfd1fdaa209ab,
    vfn_u3ac298578aa09514: __vthunk_ou3ac298578aa09514_iu3ac298578aa09514,
    vfn_uafe15b49ae15a941: __vthunk_ouafe15b49ae15a941_iuafe15b49ae15a941,
    vfn_ua4412b5a365fe817: __vthunk_oua4412b5a365fe817_iua4412b5a365fe817,
    vfn_u0604b3232f6ba27b: __vthunk_ou0604b3232f6ba27b_iu0604b3232f6ba27b,
    vfn_u140f83bfe0d335d9: __vthunk_ou140f83bfe0d335d9_iu140f83bfe0d335d9,
    vfn_ua1174916b4160f19: __vthunk_oua1174916b4160f19_iua1174916b4160f19,
    vfn_u3f872401b3199422: __vthunk_ou3f872401b3199422_iu3f872401b3199422,
    vfn_uc909131ec77881c7: __vthunk_ouc909131ec77881c7_iuc909131ec77881c7,
    vfn_uf69856450f20fddf: __vthunk_ouf69856450f20fddf_iuf69856450f20fddf,
    vfn_u366f25d08aaa768b: __vthunk_ou366f25d08aaa768b_iu366f25d08aaa768b,
    vfn_u5cb133098a439c05: __vthunk_ou5cb133098a439c05_iu5cb133098a439c05,
    vfn_ub75675325861507c: __vthunk_oub75675325861507c_iub75675325861507c,
    vfn_u8ad488637ad422a6: __vthunk_ou8ad488637ad422a6_iu8ad488637ad422a6,
    vfn_u1c00fc150b810f94: __vthunk_ou1c00fc150b810f94_iu1c00fc150b810f94,
    vfn_u590ad03b86fd5891: __vthunk_ou590ad03b86fd5891_iu590ad03b86fd5891,
    vfn_ube96b7ae489e3c55: __vthunk_oube96b7ae489e3c55_iube96b7ae489e3c55,
    vfn_udbd469aff18b2c3f: __vthunk_oudbd469aff18b2c3f_iudbd469aff18b2c3f,
    vfn_u9c077e1d35761163: __vthunk_ou9c077e1d35761163_iu9c077e1d35761163,
    vfn_u6ccdd98a888c30b3: __vthunk_ou6ccdd98a888c30b3_iu6ccdd98a888c30b3,
    vfn_ude929c5a15f036a7: __vthunk_oude929c5a15f036a7_iude929c5a15f036a7,
    vfn_u912354b9b93f5008: __vthunk_ou912354b9b93f5008_iu912354b9b93f5008,
    vfn_u19c562da0700f375: __vthunk_ou19c562da0700f375_iu19c562da0700f375,
    vfn_u2a586c6e0631400e: __vthunk_ou2a586c6e0631400e_iu2a586c6e0631400e,
    vfn_ub6e70875c84bf506: __vthunk_oub6e70875c84bf506_iub6e70875c84bf506,
    vfn_u1370e1a65e291ef2: __vthunk_ou1370e1a65e291ef2_iu1370e1a65e291ef2,
    vfn_ucf084e0173088cc5: __vthunk_oucf084e0173088cc5_iucf084e0173088cc5,
    vfn_u07330d006331c058: __vthunk_ou07330d006331c058_iu07330d006331c058,
};
