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
extern "C-unwind" {
    pub fn ace_thread_adapter(args: *mut libc::c_void) -> *mut libc::c_void;
}
#[repr(C)]
#[derive(Clone)]
pub struct Processed_Static_Svc {
    pub name_: *mut libc::c_char,
    pub assd_: *const ACE_Static_Svc_Descriptor,
}
impl Drop for Processed_Static_Svc {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_ubfd7e76020628643"]
                fn __ext(__this: *mut Processed_Static_Svc);
            }
            __ext(self as *mut Self);
        }
    }
}
/**Maintain a queue of services to be configured from the
  /// command-line.*/
pub type ACE_Service_Gestalt_ACE_SVC_QUEUE = ACE_Unbounded_Queue_ACE_String_Base_char__;
pub type ACE_Service_Gestalt_ACE_SVC_QUEUE_ITERATOR = ACE_Unbounded_Queue_Iterator_ACE_String_Base_char__;
///Maintain a set of the statically linked service descriptors.
pub type ACE_Service_Gestalt_ACE_STATIC_SVCS = ACE_Unbounded_Set_ACE_Static_Svc_Descriptor___;
pub type ACE_Service_Gestalt_ACE_STATIC_SVCS_ITERATOR = ACE_Unbounded_Set_Iterator_ACE_Static_Svc_Descriptor___;
pub type ACE_Service_Gestalt_ACE_PROCESSED_STATIC_SVCS = ACE_Unbounded_Set_ACE_Service_Gestalt__Processed_Static_Svc___;
pub type ACE_Service_Gestalt_ACE_PROCESSED_STATIC_SVCS_ITERATOR = ACE_Unbounded_Set_Iterator_ACE_Service_Gestalt__Processed_Static_Svc___;
#[doc = "* @class ACE_Service_Gestalt\n *\n * @brief Supplies common server operations for dynamic and static\n * configuration of services.\n *\n * The Gestalt embodies the concept of configuration context. On one\n * hand, it is a flat namespace, where names correspond to a Service\n * Object instance. A Gestalt owns the Service Repository instance,\n * which in turn owns the Service Object instances.\n *\n * Another aspect of a Gestalt is its responsibility for\n * record-keeping and accounting for the meta-data, necessary for\n * locating, removing or instantiating a service.\n *\n * A repository underlies an instance of a gestalt and its lifetime\n * may or may not be bounded by the lifetime of the gestalt, that owns\n * it. This feature is important for the derived classes and the\n * Service Config in particular."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Service_Gestalt {
    pub __base_0: ACE_Copy_Disabled,
    pub svc_repo_is_owned_: bool,
    pub svc_repo_size_: libc::c_ulong,
    pub is_opened_: libc::c_int,
    pub logger_key_: *const libc::c_char,
    pub no_static_svcs_: bool,
    pub svc_queue_: *mut ACE_Unbounded_Queue_ACE_String_Base_char__,
    pub svc_conf_file_queue_: *mut ACE_Unbounded_Queue_ACE_String_Base_char__,
    pub repo_: *mut ACE_Service_Repository,
    pub static_svcs_: *mut ACE_Unbounded_Set_ACE_Static_Svc_Descriptor___,
    pub processed_static_svcs_: *mut ACE_Unbounded_Set_ACE_Service_Gestalt__Processed_Static_Svc___,
    pub refcnt_: ACE_Atomic_Op_ACE_Thread_Mutex__long_,
}
impl Drop for ACE_Service_Gestalt {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u0eb5e3ca30bfabff"]
                fn __ext(__this: *mut ACE_Service_Gestalt);
            }
            __ext(self as *mut Self);
        }
    }
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
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Time_Value_zero: ACE_Time_Value;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Time_Value_max_time: ACE_Time_Value;
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
    pub use crate::full_ops_0::ACE_Utils::Truncator_long__unsigned_long_;
    pub use crate::full_ops_0::ACE_Utils::truncator;
    pub use crate::full_ops_0::ACE_Utils::truncate_cast___long__ub591475ebc843689;
}
extern "C-unwind" {
    #[link_name = "_ZlsRSoRK14ACE_Time_Value"]
    pub fn operator_shl_u87aa6c49d2c7f15d(
        o: *mut crate::__cxx_std::Ostream,
        v: *const ACE_Time_Value,
    ) -> *mut crate::__cxx_std::Ostream;
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
extern "C-unwind" {
    pub fn uname(__name: *mut utsname) -> libc::c_int;
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
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Guard_ACE_Thread_Mutex_ {
    pub lock_: *mut ACE_Thread_Mutex,
    pub owner_: libc::c_int,
}
pub unsafe extern "C-unwind" fn __xtu__ZN9ACE_GuardI16ACE_Thread_MutexEC1EPS0_(
    __this: *mut ACE_Guard_ACE_Thread_Mutex_,
    __a0: *mut ACE_Thread_Mutex,
) {
    ACE_Guard_ACE_Thread_Mutex_::new_at_s3dee0e9c3f55fa29(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN9ACE_GuardI16ACE_Thread_MutexEC1ERS0_(
    __this: *mut ACE_Guard_ACE_Thread_Mutex_,
    __a0: *mut ACE_Thread_Mutex,
) {
    ACE_Guard_ACE_Thread_Mutex_::new_at_s194b481c4e491c25(__this, __a0)
}
impl Drop for ACE_Guard_ACE_Thread_Mutex_ {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {
                <ACE_Guard_ACE_Thread_Mutex_>::release(
                    (__this) as *mut ACE_Guard_ACE_Thread_Mutex_,
                );
            }
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_uf8abb957f9c142b0(
    __this: *mut ACE_Guard_ACE_Thread_Mutex_,
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
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_ {
    pub vptr: *const (),
    pub rep_: *mut ACE_Service_Gestalt,
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
extern "C-unwind" {
    #[link_name = "_Z15ACE_TSS_cleanupPv"]
    pub fn ACE_TSS_cleanup(ptr: *mut libc::c_void);
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_String_Base_char_ {
    pub __base_0: ACE_String_Base_Const,
    pub allocator_: *mut ACE_Allocator,
    pub len_: libc::c_ulong,
    pub buf_len_: libc::c_ulong,
    pub rep_: *mut libc::c_char,
    pub release_: bool,
}
#[repr(C)]
pub struct ACE_String_Base_wchar_t_ {
    pub __base_0: ACE_String_Base_Const,
    pub allocator_: *mut ACE_Allocator,
    pub len_: libc::c_ulong,
    pub buf_len_: libc::c_ulong,
    pub rep_: *mut libc::wchar_t,
    pub release_: bool,
}
impl Clone for ACE_String_Base_wchar_t_ {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { Self::new_sd240ab6a8d28430f(self as *const Self) }
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwEC1EP13ACE_Allocator(
    __this: *mut ACE_String_Base_wchar_t_,
    __a0: *mut ACE_Allocator,
) {
    ACE_String_Base_wchar_t_::new_at_se57ca8063130c40c(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwE3setEPKwmb(
    __this: *mut ACE_String_Base_wchar_t_,
    s: *const libc::wchar_t,
    len: libc::c_ulong,
    release: bool,
) {
    unsafe { ACE_String_Base_wchar_t_::set_s62e3ada53e3eb77a(__this, s, len, release) }
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwEC1EPKwmP13ACE_Allocatorb(
    __this: *mut ACE_String_Base_wchar_t_,
    __a0: *const libc::wchar_t,
    __a1: libc::c_ulong,
    __a2: *mut ACE_Allocator,
    __a3: bool,
) {
    ACE_String_Base_wchar_t_::new_at_s591dcb439e3dca6a(__this, __a0, __a1, __a2, __a3)
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwE3setEPKwb(
    __this: *mut ACE_String_Base_wchar_t_,
    s: *const libc::wchar_t,
    release: bool,
) {
    unsafe { ACE_String_Base_wchar_t_::set_s6282c945bf4b6f29(__this, s, release) }
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwEC1EPKwP13ACE_Allocatorb(
    __this: *mut ACE_String_Base_wchar_t_,
    __a0: *const libc::wchar_t,
    __a1: *mut ACE_Allocator,
    __a2: bool,
) {
    ACE_String_Base_wchar_t_::new_at_s44bc67f7d5665fa3(__this, __a0, __a1, __a2)
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwE11fast_resizeEm(
    __this: *mut ACE_String_Base_wchar_t_,
    len: libc::c_ulong,
) {
    unsafe { ACE_String_Base_wchar_t_::fast_resize(__this, len) }
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwE6resizeEmw(
    __this: *mut ACE_String_Base_wchar_t_,
    len: libc::c_ulong,
    c: libc::wchar_t,
) {
    unsafe { ACE_String_Base_wchar_t_::resize(__this, len, c) }
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwEC1EmwP13ACE_Allocator(
    __this: *mut ACE_String_Base_wchar_t_,
    __a0: libc::c_ulong,
    __a1: libc::wchar_t,
    __a2: *mut ACE_Allocator,
) {
    ACE_String_Base_wchar_t_::new_at_sb8ac398513396d8a(__this, __a0, __a1, __a2)
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwEC1ERKS0_(
    __this: *mut ACE_String_Base_wchar_t_,
    __a0: *const ACE_String_Base_wchar_t_,
) {
    ACE_String_Base_wchar_t_::new_at_sd240ab6a8d28430f(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwEC1EwP13ACE_Allocator(
    __this: *mut ACE_String_Base_wchar_t_,
    __a0: libc::wchar_t,
    __a1: *mut ACE_Allocator,
) {
    ACE_String_Base_wchar_t_::new_at_sa2905e920df89523(__this, __a0, __a1)
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwEaSERKS0_(
    __this: *mut ACE_String_Base_wchar_t_,
    s: *const ACE_String_Base_wchar_t_,
) -> *mut ACE_String_Base_wchar_t_ {
    unsafe { ACE_String_Base_wchar_t_::operator_assign_s41b8627fed4b436a(__this, s) }
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwE6appendEPKwm(
    __this: *mut ACE_String_Base_wchar_t_,
    s: *const libc::wchar_t,
    slen: libc::c_ulong,
) -> *mut ACE_String_Base_wchar_t_ {
    unsafe { ACE_String_Base_wchar_t_::append(__this, s, slen) }
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_String_BaseIwEpLERKS0_(
    __this: *mut ACE_String_Base_wchar_t_,
    s: *const ACE_String_Base_wchar_t_,
) -> *mut ACE_String_Base_wchar_t_ {
    unsafe { ACE_String_Base_wchar_t_::operator_add_assign_s41b8627fed4b436a(__this, s) }
}
impl Drop for ACE_String_Base_wchar_t_ {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {
                if ((((((((((*__this).buf_len_ as libc::c_ulong))
                    != (((0) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                    && (((*__this).release_ as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    {
                        let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                            as *mut ACE_Allocator;
                        let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                            as *const *const __Vtbl_uf2113694993e252c);
                        ((*__vt)
                            .vfn_ucc7a27ee055bb87e)(
                            __obj,
                            (((*__this).rep_) as *mut libc::c_void),
                        )
                    };
                }
            }
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_ub1e7a422741777a2(
    __this: *mut ACE_String_Base_wchar_t_,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
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
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_String_Base_Const_npos: libc::c_ulong;
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
extern "C-unwind" {
    #[link_name = "_ZlsRSoRK15ACE_String_BaseIcE"]
    pub fn operator_shl_u5fd5ae8454682d97(
        _anon_0: *mut crate::__cxx_std::Ostream,
        _anon_1: *const ACE_String_Base_char_,
    ) -> *mut crate::__cxx_std::Ostream;
}
extern "C-unwind" {
    #[link_name = "_ZlsRSoRK15ACE_String_BaseIwE"]
    pub fn operator_shl_u5fad26845445f46b(
        _anon_0: *mut crate::__cxx_std::Ostream,
        _anon_1: *const ACE_String_Base_wchar_t_,
    ) -> *mut crate::__cxx_std::Ostream;
}
#[doc = "* @class ACE_NS_WString\n *\n * @brief This class retain the backward compatibility for\n *        ACE_Naming_Context and related classes.  The only addition to\n *        ACE_WString is a very naive \"wchar\" to \"char\" conversion\n *        function."]
#[repr(C)]
pub struct ACE_NS_WString {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_String_Base_wchar_t_>,
}
impl Clone for ACE_NS_WString {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { Self::new_u09f00c688053da5a(self as *const Self) }
    }
}
impl Drop for ACE_NS_WString {
    fn drop(&mut self) {
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_NS_WStringC1EP13ACE_Allocator(
    __this: *mut ACE_NS_WString,
    __a0: *mut ACE_Allocator,
) {
    ACE_NS_WString::new_at(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_NS_WStringC1EPKwP13ACE_Allocator(
    __this: *mut ACE_NS_WString,
    __a0: *const libc::wchar_t,
    __a1: *mut ACE_Allocator,
) {
    ACE_NS_WString::new_at_ua46d8be5d8861127(__this, __a0, __a1)
}
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_NS_WStringC1EPKwmP13ACE_Allocator(
    __this: *mut ACE_NS_WString,
    __a0: *const libc::wchar_t,
    __a1: libc::c_ulong,
    __a2: *mut ACE_Allocator,
) {
    ACE_NS_WString::new_at_u235e7c59f6bf4af0(__this, __a0, __a1, __a2)
}
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_NS_WStringC1EmP13ACE_Allocator(
    __this: *mut ACE_NS_WString,
    __a0: libc::c_ulong,
    __a1: *mut ACE_Allocator,
) {
    ACE_NS_WString::new_at_ucc1cd7240c01c0d9(__this, __a0, __a1)
}
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_NS_WStringC1ERKS_(
    __this: *mut ACE_NS_WString,
    __a0: *const ACE_NS_WString,
) {
    ACE_NS_WString::new_at_u09f00c688053da5a(__this, __a0)
}
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_NS_WStringC1EwP13ACE_Allocator(
    __this: *mut ACE_NS_WString,
    __a0: libc::wchar_t,
    __a1: *mut ACE_Allocator,
) {
    ACE_NS_WString::new_at_u945b29bb3f117680(__this, __a0, __a1)
}
pub unsafe extern "C-unwind" fn operator_add_ucf9935f2769ee036(
    mut s: *const ACE_NS_WString,
    mut t: *const ACE_NS_WString,
) -> ACE_NS_WString {
    unsafe {
        {
            let mut temp: ACE_NS_WString = ::core::mem::MaybeUninit::<
                ACE_NS_WString,
            >::zeroed()
                .assume_init();
            <ACE_NS_WString>::new_at_u09f00c688053da5a(
                (::core::ptr::addr_of_mut!(temp)) as *mut ACE_NS_WString,
                ::core::ptr::addr_of!((* s)),
            );
            <ACE_String_Base_wchar_t_>::operator_add_assign_s41b8627fed4b436a(
                (::core::ptr::addr_of_mut!(
                    (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!(temp)))
                    .__base_0) .cast:: < ACE_String_Base_wchar_t_ > ().cast_mut())
                )) as *mut ACE_String_Base_wchar_t_,
                ::core::ptr::addr_of!(
                    (* ::core::ptr::addr_of!(((* t)).__base_0) .cast:: <
                    ACE_String_Base_wchar_t_ > ().cast_mut())
                ) as *const ACE_String_Base_wchar_t_,
            );
            return <ACE_NS_WString>::new_u09f00c688053da5a(::core::ptr::addr_of!(temp));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_SString_npos: libc::c_ulong;
}
#[doc = "* @class ACE_SString\n *\n * @brief A very Simple String ACE_SString class.  This is not a\n * general-purpose string class, and you should probably consider\n * using ACE_CString is you don't understand why this class\n * exists...\n *\n * This class is optimized for efficiency, so it doesn't provide\n * any internal locking.\n * CAUTION: This class is only intended for use with applications\n * that understand how it works.  In particular, its destructor\n * does not deallocate its memory when it is destroyed...  We need\n * this class since the ACE_Map_Manager requires an object that\n * supports the operator == and operator !=.  This class uses an\n * ACE_Allocator to allocate memory.  The user can make this a\n * persistant class by providing an ACE_Allocator with a\n * persistable memory pool."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_SString {
    pub allocator_: *mut ACE_Allocator,
    pub len_: libc::c_ulong,
    pub rep_: *mut libc::c_char,
}
impl Drop for ACE_SString {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {}
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_u4d6609f0a1978709(__this: *mut ACE_SString) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
extern "C-unwind" {
    #[link_name = "_ZlsRSoRK11ACE_SString"]
    pub fn operator_shl_u35f6c0155e89e9c6(
        _anon_0: *mut crate::__cxx_std::Ostream,
        _anon_1: *const ACE_SString,
    ) -> *mut crate::__cxx_std::Ostream;
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Unbounded_Set_Ex_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____ {
    pub head_: *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
    pub cur_size_: libc::c_ulong,
    pub allocator_: *mut ACE_Allocator,
    pub comp_: ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc___,
}
pub unsafe extern "C-unwind" fn __xtu__ZN20ACE_Unbounded_Set_ExIPN19ACE_Service_Gestalt20Processed_Static_SvcE36ACE_Unbounded_Set_Default_ComparatorIS2_EEC1EP13ACE_Allocator(
    __this: *mut ACE_Unbounded_Set_Ex_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
    __a0: *mut ACE_Allocator,
) {
    ACE_Unbounded_Set_Ex_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____::new_at_s0ae916ca18eb9b4a(
        __this,
        __a0,
    )
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Unbounded_Set_Ex_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____ {
    pub head_: *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
    pub cur_size_: libc::c_ulong,
    pub allocator_: *mut ACE_Allocator,
    pub comp_: ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor___,
}
pub unsafe extern "C-unwind" fn __xtu__ZN20ACE_Unbounded_Set_ExIP25ACE_Static_Svc_Descriptor36ACE_Unbounded_Set_Default_ComparatorIS1_EEC1EP13ACE_Allocator(
    __this: *mut ACE_Unbounded_Set_Ex_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
    __a0: *mut ACE_Allocator,
) {
    ACE_Unbounded_Set_Ex_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____::new_at_s444fe85a0e6a1680(
        __this,
        __a0,
    )
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Unbounded_Set_Ex_Iterator_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____ {
    pub current_: *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
    pub set_: *mut ACE_Unbounded_Set_Ex_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
}
pub unsafe extern "C-unwind" fn __xtu__ZN29ACE_Unbounded_Set_Ex_IteratorIPN19ACE_Service_Gestalt20Processed_Static_SvcE36ACE_Unbounded_Set_Default_ComparatorIS2_EEC1ER20ACE_Unbounded_Set_ExIS2_S4_Eb(
    __this: *mut ACE_Unbounded_Set_Ex_Iterator_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
    __a0: *mut ACE_Unbounded_Set_Ex_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
    __a1: bool,
) {
    ACE_Unbounded_Set_Ex_Iterator_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____::new_at_s12410a96477c7a42(
        __this,
        __a0,
        __a1,
    )
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Unbounded_Set_Ex_Iterator_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____ {
    pub current_: *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
    pub set_: *mut ACE_Unbounded_Set_Ex_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
}
pub unsafe extern "C-unwind" fn __xtu__ZN29ACE_Unbounded_Set_Ex_IteratorIP25ACE_Static_Svc_Descriptor36ACE_Unbounded_Set_Default_ComparatorIS1_EEC1ER20ACE_Unbounded_Set_ExIS1_S3_Eb(
    __this: *mut ACE_Unbounded_Set_Ex_Iterator_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
    __a0: *mut ACE_Unbounded_Set_Ex_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
    __a1: bool,
) {
    ACE_Unbounded_Set_Ex_Iterator_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____::new_at_s3c3b9f37e89103ee(
        __this,
        __a0,
        __a1,
    )
}
#[repr(C)]
pub struct ACE_Unbounded_Queue_ACE_String_Base_char__ {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Unbounded_Queue_Iterator_ACE_String_Base_char__ {
    pub _opaque: [u8; 1],
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____ {
    pub next_: *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
    pub item_: *mut Processed_Static_Svc,
}
pub unsafe extern "C-unwind" fn __xtu__ZN8ACE_NodeIPN19ACE_Service_Gestalt20Processed_Static_SvcE36ACE_Unbounded_Set_Default_ComparatorIS2_EEC1EPS5_i(
    __this: *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
    __a0: *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
    __a1: libc::c_int,
) {
    ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____::new_at_sf07148c9327d6265(
        __this,
        __a0,
        __a1,
    )
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____ {
    pub next_: *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
    pub item_: *mut ACE_Static_Svc_Descriptor,
}
pub unsafe extern "C-unwind" fn __xtu__ZN8ACE_NodeIP25ACE_Static_Svc_Descriptor36ACE_Unbounded_Set_Default_ComparatorIS1_EEC1EPS4_i(
    __this: *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
    __a0: *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
    __a1: libc::c_int,
) {
    ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____::new_at_s7aae2d8bb68fae03(
        __this,
        __a0,
        __a1,
    )
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc___ {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor___ {}
#[repr(C)]
pub struct ACE_Unbounded_Set_ACE_Static_Svc_Descriptor___ {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Unbounded_Set_ACE_Service_Gestalt__Processed_Static_Svc___ {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Unbounded_Set_Iterator_ACE_Static_Svc_Descriptor___ {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Unbounded_Set_Iterator_ACE_Service_Gestalt__Processed_Static_Svc___ {
    pub _opaque: [u8; 1],
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Array_Map_unsigned_long__const_ACE_Service_Type___ {
    pub alloc_: crate::__cxx_std::Allocator,
    pub size_: libc::c_ulong,
    pub capacity_: libc::c_ulong,
    pub nodes_: *mut crate::__cxx_std::Pair<libc::c_ulong, *const ACE_Service_Type>,
}
#[repr(C)]
pub struct ACE_Service_Type {
    pub _opaque: [u8; 1],
}
#[doc = "* @class ACE_DLL\n *\n * @brief Provides an abstract interface for handling various DLL\n * operations.\n *\n * This class is a wrapper over the various methods for utilizing\n * a dynamically linked library (DLL), which is called a shared\n * library on some platforms.  Operations @c open(), @c close(), and\n * @c symbol() have been implemented to help opening/closing and\n * extracting symbol information from a DLL, respectively."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_DLL {
    pub open_mode_: libc::c_int,
    pub dll_name_: *mut libc::c_char,
    pub close_handle_on_destruction_: bool,
    pub dll_handle_: *mut ACE_DLL_Handle,
    pub error_: bool,
    pub errmsg_: ACE_String_Base_char_,
}
impl Drop for ACE_DLL {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u72326742edf4fc95"]
                fn __ext(__this: *mut ACE_DLL);
            }
            __ext(self as *mut Self);
        }
    }
}
pub type ACE_Service_Repository_array_type = ACE_Array_Map_unsigned_long__const_ACE_Service_Type___;
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Service_Repository_svc_rep_: *mut ACE_Service_Repository;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Service_Repository_delete_svc_rep_: bool;
}
#[doc = "* @class ACE_Service_Repository\n *\n * @brief Contains all the services offered by a Service\n * Configurator-based application.\n *\n * This class contains a vector of ACE_Service_Types *'s and\n * allows an administrative entity to centrally manage and\n * control the behavior of application services.  Note that if\n * services are removed from the middle of the repository the\n * order won't necessarily be maintained since the @a remove\n * method performs compaction.  However, the common case is not\n * to remove services, so typically they are deleted in the\n * reverse order that they were added originally."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Service_Repository {
    pub service_array_: ::core::mem::ManuallyDrop<
        ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
    >,
    pub lock_: ::core::mem::ManuallyDrop<ACE_Recursive_Thread_Mutex>,
}
impl Drop for ACE_Service_Repository {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u8e4f6144592834ef"]
                fn __ext(__this: *mut ACE_Service_Repository);
            }
            __ext(self as *mut Self);
        }
    }
}
#[doc = "* @class ACE_Service_Repository_Iterator\n *\n * @brief Iterate through the ACE_Service_Repository.\n *\n * Make sure not to delete entries as the iteration is going on\n * since this class is not designed as a robust iterator."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Service_Repository_Iterator {
    pub svc_rep_: *mut ACE_Service_Repository,
    pub next_: libc::c_ulong,
    pub ignore_suspended_: bool,
}
impl Drop for ACE_Service_Repository_Iterator {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {}
            ()
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_u843e13feabca4e1a(
    __this: *mut ACE_Service_Repository_Iterator,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[repr(C)]
pub struct ACE_Thread_Manager {
    pub _opaque: [u8; 1],
}
#[doc = "* @class ACE_Thread_Adapter\n *\n * @brief Converts a C++ function into a function that\n * can be called from a thread creation routine\n * (e.g., pthread_create() or _beginthreadex()) that expects an\n * extern \"C\" entry point.  This class also makes it possible to\n * transparently provide hooks to register a thread with an\n * ACE_Thread_Manager.\n *\n * This class is used in ACE_OS::thr_create().  In general, the\n * thread that creates an object of this class is different from\n * the thread that calls @c invoke() on this object.  Therefore,\n * the @c invoke() method is responsible for deleting itself."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Thread_Adapter {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Base_Thread_Adapter>,
    pub thr_mgr_: *mut ACE_Thread_Manager,
}
#[doc = "* @class ACE_Thread\n *\n * @brief Provides a wrapper for threads.\n *\n * This class provides a common interface that is mapped onto\n * POSIX Pthreads, Solaris threads, Win32 threads, VxWorks\n * threads, or pSoS threads.  Note, however, that it is\n * generally a better idea to use the ACE_Thread_Manager\n * programming API rather than the <ACE_Thread> API since the\n * thread manager is more powerful."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Thread {}
#[repr(C)]
pub struct ACE_Object_Manager_Preallocations {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Sig_Adapter {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Sig_Set {
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
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Framework_Repository_repository_: *mut ACE_Framework_Repository;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Framework_Repository_shutting_down_: libc::c_int;
}
#[doc = "* @class ACE_Framework_Repository\n *\n * @brief Contains all framework components used by an application.\n *\n * This class contains a vector of ACE_Framework_Component *'s.  On\n * destruction, framework components are destroyed in the reverse order\n * that they were added originally."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Framework_Repository {
    pub __base_0: ACE_Copy_Disabled,
    pub component_vector_: *mut *mut ACE_Framework_Component,
    pub current_size_: libc::c_int,
    pub total_size_: libc::c_int,
    pub lock_: ::core::mem::ManuallyDrop<ACE_Thread_Mutex>,
}
impl Drop for ACE_Framework_Repository {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u51f16968076dafde"]
                fn __ext(__this: *mut ACE_Framework_Repository);
            }
            __ext(self as *mut Self);
        }
    }
}
/**A helper class used to safely register dynamic services, which may contains
/// subordinate static services. It is used to capture the necessary data during
/// the parsing, but perform the actuall instantiation later.*/
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Service_Type_Factory {
    pub name_: ACE_String_Base_char_,
    pub type_: libc::c_int,
    pub location_: ::core::mem::ManuallyDrop<ACE_Auto_Ptr_ACE_Location_Node_>,
    pub is_active_: libc::c_int,
}
impl Drop for ACE_Service_Type_Factory {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_udbceb5f822dc9902"]
                fn __ext(__this: *mut ACE_Service_Type_Factory);
            }
            __ext(self as *mut Self);
        }
    }
}
#[doc = "* @class ACE_Location_Node\n *\n * @brief Keep track of where a shared library is located.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Location_Node {
    pub vptr: *const (),
    pub pathname_: *const libc::c_char,
    pub must_delete_: libc::c_int,
    pub dll_: ::core::mem::ManuallyDrop<ACE_DLL>,
    pub symbol_: *mut libc::c_void,
}
#[doc = "* @class ACE_Static_Svc_Descriptor\n *\n * @brief Holds the information necessary to describe a statically linked\n * Svc."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Static_Svc_Descriptor {
    pub name_: *const libc::c_char,
    pub type_: libc::c_int,
    pub alloc_: Option<
        unsafe extern "C-unwind" fn(
            *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
        ) -> *mut ACE_Service_Object,
    >,
    pub flags_: libc::c_uint,
    pub active_: libc::c_int,
}
#[doc = "* @class ACE_Svc_Conf_Param\n *\n * @brief An instance of this object will be passed down to the\n *        yyparse() and yylex() functions.\n *\n * This is intended for internal use within ACE service configuration\n * framework only.\n *\n * This class retains the state for a given parse/scan.  It primarily\n * makes it possible to hold the static object lock in the scanner\n * for as short a period of time as possible.  The resulting finer\n * grained locking prevents deadlocks from occurring when scanning a\n * `svc.conf' file and activating an ACE_Task, for example, as a\n * result of processing the directives in that file."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Svc_Conf_Param {
    pub source: _unnamed_union_at__build_ace_full_src_ACE_ace_Svc_Conf_Param_h_96_3_,
    pub r#type: libc::c_uint,
    pub yyerrno: libc::c_int,
    pub yylineno: libc::c_int,
    pub buffer: *mut ace_yy_buffer_state,
    pub obstack: ::core::mem::ManuallyDrop<ACE_Obstack_T_char_>,
    pub config: *mut ACE_Service_Gestalt,
}
pub unsafe extern "C-unwind" fn __xtu__ZN18ACE_Svc_Conf_ParamC1EP19ACE_Service_GestaltP8_IO_FILE(
    __this: *mut ACE_Svc_Conf_Param,
    __a0: *mut ACE_Service_Gestalt,
    __a1: *mut _IO_FILE,
) {
    ACE_Svc_Conf_Param::new_at(__this, __a0, __a1)
}
pub unsafe extern "C-unwind" fn __xtu__ZN18ACE_Svc_Conf_ParamC1EP19ACE_Service_GestaltPKc(
    __this: *mut ACE_Svc_Conf_Param,
    __a0: *mut ACE_Service_Gestalt,
    __a1: *const libc::c_char,
) {
    ACE_Svc_Conf_Param::new_at_u9d864a4bc56ea6c7(__this, __a0, __a1)
}
impl Drop for ACE_Svc_Conf_Param {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {
                    ace_yy_delete_buffer((*__this).buffer);
                }
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.obstack);
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_ufb14ef6be3332574(
    __this: *mut ACE_Svc_Conf_Param,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[doc = "* @class ACE_Service_Type_Dynamic_Guard\n *\n * @brief A forward service declaration guard.\n *\n * Helps to resolve an issue with hybrid services, i.e. dynamic\n * services, accompanied by static services in the same DLL.  Only\n * automatic instances of this class are supposed to exist. Those are\n * created during (dynamic) service initialization and serve to:\n *\n * (a) Ensure the service we are loading is ordered last in the\n * repository, following any other services it may cause to register,\n * as part of its own registration. This is a common case when\n * loading dynamic services from DLLs - there are often static\n * initializers, which register static services.\n *\n * (b) The SDG instance destructor detects if the dynamic service\n * initialized successfully and \"fixes-up\" all the newly registered\n * static services to hold a reference to the DLL, from which they\n * have originated."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Service_Type_Dynamic_Guard {
    pub repo_: *mut ACE_Service_Repository,
    pub repo_begin_: libc::c_ulong,
    pub name_: *const libc::c_char,
    pub repo_monitor_: ::core::mem::ManuallyDrop<ACE_Guard_ACE_Recursive_Thread_Mutex_>,
}
impl Drop for ACE_Service_Type_Dynamic_Guard {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_ue729f2d5355f08d7"]
                fn __ext(__this: *mut ACE_Service_Type_Dynamic_Guard);
            }
            __ext(self as *mut Self);
        }
    }
}
#[repr(C)]
pub struct ACE_Reactor {
    pub _opaque: [u8; 1],
}
pub type ACE_SERVICE_ALLOCATOR = Option<
    unsafe extern "C-unwind" fn(
        *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut ACE_Service_Object,
>;
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Threading_Helper_ACE_Thread_Mutex_ {
    pub key_: libc::c_uint,
}
impl Drop for ACE_Threading_Helper_ACE_Thread_Mutex_ {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u375521db2e35026e"]
                fn __ext(__this: *mut ACE_Threading_Helper_ACE_Thread_Mutex_);
            }
            __ext(self as *mut Self);
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Threading_Helper_ACE_Null_Mutex_ {}
impl Drop for ACE_Threading_Helper_ACE_Null_Mutex_ {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_u32b485fdd242dcdf"]
                fn __ext(__this: *mut ACE_Threading_Helper_ACE_Null_Mutex_);
            }
            __ext(self as *mut Self);
        }
    }
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Service_Config_reconfig_occurred_: libc::c_int;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Service_Config_be_a_daemon_: bool;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Service_Config_pid_file_name_: *mut libc::c_char;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Service_Config_signum_: libc::c_int;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Service_Config_signal_handler_: *mut ACE_Sig_Adapter;
}
#[doc = "* @class ACE_Service_Config\n *\n * @brief Supplies common server operations for dynamic and static\n * configuration of service.\n *\n * The ACE_Service_Config uses the Monostate pattern.  Therefore,\n * you can only have one of these instantiated per-process. It\n * represents the process-wide collection of services, which is\n * typically shared among all other configurable entities. The only\n * ACE_Service_Config instance is registered with and owned by the\n * ACE_Object_Manager.\n *\n * By contrast, the ACE_Service_Gestalt represents the collection\n * of services, pertaining to a configurable entity. Typically, a\n * \"configurable entity\" is an instance, which owns an instance of\n * ACE_Service_Gestalt in order to ensure full control over the\n * services it needs.\n *\n * Another facet of ACE_Service_Config is that for a given thread,\n * it provides access to its current, process-global\n * ACE_Service_Gestalt instance through its current() method.\n *\n * @note The signal_handler_ static member is allocated by the\n * ACE_Object_Manager.  The ACE_Service_Config constructor\n * uses signal_handler_.  Therefore, if the program has any\n * static ACE_Service_Config objects, there might be\n * initialization order problems.  They can be minimized, but\n * not eliminated, by _not_ #defining\n * ACE_HAS_NONSTATIC_OBJECT_MANAGER."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Service_Config {
    pub vptr: *const (),
    pub instance_: ::core::mem::ManuallyDrop<
        ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
    >,
    pub threadkey_: ::core::mem::ManuallyDrop<ACE_Threading_Helper_ACE_Thread_Mutex_>,
    pub is_opened_: bool,
    pub lock_: ::core::mem::ManuallyDrop<ACE_Thread_Mutex>,
}
#[doc = "* @class ACE_Service_Config_Guard\n *\n * @brief A guard class, designed to be instantiated on the stack.\n *\n * Instantiating it with a specific configuration ensures any references to\n * ACE_Service_Config::instance(), even when occurring in static constructors,\n * will allways access the designated configuration instance.\n * This comes very handy when a dynamic service also registers any static\n * services of its own and their static factories."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Service_Config_Guard {
    pub saved_: ::core::mem::ManuallyDrop<ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_>,
}
impl Drop for ACE_Service_Config_Guard {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_uc49fa1ae95096c9b"]
                fn __ext(__this: *mut ACE_Service_Config_Guard);
            }
            __ext(self as *mut Self);
        }
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Auto_Basic_Ptr_ACE_Location_Node_ {
    pub p_: *mut ACE_Location_Node,
}
pub unsafe extern "C-unwind" fn __xtu__ZN18ACE_Auto_Basic_PtrI17ACE_Location_NodeEC1EPS0_(
    __this: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
    __a0: *mut ACE_Location_Node,
) {
    ACE_Auto_Basic_Ptr_ACE_Location_Node_::new_at_s87b22e51700151eb(__this, __a0)
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_ {
    pub p_: *const ACE_Service_Type_Factory,
}
pub unsafe extern "C-unwind" fn __xtu__ZN18ACE_Auto_Basic_PtrIK24ACE_Service_Type_FactoryEC1EPS1_(
    __this: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
    __a0: *const ACE_Service_Type_Factory,
) {
    ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_::new_at_scab98f6cc0b80ce9(
        __this,
        __a0,
    )
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Auto_Ptr_const_ACE_Service_Type_Factory_ {
    pub __base_0: ::core::mem::ManuallyDrop<
        ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
    >,
}
impl Drop for ACE_Auto_Ptr_const_ACE_Service_Type_Factory_ {
    fn drop(&mut self) {
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Auto_PtrIK24ACE_Service_Type_FactoryEC1EPS1_(
    __this: *mut ACE_Auto_Ptr_const_ACE_Service_Type_Factory_,
    __a0: *const ACE_Service_Type_Factory,
) {
    ACE_Auto_Ptr_const_ACE_Service_Type_Factory_::new_at_sfddb0cf77137aa44(__this, __a0)
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Auto_Ptr_ACE_Location_Node_ {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Auto_Basic_Ptr_ACE_Location_Node_>,
}
impl Drop for ACE_Auto_Ptr_ACE_Location_Node_ {
    fn drop(&mut self) {
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Auto_PtrI17ACE_Location_NodeEC1EPS0_(
    __this: *mut ACE_Auto_Ptr_ACE_Location_Node_,
    __a0: *mut ACE_Location_Node,
) {
    ACE_Auto_Ptr_ACE_Location_Node_::new_at_s219cbb6102035cda(__this, __a0)
}
extern "C-unwind" {
    pub fn _dl_mcount_wrapper_check(__selfpc: *mut libc::c_void);
}
extern "C-unwind" {
    pub fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
}
extern "C-unwind" {
    pub fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn dlmopen(
        __nsid: libc::c_long,
        __file: *const libc::c_char,
        __mode: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn dlvsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
        __version: *const libc::c_char,
    ) -> *mut libc::c_void;
}
extern "C-unwind" {
    pub fn dlerror() -> *mut libc::c_char;
}
extern "C-unwind" {
    pub fn dladdr(__address: *const libc::c_void, __info: *mut Dl_info) -> libc::c_int;
}
extern "C-unwind" {
    pub fn dladdr1(
        __address: *const libc::c_void,
        __info: *mut Dl_info,
        __extra_info: *mut *mut libc::c_void,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn dlinfo(
        __handle: *mut libc::c_void,
        __request: libc::c_int,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn _dl_find_object(
        __address: *mut libc::c_void,
        __result: *mut dl_find_object,
    ) -> libc::c_int;
}
#[repr(C)]
pub struct ACE_DLL_Handle {
    pub _opaque: [u8; 1],
}
#[doc = "* @class ACE_Parse_Node\n *\n * @brief Provide the base of the object hierarchy that defines the parse\n * tree of Service Nodes.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Parse_Node {
    pub vptr: *const (),
    pub name_: *const libc::c_char,
    pub next_: *mut ACE_Parse_Node,
}
#[doc = "* @class ACE_Suspend_Node\n *\n * @brief Suspend a Service Node.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Suspend_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Parse_Node>,
}
#[doc = "* @class ACE_Resume_Node\n *\n * @brief Resume a Service Node.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Resume_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Parse_Node>,
}
#[doc = "* @class ACE_Remove_Node\n *\n * @brief Remove a Service Node.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Remove_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Parse_Node>,
}
#[doc = "* @class ACE_Static_Node\n *\n * @brief Handle a statically linked node.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Static_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Parse_Node>,
    pub parameters_: *mut libc::c_char,
}
#[doc = "* @class ACE_Dynamic_Node\n *\n * @brief Handle a dynamically linked node.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Dynamic_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Static_Node>,
    pub factory_: ::core::mem::ManuallyDrop<
        ACE_Auto_Ptr_const_ACE_Service_Type_Factory_,
    >,
}
#[doc = "* @class ACE_Stream_Node\n *\n * @brief Handle a Stream.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Stream_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Parse_Node>,
    pub node_: *const ACE_Static_Node,
    pub mods_: *const ACE_Parse_Node,
}
#[doc = "* @class ACE_Dummy_Node\n *\n * @brief I forget why this is here... ;-)\n * @brief Used in a special case of static STREAM definintion\n *\n * @note This class is only meant for INTERNAL use by ACE."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Dummy_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Parse_Node>,
    pub node_: *const ACE_Static_Node,
    pub mods_: *const ACE_Parse_Node,
}
#[doc = "* @class ACE_Object_Node\n *\n * @brief Keeps track of the symbol name for a shared object.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Object_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Location_Node>,
    pub object_name_: *const libc::c_char,
}
#[doc = "* @class ACE_Function_Node\n *\n * @brief Keeps track of the symbol name of for a shared function.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Function_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Location_Node>,
    pub function_name_: *const libc::c_char,
}
#[doc = "* @class ACE_Static_Function_Node\n *\n * @brief Keeps track of the symbol name for a function that is not\n * linked in from a DLL, but is statically linked with the\n * application.\n *\n * @note This class is only meant for INTERNAL use by ACE.\n *\n * @internal"]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Static_Function_Node {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Location_Node>,
    pub function_name_: *const libc::c_char,
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Obstack_T_char_ {
    pub allocator_strategy_: *mut ACE_Allocator,
    pub size_: libc::c_ulong,
    pub head_: *mut ACE_Obchunk,
    pub curr_: *mut ACE_Obchunk,
}
pub type ACE_Obstack = ACE_Obstack_T_char_;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ace_yy_buffer_state_Ace_ {}
#[export_name = "_ZN19ace_yy_buffer_stateC1Ev"]
pub unsafe extern "C-unwind" fn __xtu__ZN19ace_yy_buffer_stateC1Ev(
    __this: *mut ace_yy_buffer_state,
) {
    ace_yy_buffer_state::new_at(__this)
}
#[export_name = "__acedtor_u25039a0d06fab016"]
pub unsafe extern "C-unwind" fn __acedtor_u25039a0d06fab016(
    __this: *mut ace_yy_buffer_state,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[export_name = "_Z20ace_yy_delete_bufferP19ace_yy_buffer_state"]
pub unsafe extern "C-unwind" fn ace_yy_delete_buffer(
    mut buffer: *mut ace_yy_buffer_state,
) {
    unsafe {
        {
            {
                let __d = buffer;
                if !__d.is_null() {
                    let _ = Box::from_raw(__d as *mut ace_yy_buffer_state);
                }
            };
        }
        ()
    }
}
extern "C-unwind" {
    #[link_name = "_Z11ace_yyparsev"]
    pub fn ace_yyparse() -> libc::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ACE_YYSTYPE {
    pub type_: libc::c_int,
    pub location_node_: *mut ACE_Location_Node,
    pub parse_node_: *mut ACE_Parse_Node,
    pub static_node_: *mut ACE_Static_Node,
    pub svc_record_: *mut ACE_Service_Type_Factory,
    pub ident_: *mut libc::c_char,
}
#[no_mangle]
pub unsafe extern "C-unwind" fn ace_yylex(
    mut ace_yylval: *mut ACE_YYSTYPE,
    mut ace_svc_conf_parameter: *mut libc::c_void,
) -> libc::c_int {
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
            ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int as libc::c_int)
                != 0)
            {} else {
                return (-((1) as libc::c_int));
            };
            return <ACE_Svc_Conf_Lexer>::yylex(
                ace_yylval,
                ((ace_svc_conf_parameter as *mut ACE_Svc_Conf_Param)),
            );
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
}
extern "C" {
    pub static mut ace_yyin: *mut _IO_FILE;
}
extern "C-unwind" {
    #[link_name = "_Z11ace_yyerrorPvPKc"]
    pub fn ace_yyerror(
        ace_svc_conf_parameter: *mut libc::c_void,
        _anon_1: *const libc::c_char,
    );
}
extern "C-unwind" {
    #[link_name = "_Z11ace_yyerroriiPKc"]
    pub fn ace_yyerror_uef3b1714225fccac(
        yyerrno: libc::c_int,
        yylineno: libc::c_int,
        _anon_2: *const libc::c_char,
    );
}
extern "C" {
    pub static mut ace_yytext: *mut libc::c_char;
}
extern "C" {
    pub static mut ace_yyleng: libc::c_int;
}
extern "C-unwind" {
    #[link_name = "_Z23ace_create_service_typePKciPvjPFvS1_E"]
    pub fn ace_create_service_type(
        _anon_0: *const libc::c_char,
        _anon_1: libc::c_int,
        _anon_2: *mut libc::c_void,
        _anon_3: libc::c_uint,
        _anon_4: Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut ACE_Service_Type_Impl;
}
#[repr(C)]
pub struct ACE_Encoding_Converter {
    pub _opaque: [u8; 1],
}
/**This class lexes the classic ACE Service Configurator language.
 * The entry point is similar to what flex would generate.  However, it
 * is a static method in this class (which is really just name space).*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_Svc_Conf_Lexer {}
#[export_name = "_ZN18ACE_Svc_Conf_Lexer5yylexEP11ACE_YYSTYPEP18ACE_Svc_Conf_Param"]
pub unsafe extern "C-unwind" fn __xtu__ZN18ACE_Svc_Conf_Lexer5yylexEP11ACE_YYSTYPEP18ACE_Svc_Conf_Param(
    ace_yylval: *mut ACE_YYSTYPE,
    param: *mut ACE_Svc_Conf_Param,
) -> libc::c_int {
    unsafe { ACE_Svc_Conf_Lexer::yylex(ace_yylval, param) }
}
#[export_name = "_ZN18ACE_Svc_Conf_Lexer5inputEP18ACE_Svc_Conf_ParamPcm"]
pub unsafe extern "C-unwind" fn __xtu__ZN18ACE_Svc_Conf_Lexer5inputEP18ACE_Svc_Conf_ParamPcm(
    param: *mut ACE_Svc_Conf_Param,
    buf: *mut libc::c_char,
    max_size: libc::c_ulong,
) -> libc::c_ulong {
    unsafe { ACE_Svc_Conf_Lexer::input(param, buf, max_size) }
}
#[export_name = "_ZN18ACE_Svc_Conf_Lexer4scanEP11ACE_YYSTYPEP18ACE_Svc_Conf_Param"]
pub unsafe extern "C-unwind" fn __xtu__ZN18ACE_Svc_Conf_Lexer4scanEP11ACE_YYSTYPEP18ACE_Svc_Conf_Param(
    ace_yylval: *mut ACE_YYSTYPE,
    param: *mut ACE_Svc_Conf_Param,
) -> libc::c_int {
    unsafe { ACE_Svc_Conf_Lexer::scan(ace_yylval, param) }
}
extern "C-unwind" {
    #[link_name = "_Z11ace_yyparsePv"]
    pub fn ace_yyparse_u74884a69bbbc7699(
        ace_svc_conf_parameter: *mut libc::c_void,
    ) -> libc::c_int;
}
#[export_name = "_Z9normalizem"]
pub unsafe extern "C-unwind" fn normalize(mut length: libc::c_ulong) -> libc::c_ulong {
    unsafe {
        {
            return (if (((((length as libc::c_ulong)) >= (((4) as libc::c_ulong)))
                as libc::c_int as libc::c_int) != 0)
            {
                (((((((length) as libc::c_ulong)) / ((4) as libc::c_ulong)))
                    as libc::c_ulong))
                    .wrapping_mul((4) as libc::c_ulong)
            } else {
                length
            });
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
    pub fn __builtin_va_start(_anon_0: ::core::ffi::VaList<'_>, ...);
}
extern "C-unwind" {
    pub fn __builtin_va_end(_anon_0: ::core::ffi::VaList<'_>);
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
    pub fn __builtin_ceil(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __builtin_floor(_anon_0: libc::c_double) -> libc::c_double;
}
extern "C-unwind" {
    pub fn __atomic_thread_fence(_anon_0: libc::c_int);
}
extern "C-unwind" {
    pub fn __atomic_always_lock_free(
        _anon_0: libc::c_ulong,
        _anon_1: *const libc::c_void,
    ) -> bool;
}
extern "C-unwind" {
    pub fn __atomic_test_and_set(
        _anon_0: *mut libc::c_void,
        _anon_1: libc::c_int,
    ) -> bool;
}
extern "C-unwind" {
    pub fn __builtin_unreachable();
}
extern "C-unwind" {
    pub fn __atomic_clear(_anon_0: *mut libc::c_void, _anon_1: libc::c_int);
}
impl ACE_Service_Gestalt {
    /**Constructor either associates the instance with the process-wide
  /// singleton instance of ACE_Service_Repository, or creates and
  /// manages its own instance of the specified size.*/
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: libc::c_ulong,
        mut __a1: bool,
        mut __a2: bool,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_GestaltC1Embb"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                __a0: libc::c_ulong,
                __a1: bool,
                __a2: bool,
            );
        }
        __ext(__this as *mut ACE_Service_Gestalt, __a0, __a1, __a2)
    }
    pub unsafe fn new(mut __a0: libc::c_ulong, mut __a1: bool, mut __a2: bool) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1, __a2);
        __obj
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK19ACE_Service_Gestalt4dumpEv"]
            fn __ext(__this: *const ACE_Service_Gestalt);
        }
        __ext(__this as *const ACE_Service_Gestalt)
    }
    #[doc = "* Performs an open without parsing command-line arguments.  The\n   * @a logger_key indicates where to write the logging output, which\n   * is typically either a STREAM pipe or a socket address.  If\n   * @a ignore_static_svcs is true then static services are not loaded,\n   * otherwise, they are loaded.  If @a ignore_default_svc_conf_file is\n   * true then the @c svc.conf configuration file will be ignored.\n   * Returns zero upon success, -1 if the file is not found or cannot\n   * be opened (errno is set accordingly), otherwise returns the\n   * number of errors encountered loading the services in the\n   * specified svc.conf configuration file.  If @a ignore_debug_flag is\n   * true then the application is responsible for setting the\n   * ACE_Log_Msg::priority_mask appropriately."]
    pub unsafe fn open(
        __this: *mut Self,
        mut program_name: *const libc::c_char,
        mut logger_key: *const libc::c_char,
        mut ignore_static_svcs: bool,
        mut ignore_default_svc_conf: bool,
        mut ignore_debug_flag: bool,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).no_static_svcs_ = ignore_static_svcs;
                return <ACE_Service_Gestalt>::open_i(
                    (__this) as *mut ACE_Service_Gestalt,
                    ((program_name) as *const libc::c_char),
                    logger_key,
                    ignore_static_svcs,
                    ignore_default_svc_conf,
                    ignore_debug_flag,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* This is the primary entry point into the ACE_Service_Config (the\n   * constructor just handles simple initializations).  It parses\n   * arguments passed in from @a argc and @a argv parameters.  The\n   * arguments that are valid in a call to this method include:\n   *\n   * - '-b' Option to indicate that we should be a daemon. Note that when\n   *        this option is used, the process will be daemonized before the\n   *        service configuration file(s) are read. During daemonization,\n   *        (on POSIX systems) the current directory will be changed to \"/\"\n   *        so the caller should either fully specify the file names, or\n   *        execute a @c chroot() to the appropriate directory.\n   *        @sa ACE::daemonize().\n   * - '-d' Turn on debugging mode\n   * - '-f' Specifies a configuration file name other than the default\n   *        svc.conf. Can be specified multiple times to use multiple files.\n   *        If any configuration file is provided with this option then\n   *        the default svc.conf will be ignored.\n   * - '-k' Specifies the rendezvous point to use for the ACE distributed\n   *        logger.\n   * - '-y' Explicitly enables the use of static services. This flag\n   *        overrides the @a ignore_static_svcs parameter value.\n   * - '-n' Explicitly disables the use of static services. This flag\n   *        overrides the @a ignore_static_svcs parameter value.\n   * - '-p' Specifies a pathname which is used to store the process id.\n   * - '-s' Specifies a signal number other than SIGHUP to trigger reprocessing\n   *        of the configuration file(s). Ignored for platforms that do not\n   *        have POSIX signals, such as Windows.\n   * - '-S' Specifies a service directive string. Enclose the string in quotes\n   *        and escape any embedded quotes with a backslash. This option\n   *        specifies service directives without the need for a configuration\n   *        file. Can be specified multiple times.\n   *\n   * Note: Options '-f' and '-S' complement each other. Directives\n   * from files and from '-S' option are processed together in the\n   * following order. First, the default file \"./svc.conf\" is\n   * evaluated if not ignored, then all files are processed in the\n   * order they are specified in '-f' @a argv parameter. Finally, all\n   * '-S' directive strings are executed in the order the directives\n   * appear in @a argv parameter.\n   *\n   * If no files or directives are added via the '-f' and '-S'\n   * arguments, and the default file is not ignored, it will be\n   * evaluated whether it exists or not, possibly causing a failure\n   * return. If any other directives are added then the default file\n   * will be evaluated only if it exists.\n   *\n   * @param argc The number of commandline arguments.\n   * @param argv The array with commandline arguments\n   * @param logger_key   Indicates where to write the logging output,\n   *                     which is typically either a STREAM pipe or a\n   *                     socket address.\n   * @param ignore_static_svcs   If true then static services are not loaded,\n   *                             otherwise, they are loaded.\n   * @param ignore_default_svc_conf_file  If false then the @c ./svc.conf\n   *                                      configuration file will be ignored.\n   * @param ignore_debug_flag If false then the application is responsible\n   *                          for setting the @c ACE_Log_Msg::priority_mask\n   *                          appropriately.\n   *\n   * @retval -1   A configuration file is not found or cannot\n   *              be opened (errno is set accordingly).\n   * @retval  0   Success.\n   * @retval  >0  The number of directive errors encountered while processing\n   *              the service configuration file(s)."]
    pub unsafe fn open_uf8bfee49ed7e36e7(
        __this: *mut Self,
        mut argc: libc::c_int,
        mut argv: *mut *mut libc::c_char,
        mut logger_key: *const libc::c_char,
        mut ignore_static_svcs: bool,
        mut ignore_default_svc_conf: bool,
        mut ignore_debug_flag: bool,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                (*__this).no_static_svcs_ = ignore_static_svcs;
                if (((((<ACE_Service_Gestalt>::parse_args_i(
                    (__this) as *mut ACE_Service_Gestalt,
                    argc,
                    ((argv) as *mut *mut libc::c_char),
                    ::core::ptr::addr_of_mut!(ignore_default_svc_conf),
                ) as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                return <ACE_Service_Gestalt>::open_i(
                    (__this) as *mut ACE_Service_Gestalt,
                    ((if (((((argv).is_null()) as libc::c_int) as libc::c_int) != 0) {
                        ((0) as *mut libc::c_char)
                    } else {
                        (*(argv).wrapping_offset((0) as isize))
                    }) as *const libc::c_char),
                    logger_key,
                    (*__this).no_static_svcs_,
                    ignore_default_svc_conf,
                    ignore_debug_flag,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Has it been opened?  Returns the difference between the times
  /// open and close have been called on this instance*/
    pub unsafe fn is_opened(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt9is_openedEv"]
            fn __ext(__this: *mut ACE_Service_Gestalt) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt)
    }
    /**Process one service configuration @a directive, which is passed as
  /// a string.  Returns the number of errors that occurred.*/
    pub unsafe fn process_directive(
        __this: *mut Self,
        mut directive: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt17process_directiveEPKc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                directive: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, directive)
    }
    #[doc = "Process one static service definition.\n  /**\n   * Load a new static service.\n   *\n   * @param ssd Service descriptor, see the document of\n   *        ACE_Static_Svc_Descriptor for more details.\n   *\n   * @param force_replace If set the new service descriptor replaces\n   *        any previous instance in the repository.\n   *\n   * @return Returns -1 if the service cannot be 'loaded'."]
    pub unsafe fn process_directive_u897e71149117fba7(
        __this: *mut Self,
        mut ssd: *const ACE_Static_Svc_Descriptor,
        mut force_replace: bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt17process_directiveERK25ACE_Static_Svc_Descriptorb"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                ssd: *const ACE_Static_Svc_Descriptor,
                force_replace: bool,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, ssd, force_replace)
    }
    /**Process a file containing a list of service configuration
  /// directives.*/
    pub unsafe fn process_file(
        __this: *mut Self,
        mut file: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt12process_fileEPKc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                file: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, file)
    }
    #[doc = "* Locate an entry with @a name in the table.  If @a ignore_suspended\n   * is set then only consider services marked as resumed.  If the\n   * caller wants the located entry, pass back a pointer to the\n   * located entry via @a srp.  If @a name is not found, -1 is returned.\n   * If @a name is found, but it is suspended and the caller wants to\n   * ignore suspended services a -2 is returned."]
    pub unsafe fn find(
        __this: *const Self,
        mut name: *const libc::c_char,
        mut srp: *mut *const ACE_Service_Type,
        mut ignore_suspended: bool,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((!((*__this).repo_).is_null()) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    return <ACE_Service_Repository>::find(
                        ((*__this).repo_) as *const ACE_Service_Repository,
                        ((name) as *const libc::c_char),
                        srp,
                        ignore_suspended,
                    );
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Handle the command-line options intended for the\n   * ACE_Service_Gestalt.  Note that @c argv[0] is assumed to be the\n   * program name.\n   *\n   * The arguments that are valid in a call to this method are\n   * - '-d' Turn on debugging mode\n   * - '-f' Option to read in the list of svc.conf file names\n   * - '-k' Option to read a wide string where in the logger output can\n   *        be written\n   * - '-y' Turn on the flag for a  repository of statically\n   *        linked services\n   * - '-n' Need not have a repository of statically linked services\n   * - '-S' Option to read in the list of services on the command-line\n   *        Please observe the difference between options '-f' that looks\n   *        for a list of files and here a list of services."]
    pub unsafe fn parse_args(
        __this: *mut Self,
        mut argc: libc::c_int,
        mut argv: *mut *mut libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt10parse_argsEiPPc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                argc: libc::c_int,
                argv: *mut *mut libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, argc, argv)
    }
    #[doc = "* Process (or re-process) service configuration requests that are\n   * provided in the svc.conf file(s).  Returns the number of errors\n   * that occurred."]
    pub unsafe fn process_directives(
        __this: *mut Self,
        mut defunct_option: bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt18process_directivesEb"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                defunct_option: bool,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, defunct_option)
    }
    /**Tidy up and perform last rites when ACE_Service_Config is shut
  /// down.  This method calls @c close_svcs.  Returns 0.*/
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt5closeEv"]
            fn __ext(__this: *mut ACE_Service_Gestalt) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt)
    }
    ///Registers a service descriptor for a static service object
    pub unsafe fn insert(
        __this: *mut Self,
        mut stsd: *mut ACE_Static_Svc_Descriptor,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt6insertEP25ACE_Static_Svc_Descriptor"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                stsd: *mut ACE_Static_Svc_Descriptor,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, stsd)
    }
    /**Dynamically link the shared object file and retrieve a pointer to
  /// the designated shared object in this file. Also account for the
  /// possibility to have static services registered when loading the DLL, by
  /// ensuring that the dynamic service is registered before any of its
  /// subordinate static services. Thus avoiding any finalization order
  /// problems.*/
    pub unsafe fn initialize(
        __this: *mut Self,
        mut _anon_0: *const ACE_Service_Type_Factory,
        mut parameters: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt10initializeEPK24ACE_Service_Type_FactoryPKc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                _anon_0: *const ACE_Service_Type_Factory,
                parameters: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, _anon_0, parameters)
    }
    /**Dynamically link the shared object file and retrieve a pointer to
  /// the designated shared object in this file.
  /// @deprecated
  /// @note This is error-prone in the presense of dynamic services,
  /// which in turn initialize their own static services. This method
  /// will allow those static services to register *before* the dynamic
  /// service that owns them.  Upon finalization of the static services
  /// the process will typically crash, because the dynamic service's
  /// DLL may have been already released, together with the memory in
  /// which the static services reside.  It may not crash, for
  /// instance, when the first static service to register is the same
  /// as the dynamic service being loaded. You should be so lucky!*/
    pub unsafe fn initialize_ua4f900f5d00da189(
        __this: *mut Self,
        mut _anon_0: *const ACE_Service_Type,
        mut parameters: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt10initializeEPK16ACE_Service_TypePKc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                _anon_0: *const ACE_Service_Type,
                parameters: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, _anon_0, parameters)
    }
    ///Initialize and activate a statically @a svc_name service.
    pub unsafe fn initialize_u32660f5ed5aef539(
        __this: *mut Self,
        mut svc_name: *const libc::c_char,
        mut parameters: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt10initializeEPKcS1_"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                svc_name: *const libc::c_char,
                parameters: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, svc_name, parameters)
    }
    /**Resume a @a svc_name that was previously suspended or has not yet
  /// been resumed (e.g., a static service).*/
    pub unsafe fn resume(
        __this: *mut Self,
        mut svc_name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt6resumeEPKc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                svc_name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, svc_name)
    }
    #[doc = "* Suspend @a svc_name.  Note that this will not unlink the service\n   * from the daemon if it was dynamically linked, it will mark it as\n   * being suspended in the Service Repository and call the @c suspend()\n   * member function on the appropriate ACE_Service_Object.  A\n   * service can be resumed later on by calling the @c resume() member\n   * function..."]
    pub unsafe fn suspend(
        __this: *mut Self,
        mut svc_name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt7suspendEPKc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                svc_name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, svc_name)
    }
    /**Totally remove @a svc_name from the daemon by removing it
  /// from the ACE_Reactor, and unlinking it if necessary.*/
    pub unsafe fn remove(
        __this: *mut Self,
        mut svc_name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt6removeEPKc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                svc_name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, svc_name)
    }
    #[doc = "* Using the supplied name, finds and (if needed) returns a pointer to a\n   * static service descriptor. Returns 0 for success and -1 for failure"]
    pub unsafe fn find_static_svc_descriptor(
        __this: *const Self,
        mut name: *const libc::c_char,
        mut ssd: *mut *mut ACE_Static_Svc_Descriptor,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK19ACE_Service_Gestalt26find_static_svc_descriptorEPKcPP25ACE_Static_Svc_Descriptor"]
            fn __ext(
                __this: *const ACE_Service_Gestalt,
                name: *const libc::c_char,
                ssd: *mut *mut ACE_Static_Svc_Descriptor,
            ) -> libc::c_int;
        }
        __ext(__this as *const ACE_Service_Gestalt, name, ssd)
    }
    ///Get the current ACE_Service_Repository held by this object.
    pub unsafe fn current_service_repository(
        __this: *mut Self,
    ) -> *mut ACE_Service_Repository {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).repo_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn parse_args_i(
        __this: *mut Self,
        mut _anon_0: libc::c_int,
        mut argv: *mut *mut libc::c_char,
        mut ignore_default_svc_conf_file: *mut bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt12parse_args_iEiPPcRb"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                _anon_0: libc::c_int,
                argv: *mut *mut libc::c_char,
                ignore_default_svc_conf_file: *mut bool,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_Service_Gestalt,
            _anon_0,
            argv,
            ignore_default_svc_conf_file,
        )
    }
    #[doc = "* Performs an open without parsing command-line arguments.  The @a\n   * logger_key indicates where to write the logging output, which is\n   * typically either a STREAM pipe or a socket address.  If @a\n   * ignore_default_svc_conf_file is non-0 then the \"svc.conf\" file\n   * will not be added by default.  If @a ignore_debug_flag is non-0\n   * then the application is responsible for setting the @c\n   * ACE_Log_Msg::priority_mask() appropriately.  Returns number of\n   * errors that occurred on failure and 0 otherwise."]
    pub unsafe fn open_i(
        __this: *mut Self,
        mut program_name: *const libc::c_char,
        mut logger_key: *const libc::c_char,
        mut ignore_static_svcs: bool,
        mut ignore_default_svc_conf_file: bool,
        mut ignore_debug_flag: bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt6open_iEPKcS1_bbb"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                program_name: *const libc::c_char,
                logger_key: *const libc::c_char,
                ignore_static_svcs: bool,
                ignore_default_svc_conf_file: bool,
                ignore_debug_flag: bool,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_Service_Gestalt,
            program_name,
            logger_key,
            ignore_static_svcs,
            ignore_default_svc_conf_file,
            ignore_debug_flag,
        )
    }
    ///Initialize the @c svc_conf_file_queue_ if necessary.
    pub unsafe fn init_svc_conf_file_queue(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt24init_svc_conf_file_queueEv"]
            fn __ext(__this: *mut ACE_Service_Gestalt) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt)
    }
    /**Add the default statically-linked services to the
  /// ACE_Service_Repository.*/
    pub unsafe fn load_static_svcs(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt16load_static_svcsEv"]
            fn __ext(__this: *mut ACE_Service_Gestalt) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt)
    }
    /**Process service configuration requests that were provided on the
  /// command-line.  Returns the number of errors that occurred.*/
    pub unsafe fn process_commandline_directives(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt30process_commandline_directivesEv"]
            fn __ext(__this: *mut ACE_Service_Gestalt) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt)
    }
    /**Process a static directive without also inserting its descriptor
  /// the global table. This avoids multiple additions when processing
  /// directives in non-global gestalts.*/
    pub unsafe fn process_directive_i(
        __this: *mut Self,
        mut ssd: *const ACE_Static_Svc_Descriptor,
        mut force_replace: bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt19process_directive_iERK25ACE_Static_Svc_Descriptorb"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                ssd: *const ACE_Static_Svc_Descriptor,
                force_replace: bool,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, ssd, force_replace)
    }
    /**This is the implementation function that process_directives()
  /// and process_directive() both call.  Returns the number of errors
  /// that occurred.*/
    pub unsafe fn process_directives_i(
        __this: *mut Self,
        mut param: *mut ACE_Svc_Conf_Param,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt20process_directives_iEP18ACE_Svc_Conf_Param"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                param: *mut ACE_Svc_Conf_Param,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, param)
    }
    /**Dynamically link the shared object file and retrieve a pointer to
  /// the designated shared object in this file.*/
    pub unsafe fn initialize_i(
        __this: *mut Self,
        mut sr: *const ACE_Service_Type,
        mut parameters: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt12initialize_iEPK16ACE_Service_TypePKc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                sr: *const ACE_Service_Type,
                parameters: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt, sr, parameters)
    }
    pub unsafe fn find_processed_static_svc(
        __this: *mut Self,
        mut _anon_0: *const libc::c_char,
    ) -> *const ACE_Static_Svc_Descriptor {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt25find_processed_static_svcEPKc"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                _anon_0: *const libc::c_char,
            ) -> *const ACE_Static_Svc_Descriptor;
        }
        __ext(__this as *mut ACE_Service_Gestalt, _anon_0)
    }
    pub unsafe fn add_processed_static_svc(
        __this: *mut Self,
        mut _anon_0: *const ACE_Static_Svc_Descriptor,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt24add_processed_static_svcEPK25ACE_Static_Svc_Descriptor"]
            fn __ext(
                __this: *mut ACE_Service_Gestalt,
                _anon_0: *const ACE_Static_Svc_Descriptor,
            );
        }
        __ext(__this as *mut ACE_Service_Gestalt, _anon_0)
    }
    /**Performs the common initialization tasks for a new or previously
  /// closed instance. Must not be virtual, as it is called from the
  /// constructor.*/
    pub unsafe fn init_i(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt6init_iEv"]
            fn __ext(__this: *mut ACE_Service_Gestalt) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Gestalt)
    }
    pub unsafe fn intrusive_add_ref(mut _anon_0: *mut ACE_Service_Gestalt) {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt17intrusive_add_refEPS_"]
            fn __ext(_anon_0: *mut ACE_Service_Gestalt);
        }
        __ext(_anon_0)
    }
    pub unsafe fn intrusive_remove_ref(mut _anon_0: *mut ACE_Service_Gestalt) {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt20intrusive_remove_refEPS_"]
            fn __ext(_anon_0: *mut ACE_Service_Gestalt);
        }
        __ext(_anon_0)
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
impl ACE_String_Base_wchar_t_ {
    #[doc = "*  Default constructor.\n    *\n    *  @param the_allocator ACE_Allocator associated with string\n    *  @return Default ACE_String_Base string."]
    pub unsafe fn new_at_se57ca8063130c40c(
        __this: *mut Self,
        mut the_allocator: *mut ACE_Allocator,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).allocator_),
                if (!(the_allocator).is_null()) {
                    the_allocator
                } else {
                    <ACE_Allocator>::instance()
                },
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).buf_len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).rep_),
                ::core::ptr::addr_of_mut!(ACE_String_Base_wchar_t__NULL_String_)
                    as *mut libc::wchar_t,
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).release_), false);
            {}
            ()
        }
    }
    pub unsafe fn new_se57ca8063130c40c(mut __a0: *mut ACE_Allocator) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_se57ca8063130c40c(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    #[doc = "*  Copy @a len bytes of @a s (will zero terminate the result).\n   *\n   * If release == true then a new buffer is allocated internally if the\n   *   existing one is not big enough to hold s. If the existing\n   *   buffer is big enough, then it will be used. This means that\n   *   set(*, *, 1) is illegal when the string is constructed with a\n   *   non-owned const char*. (e.g. ACE_String_Base(\"test\", 0, 0))\n   *\n   * If release == false then the s buffer is used directly, and any\n   *   existing buffer is destroyed. If s == 0 then it will _not_ be\n   *   used, and instead the internal buffer is set to NULL_String_.\n   *\n   *  @param s Non-zero terminated input string\n   *  @param len Length of input string 's'\n   *  @param release Allocator responsible(true)/not responsible(false) for\n   *    freeing memory."]
    pub unsafe fn set_s62e3ada53e3eb77a(
        __this: *mut Self,
        mut s: *const libc::wchar_t,
        mut len: libc::c_ulong,
        mut release: bool,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut new_buf_len: libc::c_ulong = (((len) as libc::c_ulong))
                    .wrapping_add((1) as libc::c_ulong);
                if ((((((((((((((((!(s).is_null()) as libc::c_int) as libc::c_int) != 0)
                    && (((((len as libc::c_ulong)) != (((0) as libc::c_ulong)))
                        as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0) && ((release as libc::c_int) != 0))
                    as libc::c_int) as libc::c_int) != 0)
                    && ((((((*__this).buf_len_ as libc::c_ulong))
                        < (((new_buf_len) as libc::c_ulong))) as libc::c_int
                        as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                {
                    let mut temp: *mut libc::wchar_t = ((0) as *mut libc::wchar_t);
                    'dowhile_0: loop {
                        'cont_0: loop {
                            {
                                {
                                    temp = ({
                                        let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                                            as *mut ACE_Allocator;
                                        let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                                            as *const *const __Vtbl_uf2113694993e252c);
                                        ((*__vt)
                                            .vfn_u685215409e23bf32)(
                                            __obj,
                                            (((((new_buf_len) as libc::c_ulong))
                                                .wrapping_mul((4) as libc::c_ulong)) as libc::c_ulong),
                                        )
                                    } as *mut libc::wchar_t);
                                    if (((((temp).is_null()) as libc::c_int) as libc::c_int)
                                        != 0)
                                    {
                                        ((*(__errno_location()))) = 12;
                                        return;
                                    }
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                        if !(((0) != 0)) {
                            break 'dowhile_0;
                        }
                    }
                    if ((((((((((*__this).buf_len_ as libc::c_ulong))
                        != (((0) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                        && (((*__this).release_ as libc::c_int) != 0)) as libc::c_int)
                        as libc::c_int) != 0)
                    {
                        {
                            let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                                as *mut ACE_Allocator;
                            let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                                as *const *const __Vtbl_uf2113694993e252c);
                            ((*__vt)
                                .vfn_ucc7a27ee055bb87e)(
                                __obj,
                                (((*__this).rep_) as *mut libc::c_void),
                            )
                        };
                    }
                    (*__this).rep_ = temp;
                    (*__this).buf_len_ = new_buf_len;
                    (*__this).release_ = true;
                    (*__this).len_ = len;
                    ACE_OS::memcpy_u6033eb81edaf9212(
                        (((*__this).rep_) as *mut libc::c_void),
                        ((s) as *const libc::c_void),
                        (((((len) as libc::c_ulong)).wrapping_mul((4) as libc::c_ulong))
                            as libc::c_ulong),
                    );
                    (*((*__this).rep_).wrapping_offset((len) as isize)) = ((0)
                        as libc::wchar_t);
                } else {
                    if (((((((((((!(((release as libc::c_int) != 0)) as libc::c_int)
                        as libc::c_int) != 0)
                        || (((((s).is_null()) as libc::c_int) as libc::c_int) != 0))
                        as libc::c_int) as libc::c_int) != 0)
                        || (((((len as libc::c_ulong)) == (((0) as libc::c_ulong)))
                            as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                        as libc::c_int) != 0)
                    {
                        if ((((((((((*__this).buf_len_ as libc::c_ulong))
                            != (((0) as libc::c_ulong))) as libc::c_int as libc::c_int)
                            != 0) && (((*__this).release_ as libc::c_int) != 0))
                            as libc::c_int) as libc::c_int) != 0)
                        {
                            {
                                let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                                    as *mut ACE_Allocator;
                                let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                                    as *const *const __Vtbl_uf2113694993e252c);
                                ((*__vt)
                                    .vfn_ucc7a27ee055bb87e)(
                                    __obj,
                                    (((*__this).rep_) as *mut libc::c_void),
                                )
                            };
                            (*__this).release_ = false;
                        }
                    }
                    if (((((((((s).is_null()) as libc::c_int) as libc::c_int) != 0)
                        || (((((len as libc::c_ulong)) == (((0) as libc::c_ulong)))
                            as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                        as libc::c_int) != 0)
                    {
                        (*__this).buf_len_ = ((0) as libc::c_ulong);
                        (*__this).len_ = ((0) as libc::c_ulong);
                        (*__this).rep_ = ::core::ptr::addr_of_mut!(
                            ACE_String_Base_wchar_t__NULL_String_
                        ) as *mut libc::wchar_t;
                        (*__this).release_ = false;
                    } else {
                        if (((!(((release as libc::c_int) != 0)) as libc::c_int)
                            as libc::c_int) != 0)
                        {
                            (*__this).buf_len_ = len;
                            (*__this).len_ = len;
                            (*__this).rep_ = (s as *mut libc::wchar_t);
                            (*__this).release_ = false;
                        } else {
                            ACE_OS::memcpy_u6033eb81edaf9212(
                                (((*__this).rep_) as *mut libc::c_void),
                                ((s) as *const libc::c_void),
                                (((((len) as libc::c_ulong))
                                    .wrapping_mul((4) as libc::c_ulong)) as libc::c_ulong),
                            );
                            (*((*__this).rep_).wrapping_offset((len) as isize)) = ((0)
                                as libc::wchar_t);
                            (*__this).len_ = len;
                        }
                    }
                }
            }
            ()
        }
    }
    #[doc = "* Constructor that copies @a len CHARs of @a s into dynamically\n   * allocated memory (will zero terminate the result).\n   *\n   * if release == true then a new buffer is allocated internally.\n   *   s is copied to the internal buffer.\n   * if release == false then the s buffer is used directly. If s == 0\n   *   then it will _not_ be used, and instead the internal buffer\n   *   is set to NULL_String_.\n   *\n   * @param s Non-zero terminated input string\n   * @param len Length of non-zero terminated input string\n   * @param the_allocator ACE_Allocator associated with string\n   * @param release Allocator responsible(true)/not responsible(false) for\n   *    freeing memory.\n   * @return ACE_String_Base containing const ACE_CHAR_T *s"]
    pub unsafe fn new_at_s591dcb439e3dca6a(
        __this: *mut Self,
        mut s: *const libc::wchar_t,
        mut len: libc::c_ulong,
        mut the_allocator: *mut ACE_Allocator,
        mut release: bool,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).allocator_),
                if (!(the_allocator).is_null()) {
                    the_allocator
                } else {
                    <ACE_Allocator>::instance()
                },
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).buf_len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).rep_),
                ((0) as *mut libc::wchar_t),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).release_), false);
            {
                <ACE_String_Base_wchar_t_>::set_s62e3ada53e3eb77a(
                    (__this) as *mut ACE_String_Base_wchar_t_,
                    s,
                    len,
                    release,
                );
            }
            ()
        }
    }
    pub unsafe fn new_s591dcb439e3dca6a(
        mut __a0: *const libc::wchar_t,
        mut __a1: libc::c_ulong,
        mut __a2: *mut ACE_Allocator,
        mut __a3: bool,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s591dcb439e3dca6a(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
            __a3,
        );
        __obj
    }
    #[doc = "* Copy @a s into this @a ACE_String_Base.\n   *\n   * If release == true then a new buffer is allocated internally if the\n   *   existing one is not big enough to hold s. If the existing\n   *   buffer is big enough, then it will be used. This means that\n   *   set(*, 1) can be illegal when the string is constructed with a\n   *   const char*. (e.g. ACE_String_Base(\"test\", 0, false)).\n   *\n   * if release == false then the s buffer is used directly, and any\n   *   existing buffer is destroyed. If s == 0 then it will _not_ be\n   *   used, and instead the internal buffer is set to NULL_String_.\n   *\n   * @param s Null terminated input string\n   * @param release Allocator responsible(true)/not responsible(false) for\n   *    freeing memory."]
    pub unsafe fn set_s6282c945bf4b6f29(
        __this: *mut Self,
        mut s: *const libc::wchar_t,
        mut release: bool,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut length: libc::c_ulong = ((0) as libc::c_ulong);
                if ((((!(s).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    length = ACE_OS::strlen_u07b44aa22513a9ba(s);
                }
                <ACE_String_Base_wchar_t_>::set_s62e3ada53e3eb77a(
                    (__this) as *mut ACE_String_Base_wchar_t_,
                    s,
                    ((length) as libc::c_ulong),
                    release,
                );
            }
            ()
        }
    }
    #[doc = "* Constructor that copies @a s into dynamically allocated memory.\n   *\n   * if release == true then a new buffer is allocated internally, and\n   *   s is copied to the internal buffer.\n   * if release == false then the s buffer is used directly. If s == 0\n   *   then it will _not_ be used, and instead the internal buffer\n   *   is set to NULL_String_.\n   *\n   * @param s Zero terminated input string\n   * @param the_allocator ACE_Allocator associated with string\n   * @param release Allocator responsible(true)/not responsible(false) for\n   *    freeing memory.\n   * @return ACE_String_Base containing const ACE_CHAR_T *s"]
    pub unsafe fn new_at_s44bc67f7d5665fa3(
        __this: *mut Self,
        mut s: *const libc::wchar_t,
        mut the_allocator: *mut ACE_Allocator,
        mut release: bool,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).allocator_),
                if (!(the_allocator).is_null()) {
                    the_allocator
                } else {
                    <ACE_Allocator>::instance()
                },
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).buf_len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).rep_),
                ((0) as *mut libc::wchar_t),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).release_), false);
            {
                <ACE_String_Base_wchar_t_>::set_s6282c945bf4b6f29(
                    (__this) as *mut ACE_String_Base_wchar_t_,
                    s,
                    release,
                );
            }
            ()
        }
    }
    pub unsafe fn new_s44bc67f7d5665fa3(
        mut __a0: *const libc::wchar_t,
        mut __a1: *mut ACE_Allocator,
        mut __a2: bool,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s44bc67f7d5665fa3(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    pub unsafe fn fast_resize(__this: *mut Self, mut len: libc::c_ulong) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).buf_len_ as libc::c_ulong))
                    <= (((len) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                {
                    if ((((((((((*__this).buf_len_ as libc::c_ulong))
                        != (((0) as libc::c_ulong))) as libc::c_int as libc::c_int) != 0)
                        && (((*__this).release_ as libc::c_int) != 0)) as libc::c_int)
                        as libc::c_int) != 0)
                    {
                        {
                            let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                                as *mut ACE_Allocator;
                            let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                                as *const *const __Vtbl_uf2113694993e252c);
                            ((*__vt)
                                .vfn_ucc7a27ee055bb87e)(
                                __obj,
                                (((*__this).rep_) as *mut libc::c_void),
                            )
                        };
                    }
                    (*__this).rep_ = ({
                        let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                            as *mut ACE_Allocator;
                        let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                            as *const *const __Vtbl_uf2113694993e252c);
                        ((*__vt)
                            .vfn_u685215409e23bf32)(
                            __obj,
                            (((((((((len) as libc::c_ulong))
                                .wrapping_add((1) as libc::c_ulong))) as libc::c_ulong))
                                .wrapping_mul((4) as libc::c_ulong)) as libc::c_ulong),
                        )
                    } as *mut libc::wchar_t);
                    (*__this).buf_len_ = (((((len) as libc::c_ulong))
                        .wrapping_add((1) as libc::c_ulong)) as libc::c_ulong);
                    (*__this).release_ = true;
                }
                (*__this).len_ = ((0) as libc::c_ulong);
                if (((((len as libc::c_ulong)) > (((0) as libc::c_ulong))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    (*((*__this).rep_).wrapping_offset((0) as isize)) = ((0)
                        as libc::wchar_t);
                }
            }
            ()
        }
    }
    #[doc = "* This method is designed for high-performance. Please use with\n   * care ;-)\n   *\n   * Warning : This method was documented incorrectly in the past.\n   * The original intention was to change the length of the string to\n   * len, and to fill the whole thing with c CHARs.\n   * However, what was actually done was to set the length of the\n   * string to zero, and fill the buffer with c's. The buffer was\n   * also not null-terminated unless c happened to be zero.\n   * Rather than fix the method to work as documented, the code is\n   * left as is, but the second parameter should probably not be used.\n   *\n   * fast_resize just adjusts the buffer if needed and sets the length,\n   * it doesn't fill the buffer, so is much faster.\n   *\n   * @param len The number of CHARs to reserve\n   * @param c The ACE_CHAR_T to use when filling the string."]
    pub unsafe fn resize(
        __this: *mut Self,
        mut len: libc::c_ulong,
        mut c: libc::wchar_t,
    ) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_String_Base_wchar_t_>::fast_resize(
                    (__this) as *mut ACE_String_Base_wchar_t_,
                    ((len) as libc::c_ulong),
                );
                ACE_OS::memset_u2b5dfc47d301370a(
                    (((*__this).rep_) as *mut libc::c_void),
                    ((c) as libc::c_int),
                    ((((((*__this).buf_len_) as libc::c_ulong))
                        .wrapping_mul((4) as libc::c_ulong)) as libc::c_ulong),
                );
            }
            ()
        }
    }
    #[doc = "*  Constructor that allocates a len long string.\n   *\n   *  Warning : This constructor was incorrectly documented in the past.\n   *  It simply calls resize(len, c).\n   *  It is probably not advisable to use the second parameter. See\n   *  resize() for more information.\n   *\n   *  @param len Amount of space to reserve for the string.\n   *  @param c The array is filled with c's\n   *  @param the_allocator ACE_Allocator associated with string\n   *  @return Empty ACE_String_Base with room for len CHARs"]
    pub unsafe fn new_at_sb8ac398513396d8a(
        __this: *mut Self,
        mut len: libc::c_ulong,
        mut c: libc::wchar_t,
        mut the_allocator: *mut ACE_Allocator,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).allocator_),
                if (!(the_allocator).is_null()) {
                    the_allocator
                } else {
                    <ACE_Allocator>::instance()
                },
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).buf_len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).rep_),
                ((0) as *mut libc::wchar_t),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).release_), false);
            {
                <ACE_String_Base_wchar_t_>::resize(
                    (__this) as *mut ACE_String_Base_wchar_t_,
                    len,
                    c,
                );
            }
            ()
        }
    }
    pub unsafe fn new_sb8ac398513396d8a(
        mut __a0: libc::c_ulong,
        mut __a1: libc::wchar_t,
        mut __a2: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb8ac398513396d8a(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    #[doc = "*  Copy constructor.\n   *\n   *  @param s Input ACE_String_Base string to copy\n   *  @return Copy of input string @a s"]
    pub unsafe fn new_at_sd240ab6a8d28430f(
        __this: *mut Self,
        mut s: *const ACE_String_Base_wchar_t_,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).allocator_),
                if (!((*s).allocator_).is_null()) {
                    (((*s).allocator_) as *mut ACE_Allocator)
                } else {
                    <ACE_Allocator>::instance()
                },
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).buf_len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).rep_),
                ((0) as *mut libc::wchar_t),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).release_), false);
            {
                <ACE_String_Base_wchar_t_>::set_s62e3ada53e3eb77a(
                    (__this) as *mut ACE_String_Base_wchar_t_,
                    (((*s).rep_) as *const libc::wchar_t),
                    (((*s).len_) as libc::c_ulong),
                    true,
                );
            }
            ()
        }
    }
    pub unsafe fn new_sd240ab6a8d28430f(
        mut __a0: *const ACE_String_Base_wchar_t_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sd240ab6a8d28430f(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    #[doc = "*  Constructor that copies @a c into dynamically allocated memory.\n   *\n   *  @param c Single input character.\n   *  @param the_allocator ACE_Allocator associated with string\n   *  @return ACE_String_Base containing ACE_CHAR_T 'c'"]
    pub unsafe fn new_at_sa2905e920df89523(
        __this: *mut Self,
        mut c: libc::wchar_t,
        mut the_allocator: *mut ACE_Allocator,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).allocator_),
                if (!(the_allocator).is_null()) {
                    the_allocator
                } else {
                    <ACE_Allocator>::instance()
                },
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).buf_len_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).rep_),
                ((0) as *mut libc::wchar_t),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).release_), false);
            {
                <ACE_String_Base_wchar_t_>::set_s62e3ada53e3eb77a(
                    (__this) as *mut ACE_String_Base_wchar_t_,
                    ((::core::ptr::addr_of_mut!(c) as *mut libc::wchar_t)
                        as *const libc::wchar_t),
                    ((1) as libc::c_ulong),
                    true,
                );
            }
            ()
        }
    }
    pub unsafe fn new_sa2905e920df89523(
        mut __a0: libc::wchar_t,
        mut __a1: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sa2905e920df89523(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    #[doc = "*  Assignment operator (does copy memory).\n   *\n   *  @param s Input ACE_String_Base string to assign to this object.\n   *  @return Return a copy of the this string."]
    pub unsafe fn operator_assign_s41b8627fed4b436a(
        __this: *mut Self,
        mut s: *const ACE_String_Base_wchar_t_,
    ) -> *mut ACE_String_Base_wchar_t_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((__this) as *const u8))
                    != (((((::core::ptr::addr_of!((* s))
                        as *const ACE_String_Base_wchar_t_)
                        as *mut ACE_String_Base_wchar_t_)) as *const u8))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    <ACE_String_Base_wchar_t_>::set_s62e3ada53e3eb77a(
                        (__this) as *mut ACE_String_Base_wchar_t_,
                        (((*s).rep_) as *const libc::wchar_t),
                        (((*s).len_) as libc::c_ulong),
                        true,
                    );
                }
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "*  Append function (copies memory).\n   *\n   *  @param s Input ACE_CHAR_T array to concatenate to this string.\n   *  @param slen The length of the array.\n   *  @return The combined string (input append to the end of the old). New\n   *    string is zero terminated."]
    pub unsafe fn append(
        __this: *mut Self,
        mut s: *const libc::wchar_t,
        mut slen: libc::c_ulong,
    ) -> *mut ACE_String_Base_wchar_t_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((((((slen as libc::c_ulong)) > (((0) as libc::c_ulong)))
                    as libc::c_int as libc::c_int) != 0)
                    && (((((slen as libc::c_ulong))
                        != (((ACE_String_Base_Const_npos) as libc::c_ulong)))
                        as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    if ((((((*__this).buf_len_ as libc::c_ulong))
                        >= ((((((((((*__this).len_) as libc::c_ulong))
                            .wrapping_add((slen) as libc::c_ulong)) as libc::c_ulong))
                            .wrapping_add((1) as libc::c_ulong)) as libc::c_ulong)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        ACE_OS::memcpy_u6033eb81edaf9212(
                            ((((*__this).rep_)
                                .wrapping_offset(((*__this).len_) as isize))
                                as *mut libc::c_void),
                            ((s) as *const libc::c_void),
                            (((((slen) as libc::c_ulong))
                                .wrapping_mul((4) as libc::c_ulong)) as libc::c_ulong),
                        );
                    } else {
                        let mut new_buf_len: libc::c_ulong = (((*ace_max___const_unsigned_long____const_unsigned_long___(
                            &((((((((*__this).len_) as libc::c_ulong))
                                .wrapping_add((slen) as libc::c_ulong)) as libc::c_ulong))
                                .wrapping_add((1) as libc::c_ulong))
                                as *const libc::c_ulong,
                            &(((((*__this).buf_len_) as libc::c_ulong))
                                .wrapping_add(
                                    (((((*__this).buf_len_) as libc::c_ulong))
                                        / ((2) as libc::c_ulong)) as libc::c_ulong,
                                )) as *const libc::c_ulong,
                        ))) as libc::c_ulong);
                        let mut t: *mut libc::wchar_t = ((0) as *mut libc::wchar_t);
                        'dowhile_0: loop {
                            'cont_0: loop {
                                {
                                    {
                                        t = ({
                                            let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                                                as *mut ACE_Allocator;
                                            let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                                                as *const *const __Vtbl_uf2113694993e252c);
                                            ((*__vt)
                                                .vfn_u685215409e23bf32)(
                                                __obj,
                                                (((((new_buf_len) as libc::c_ulong))
                                                    .wrapping_mul((4) as libc::c_ulong)) as libc::c_ulong),
                                            )
                                        } as *mut libc::wchar_t);
                                        if (((((t).is_null()) as libc::c_int) as libc::c_int) != 0)
                                        {
                                            ((*(__errno_location()))) = 12;
                                            return __this;
                                        }
                                    }
                                }
                                #[allow(unreachable_code)] break 'cont_0;
                            }
                            if !(((0) != 0)) {
                                break 'dowhile_0;
                            }
                        }
                        ACE_OS::memcpy_u6033eb81edaf9212(
                            ((t) as *mut libc::c_void),
                            (((*__this).rep_) as *const libc::c_void),
                            ((((((*__this).len_) as libc::c_ulong))
                                .wrapping_mul((4) as libc::c_ulong)) as libc::c_ulong),
                        );
                        ACE_OS::memcpy_u6033eb81edaf9212(
                            (((t).wrapping_offset(((*__this).len_) as isize))
                                as *mut libc::c_void),
                            ((s) as *const libc::c_void),
                            (((((slen) as libc::c_ulong))
                                .wrapping_mul((4) as libc::c_ulong)) as libc::c_ulong),
                        );
                        if ((((((((((*__this).buf_len_ as libc::c_ulong))
                            != (((0) as libc::c_ulong))) as libc::c_int as libc::c_int)
                            != 0) && (((*__this).release_ as libc::c_int) != 0))
                            as libc::c_int) as libc::c_int) != 0)
                        {
                            {
                                let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                                    as *mut ACE_Allocator;
                                let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                                    as *const *const __Vtbl_uf2113694993e252c);
                                ((*__vt)
                                    .vfn_ucc7a27ee055bb87e)(
                                    __obj,
                                    (((*__this).rep_) as *mut libc::c_void),
                                )
                            };
                        }
                        (*__this).release_ = true;
                        (*__this).rep_ = t;
                        (*__this).buf_len_ = ((new_buf_len) as libc::c_ulong);
                    }
                    {
                        let __lv = ::core::ptr::addr_of_mut!((* __this).len_);
                        unsafe {
                            *__lv = ((((*__lv)) as libc::c_ulong))
                                .wrapping_add((slen) as libc::c_ulong);
                            *__lv
                        }
                    };
                    (*((*__this).rep_).wrapping_offset(((*__this).len_) as isize)) = ((0)
                        as libc::wchar_t);
                }
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "*  Concat operator (copies memory).\n   *\n   *  @param s Input ACE_String_Base string to concatenate to this string.\n   *  @return The combined string (input append to the end of the old). New\n   *    string is zero terminated."]
    pub unsafe fn operator_add_assign_s41b8627fed4b436a(
        __this: *mut Self,
        mut s: *const ACE_String_Base_wchar_t_,
    ) -> *mut ACE_String_Base_wchar_t_ {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (<ACE_String_Base_wchar_t_>::append(
                    (__this) as *mut ACE_String_Base_wchar_t_,
                    (((*s).rep_) as *const libc::wchar_t),
                    (((*s).len_) as libc::c_ulong),
                )) as *mut ACE_String_Base_wchar_t_;
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
impl ACE_NS_WString {
    ///Default constructor.
    pub unsafe fn new_at(__this: *mut Self, mut alloc: *mut ACE_Allocator) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_String_Base_wchar_t_>::new_at_se57ca8063130c40c(
                (::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_WString>())
                    as *mut ACE_String_Base_wchar_t_,
                alloc,
            );
            {}
            ()
        }
    }
    pub unsafe fn new(mut __a0: *mut ACE_Allocator) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Constructor that copies @a s into dynamically allocated memory.
    pub unsafe fn new_at_u818746305c55f813(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_NS_WStringC1EPKcP13ACE_Allocator"]
            fn __ext(
                __this: *mut ACE_NS_WString,
                __a0: *const libc::c_char,
                __a1: *mut ACE_Allocator,
            );
        }
        __ext(__this as *mut ACE_NS_WString, __a0, __a1)
    }
    pub unsafe fn new_u818746305c55f813(
        mut __a0: *const libc::c_char,
        mut __a1: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u818746305c55f813(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Constructor that copies @a s into dynamically allocated memory.
    pub unsafe fn new_at_ua46d8be5d8861127(
        __this: *mut Self,
        mut s: *const libc::wchar_t,
        mut alloc: *mut ACE_Allocator,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_String_Base_wchar_t_>::new_at_s44bc67f7d5665fa3(
                (::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_WString>())
                    as *mut ACE_String_Base_wchar_t_,
                ((s) as *const libc::wchar_t),
                alloc,
                true,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_ua46d8be5d8861127(
        mut __a0: *const libc::wchar_t,
        mut __a1: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ua46d8be5d8861127(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Constructor that takes in a ushort16 string (mainly used by the
  /// ACE Name_Space classes)*/
    pub unsafe fn new_at_u51d40ed151379864(
        __this: *mut Self,
        mut __a0: *const libc::c_ushort,
        mut __a1: libc::c_ulong,
        mut __a2: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_NS_WStringC1EPKtmP13ACE_Allocator"]
            fn __ext(
                __this: *mut ACE_NS_WString,
                __a0: *const libc::c_ushort,
                __a1: libc::c_ulong,
                __a2: *mut ACE_Allocator,
            );
        }
        __ext(__this as *mut ACE_NS_WString, __a0, __a1, __a2)
    }
    pub unsafe fn new_u51d40ed151379864(
        mut __a0: *const libc::c_ushort,
        mut __a1: libc::c_ulong,
        mut __a2: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u51d40ed151379864(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    /**Constructor that copies @a len ACE_WSTRING_TYPE's of @a s into dynamically
  /// allocated memory (will NUL terminate the result).*/
    pub unsafe fn new_at_u235e7c59f6bf4af0(
        __this: *mut Self,
        mut s: *const libc::wchar_t,
        mut len: libc::c_ulong,
        mut alloc: *mut ACE_Allocator,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_String_Base_wchar_t_>::new_at_s591dcb439e3dca6a(
                (::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_WString>())
                    as *mut ACE_String_Base_wchar_t_,
                ((s) as *const libc::wchar_t),
                len,
                alloc,
                true,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u235e7c59f6bf4af0(
        mut __a0: *const libc::wchar_t,
        mut __a1: libc::c_ulong,
        mut __a2: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u235e7c59f6bf4af0(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    /**Constructor that dynamically allocates memory for @a len + 1
  /// ACE_WSTRING_TYPE characters. The newly created memory is set memset to 0.*/
    pub unsafe fn new_at_ucc1cd7240c01c0d9(
        __this: *mut Self,
        mut len: libc::c_ulong,
        mut alloc: *mut ACE_Allocator,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_String_Base_wchar_t_>::new_at_sb8ac398513396d8a(
                (::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_WString>())
                    as *mut ACE_String_Base_wchar_t_,
                len,
                ((0) as libc::wchar_t),
                alloc,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_ucc1cd7240c01c0d9(
        mut __a0: libc::c_ulong,
        mut __a1: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ucc1cd7240c01c0d9(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Copy constructor.
    pub unsafe fn new_at_u09f00c688053da5a(
        __this: *mut Self,
        mut s: *const ACE_NS_WString,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_String_Base_wchar_t_>::new_at_sd240ab6a8d28430f(
                (::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_WString>())
                    as *mut ACE_String_Base_wchar_t_,
                ::core::ptr::addr_of!(
                    (* ::core::ptr::addr_of!(((* s)).__base_0) .cast:: <
                    ACE_String_Base_wchar_t_ > ().cast_mut())
                ) as *const ACE_String_Base_wchar_t_,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u09f00c688053da5a(mut __a0: *const ACE_NS_WString) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u09f00c688053da5a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Constructor that copies @a c into dynamically allocated memory.
    pub unsafe fn new_at_u945b29bb3f117680(
        __this: *mut Self,
        mut c: libc::wchar_t,
        mut alloc: *mut ACE_Allocator,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_String_Base_wchar_t_>::new_at_sa2905e920df89523(
                (::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_WString>())
                    as *mut ACE_String_Base_wchar_t_,
                ((c) as libc::wchar_t),
                alloc,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_u945b29bb3f117680(
        mut __a0: libc::wchar_t,
        mut __a1: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u945b29bb3f117680(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Assignment operator (does copy memory).
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: *const ACE_NS_WString,
    ) -> *mut ACE_NS_WString {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                {
                    let __v = ((*::core::ptr::addr_of!(((* rhs)).__base_0)
                        .cast::<ACE_String_Base_wchar_t_>()
                        .cast_mut()))
                        .clone();
                    (*::core::ptr::addr_of!((* (__this)).__base_0)
                        .cast::<ACE_String_Base_wchar_t_>()
                        .cast_mut()) = __v;
                };
                return __this;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Transform into a copy of the ASCII character representation.
  /// (caller must delete)*/
    pub unsafe fn char_rep(__this: *const Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_NS_WString8char_repEv"]
            fn __ext(__this: *const ACE_NS_WString) -> *mut libc::c_char;
        }
        __ext(__this as *const ACE_NS_WString)
    }
    /**Transform into a copy of a USHORT16 representation (caller must
  /// delete).  Note, behavior is undefined when sizeof (wchar_t) != 2.*/
    pub unsafe fn ushort_rep(__this: *const Self) -> *mut libc::c_ushort {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_NS_WString10ushort_repEv"]
            fn __ext(__this: *const ACE_NS_WString) -> *mut libc::c_ushort;
        }
        __ext(__this as *const ACE_NS_WString)
    }
}
impl ACE_SString {
    ///Default constructor.
    pub unsafe fn new_at(__this: *mut Self, mut __a0: *mut ACE_Allocator) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_SStringC1EP13ACE_Allocator"]
            fn __ext(__this: *mut ACE_SString, __a0: *mut ACE_Allocator);
        }
        __ext(__this as *mut ACE_SString, __a0)
    }
    pub unsafe fn new(mut __a0: *mut ACE_Allocator) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Constructor that copies @a s into dynamically allocated memory.
    pub unsafe fn new_at_ua72d6fc86fb41dd5(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_SStringC1EPKcP13ACE_Allocator"]
            fn __ext(
                __this: *mut ACE_SString,
                __a0: *const libc::c_char,
                __a1: *mut ACE_Allocator,
            );
        }
        __ext(__this as *mut ACE_SString, __a0, __a1)
    }
    pub unsafe fn new_ua72d6fc86fb41dd5(
        mut __a0: *const libc::c_char,
        mut __a1: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ua72d6fc86fb41dd5(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Constructor that copies @a len chars of @a s into dynamically
  /// allocated memory (will NUL terminate the result).*/
    pub unsafe fn new_at_u9ecb07a69b4d06ba(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_SStringC1EPKcmP13ACE_Allocator"]
            fn __ext(
                __this: *mut ACE_SString,
                __a0: *const libc::c_char,
                __a1: libc::c_ulong,
                __a2: *mut ACE_Allocator,
            );
        }
        __ext(__this as *mut ACE_SString, __a0, __a1, __a2)
    }
    pub unsafe fn new_u9ecb07a69b4d06ba(
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u9ecb07a69b4d06ba(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    ///Copy constructor.
    pub unsafe fn new_at_u56e3a24ed0edea16(
        __this: *mut Self,
        mut __a0: *const ACE_SString,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_SStringC1ERKS_"]
            fn __ext(__this: *mut ACE_SString, __a0: *const ACE_SString);
        }
        __ext(__this as *mut ACE_SString, __a0)
    }
    pub unsafe fn new_u56e3a24ed0edea16(mut __a0: *const ACE_SString) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u56e3a24ed0edea16(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Constructor that copies @a c into dynamically allocated memory.
    pub unsafe fn new_at_u3e70a8842f1ed9da(
        __this: *mut Self,
        mut __a0: libc::c_char,
        mut __a1: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_SStringC1EcP13ACE_Allocator"]
            fn __ext(
                __this: *mut ACE_SString,
                __a0: libc::c_char,
                __a1: *mut ACE_Allocator,
            );
        }
        __ext(__this as *mut ACE_SString, __a0, __a1)
    }
    pub unsafe fn new_u3e70a8842f1ed9da(
        mut __a0: libc::c_char,
        mut __a1: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u3e70a8842f1ed9da(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Return the slot'th character in the string (doesn't perform
  /// bounds checking).*/
    pub unsafe fn operator_index(
        __this: *const Self,
        mut slot: libc::c_ulong,
    ) -> libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*((*__this).rep_).wrapping_offset((slot) as isize));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Return the slot'th character by reference in the string
  /// (doesn't perform bounds checking).*/
    pub unsafe fn operator_index_u4c0f9d266accc80c(
        __this: *mut Self,
        mut slot: libc::c_ulong,
    ) -> *mut libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of_mut!(
                    (* ((* __this).rep_).wrapping_offset((slot) as isize))
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Assignment operator (does copy memory).
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_SString,
    ) -> *mut ACE_SString {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_SStringaSERKS_"]
            fn __ext(
                __this: *mut ACE_SString,
                _anon_0: *const ACE_SString,
            ) -> *mut ACE_SString;
        }
        __ext(__this as *mut ACE_SString, _anon_0)
    }
    #[doc = "* Return a substring given an offset and length, if length == npos\n   * use rest of str return empty substring if offset or offset/length\n   * are invalid"]
    pub unsafe fn substring(
        __this: *const Self,
        mut offset: libc::c_ulong,
        mut length: libc::c_ulong,
    ) -> ACE_SString {
        extern "C-unwind" {
            #[link_name = "_ZNK11ACE_SString9substringEmm"]
            fn __ext(
                __this: *const ACE_SString,
                offset: libc::c_ulong,
                length: libc::c_ulong,
            ) -> ACE_SString;
        }
        __ext(__this as *const ACE_SString, offset, length)
    }
    ///Same as substring
    pub unsafe fn substr(
        __this: *const Self,
        mut offset: libc::c_ulong,
        mut length: libc::c_ulong,
    ) -> ACE_SString {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_SString>::substring(
                    (__this) as *const ACE_SString,
                    offset,
                    length,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Returns a hash value for this string.
    pub unsafe fn hash(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE::hash_pjw((((*__this).rep_) as *const libc::c_char));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the length of the string.
    pub unsafe fn length(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).len_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Set the underlying pointer.  Since this does not copy memory or
  /// delete existing memory use with extreme caution!!!*/
    pub unsafe fn rep(__this: *mut Self, mut s: *mut libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_SString3repEPc"]
            fn __ext(__this: *mut ACE_SString, s: *mut libc::c_char);
        }
        __ext(__this as *mut ACE_SString, s)
    }
    ///Get the underlying pointer.
    pub unsafe fn rep_u9cbb83dac80a7ea7(__this: *const Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).rep_) as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the underlying pointer.
    pub unsafe fn fast_rep(__this: *const Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).rep_) as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Same as STL String's c_str() and fast_rep().
    pub unsafe fn c_str(__this: *const Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).rep_) as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Comparison operator that will match substrings.  Returns the
  /// slot of the first location that matches, else @c npos.*/
    pub unsafe fn strstr(
        __this: *const Self,
        mut s: *const ACE_SString,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_SString>::find_u00b59e23aa6ddf6d(
                    (__this) as *const ACE_SString,
                    (((*s).rep_) as *const libc::c_char),
                    ((0) as libc::c_ulong),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Find @a str starting at pos.  Returns the slot of the first
  /// location that matches (will be >= pos), else npos.*/
    pub unsafe fn find(
        __this: *const Self,
        mut str: *const ACE_SString,
        mut pos: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_SString>::find_u00b59e23aa6ddf6d(
                    (__this) as *const ACE_SString,
                    (((*str).rep_) as *const libc::c_char),
                    pos,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Find @a s starting at pos.  Returns the slot of the first
  /// location that matches (will be >= pos), else npos.*/
    pub unsafe fn find_u00b59e23aa6ddf6d(
        __this: *const Self,
        mut s: *const libc::c_char,
        mut pos: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut substr: *mut libc::c_char = ((*__this).rep_)
                    .wrapping_offset((pos) as isize);
                let mut pointer: *mut libc::c_char = ACE_OS::strstr_u25e13fe23ca4c804(
                    substr,
                    s,
                );
                if (((((pointer).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return ((ACE_SString_npos) as libc::c_ulong);
                } else {
                    return (((pointer).offset_from((*__this).rep_)) as libc::c_long
                        as libc::c_ulong);
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Find @a c starting at pos.  Returns the slot of the first
  /// location that matches (will be >= pos), else npos.*/
    pub unsafe fn find_u2252c08f0db585ce(
        __this: *const Self,
        mut c: libc::c_char,
        mut pos: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut substr: *mut libc::c_char = ((*__this).rep_)
                    .wrapping_offset((pos) as isize);
                let mut pointer: *mut libc::c_char = ACE_OS::strchr_u824406bee5e3796b(
                    substr,
                    ((c) as libc::c_int),
                );
                if (((((pointer).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return ((ACE_SString_npos) as libc::c_ulong);
                } else {
                    return (((pointer).offset_from((*__this).rep_)) as libc::c_long
                        as libc::c_ulong);
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Find @a c starting at pos (counting from the end).  Returns the
  /// slot of the first location that matches, else npos.*/
    pub unsafe fn rfind(
        __this: *const Self,
        mut c: libc::c_char,
        mut pos: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((pos as libc::c_ulong))
                    == (((ACE_SString_npos) as libc::c_ulong))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    pos = (((*__this).len_) as libc::c_ulong);
                }
                {
                    let mut i: libc::c_ulong = pos;
                    'for_0: loop {
                        if !(((((({
                            let __lv = &mut (i);
                            let __r = *__lv;
                            *__lv = (*__lv).wrapping_sub(1);
                            __r
                        } as libc::c_ulong)) != (((0) as libc::c_ulong))) as libc::c_int
                            as libc::c_int) != 0))
                        {
                            break;
                        }
                        'cont_0: loop {
                            {
                                if ((((((*((*__this).rep_).wrapping_offset((i) as isize))
                                    as libc::c_int as libc::c_char))
                                    == (((c) as libc::c_int as libc::c_char))) as libc::c_int
                                    as libc::c_int) != 0)
                                {
                                    return ((i) as libc::c_ulong);
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                    }
                }
                return ((ACE_SString_npos) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Equality comparison operator (must match entire string).
    pub unsafe fn operator_eq(__this: *const Self, mut s: *const ACE_SString) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((((((((((*__this).len_ as libc::c_ulong))
                    == ((((*s).len_) as libc::c_ulong))) as libc::c_int as libc::c_int)
                    != 0)
                    && (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                        (((*__this).rep_) as *const libc::c_char),
                        (((*s).rep_) as *const libc::c_char),
                    ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Less than comparison operator.
    pub unsafe fn operator_lt(__this: *const Self, mut s: *const ACE_SString) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    if ((((((!((*__this).rep_).is_null()) && (!((*s).rep_).is_null()))
                        as libc::c_int)) as libc::c_int) != 0)
                    {
                        (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                            (((*__this).rep_) as *const libc::c_char),
                            (((*s).rep_) as *const libc::c_char),
                        ) as libc::c_int)) < (((0) as libc::c_int))) as libc::c_int
                            as libc::c_int) != 0)
                    } else {
                        (if (!(((*s).rep_)).is_null()) { true } else { false })
                    },
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Greater than comparison operator.
    pub unsafe fn operator_gt(__this: *const Self, mut s: *const ACE_SString) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return crate::__cxx_std::__Truthy::__truthy(
                    if ((((((!((*__this).rep_).is_null()) && (!((*s).rep_).is_null()))
                        as libc::c_int)) as libc::c_int) != 0)
                    {
                        (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                            (((*__this).rep_) as *const libc::c_char),
                            (((*s).rep_) as *const libc::c_char),
                        ) as libc::c_int)) > (((0) as libc::c_int))) as libc::c_int
                            as libc::c_int) != 0)
                    } else {
                        (if (!(((*__this).rep_)).is_null()) { true } else { false })
                    },
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Inequality comparison operator.
    pub unsafe fn operator_ne(__this: *const Self, mut s: *const ACE_SString) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((!((((<ACE_SString>::operator_eq(
                    (__this) as *const ACE_SString,
                    ::core::ptr::addr_of!((* s)),
                )) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Performs a strcmp()-style comparison.
    pub unsafe fn compare(
        __this: *const Self,
        mut s: *const ACE_SString,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::strcmp_u2f671283fc8b6d4a(
                    (((*__this).rep_) as *const libc::c_char),
                    (((*s).rep_) as *const libc::c_char),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK11ACE_SString4dumpEv"]
            fn __ext(__this: *const ACE_SString);
        }
        __ext(__this as *const ACE_SString)
    }
}
impl ACE_DLL {
    #[doc = "* Default constructor.  By default, the close() operation on the\n   * object will be invoked before it is destroyed.\n   * @param close_handle_on_destruction  Indicates whether or not the\n   *        close() method will be called to close an open DLL when this\n   *        object is destroyed. By default, close() will be called.\n   *        Set this parameter to false for situations where the DLL's lifetime\n   *        is controlled in a scope other than that of this ACE_DLL object.\n   *        For example, termination by ACE_DLL_Manager via ACE::fini()."]
    pub unsafe fn new_at(__this: *mut Self, mut __a0: bool) {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_DLLC1Eb"]
            fn __ext(__this: *mut ACE_DLL, __a0: bool);
        }
        __ext(__this as *mut ACE_DLL, __a0)
    }
    pub unsafe fn new(mut __a0: bool) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Allow assignment
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: *const ACE_DLL,
    ) -> *mut ACE_DLL {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_DLLaSERKS_"]
            fn __ext(__this: *mut ACE_DLL, rhs: *const ACE_DLL) -> *mut ACE_DLL;
        }
        __ext(__this as *mut ACE_DLL, rhs)
    }
    #[doc = "* This constructor performs the actions of open() during construction.\n   * @param dll_name  The name or path of the DLL to load.\n   * @param open_mode  Flags to alter the actions taken when loading the DLL.\n   *        The possible values are:\n   *        @li @c RTLD_LAZY (this the default): loads identifier symbols but\n   *            not the symbols for functions, which are loaded dynamically\n   *            on-demand.\n   *        @li @c RTLD_NOW: performs all necessary relocations when\n   *            @a dll_name is first loaded\n   *        @li RTLD_GLOBAL: makes symbols available for relocation\n   *            processing of any other DLLs.\n   * @param close_handle_on_destruction  Indicates whether or not the\n   *        close() method will be called to close an open DLL when this\n   *        object is destroyed. By default, close() will be called.\n   *        Set this parameter to 0 for situations where the DLL's lifetime\n   *        is controlled in a scope other than that of this ACE_DLL object.\n   *        For example, termination by ACE_DLL_Manager via ACE::fini()."]
    pub unsafe fn new_at_u08b225859092d161(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_int,
        mut __a2: bool,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_DLLC1EPKcib"]
            fn __ext(
                __this: *mut ACE_DLL,
                __a0: *const libc::c_char,
                __a1: libc::c_int,
                __a2: bool,
            );
        }
        __ext(__this as *mut ACE_DLL, __a0, __a1, __a2)
    }
    pub unsafe fn new_u08b225859092d161(
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_int,
        mut __a2: bool,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u08b225859092d161(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    ///Copy constructor.
    pub unsafe fn new_at_uee3c603b69ee15fa(__this: *mut Self, mut __a0: *const ACE_DLL) {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_DLLC1ERKS_"]
            fn __ext(__this: *mut ACE_DLL, __a0: *const ACE_DLL);
        }
        __ext(__this as *mut ACE_DLL, __a0)
    }
    pub unsafe fn new_uee3c603b69ee15fa(mut __a0: *const ACE_DLL) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_uee3c603b69ee15fa(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    #[doc = "* This method opens and dynamically links a specified DLL.\n   * @param dll_name  The filename or path of the DLL to load. ACE will\n   *        attempt to apply the platform's standard library/DLL prefixes\n   *        and suffixes, allowing a simple, unadorned name to be passed\n   *        regardless of platform. The set of name transforms is listed\n   *        below. A @i decorator is a platform's name designator for a debug\n   *        vs release build. For example, on Windows it is usually \"d\".\n   *        @li Prefix + name + decorator + suffix\n   *        @li Prefix + name + suffix\n   *        @li Name + decorator + suffix\n   *        @li Name + suffix\n   *        @li Name\n   *        Note that the transforms with @i decorator will be avoided if\n   *        ACE is built with the @c ACE_DISABLE_DEBUG_DLL_CHECK config macro.\n   *\n   *        @Note There is another mode for locating library/DLL files that\n   *        was used in old versions of ACE. The alternate method builds\n   *        more combinations of pathname by combining the names transforms\n   *        above with locations listed in the platform's standard \"path\"\n   *        locations (e.g., @c LD_LIBRARY_PATH). It can be enabled by building\n   *        ACE with the @c ACE_MUST_HELP_DLOPEN_SEARCH_PATH config macro.\n   *        Use of this option is discouraged since it avoids the standard\n   *        platform search options and security mechanisms.\n   *\n   * @param open_mode  Flags to alter the actions taken when loading the DLL.\n   *        The possible values are:\n   *        @li @c RTLD_LAZY (this the default): loads identifier symbols but\n   *            not the symbols for functions, which are loaded dynamically\n   *            on demand.\n   *        @li @c RTLD_NOW: performs all necessary relocations when\n   *            @a dll_name is first loaded\n   *        @li @c RTLD_GLOBAL: makes symbols available for relocation\n   *            processing of any other DLLs.\n   * @param close_handle_on_destruction  Indicates whether or not the\n   *        close() method will be called to close an open DLL when this\n   *        object is destroyed. By default, close() will be called.\n   *        Set this parameter to 0 for situations where the DLL's lifetime\n   *        is controlled in a scope other than that of this ACE_DLL object.\n   *        For example, termination by ACE_DLL_Manager via ACE::fini().\n   * @retval -1 On failure\n   * @retval 0 On success."]
    pub unsafe fn open(
        __this: *mut Self,
        mut dll_name: *const libc::c_char,
        mut open_mode: libc::c_int,
        mut close_handle_on_destruction: bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_DLL4openEPKcib"]
            fn __ext(
                __this: *mut ACE_DLL,
                dll_name: *const libc::c_char,
                open_mode: libc::c_int,
                close_handle_on_destruction: bool,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_DLL, dll_name, open_mode, close_handle_on_destruction)
    }
    ///Call to close the DLL object.
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_DLL5closeEv"]
            fn __ext(__this: *mut ACE_DLL) -> libc::c_int;
        }
        __ext(__this as *mut ACE_DLL)
    }
    #[doc = "* Look up a named symbol in the DLL. DLL must be successfully opened\n   * before calling symbol().\n   * @param symbol_name The symbol name to look up.\n   * @param ignore_errors If set to 1, allows you to probe a dll without\n   *        generating error messages in the log.  Handy for determining\n   *        the capabilities of a library.\n   * @return Returns the value of @a symbol_name if it is a valid symbol\n   *        in the DLL. Otherwise, returns 0."]
    pub unsafe fn symbol(
        __this: *mut Self,
        mut symbol_name: *const libc::c_char,
        mut ignore_errors: libc::c_int,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_DLL6symbolEPKci"]
            fn __ext(
                __this: *mut ACE_DLL,
                symbol_name: *const libc::c_char,
                ignore_errors: libc::c_int,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_DLL, symbol_name, ignore_errors)
    }
    /**Returns a pointer to a string explaining that an error occurred.  You
  /// will need to consult the error log for the actual error string
  /// returned by the OS.*/
    pub unsafe fn error(__this: *const Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZNK7ACE_DLL5errorEv"]
            fn __ext(__this: *const ACE_DLL) -> *mut libc::c_char;
        }
        __ext(__this as *const ACE_DLL)
    }
    #[doc = "* Return the handle to the caller.  If @a become_owner is true then\n   * caller assumes ownership of the handle and the ACE_DLL object\n   * won't call close() when it goes out of scope, even if\n   * @c close_handle_on_destruction is set."]
    pub unsafe fn get_handle(
        __this: *const Self,
        mut become_owner: bool,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZNK7ACE_DLL10get_handleEb"]
            fn __ext(__this: *const ACE_DLL, become_owner: bool) -> *mut libc::c_void;
        }
        __ext(__this as *const ACE_DLL, become_owner)
    }
    /**Set the handle for the DLL object. By default, the close()
  /// operation on / the object will be invoked before it is destroyed.*/
    pub unsafe fn set_handle(
        __this: *mut Self,
        mut handle: *mut libc::c_void,
        mut close_handle_on_destruction: bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_DLL10set_handleEPvb"]
            fn __ext(
                __this: *mut ACE_DLL,
                handle: *mut libc::c_void,
                close_handle_on_destruction: bool,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_DLL, handle, close_handle_on_destruction)
    }
    pub unsafe fn open_i(
        __this: *mut Self,
        mut dll_name: *const libc::c_char,
        mut open_mode: libc::c_int,
        mut close_handle_on_destruction: bool,
        mut handle: *mut libc::c_void,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN7ACE_DLL6open_iEPKcibPv"]
            fn __ext(
                __this: *mut ACE_DLL,
                dll_name: *const libc::c_char,
                open_mode: libc::c_int,
                close_handle_on_destruction: bool,
                handle: *mut libc::c_void,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_DLL,
            dll_name,
            open_mode,
            close_handle_on_destruction,
            handle,
        )
    }
}
impl ACE_Service_Repository {
    ///Initialize the repository.
    pub unsafe fn new_at(__this: *mut Self, mut __a0: libc::c_ulong) {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_RepositoryC1Em"]
            fn __ext(__this: *mut ACE_Service_Repository, __a0: libc::c_ulong);
        }
        __ext(__this as *mut ACE_Service_Repository, __a0)
    }
    pub unsafe fn new(mut __a0: libc::c_ulong) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Initialize the repository.
    pub unsafe fn open(__this: *mut Self, mut size: libc::c_ulong) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository4openEm"]
            fn __ext(
                __this: *mut ACE_Service_Repository,
                size: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository, size)
    }
    /**Close down the repository and free up dynamically allocated
  /// resources.*/
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository5closeEv"]
            fn __ext(__this: *mut ACE_Service_Repository) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository)
    }
    /**Finalize all the services by calling fini() and deleting
  /// dynamically allocated services.*/
    pub unsafe fn fini(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository4finiEv"]
            fn __ext(__this: *mut ACE_Service_Repository) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository)
    }
    ///Get pointer to a process-wide ACE_Service_Repository.
    pub unsafe fn instance(mut size: libc::c_ulong) -> *mut ACE_Service_Repository {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository8instanceEm"]
            fn __ext(size: libc::c_ulong) -> *mut ACE_Service_Repository;
        }
        __ext(size)
    }
    /**Set pointer to a process-wide ACE_Service_Repository and return
  /// existing pointer.*/
    pub unsafe fn instance_u1d061286340a754d(
        mut _anon_0: *mut ACE_Service_Repository,
    ) -> *mut ACE_Service_Repository {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository8instanceEPS_"]
            fn __ext(
                _anon_0: *mut ACE_Service_Repository,
            ) -> *mut ACE_Service_Repository;
        }
        __ext(_anon_0)
    }
    ///Delete the dynamically allocated Singleton.
    pub unsafe fn close_singleton() {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository15close_singletonEv"]
            fn __ext();
        }
        __ext()
    }
    /**Insert a new service record.  Returns -1 when the service repository
  /// is full and 0 on success.*/
    pub unsafe fn insert(
        __this: *mut Self,
        mut sr: *const ACE_Service_Type,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository6insertEPK16ACE_Service_Type"]
            fn __ext(
                __this: *mut ACE_Service_Repository,
                sr: *const ACE_Service_Type,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository, sr)
    }
    #[doc = "* Locate a named entry in the service table, optionally ignoring\n   * suspended entries.\n   *\n   * @param name The name of the service to search for.\n   * @param srp  Optional; if not 0, it is a pointer to a location\n   *             to receive the ACE_Service_Type pointer for the\n   *             located service. Meaningless if this method\n   *             returns -1.\n   * @param ignore_suspended If true, the search ignores suspended services.\n   *\n   * @retval  0 Named service was located.\n   * @retval -1 Named service was not found.\n   * @retval -2 Named service was found, but is suspended and\n   *            @a ignore_suspended is true."]
    pub unsafe fn find(
        __this: *const Self,
        mut name: *const libc::c_char,
        mut srp: *mut *const ACE_Service_Type,
        mut ignore_suspended: bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK22ACE_Service_Repository4findEPKcPPK16ACE_Service_Typeb"]
            fn __ext(
                __this: *const ACE_Service_Repository,
                name: *const libc::c_char,
                srp: *mut *const ACE_Service_Type,
                ignore_suspended: bool,
            ) -> libc::c_int;
        }
        __ext(__this as *const ACE_Service_Repository, name, srp, ignore_suspended)
    }
    /**Remove an existing service record. If @a sr == 0, the service record
  /// is deleted before control is returned to the caller. If @a sr != 0,
  /// the service's record is removed from the repository, but not deleted;
  /// *sr receives the service record pointer and the caller is responsible
  /// for properly disposing of it.*/
    pub unsafe fn remove(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut sr: *mut *mut ACE_Service_Type,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository6removeEPKcPP16ACE_Service_Type"]
            fn __ext(
                __this: *mut ACE_Service_Repository,
                name: *const libc::c_char,
                sr: *mut *mut ACE_Service_Type,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository, name, sr)
    }
    ///Resume a service record.
    pub unsafe fn resume(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut srp: *mut *const ACE_Service_Type,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository6resumeEPKcPPK16ACE_Service_Type"]
            fn __ext(
                __this: *mut ACE_Service_Repository,
                name: *const libc::c_char,
                srp: *mut *const ACE_Service_Type,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository, name, srp)
    }
    ///Suspend a service record.
    pub unsafe fn suspend(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut srp: *mut *const ACE_Service_Type,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository7suspendEPKcPPK16ACE_Service_Type"]
            fn __ext(
                __this: *mut ACE_Service_Repository,
                name: *const libc::c_char,
                srp: *mut *const ACE_Service_Type,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository, name, srp)
    }
    ///Return the current size of the repository.
    pub unsafe fn current_size(__this: *const Self) -> libc::c_ulong {
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
                    ((::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Recursive_Thread_Mutex > ().cast_mut())
                    )) as *mut ACE_Recursive_Thread_Mutex),
                );
                if (((((<ACE_Guard_ACE_Recursive_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Recursive_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return ((0) as libc::c_ulong);
                };
                return ((<ACE_Array_Map_unsigned_long__const_ACE_Service_Type___>::size(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).service_array_) .cast:: <
                        ACE_Array_Map_unsigned_long__const_ACE_Service_Type___ > ()
                        .cast_mut())
                    )) as *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                )) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK22ACE_Service_Repository4dumpEv"]
            fn __ext(__this: *const ACE_Service_Repository);
        }
        __ext(__this as *const ACE_Service_Repository)
    }
    ///Returns a reference to the lock used by the ACE_Service_Repository
    pub unsafe fn lock(__this: *const Self) -> *mut ACE_Recursive_Thread_Mutex {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of_mut!(
                    (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                    ACE_Recursive_Thread_Mutex > ().cast_mut())
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Remove an existing service record. It requires @a sr != 0, which
  /// receives the service record pointer and the caller is
  /// responsible for properly disposing of it.*/
    pub unsafe fn remove_i(
        __this: *mut Self,
        mut _anon_0: *const libc::c_char,
        mut sr: *mut *mut ACE_Service_Type,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository8remove_iEPKcPP16ACE_Service_Type"]
            fn __ext(
                __this: *mut ACE_Service_Repository,
                _anon_0: *const libc::c_char,
                sr: *mut *mut ACE_Service_Type,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository, _anon_0, sr)
    }
    #[doc = "* Locate a named entry in the service table, optionally ignoring\n   * suspended entries.\n   *\n   * @param service_name The name of the service to search for.\n   * @param slot         Receives the position index of the service if it\n   *                     is found. Contents are meaningless if this method\n   *                     returns -1.\n   * @param srp          Optional; if not 0, it is a pointer to a location\n   *                     to receive the ACE_Service_Type pointer for the\n   *                     located service. Meaningless if this method\n   *                     returns -1.\n   * @param ignore_suspended If true, the search ignores suspended services.\n   *\n   * @retval  0 Named service was located; index in the table is set in\n   *            @a slot.\n   * @retval -1 Named service was not found.\n   * @retval -2 Named service was found, but is suspended and\n   *            @a ignore_suspended is true."]
    pub unsafe fn find_i(
        __this: *const Self,
        mut service_name: *const libc::c_char,
        mut slot: *mut libc::c_ulong,
        mut srp: *mut *const ACE_Service_Type,
        mut ignore_suspended: bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK22ACE_Service_Repository6find_iEPKcRmPPK16ACE_Service_Typeb"]
            fn __ext(
                __this: *const ACE_Service_Repository,
                service_name: *const libc::c_char,
                slot: *mut libc::c_ulong,
                srp: *mut *const ACE_Service_Type,
                ignore_suspended: bool,
            ) -> libc::c_int;
        }
        __ext(
            __this as *const ACE_Service_Repository,
            service_name,
            slot,
            srp,
            ignore_suspended,
        )
    }
    /**@brief Relocate (static) services to another DLL.
  ///
  /// If any have been registered in the context of a "forward
  /// declaration" guard, those really aren't static services. Their
  /// code is in the DLL's code segment, or in one of the dependent
  /// DLLs. Therefore, such services need to be associated with the
  /// proper DLL in order to prevent failures upon finalization. The
  /// method locks the repo.
  ///
  /// Works by having the service type keep a reference to a specific
  /// DLL. No locking, caller makes sure calling it is safe. You can
  /// forcefully relocate any DLLs in the given range, not only the
  /// static ones - but that will cause Very Bad Things (tm) to happen.*/
    pub unsafe fn relocate_i(
        __this: *mut Self,
        mut begin: libc::c_ulong,
        mut end: libc::c_ulong,
        mut adll: *const ACE_DLL,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Service_Repository10relocate_iEmmRK7ACE_DLL"]
            fn __ext(
                __this: *mut ACE_Service_Repository,
                begin: libc::c_ulong,
                end: libc::c_ulong,
                adll: *const ACE_DLL,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository, begin, end, adll)
    }
}
impl ACE_Array_Map_unsigned_long__const_ACE_Service_Type___ {
    #[doc = "Default Constructor.\n  /**\n   * Create an empty map with a preallocated buffer of size @a s."]
    pub unsafe fn new_at_s6ca216cc9de306a9(__this: *mut Self, mut __a0: libc::c_ulong) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEEC1Em"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                __a0: libc::c_ulong,
            );
        }
        __ext(
            __this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            __a0,
        )
    }
    pub unsafe fn new_s6ca216cc9de306a9(mut __a0: libc::c_ulong) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s6ca216cc9de306a9(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_sde87adb779a2fc87(
        __this: *mut Self,
        mut __a0: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEEC1ERKS8_"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                __a0: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            );
        }
        __ext(
            __this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            __a0,
        )
    }
    pub unsafe fn new_sde87adb779a2fc87(
        mut __a0: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sde87adb779a2fc87(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut map: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
    ) -> *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___ {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEEaSERKS8_"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                map: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            ) -> *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___;
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___, map)
    }
    pub unsafe fn begin_s49079642b530f057(__this: *mut Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE5beginEv"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___)
    }
    pub unsafe fn end_s49079642b530f057(__this: *mut Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE3endEv"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___)
    }
    pub unsafe fn begin_sb9268533dba3aeba(__this: *const Self) -> *const libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE5beginEv"]
            fn __ext(
                __this: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            ) -> *const libc::c_void;
        }
        __ext(__this as *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___)
    }
    pub unsafe fn end_sb9268533dba3aeba(__this: *const Self) -> *const libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE3endEv"]
            fn __ext(
                __this: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            ) -> *const libc::c_void;
        }
        __ext(__this as *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___)
    }
    ///Maximum number of elements the map can hold.
    pub unsafe fn max_size(__this: *const Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE8max_sizeEv"]
            fn __ext(
                __this: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            ) -> libc::c_ulong;
        }
        __ext(__this as *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___)
    }
    ///Return @c true if the map is empty, else @c false.
    pub unsafe fn is_empty(__this: *const Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE8is_emptyEv"]
            fn __ext(
                __this: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            ) -> bool;
        }
        __ext(__this as *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___)
    }
    #[doc = "* Return @c true if the map is empty, else @c false.  We recommend\n   * using @c is_empty() instead since it's more consistent with the\n   * ACE container naming conventions."]
    pub unsafe fn empty(__this: *const Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE5emptyEv"]
            fn __ext(
                __this: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            ) -> bool;
        }
        __ext(__this as *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___)
    }
    /**Swap the contents of this map with the given @a map in an
  /// exception-safe manner.*/
    pub unsafe fn swap(
        __this: *mut Self,
        mut map: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE4swapERS8_"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                map: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            );
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___, map)
    }
    ///Remove element at position @a pos from the map.
    pub unsafe fn erase_s6cf4a3422a118593(
        __this: *mut Self,
        mut pos: *mut crate::__cxx_std::Pair<libc::c_ulong, *const ACE_Service_Type>,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE5eraseEPS6_"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                pos: *mut crate::__cxx_std::Pair<libc::c_ulong, *const ACE_Service_Type>,
            );
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___, pos)
    }
    #[doc = "Remove element corresponding to key @a k from the map.\n  /**\n   * @return Number of elements that were erased."]
    pub unsafe fn erase_scd30614132716564(
        __this: *mut Self,
        mut k: *const libc::c_ulong,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE5eraseERKm"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                k: *const libc::c_ulong,
            ) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___, k)
    }
    #[doc = "Remove range of elements [@a first, @a last) from the map.\n  /**\n   * @note [@a first, @a last) must be valid range within the map."]
    pub unsafe fn erase_sbe0edb76f5263820(
        __this: *mut Self,
        mut first: *mut crate::__cxx_std::Pair<libc::c_ulong, *const ACE_Service_Type>,
        mut last: *mut crate::__cxx_std::Pair<libc::c_ulong, *const ACE_Service_Type>,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE5eraseEPS6_S9_"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                first: *mut crate::__cxx_std::Pair<
                    libc::c_ulong,
                    *const ACE_Service_Type,
                >,
                last: *mut crate::__cxx_std::Pair<libc::c_ulong, *const ACE_Service_Type>,
            );
        }
        __ext(
            __this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            first,
            last,
        )
    }
    #[doc = "Clear contents of map.\n  /**\n   * @note This a constant time (O(1)) operation."]
    pub unsafe fn clear(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE5clearEv"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
            );
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___)
    }
    #[doc = "* @return @c end() if data corresponding to key @a k is not in the\n   *         map."]
    pub unsafe fn find_s0a7bc01cd46dd81b(
        __this: *mut Self,
        mut k: *const libc::c_ulong,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE4findERKm"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                k: *const libc::c_ulong,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___, k)
    }
    #[doc = "* @return @c end() if data corresponding to key @a k is not in the\n   *         map."]
    pub unsafe fn find_sf9774f0f80bcbaea(
        __this: *const Self,
        mut k: *const libc::c_ulong,
    ) -> *const libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE4findERKm"]
            fn __ext(
                __this: *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                k: *const libc::c_ulong,
            ) -> *const libc::c_void;
        }
        __ext(__this as *const ACE_Array_Map_unsigned_long__const_ACE_Service_Type___, k)
    }
    #[doc = "Count the number of elements corresponding to key @a k.\n  /**\n   * @return In the case of this map, the count will always be one if\n   *         such exists in the map."]
    pub unsafe fn count(
        __this: *mut Self,
        mut k: *const libc::c_ulong,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE5countERKm"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                k: *const libc::c_ulong,
            ) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___, k)
    }
    #[doc = "Convenience array index operator.\n  /**\n   * Array index operator that allows insertion and retrieval of\n   * elements using an array index syntax, such as:\n   * @par\n   * map[\"Foo\"] = 12;"]
    pub unsafe fn operator_index(
        __this: *mut Self,
        mut k: *const libc::c_ulong,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEEixERKm"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                k: *const libc::c_ulong,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___, k)
    }
    pub unsafe fn get_allocator(__this: *const Self) -> crate::__cxx_std::Allocator {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).alloc_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Increase size of underlying buffer by @a s.
    pub unsafe fn grow(__this: *mut Self, mut s: libc::c_ulong) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Array_MapImPK16ACE_Service_TypeSt8equal_toImESaISt4pairImS2_EEE4growEm"]
            fn __ext(
                __this: *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___,
                s: libc::c_ulong,
            );
        }
        __ext(__this as *mut ACE_Array_Map_unsigned_long__const_ACE_Service_Type___, s)
    }
    #[doc = "Return current size of map.\n  /**\n   * @return The number of elements in the map."]
    pub unsafe fn size(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).size_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Service_Repository_Iterator {
    ///Constructor initializes the iterator.
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *mut ACE_Service_Repository,
        mut __a1: bool,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN31ACE_Service_Repository_IteratorC1ER22ACE_Service_Repositoryb"]
            fn __ext(
                __this: *mut ACE_Service_Repository_Iterator,
                __a0: *mut ACE_Service_Repository,
                __a1: bool,
            );
        }
        __ext(__this as *mut ACE_Service_Repository_Iterator, __a0, __a1)
    }
    pub unsafe fn new(mut __a0: *mut ACE_Service_Repository, mut __a1: bool) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Pass back the @a next_item that hasn't been seen in the repository.
  /// Returns 0 when all items have been seen, else 1.*/
    pub unsafe fn next(
        __this: *mut Self,
        mut next_item: *mut *const ACE_Service_Type,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN31ACE_Service_Repository_Iterator4nextERPK16ACE_Service_Type"]
            fn __ext(
                __this: *mut ACE_Service_Repository_Iterator,
                next_item: *mut *const ACE_Service_Type,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository_Iterator, next_item)
    }
    ///Returns 1 when all items have been seen, else 0.
    pub unsafe fn done(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ((((((*__this).next_ as libc::c_ulong))
                    >= (((<ACE_Service_Repository>::current_size(
                        (::core::ptr::addr_of!((* (* __this).svc_rep_)))
                            as *const ACE_Service_Repository,
                    )) as libc::c_ulong))) as libc::c_int) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Move forward by one element in the repository.  Returns 0 when all the
  /// items in the set have been seen, else 1.*/
    pub unsafe fn advance(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN31ACE_Service_Repository_Iterator7advanceEv"]
            fn __ext(__this: *mut ACE_Service_Repository_Iterator) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Repository_Iterator)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK31ACE_Service_Repository_Iterator4dumpEv"]
            fn __ext(__this: *const ACE_Service_Repository_Iterator);
        }
        __ext(__this as *const ACE_Service_Repository_Iterator)
    }
    pub unsafe fn valid(__this: *const Self) -> bool {
        extern "C-unwind" {
            #[link_name = "_ZNK31ACE_Service_Repository_Iterator5validEv"]
            fn __ext(__this: *const ACE_Service_Repository_Iterator) -> bool;
        }
        __ext(__this as *const ACE_Service_Repository_Iterator)
    }
    pub unsafe fn new_at_u42c9cc6fbf2e6d70(
        __this: *mut Self,
        mut __a0: *const ACE_Service_Repository_Iterator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN31ACE_Service_Repository_IteratorC1ERKS_"]
            fn __ext(
                __this: *mut ACE_Service_Repository_Iterator,
                __a0: *const ACE_Service_Repository_Iterator,
            );
        }
        __ext(__this as *mut ACE_Service_Repository_Iterator, __a0)
    }
    pub unsafe fn new_u42c9cc6fbf2e6d70(
        mut __a0: *const ACE_Service_Repository_Iterator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u42c9cc6fbf2e6d70(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
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
impl ACE_Thread_Adapter {
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
        mut __a3: *mut ACE_Thread_Manager,
        mut __a4: *mut ACE_Thread_Descriptor,
        mut __a5: libc::c_long,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Thread_AdapterC1EPFPvS0_ES0_S2_P18ACE_Thread_ManagerP21ACE_Thread_Descriptorl"]
            fn __ext(
                __this: *mut ACE_Thread_Adapter,
                __a0: Option<
                    unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
                >,
                __a1: *mut libc::c_void,
                __a2: Option<
                    unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
                >,
                __a3: *mut ACE_Thread_Manager,
                __a4: *mut ACE_Thread_Descriptor,
                __a5: libc::c_long,
            );
        }
        __ext(__this as *mut ACE_Thread_Adapter, __a0, __a1, __a2, __a3, __a4, __a5)
    }
    pub unsafe fn new(
        mut __a0: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        mut __a1: *mut libc::c_void,
        mut __a2: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        mut __a3: *mut ACE_Thread_Manager,
        mut __a4: *mut ACE_Thread_Descriptor,
        mut __a5: libc::c_long,
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
        );
        __obj
    }
    #[doc = "* Execute the <user_func_> with the <arg>.  This function deletes\n   * @c this, thereby rendering the object useless after the call\n   * returns."]
    pub unsafe fn invoke(__this: *mut Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Thread_Adapter6invokeEv"]
            fn __ext(__this: *mut ACE_Thread_Adapter) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Thread_Adapter)
    }
    ///Accessor for the optional ACE_Thread_Manager.
    pub unsafe fn thr_mgr(__this: *mut Self) -> *mut ACE_Thread_Manager {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (*__this).thr_mgr_;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Called by invoke, mainly here to separate the SEH stuff because
  /// SEH on Win32 doesn't compile with local vars with destructors.*/
    pub unsafe fn invoke_i(__this: *mut Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Thread_Adapter8invoke_iEv"]
            fn __ext(__this: *mut ACE_Thread_Adapter) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Thread_Adapter)
    }
}
impl ACE_Thread {
    #[doc = "* Creates a new thread having @a flags attributes and running @a func\n   * with @a args (if @a thread_adapter is non-0 then @a func and @a args\n   * are ignored and are obtained from @a thread_adapter>.  @a thr_id\n   * and @a t_handle are set to the thread's ID and handle (?),\n   * respectively.  The thread runs at @a priority priority (see\n   * below).\n   *\n   * The @a flags are a bitwise-OR of the following:\n   * = BEGIN<INDENT>\n   * THR_CANCEL_DISABLE, THR_CANCEL_ENABLE, THR_CANCEL_DEFERRED,\n   * THR_CANCEL_ASYNCHRONOUS, THR_BOUND, THR_NEW_LWP, THR_DETACHED,\n   * THR_SUSPENDED, THR_DAEMON, THR_JOINABLE, THR_SCHED_FIFO,\n   * THR_SCHED_RR, THR_SCHED_DEFAULT, THR_EXPLICIT_SCHED,\n   * THR_SCOPE_SYSTEM, THR_SCOPE_PROCESS\n   * = END<INDENT>\n   *\n   * By default, or if @a priority is set to\n   * ACE_DEFAULT_THREAD_PRIORITY, an \"appropriate\" priority value for\n   * the given scheduling policy (specified in @a flags, e.g.,\n   * @c THR_SCHED_DEFAULT is used.  This value is calculated\n   * dynamically, and is the median value between the minimum and\n   * maximum priority values for the given policy.  If an explicit\n   * value is given, it is used.  Note that actual priority values are\n   * EXTREMELY implementation-dependent, and are probably best\n   * avoided.\n   *\n   * Note that @a thread_adapter is always deleted when @a spawn\n   * is called, so it must be allocated with global operator new."]
    pub unsafe fn spawn(
        mut func: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        mut arg: *mut libc::c_void,
        mut flags: libc::c_long,
        mut t_id: *mut libc::c_ulong,
        mut t_handle: *mut libc::c_ulong,
        mut priority: libc::c_long,
        mut thr_stack: *mut libc::c_void,
        mut thr_stack_size: libc::c_ulong,
        mut thread_adapter: *mut ACE_Thread_Adapter,
        mut thr_name: *mut *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_create(
                    func,
                    arg,
                    flags,
                    t_id,
                    t_handle,
                    priority,
                    thr_stack,
                    thr_stack_size,
                    ((thread_adapter) as *mut ACE_Base_Thread_Adapter),
                    thr_name,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Spawn N new threads, which execute @a func with argument @a arg (if\n   * @a thread_adapter is non-0 then @a func and @a args are ignored and\n   * are obtained from @a thread_adapter).  If @a stack != 0 it is\n   * assumed to be an array of @a n pointers to the base of the stacks\n   * to use for the threads being spawned.  Likewise, if @a stack_size\n   * != 0 it is assumed to be an array of @a n values indicating how\n   * big each of the corresponding @a stacks are.  Returns the number\n   * of threads actually spawned (if this doesn't equal the number\n   * requested then something has gone wrong and @c errno will\n   * explain...).\n   *\n   * @see spawn()"]
    pub unsafe fn spawn_n(
        mut n: libc::c_ulong,
        mut func: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        mut arg: *mut libc::c_void,
        mut flags: libc::c_long,
        mut priority: libc::c_long,
        mut stack: *mut *mut libc::c_void,
        mut stack_size: *mut libc::c_ulong,
        mut thread_adapter: *mut ACE_Thread_Adapter,
        mut thr_name: *mut *const libc::c_char,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN10ACE_Thread7spawn_nEmPFPvS0_ES0_llPS0_PmP18ACE_Thread_AdapterPPKc"]
            fn __ext(
                n: libc::c_ulong,
                func: Option<
                    unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
                >,
                arg: *mut libc::c_void,
                flags: libc::c_long,
                priority: libc::c_long,
                stack: *mut *mut libc::c_void,
                stack_size: *mut libc::c_ulong,
                thread_adapter: *mut ACE_Thread_Adapter,
                thr_name: *mut *const libc::c_char,
            ) -> libc::c_ulong;
        }
        __ext(n, func, arg, flags, priority, stack, stack_size, thread_adapter, thr_name)
    }
    #[doc = "* Spawn @a n new threads, which execute @a func with argument @a arg\n   * (if @a thread_adapter is non-0 then @a func and @a args are ignored\n   * and are obtained from @a thread_adapter).  The thread_ids of\n   * successfully spawned threads will be placed into the @a thread_ids\n   * buffer (which must be the same size as @a n).  If @a stack != 0 it\n   * is assumed to be an array of @a n pointers to the base of the\n   * stacks to use for the threads being spawned.  If @a stack_size !=\n   * 0 it is assumed to be an array of @a n values indicating how big\n   * each of the corresponding @a stacks are.  If @a thread_handles != 0\n   * it is assumed to be an array of @a n thread_handles that will be\n   * assigned the values of the thread handles being spawned.  Returns\n   * the number of threads actually spawned (if this doesn't equal the\n   * number requested then something has gone wrong and @c errno will\n   * explain...).\n   *\n   * @see spawn()"]
    pub unsafe fn spawn_n_u809bafb34e21c2cf(
        mut thread_ids: *mut libc::c_ulong,
        mut n: libc::c_ulong,
        mut func: Option<
            unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        mut arg: *mut libc::c_void,
        mut flags: libc::c_long,
        mut priority: libc::c_long,
        mut stack: *mut *mut libc::c_void,
        mut stack_size: *mut libc::c_ulong,
        mut thread_handles: *mut libc::c_ulong,
        mut thread_adapter: *mut ACE_Thread_Adapter,
        mut thr_name: *mut *const libc::c_char,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN10ACE_Thread7spawn_nEPmmPFPvS1_ES1_llPS1_S0_S0_P18ACE_Thread_AdapterPPKc"]
            fn __ext(
                thread_ids: *mut libc::c_ulong,
                n: libc::c_ulong,
                func: Option<
                    unsafe extern "C-unwind" fn(*mut libc::c_void) -> *mut libc::c_void,
                >,
                arg: *mut libc::c_void,
                flags: libc::c_long,
                priority: libc::c_long,
                stack: *mut *mut libc::c_void,
                stack_size: *mut libc::c_ulong,
                thread_handles: *mut libc::c_ulong,
                thread_adapter: *mut ACE_Thread_Adapter,
                thr_name: *mut *const libc::c_char,
            ) -> libc::c_ulong;
        }
        __ext(
            thread_ids,
            n,
            func,
            arg,
            flags,
            priority,
            stack,
            stack_size,
            thread_handles,
            thread_adapter,
            thr_name,
        )
    }
    #[doc = "* Wait for one or more threads to exit and reap their exit status.\n   * thr_join() returns successfully when the target thread terminates.\n   *\n   * @param thread_id is the ACE_thread_t ID of the thread to wait for.\n   *                  If @a thread_id is 0, join() waits for any\n   *                  undetached thread in the process to terminate\n   *                  on platforms that support this capability\n   *                  (for example, Solaris).\n   * @param departed  points to a location that is set to the ID of the\n   *                  terminated thread if join() returns successfully.\n   *                  If @a departed is 0, it is ignored.\n   * @param status    Points to the location that receives the joined\n   *                  thread's exit value. If @a status is 0, it is ignored.\n   *\n   * @retval  0 for success\n   * @retval  -1 (with errno set) for failure."]
    pub unsafe fn join(
        mut wait_for: libc::c_ulong,
        mut departed: *mut libc::c_ulong,
        mut status: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_join_u1274f10f177d1aed(wait_for, departed, status);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Wait for one thread to exit and reap its exit status.
    pub unsafe fn join_uddd1ac13acdfb62a(
        mut wait_for: libc::c_ulong,
        mut status: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_join(wait_for, status);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Continue the execution of a previously suspended thread.
    pub unsafe fn resume(mut t_id: libc::c_ulong) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_continue(t_id);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Suspend the execution of a particular thread.
    pub unsafe fn suspend(mut t_id: libc::c_ulong) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_suspend(t_id);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the priority of a particular thread.
    pub unsafe fn getprio(
        mut ht_id: libc::c_ulong,
        mut priority: *mut libc::c_int,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_getprio(
                    ht_id,
                    ::core::ptr::addr_of_mut!((* priority)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the priority and policy of a particular thread.
    pub unsafe fn getprio_ubd015c3d6260c624(
        mut ht_id: libc::c_ulong,
        mut priority: *mut libc::c_int,
        mut policy: *mut libc::c_int,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_getprio_u470508780c2aa1c6(
                    ht_id,
                    ::core::ptr::addr_of_mut!((* priority)),
                    ::core::ptr::addr_of_mut!((* policy)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the priority of a particular thread.
    pub unsafe fn setprio(
        mut ht_id: libc::c_ulong,
        mut priority: libc::c_int,
        mut policy: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_setprio_ucfa01c0139522e03(ht_id, priority, policy);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Send a signal to the thread.
    pub unsafe fn kill(mut t_id: libc::c_ulong, mut signum: libc::c_int) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_kill(t_id, signum);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Yield the thread to another.
    pub unsafe fn r#yield() {
        unsafe {
            {
                ACE_OS::thr_yield();
            }
            ()
        }
    }
    #[doc = "* Return the unique kernel handle of the thread.  Note that on\n   * Win32 this is actually a pseudohandle, which cannot be shared\n   * with other processes or waited on by threads.  To locate the real\n   * handle, please use the ACE_Thread_Manager::thr_self() method."]
    pub unsafe fn self_(mut t_id: *mut libc::c_ulong) {
        unsafe {
            {
                ACE_OS::thr_self_u35bfcd9d5906cbcf(::core::ptr::addr_of_mut!((* t_id)));
            }
            ()
        }
    }
    ///Return the unique ID of the thread.
    pub unsafe fn self_u2ee556c73b662daa() -> libc::c_ulong {
        unsafe {
            {
                return ACE_OS::thr_self();
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Exit the current thread and return "status".
  /// Should _not_ be called by main thread.*/
    pub unsafe fn exit(mut status: *mut libc::c_void) {
        unsafe {
            {
                ACE_OS::thr_exit(status);
            }
            ()
        }
    }
    ///Get the LWP concurrency level of the process.
    pub unsafe fn getconcurrency() -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_getconcurrency();
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the LWP concurrency level of the process.
    pub unsafe fn setconcurrency(mut new_level: libc::c_int) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_setconcurrency(new_level);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Change and/or examine calling thread's signal mask.
    pub unsafe fn sigsetmask(
        mut how: libc::c_int,
        mut sigset: *const __sigset_t,
        mut osigset: *mut __sigset_t,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_sigsetmask(how, sigset, osigset);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Allocates a @a keyp that is used to identify data that is specific\n   * to each thread in the process.  The key is global to all threads\n   * in the process."]
    pub unsafe fn keycreate(
        mut keyp: *mut libc::c_uint,
        mut destructor: Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_keycreate(keyp, destructor);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Free up the key so that other threads can reuse it.
    pub unsafe fn keyfree(mut key: libc::c_uint) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_keyfree(key);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Bind value to the thread-specific data key, @a key, for the calling
  /// thread.*/
    pub unsafe fn setspecific(
        mut key: libc::c_uint,
        mut value: *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_setspecific(key, value);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Stores the current value bound to @a key for the calling thread
  /// into the location pointed to by @a valuep.*/
    pub unsafe fn getspecific(
        mut key: libc::c_uint,
        mut valuep: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_getspecific(key, valuep);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Disable thread cancellation.
    pub unsafe fn disablecancel(mut old_state: *mut cancel_state) -> libc::c_int {
        unsafe {
            {
                let mut old_cstate: libc::c_int = 0;
                let mut result: libc::c_int = ACE_OS::thr_setcancelstate(
                    256,
                    ::core::ptr::addr_of_mut!(old_cstate) as *mut libc::c_int,
                );
                if (((((((((result as libc::c_int)) == (((0) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                    && ((((!(old_state).is_null()) as libc::c_int) as libc::c_int) != 0))
                    as libc::c_int) as libc::c_int) != 0)
                {
                    ACE_OS::memset_u2b5dfc47d301370a(
                        ((old_state) as *mut libc::c_void),
                        0,
                        ((::core::mem::size_of::<cancel_state>() as libc::c_ulong)
                            as libc::c_ulong),
                    );
                    (*old_state).cancelstate = old_cstate;
                }
                return result;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Enable thread cancellation.
    pub unsafe fn enablecancel(
        mut old_state: *mut cancel_state,
        mut flag: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            {
                let mut old_cstate: libc::c_int = 0;
                let mut old_ctype: libc::c_int = 0;
                let mut result: libc::c_int = unsafe { ::core::mem::zeroed() };
                result = ACE_OS::thr_setcancelstate(
                    512,
                    ::core::ptr::addr_of_mut!(old_cstate) as *mut libc::c_int,
                );
                if (((((result as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return result;
                }
                result = ACE_OS::thr_setcanceltype(
                    flag,
                    ::core::ptr::addr_of_mut!(old_ctype) as *mut libc::c_int,
                );
                if (((((result as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return result;
                }
                if ((((!(old_state).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    (*old_state).cancelstate = old_cstate;
                    (*old_state).canceltype = old_ctype;
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Set the cancellation state.
    pub unsafe fn setcancelstate(
        mut new_state: *mut cancel_state,
        mut old_state: *mut cancel_state,
    ) -> libc::c_int {
        unsafe {
            {
                let mut old_cstate: libc::c_int = 0;
                let mut old_ctype: libc::c_int = 0;
                if ((((((((((*new_state).cancelstate as libc::c_int))
                    != (((0) as libc::c_int))) as libc::c_int as libc::c_int) != 0)
                    && (((((ACE_OS::thr_setcancelstate(
                        (*new_state).cancelstate,
                        ::core::ptr::addr_of_mut!(old_cstate) as *mut libc::c_int,
                    ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                if ((((((((((*new_state).canceltype as libc::c_int))
                    != (((0) as libc::c_int))) as libc::c_int as libc::c_int) != 0)
                    && (((((ACE_OS::thr_setcanceltype(
                        (*new_state).canceltype,
                        ::core::ptr::addr_of_mut!(old_ctype) as *mut libc::c_int,
                    ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                {
                    let mut o_cstate: libc::c_int = unsafe { ::core::mem::zeroed() };
                    ACE_OS::thr_setcancelstate(
                        old_cstate,
                        ::core::ptr::addr_of_mut!(o_cstate) as *mut libc::c_int,
                    );
                    return (-((1) as libc::c_int));
                }
                if ((((!(old_state).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    (*old_state).cancelstate = old_cstate;
                    (*old_state).canceltype = old_ctype;
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Cancel a thread.\n   * @note This method is only portable on platforms, such as POSIX pthreads,\n   * that support thread cancellation."]
    pub unsafe fn cancel(mut t_id: libc::c_ulong) -> libc::c_int {
        unsafe {
            {
                return ACE_OS::thr_cancel(t_id);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Test the cancel.
    pub unsafe fn testcancel() {
        unsafe {
            {
                ACE_OS::thr_testcancel();
            }
            ()
        }
    }
    ///Ensure that we don't get instantiated.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN10ACE_ThreadC1Ev"]
            fn __ext(__this: *mut ACE_Thread);
        }
        __ext(__this as *mut ACE_Thread)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
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
impl ACE_Framework_Repository {
    ///Initialize the repository.
    pub unsafe fn open(__this: *mut Self, mut size: libc::c_int) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_Repository4openEi"]
            fn __ext(
                __this: *mut ACE_Framework_Repository,
                size: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Framework_Repository, size)
    }
    /**Close down the repository and free up dynamically allocated
  /// resources, also called by dtor.*/
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_Repository5closeEv"]
            fn __ext(__this: *mut ACE_Framework_Repository) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Framework_Repository)
    }
    ///Get pointer to a process-wide ACE_Framework_Repository.
    pub unsafe fn instance(mut size: libc::c_int) -> *mut ACE_Framework_Repository {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_Repository8instanceEi"]
            fn __ext(size: libc::c_int) -> *mut ACE_Framework_Repository;
        }
        __ext(size)
    }
    ///Delete the dynamically allocated Singleton.
    pub unsafe fn close_singleton() {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_Repository15close_singletonEv"]
            fn __ext();
        }
        __ext()
    }
    /**Insert a new component.  Returns -1 when the repository is full
  /// and 0 on success.*/
    pub unsafe fn register_component(
        __this: *mut Self,
        mut fc: *mut ACE_Framework_Component,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_Repository18register_componentEP23ACE_Framework_Component"]
            fn __ext(
                __this: *mut ACE_Framework_Repository,
                fc: *mut ACE_Framework_Component,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Framework_Repository, fc)
    }
    /**Remove a component.  Returns -1 on error or if component not found
  /// and 0 on success.*/
    pub unsafe fn remove_component(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_Repository16remove_componentEPKc"]
            fn __ext(
                __this: *mut ACE_Framework_Repository,
                name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Framework_Repository, name)
    }
    ///Remove all components associated with a particular dll.
    pub unsafe fn remove_dll_components(
        __this: *mut Self,
        mut dll_name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_Repository21remove_dll_componentsEPKc"]
            fn __ext(
                __this: *mut ACE_Framework_Repository,
                dll_name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Framework_Repository, dll_name)
    }
    ///Return the current size of the repository.
    pub unsafe fn current_size(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Thread_Mutex_>::new_at_s194b481c4e491c25(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Thread_Mutex_,
                    ((::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Thread_Mutex > ().cast_mut())
                    )) as *mut ACE_Thread_Mutex),
                );
                if (((((<ACE_Guard_ACE_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                return (((*__this).current_size_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the total size of the repository.
    pub unsafe fn total_size(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Thread_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Thread_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Thread_Mutex_>::new_at_s194b481c4e491c25(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Thread_Mutex_,
                    ((::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* __this).lock_) .cast:: <
                        ACE_Thread_Mutex > ().cast_mut())
                    )) as *mut ACE_Thread_Mutex),
                );
                if (((((<ACE_Guard_ACE_Thread_Mutex_>::locked(
                    (::core::ptr::addr_of!(ace_mon))
                        as *const ACE_Guard_ACE_Thread_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                return (((*__this).total_size_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK24ACE_Framework_Repository4dumpEv"]
            fn __ext(__this: *const ACE_Framework_Repository);
        }
        __ext(__this as *const ACE_Framework_Repository)
    }
    ///Initialize the repository.
    pub unsafe fn new_at(__this: *mut Self, mut __a0: libc::c_int) {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_RepositoryC1Ei"]
            fn __ext(__this: *mut ACE_Framework_Repository, __a0: libc::c_int);
        }
        __ext(__this as *mut ACE_Framework_Repository, __a0)
    }
    pub unsafe fn new(mut __a0: libc::c_int) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Actually removes the dll components, must be called with locks held.
    pub unsafe fn remove_dll_components_i(
        __this: *mut Self,
        mut dll_name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_Repository23remove_dll_components_iEPKc"]
            fn __ext(
                __this: *mut ACE_Framework_Repository,
                dll_name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Framework_Repository, dll_name)
    }
    /**Compact component_vector_ after components have been removed__maintains
  /// order.*/
    pub unsafe fn compact(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Framework_Repository7compactEv"]
            fn __ext(__this: *mut ACE_Framework_Repository);
        }
        __ext(__this as *mut ACE_Framework_Repository)
    }
}
impl ACE_Guard_ACE_Thread_Mutex_ {
    /**Implicitly and automatically acquire (or try to acquire) the
  /// lock.  If @a block is non-0 then acquire() the ACE_LOCK, else
  /// tryacquire() it.*/
    pub unsafe fn new_at_sd4259c8596315ca4(
        __this: *mut Self,
        mut __a0: *mut ACE_Thread_Mutex,
        mut __a1: bool,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI16ACE_Thread_MutexEC1ERS0_b"]
            fn __ext(
                __this: *mut ACE_Guard_ACE_Thread_Mutex_,
                __a0: *mut ACE_Thread_Mutex,
                __a1: bool,
            );
        }
        __ext(__this as *mut ACE_Guard_ACE_Thread_Mutex_, __a0, __a1)
    }
    pub unsafe fn new_sd4259c8596315ca4(
        mut __a0: *mut ACE_Thread_Mutex,
        mut __a1: bool,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sd4259c8596315ca4(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Initialize the guard without implicitly acquiring the lock. The
  /// @a become_owner parameter indicates whether the guard should release
  /// the lock implicitly on destruction. The @a block parameter is
  /// ignored and is used here to disambiguate with the preceding
  /// constructor.*/
    pub unsafe fn new_at_s2c939919fb7d55f9(
        __this: *mut Self,
        mut __a0: *mut ACE_Thread_Mutex,
        mut __a1: bool,
        mut __a2: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI16ACE_Thread_MutexEC1ERS0_bi"]
            fn __ext(
                __this: *mut ACE_Guard_ACE_Thread_Mutex_,
                __a0: *mut ACE_Thread_Mutex,
                __a1: bool,
                __a2: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Guard_ACE_Thread_Mutex_, __a0, __a1, __a2)
    }
    pub unsafe fn new_s2c939919fb7d55f9(
        mut __a0: *mut ACE_Thread_Mutex,
        mut __a1: bool,
        mut __a2: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s2c939919fb7d55f9(
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
            #[link_name = "_ZN9ACE_GuardI16ACE_Thread_MutexE10tryacquireEv"]
            fn __ext(__this: *mut ACE_Guard_ACE_Thread_Mutex_) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Guard_ACE_Thread_Mutex_)
    }
    /**Relinquish ownership of the lock so that it is not released
  /// implicitly in the destructor.*/
    pub unsafe fn disown(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI16ACE_Thread_MutexE6disownEv"]
            fn __ext(__this: *mut ACE_Guard_ACE_Thread_Mutex_);
        }
        __ext(__this as *mut ACE_Guard_ACE_Thread_Mutex_)
    }
    ///Explicitly remove the lock.
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI16ACE_Thread_MutexE6removeEv"]
            fn __ext(__this: *mut ACE_Guard_ACE_Thread_Mutex_) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Guard_ACE_Thread_Mutex_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK9ACE_GuardI16ACE_Thread_MutexE4dumpEv"]
            fn __ext(__this: *const ACE_Guard_ACE_Thread_Mutex_);
        }
        __ext(__this as *const ACE_Guard_ACE_Thread_Mutex_)
    }
    ///Helper, meant for subclass only.
    pub unsafe fn new_at_s3dee0e9c3f55fa29(
        __this: *mut Self,
        mut lock: *mut ACE_Thread_Mutex,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).lock_),
                ((lock) as *mut ACE_Thread_Mutex),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).owner_), 0);
            {}
            ()
        }
    }
    pub unsafe fn new_s3dee0e9c3f55fa29(mut __a0: *mut ACE_Thread_Mutex) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s3dee0e9c3f55fa29(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *mut ACE_Guard_ACE_Thread_Mutex_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI16ACE_Thread_MutexEaSERKS1_"]
            fn __ext(
                __this: *mut ACE_Guard_ACE_Thread_Mutex_,
                _anon_0: *mut ACE_Guard_ACE_Thread_Mutex_,
            );
        }
        __ext(__this as *mut ACE_Guard_ACE_Thread_Mutex_, _anon_0)
    }
    pub unsafe fn new_at_s1eda1ac49f200919(
        __this: *mut Self,
        mut __a0: *const ACE_Guard_ACE_Thread_Mutex_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN9ACE_GuardI16ACE_Thread_MutexEC1ERKS1_"]
            fn __ext(
                __this: *mut ACE_Guard_ACE_Thread_Mutex_,
                __a0: *const ACE_Guard_ACE_Thread_Mutex_,
            );
        }
        __ext(__this as *mut ACE_Guard_ACE_Thread_Mutex_, __a0)
    }
    pub unsafe fn new_s1eda1ac49f200919(
        mut __a0: *const ACE_Guard_ACE_Thread_Mutex_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s1eda1ac49f200919(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///Explicitly acquire the lock.
    pub unsafe fn acquire(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return {
                    let __v = <ACE_Thread_Mutex>::acquire(
                        ((*__this).lock_) as *mut ACE_Thread_Mutex,
                    );
                    (*__this).owner_ = __v;
                    __v
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn new_at_s194b481c4e491c25(
        __this: *mut Self,
        mut l: *mut ACE_Thread_Mutex,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).lock_),
                ::core::ptr::addr_of_mut!((* l)) as *mut ACE_Thread_Mutex,
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).owner_), 0);
            {
                <ACE_Guard_ACE_Thread_Mutex_>::acquire(
                    (__this) as *mut ACE_Guard_ACE_Thread_Mutex_,
                );
            }
            ()
        }
    }
    pub unsafe fn new_s194b481c4e491c25(mut __a0: *mut ACE_Thread_Mutex) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s194b481c4e491c25(::core::ptr::addr_of_mut!(__obj), __a0);
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
                    return <ACE_Thread_Mutex>::release(
                        ((*__this).lock_) as *mut ACE_Thread_Mutex,
                    );
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Service_Type_Factory {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Location_Node,
        mut __a3: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Service_Type_FactoryC1EPKciP17ACE_Location_Nodei"]
            fn __ext(
                __this: *mut ACE_Service_Type_Factory,
                __a0: *const libc::c_char,
                __a1: libc::c_int,
                __a2: *mut ACE_Location_Node,
                __a3: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Service_Type_Factory, __a0, __a1, __a2, __a3)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_int,
        mut __a2: *mut ACE_Location_Node,
        mut __a3: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1, __a2, __a3);
        __obj
    }
    pub unsafe fn make_service_type(
        __this: *const Self,
        mut pcfg: *mut ACE_Service_Gestalt,
    ) -> *mut ACE_Service_Type {
        extern "C-unwind" {
            #[link_name = "_ZNK24ACE_Service_Type_Factory17make_service_typeEP19ACE_Service_Gestalt"]
            fn __ext(
                __this: *const ACE_Service_Type_Factory,
                pcfg: *mut ACE_Service_Gestalt,
            ) -> *mut ACE_Service_Type;
        }
        __ext(__this as *const ACE_Service_Type_Factory, pcfg)
    }
    pub unsafe fn name(__this: *const Self) -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZNK24ACE_Service_Type_Factory4nameEv"]
            fn __ext(__this: *const ACE_Service_Type_Factory) -> *const libc::c_char;
        }
        __ext(__this as *const ACE_Service_Type_Factory)
    }
    ///* Not implemented to enforce no copying
    pub unsafe fn new_at_u7b244a0512a1700c(
        __this: *mut Self,
        mut __a0: *const ACE_Service_Type_Factory,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Service_Type_FactoryC1ERKS_"]
            fn __ext(
                __this: *mut ACE_Service_Type_Factory,
                __a0: *const ACE_Service_Type_Factory,
            );
        }
        __ext(__this as *mut ACE_Service_Type_Factory, __a0)
    }
    pub unsafe fn new_u7b244a0512a1700c(
        mut __a0: *const ACE_Service_Type_Factory,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u7b244a0512a1700c(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    ///* Not implemented to enforce no copying
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Service_Type_Factory,
    ) -> *mut ACE_Service_Type_Factory {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Service_Type_FactoryaSERKS_"]
            fn __ext(
                __this: *mut ACE_Service_Type_Factory,
                _anon_0: *const ACE_Service_Type_Factory,
            ) -> *mut ACE_Service_Type_Factory;
        }
        __ext(__this as *mut ACE_Service_Type_Factory, _anon_0)
    }
}
impl ACE_Location_Node {
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Location_NodeC1Ev"]
            fn __ext(__this: *mut ACE_Location_Node);
        }
        __ext(__this as *mut ACE_Location_Node)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    pub unsafe fn dll(__this: *mut Self) -> *const ACE_DLL {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Location_Node3dllEv"]
            fn __ext(__this: *mut ACE_Location_Node) -> *const ACE_DLL;
        }
        __ext(__this as *mut ACE_Location_Node)
    }
    pub unsafe fn pathname(__this: *const Self) -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Location_Node8pathnameEv"]
            fn __ext(__this: *const ACE_Location_Node) -> *const libc::c_char;
        }
        __ext(__this as *const ACE_Location_Node)
    }
    pub unsafe fn pathname_u3f760b15bc3e6a96(
        __this: *mut Self,
        mut h: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Location_Node8pathnameEPKc"]
            fn __ext(__this: *mut ACE_Location_Node, h: *const libc::c_char);
        }
        __ext(__this as *mut ACE_Location_Node, h)
    }
    pub unsafe fn dispose(__this: *const Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Location_Node7disposeEv"]
            fn __ext(__this: *const ACE_Location_Node) -> libc::c_int;
        }
        __ext(__this as *const ACE_Location_Node)
    }
    pub unsafe fn set_symbol(__this: *mut Self, mut h: *mut libc::c_void) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Location_Node10set_symbolEPv"]
            fn __ext(__this: *mut ACE_Location_Node, h: *mut libc::c_void);
        }
        __ext(__this as *mut ACE_Location_Node, h)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Location_Node4dumpEv"]
            fn __ext(__this: *const ACE_Location_Node);
        }
        __ext(__this as *const ACE_Location_Node)
    }
    pub unsafe fn open_dll(
        __this: *mut Self,
        mut yyerrno: *mut libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Location_Node8open_dllERi"]
            fn __ext(
                __this: *mut ACE_Location_Node,
                yyerrno: *mut libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Location_Node, yyerrno)
    }
    pub unsafe fn new_at_u6ac93af70e8c909e(
        __this: *mut Self,
        mut __a0: *const ACE_Location_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Location_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Location_Node, __a0: *const ACE_Location_Node);
        }
        __ext(__this as *mut ACE_Location_Node, __a0)
    }
    pub unsafe fn new_u6ac93af70e8c909e(mut __a0: *const ACE_Location_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u6ac93af70e8c909e(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Location_Node,
    ) -> *mut ACE_Location_Node {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Location_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Location_Node,
                _anon_0: *const ACE_Location_Node,
            ) -> *mut ACE_Location_Node;
        }
        __ext(__this as *mut ACE_Location_Node, _anon_0)
    }
}
impl ACE_Static_Svc_Descriptor {
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK25ACE_Static_Svc_Descriptor4dumpEv"]
            fn __ext(__this: *const ACE_Static_Svc_Descriptor);
        }
        __ext(__this as *const ACE_Static_Svc_Descriptor)
    }
    ///Compare two service descriptors for equality.
    pub unsafe fn operator_eq(
        __this: *const Self,
        mut d: *mut ACE_Static_Svc_Descriptor,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                    (((*__this).name_) as *const libc::c_char),
                    (((*d).name_) as *const libc::c_char),
                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Compare two service descriptors for inequality.
    pub unsafe fn operator_ne(
        __this: *const Self,
        mut d: *mut ACE_Static_Svc_Descriptor,
    ) -> bool {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((!((((<ACE_Static_Svc_Descriptor>::operator_eq(
                    (__this) as *const ACE_Static_Svc_Descriptor,
                    ::core::ptr::addr_of_mut!((* d)),
                )) as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Svc_Conf_Param {
    ///Constructor
    pub unsafe fn new_at(
        __this: *mut Self,
        mut gestalt: *mut ACE_Service_Gestalt,
        mut file: *mut _IO_FILE,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).r#type),
                ((SVC_CONF_FILE) as libc::c_uint),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).yyerrno), 0);
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).yylineno), 1);
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).buffer),
                ((0) as *mut ace_yy_buffer_state),
            );
            <ACE_Obstack_T_char_>::new_at_se16342d999bb447e(
                (::core::ptr::addr_of_mut!((* __this).obstack)
                    .cast::<ACE_Obstack_T_char_>()) as *mut ACE_Obstack_T_char_,
                (((((((((4096) as libc::c_ulong)).wrapping_mul((1) as libc::c_ulong)))
                    as libc::c_ulong))
                    .wrapping_sub((40) as libc::c_ulong)) as libc::c_ulong),
                ((0) as *mut ACE_Allocator),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).config), gestalt);
            {
                (*__this).source.file = file;
            }
            ()
        }
    }
    pub unsafe fn new(
        mut __a0: *mut ACE_Service_Gestalt,
        mut __a1: *mut _IO_FILE,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Constructor
    pub unsafe fn new_at_u9d864a4bc56ea6c7(
        __this: *mut Self,
        mut gestalt: *mut ACE_Service_Gestalt,
        mut directive: *const libc::c_char,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).r#type),
                ((SVC_CONF_DIRECTIVE) as libc::c_uint),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).yyerrno), 0);
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).yylineno), 1);
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).buffer),
                ((0) as *mut ace_yy_buffer_state),
            );
            <ACE_Obstack_T_char_>::new_at_se16342d999bb447e(
                (::core::ptr::addr_of_mut!((* __this).obstack)
                    .cast::<ACE_Obstack_T_char_>()) as *mut ACE_Obstack_T_char_,
                (((((((((4096) as libc::c_ulong)).wrapping_mul((1) as libc::c_ulong)))
                    as libc::c_ulong))
                    .wrapping_sub((40) as libc::c_ulong)) as libc::c_ulong),
                ((0) as *mut ACE_Allocator),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).config), gestalt);
            {
                (*__this).source.directive = directive;
            }
            ()
        }
    }
    pub unsafe fn new_u9d864a4bc56ea6c7(
        mut __a0: *mut ACE_Service_Gestalt,
        mut __a1: *const libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u9d864a4bc56ea6c7(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
}
impl Processed_Static_Svc {
    pub unsafe fn new_at(__this: *mut Self, mut __a0: *const ACE_Static_Svc_Descriptor) {
        extern "C-unwind" {
            #[link_name = "_ZN19ACE_Service_Gestalt20Processed_Static_SvcC1EPK25ACE_Static_Svc_Descriptor"]
            fn __ext(
                __this: *mut Processed_Static_Svc,
                __a0: *const ACE_Static_Svc_Descriptor,
            );
        }
        __ext(__this as *mut Processed_Static_Svc, __a0)
    }
    pub unsafe fn new(mut __a0: *const ACE_Static_Svc_Descriptor) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_Service_Type_Dynamic_Guard {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *mut ACE_Service_Repository,
        mut __a1: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN30ACE_Service_Type_Dynamic_GuardC1ER22ACE_Service_RepositoryPKc"]
            fn __ext(
                __this: *mut ACE_Service_Type_Dynamic_Guard,
                __a0: *mut ACE_Service_Repository,
                __a1: *const libc::c_char,
            );
        }
        __ext(__this as *mut ACE_Service_Type_Dynamic_Guard, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *mut ACE_Service_Repository,
        mut __a1: *const libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
}
impl ACE_Threading_Helper_ACE_Thread_Mutex_ {
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_Threading_HelperI16ACE_Thread_MutexEC1Ev"]
            fn __ext(__this: *mut ACE_Threading_Helper_ACE_Thread_Mutex_);
        }
        __ext(__this as *mut ACE_Threading_Helper_ACE_Thread_Mutex_)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    pub unsafe fn set(__this: *mut Self, mut _anon_0: *mut libc::c_void) {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_Threading_HelperI16ACE_Thread_MutexE3setEPv"]
            fn __ext(
                __this: *mut ACE_Threading_Helper_ACE_Thread_Mutex_,
                _anon_0: *mut libc::c_void,
            );
        }
        __ext(__this as *mut ACE_Threading_Helper_ACE_Thread_Mutex_, _anon_0)
    }
    pub unsafe fn get(__this: *mut Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_Threading_HelperI16ACE_Thread_MutexE3getEv"]
            fn __ext(
                __this: *mut ACE_Threading_Helper_ACE_Thread_Mutex_,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Threading_Helper_ACE_Thread_Mutex_)
    }
}
impl ACE_Threading_Helper_ACE_Null_Mutex_ {
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_Threading_HelperI14ACE_Null_MutexEC1Ev"]
            fn __ext(__this: *mut ACE_Threading_Helper_ACE_Null_Mutex_);
        }
        __ext(__this as *mut ACE_Threading_Helper_ACE_Null_Mutex_)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    pub unsafe fn set(__this: *mut Self, mut _anon_0: *mut libc::c_void) {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_Threading_HelperI14ACE_Null_MutexE3setEPv"]
            fn __ext(
                __this: *mut ACE_Threading_Helper_ACE_Null_Mutex_,
                _anon_0: *mut libc::c_void,
            );
        }
        __ext(__this as *mut ACE_Threading_Helper_ACE_Null_Mutex_, _anon_0)
    }
    pub unsafe fn get(__this: *mut Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_Threading_HelperI14ACE_Null_MutexE3getEv"]
            fn __ext(
                __this: *mut ACE_Threading_Helper_ACE_Null_Mutex_,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Threading_Helper_ACE_Null_Mutex_)
    }
}
impl ACE_Service_Config {
    #[doc = "* Initialize the Service Repository. Note that initialising @a\n   * signum to a negative number will prevent a signal handler being\n   * registered when the repository is opened."]
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: bool,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_ConfigC1Ebmi"]
            fn __ext(
                __this: *mut ACE_Service_Config,
                __a0: bool,
                __a1: libc::c_ulong,
                __a2: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Service_Config, __a0, __a1, __a2)
    }
    pub unsafe fn new(
        mut __a0: bool,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1, __a2);
        __obj
    }
    #[doc = "* Performs an open without parsing command-line arguments.  The\n   * @a logger_key indicates where to write the logging output, which\n   * is typically either a STREAM pipe or a socket address."]
    pub unsafe fn new_at_u91c6b7308e4fc34b(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_ConfigC1EPKcS1_"]
            fn __ext(
                __this: *mut ACE_Service_Config,
                __a0: *const libc::c_char,
                __a1: *const libc::c_char,
            );
        }
        __ext(__this as *mut ACE_Service_Config, __a0, __a1)
    }
    pub unsafe fn new_u91c6b7308e4fc34b(
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u91c6b7308e4fc34b(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    #[doc = "* Performs an open without parsing command-line arguments.\n   * Implements whats different in the opening sequence\n   * for this class, as opposed to the base class.\n   *\n   * The @a logger_key indicates where to write the logging output, which\n   * is typically either a STREAM pipe or a socket address.  If\n   * @a ignore_default_svc_conf_file is non-0 then the \"svc.conf\" file\n   * will be ignored.  If @a ignore_debug_flag is non-0 then the\n   * application is responsible for setting the\n   * @c ACE_Log_Msg::priority_mask() appropriately.  Returns number of\n   * errors that occurred on failure and 0 otherwise."]
    pub unsafe fn open_i(
        __this: *mut Self,
        mut program_name: *const libc::c_char,
        mut logger_key: *const libc::c_char,
        mut ignore_static_svcs: bool,
        mut ignore_default_svc_conf_file: bool,
        mut ignore_debug_flag: bool,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config6open_iEPKcS1_bbb"]
            fn __ext(
                __this: *mut ACE_Service_Config,
                program_name: *const libc::c_char,
                logger_key: *const libc::c_char,
                ignore_static_svcs: bool,
                ignore_default_svc_conf_file: bool,
                ignore_debug_flag: bool,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_Service_Config,
            program_name,
            logger_key,
            ignore_static_svcs,
            ignore_default_svc_conf_file,
            ignore_debug_flag,
        )
    }
    #[doc = "* Implements whats different in the command line parameter processing\n   * for this class, as opposed to the base class."]
    pub unsafe fn parse_args_i(
        __this: *mut Self,
        mut argc: libc::c_int,
        mut argv: *mut *mut libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config12parse_args_iEiPPc"]
            fn __ext(
                __this: *mut ACE_Service_Config,
                argc: libc::c_int,
                argv: *mut *mut libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Service_Config, argc, argv)
    }
    #[doc = "* Returns the process-wide global singleton instance. It would\n   * have been created and will be managed by the Object Manager."]
    pub unsafe fn singleton() -> *mut ACE_Service_Config {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config9singletonEv"]
            fn __ext() -> *mut ACE_Service_Config;
        }
        __ext()
    }
    #[doc = "* Mutator for the currently active configuration context instance\n   * (gestalt). Intended for use by helper classes like @see\n   * ACE_Service_Config_Guard. Stack-based instances can be used to\n   * temporarily change which gestalt is seen as global by static\n   * initializers (especially those in DLLs loaded at run-time)."]
    pub unsafe fn current(mut _anon_0: *mut ACE_Service_Gestalt) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config7currentEP19ACE_Service_Gestalt"]
            fn __ext(_anon_0: *mut ACE_Service_Gestalt);
        }
        __ext(_anon_0)
    }
    ///* Accessor for the "current" service gestalt
    pub unsafe fn current_ud6b3b837e8530981() -> *mut ACE_Service_Gestalt {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config7currentEv"]
            fn __ext() -> *mut ACE_Service_Gestalt;
        }
        __ext()
    }
    #[doc = "* This is what the static service initializators are hard-wired to\n   * use, so in order to avoid interface changes this method merely\n   * forwards to @c ACE_Service_Config::current. This enables us to\n   * enforce which Service Gestalt is used for services registering\n   * through static initializers. Especially important for DLL-based\n   * dynamic services, which can contain their own static services and\n   * static initializers.\n   *\n   * @deprecated Use current() instead."]
    pub unsafe fn instance() -> *mut ACE_Service_Gestalt {
        unsafe {
            {
                return <ACE_Service_Config>::current_ud6b3b837e8530981();
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Returns a process-wide global singleton instance in contrast with\n   * current (), which may return a different instance at different\n   * times, dependent on the context. Modifying this method's return\n   * value is strongly discouraged as it will circumvent the mechanism\n   * for dynamically loading services. If you must, use with extreme\n   * caution!"]
    pub unsafe fn global() -> *mut ACE_Service_Gestalt {
        unsafe {
            {
                return <ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_>::get(
                    (::core::ptr::addr_of!(
                        (* ::core::ptr::addr_of!((* < ACE_Service_Config > ::singleton())
                        .instance_) .cast:: < ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_
                        > ().cast_mut())
                    )) as *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Performs an open without parsing command-line arguments.  The\n   * @a logger_key indicates where to write the logging output, which\n   * is typically either a STREAM pipe or a socket address.  If\n   * @a ignore_static_svcs is true then static services are not loaded,\n   * otherwise, they are loaded.  If @a ignore_default_svc_conf_file is\n   * non-0 then the <svc.conf> configuration file will be ignored.\n   * Returns zero upon success, -1 if the file is not found or cannot\n   * be opened (errno is set accordingly), otherwise returns the\n   * number of errors encountered loading the services in the\n   * specified svc.conf configuration file.  If @a ignore_debug_flag is\n   * non-0 then the application is responsible for setting the\n   * @c ACE_Log_Msg::priority_mask appropriately."]
    pub unsafe fn open(
        mut program_name: *const libc::c_char,
        mut logger_key: *const libc::c_char,
        mut ignore_static_svcs: bool,
        mut ignore_default_svc_conf: bool,
        mut ignore_debug_flag: bool,
    ) -> libc::c_int {
        unsafe {
            {
                if ((((({
                    let __obj: *mut ACE_Service_Config = (<ACE_Service_Config>::singleton())
                        as *mut ACE_Service_Config;
                    let __vt: *const __Vtbl_uf6956a2932fdd159 = *(__obj
                        as *const *const __Vtbl_uf6956a2932fdd159);
                    ((*__vt)
                        .vfn_u2de6f86bd6d76c3a)(
                        __obj,
                        ((program_name) as *const libc::c_char),
                        logger_key,
                        ignore_static_svcs,
                        ignore_default_svc_conf,
                        ignore_debug_flag,
                    )
                } as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                return <ACE_Service_Gestalt>::open(
                    (<ACE_Service_Config>::current_ud6b3b837e8530981())
                        as *mut ACE_Service_Gestalt,
                    ((program_name) as *const libc::c_char),
                    logger_key,
                    ignore_static_svcs,
                    ignore_default_svc_conf,
                    ignore_debug_flag,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* This is the primary entry point into the ACE_Service_Config (the\n   * constructor just handles simple initializations).  It parses\n   * arguments passed in from @a argc and @a argv parameters.  The\n   * arguments that are valid in a call to this method include:\n   *\n   * - '-b' Option to indicate that we should be a daemon. Note that when\n   *        this option is used, the process will be daemonized before the\n   *        service configuration file(s) are read. During daemonization,\n   *        (on POSIX systems) the current directory will be changed to \"/\"\n   *        so the caller should either fully specify the file names, or\n   *        execute a @c chroot() to the appropriate directory.\n   *        @sa ACE::daemonize().\n   * - '-d' Turn on debugging mode\n   * - '-f' Specifies a configuration file name other than the default\n   *        svc.conf. Can be specified multiple times to use multiple files.\n   *        If any configuration file is provided with this option then\n   *        the default svc.conf will be ignored.\n   * - '-k' Specifies the rendezvous point to use for the ACE distributed\n   *        logger.\n   * - '-y' Explicitly enables the use of static services. This flag\n   *        overrides the @a ignore_static_svcs parameter value.\n   * - '-n' Explicitly disables the use of static services. This flag\n   *        overrides the @a ignore_static_svcs parameter value.\n   * - '-p' Specifies a pathname which is used to store the process id.\n   * - '-s' Specifies a signal number other than SIGHUP to trigger reprocessing\n   *        of the configuration file(s). Ignored for platforms that do not\n   *        have POSIX signals, such as Windows.\n   * - '-S' Specifies a service directive string. Enclose the string in quotes\n   *        and escape any embedded quotes with a backslash. This option\n   *        specifies service directives without the need for a configuration\n   *        file. Can be specified multiple times.\n   *\n   * Note: Options '-f' and '-S' complement each other. Directives from files\n   * and from '-S' option are processed together in the following order. First,\n   * all files are processed in the order they are specified in @a argv\n   * parameter. Second, all directive strings are executed in the order the\n   * directives appear in @a argv parameter.\n   *\n   * @param argc The number of commandline arguments.\n   * @param argv The array with commandline arguments\n   * @param logger_key   Indicates where to write the logging output,\n   *                     which is typically either a STREAM pipe or a\n   *                     socket address.\n   * @param ignore_static_svcs   If true then static services are not loaded,\n   *                             otherwise, they are loaded.\n   * @param ignore_default_svc_conf_file  If non-0 then the @c svc.conf\n   *                                      configuration file will be ignored.\n   * @param ignore_debug_flag If true then the application is responsible\n   *                          for setting the @c ACE_Log_Msg::priority_mask\n   *                          appropriately.\n   *\n   * @retval -1   The configuration file is not found or cannot\n   *              be opened (errno is set accordingly).\n   * @retval  0   Success.\n   * @retval  >0  The number of errors encountered while processing\n   *              the service configuration file(s)."]
    pub unsafe fn open_u2fd4a67101f8c9e2(
        mut argc: libc::c_int,
        mut argv: *mut *mut libc::c_char,
        mut logger_key: *const libc::c_char,
        mut ignore_static_svcs: bool,
        mut ignore_default_svc_conf: bool,
        mut ignore_debug_flag: bool,
    ) -> libc::c_int {
        unsafe {
            {
                if ((((({
                    let __obj: *mut ACE_Service_Config = (<ACE_Service_Config>::singleton())
                        as *mut ACE_Service_Config;
                    let __vt: *const __Vtbl_uf6956a2932fdd159 = *(__obj
                        as *const *const __Vtbl_uf6956a2932fdd159);
                    ((*__vt)
                        .vfn_ubec69e76eca3101a)(
                        __obj,
                        argc,
                        ((argv) as *mut *mut libc::c_char),
                    )
                } as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                if ((((({
                    let __obj: *mut ACE_Service_Config = (<ACE_Service_Config>::singleton())
                        as *mut ACE_Service_Config;
                    let __vt: *const __Vtbl_uf6956a2932fdd159 = *(__obj
                        as *const *const __Vtbl_uf6956a2932fdd159);
                    ((*__vt)
                        .vfn_u2de6f86bd6d76c3a)(
                        __obj,
                        (((*(argv).wrapping_offset((0) as isize)))
                            as *const libc::c_char),
                        logger_key,
                        ignore_static_svcs,
                        ignore_default_svc_conf,
                        ignore_debug_flag,
                    )
                } as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                return <ACE_Service_Gestalt>::open_uf8bfee49ed7e36e7(
                    (<ACE_Service_Config>::current_ud6b3b837e8530981())
                        as *mut ACE_Service_Gestalt,
                    argc,
                    ((argv) as *mut *mut libc::c_char),
                    logger_key,
                    ignore_static_svcs,
                    ignore_default_svc_conf,
                    ignore_debug_flag,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Tidy up and perform last rites when ACE_Service_Config is shut
  /// down.  This method calls close_svcs().  Returns 0.*/
    pub unsafe fn close() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config5closeEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    /**Perform user-specified close hooks and possibly delete all of the
  /// configured services in the <Service_Repository>.*/
    pub unsafe fn fini_svcs() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config9fini_svcsEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    ///True if reconfiguration occurred.
    pub unsafe fn reconfig_occurred() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config17reconfig_occurredEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    ///Indicate that reconfiguration occurred.
    pub unsafe fn reconfig_occurred_uea1fede1e609e40b(mut _anon_0: libc::c_int) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config17reconfig_occurredEi"]
            fn __ext(_anon_0: libc::c_int);
        }
        __ext(_anon_0)
    }
    ///Perform the reconfiguration process.
    pub unsafe fn reconfigure() {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config11reconfigureEv"]
            fn __ext();
        }
        __ext()
    }
    /**Returns a pointer to the list of statically linked services.
  ///
  /// @deprecated - Same as instance(), but still useful in legacy code,
  /// (notably, one that can not be easily modified) which uses the following
  /// idiom for registering static services:
  ///
  ///    ACE_Service_Config::static_svcs ()->insert (...);*/
    pub unsafe fn static_svcs() -> *mut ACE_Service_Gestalt {
        unsafe {
            {
                return <ACE_Service_Config>::current_ud6b3b837e8530981();
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Insert a static service descriptor for processing on open_i(). The
  /// corresponding ACE_STATIC_SVC_* macros were changed to use this method
  /// instead of obtaining a ptr to a container. See the note on static_svcs().
  /// Added to prevent exposing the internal storage representation of the
  /// services repository and provide a better way of debugging service
  /// loading and registration problems.*/
    pub unsafe fn insert(mut svc: *mut ACE_Static_Svc_Descriptor) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config6insertEP25ACE_Static_Svc_Descriptor"]
            fn __ext(svc: *mut ACE_Static_Svc_Descriptor) -> libc::c_int;
        }
        __ext(svc)
    }
    /**Dynamically link the shared object file and retrieve a pointer to
  /// the designated shared object in this file.*/
    pub unsafe fn initialize(
        mut sr: *const ACE_Service_Type,
        mut parameters: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            {
                return <ACE_Service_Gestalt>::initialize_ua4f900f5d00da189(
                    (<ACE_Service_Config>::current_ud6b3b837e8530981())
                        as *mut ACE_Service_Gestalt,
                    sr,
                    parameters,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Initialize and activate a statically @a svc_name service.
    pub unsafe fn initialize_ubd810b73c780d984(
        mut svc_name: *const libc::c_char,
        mut parameters: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            {
                return <ACE_Service_Gestalt>::initialize_u32660f5ed5aef539(
                    (<ACE_Service_Config>::current_ud6b3b837e8530981())
                        as *mut ACE_Service_Gestalt,
                    svc_name,
                    parameters,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Resume a @a svc_name that was previously suspended or has not yet
  /// been resumed (e.g., a static service).*/
    pub unsafe fn resume(mut svc_name: *const libc::c_char) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config6resumeEPKc"]
            fn __ext(svc_name: *const libc::c_char) -> libc::c_int;
        }
        __ext(svc_name)
    }
    #[doc = "* Suspend @a svc_name.  Note that this will not unlink the service\n   * from the daemon if it was dynamically linked, it will mark it as\n   * being suspended in the Service Repository and call the <suspend>\n   * member function on the appropriate ACE_Service_Object.  A\n   * service can be resumed later on by calling the <RESUME> member\n   * function..."]
    pub unsafe fn suspend(mut svc_name: *const libc::c_char) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config7suspendEPKc"]
            fn __ext(svc_name: *const libc::c_char) -> libc::c_int;
        }
        __ext(svc_name)
    }
    /**Totally remove @a svc_name from the daemon by removing it
  /// from the ACE_Reactor, and unlinking it if necessary.*/
    pub unsafe fn remove(mut svc_name: *const libc::c_char) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config6removeEPKc"]
            fn __ext(svc_name: *const libc::c_char) -> libc::c_int;
        }
        __ext(svc_name)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Service_Config4dumpEv"]
            fn __ext(__this: *const ACE_Service_Config);
        }
        __ext(__this as *const ACE_Service_Config)
    }
    ///Set the signal_handler for internal use by ACE_Object_Manager only.
    pub unsafe fn signal_handler(mut signal_handler: *mut ACE_Sig_Adapter) {
        unsafe {
            {
                ACE_Service_Config_signal_handler_ = signal_handler;
            }
            ()
        }
    }
    /**Process a file containing a list of service configuration
  /// directives.*/
    pub unsafe fn process_file(mut file: *const libc::c_char) -> libc::c_int {
        unsafe {
            {
                return <ACE_Service_Gestalt>::process_file(
                    (<ACE_Service_Config>::current_ud6b3b837e8530981())
                        as *mut ACE_Service_Gestalt,
                    ((file) as *const libc::c_char),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Process one service configuration @a directive, which is passed as
  /// a string.  Returns the number of errors that occurred.*/
    pub unsafe fn process_directive(mut directive: *const libc::c_char) -> libc::c_int {
        unsafe {
            {
                return <ACE_Service_Gestalt>::process_directive(
                    (<ACE_Service_Config>::current_ud6b3b837e8530981())
                        as *mut ACE_Service_Gestalt,
                    ((directive) as *const libc::c_char),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Process one static service definition.  Load a new static service\n   * into the ACE_Service_Repository.\n   *\n   * @param ssd Service descriptor, see the document of\n   *        ACE_Static_Svc_Descriptor for more details.\n   *\n   * @param force_replace If set the new service descriptor replaces\n   *        any previous instance in the ACE_Service_Repository.\n   *\n   * @return Returns -1 if the service cannot be 'loaded'."]
    pub unsafe fn process_directive_u4827950c0b91bcde(
        mut ssd: *const ACE_Static_Svc_Descriptor,
        mut force_replace: bool,
    ) -> libc::c_int {
        unsafe {
            {
                return <ACE_Service_Gestalt>::process_directive_u897e71149117fba7(
                    (<ACE_Service_Config>::current_ud6b3b837e8530981())
                        as *mut ACE_Service_Gestalt,
                    ::core::ptr::addr_of!((* ssd)),
                    force_replace,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Process (or re-process) service configuration requests that are\n   * provided in the svc.conf file(s).  Returns the number of errors\n   * that occurred."]
    pub unsafe fn process_directives() -> libc::c_int {
        unsafe {
            {
                return <ACE_Service_Gestalt>::process_directives(
                    (<ACE_Service_Config>::current_ud6b3b837e8530981())
                        as *mut ACE_Service_Gestalt,
                    false,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Handles signals to trigger reconfigurations.
    pub unsafe fn handle_signal(
        mut sig: libc::c_int,
        mut _anon_1: *mut siginfo_t,
        mut _anon_2: *mut ucontext_t,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config13handle_signalEiP9siginfo_tP10ucontext_t"]
            fn __ext(
                sig: libc::c_int,
                _anon_1: *mut siginfo_t,
                _anon_2: *mut ucontext_t,
            );
        }
        __ext(sig, _anon_1, _anon_2)
    }
    #[doc = "* Handle the command-line options intended for the\n   * ACE_Service_Config.  Note that @c argv[0] is assumed to be the\n   * program name.\n   * The arguments that are valid in a call to this method are\n   * - '-b' Option to indicate that we should be a daemon\n   * - '-d' Turn on debugging mode\n   * - '-f' Option to read in the list of svc.conf file names\n   * - '-k' Option to read a wide string where in the logger output can\n   *        be written\n   * - '-y' Turn on the flag for a  repository of statically\n   *        linked services\n   * - '-n' Need not have a repository of statically linked services\n   * - '-S' Option to read in the list of services on the command-line\n   *        Please observe the difference between options '-f' that looks\n   *        for a list of files and here a list of services."]
    pub unsafe fn parse_args(
        mut argc: libc::c_int,
        mut argv: *mut *mut libc::c_char,
    ) -> libc::c_int {
        unsafe {
            {
                return <ACE_Service_Gestalt>::parse_args(
                    (<ACE_Service_Config>::current_ud6b3b837e8530981())
                        as *mut ACE_Service_Gestalt,
                    argc,
                    ((argv) as *mut *mut libc::c_char),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn create_service_type_impl(
        mut name: *const libc::c_char,
        mut r#type: libc::c_int,
        mut symbol: *mut libc::c_void,
        mut flags: libc::c_uint,
        mut gobbler: Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut ACE_Service_Type_Impl {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config24create_service_type_implEPKciPvjPFvS2_E"]
            fn __ext(
                name: *const libc::c_char,
                r#type: libc::c_int,
                symbol: *mut libc::c_void,
                flags: libc::c_uint,
                gobbler: Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
            ) -> *mut ACE_Service_Type_Impl;
        }
        __ext(name, r#type, symbol, flags, gobbler)
    }
    /**@deprecated
  /// Process service configuration requests that were provided on the
  /// command-line.  Returns the number of errors that occurred.*/
    pub unsafe fn process_commandline_directives() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config30process_commandline_directivesEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    ///Become a daemon.
    pub unsafe fn start_daemon() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config12start_daemonEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    pub unsafe fn load_static_svcs() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config16load_static_svcsEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    /**@deprecated
  /// This is the implementation function that process_directives()
  /// and process_directive() both call.  Returns the number of errors
  /// that occurred.*/
    pub unsafe fn process_directives_i(
        mut param: *mut ACE_Svc_Conf_Param,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Service_Config20process_directives_iEP18ACE_Svc_Conf_Param"]
            fn __ext(param: *mut ACE_Svc_Conf_Param) -> libc::c_int;
        }
        __ext(param)
    }
}
impl ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_ {
    ///Used to define a proper boolean conversion for "if (sp) ..."
    pub unsafe fn unspecified_bool(
        mut _anon_0: *mut *mut *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
    ) {
        unsafe {
            {}
            ()
        }
    }
    ///Enables "if (sp) ..."
    pub unsafe fn operator_void_____ACE_Intrusive_Auto_Ptr_X______(
        __this: *const Self,
    ) -> Option<
        unsafe extern "C-unwind" fn(
            *mut *mut *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
        ),
    > {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (if ((((((*__this).rep_).is_null()) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    None
                } else {
                    Some({
                        unsafe extern "C-unwind" fn __shim(
                            a0: *mut *mut *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
                        ) {
                            unsafe {
                                ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_::unspecified_bool(
                                    a0,
                                )
                            }
                        }
                        __shim
                            as unsafe extern "C-unwind" fn(
                                *mut *mut *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
                            )
                    })
                });
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Constructor that initializes an ACE_Intrusive_Auto_Ptr to
  /// the specified pointer value.*/
    pub unsafe fn new_at_s8689d5711c8a8649(
        __this: *mut Self,
        mut __a0: *mut ACE_Service_Gestalt,
        mut __a1: bool,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Intrusive_Auto_PtrI19ACE_Service_GestaltEC1EPS0_b"]
            fn __ext(
                __this: *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
                __a0: *mut ACE_Service_Gestalt,
                __a1: bool,
            );
        }
        __ext(__this as *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_, __a0, __a1)
    }
    pub unsafe fn new_s8689d5711c8a8649(
        mut __a0: *mut ACE_Service_Gestalt,
        mut __a1: bool,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s8689d5711c8a8649(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Copy constructor binds the new ACE_Intrusive_Auto_Ptr to the
  /// representation object referenced by @a r.
  /// An ACE_Intrusive_Auto_Ptr_Rep is created if necessary.*/
    pub unsafe fn new_at_s07ba858f567f4433(
        __this: *mut Self,
        mut __a0: *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Intrusive_Auto_PtrI19ACE_Service_GestaltEC1ERKS1_"]
            fn __ext(
                __this: *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
                __a0: *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
            );
        }
        __ext(__this as *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_, __a0)
    }
    pub unsafe fn new_s07ba858f567f4433(
        mut __a0: *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s07ba858f567f4433(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    /**Assignment operator that binds the current object and @a r to the same
  /// ACE_Intrusive_Auto_Ptr_Rep. An ACE_Intrusive_Auto_Ptr_Rep
  /// is created if necessary.*/
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut r: *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Intrusive_Auto_PtrI19ACE_Service_GestaltEaSERKS1_"]
            fn __ext(
                __this: *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
                r: *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
            );
        }
        __ext(__this as *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_, r)
    }
    ///Redirection operator
    pub unsafe fn operator_arrow(__this: *const Self) -> *mut ACE_Service_Gestalt {
        extern "C-unwind" {
            #[link_name = "_ZNK22ACE_Intrusive_Auto_PtrI19ACE_Service_GestaltEptEv"]
            fn __ext(
                __this: *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
            ) -> *mut ACE_Service_Gestalt;
        }
        __ext(__this as *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_)
    }
    ///Accessor method.
    pub unsafe fn operator_mul(__this: *const Self) -> *mut ACE_Service_Gestalt {
        extern "C-unwind" {
            #[link_name = "_ZNK22ACE_Intrusive_Auto_PtrI19ACE_Service_GestaltEdeEv"]
            fn __ext(
                __this: *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
            ) -> *mut ACE_Service_Gestalt;
        }
        __ext(__this as *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_)
    }
    /**Releases the reference to the underlying representation object.
  /// @retval The pointer value prior to releasing it.*/
    pub unsafe fn release(__this: *mut Self) -> *mut ACE_Service_Gestalt {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Intrusive_Auto_PtrI19ACE_Service_GestaltE7releaseEv"]
            fn __ext(
                __this: *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
            ) -> *mut ACE_Service_Gestalt;
        }
        __ext(__this as *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_)
    }
    /**Releases the current pointer value and then sets a new
  /// pointer value specified by @a p.*/
    pub unsafe fn reset(__this: *mut Self, mut p: *mut ACE_Service_Gestalt) {
        extern "C-unwind" {
            #[link_name = "_ZN22ACE_Intrusive_Auto_PtrI19ACE_Service_GestaltE5resetEPS0_"]
            fn __ext(
                __this: *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
                p: *mut ACE_Service_Gestalt,
            );
        }
        __ext(__this as *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_, p)
    }
    ///Get the reference count value.
    pub unsafe fn count(__this: *const Self) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZNK22ACE_Intrusive_Auto_PtrI19ACE_Service_GestaltE5countEv"]
            fn __ext(
                __this: *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
            ) -> libc::c_long;
        }
        __ext(__this as *const ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_)
    }
    ///Get the pointer value.
    pub unsafe fn get(__this: *const Self) -> *mut ACE_Service_Gestalt {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).rep_) as *mut ACE_Service_Gestalt);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Service_Config_Guard {
    pub unsafe fn new_at(__this: *mut Self, mut __a0: *mut ACE_Service_Gestalt) {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Service_Config_GuardC1EP19ACE_Service_Gestalt"]
            fn __ext(
                __this: *mut ACE_Service_Config_Guard,
                __a0: *mut ACE_Service_Gestalt,
            );
        }
        __ext(__this as *mut ACE_Service_Config_Guard, __a0)
    }
    pub unsafe fn new(mut __a0: *mut ACE_Service_Gestalt) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn new_at_u2c18a94a8588237a(
        __this: *mut Self,
        mut __a0: *const ACE_Service_Config_Guard,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Service_Config_GuardC1ERKS_"]
            fn __ext(
                __this: *mut ACE_Service_Config_Guard,
                __a0: *const ACE_Service_Config_Guard,
            );
        }
        __ext(__this as *mut ACE_Service_Config_Guard, __a0)
    }
    pub unsafe fn new_u2c18a94a8588237a(
        mut __a0: *const ACE_Service_Config_Guard,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u2c18a94a8588237a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Service_Config_Guard,
    ) -> *mut ACE_Service_Config_Guard {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Service_Config_GuardaSERKS_"]
            fn __ext(
                __this: *mut ACE_Service_Config_Guard,
                _anon_0: *const ACE_Service_Config_Guard,
            ) -> *mut ACE_Service_Config_Guard;
        }
        __ext(__this as *mut ACE_Service_Config_Guard, _anon_0)
    }
}
impl ACE_Parse_Node {
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Parse_NodeC1Ev"]
            fn __ext(__this: *mut ACE_Parse_Node);
        }
        __ext(__this as *mut ACE_Parse_Node)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    pub unsafe fn new_at_u929c343393ac2710(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Parse_NodeC1EPKc"]
            fn __ext(__this: *mut ACE_Parse_Node, __a0: *const libc::c_char);
        }
        __ext(__this as *mut ACE_Parse_Node, __a0)
    }
    pub unsafe fn new_u929c343393ac2710(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u929c343393ac2710(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn link(__this: *const Self) -> *mut ACE_Parse_Node {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Parse_Node4linkEv"]
            fn __ext(__this: *const ACE_Parse_Node) -> *mut ACE_Parse_Node;
        }
        __ext(__this as *const ACE_Parse_Node)
    }
    pub unsafe fn link_u9307fa7c35e0a61d(
        __this: *mut Self,
        mut _anon_0: *mut ACE_Parse_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Parse_Node4linkEPS_"]
            fn __ext(__this: *mut ACE_Parse_Node, _anon_0: *mut ACE_Parse_Node);
        }
        __ext(__this as *mut ACE_Parse_Node, _anon_0)
    }
    pub unsafe fn name(__this: *const Self) -> *const libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Parse_Node4nameEv"]
            fn __ext(__this: *const ACE_Parse_Node) -> *const libc::c_char;
        }
        __ext(__this as *const ACE_Parse_Node)
    }
    pub unsafe fn print(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Parse_Node5printEv"]
            fn __ext(__this: *const ACE_Parse_Node);
        }
        __ext(__this as *const ACE_Parse_Node)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Parse_Node4dumpEv"]
            fn __ext(__this: *const ACE_Parse_Node);
        }
        __ext(__this as *const ACE_Parse_Node)
    }
    pub unsafe fn new_at_u7dc8712d740064d2(
        __this: *mut Self,
        mut __a0: *const ACE_Parse_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Parse_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Parse_Node, __a0: *const ACE_Parse_Node);
        }
        __ext(__this as *mut ACE_Parse_Node, __a0)
    }
    pub unsafe fn new_u7dc8712d740064d2(mut __a0: *const ACE_Parse_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u7dc8712d740064d2(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Parse_Node,
    ) -> *mut ACE_Parse_Node {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Parse_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Parse_Node,
                _anon_0: *const ACE_Parse_Node,
            ) -> *mut ACE_Parse_Node;
        }
        __ext(__this as *mut ACE_Parse_Node, _anon_0)
    }
}
impl ACE_Suspend_Node {
    pub unsafe fn new_at(__this: *mut Self, mut __a0: *const libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Suspend_NodeC1EPKc"]
            fn __ext(__this: *mut ACE_Suspend_Node, __a0: *const libc::c_char);
        }
        __ext(__this as *mut ACE_Suspend_Node, __a0)
    }
    pub unsafe fn new(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn apply(
        __this: *mut Self,
        mut cfg: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Suspend_Node5applyEP19ACE_Service_GestaltRi"]
            fn __ext(
                __this: *mut ACE_Suspend_Node,
                cfg: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Suspend_Node, cfg, yyerrno)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK16ACE_Suspend_Node4dumpEv"]
            fn __ext(__this: *const ACE_Suspend_Node);
        }
        __ext(__this as *const ACE_Suspend_Node)
    }
    pub unsafe fn new_at_u28ef93cdcc8e73b4(
        __this: *mut Self,
        mut __a0: *const ACE_Suspend_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Suspend_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Suspend_Node, __a0: *const ACE_Suspend_Node);
        }
        __ext(__this as *mut ACE_Suspend_Node, __a0)
    }
    pub unsafe fn new_u28ef93cdcc8e73b4(mut __a0: *const ACE_Suspend_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u28ef93cdcc8e73b4(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Suspend_Node,
    ) -> *mut ACE_Suspend_Node {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Suspend_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Suspend_Node,
                _anon_0: *const ACE_Suspend_Node,
            ) -> *mut ACE_Suspend_Node;
        }
        __ext(__this as *mut ACE_Suspend_Node, _anon_0)
    }
}
impl ACE_Resume_Node {
    pub unsafe fn new_at(__this: *mut Self, mut __a0: *const libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Resume_NodeC1EPKc"]
            fn __ext(__this: *mut ACE_Resume_Node, __a0: *const libc::c_char);
        }
        __ext(__this as *mut ACE_Resume_Node, __a0)
    }
    pub unsafe fn new(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn apply(
        __this: *mut Self,
        mut cfg: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Resume_Node5applyEP19ACE_Service_GestaltRi"]
            fn __ext(
                __this: *mut ACE_Resume_Node,
                cfg: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Resume_Node, cfg, yyerrno)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK15ACE_Resume_Node4dumpEv"]
            fn __ext(__this: *const ACE_Resume_Node);
        }
        __ext(__this as *const ACE_Resume_Node)
    }
    pub unsafe fn new_at_u15ed62b2a2f03dc6(
        __this: *mut Self,
        mut __a0: *const ACE_Resume_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Resume_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Resume_Node, __a0: *const ACE_Resume_Node);
        }
        __ext(__this as *mut ACE_Resume_Node, __a0)
    }
    pub unsafe fn new_u15ed62b2a2f03dc6(mut __a0: *const ACE_Resume_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u15ed62b2a2f03dc6(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Resume_Node,
    ) -> *mut ACE_Resume_Node {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Resume_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Resume_Node,
                _anon_0: *const ACE_Resume_Node,
            ) -> *mut ACE_Resume_Node;
        }
        __ext(__this as *mut ACE_Resume_Node, _anon_0)
    }
}
impl ACE_Remove_Node {
    pub unsafe fn new_at(__this: *mut Self, mut __a0: *const libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Remove_NodeC1EPKc"]
            fn __ext(__this: *mut ACE_Remove_Node, __a0: *const libc::c_char);
        }
        __ext(__this as *mut ACE_Remove_Node, __a0)
    }
    pub unsafe fn new(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn apply(
        __this: *mut Self,
        mut cfg: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Remove_Node5applyEP19ACE_Service_GestaltRi"]
            fn __ext(
                __this: *mut ACE_Remove_Node,
                cfg: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Remove_Node, cfg, yyerrno)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK15ACE_Remove_Node4dumpEv"]
            fn __ext(__this: *const ACE_Remove_Node);
        }
        __ext(__this as *const ACE_Remove_Node)
    }
    pub unsafe fn new_at_uc0e30d3c1a990d28(
        __this: *mut Self,
        mut __a0: *const ACE_Remove_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Remove_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Remove_Node, __a0: *const ACE_Remove_Node);
        }
        __ext(__this as *mut ACE_Remove_Node, __a0)
    }
    pub unsafe fn new_uc0e30d3c1a990d28(mut __a0: *const ACE_Remove_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_uc0e30d3c1a990d28(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Remove_Node,
    ) -> *mut ACE_Remove_Node {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Remove_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Remove_Node,
                _anon_0: *const ACE_Remove_Node,
            ) -> *mut ACE_Remove_Node;
        }
        __ext(__this as *mut ACE_Remove_Node, _anon_0)
    }
}
impl ACE_Static_Node {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *mut libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Static_NodeC1EPKcPc"]
            fn __ext(
                __this: *mut ACE_Static_Node,
                __a0: *const libc::c_char,
                __a1: *mut libc::c_char,
            );
        }
        __ext(__this as *mut ACE_Static_Node, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: *mut libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    pub unsafe fn apply(
        __this: *mut Self,
        mut cfg: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Static_Node5applyEP19ACE_Service_GestaltRi"]
            fn __ext(
                __this: *mut ACE_Static_Node,
                cfg: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Static_Node, cfg, yyerrno)
    }
    pub unsafe fn record(
        __this: *const Self,
        mut g: *const ACE_Service_Gestalt,
    ) -> *const ACE_Service_Type {
        extern "C-unwind" {
            #[link_name = "_ZNK15ACE_Static_Node6recordEPK19ACE_Service_Gestalt"]
            fn __ext(
                __this: *const ACE_Static_Node,
                g: *const ACE_Service_Gestalt,
            ) -> *const ACE_Service_Type;
        }
        __ext(__this as *const ACE_Static_Node, g)
    }
    pub unsafe fn parameters(__this: *const Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZNK15ACE_Static_Node10parametersEv"]
            fn __ext(__this: *const ACE_Static_Node) -> *mut libc::c_char;
        }
        __ext(__this as *const ACE_Static_Node)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK15ACE_Static_Node4dumpEv"]
            fn __ext(__this: *const ACE_Static_Node);
        }
        __ext(__this as *const ACE_Static_Node)
    }
    pub unsafe fn new_at_ua4c56ed6c9dfb8f0(
        __this: *mut Self,
        mut __a0: *const ACE_Static_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Static_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Static_Node, __a0: *const ACE_Static_Node);
        }
        __ext(__this as *mut ACE_Static_Node, __a0)
    }
    pub unsafe fn new_ua4c56ed6c9dfb8f0(mut __a0: *const ACE_Static_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ua4c56ed6c9dfb8f0(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Static_Node,
    ) -> *mut ACE_Static_Node {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Static_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Static_Node,
                _anon_0: *const ACE_Static_Node,
            ) -> *mut ACE_Static_Node;
        }
        __ext(__this as *mut ACE_Static_Node, _anon_0)
    }
}
impl ACE_Dynamic_Node {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const ACE_Service_Type_Factory,
        mut __a1: *mut libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Dynamic_NodeC1EPK24ACE_Service_Type_FactoryPc"]
            fn __ext(
                __this: *mut ACE_Dynamic_Node,
                __a0: *const ACE_Service_Type_Factory,
                __a1: *mut libc::c_char,
            );
        }
        __ext(__this as *mut ACE_Dynamic_Node, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const ACE_Service_Type_Factory,
        mut __a1: *mut libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    pub unsafe fn apply(
        __this: *mut Self,
        mut cfg: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Dynamic_Node5applyEP19ACE_Service_GestaltRi"]
            fn __ext(
                __this: *mut ACE_Dynamic_Node,
                cfg: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Dynamic_Node, cfg, yyerrno)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK16ACE_Dynamic_Node4dumpEv"]
            fn __ext(__this: *const ACE_Dynamic_Node);
        }
        __ext(__this as *const ACE_Dynamic_Node)
    }
    pub unsafe fn new_at_u845efb4a2f162f1a(
        __this: *mut Self,
        mut __a0: *const ACE_Dynamic_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Dynamic_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Dynamic_Node, __a0: *const ACE_Dynamic_Node);
        }
        __ext(__this as *mut ACE_Dynamic_Node, __a0)
    }
    pub unsafe fn new_u845efb4a2f162f1a(mut __a0: *const ACE_Dynamic_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u845efb4a2f162f1a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Dynamic_Node,
    ) -> *mut ACE_Dynamic_Node {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Dynamic_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Dynamic_Node,
                _anon_0: *const ACE_Dynamic_Node,
            ) -> *mut ACE_Dynamic_Node;
        }
        __ext(__this as *mut ACE_Dynamic_Node, _anon_0)
    }
}
impl ACE_Auto_Ptr_const_ACE_Service_Type_Factory_ {
    pub unsafe fn new_at_sfddb0cf77137aa44(
        __this: *mut Self,
        mut p: *const ACE_Service_Type_Factory,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_>::new_at_scab98f6cc0b80ce9(
                (::core::ptr::addr_of_mut!((* __this).__base_0)
                    .cast::<ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_>())
                    as *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
                ((p) as *const ACE_Service_Type_Factory),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sfddb0cf77137aa44(
        mut __a0: *const ACE_Service_Type_Factory,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sfddb0cf77137aa44(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_arrow(
        __this: *const Self,
    ) -> *const ACE_Service_Type_Factory {
        extern "C-unwind" {
            #[link_name = "_ZNK12ACE_Auto_PtrIK24ACE_Service_Type_FactoryEptEv"]
            fn __ext(
                __this: *const ACE_Auto_Ptr_const_ACE_Service_Type_Factory_,
            ) -> *const ACE_Service_Type_Factory;
        }
        __ext(__this as *const ACE_Auto_Ptr_const_ACE_Service_Type_Factory_)
    }
}
impl ACE_Stream_Node {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const ACE_Static_Node,
        mut __a1: *const ACE_Parse_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Stream_NodeC1EPK15ACE_Static_NodePK14ACE_Parse_Node"]
            fn __ext(
                __this: *mut ACE_Stream_Node,
                __a0: *const ACE_Static_Node,
                __a1: *const ACE_Parse_Node,
            );
        }
        __ext(__this as *mut ACE_Stream_Node, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const ACE_Static_Node,
        mut __a1: *const ACE_Parse_Node,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    pub unsafe fn apply(
        __this: *mut Self,
        mut cfg: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Stream_Node5applyEP19ACE_Service_GestaltRi"]
            fn __ext(
                __this: *mut ACE_Stream_Node,
                cfg: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Stream_Node, cfg, yyerrno)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK15ACE_Stream_Node4dumpEv"]
            fn __ext(__this: *const ACE_Stream_Node);
        }
        __ext(__this as *const ACE_Stream_Node)
    }
    pub unsafe fn new_at_uc626d61d2a1279b8(
        __this: *mut Self,
        mut __a0: *const ACE_Stream_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Stream_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Stream_Node, __a0: *const ACE_Stream_Node);
        }
        __ext(__this as *mut ACE_Stream_Node, __a0)
    }
    pub unsafe fn new_uc626d61d2a1279b8(mut __a0: *const ACE_Stream_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_uc626d61d2a1279b8(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Stream_Node,
    ) -> *mut ACE_Stream_Node {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Stream_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Stream_Node,
                _anon_0: *const ACE_Stream_Node,
            ) -> *mut ACE_Stream_Node;
        }
        __ext(__this as *mut ACE_Stream_Node, _anon_0)
    }
}
impl ACE_Dummy_Node {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const ACE_Static_Node,
        mut __a1: *const ACE_Parse_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Dummy_NodeC1EPK15ACE_Static_NodePK14ACE_Parse_Node"]
            fn __ext(
                __this: *mut ACE_Dummy_Node,
                __a0: *const ACE_Static_Node,
                __a1: *const ACE_Parse_Node,
            );
        }
        __ext(__this as *mut ACE_Dummy_Node, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const ACE_Static_Node,
        mut __a1: *const ACE_Parse_Node,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    pub unsafe fn apply(
        __this: *mut Self,
        mut cfg: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Dummy_Node5applyEP19ACE_Service_GestaltRi"]
            fn __ext(
                __this: *mut ACE_Dummy_Node,
                cfg: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Dummy_Node, cfg, yyerrno)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK14ACE_Dummy_Node4dumpEv"]
            fn __ext(__this: *const ACE_Dummy_Node);
        }
        __ext(__this as *const ACE_Dummy_Node)
    }
    pub unsafe fn new_at_ua6bccbc179dbe748(
        __this: *mut Self,
        mut __a0: *const ACE_Dummy_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Dummy_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Dummy_Node, __a0: *const ACE_Dummy_Node);
        }
        __ext(__this as *mut ACE_Dummy_Node, __a0)
    }
    pub unsafe fn new_ua6bccbc179dbe748(mut __a0: *const ACE_Dummy_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ua6bccbc179dbe748(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Dummy_Node,
    ) -> *mut ACE_Dummy_Node {
        extern "C-unwind" {
            #[link_name = "_ZN14ACE_Dummy_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Dummy_Node,
                _anon_0: *const ACE_Dummy_Node,
            ) -> *mut ACE_Dummy_Node;
        }
        __ext(__this as *mut ACE_Dummy_Node, _anon_0)
    }
}
impl ACE_Object_Node {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Object_NodeC1EPKcS1_"]
            fn __ext(
                __this: *mut ACE_Object_Node,
                __a0: *const libc::c_char,
                __a1: *const libc::c_char,
            );
        }
        __ext(__this as *mut ACE_Object_Node, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    pub unsafe fn symbol(
        __this: *mut Self,
        mut config: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
        mut _anon_2: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Object_Node6symbolEP19ACE_Service_GestaltRiPPFvPvE"]
            fn __ext(
                __this: *mut ACE_Object_Node,
                config: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
                _anon_2: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Object_Node, config, yyerrno, _anon_2)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK15ACE_Object_Node4dumpEv"]
            fn __ext(__this: *const ACE_Object_Node);
        }
        __ext(__this as *const ACE_Object_Node)
    }
    pub unsafe fn new_at_u4e2819571b24e836(
        __this: *mut Self,
        mut __a0: *const ACE_Object_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Object_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Object_Node, __a0: *const ACE_Object_Node);
        }
        __ext(__this as *mut ACE_Object_Node, __a0)
    }
    pub unsafe fn new_u4e2819571b24e836(mut __a0: *const ACE_Object_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u4e2819571b24e836(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Object_Node,
    ) -> *mut ACE_Object_Node {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Object_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Object_Node,
                _anon_0: *const ACE_Object_Node,
            ) -> *mut ACE_Object_Node;
        }
        __ext(__this as *mut ACE_Object_Node, _anon_0)
    }
}
impl ACE_Function_Node {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Function_NodeC1EPKcS1_"]
            fn __ext(
                __this: *mut ACE_Function_Node,
                __a0: *const libc::c_char,
                __a1: *const libc::c_char,
            );
        }
        __ext(__this as *mut ACE_Function_Node, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    pub unsafe fn symbol(
        __this: *mut Self,
        mut config: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
        mut gobbler: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Function_Node6symbolEP19ACE_Service_GestaltRiPPFvPvE"]
            fn __ext(
                __this: *mut ACE_Function_Node,
                config: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
                gobbler: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Function_Node, config, yyerrno, gobbler)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_Function_Node4dumpEv"]
            fn __ext(__this: *const ACE_Function_Node);
        }
        __ext(__this as *const ACE_Function_Node)
    }
    #[doc = "Return mangled function name that takes into account ACE\n  /// versioned namespace.\n  /**\n   * This function embeds the ACE versioned namespace name into the\n   * original function name if versioned namespace support has been\n   * enabled and the original function name conforms to the ACE\n   * Service Object factory function naming conventions.  For example\n   * \"@c _make_Foo\" becomes \"@c make_ACE_5_4_7_Foo\".\n   * @par\n   * If versioned namespace support is disabled or the factory\n   * function name does conform to ACE conventions, no mangling will\n   * occur and the verbatim function name is returned.\n   *\n   * @return Function name that takes into account versioned namespace\n   *         name.  Caller is responsible for calling operator\n   *         delete[] or ACE::strdelete() on the returned string."]
    pub unsafe fn make_func_name(
        __this: *mut Self,
        mut func_name: *const libc::c_char,
    ) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Function_Node14make_func_nameEPKc"]
            fn __ext(
                __this: *mut ACE_Function_Node,
                func_name: *const libc::c_char,
            ) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_Function_Node, func_name)
    }
    pub unsafe fn new_at_ua9713ae2c8cf2d08(
        __this: *mut Self,
        mut __a0: *const ACE_Function_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Function_NodeC1ERKS_"]
            fn __ext(__this: *mut ACE_Function_Node, __a0: *const ACE_Function_Node);
        }
        __ext(__this as *mut ACE_Function_Node, __a0)
    }
    pub unsafe fn new_ua9713ae2c8cf2d08(mut __a0: *const ACE_Function_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_ua9713ae2c8cf2d08(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Function_Node,
    ) -> *mut ACE_Function_Node {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_Function_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Function_Node,
                _anon_0: *const ACE_Function_Node,
            ) -> *mut ACE_Function_Node;
        }
        __ext(__this as *mut ACE_Function_Node, _anon_0)
    }
}
impl ACE_Static_Function_Node {
    pub unsafe fn new_at(__this: *mut Self, mut __a0: *const libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Static_Function_NodeC1EPKc"]
            fn __ext(__this: *mut ACE_Static_Function_Node, __a0: *const libc::c_char);
        }
        __ext(__this as *mut ACE_Static_Function_Node, __a0)
    }
    pub unsafe fn new(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn symbol(
        __this: *mut Self,
        mut config: *mut ACE_Service_Gestalt,
        mut yyerrno: *mut libc::c_int,
        mut _anon_2: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Static_Function_Node6symbolEP19ACE_Service_GestaltRiPPFvPvE"]
            fn __ext(
                __this: *mut ACE_Static_Function_Node,
                config: *mut ACE_Service_Gestalt,
                yyerrno: *mut libc::c_int,
                _anon_2: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Static_Function_Node, config, yyerrno, _anon_2)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK24ACE_Static_Function_Node4dumpEv"]
            fn __ext(__this: *const ACE_Static_Function_Node);
        }
        __ext(__this as *const ACE_Static_Function_Node)
    }
    pub unsafe fn new_at_u12dff81a55b43b02(
        __this: *mut Self,
        mut __a0: *const ACE_Static_Function_Node,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Static_Function_NodeC1ERKS_"]
            fn __ext(
                __this: *mut ACE_Static_Function_Node,
                __a0: *const ACE_Static_Function_Node,
            );
        }
        __ext(__this as *mut ACE_Static_Function_Node, __a0)
    }
    pub unsafe fn new_u12dff81a55b43b02(
        mut __a0: *const ACE_Static_Function_Node,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u12dff81a55b43b02(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut _anon_0: *const ACE_Static_Function_Node,
    ) -> *mut ACE_Static_Function_Node {
        extern "C-unwind" {
            #[link_name = "_ZN24ACE_Static_Function_NodeaSERKS_"]
            fn __ext(
                __this: *mut ACE_Static_Function_Node,
                _anon_0: *const ACE_Static_Function_Node,
            ) -> *mut ACE_Static_Function_Node;
        }
        __ext(__this as *mut ACE_Static_Function_Node, _anon_0)
    }
}
impl ACE_Auto_Ptr_ACE_Location_Node_ {
    pub unsafe fn new_at_s219cbb6102035cda(
        __this: *mut Self,
        mut p: *mut ACE_Location_Node,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Auto_Basic_Ptr_ACE_Location_Node_>::new_at_s87b22e51700151eb(
                (::core::ptr::addr_of_mut!((* __this).__base_0)
                    .cast::<ACE_Auto_Basic_Ptr_ACE_Location_Node_>())
                    as *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
                ((p) as *mut ACE_Location_Node),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s219cbb6102035cda(mut __a0: *mut ACE_Location_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s219cbb6102035cda(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_arrow(__this: *const Self) -> *mut ACE_Location_Node {
        extern "C-unwind" {
            #[link_name = "_ZNK12ACE_Auto_PtrI17ACE_Location_NodeEptEv"]
            fn __ext(
                __this: *const ACE_Auto_Ptr_ACE_Location_Node_,
            ) -> *mut ACE_Location_Node;
        }
        __ext(__this as *const ACE_Auto_Ptr_ACE_Location_Node_)
    }
}
impl ACE_Obstack_T_char_ {
    /**Request Obstack to prepare a block at least @a len long for building
  /// a new string.  Return -1 if fail, 0 if success.*/
    pub unsafe fn request(__this: *mut Self, mut len: libc::c_ulong) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcE7requestEm"]
            fn __ext(
                __this: *mut ACE_Obstack_T_char_,
                len: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Obstack_T_char_, len)
    }
    /**Inserting a new ACE_CHAR_T \a c into the current building block
  /// without freezing (null terminating) the block.  This function
  /// will create new chunk by checking the boundary of current
  /// Obchunk.  Return the location \a c gets inserted to, or 0 if
  /// error.*/
    pub unsafe fn grow(__this: *mut Self, mut c: libc::c_char) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcE4growEc"]
            fn __ext(
                __this: *mut ACE_Obstack_T_char_,
                c: libc::c_char,
            ) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_Obstack_T_char_, c)
    }
    /**Inserting a new ACE_CHAR_T \a c into the current building
  /// block without freezing (null terminating) the block and without
  /// checking for out-of-bound error.*/
    pub unsafe fn grow_fast(__this: *mut Self, mut c: libc::c_char) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcE9grow_fastEc"]
            fn __ext(__this: *mut ACE_Obstack_T_char_, c: libc::c_char);
        }
        __ext(__this as *mut ACE_Obstack_T_char_, c)
    }
    /**Freeze the current building block by null terminating it.
  /// Return the starting address of the current building block, 0
  /// if error occurs.*/
    pub unsafe fn freeze(__this: *mut Self) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcE6freezeEv"]
            fn __ext(__this: *mut ACE_Obstack_T_char_) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_Obstack_T_char_)
    }
    ///Return the maximum @a size
    pub unsafe fn size(__this: *const Self) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_Obstack_TIcE4sizeEv"]
            fn __ext(__this: *const ACE_Obstack_T_char_) -> libc::c_ulong;
        }
        __ext(__this as *const ACE_Obstack_T_char_)
    }
    /**"Unwind" the stack. If @a obj is a null pointer, everything allocated
  /// in the stack is released. Otherwise, @a obj must be an address of an
  /// object allocated in the stack. In this case, @a obj is released along
  /// with everything allocated in the Obstack since @a obj.*/
    pub unsafe fn unwind(__this: *mut Self, mut obj: *mut libc::c_void) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcE6unwindEPv"]
            fn __ext(__this: *mut ACE_Obstack_T_char_, obj: *mut libc::c_void);
        }
        __ext(__this as *mut ACE_Obstack_T_char_, obj)
    }
    /**"Release" the entire stack of Obchunks, putting it back on the free
  /// list.*/
    pub unsafe fn release(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcE7releaseEv"]
            fn __ext(__this: *mut ACE_Obstack_T_char_);
        }
        __ext(__this as *mut ACE_Obstack_T_char_)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK13ACE_Obstack_TIcE4dumpEv"]
            fn __ext(__this: *const ACE_Obstack_T_char_);
        }
        __ext(__this as *const ACE_Obstack_T_char_)
    }
    pub unsafe fn new_chunk(__this: *mut Self) -> *mut ACE_Obchunk {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcE9new_chunkEv"]
            fn __ext(__this: *mut ACE_Obstack_T_char_) -> *mut ACE_Obchunk;
        }
        __ext(__this as *mut ACE_Obstack_T_char_)
    }
    /**Search through the list of Obchunks and release them. Helper function
  /// used by unwind.*/
    pub unsafe fn unwind_i(__this: *mut Self, mut obj: *mut libc::c_void) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcE8unwind_iEPv"]
            fn __ext(__this: *mut ACE_Obstack_T_char_, obj: *mut libc::c_void);
        }
        __ext(__this as *mut ACE_Obstack_T_char_, obj)
    }
    pub unsafe fn new_at_se16342d999bb447e(
        __this: *mut Self,
        mut __a0: libc::c_ulong,
        mut __a1: *mut ACE_Allocator,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcEC1EmP13ACE_Allocator"]
            fn __ext(
                __this: *mut ACE_Obstack_T_char_,
                __a0: libc::c_ulong,
                __a1: *mut ACE_Allocator,
            );
        }
        __ext(__this as *mut ACE_Obstack_T_char_, __a0, __a1)
    }
    pub unsafe fn new_se16342d999bb447e(
        mut __a0: libc::c_ulong,
        mut __a1: *mut ACE_Allocator,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_se16342d999bb447e(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    /**Copy the data into the current Obchunk and freeze the current
  /// block.  Return the starting address of the current building
  /// block, 0 if error occurs.  @a len specify the string length,
  /// not the actually data size.*/
    pub unsafe fn copy(
        __this: *mut Self,
        mut data: *const libc::c_char,
        mut len: libc::c_ulong,
    ) -> *mut libc::c_char {
        extern "C-unwind" {
            #[link_name = "_ZN13ACE_Obstack_TIcE4copyEPKcm"]
            fn __ext(
                __this: *mut ACE_Obstack_T_char_,
                data: *const libc::c_char,
                len: libc::c_ulong,
            ) -> *mut libc::c_char;
        }
        __ext(__this as *mut ACE_Obstack_T_char_, data, len)
    }
}
impl ACE_Svc_Conf_Lexer {
    /**This is similar to the C function, ace_yylex, which a bison
  /// generated parser expects.  It returns information in the ace_yylval
  /// parameter and uses input stored in the param parameter.*/
    pub unsafe fn yylex(
        mut ace_yylval: *mut ACE_YYSTYPE,
        mut param: *mut ACE_Svc_Conf_Param,
    ) -> libc::c_int {
        unsafe {
            {
                if ((((((*param).buffer).is_null()) as libc::c_int) as libc::c_int) != 0)
                {
                    'dowhile_0: loop {
                        'cont_0: loop {
                            {
                                {
                                    (*param).buffer = {
                                        let __new: *mut ace_yy_buffer_state = Box::into_raw(
                                            Box::new(
                                                ::core::mem::MaybeUninit::<ace_yy_buffer_state>::zeroed()
                                                    .assume_init(),
                                            ),
                                        );
                                        <ace_yy_buffer_state>::new_at(
                                            (__new) as *mut ace_yy_buffer_state,
                                        );
                                        __new
                                    };
                                    if ((((((*param).buffer).is_null()) as libc::c_int)
                                        as libc::c_int) != 0)
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
                }
                let mut token: libc::c_int = (-((1) as libc::c_int));
                'dowhile_1: loop {
                    'cont_1: loop {
                        {
                            {
                                if (((*(*param).buffer).need_more_ as libc::c_int) != 0) {
                                    (*(*param).buffer).need_more_ = false;
                                    let mut amount: libc::c_ulong = <ACE_Svc_Conf_Lexer>::input(
                                        param,
                                        ((((*(*param).buffer).input_).as_ptr()
                                            as *mut libc::c_char))
                                            .wrapping_offset(((*(*param).buffer).size_) as isize),
                                        normalize(
                                            (((4096) as libc::c_ulong))
                                                .wrapping_sub(((*(*param).buffer).size_) as libc::c_ulong),
                                        ),
                                    );
                                    if (((((amount as libc::c_ulong))
                                        == (((0) as libc::c_ulong))) as libc::c_int as libc::c_int)
                                        != 0)
                                    {
                                        (*(*param).buffer).eof_ = true;
                                    } else {
                                        {
                                            let __lv = ::core::ptr::addr_of_mut!(
                                                (* (* param).buffer).size_
                                            );
                                            unsafe {
                                                *__lv = ((((*__lv)) as libc::c_ulong))
                                                    .wrapping_add((amount) as libc::c_ulong);
                                                *__lv
                                            }
                                        };
                                    }
                                }
                                token = <ACE_Svc_Conf_Lexer>::scan(ace_yylval, param);
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_1;
                    }
                    if !((((((((((token as libc::c_int))
                        == ((((-((1) as libc::c_int))) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)
                        && (((*(*param).buffer).need_more_ as libc::c_int) != 0))
                        as libc::c_int) as libc::c_int) != 0))
                    {
                        break 'dowhile_1;
                    }
                }
                return token;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn input(
        mut param: *mut ACE_Svc_Conf_Param,
        mut buf: *mut libc::c_char,
        mut max_size: libc::c_ulong,
    ) -> libc::c_ulong {
        unsafe {
            {
                let mut result: libc::c_ulong = ((0) as libc::c_ulong);
                'switch_0: {
                    let __c: libc::c_uint = (*param).r#type;
                    #[allow(unused_assignments)]
                    let mut __m: u32 = 2;
                    {
                        let __cv_0: libc::c_uint = (0 as libc::c_uint);
                        if __c == __cv_0 {
                            __m = 0;
                        }
                    }
                    if __m == 2 {
                        let __cv_1: libc::c_uint = (1 as libc::c_uint);
                        if __c == __cv_1 {
                            __m = 1;
                        }
                    }
                    if __m <= 0 {
                        ((*(__errno_location()))) = 0;
                        'while_0: loop {
                            if !((((((((((({
                                let __v = ACE_OS::fread_u7e9e336bcc2b5ed8(
                                    ((buf) as *mut libc::c_void),
                                    ((1) as libc::c_ulong),
                                    max_size,
                                    (*param).source.file,
                                );
                                result = __v;
                                __v
                            }) as libc::c_ulong)) == (((0) as libc::c_ulong)))
                                as libc::c_int as libc::c_int) != 0)
                                && ((ferror((*param).source.file)) != 0)) as libc::c_int)
                                as libc::c_int) != 0))
                            {
                                break 'while_0;
                            }
                            'cont_0: loop {
                                {
                                    {
                                        if (((((((*(__errno_location()))) as libc::c_int))
                                            == (((4) as libc::c_int))) as libc::c_int as libc::c_int)
                                            != 0)
                                        {
                                            ((*(__errno_location()))) = 0;
                                            ACE_OS::clearerr_ue05057310b33b5c9((*param).source.file);
                                        } else {
                                            ACE_OS::fprintf_u7a83295f0e5d324f(
                                                stderr,
                                                ((b"ERROR: input in scanner failed\n\0".as_ptr()
                                                    as *const libc::c_char) as *const libc::c_char),
                                            );
                                            ACE_OS::exit_ud318a3a23e137d2b(2);
                                        }
                                    }
                                }
                                #[allow(unreachable_code)] break 'cont_0;
                            }
                        }
                        break 'switch_0;
                    }
                    if __m <= 1 {
                        result = (((((ACE_OS::strlen_u07dd12a225364fa6(
                            ((((*param).source.directive)
                                .wrapping_offset(((*(*param).buffer).start_) as isize))
                                as *const libc::c_char),
                        )) as libc::c_ulong))
                            .wrapping_mul((1) as libc::c_ulong)) as libc::c_ulong);
                        if (((((result as libc::c_ulong)) != (((0) as libc::c_ulong)))
                            as libc::c_int as libc::c_int) != 0)
                        {
                            if (((((result as libc::c_ulong))
                                > (((max_size) as libc::c_ulong))) as libc::c_int
                                as libc::c_int) != 0)
                            {
                                result = max_size;
                            }
                            ACE_OS::memcpy_u6033eb81edaf9212(
                                ((buf) as *mut libc::c_void),
                                ((((*param).source.directive)
                                    .wrapping_offset(((*(*param).buffer).start_) as isize))
                                    as *const libc::c_void),
                                result,
                            );
                            {
                                let __lv = ::core::ptr::addr_of_mut!(
                                    (* (* param).buffer).start_
                                );
                                unsafe {
                                    *__lv = ((((*__lv)) as libc::c_ulong))
                                        .wrapping_add(
                                            (((((result) as libc::c_ulong)) / ((1) as libc::c_ulong)))
                                                as libc::c_ulong,
                                        );
                                    *__lv
                                }
                            };
                        }
                        break 'switch_0;
                    }
                    if __m <= 2 {
                        ace_yyerror_uef3b1714225fccac(
                            {
                                let __lv = &mut ((*param).yyerrno);
                                *__lv = (*__lv).wrapping_add(1);
                                *__lv
                            },
                            (*param).yylineno,
                            ((b"Invalid Service Configurator type in ACE_Svc_Conf_Lexer::input\0"
                                .as_ptr() as *const libc::c_char) as *const libc::c_char),
                        );
                    }
                    #[allow(unreachable_code)] break 'switch_0;
                }
                return result;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub unsafe fn scan(
        mut ace_yylval: *mut ACE_YYSTYPE,
        mut param: *mut ACE_Svc_Conf_Param,
    ) -> libc::c_int {
        unsafe {
            let mut __alloca_arena: ::std::vec::Vec<::std::boxed::Box<[u128]>> = ::std::vec::Vec::new();
            {
                let mut buffer: *mut ace_yy_buffer_state = (*param).buffer;
                if ((((((*buffer).state_ as libc::c_int))
                    == ((((-((1) as libc::c_int))) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    'while_0: loop {
                        if !(((((((((((*buffer).index_ as libc::c_ulong))
                            < ((((*buffer).size_) as libc::c_ulong))) as libc::c_int
                            as libc::c_int) != 0)
                            && ((isspace(
                                (((*((((*buffer).input_).as_ptr() as *mut libc::c_char)
                                    .wrapping_offset(((*buffer).index_) as isize))))
                                    as libc::c_int),
                            )) != 0)) as libc::c_int) as libc::c_int) != 0))
                        {
                            break 'while_0;
                        }
                        'cont_0: loop {
                            {
                                {
                                    if ((((((*((((*buffer).input_).as_ptr()
                                        as *mut libc::c_char)
                                        .wrapping_offset(((*buffer).index_) as isize)))
                                        as libc::c_int as libc::c_char))
                                        == (((10 as libc::c_char) as libc::c_int as libc::c_char)))
                                        as libc::c_int as libc::c_int) != 0)
                                    {
                                        {
                                            let __lv = &mut ((*param).yylineno);
                                            *__lv = (*__lv).wrapping_add(1);
                                            *__lv
                                        };
                                    }
                                    {
                                        let __lv = &mut ((*buffer).index_);
                                        *__lv = (*__lv).wrapping_add(1);
                                        *__lv
                                    };
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                    }
                }
                let mut current: libc::c_ulong = unsafe { ::core::mem::zeroed() };
                let mut last: libc::c_ulong = ((((*buffer).size_) as libc::c_ulong))
                    .wrapping_add(
                        ((if (((*buffer).eof_ as libc::c_int) != 0) { 1 } else { 0 }))
                            as libc::c_ulong,
                    );
                {
                    current = (*buffer).index_;
                    'for_1: loop {
                        if !((((((current as libc::c_ulong))
                            < (((last) as libc::c_ulong))) as libc::c_int as libc::c_int)
                            != 0))
                        {
                            break;
                        }
                        'cont_1: loop {
                            {
                                {
                                    static mut separators: *const libc::c_char = b" \t\r\n:*(){}\0"
                                        .as_ptr() as *const libc::c_char;
                                    let mut c: libc::c_char = (if (((((((*buffer).eof_
                                        as libc::c_int) != 0)
                                        && (((((current as libc::c_ulong))
                                            == ((((*buffer).size_) as libc::c_ulong))) as libc::c_int
                                            as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                                    {
                                        10 as libc::c_char
                                    } else {
                                        (*((((*buffer).input_).as_ptr() as *mut libc::c_char)
                                            .wrapping_offset((current) as isize)))
                                    });
                                    'switch_0: {
                                        let __c: libc::c_int = (*buffer).state_;
                                        #[allow(unused_assignments)]
                                        let mut __m: u32 = 3;
                                        {
                                            let __cv_0: libc::c_int = (0 as libc::c_int);
                                            if __c == __cv_0 {
                                                __m = 0;
                                            }
                                        }
                                        if __m == 3 {
                                            let __cv_1: libc::c_int = (271 as libc::c_int);
                                            if __c == __cv_1 {
                                                __m = 1;
                                            }
                                        }
                                        if __m == 3 {
                                            let __cv_2: libc::c_int = (-1 as libc::c_int);
                                            if __c == __cv_2 {
                                                __m = 2;
                                            }
                                        }
                                        if __m <= 0 {
                                            if (((((c as libc::c_int as libc::c_char))
                                                == (((10 as libc::c_char) as libc::c_int as libc::c_char)))
                                                as libc::c_int as libc::c_int) != 0)
                                            {
                                                (*buffer).state_ = (-((1) as libc::c_int));
                                                (*buffer).index_ = (((current) as libc::c_ulong))
                                                    .wrapping_add((1) as libc::c_ulong);
                                                {
                                                    let __lv = &mut ((*param).yylineno);
                                                    *__lv = (*__lv).wrapping_add(1);
                                                    *__lv
                                                };
                                            }
                                            break 'switch_0;
                                        }
                                        if __m <= 1 {
                                            if (((!(((((((((((c as libc::c_int as libc::c_char))
                                                >= (((32 as libc::c_char) as libc::c_int as libc::c_char)))
                                                as libc::c_int as libc::c_int) != 0)
                                                && (((((c as libc::c_int as libc::c_char))
                                                    <= (((126 as libc::c_char) as libc::c_int as libc::c_char)))
                                                    as libc::c_int as libc::c_int) != 0)) as libc::c_int))
                                                as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                                            {
                                                {
                                                    let __lv = &mut (current);
                                                    *__lv = (*__lv).wrapping_sub(1);
                                                    *__lv
                                                };
                                                let mut source: *mut libc::c_char = (((((*buffer).input_)
                                                    .as_ptr() as *mut libc::c_char))
                                                    .wrapping_offset(((*buffer).index_) as isize))
                                                    .wrapping_offset((1) as isize);
                                                let mut string_end_found: bool = false;
                                                if (((((current as libc::c_ulong))
                                                    > ((((*buffer).index_) as libc::c_ulong))) as libc::c_int
                                                    as libc::c_int) != 0)
                                                {
                                                    {
                                                        let mut i: libc::c_ulong = (((current) as libc::c_ulong))
                                                            .wrapping_sub(((*buffer).index_) as libc::c_ulong);
                                                        'for_2: loop {
                                                            if !(((((({
                                                                let __lv = &mut (i);
                                                                let __r = *__lv;
                                                                *__lv = (*__lv).wrapping_sub(1);
                                                                __r
                                                            } as libc::c_ulong)) != (((0) as libc::c_ulong)))
                                                                as libc::c_int as libc::c_int) != 0))
                                                            {
                                                                break;
                                                            }
                                                            'cont_2: loop {
                                                                {
                                                                    {
                                                                        if ((((((*(source).wrapping_offset((i) as isize))
                                                                            as libc::c_int as libc::c_char))
                                                                            == ((((*buffer).string_start_) as libc::c_int
                                                                                as libc::c_char))) as libc::c_int as libc::c_int) != 0)
                                                                        {
                                                                            current = (((((((*buffer).index_) as libc::c_ulong))
                                                                                .wrapping_add((i) as libc::c_ulong)) as libc::c_ulong))
                                                                                .wrapping_add((1) as libc::c_ulong);
                                                                            string_end_found = true;
                                                                            break 'for_2;
                                                                        }
                                                                    }
                                                                }
                                                                #[allow(unreachable_code)] break 'cont_2;
                                                            }
                                                        }
                                                    }
                                                }
                                                if (((!(((string_end_found as libc::c_int) != 0))
                                                    as libc::c_int) as libc::c_int) != 0)
                                                {
                                                    ace_yyerror_uef3b1714225fccac(
                                                        {
                                                            let __lv = &mut ((*param).yyerrno);
                                                            *__lv = (*__lv).wrapping_add(1);
                                                            *__lv
                                                        },
                                                        (*param).yylineno,
                                                        ((b"Unable to find the end of the string\0".as_ptr()
                                                            as *const libc::c_char) as *const libc::c_char),
                                                    );
                                                    return (-((1) as libc::c_int));
                                                }
                                                let mut amount: libc::c_ulong = (((((((current)
                                                    as libc::c_ulong))
                                                    .wrapping_sub(((*buffer).index_) as libc::c_ulong)))
                                                    as libc::c_ulong))
                                                    .wrapping_sub((1) as libc::c_ulong);
                                                let mut target: *mut libc::c_char = source;
                                                (*ace_yylval).ident_ = ((<ACE_Obstack_T_char_>::copy(
                                                    (::core::ptr::addr_of_mut!(
                                                        (* ::core::ptr::addr_of!((* param).obstack) .cast:: <
                                                        ACE_Obstack_T_char_ > ().cast_mut())
                                                    )) as *mut ACE_Obstack_T_char_,
                                                    ((target) as *const libc::c_char),
                                                    amount,
                                                )) as *mut libc::c_char);
                                                (*buffer).state_ = (-((1) as libc::c_int));
                                                (*buffer).index_ = (((current) as libc::c_ulong))
                                                    .wrapping_add((1) as libc::c_ulong);
                                                return ((ACE_STRING) as libc::c_int);
                                            }
                                            break 'switch_0;
                                        }
                                        if __m <= 2 {
                                            if (((((((((c as libc::c_int as libc::c_char))
                                                == (((34 as libc::c_char) as libc::c_int as libc::c_char)))
                                                as libc::c_int as libc::c_int) != 0)
                                                || (((((c as libc::c_int as libc::c_char))
                                                    == (((39 as libc::c_char) as libc::c_int as libc::c_char)))
                                                    as libc::c_int as libc::c_int) != 0)) as libc::c_int)
                                                as libc::c_int) != 0)
                                            {
                                                (*buffer).string_start_ = c;
                                                (*buffer).state_ = ((ACE_STRING) as libc::c_int);
                                            } else {
                                                if (((((c as libc::c_int as libc::c_char))
                                                    == (((35 as libc::c_char) as libc::c_int as libc::c_char)))
                                                    as libc::c_int as libc::c_int) != 0)
                                                {
                                                    (*buffer).state_ = 0;
                                                } else {
                                                    if ((((!(ACE_OS::strchr_ue2d436a738f8836a(
                                                        separators,
                                                        ((c) as libc::c_int),
                                                    ))
                                                        .is_null()) as libc::c_int) as libc::c_int) != 0)
                                                    {
                                                        if (((((c as libc::c_int as libc::c_char))
                                                            == (((10 as libc::c_char) as libc::c_int as libc::c_char)))
                                                            as libc::c_int as libc::c_int) != 0)
                                                        {
                                                            {
                                                                let __lv = &mut ((*param).yylineno);
                                                                *__lv = (*__lv).wrapping_add(1);
                                                                *__lv
                                                            };
                                                        }
                                                        if (((((current as libc::c_ulong))
                                                            == (((((((*buffer).index_) as libc::c_ulong))
                                                                .wrapping_add((1) as libc::c_ulong)) as libc::c_ulong)))
                                                            as libc::c_int as libc::c_int) != 0)
                                                        {
                                                            let mut lower: libc::c_int = ((ACE_OS::ace_tolower(
                                                                (((*((((*buffer).input_).as_ptr() as *mut libc::c_char)
                                                                    .wrapping_offset(
                                                                        ((((current) as libc::c_ulong))
                                                                            .wrapping_sub((1) as libc::c_ulong)) as isize,
                                                                    )))) as libc::c_int),
                                                            )) as libc::c_int);
                                                            if (((((((((c as libc::c_int as libc::c_char))
                                                                == (((58 as libc::c_char) as libc::c_int as libc::c_char)))
                                                                as libc::c_int as libc::c_int) != 0)
                                                                && (((((((((((*((((*buffer).input_).as_ptr()
                                                                    as *mut libc::c_char)
                                                                    .wrapping_offset(
                                                                        ((((current) as libc::c_ulong))
                                                                            .wrapping_sub((1) as libc::c_ulong)) as isize,
                                                                    ))) as libc::c_int as libc::c_char))
                                                                    == (((37 as libc::c_char) as libc::c_int as libc::c_char)))
                                                                    as libc::c_int as libc::c_int) != 0)
                                                                    || ((((((((((lower as libc::c_int))
                                                                        >= (((97 as libc::c_char) as libc::c_int))) as libc::c_int
                                                                        as libc::c_int) != 0)
                                                                        && (((((lower as libc::c_int))
                                                                            <= (((122 as libc::c_char) as libc::c_int))) as libc::c_int
                                                                            as libc::c_int) != 0)) as libc::c_int)) as libc::c_int)
                                                                        != 0)) as libc::c_int)) as libc::c_int) != 0))
                                                                as libc::c_int) as libc::c_int) != 0)
                                                            {
                                                                break 'switch_0;
                                                            }
                                                        }
                                                        if (((((current as libc::c_ulong))
                                                            == ((((*buffer).index_) as libc::c_ulong))) as libc::c_int
                                                            as libc::c_int) != 0)
                                                        {
                                                            (*buffer).index_ = (((current) as libc::c_ulong))
                                                                .wrapping_add((1) as libc::c_ulong);
                                                            if ((isspace(((c) as libc::c_int))) != 0) {
                                                                break 'switch_0;
                                                            } else {
                                                                return ((c) as libc::c_int);
                                                            }
                                                        }
                                                        let mut size: libc::c_ulong = (((((((current)
                                                            as libc::c_ulong))
                                                            .wrapping_sub(((*buffer).index_) as libc::c_ulong)))
                                                            as libc::c_ulong))
                                                            .wrapping_add((1) as libc::c_ulong);
                                                        let str: *mut libc::c_char = {
                                                            let __n: usize = (size) as usize;
                                                            let __bytes: usize = __n
                                                                .saturating_mul(::core::mem::size_of::<libc::c_char>());
                                                            let __words: usize = (__bytes.saturating_add(15) / 16)
                                                                .max(1);
                                                            __alloca_arena
                                                                .push(::std::vec![0u128; __words].into_boxed_slice());
                                                            __alloca_arena.last_mut().unwrap().as_mut_ptr()
                                                                as *mut libc::c_char
                                                        };
                                                        ACE_OS::strncpy_u11a5be0fa5efbef0(
                                                            ((str) as *mut libc::c_char),
                                                            ((((((*buffer).input_).as_ptr() as *mut libc::c_char))
                                                                .wrapping_offset(((*buffer).index_) as isize))
                                                                as *const libc::c_char),
                                                            (((size) as libc::c_ulong))
                                                                .wrapping_sub((1) as libc::c_ulong),
                                                        );
                                                        (*(str)
                                                            .wrapping_offset(
                                                                ((((size) as libc::c_ulong))
                                                                    .wrapping_sub((1) as libc::c_ulong)) as isize,
                                                            )) = 0 as libc::c_char;
                                                        if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                            ((str) as *const libc::c_char),
                                                            ((b"dynamic\0".as_ptr() as *const libc::c_char)
                                                                as *const libc::c_char),
                                                        ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                            as libc::c_int) != 0)
                                                        {
                                                            (*buffer).index_ = current;
                                                            return ((ACE_DYNAMIC) as libc::c_int);
                                                        } else {
                                                            if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                ((str) as *const libc::c_char),
                                                                ((b"static\0".as_ptr() as *const libc::c_char)
                                                                    as *const libc::c_char),
                                                            ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                as libc::c_int) != 0)
                                                            {
                                                                (*buffer).index_ = current;
                                                                return ((ACE_STATIC) as libc::c_int);
                                                            } else {
                                                                if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                    ((str) as *const libc::c_char),
                                                                    ((b"suspend\0".as_ptr() as *const libc::c_char)
                                                                        as *const libc::c_char),
                                                                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                    as libc::c_int) != 0)
                                                                {
                                                                    (*buffer).index_ = current;
                                                                    return ((ACE_SUSPEND) as libc::c_int);
                                                                } else {
                                                                    if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                        ((str) as *const libc::c_char),
                                                                        ((b"resume\0".as_ptr() as *const libc::c_char)
                                                                            as *const libc::c_char),
                                                                    ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                        as libc::c_int) != 0)
                                                                    {
                                                                        (*buffer).index_ = current;
                                                                        return ((ACE_RESUME) as libc::c_int);
                                                                    } else {
                                                                        if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                            ((str) as *const libc::c_char),
                                                                            ((b"remove\0".as_ptr() as *const libc::c_char)
                                                                                as *const libc::c_char),
                                                                        ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                            as libc::c_int) != 0)
                                                                        {
                                                                            (*buffer).index_ = current;
                                                                            return ((ACE_REMOVE) as libc::c_int);
                                                                        } else {
                                                                            if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                                ((str) as *const libc::c_char),
                                                                                ((b"stream\0".as_ptr() as *const libc::c_char)
                                                                                    as *const libc::c_char),
                                                                            ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                                as libc::c_int) != 0)
                                                                            {
                                                                                (*buffer).index_ = current;
                                                                                return ((ACE_USTREAM) as libc::c_int);
                                                                            } else {
                                                                                if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                                    ((str) as *const libc::c_char),
                                                                                    ((b"Module\0".as_ptr() as *const libc::c_char)
                                                                                        as *const libc::c_char),
                                                                                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                                    as libc::c_int) != 0)
                                                                                {
                                                                                    (*buffer).index_ = current;
                                                                                    return ((ACE_MODULE_T) as libc::c_int);
                                                                                } else {
                                                                                    if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                                        ((str) as *const libc::c_char),
                                                                                        ((b"Service_Object\0".as_ptr() as *const libc::c_char)
                                                                                            as *const libc::c_char),
                                                                                    ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                                        as libc::c_int) != 0)
                                                                                    {
                                                                                        (*buffer).index_ = current;
                                                                                        return ((ACE_SVC_OBJ_T) as libc::c_int);
                                                                                    } else {
                                                                                        if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                                            ((str) as *const libc::c_char),
                                                                                            ((b"STREAM\0".as_ptr() as *const libc::c_char)
                                                                                                as *const libc::c_char),
                                                                                        ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                                            as libc::c_int) != 0)
                                                                                        {
                                                                                            (*buffer).index_ = current;
                                                                                            return ((ACE_STREAM_T) as libc::c_int);
                                                                                        } else {
                                                                                            if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                                                ((str) as *const libc::c_char),
                                                                                                ((b"active\0".as_ptr() as *const libc::c_char)
                                                                                                    as *const libc::c_char),
                                                                                            ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                                                as libc::c_int) != 0)
                                                                                            {
                                                                                                (*buffer).index_ = current;
                                                                                                return ((ACE_ACTIVE) as libc::c_int);
                                                                                            } else {
                                                                                                if (((((ACE_OS::strcmp_u2f671283fc8b6d4a(
                                                                                                    ((str) as *const libc::c_char),
                                                                                                    ((b"inactive\0".as_ptr() as *const libc::c_char)
                                                                                                        as *const libc::c_char),
                                                                                                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                                                                                    as libc::c_int) != 0)
                                                                                                {
                                                                                                    (*buffer).index_ = current;
                                                                                                    return ((ACE_INACTIVE) as libc::c_int);
                                                                                                } else {
                                                                                                    let mut token: libc::c_int = ((ACE_IDENT) as libc::c_int);
                                                                                                    let mut amount: libc::c_ulong = (((size) as libc::c_ulong))
                                                                                                        .wrapping_sub((1) as libc::c_ulong);
                                                                                                    let mut target: *mut libc::c_char = ((str)
                                                                                                        as *mut libc::c_char);
                                                                                                    (*ace_yylval).ident_ = ((<ACE_Obstack_T_char_>::copy(
                                                                                                        (::core::ptr::addr_of_mut!(
                                                                                                            (* ::core::ptr::addr_of!((* param).obstack) .cast:: <
                                                                                                            ACE_Obstack_T_char_ > ().cast_mut())
                                                                                                        )) as *mut ACE_Obstack_T_char_,
                                                                                                        ((target) as *const libc::c_char),
                                                                                                        amount,
                                                                                                    )) as *mut libc::c_char);
                                                                                                    if ((ACE_OS::ace_isdigit(
                                                                                                        (*((*ace_yylval).ident_).wrapping_offset((0) as isize)),
                                                                                                    )) != 0)
                                                                                                    {
                                                                                                        token = ((ACE_PATHNAME) as libc::c_int);
                                                                                                    } else {
                                                                                                        static mut path_parts: *const libc::c_char = b"/\\:%.~-\0"
                                                                                                            .as_ptr() as *const libc::c_char;
                                                                                                        {
                                                                                                            let mut p: *const libc::c_char = path_parts;
                                                                                                            'for_3: loop {
                                                                                                                if !(((((((*(p)) as libc::c_int as libc::c_char))
                                                                                                                    != (((0 as libc::c_char) as libc::c_int as libc::c_char)))
                                                                                                                    as libc::c_int as libc::c_int) != 0))
                                                                                                                {
                                                                                                                    break;
                                                                                                                }
                                                                                                                'cont_3: loop {
                                                                                                                    {
                                                                                                                        {
                                                                                                                            if ((((!(ACE_OS::strchr_u824406bee5e3796b(
                                                                                                                                (((*ace_yylval).ident_) as *mut libc::c_char),
                                                                                                                                (((*(p))) as libc::c_int),
                                                                                                                            ))
                                                                                                                                .is_null()) as libc::c_int) as libc::c_int) != 0)
                                                                                                                            {
                                                                                                                                token = ((ACE_PATHNAME) as libc::c_int);
                                                                                                                                break 'for_3;
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                    #[allow(unreachable_code)] break 'cont_3;
                                                                                                                }
                                                                                                                {
                                                                                                                    let __lv = &mut (p);
                                                                                                                    let __r = *__lv;
                                                                                                                    *__lv = (*__lv).wrapping_offset(1);
                                                                                                                    __r
                                                                                                                };
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    (*buffer).state_ = (-((1) as libc::c_int));
                                                                                                    (*buffer).index_ = current;
                                                                                                    return token;
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            break 'switch_0;
                                        }
                                        if __m <= 3 {
                                            ace_yyerror_uef3b1714225fccac(
                                                {
                                                    let __lv = &mut ((*param).yyerrno);
                                                    *__lv = (*__lv).wrapping_add(1);
                                                    *__lv
                                                },
                                                (*param).yylineno,
                                                ((b"Unexpected state in ACE_Svc_Conf_Lexer::scan\0".as_ptr()
                                                    as *const libc::c_char) as *const libc::c_char),
                                            );
                                            return (-((1) as libc::c_int));
                                        }
                                        #[allow(unreachable_code)] break 'switch_0;
                                    }
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_1;
                        }
                        {
                            let __lv = &mut (current);
                            let __r = *__lv;
                            *__lv = (*__lv).wrapping_add(1);
                            __r
                        };
                    }
                }
                if (((!((((*buffer).eof_ as libc::c_int) != 0)) as libc::c_int)
                    as libc::c_int) != 0)
                {
                    (*buffer).need_more_ = true;
                    if ((((((*buffer).state_ as libc::c_int)) == (((0) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        (*buffer).index_ = ((0) as libc::c_ulong);
                        (*buffer).size_ = ((0) as libc::c_ulong);
                    } else {
                        (*buffer).size_ = (((current) as libc::c_ulong))
                            .wrapping_sub(((*buffer).index_) as libc::c_ulong);
                        if ((((((((((*buffer).size_ as libc::c_ulong))
                            != (((0) as libc::c_ulong))) as libc::c_int as libc::c_int)
                            != 0)
                            && ((((((*buffer).index_ as libc::c_ulong))
                                != (((0) as libc::c_ulong))) as libc::c_int as libc::c_int)
                                != 0)) as libc::c_int) as libc::c_int) != 0)
                        {
                            ACE_OS::memmove_u5dc7ae11e120ed2b(
                                (((*buffer).input_).as_mut_ptr() as *mut libc::c_void),
                                ((((((*buffer).input_).as_ptr() as *mut libc::c_char))
                                    .wrapping_offset(((*buffer).index_) as isize))
                                    as *const libc::c_void),
                                (*buffer).size_,
                            );
                        }
                        (*buffer).index_ = ((0) as libc::c_ulong);
                        (*buffer).state_ = (-((1) as libc::c_int));
                    }
                }
                return (-((1) as libc::c_int));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Auto_Basic_Ptr_ACE_Location_Node_ {
    pub unsafe fn new_at_sb3028e88a0eb16a6(
        __this: *mut Self,
        mut __a0: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Auto_Basic_PtrI17ACE_Location_NodeEC1ERS1_"]
            fn __ext(
                __this: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
                __a0: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
            );
        }
        __ext(__this as *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_, __a0)
    }
    pub unsafe fn new_sb3028e88a0eb16a6(
        mut __a0: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sb3028e88a0eb16a6(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
    ) -> *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_ {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Auto_Basic_PtrI17ACE_Location_NodeEaSERS1_"]
            fn __ext(
                __this: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
                rhs: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
            ) -> *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_;
        }
        __ext(__this as *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_, rhs)
    }
    pub unsafe fn operator_mul(__this: *const Self) -> *mut ACE_Location_Node {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Auto_Basic_PtrI17ACE_Location_NodeEdeEv"]
            fn __ext(
                __this: *const ACE_Auto_Basic_Ptr_ACE_Location_Node_,
            ) -> *mut ACE_Location_Node;
        }
        __ext(__this as *const ACE_Auto_Basic_Ptr_ACE_Location_Node_)
    }
    pub unsafe fn get(__this: *const Self) -> *mut ACE_Location_Node {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Auto_Basic_PtrI17ACE_Location_NodeE3getEv"]
            fn __ext(
                __this: *const ACE_Auto_Basic_Ptr_ACE_Location_Node_,
            ) -> *mut ACE_Location_Node;
        }
        __ext(__this as *const ACE_Auto_Basic_Ptr_ACE_Location_Node_)
    }
    pub unsafe fn release(__this: *mut Self) -> *mut ACE_Location_Node {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Auto_Basic_PtrI17ACE_Location_NodeE7releaseEv"]
            fn __ext(
                __this: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
            ) -> *mut ACE_Location_Node;
        }
        __ext(__this as *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_)
    }
    pub unsafe fn reset(__this: *mut Self, mut p: *mut ACE_Location_Node) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Auto_Basic_PtrI17ACE_Location_NodeE5resetEPS0_"]
            fn __ext(
                __this: *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_,
                p: *mut ACE_Location_Node,
            );
        }
        __ext(__this as *mut ACE_Auto_Basic_Ptr_ACE_Location_Node_, p)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Auto_Basic_PtrI17ACE_Location_NodeE4dumpEv"]
            fn __ext(__this: *const ACE_Auto_Basic_Ptr_ACE_Location_Node_);
        }
        __ext(__this as *const ACE_Auto_Basic_Ptr_ACE_Location_Node_)
    }
    pub unsafe fn new_at_s87b22e51700151eb(
        __this: *mut Self,
        mut p: *mut ACE_Location_Node,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).p_), p);
            {}
            ()
        }
    }
    pub unsafe fn new_s87b22e51700151eb(mut __a0: *mut ACE_Location_Node) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s87b22e51700151eb(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_ {
    pub unsafe fn new_at_sa570c90e930cc2c2(
        __this: *mut Self,
        mut __a0: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Auto_Basic_PtrIK24ACE_Service_Type_FactoryEC1ERS2_"]
            fn __ext(
                __this: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
                __a0: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
            );
        }
        __ext(__this as *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_, __a0)
    }
    pub unsafe fn new_sa570c90e930cc2c2(
        mut __a0: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sa570c90e930cc2c2(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    pub unsafe fn operator_assign(
        __this: *mut Self,
        mut rhs: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
    ) -> *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_ {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Auto_Basic_PtrIK24ACE_Service_Type_FactoryEaSERS2_"]
            fn __ext(
                __this: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
                rhs: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
            ) -> *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_;
        }
        __ext(__this as *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_, rhs)
    }
    pub unsafe fn operator_mul(__this: *const Self) -> *const ACE_Service_Type_Factory {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Auto_Basic_PtrIK24ACE_Service_Type_FactoryEdeEv"]
            fn __ext(
                __this: *const ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
            ) -> *const ACE_Service_Type_Factory;
        }
        __ext(__this as *const ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_)
    }
    pub unsafe fn get(__this: *const Self) -> *const ACE_Service_Type_Factory {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Auto_Basic_PtrIK24ACE_Service_Type_FactoryE3getEv"]
            fn __ext(
                __this: *const ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
            ) -> *const ACE_Service_Type_Factory;
        }
        __ext(__this as *const ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_)
    }
    pub unsafe fn release(__this: *mut Self) -> *const ACE_Service_Type_Factory {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Auto_Basic_PtrIK24ACE_Service_Type_FactoryE7releaseEv"]
            fn __ext(
                __this: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
            ) -> *const ACE_Service_Type_Factory;
        }
        __ext(__this as *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_)
    }
    pub unsafe fn reset(__this: *mut Self, mut p: *const ACE_Service_Type_Factory) {
        extern "C-unwind" {
            #[link_name = "_ZN18ACE_Auto_Basic_PtrIK24ACE_Service_Type_FactoryE5resetEPS1_"]
            fn __ext(
                __this: *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_,
                p: *const ACE_Service_Type_Factory,
            );
        }
        __ext(__this as *mut ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_, p)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK18ACE_Auto_Basic_PtrIK24ACE_Service_Type_FactoryE4dumpEv"]
            fn __ext(__this: *const ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_);
        }
        __ext(__this as *const ACE_Auto_Basic_Ptr_const_ACE_Service_Type_Factory_)
    }
    pub unsafe fn new_at_scab98f6cc0b80ce9(
        __this: *mut Self,
        mut p: *const ACE_Service_Type_Factory,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).p_), p);
            {}
            ()
        }
    }
    pub unsafe fn new_scab98f6cc0b80ce9(
        mut __a0: *const ACE_Service_Type_Factory,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_scab98f6cc0b80ce9(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_Unbounded_Set_Ex_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____ {
    #[doc = "Constructor.  Use user specified allocation strategy\n  /// if specified.\n  /**\n   * Initialize an empty set using the allocation strategy of the user if\n   * provided."]
    pub unsafe fn new_at_s0ae916ca18eb9b4a(
        __this: *mut Self,
        mut alloc: *mut ACE_Allocator,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).head_),
                ((0)
                    as *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).cur_size_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).allocator_), alloc);
            {
                if ((((((*__this).allocator_).is_null()) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    (*__this).allocator_ = <ACE_Allocator>::instance();
                }
                'dowhile_0: loop {
                    'cont_0: loop {
                        {
                            {
                                (*__this).head_ = ({
                                    let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                                        as *mut ACE_Allocator;
                                    let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                                        as *const *const __Vtbl_uf2113694993e252c);
                                    ((*__vt)
                                        .vfn_u685215409e23bf32)(__obj, ((16) as libc::c_ulong))
                                }
                                    as *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____);
                                if ((((((*__this).head_).is_null()) as libc::c_int)
                                    as libc::c_int) != 0)
                                {
                                    ((*(__errno_location()))) = 12;
                                    return;
                                } else {
                                    {
                                        let _ = {
                                            let __place: *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____ = ((*__this)
                                                .head_)
                                                as *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____;
                                            <ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____>::new_at_sf07148c9327d6265(
                                                (__place)
                                                    as *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
                                                ((0)
                                                    as *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____),
                                                0,
                                            );
                                            __place
                                        };
                                    };
                                }
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                    if !(((0) != 0)) {
                        break 'dowhile_0;
                    }
                }
                (*(*__this).head_).next_ = (((*__this).head_)
                    as *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____);
            }
            ()
        }
    }
    pub unsafe fn new_s0ae916ca18eb9b4a(mut __a0: *mut ACE_Allocator) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s0ae916ca18eb9b4a(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____ {
    pub unsafe fn new_at_sf07148c9327d6265(
        __this: *mut Self,
        mut n: *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
        mut _anon_1: libc::c_int,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).next_), n);
            {}
            ()
        }
    }
    pub unsafe fn new_sf07148c9327d6265(
        mut __a0: *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
        mut __a1: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sf07148c9327d6265(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
}
impl ACE_Unbounded_Set_Ex_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____ {
    #[doc = "Constructor.  Use user specified allocation strategy\n  /// if specified.\n  /**\n   * Initialize an empty set using the allocation strategy of the user if\n   * provided."]
    pub unsafe fn new_at_s444fe85a0e6a1680(
        __this: *mut Self,
        mut alloc: *mut ACE_Allocator,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).head_),
                ((0)
                    as *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).cur_size_),
                ((0) as libc::c_ulong),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).allocator_), alloc);
            {
                if ((((((*__this).allocator_).is_null()) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    (*__this).allocator_ = <ACE_Allocator>::instance();
                }
                'dowhile_0: loop {
                    'cont_0: loop {
                        {
                            {
                                (*__this).head_ = ({
                                    let __obj: *mut ACE_Allocator = ((*__this).allocator_)
                                        as *mut ACE_Allocator;
                                    let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                                        as *const *const __Vtbl_uf2113694993e252c);
                                    ((*__vt)
                                        .vfn_u685215409e23bf32)(__obj, ((16) as libc::c_ulong))
                                }
                                    as *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____);
                                if ((((((*__this).head_).is_null()) as libc::c_int)
                                    as libc::c_int) != 0)
                                {
                                    ((*(__errno_location()))) = 12;
                                    return;
                                } else {
                                    {
                                        let _ = {
                                            let __place: *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____ = ((*__this)
                                                .head_)
                                                as *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____;
                                            <ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____>::new_at_s7aae2d8bb68fae03(
                                                (__place)
                                                    as *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
                                                ((0)
                                                    as *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____),
                                                0,
                                            );
                                            __place
                                        };
                                    };
                                }
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                    if !(((0) != 0)) {
                        break 'dowhile_0;
                    }
                }
                (*(*__this).head_).next_ = (((*__this).head_)
                    as *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____);
            }
            ()
        }
    }
    pub unsafe fn new_s444fe85a0e6a1680(mut __a0: *mut ACE_Allocator) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s444fe85a0e6a1680(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____ {
    pub unsafe fn new_at_s7aae2d8bb68fae03(
        __this: *mut Self,
        mut n: *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
        mut _anon_1: libc::c_int,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).next_), n);
            {}
            ()
        }
    }
    pub unsafe fn new_s7aae2d8bb68fae03(
        mut __a0: *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
        mut __a1: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s7aae2d8bb68fae03(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
}
impl ACE_Unbounded_Set_Ex_Iterator_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____ {
    pub unsafe fn new_at_s12410a96477c7a42(
        __this: *mut Self,
        mut s: *mut ACE_Unbounded_Set_Ex_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
        mut end: bool,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).current_),
                if (((!(((end as libc::c_int) != 0)) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    (*(*s).head_).next_
                } else {
                    (((*s).head_)
                        as *mut ACE_Node_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____)
                },
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).set_),
                ::core::ptr::addr_of_mut!((* s))
                    as *mut ACE_Unbounded_Set_Ex_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s12410a96477c7a42(
        mut __a0: *mut ACE_Unbounded_Set_Ex_ACE_Service_Gestalt__Processed_Static_Svc____ACE_Unbounded_Set_Default_Comparator_ACE_Service_Gestalt__Processed_Static_Svc____,
        mut __a1: bool,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s12410a96477c7a42(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
}
impl ACE_Unbounded_Set_Ex_Iterator_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____ {
    pub unsafe fn new_at_s3c3b9f37e89103ee(
        __this: *mut Self,
        mut s: *mut ACE_Unbounded_Set_Ex_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
        mut end: bool,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).current_),
                if (((!(((end as libc::c_int) != 0)) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    (*(*s).head_).next_
                } else {
                    (((*s).head_)
                        as *mut ACE_Node_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____)
                },
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).set_),
                ::core::ptr::addr_of_mut!((* s))
                    as *mut ACE_Unbounded_Set_Ex_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s3c3b9f37e89103ee(
        mut __a0: *mut ACE_Unbounded_Set_Ex_ACE_Static_Svc_Descriptor____ACE_Unbounded_Set_Default_Comparator_ACE_Static_Svc_Descriptor____,
        mut __a1: bool,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s3c3b9f37e89103ee(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
}
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
pub unsafe fn __vdtor_ucb26d29ba2e3c533(
    __this: *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_,
) {
    let _ = Box::from_raw(__this as *mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_);
}
#[repr(C)]
pub struct __Vtbl_ucb26d29ba2e3c533 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Intrusive_Auto_Ptr_ACE_Service_Gestalt_),
}
pub static __VTBL_ucb26d29ba2e3c533: __Vtbl_ucb26d29ba2e3c533 = __Vtbl_ucb26d29ba2e3c533 {
    __type_info: &__TYPEINFO_22ACE_Intrusive_Auto_Ptr,
    __vdtor: __vdtor_ucb26d29ba2e3c533,
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
pub unsafe fn __vdtor_ue6656ce76ce4e993(__this: *mut ACE_Base_Thread_Adapter) {
    let _ = Box::from_raw(__this as *mut ACE_Thread_Adapter);
}
pub unsafe fn __vthunk_oub40ec5ed4e157b37_iu11d7950624b3f9a1(
    __this: *mut ACE_Base_Thread_Adapter,
) -> *mut libc::c_void {
    <ACE_Thread_Adapter>::invoke((__this as *mut ACE_Thread_Adapter))
}
pub unsafe fn __vthunk_ou5e5fda67bfbfca07_iu5e5fda67bfbfca07(
    __this: *mut ACE_Thread_Adapter,
) -> *mut libc::c_void {
    <ACE_Thread_Adapter>::invoke_i((__this as *mut ACE_Thread_Adapter))
}
#[repr(C)]
pub struct __Vtbl_ue6656ce76ce4e993 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Base_Thread_Adapter),
    pub vfn_u11d7950624b3f9a1: unsafe fn(
        *mut ACE_Base_Thread_Adapter,
    ) -> *mut libc::c_void,
    pub vfn_u5e5fda67bfbfca07: unsafe fn(*mut ACE_Thread_Adapter) -> *mut libc::c_void,
}
pub static __VTBL_ue6656ce76ce4e993: __Vtbl_ue6656ce76ce4e993 = __Vtbl_ue6656ce76ce4e993 {
    __type_info: &__TYPEINFO_18ACE_Thread_Adapter,
    __vdtor: __vdtor_ue6656ce76ce4e993,
    vfn_u11d7950624b3f9a1: __vthunk_oub40ec5ed4e157b37_iu11d7950624b3f9a1,
    vfn_u5e5fda67bfbfca07: __vthunk_ou5e5fda67bfbfca07_iu5e5fda67bfbfca07,
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
pub unsafe fn __vdtor_u5b0403935513b8a9(__this: *mut ACE_Location_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Location_Node);
}
pub unsafe fn __vthunk_oudc7fab4911c7a1fb_iudc7fab4911c7a1fb(
    __this: *mut ACE_Location_Node,
    p0: *mut libc::c_void,
) {
    <ACE_Location_Node>::set_symbol((__this as *mut ACE_Location_Node), p0)
}
pub unsafe fn __vthunk_ou3c863d745f296010_iu3c863d745f296010(
    __this: *mut ACE_Location_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
    p2: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
) -> *mut libc::c_void {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_u5b0403935513b8a9 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Location_Node),
    pub vfn_udc7fab4911c7a1fb: unsafe fn(*mut ACE_Location_Node, *mut libc::c_void),
    pub vfn_u3c863d745f296010: unsafe fn(
        *mut ACE_Location_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
        *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut libc::c_void,
}
pub static __VTBL_u5b0403935513b8a9: __Vtbl_u5b0403935513b8a9 = __Vtbl_u5b0403935513b8a9 {
    __type_info: &__TYPEINFO_17ACE_Location_Node,
    __vdtor: __vdtor_u5b0403935513b8a9,
    vfn_udc7fab4911c7a1fb: __vthunk_oudc7fab4911c7a1fb_iudc7fab4911c7a1fb,
    vfn_u3c863d745f296010: __vthunk_ou3c863d745f296010_iu3c863d745f296010,
};
pub unsafe fn __vdtor_uf6956a2932fdd159(__this: *mut ACE_Service_Config) {
    let _ = Box::from_raw(__this as *mut ACE_Service_Config);
}
pub unsafe fn __vthunk_ou2de6f86bd6d76c3a_iu2de6f86bd6d76c3a(
    __this: *mut ACE_Service_Config,
    p0: *const libc::c_char,
    p1: *const libc::c_char,
    p2: bool,
    p3: bool,
    p4: bool,
) -> libc::c_int {
    <ACE_Service_Config>::open_i((__this as *mut ACE_Service_Config), p0, p1, p2, p3, p4)
}
pub unsafe fn __vthunk_oubec69e76eca3101a_iubec69e76eca3101a(
    __this: *mut ACE_Service_Config,
    p0: libc::c_int,
    p1: *mut *mut libc::c_char,
) -> libc::c_int {
    <ACE_Service_Config>::parse_args_i((__this as *mut ACE_Service_Config), p0, p1)
}
#[repr(C)]
pub struct __Vtbl_uf6956a2932fdd159 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Service_Config),
    pub vfn_u2de6f86bd6d76c3a: unsafe fn(
        *mut ACE_Service_Config,
        *const libc::c_char,
        *const libc::c_char,
        bool,
        bool,
        bool,
    ) -> libc::c_int,
    pub vfn_ubec69e76eca3101a: unsafe fn(
        *mut ACE_Service_Config,
        libc::c_int,
        *mut *mut libc::c_char,
    ) -> libc::c_int,
}
pub static __VTBL_uf6956a2932fdd159: __Vtbl_uf6956a2932fdd159 = __Vtbl_uf6956a2932fdd159 {
    __type_info: &__TYPEINFO_18ACE_Service_Config,
    __vdtor: __vdtor_uf6956a2932fdd159,
    vfn_u2de6f86bd6d76c3a: __vthunk_ou2de6f86bd6d76c3a_iu2de6f86bd6d76c3a,
    vfn_ubec69e76eca3101a: __vthunk_oubec69e76eca3101a_iubec69e76eca3101a,
};
pub unsafe fn __vdtor_u4e15e2dbc14be98b(__this: *mut ACE_Parse_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Parse_Node);
}
pub unsafe fn __vthunk_ouf4f99d81e0986cd7_iuf4f99d81e0986cd7(
    __this: *mut ACE_Parse_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
) {
    panic!("pure virtual function called")
}
#[repr(C)]
pub struct __Vtbl_u4e15e2dbc14be98b {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Parse_Node),
    pub vfn_uf4f99d81e0986cd7: unsafe fn(
        *mut ACE_Parse_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
    ),
}
pub static __VTBL_u4e15e2dbc14be98b: __Vtbl_u4e15e2dbc14be98b = __Vtbl_u4e15e2dbc14be98b {
    __type_info: &__TYPEINFO_14ACE_Parse_Node,
    __vdtor: __vdtor_u4e15e2dbc14be98b,
    vfn_uf4f99d81e0986cd7: __vthunk_ouf4f99d81e0986cd7_iuf4f99d81e0986cd7,
};
pub unsafe fn __vdtor_uf289fa8065239b36(__this: *mut ACE_Parse_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Suspend_Node);
}
pub unsafe fn __vthunk_ou5ce21d0575f9379b_iuf4f99d81e0986cd7(
    __this: *mut ACE_Parse_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
) {
    <ACE_Suspend_Node>::apply((__this as *mut ACE_Suspend_Node), p0, p1)
}
#[repr(C)]
pub struct __Vtbl_uf289fa8065239b36 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Parse_Node),
    pub vfn_uf4f99d81e0986cd7: unsafe fn(
        *mut ACE_Parse_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
    ),
}
pub static __VTBL_uf289fa8065239b36: __Vtbl_uf289fa8065239b36 = __Vtbl_uf289fa8065239b36 {
    __type_info: &__TYPEINFO_16ACE_Suspend_Node,
    __vdtor: __vdtor_uf289fa8065239b36,
    vfn_uf4f99d81e0986cd7: __vthunk_ou5ce21d0575f9379b_iuf4f99d81e0986cd7,
};
pub unsafe fn __vdtor_u766fa49030a17a1d(__this: *mut ACE_Parse_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Resume_Node);
}
pub unsafe fn __vthunk_ou7670418e10875a91_iuf4f99d81e0986cd7(
    __this: *mut ACE_Parse_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
) {
    <ACE_Resume_Node>::apply((__this as *mut ACE_Resume_Node), p0, p1)
}
#[repr(C)]
pub struct __Vtbl_u766fa49030a17a1d {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Parse_Node),
    pub vfn_uf4f99d81e0986cd7: unsafe fn(
        *mut ACE_Parse_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
    ),
}
pub static __VTBL_u766fa49030a17a1d: __Vtbl_u766fa49030a17a1d = __Vtbl_u766fa49030a17a1d {
    __type_info: &__TYPEINFO_15ACE_Resume_Node,
    __vdtor: __vdtor_u766fa49030a17a1d,
    vfn_uf4f99d81e0986cd7: __vthunk_ou7670418e10875a91_iuf4f99d81e0986cd7,
};
pub unsafe fn __vdtor_u214f6b9bd19d1f6c(__this: *mut ACE_Parse_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Remove_Node);
}
pub unsafe fn __vthunk_ou0b082b004d4d300f_iuf4f99d81e0986cd7(
    __this: *mut ACE_Parse_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
) {
    <ACE_Remove_Node>::apply((__this as *mut ACE_Remove_Node), p0, p1)
}
#[repr(C)]
pub struct __Vtbl_u214f6b9bd19d1f6c {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Parse_Node),
    pub vfn_uf4f99d81e0986cd7: unsafe fn(
        *mut ACE_Parse_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
    ),
}
pub static __VTBL_u214f6b9bd19d1f6c: __Vtbl_u214f6b9bd19d1f6c = __Vtbl_u214f6b9bd19d1f6c {
    __type_info: &__TYPEINFO_15ACE_Remove_Node,
    __vdtor: __vdtor_u214f6b9bd19d1f6c,
    vfn_uf4f99d81e0986cd7: __vthunk_ou0b082b004d4d300f_iuf4f99d81e0986cd7,
};
pub unsafe fn __vdtor_u60042e6f9d4f5a0e(__this: *mut ACE_Parse_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Static_Node);
}
pub unsafe fn __vthunk_ou075bd831a0df7583_iuf4f99d81e0986cd7(
    __this: *mut ACE_Parse_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
) {
    <ACE_Static_Node>::apply((__this as *mut ACE_Static_Node), p0, p1)
}
pub unsafe fn __vthunk_ou178b9b285b935234_iu178b9b285b935234(
    __this: *mut ACE_Static_Node,
    p0: *const ACE_Service_Gestalt,
) -> *const ACE_Service_Type {
    <ACE_Static_Node>::record((__this as *mut ACE_Static_Node), p0)
}
#[repr(C)]
pub struct __Vtbl_u60042e6f9d4f5a0e {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Parse_Node),
    pub vfn_uf4f99d81e0986cd7: unsafe fn(
        *mut ACE_Parse_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
    ),
    pub vfn_u178b9b285b935234: unsafe fn(
        *mut ACE_Static_Node,
        *const ACE_Service_Gestalt,
    ) -> *const ACE_Service_Type,
}
pub static __VTBL_u60042e6f9d4f5a0e: __Vtbl_u60042e6f9d4f5a0e = __Vtbl_u60042e6f9d4f5a0e {
    __type_info: &__TYPEINFO_15ACE_Static_Node,
    __vdtor: __vdtor_u60042e6f9d4f5a0e,
    vfn_uf4f99d81e0986cd7: __vthunk_ou075bd831a0df7583_iuf4f99d81e0986cd7,
    vfn_u178b9b285b935234: __vthunk_ou178b9b285b935234_iu178b9b285b935234,
};
pub unsafe fn __vdtor_u42b908333a6f854d(__this: *mut ACE_Parse_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Dynamic_Node);
}
pub unsafe fn __vthunk_ouc13aa2ffd4b217ef_iuf4f99d81e0986cd7(
    __this: *mut ACE_Parse_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
) {
    <ACE_Dynamic_Node>::apply((__this as *mut ACE_Dynamic_Node), p0, p1)
}
#[repr(C)]
pub struct __Vtbl_u42b908333a6f854d {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Parse_Node),
    pub vfn_uf4f99d81e0986cd7: unsafe fn(
        *mut ACE_Parse_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
    ),
    pub vfn_u178b9b285b935234: unsafe fn(
        *mut ACE_Static_Node,
        *const ACE_Service_Gestalt,
    ) -> *const ACE_Service_Type,
}
pub static __VTBL_u42b908333a6f854d: __Vtbl_u42b908333a6f854d = __Vtbl_u42b908333a6f854d {
    __type_info: &__TYPEINFO_16ACE_Dynamic_Node,
    __vdtor: __vdtor_u42b908333a6f854d,
    vfn_uf4f99d81e0986cd7: __vthunk_ouc13aa2ffd4b217ef_iuf4f99d81e0986cd7,
    vfn_u178b9b285b935234: __vthunk_ou178b9b285b935234_iu178b9b285b935234,
};
pub unsafe fn __vdtor_ucc6348cec3018548(__this: *mut ACE_Parse_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Stream_Node);
}
pub unsafe fn __vthunk_ou0206c78045318137_iuf4f99d81e0986cd7(
    __this: *mut ACE_Parse_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
) {
    <ACE_Stream_Node>::apply((__this as *mut ACE_Stream_Node), p0, p1)
}
#[repr(C)]
pub struct __Vtbl_ucc6348cec3018548 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Parse_Node),
    pub vfn_uf4f99d81e0986cd7: unsafe fn(
        *mut ACE_Parse_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
    ),
}
pub static __VTBL_ucc6348cec3018548: __Vtbl_ucc6348cec3018548 = __Vtbl_ucc6348cec3018548 {
    __type_info: &__TYPEINFO_15ACE_Stream_Node,
    __vdtor: __vdtor_ucc6348cec3018548,
    vfn_uf4f99d81e0986cd7: __vthunk_ou0206c78045318137_iuf4f99d81e0986cd7,
};
pub unsafe fn __vdtor_uc870d50f1ce330ca(__this: *mut ACE_Parse_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Dummy_Node);
}
pub unsafe fn __vthunk_ou7523654600cb3e37_iuf4f99d81e0986cd7(
    __this: *mut ACE_Parse_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
) {
    <ACE_Dummy_Node>::apply((__this as *mut ACE_Dummy_Node), p0, p1)
}
#[repr(C)]
pub struct __Vtbl_uc870d50f1ce330ca {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Parse_Node),
    pub vfn_uf4f99d81e0986cd7: unsafe fn(
        *mut ACE_Parse_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
    ),
}
pub static __VTBL_uc870d50f1ce330ca: __Vtbl_uc870d50f1ce330ca = __Vtbl_uc870d50f1ce330ca {
    __type_info: &__TYPEINFO_14ACE_Dummy_Node,
    __vdtor: __vdtor_uc870d50f1ce330ca,
    vfn_uf4f99d81e0986cd7: __vthunk_ou7523654600cb3e37_iuf4f99d81e0986cd7,
};
pub unsafe fn __vdtor_u44e2f411f71b725d(__this: *mut ACE_Location_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Object_Node);
}
pub unsafe fn __vthunk_ou9bb11e83dcb1802c_iu3c863d745f296010(
    __this: *mut ACE_Location_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
    p2: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
) -> *mut libc::c_void {
    <ACE_Object_Node>::symbol((__this as *mut ACE_Object_Node), p0, p1, p2)
}
#[repr(C)]
pub struct __Vtbl_u44e2f411f71b725d {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Location_Node),
    pub vfn_udc7fab4911c7a1fb: unsafe fn(*mut ACE_Location_Node, *mut libc::c_void),
    pub vfn_u3c863d745f296010: unsafe fn(
        *mut ACE_Location_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
        *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut libc::c_void,
}
pub static __VTBL_u44e2f411f71b725d: __Vtbl_u44e2f411f71b725d = __Vtbl_u44e2f411f71b725d {
    __type_info: &__TYPEINFO_15ACE_Object_Node,
    __vdtor: __vdtor_u44e2f411f71b725d,
    vfn_udc7fab4911c7a1fb: __vthunk_oudc7fab4911c7a1fb_iudc7fab4911c7a1fb,
    vfn_u3c863d745f296010: __vthunk_ou9bb11e83dcb1802c_iu3c863d745f296010,
};
pub unsafe fn __vdtor_u590c517902f0bac0(__this: *mut ACE_Location_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Function_Node);
}
pub unsafe fn __vthunk_ou5db86597c2d7cc4e_iu3c863d745f296010(
    __this: *mut ACE_Location_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
    p2: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
) -> *mut libc::c_void {
    <ACE_Function_Node>::symbol((__this as *mut ACE_Function_Node), p0, p1, p2)
}
#[repr(C)]
pub struct __Vtbl_u590c517902f0bac0 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Location_Node),
    pub vfn_udc7fab4911c7a1fb: unsafe fn(*mut ACE_Location_Node, *mut libc::c_void),
    pub vfn_u3c863d745f296010: unsafe fn(
        *mut ACE_Location_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
        *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut libc::c_void,
}
pub static __VTBL_u590c517902f0bac0: __Vtbl_u590c517902f0bac0 = __Vtbl_u590c517902f0bac0 {
    __type_info: &__TYPEINFO_17ACE_Function_Node,
    __vdtor: __vdtor_u590c517902f0bac0,
    vfn_udc7fab4911c7a1fb: __vthunk_oudc7fab4911c7a1fb_iudc7fab4911c7a1fb,
    vfn_u3c863d745f296010: __vthunk_ou5db86597c2d7cc4e_iu3c863d745f296010,
};
pub unsafe fn __vdtor_u92844d526b45672b(__this: *mut ACE_Location_Node) {
    let _ = Box::from_raw(__this as *mut ACE_Static_Function_Node);
}
pub unsafe fn __vthunk_ou23c32a3ae3bf64ea_iu3c863d745f296010(
    __this: *mut ACE_Location_Node,
    p0: *mut ACE_Service_Gestalt,
    p1: *mut libc::c_int,
    p2: *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
) -> *mut libc::c_void {
    <ACE_Static_Function_Node>::symbol(
        (__this as *mut ACE_Static_Function_Node),
        p0,
        p1,
        p2,
    )
}
#[repr(C)]
pub struct __Vtbl_u92844d526b45672b {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Location_Node),
    pub vfn_udc7fab4911c7a1fb: unsafe fn(*mut ACE_Location_Node, *mut libc::c_void),
    pub vfn_u3c863d745f296010: unsafe fn(
        *mut ACE_Location_Node,
        *mut ACE_Service_Gestalt,
        *mut libc::c_int,
        *mut Option<unsafe extern "C-unwind" fn(*mut libc::c_void)>,
    ) -> *mut libc::c_void,
}
pub static __VTBL_u92844d526b45672b: __Vtbl_u92844d526b45672b = __Vtbl_u92844d526b45672b {
    __type_info: &__TYPEINFO_24ACE_Static_Function_Node,
    __vdtor: __vdtor_u92844d526b45672b,
    vfn_udc7fab4911c7a1fb: __vthunk_oudc7fab4911c7a1fb_iudc7fab4911c7a1fb,
    vfn_u3c863d745f296010: __vthunk_ou23c32a3ae3bf64ea_iu3c863d745f296010,
};
