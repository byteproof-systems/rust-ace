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
#[repr(C)]
pub struct ACE_Message_Block {
    pub _opaque: [u8; 1],
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
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6ldfindEPKcPcm"]
        pub fn ldfind(
            filename: *const libc::c_char,
            pathname: *mut libc::c_char,
            maxpathnamelen: libc::c_ulong,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6ldopenEPKcS1_"]
        pub fn ldopen(
            filename: *const libc::c_char,
            r#type: *const libc::c_char,
        ) -> *mut super::_IO_FILE;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE6ldnameEPKc"]
        pub fn ldname(entry_point: *const libc::c_char) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE12get_temp_dirEPcm"]
        pub fn get_temp_dir(
            buffer: *mut libc::c_char,
            buffer_len: libc::c_ulong,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE14open_temp_fileEPKcii"]
        pub fn open_temp_file(
            name: *const libc::c_char,
            mode: libc::c_int,
            perm: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7strreplEPccc"]
        pub fn strrepl(
            s: *mut libc::c_char,
            search: libc::c_char,
            replace: libc::c_char,
        ) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE10strsplit_rEPcPKcRS0_"]
        pub fn strsplit_r(
            s: *mut libc::c_char,
            token: *const libc::c_char,
            next_start: *mut *mut libc::c_char,
        ) -> *mut libc::c_char;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE7strreplEPwww"]
        pub fn strrepl_u65535d728da90bba(
            s: *mut libc::wchar_t,
            search: libc::wchar_t,
            replace: libc::wchar_t,
        ) -> libc::c_ulong;
    }
    extern "C-unwind" {
        #[link_name = "_ZN3ACE10strsplit_rEPwPKwRS0_"]
        pub fn strsplit_r_uff181bcf52540678(
            s: *mut libc::wchar_t,
            token: *const libc::wchar_t,
            next_start: *mut *mut libc::wchar_t,
        ) -> *mut libc::wchar_t;
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
#[repr(C)]
pub struct ACE_Object_Manager {
    pub _opaque: [u8; 1],
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
pub struct ACE_Process_Mutex {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Recursive_Thread_Mutex {
    pub _opaque: [u8; 1],
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
pub struct ACE_Reactor {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Thread_Manager {
    pub _opaque: [u8; 1],
}
#[repr(C)]
pub struct ACE_Process {
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
#[repr(C)]
pub struct ACE_Sig_Action {
    pub _opaque: [u8; 1],
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Sig_Handler_sig_pending_: libc::c_int;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Sig_Handler_signal_handlers_: [*mut ACE_Event_Handler; 65usize];
}
#[doc = "* @class ACE_Sig_Handler\n *\n * @brief This is the main dispatcher of signals for ACE.  It improves\n * the existing UNIX signal handling mechanism by allowing C++\n * objects to handle signals in a way that avoids the use of\n * global/static variables and functions.\n *\n * Using this class a program can register an ACE_Event_Handler\n * with the ACE_Sig_Handler in order to handle a designated\n * @a signum.  When a signal occurs that corresponds to this\n * @a signum, the @c handle_signal method of the registered\n * ACE_Event_Handler is invoked automatically."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Sig_Handler {
    pub vptr: *const (),
}
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_Sig_HandlerC1Ev(
    __this: *mut ACE_Sig_Handler,
) {
    ACE_Sig_Handler::new_at(__this)
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Sig_Handlers_sigkey_: libc::c_int;
}
extern "C" {
    #[allow(non_upper_case_globals)]
    pub static mut ACE_Sig_Handlers_third_party_sig_handler_: bool;
}
#[doc = "* @class ACE_Sig_Handlers\n *\n * @brief This is an alternative signal handling dispatcher for ACE.  It\n * allows a list of signal handlers to be registered for each\n * signal.  It also makes SA_RESTART the default mode.\n *\n * Using this class a program can register one or more\n * ACE_Event_Handler with the ACE_Sig_Handler in order to\n * handle a designated @a signum.  When a signal occurs that\n * corresponds to this @a signum, the <handle_signal> methods of\n * all the registered ACE_Event_Handlers are invoked\n * automatically."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Sig_Handlers {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Sig_Handler>,
}
impl Drop for ACE_Sig_Handlers {
    fn drop(&mut self) {
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
#[doc = "* @class ACE_Mem_Map\n *\n * @brief C++ interface OS memory mapping system call.\n *\n * This class works with both the mmap(2) UNIX system and the\n * Win32 family of memory mapping system calls."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Mem_Map {
    pub __base_0: ACE_Copy_Disabled,
    pub base_addr_: *mut libc::c_void,
    pub filename_: [libc::c_char; 4097usize],
    pub length_: libc::c_ulong,
    pub handle_: libc::c_int,
    pub file_mapping_: libc::c_int,
    pub close_handle_: bool,
}
impl Drop for ACE_Mem_Map {
    fn drop(&mut self) {
        unsafe {
            extern "C-unwind" {
                #[link_name = "__acedtor_ue12ccc626227af79"]
                fn __ext(__this: *mut ACE_Mem_Map);
            }
            __ext(self as *mut Self);
        }
    }
}
#[doc = "* @class ACE_MMAP_Memory_Pool\n *\n * @brief Make a memory pool that is based on @c mmap(2).  This\n * implementation allows memory to be shared between processes."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_MMAP_Memory_Pool {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Event_Handler>,
    pub signal_handler_: ::core::mem::ManuallyDrop<ACE_Sig_Handler>,
    pub mmap_: ::core::mem::ManuallyDrop<ACE_Mem_Map>,
    pub base_addr_: *mut libc::c_void,
    pub use_fixed_addr_: libc::c_int,
    pub flags_: libc::c_int,
    pub write_each_page_: bool,
    pub minimum_bytes_: libc::c_ulong,
    pub backing_store_name_: [libc::c_char; 4097usize],
    pub guess_on_fault_: bool,
    pub sa_: libc::c_int,
    pub file_mode_: libc::c_uint,
    pub install_signal_handler_: bool,
}
#[doc = "* @class ACE_Lite_MMAP_Memory_Pool\n *\n * @brief Make a ``lighter-weight'' memory pool based ACE_Mem_Map.\n *\n * This implementation allows memory to be shared between\n * processes.  However, unlike the ACE_MMAP_Memory_Pool\n * the @c sync methods are no-ops, which means that we don't pay\n * for the price of flushing the memory to the backing store on\n * every update.  Naturally, this trades off increased\n * performance for less reliability if the machine crashes."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Lite_MMAP_Memory_Pool {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_MMAP_Memory_Pool>,
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ {
    pub __base_0: ::core::mem::ManuallyDrop<
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    >,
}
impl Drop for ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ {
    fn drop(&mut self) {
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
#[export_name = "_ZN10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEC1EPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEC1EPKc(
    __this: *mut ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_,
    __a0: *const libc::c_char,
) {
    ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_::new_at_sd318f86520b5d207(
        __this,
        __a0,
    )
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Allocator>,
    pub allocator_: ::core::mem::ManuallyDrop<
        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_,
    >,
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEEC1EPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEEC1EPKc(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    __a0: *const libc::c_char,
) {
    ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::new_at_s039880e4b7dd6368(
        __this,
        __a0,
    )
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4findEPKcRPv"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4findEPKcRPv(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    name: *const libc::c_char,
    pointer: *mut *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::find_sd1d97c7eb0448d41(
            __this,
            name,
            pointer,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4bindEPKcPvi"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4bindEPKcPvi(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    name: *const libc::c_char,
    pointer: *mut libc::c_void,
    duplicates: libc::c_int,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::bind(
            __this,
            name,
            pointer,
            duplicates,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6callocEmc"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6callocEmc(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    nbytes: libc::c_ulong,
    initial_value: libc::c_char,
) -> *mut libc::c_void {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::calloc_s1ae7c325fc123850(
            __this,
            nbytes,
            initial_value,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6callocEmmc"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6callocEmmc(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    n_elem: libc::c_ulong,
    elem_size: libc::c_ulong,
    initial_value: libc::c_char,
) -> *mut libc::c_void {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::calloc_s20377fc4621028db(
            __this,
            n_elem,
            elem_size,
            initial_value,
        )
    }
}
#[export_name = "_ZNK21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4dumpEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4dumpEv(
    __this: *const ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
) {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::dump(
            __this,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4findEPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4findEPKc(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    name: *const libc::c_char,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::find_se7e55738a6d934ec(
            __this,
            name,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4freeEPv"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4freeEPv(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    ptr: *mut libc::c_void,
) {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::free(
            __this,
            ptr,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6mallocEm"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6mallocEm(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    nbytes: libc::c_ulong,
) -> *mut libc::c_void {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::malloc(
            __this,
            nbytes,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE7protectEli"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE7protectEli(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    len: libc::c_long,
    flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::protect_s48b3af97b45c173e(
            __this,
            len,
            flags,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE7protectEPvmi"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE7protectEPvmi(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    addr: *mut libc::c_void,
    len: libc::c_ulong,
    flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::protect_sdf95393c41ed2b14(
            __this,
            addr,
            len,
            flags,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6removeEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6removeEv(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::remove(
            __this,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4syncEli"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4syncEli(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    len: libc::c_long,
    flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::sync_s48b3af97b45c173e(
            __this,
            len,
            flags,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4syncEPvmi"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE4syncEPvmi(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    addr: *mut libc::c_void,
    len: libc::c_ulong,
    flags: libc::c_int,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::sync_sdf95393c41ed2b14(
            __this,
            addr,
            len,
            flags,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE7trybindEPKcRPv"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE7trybindEPKcRPv(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    name: *const libc::c_char,
    pointer: *mut *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::trybind(
            __this,
            name,
            pointer,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6unbindEPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6unbindEPKc(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    name: *const libc::c_char,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::unbind_se7e55738a6d934ec(
            __this,
            name,
        )
    }
}
#[export_name = "_ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6unbindEPKcRPv"]
pub unsafe extern "C-unwind" fn __xtu__ZN21ACE_Allocator_AdapterI10ACE_MallocI20ACE_MMAP_Memory_Pool14ACE_Null_MutexEE6unbindEPKcRPv(
    __this: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    name: *const libc::c_char,
    pointer: *mut *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__::unbind_sd1d97c7eb0448d41(
            __this,
            name,
            pointer,
        )
    }
}
impl Drop for ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {}
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.allocator_);
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
pub type ACE_System_Time_MALLOC = ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_;
pub type ACE_System_Time_ALLOCATOR = ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__;
#[doc = "* @class ACE_System_Time\n *\n * @brief Defines the timer services of the OS interface to access the\n * system time either on the local host or on the central time\n * server in the network."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_System_Time {
    pub shmem_: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    pub poolname_: [libc::c_char; 4097usize],
    pub delta_time_: *mut libc::c_long,
}
#[export_name = "_ZN15ACE_System_TimeC1EPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_System_TimeC1EPKc(
    __this: *mut ACE_System_Time,
    __a0: *const libc::c_char,
) {
    ACE_System_Time::new_at(__this, __a0)
}
impl Drop for ACE_System_Time {
    fn drop(&mut self) {
        unsafe {
            let __this: *mut Self = self as *mut Self;
            {
                {
                    let __p = (*__this).shmem_;
                    if !__p.is_null() {
                        let __vt: *const __Vtbl_uf2113694993e252c = *(__p
                            as *const *const __Vtbl_uf2113694993e252c);
                        ((*__vt).__vdtor)(__p as *mut ACE_Allocator);
                    }
                };
            }
            ()
        }
    }
}
#[export_name = "__acedtor_u43c8efd015dd752e"]
pub unsafe extern "C-unwind" fn __acedtor_u43c8efd015dd752e(
    __this: *mut ACE_System_Time,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
    }
}
#[export_name = "_ZN15ACE_System_Time21get_local_system_timeERl"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_System_Time21get_local_system_timeERl(
    time_out: *mut libc::c_long,
) -> libc::c_int {
    unsafe { ACE_System_Time::get_local_system_time(time_out) }
}
#[export_name = "_ZN15ACE_System_Time21get_local_system_timeER14ACE_Time_Value"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_System_Time21get_local_system_timeER14ACE_Time_Value(
    time_out: *mut ACE_Time_Value,
) -> libc::c_int {
    unsafe { ACE_System_Time::get_local_system_time_ue35a4077f4b76f21(time_out) }
}
#[export_name = "_ZN15ACE_System_Time22get_master_system_timeERl"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_System_Time22get_master_system_timeERl(
    __this: *mut ACE_System_Time,
    time_out: *mut libc::c_long,
) -> libc::c_int {
    unsafe { ACE_System_Time::get_master_system_time(__this, time_out) }
}
#[export_name = "_ZN15ACE_System_Time22get_master_system_timeER14ACE_Time_Value"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_System_Time22get_master_system_timeER14ACE_Time_Value(
    __this: *mut ACE_System_Time,
    time_out: *mut ACE_Time_Value,
) -> libc::c_int {
    unsafe {
        ACE_System_Time::get_master_system_time_u6e625afa58325265(__this, time_out)
    }
}
#[export_name = "_ZN15ACE_System_Time22sync_local_system_timeENS_9Sync_ModeE"]
pub unsafe extern "C-unwind" fn __xtu__ZN15ACE_System_Time22sync_local_system_timeENS_9Sync_ModeE(
    __this: *mut ACE_System_Time,
    _anon_0: libc::c_uint,
) -> libc::c_int {
    unsafe { ACE_System_Time::sync_local_system_time(__this, _anon_0) }
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
#[doc = "* @class ACE_New_Allocator\n *\n * @brief Defines a class that provided a simple implementation of\n * memory allocation.\n *\n * This class uses the new/delete operators to allocate and free up\n * memory.  Please note that the only methods that are supported are\n * malloc(), calloc(), and free(). All other methods are no-ops that\n * return -1 and set @c errno to @c ENOTSUP.  If you require this\n * functionality, please use: ACE_Allocator_Adapter <ACE_Malloc\n * <ACE_LOCAL_MEMORY_POOL, MUTEX> >, which will allow you to use the\n * added functionality of bind/find/etc. while using the new/delete\n * operators."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_New_Allocator {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Allocator>,
}
impl Drop for ACE_New_Allocator {
    fn drop(&mut self) {
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
#[doc = "* @class ACE_Static_Allocator_Base\n *\n * @brief Defines a class that provided a highly optimized memory\n * management scheme for allocating memory statically.\n *\n * This class manages a fixed-size @c POOL_SIZE of memory.  Every\n * time malloc()/calloc() is called, it simply moves an internal\n * index forward and returns a pointer to the requested chunk.\n * All memory is allocated statically (typically via the\n * ACE_Static_Allocator template) and free() is a no-op.  This\n * behavior is useful for use-cases where all the memory\n * allocation needs are known in advance and no deletions ever\n * occur."]
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Static_Allocator_Base {
    pub __base_0: ::core::mem::ManuallyDrop<ACE_Allocator>,
    pub buffer_: *mut libc::c_char,
    pub size_: libc::c_ulong,
    pub offset_: libc::c_ulong,
}
impl Drop for ACE_Static_Allocator_Base {
    fn drop(&mut self) {
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.__base_0);
        }
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN25ACE_Static_Allocator_BaseC1EPcm(
    __this: *mut ACE_Static_Allocator_Base,
    __a0: *mut libc::c_char,
    __a1: libc::c_ulong,
) {
    ACE_Static_Allocator_Base::new_at(__this, __a0, __a1)
}
#[repr(C)]
#[derive(Clone)]
pub struct ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_ {
    pub cb_ptr_: *mut ACE_Control_Block,
    pub memory_pool_: ::core::mem::ManuallyDrop<ACE_MMAP_Memory_Pool>,
    pub lock_: *mut ACE_Null_Mutex,
    pub delete_lock_: bool,
    pub bad_flag_: libc::c_int,
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE11shared_freeEPv(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    ap: *mut libc::c_void,
) {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::shared_free(
            __this,
            ap,
        )
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE4openEv(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
) -> libc::c_int {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::open(
            __this,
        )
    }
}
#[export_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockEC1EPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockEC1EPKc(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    __a0: *const libc::c_char,
) {
    ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::new_at_s64e2e5cf64745a9b(
        __this,
        __a0,
    )
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE11shared_findEPKc(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    name: *const libc::c_char,
) -> *mut libc::c_void {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::shared_find(
            __this,
            name,
        )
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE4findEPKcRPv(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    name: *const libc::c_char,
    pointer: *mut *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::find_sb79c8f0521a6ddd8(
            __this,
            name,
            pointer,
        )
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE13shared_mallocEm(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    nbytes: libc::c_ulong,
) -> *mut libc::c_void {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::shared_malloc(
            __this,
            nbytes,
        )
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6mallocEm(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    nbytes: libc::c_ulong,
) -> *mut libc::c_void {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::malloc(
            __this,
            nbytes,
        )
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE11shared_bindEPKcPv(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    name: *const libc::c_char,
    pointer: *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::shared_bind(
            __this,
            name,
            pointer,
        )
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE4bindEPKcPvi(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    name: *const libc::c_char,
    pointer: *mut libc::c_void,
    duplicates: libc::c_int,
) -> libc::c_int {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::bind(
            __this,
            name,
            pointer,
            duplicates,
        )
    }
}
#[export_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6callocEmc"]
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6callocEmc(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    nbytes: libc::c_ulong,
    initial_value: libc::c_char,
) -> *mut libc::c_void {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::calloc_s6ac7ac984caf1001(
            __this,
            nbytes,
            initial_value,
        )
    }
}
#[export_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6callocEmmc"]
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6callocEmmc(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    n_elem: libc::c_ulong,
    elem_size: libc::c_ulong,
    initial_value: libc::c_char,
) -> *mut libc::c_void {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::calloc_s7fc4b13687d88c4a(
            __this,
            n_elem,
            elem_size,
            initial_value,
        )
    }
}
#[export_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE4findEPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE4findEPKc(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    name: *const libc::c_char,
) -> libc::c_int {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::find_s07b337cff4ad6a1f(
            __this,
            name,
        )
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE4freeEPv(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    ptr: *mut libc::c_void,
) {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::free(
            __this,
            ptr,
        )
    }
}
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6removeEv(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
) -> libc::c_int {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::remove(
            __this,
        )
    }
}
#[export_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE7trybindEPKcRPv"]
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE7trybindEPKcRPv(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    name: *const libc::c_char,
    pointer: *mut *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::trybind(
            __this,
            name,
            pointer,
        )
    }
}
#[export_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6unbindEPKcRPv"]
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6unbindEPKcRPv(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    name: *const libc::c_char,
    pointer: *mut *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::unbind_sb79c8f0521a6ddd8(
            __this,
            name,
            pointer,
        )
    }
}
#[export_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6unbindEPKc"]
pub unsafe extern "C-unwind" fn __xtu__ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE6unbindEPKc(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
    name: *const libc::c_char,
) -> libc::c_int {
    unsafe {
        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_::unbind_s07b337cff4ad6a1f(
            __this,
            name,
        )
    }
}
impl Drop for ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_ {
    fn drop(&mut self) {
        {
            unsafe {
                let __this: *mut Self = self as *mut Self;
                {
                    if (((*__this).delete_lock_ as libc::c_int) != 0) {
                        {
                            let __d = (*__this).lock_;
                            if !__d.is_null() {
                                let _ = Box::from_raw(__d as *mut ACE_Null_Mutex);
                            }
                        };
                        (*__this).lock_ = ((0) as *mut ACE_Null_Mutex);
                    }
                }
                ()
            }
        }
        unsafe {
            ::core::mem::ManuallyDrop::drop(&mut self.memory_pool_);
        }
    }
}
pub unsafe extern "C-unwind" fn __acedtor_uddb116a52d4de85c(
    __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
) {
    unsafe {
        ::core::ptr::drop_in_place(__this);
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
impl ACE_Sig_Handler {
    ///Default constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            *(__this as *mut *const ()) = &__VTBL_ue6180d8ab1473747
                as *const __Vtbl_ue6180d8ab1473747 as *const ();
            {}
            ()
        }
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    #[doc = "* Add a new ACE_Event_Handler and a new sigaction associated with\n   * @a signum.  Passes back the existing ACE_Event_Handler and its\n   * sigaction if pointers are non-zero.  Returns -1 on failure and >=\n   * 0 on success."]
    pub unsafe fn register_handler(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut new_sh: *mut ACE_Event_Handler,
        mut new_disp: *mut ACE_Sig_Action,
        mut old_sh: *mut *mut ACE_Event_Handler,
        mut old_disp: *mut ACE_Sig_Action,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler16register_handlerEiP17ACE_Event_HandlerP14ACE_Sig_ActionPS1_S3_"]
            fn __ext(
                __this: *mut ACE_Sig_Handler,
                signum: libc::c_int,
                new_sh: *mut ACE_Event_Handler,
                new_disp: *mut ACE_Sig_Action,
                old_sh: *mut *mut ACE_Event_Handler,
                old_disp: *mut ACE_Sig_Action,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Sig_Handler, signum, new_sh, new_disp, old_sh, old_disp)
    }
    #[doc = "* Remove the ACE_Event_Handler currently associated with\n   * @a signum.  @a sigkey is ignored in this implementation since there\n   * is only one instance of a signal handler.  Install the new\n   * disposition (if given) and return the previous disposition (if\n   * desired by the caller).  Returns 0 on success and -1 if @a signum\n   * is invalid."]
    pub unsafe fn remove_handler(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut new_disp: *mut ACE_Sig_Action,
        mut old_disp: *mut ACE_Sig_Action,
        mut sigkey: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler14remove_handlerEiP14ACE_Sig_ActionS1_i"]
            fn __ext(
                __this: *mut ACE_Sig_Handler,
                signum: libc::c_int,
                new_disp: *mut ACE_Sig_Action,
                old_disp: *mut ACE_Sig_Action,
                sigkey: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Sig_Handler, signum, new_disp, old_disp, sigkey)
    }
    ///True if there is a pending signal.
    pub unsafe fn sig_pending() -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler11sig_pendingEv"]
            fn __ext() -> libc::c_int;
        }
        __ext()
    }
    ///Reset the value of <sig_pending_> so that no signal is pending.
    pub unsafe fn sig_pending_ua639f6eab31bf677(mut _anon_0: libc::c_int) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler11sig_pendingEi"]
            fn __ext(_anon_0: libc::c_int);
        }
        __ext(_anon_0)
    }
    ///Return the ACE_Sig_Handler associated with @a signum.
    pub unsafe fn handler(
        __this: *mut Self,
        mut signum: libc::c_int,
    ) -> *mut ACE_Event_Handler {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler7handlerEi"]
            fn __ext(
                __this: *mut ACE_Sig_Handler,
                signum: libc::c_int,
            ) -> *mut ACE_Event_Handler;
        }
        __ext(__this as *mut ACE_Sig_Handler, signum)
    }
    /**Set a new ACE_Event_Handler that is associated with @a signum.
  /// Return the existing handler.*/
    pub unsafe fn handler_uc91b6b15865c04fe(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut _anon_1: *mut ACE_Event_Handler,
    ) -> *mut ACE_Event_Handler {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler7handlerEiP17ACE_Event_Handler"]
            fn __ext(
                __this: *mut ACE_Sig_Handler,
                signum: libc::c_int,
                _anon_1: *mut ACE_Event_Handler,
            ) -> *mut ACE_Event_Handler;
        }
        __ext(__this as *mut ACE_Sig_Handler, signum, _anon_1)
    }
    #[doc = "* Callback routine registered with sigaction(2) that dispatches the\n   * <handle_signal> method of the appropriate pre-registered\n   * ACE_Event_Handler."]
    pub unsafe fn dispatch(
        mut _anon_0: libc::c_int,
        mut _anon_1: *mut siginfo_t,
        mut _anon_2: *mut ucontext_t,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler8dispatchEiP9siginfo_tP10ucontext_t"]
            fn __ext(
                _anon_0: libc::c_int,
                _anon_1: *mut siginfo_t,
                _anon_2: *mut ucontext_t,
            );
        }
        __ext(_anon_0, _anon_1, _anon_2)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK15ACE_Sig_Handler4dumpEv"]
            fn __ext(__this: *const ACE_Sig_Handler);
        }
        __ext(__this as *const ACE_Sig_Handler)
    }
    #[doc = "* Set a new ACE_Event_Handler that is associated with @a signum.\n   * Return the existing handler.  Does not acquire any locks so that\n   * it can be called from a signal handler, such as <dispatch>."]
    pub unsafe fn handler_i(
        mut signum: libc::c_int,
        mut _anon_1: *mut ACE_Event_Handler,
    ) -> *mut ACE_Event_Handler {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler9handler_iEiP17ACE_Event_Handler"]
            fn __ext(
                signum: libc::c_int,
                _anon_1: *mut ACE_Event_Handler,
            ) -> *mut ACE_Event_Handler;
        }
        __ext(signum, _anon_1)
    }
    #[doc = "* This implementation method is called by <register_handler> and\n   * @c dispatch.  It doesn't do any locking so that it can be called\n   * within a signal handler, such as @c dispatch.  It adds a new\n   * ACE_Event_Handler and a new sigaction associated with @a signum.\n   * Passes back the existing ACE_Event_Handler and its sigaction if\n   * pointers are non-zero.  Returns -1 on failure and >= 0 on\n   * success."]
    pub unsafe fn register_handler_i(
        mut signum: libc::c_int,
        mut new_sh: *mut ACE_Event_Handler,
        mut new_disp: *mut ACE_Sig_Action,
        mut old_sh: *mut *mut ACE_Event_Handler,
        mut old_disp: *mut ACE_Sig_Action,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler18register_handler_iEiP17ACE_Event_HandlerP14ACE_Sig_ActionPS1_S3_"]
            fn __ext(
                signum: libc::c_int,
                new_sh: *mut ACE_Event_Handler,
                new_disp: *mut ACE_Sig_Action,
                old_sh: *mut *mut ACE_Event_Handler,
                old_disp: *mut ACE_Sig_Action,
            ) -> libc::c_int;
        }
        __ext(signum, new_sh, new_disp, old_sh, old_disp)
    }
    pub unsafe fn remove_handler_i(
        mut signum: libc::c_int,
        mut new_disp: *mut ACE_Sig_Action,
        mut old_disp: *mut ACE_Sig_Action,
        mut sigkey: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN15ACE_Sig_Handler16remove_handler_iEiP14ACE_Sig_ActionS1_i"]
            fn __ext(
                signum: libc::c_int,
                new_disp: *mut ACE_Sig_Action,
                old_disp: *mut ACE_Sig_Action,
                sigkey: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(signum, new_disp, old_disp, sigkey)
    }
    ///Check whether the SIGNUM is within the legal range of signals.
    pub unsafe fn in_range(mut signum: libc::c_int) -> libc::c_int {
        unsafe {
            {
                return (((((((((signum as libc::c_int)) > (((0) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                    && (((((signum as libc::c_int))
                        < (((((((64) as libc::c_int)).wrapping_add((1) as libc::c_int)))
                            as libc::c_int))) as libc::c_int as libc::c_int) != 0))
                    as libc::c_int)) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Sig_Handlers {
    ///Default constructor
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Sig_HandlersC1Ev"]
            fn __ext(__this: *mut ACE_Sig_Handlers);
        }
        __ext(__this as *mut ACE_Sig_Handlers)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    #[doc = "* Add a new ACE_Event_Handler and a new sigaction associated with\n   * @a signum.  Passes back the existing ACE_Event_Handler and its\n   * sigaction if pointers are non-zero.  Returns -1 on failure and\n   * a <sigkey> that is >= 0 on success."]
    pub unsafe fn register_handler(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut new_sh: *mut ACE_Event_Handler,
        mut new_disp: *mut ACE_Sig_Action,
        mut old_sh: *mut *mut ACE_Event_Handler,
        mut old_disp: *mut ACE_Sig_Action,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Sig_Handlers16register_handlerEiP17ACE_Event_HandlerP14ACE_Sig_ActionPS1_S3_"]
            fn __ext(
                __this: *mut ACE_Sig_Handlers,
                signum: libc::c_int,
                new_sh: *mut ACE_Event_Handler,
                new_disp: *mut ACE_Sig_Action,
                old_sh: *mut *mut ACE_Event_Handler,
                old_disp: *mut ACE_Sig_Action,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_Sig_Handlers,
            signum,
            new_sh,
            new_disp,
            old_sh,
            old_disp,
        )
    }
    #[doc = "* Remove an ACE_Event_Handler currently associated with @a signum.\n   * We remove the handler if (1) its sigkey> matches the @a sigkey\n   * passed as a parameter or (2) if we've been told to remove all the\n   * handlers, i.e., @a sigkey == -1.  If a new disposition is given it\n   * is installed and the previous disposition is returned (if desired\n   * by the caller).  Returns 0 on success and -1 if @a signum is\n   * invalid."]
    pub unsafe fn remove_handler(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut new_disp: *mut ACE_Sig_Action,
        mut old_disp: *mut ACE_Sig_Action,
        mut sigkey: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Sig_Handlers14remove_handlerEiP14ACE_Sig_ActionS1_i"]
            fn __ext(
                __this: *mut ACE_Sig_Handlers,
                signum: libc::c_int,
                new_disp: *mut ACE_Sig_Action,
                old_disp: *mut ACE_Sig_Action,
                sigkey: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Sig_Handlers, signum, new_disp, old_disp, sigkey)
    }
    /**Return the head of the list of ACE_Sig_Handlers associated with
  /// SIGNUM.*/
    pub unsafe fn handler(
        __this: *mut Self,
        mut signum: libc::c_int,
    ) -> *mut ACE_Event_Handler {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Sig_Handlers7handlerEi"]
            fn __ext(
                __this: *mut ACE_Sig_Handlers,
                signum: libc::c_int,
            ) -> *mut ACE_Event_Handler;
        }
        __ext(__this as *mut ACE_Sig_Handlers, signum)
    }
    #[doc = "* Set a new ACE_Event_Handler that is associated with SIGNUM at\n   * the head of the list of signals.  Return the existing handler\n   * that was at the head."]
    pub unsafe fn handler_u4822a5c3ba6d369c(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut _anon_1: *mut ACE_Event_Handler,
    ) -> *mut ACE_Event_Handler {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Sig_Handlers7handlerEiP17ACE_Event_Handler"]
            fn __ext(
                __this: *mut ACE_Sig_Handlers,
                signum: libc::c_int,
                _anon_1: *mut ACE_Event_Handler,
            ) -> *mut ACE_Event_Handler;
        }
        __ext(__this as *mut ACE_Sig_Handlers, signum, _anon_1)
    }
    #[doc = "* Callback routine registered with sigaction(2) that dispatches the\n   * handle_signal() method of all the pre-registered\n   * ACE_Event_Handlers for @a signum"]
    pub unsafe fn dispatch(
        mut signum: libc::c_int,
        mut _anon_1: *mut siginfo_t,
        mut _anon_2: *mut ucontext_t,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN16ACE_Sig_Handlers8dispatchEiP9siginfo_tP10ucontext_t"]
            fn __ext(
                signum: libc::c_int,
                _anon_1: *mut siginfo_t,
                _anon_2: *mut ucontext_t,
            );
        }
        __ext(signum, _anon_1, _anon_2)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK16ACE_Sig_Handlers4dumpEv"]
            fn __ext(__this: *const ACE_Sig_Handlers);
        }
        __ext(__this as *const ACE_Sig_Handlers)
    }
}
impl ACE_Mem_Map {
    ///Default constructor.
    pub unsafe fn new_at(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Mem_MapC1Ev"]
            fn __ext(__this: *mut ACE_Mem_Map);
        }
        __ext(__this as *mut ACE_Mem_Map)
    }
    pub unsafe fn new() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
    /**Map a file from an open file descriptor @a handle.  This function
  /// will lookup the length of the file if it is not given.*/
    pub unsafe fn new_at_u7d7def492f0113c0(
        __this: *mut Self,
        mut __a0: libc::c_int,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: libc::c_int,
        mut __a4: *mut libc::c_void,
        mut __a5: libc::c_long,
        mut __a6: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Mem_MapC1EimiiPvli"]
            fn __ext(
                __this: *mut ACE_Mem_Map,
                __a0: libc::c_int,
                __a1: libc::c_ulong,
                __a2: libc::c_int,
                __a3: libc::c_int,
                __a4: *mut libc::c_void,
                __a5: libc::c_long,
                __a6: libc::c_int,
            );
        }
        __ext(__this as *mut ACE_Mem_Map, __a0, __a1, __a2, __a3, __a4, __a5, __a6)
    }
    pub unsafe fn new_u7d7def492f0113c0(
        mut __a0: libc::c_int,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: libc::c_int,
        mut __a4: *mut libc::c_void,
        mut __a5: libc::c_long,
        mut __a6: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u7d7def492f0113c0(
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
    ///Map a file specified by @a file_name.
    pub unsafe fn new_at_u5b69368d350aefd5(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: libc::c_uint,
        mut __a4: libc::c_int,
        mut __a5: libc::c_int,
        mut __a6: *mut libc::c_void,
        mut __a7: libc::c_long,
        mut __a8: libc::c_int,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Mem_MapC1EPKcmijiiPvli"]
            fn __ext(
                __this: *mut ACE_Mem_Map,
                __a0: *const libc::c_char,
                __a1: libc::c_ulong,
                __a2: libc::c_int,
                __a3: libc::c_uint,
                __a4: libc::c_int,
                __a5: libc::c_int,
                __a6: *mut libc::c_void,
                __a7: libc::c_long,
                __a8: libc::c_int,
            );
        }
        __ext(
            __this as *mut ACE_Mem_Map,
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
    pub unsafe fn new_u5b69368d350aefd5(
        mut __a0: *const libc::c_char,
        mut __a1: libc::c_ulong,
        mut __a2: libc::c_int,
        mut __a3: libc::c_uint,
        mut __a4: libc::c_int,
        mut __a5: libc::c_int,
        mut __a6: *mut libc::c_void,
        mut __a7: libc::c_long,
        mut __a8: libc::c_int,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u5b69368d350aefd5(
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
    /**Map a file from an open file descriptor @a handle.  This function
  /// will lookup the length of the file if it is not given.*/
    pub unsafe fn map(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut length: libc::c_ulong,
        mut prot: libc::c_int,
        mut share: libc::c_int,
        mut addr: *mut libc::c_void,
        mut offset: libc::c_long,
        mut sa: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Mem_Map>::map_it(
                    (__this) as *mut ACE_Mem_Map,
                    handle,
                    length,
                    prot,
                    share,
                    addr,
                    offset,
                    sa,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Remap the file associated with <handle_>.
    pub unsafe fn map_ud24c6c3b9d9e1386(
        __this: *mut Self,
        mut length: libc::c_ulong,
        mut prot: libc::c_int,
        mut share: libc::c_int,
        mut addr: *mut libc::c_void,
        mut offset: libc::c_long,
        mut sa: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((((((((((addr).is_null()) as libc::c_int) as libc::c_int) != 0)
                    && ((((!((*__this).base_addr_).is_null()) as libc::c_int)
                        as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                    && (((((((*__this).base_addr_) as *const u8))
                        != ((((((-((1) as libc::c_int)) as *mut libc::c_void)))
                            as *const u8))) as libc::c_int as libc::c_int) != 0))
                    as libc::c_int) as libc::c_int) != 0)
                {
                    {
                        share = (((share) as libc::c_int)) | ((16) as libc::c_int);
                        share
                    };
                    addr = (*__this).base_addr_;
                }
                return <ACE_Mem_Map>::map_it(
                    (__this) as *mut ACE_Mem_Map,
                    <ACE_Mem_Map>::handle((__this) as *const ACE_Mem_Map),
                    length,
                    prot,
                    share,
                    addr,
                    offset,
                    sa,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Map a file specified by @a filename.
    pub unsafe fn map_uf8d94af3fe7dec57(
        __this: *mut Self,
        mut filename: *const libc::c_char,
        mut length: libc::c_ulong,
        mut flags: libc::c_int,
        mut mode: libc::c_uint,
        mut prot: libc::c_int,
        mut share: libc::c_int,
        mut addr: *mut libc::c_void,
        mut offset: libc::c_long,
        mut sa: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Mem_Map3mapEPKcmijiiPvli"]
            fn __ext(
                __this: *mut ACE_Mem_Map,
                filename: *const libc::c_char,
                length: libc::c_ulong,
                flags: libc::c_int,
                mode: libc::c_uint,
                prot: libc::c_int,
                share: libc::c_int,
                addr: *mut libc::c_void,
                offset: libc::c_long,
                sa: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(
            __this as *mut ACE_Mem_Map,
            filename,
            length,
            flags,
            mode,
            prot,
            share,
            addr,
            offset,
            sa,
        )
    }
    ///Open the file without mapping it.
    pub unsafe fn open(
        __this: *mut Self,
        mut filename: *const libc::c_char,
        mut flags: libc::c_int,
        mut perms: libc::c_uint,
        mut sa: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Mem_Map4openEPKciji"]
            fn __ext(
                __this: *mut ACE_Mem_Map,
                filename: *const libc::c_char,
                flags: libc::c_int,
                perms: libc::c_uint,
                sa: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Mem_Map, filename, flags, perms, sa)
    }
    ///Close down the <handle_> if necessary and unmap the mapping.
    pub unsafe fn close(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Mem_Map5closeEv"]
            fn __ext(__this: *mut ACE_Mem_Map) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Mem_Map)
    }
    ///Close down the <handle_> if necessary.
    pub unsafe fn close_handle(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut result: libc::c_int = 0;
                if (((*__this).close_handle_ as libc::c_int) != 0) {
                    (*__this).close_handle_ = false;
                    result = ACE_OS::close_u07c1eb6343f8c667((*__this).handle_);
                    (*__this).handle_ = (((-((1) as libc::c_int))) as libc::c_int);
                }
                return result;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Close down the internal <file_mapping_> if necessary.  This is\n   * mostly necessary on Win32, which has a different handle for\n   * file-mapping kernel object."]
    pub unsafe fn close_filemapping_handle(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut result: libc::c_int = 0;
                if ((((((((((*__this).file_mapping_ as libc::c_int))
                    != ((((*__this).handle_) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                    && ((((((*__this).file_mapping_ as libc::c_int))
                        != ((((-((1) as libc::c_int))) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)) as libc::c_int) as libc::c_int) != 0)
                {
                    result = ACE_OS::close_u07c1eb6343f8c667((*__this).file_mapping_);
                    (*__this).file_mapping_ = (((-((1) as libc::c_int))) as libc::c_int);
                }
                return result;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**This operator passes back the starting address of the mapped
  /// file.*/
    pub unsafe fn operator_call(
        __this: *mut Self,
        mut addr: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((((*__this).base_addr_) as *const u8))
                    == ((((((-((1) as libc::c_int)) as *mut libc::c_void)))
                        as *const u8))) as libc::c_int as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                } else {
                    (*addr) = (*__this).base_addr_;
                    return 0;
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the base address.
    pub unsafe fn addr(__this: *const Self) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).base_addr_) as *mut libc::c_void);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**This function returns the number of bytes currently mapped in the
  /// file.*/
    pub unsafe fn size(__this: *const Self) -> libc::c_ulong {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).length_) as libc::c_ulong);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Unmap the region starting at base_addr_.
    pub unsafe fn unmap(__this: *mut Self, mut len: libc::c_long) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Mem_Map>::close_filemapping_handle((__this) as *mut ACE_Mem_Map);
                if (((((((*__this).base_addr_) as *const u8))
                    != ((((((-((1) as libc::c_int)) as *mut libc::c_void)))
                        as *const u8))) as libc::c_int as libc::c_int) != 0)
                {
                    let mut result: libc::c_int = ((ACE_OS::munmap_u78403a470c5b78d3(
                        (*__this).base_addr_,
                        if (((((len as libc::c_long)) < (((0) as libc::c_long)))
                            as libc::c_int as libc::c_int) != 0)
                        {
                            (*__this).length_
                        } else {
                            (len as libc::c_ulong)
                        },
                    )) as libc::c_int);
                    (*__this).base_addr_ = (((-((1) as libc::c_int))
                        as *mut libc::c_void));
                    return ((result) as libc::c_int);
                } else {
                    return 0;
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Unmap the region starting at addr_.
    pub unsafe fn unmap_ubfb9bcdd85d528e8(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_long,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Mem_Map>::close_filemapping_handle((__this) as *mut ACE_Mem_Map);
                return ACE_OS::munmap_u78403a470c5b78d3(
                    addr,
                    if (((((len as libc::c_long)) < (((0) as libc::c_long)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        (*__this).length_
                    } else {
                        (len as libc::c_ulong)
                    },
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Sync @a len bytes of the memory region to the backing store\n   * starting at base_addr_.  If @a len == -1 then sync the whole\n   * region."]
    pub unsafe fn sync(
        __this: *mut Self,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::msync_u3075bc10fec907f5((*__this).base_addr_, len, flags);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Sync the whole memory region to the backing store\n   * starting at base_addr_."]
    pub unsafe fn sync_u7597fdff378318bc(
        __this: *mut Self,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::msync_u3075bc10fec907f5(
                    (*__this).base_addr_,
                    (*__this).length_,
                    flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Sync @a len bytes of the memory region to the backing store
  /// starting at addr_.*/
    pub unsafe fn sync_u404345fa41e63050(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::msync_u3075bc10fec907f5(addr, len, flags);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Change the protection of the pages of the mapped region to @a prot\n   * starting at base_addr_ up to @a len bytes."]
    pub unsafe fn protect(
        __this: *mut Self,
        mut len: libc::c_ulong,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::mprotect_ufb2560969f20c857(
                    (*__this).base_addr_,
                    len,
                    prot,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Change the protection of all the pages of the mapped region to @a prot\n   * starting at base_addr_."]
    pub unsafe fn protect_u4f10c36ea913e5f8(
        __this: *mut Self,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::mprotect_ufb2560969f20c857(
                    (*__this).base_addr_,
                    (*__this).length_,
                    prot,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Change the protection of the pages of the mapped region to @a prot
  /// starting at @a addr up to @a len bytes.*/
    pub unsafe fn protect_uce55d247cf925fa4(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ACE_OS::mprotect_ufb2560969f20c857(addr, len, prot);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Close and remove the file from the file system.
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Mem_Map6removeEv"]
            fn __ext(__this: *mut ACE_Mem_Map) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Mem_Map)
    }
    ///Hook into the underlying VM system.
    pub unsafe fn advise(
        __this: *mut Self,
        mut behavior: libc::c_int,
        mut len: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut advise_len: libc::c_ulong = ((if (((((len as libc::c_int))
                    < (((0) as libc::c_int))) as libc::c_int as libc::c_int) != 0)
                {
                    (*__this).length_
                } else {
                    (len as libc::c_ulong)
                }) as libc::c_ulong);
                return ACE_OS::madvise_udf79f790df65d77b(
                    ((*__this).base_addr_ as *mut libc::c_char),
                    ((advise_len) as libc::c_ulong),
                    behavior,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the underlying <handle_>.
    pub unsafe fn handle(__this: *const Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).handle_) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Return the name of file that is mapped (if any).
    pub unsafe fn filename(__this: *const Self) -> *const libc::c_char {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return (((*__this).filename_).as_ptr() as *const libc::c_char);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK11ACE_Mem_Map4dumpEv"]
            fn __ext(__this: *const ACE_Mem_Map);
        }
        __ext(__this as *const ACE_Mem_Map)
    }
    /**This method does the dirty work of actually calling ::mmap to map
  /// the file into memory.*/
    pub unsafe fn map_it(
        __this: *mut Self,
        mut handle: libc::c_int,
        mut len: libc::c_ulong,
        mut prot: libc::c_int,
        mut share: libc::c_int,
        mut addr: *mut libc::c_void,
        mut offset: libc::c_long,
        mut sa: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN11ACE_Mem_Map6map_itEimiiPvli"]
            fn __ext(
                __this: *mut ACE_Mem_Map,
                handle: libc::c_int,
                len: libc::c_ulong,
                prot: libc::c_int,
                share: libc::c_int,
                addr: *mut libc::c_void,
                offset: libc::c_long,
                sa: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Mem_Map, handle, len, prot, share, addr, offset, sa)
    }
}
impl ACE_MMAP_Memory_Pool {
    ///Initialize the pool.
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *const ACE_MMAP_Memory_Pool_Options,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_PoolC1EPKcPK28ACE_MMAP_Memory_Pool_Options"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                __a0: *const libc::c_char,
                __a1: *const ACE_MMAP_Memory_Pool_Options,
            );
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: *const ACE_MMAP_Memory_Pool_Options,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Ask system for initial chunk of shared memory.
    pub unsafe fn init_acquire(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
        mut rounded_bytes: *mut libc::c_ulong,
        mut first_time: *mut libc::c_int,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool12init_acquireEmRmRi"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                nbytes: libc::c_ulong,
                rounded_bytes: *mut libc::c_ulong,
                first_time: *mut libc::c_int,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, nbytes, rounded_bytes, first_time)
    }
    #[doc = "* Acquire at least @a nbytes from the memory pool. @a rounded_bytes\n   * is the actual number of bytes allocated.  Also acquires an\n   * internal semaphore that ensures proper serialization of\n   * ACE_MMAP_Memory_Pool initialization across processes."]
    pub unsafe fn acquire(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
        mut rounded_bytes: *mut libc::c_ulong,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool7acquireEmRm"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                nbytes: libc::c_ulong,
                rounded_bytes: *mut libc::c_ulong,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, nbytes, rounded_bytes)
    }
    ///Instruct the memory pool to release all of its resources.
    pub unsafe fn release(__this: *mut Self, mut destroy: libc::c_int) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool7releaseEi"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                destroy: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, destroy)
    }
    /**Sync the memory region to the backing store starting at
  /// @c this->base_addr_.*/
    pub unsafe fn sync(
        __this: *mut Self,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool4syncEmi"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                len: libc::c_ulong,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, len, flags)
    }
    /**Sync the memory region to the backing store starting at
  /// @c this->base_addr_.  Will sync as much as the backing file
  /// allows.*/
    pub unsafe fn sync_ua77e6bd2e6007bc6(
        __this: *mut Self,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool4syncEi"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, flags)
    }
    ///Sync the memory region to the backing store starting at @a addr.
    pub unsafe fn sync_u8f2f28f4f83a3cf6(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool4syncEPvmi"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                addr: *mut libc::c_void,
                len: libc::c_ulong,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, addr, len, flags)
    }
    #[doc = "* Change the protection of the pages of the mapped region to @a prot\n   * starting at @c this->base_addr_ up to @a len bytes.  If @a len == -1\n   * then change protection of all pages in the mapped region."]
    pub unsafe fn protect(
        __this: *mut Self,
        mut len: libc::c_ulong,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool7protectEmi"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                len: libc::c_ulong,
                prot: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, len, prot)
    }
    #[doc = "* Change the protection of all the pages of the mapped region to @a prot\n   * starting at @c this->base_addr_."]
    pub unsafe fn protect_u540e04f76dbb68be(
        __this: *mut Self,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool7protectEi"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                prot: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, prot)
    }
    /**Change the protection of the pages of the mapped region to @a prot
  /// starting at @a addr up to @a len bytes.*/
    pub unsafe fn protect_u1b7d5a46d0b852fe(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool7protectEPvmi"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                addr: *mut libc::c_void,
                len: libc::c_ulong,
                prot: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, addr, len, prot)
    }
    #[doc = "* Try to extend the virtual address space so that @a addr is now\n   * covered by the address mapping.  The method succeeds and returns\n   * 0 if the backing store has adequate memory to cover this address.\n   * Otherwise, it returns -1.  This method is typically called by a\n   * UNIX signal handler for SIGSEGV or a Win32 structured exception\n   * when another process has grown the backing store (and its\n   * mapping) and our process now incurs a fault because our mapping\n   * isn't in range (yet)."]
    pub unsafe fn remap(__this: *mut Self, mut addr: *mut libc::c_void) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool5remapEPv"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                addr: *mut libc::c_void,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, addr)
    }
    ///Return the base address of this memory pool.
    pub unsafe fn base_addr(__this: *const Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZNK20ACE_MMAP_Memory_Pool9base_addrEv"]
            fn __ext(__this: *const ACE_MMAP_Memory_Pool) -> *mut libc::c_void;
        }
        __ext(__this as *const ACE_MMAP_Memory_Pool)
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK20ACE_MMAP_Memory_Pool4dumpEv"]
            fn __ext(__this: *const ACE_MMAP_Memory_Pool);
        }
        __ext(__this as *const ACE_MMAP_Memory_Pool)
    }
    ///Get reference to underlying ACE_Mem_Map object.
    pub unsafe fn mmap(__this: *const Self) -> *const ACE_Mem_Map {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of!(
                    (* ::core::ptr::addr_of!((* __this).mmap_) .cast:: < ACE_Mem_Map > ()
                    .cast_mut())
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get reference to underlying ACE_Mem_Map object.
    pub unsafe fn mmap_ucf3123bba782f2ea(__this: *mut Self) -> *mut ACE_Mem_Map {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return ::core::ptr::addr_of_mut!(
                    (* ::core::ptr::addr_of!((* __this).mmap_) .cast:: < ACE_Mem_Map > ()
                    .cast_mut())
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Implement the algorithm for rounding up the request to an
  /// appropriate chunksize.*/
    pub unsafe fn round_up(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
    ) -> libc::c_ulong {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool8round_upEm"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                nbytes: libc::c_ulong,
            ) -> libc::c_ulong;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, nbytes)
    }
    /**Compute the new @a map_size of the backing store and commit the
  /// memory.*/
    pub unsafe fn commit_backing_store_name(
        __this: *mut Self,
        mut rounded_bytes: libc::c_ulong,
        mut map_size: *mut libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool25commit_backing_store_nameEmRm"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                rounded_bytes: libc::c_ulong,
                map_size: *mut libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, rounded_bytes, map_size)
    }
    ///Memory map the file up to @a map_size bytes.
    pub unsafe fn map_file(
        __this: *mut Self,
        mut map_size: libc::c_ulong,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool8map_fileEm"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                map_size: libc::c_ulong,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, map_size)
    }
    #[doc = "* Handle SIGSEGV and SIGBUS signals to remap memory properly.  When a\n   * process reads or writes to non-mapped memory a signal (SIGBUS or\n   * SIGSEGV) will be triggered.  At that point, the ACE_Sig_Handler\n   * (which is part of the ACE_Reactor) will catch the signal and\n   * dispatch the handle_signal() method defined here.  If the SIGSEGV\n   * signal occurred due to the fact that the mapping wasn't up to date\n   * with respect to the backing store, the handler method below will\n   * update the mapping accordingly. When the signal handler returns,\n   * the instruction should be restarted and the operation should work."]
    pub unsafe fn handle_signal(
        __this: *mut Self,
        mut signum: libc::c_int,
        mut _anon_1: *mut siginfo_t,
        mut _anon_2: *mut ucontext_t,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN20ACE_MMAP_Memory_Pool13handle_signalEiP9siginfo_tP10ucontext_t"]
            fn __ext(
                __this: *mut ACE_MMAP_Memory_Pool,
                signum: libc::c_int,
                _anon_1: *mut siginfo_t,
                _anon_2: *mut ucontext_t,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_MMAP_Memory_Pool, signum, _anon_1, _anon_2)
    }
}
impl ACE_Lite_MMAP_Memory_Pool {
    ///Initialize the pool.
    pub unsafe fn new_at(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *const ACE_MMAP_Memory_Pool_Options,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Lite_MMAP_Memory_PoolC1EPKcPK28ACE_MMAP_Memory_Pool_Options"]
            fn __ext(
                __this: *mut ACE_Lite_MMAP_Memory_Pool,
                __a0: *const libc::c_char,
                __a1: *const ACE_MMAP_Memory_Pool_Options,
            );
        }
        __ext(__this as *mut ACE_Lite_MMAP_Memory_Pool, __a0, __a1)
    }
    pub unsafe fn new(
        mut __a0: *const libc::c_char,
        mut __a1: *const ACE_MMAP_Memory_Pool_Options,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    ///Overwrite the default sync behavior with no-op
    pub unsafe fn sync(
        __this: *mut Self,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Lite_MMAP_Memory_Pool4syncEmi"]
            fn __ext(
                __this: *mut ACE_Lite_MMAP_Memory_Pool,
                len: libc::c_ulong,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Lite_MMAP_Memory_Pool, len, flags)
    }
    ///Overwrite the default sync behavior with no-op
    pub unsafe fn sync_u5c8be02cdb2a7232(
        __this: *mut Self,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Lite_MMAP_Memory_Pool4syncEi"]
            fn __ext(
                __this: *mut ACE_Lite_MMAP_Memory_Pool,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Lite_MMAP_Memory_Pool, flags)
    }
    ///Overwrite the default sync behavior with no-op
    pub unsafe fn sync_ub3caaa2d6b80b7fa(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Lite_MMAP_Memory_Pool4syncEPvmi"]
            fn __ext(
                __this: *mut ACE_Lite_MMAP_Memory_Pool,
                addr: *mut libc::c_void,
                len: libc::c_ulong,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Lite_MMAP_Memory_Pool, addr, len, flags)
    }
}
impl ACE_System_Time {
    ///Default constructor.
    pub unsafe fn new_at(__this: *mut Self, mut poolname: *const libc::c_char) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).shmem_),
                ((0)
                    as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).delta_time_),
                ((0) as *mut libc::c_long),
            );
            {
                if (((((poolname).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    if (((((ACE::get_temp_dir(
                        (((*__this).poolname_).as_mut_ptr() as *mut libc::c_char),
                        (((((4096) as libc::c_int)).wrapping_sub((17) as libc::c_int))
                            as libc::c_ulong),
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
                                            ((b"/build/ace_full/src/ACE/ace/System_Time.cpp\0".as_ptr()
                                                as *const libc::c_char) as *const libc::c_char),
                                            36,
                                            (-((1) as libc::c_int)),
                                            ((__ace_error) as libc::c_int),
                                        );
                                        <ACE_Log_Category_TSS>::log(
                                            (ace___) as *mut ACE_Log_Category_TSS,
                                            LM_ERROR,
                                            ((b"Temporary path too long, defaulting to current directory\n\0"
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
                        (*((((*__this).poolname_).as_ptr() as *mut libc::c_char)
                            .wrapping_offset((0) as isize))) = ((0) as libc::c_char);
                    }
                    ACE_OS::strcat_u59e0931d4af5fb3f(
                        (((*__this).poolname_).as_mut_ptr() as *mut libc::c_char),
                        ((b"ace-malloc-XXXXXX\0".as_ptr() as *const libc::c_char)
                            as *const libc::c_char),
                    );
                } else {
                    ACE_OS::strsncpy(
                        (((*__this).poolname_).as_mut_ptr() as *mut libc::c_char),
                        ((poolname) as *const libc::c_char),
                        ((((((::core::mem::size_of::<[libc::c_char; 4097usize]>()
                            as libc::c_ulong) as libc::c_ulong))
                            / ((1) as libc::c_ulong))) as libc::c_ulong),
                    );
                }
                'dowhile_1: loop {
                    'cont_1: loop {
                        {
                            {
                                (*__this).shmem_ = {
                                    let __new: *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ = Box::into_raw(
                                        Box::new(
                                            ::core::mem::MaybeUninit::<
                                                ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
                                            >::zeroed()
                                                .assume_init(),
                                        ),
                                    );
                                    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::new_at_s039880e4b7dd6368(
                                        (__new)
                                            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
                                        (((*__this).poolname_).as_ptr() as *const libc::c_char),
                                    );
                                    __new
                                };
                                if ((((((*__this).shmem_).is_null()) as libc::c_int)
                                    as libc::c_int) != 0)
                                {
                                    ((*(__errno_location()))) = 12;
                                    return;
                                }
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_1;
                    }
                    if !(((0) != 0)) {
                        break 'dowhile_1;
                    }
                }
            }
            ()
        }
    }
    pub unsafe fn new(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    /**Get the local system time, i.e., the value returned by
  /// ACE_OS::time().*/
    pub unsafe fn get_local_system_time(mut time_out: *mut libc::c_long) -> libc::c_int {
        unsafe {
            {
                (*time_out) = ACE_OS::time_udaedd75f32971343(((0) as *mut libc::c_long));
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Get the local system time, i.e., the value returned by
  /// ACE_OS::time().*/
    pub unsafe fn get_local_system_time_ue35a4077f4b76f21(
        mut time_out: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            {
                <ACE_Time_Value>::set(
                    (::core::ptr::addr_of_mut!((* time_out))) as *mut ACE_Time_Value,
                    ACE_OS::time_udaedd75f32971343(((0) as *mut libc::c_long)),
                    ((0) as libc::c_long),
                );
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the system time of the central time server.
    pub unsafe fn get_master_system_time(
        __this: *mut Self,
        mut time_out: *mut libc::c_long,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).delta_time_).is_null()) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    let mut temp: *mut libc::c_void = ((0) as *mut libc::c_void);
                    if ((((({
                        let __obj: *mut ACE_Allocator = ((*__this).shmem_)
                            as *mut ACE_Allocator;
                        let __vt: *const __Vtbl_uf2113694993e252c = *(__obj
                            as *const *const __Vtbl_uf2113694993e252c);
                        ((*__vt)
                            .vfn_u41a4de2216226892)(
                            __obj,
                            ((b"ACE_TS_TIME\0".as_ptr() as *const libc::c_char)
                                as *const libc::c_char),
                            ::core::ptr::addr_of_mut!(temp),
                        )
                    } as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        return {
                            let _ = __this;
                            <ACE_System_Time>::get_local_system_time(
                                ::core::ptr::addr_of_mut!((* time_out)),
                            )
                        };
                    } else {
                        (*__this).delta_time_ = (temp as *mut libc::c_long);
                    }
                }
                let mut local_time: libc::c_long = unsafe { ::core::mem::zeroed() };
                if ((((((*((*__this).delta_time_)) as libc::c_long))
                    >= (((0) as libc::c_long))) as libc::c_int as libc::c_int) != 0)
                {
                    {
                        let _ = __this;
                        <ACE_System_Time>::get_local_system_time(
                            ::core::ptr::addr_of_mut!(local_time),
                        )
                    };
                    (*time_out) = (((local_time) as libc::c_long))
                        .wrapping_add(
                            (((*((*__this).delta_time_)) as libc::c_uint))
                                as libc::c_long,
                        );
                } else {
                    (*time_out) = (((*((((*__this).delta_time_)
                        .wrapping_offset((1) as isize))))) as libc::c_long);
                }
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Get the system time of the central time server.
    pub unsafe fn get_master_system_time_u6e625afa58325265(
        __this: *mut Self,
        mut time_out: *mut ACE_Time_Value,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut to: libc::c_long = unsafe { ::core::mem::zeroed() };
                if (((((<ACE_System_Time>::get_master_system_time(
                    (__this) as *mut ACE_System_Time,
                    ::core::ptr::addr_of_mut!(to),
                ) as libc::c_int)) == ((((-((1) as libc::c_int))) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                <ACE_Time_Value>::sec_u74028f0493c718a1(
                    (::core::ptr::addr_of_mut!((* time_out))) as *mut ACE_Time_Value,
                    to,
                );
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Synchronize local system time with the central time server using
  /// specified mode.*/
    pub unsafe fn sync_local_system_time(
        __this: *mut Self,
        mut _anon_0: libc::c_uint,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                'dowhile_0: loop {
                    'cont_0: loop {
                        {
                            {
                                ((*(__errno_location()))) = 95;
                                return (-((1) as libc::c_int));
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                    if !(((0) != 0)) {
                        break 'dowhile_0;
                    }
                }
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
impl ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ {
    #[doc = "* Initialize ACE_Malloc.  This constructor passes @a pool_name to\n   * initialize the memory pool, and uses ACE::basename() to\n   * automatically extract out the name used for the underlying lock\n   * name (if necessary).  Note that @a pool_name should be located in\n   * a directory with the appropriate visibility and protection so\n   * that all processes that need to access it can do so."]
    pub unsafe fn new_at_sd318f86520b5d207(
        __this: *mut Self,
        mut pool_name: *const libc::c_char,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::new_at_s64e2e5cf64745a9b(
                (::core::ptr::addr_of_mut!((* __this).__base_0)
                    .cast::<
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    >())
                    as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                pool_name,
            );
            {}
            ()
        }
    }
    pub unsafe fn new_sd318f86520b5d207(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sd318f86520b5d207(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
}
impl ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ {
    #[doc = "* Note that @a pool_name should be located in\n   * a directory with the appropriate visibility and protection so\n   * that all processes that need to access it can do so."]
    pub unsafe fn new_at_s039880e4b7dd6368(
        __this: *mut Self,
        mut pool_name: *const libc::c_char,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Allocator>::new_at(
                ::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_Allocator>(),
            );
            *(__this as *mut *const ()) = &__VTBL_u36615f686fb5613d
                as *const __Vtbl_u36615f686fb5613d as *const ();
            <ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_>::new_at_sd318f86520b5d207(
                (::core::ptr::addr_of_mut!((* __this).allocator_)
                    .cast::<ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_>())
                    as *mut ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_,
                ((pool_name) as *const libc::c_char),
            );
            {}
            ()
        }
    }
    pub unsafe fn new_s039880e4b7dd6368(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s039880e4b7dd6368(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    /**Locate @a name and pass out parameter via pointer.  If found,
  /// return 0, returns -1 if @a name isn't found.*/
    pub unsafe fn find_sd1d97c7eb0448d41(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::find_sb79c8f0521a6ddd8(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                    ::core::ptr::addr_of_mut!((* pointer)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Associate @a name with @a pointer.  If @a duplicates == 0 then do\n   * not allow duplicate @a name/pointer associations, else if\n   * @a duplicates != 0 then allow duplicate @a name/pointer\n   * associations.  Returns 0 if successfully binds (1) a previously\n   * unbound @a name or (2) @a duplicates != 0, returns 1 if trying to\n   * bind a previously bound @a name and @a duplicates == 0, else\n   * returns -1 if a resource failure occurs."]
    pub unsafe fn bind(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut libc::c_void,
        mut duplicates: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::bind(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                    pointer,
                    duplicates,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Allocate @a nbytes, giving them all an @a initial_value.
    pub unsafe fn calloc_s1ae7c325fc123850(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
        mut initial_value: libc::c_char,
    ) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::calloc_s6ac7ac984caf1001(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    nbytes,
                    initial_value,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Allocate @a n_elem each of size @a elem_size, giving them
  /// @a initial_value.*/
    pub unsafe fn calloc_s20377fc4621028db(
        __this: *mut Self,
        mut n_elem: libc::c_ulong,
        mut elem_size: libc::c_ulong,
        mut initial_value: libc::c_char,
    ) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::calloc_s7fc4b13687d88c4a(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    n_elem,
                    elem_size,
                    initial_value,
                );
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
    ///Returns 0 if the name is in the mapping and -1 if not.
    pub unsafe fn find_se7e55738a6d934ec(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::find_s07b337cff4ad6a1f(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Free @a ptr (must have been allocated by ACE_Allocator::malloc()).
    pub unsafe fn free(__this: *mut Self, mut ptr: *mut libc::c_void) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::free(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    ptr,
                );
            }
            ()
        }
    }
    ///Allocate @a nbytes, but don't give them any initial value.
    pub unsafe fn malloc(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
    ) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::malloc(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    nbytes,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Change the protection of the pages of the mapped region to @a prot\n   * starting at @c this->base_addr_ up to @a len bytes.  If @a len == -1\n   * then change protection of all pages in the mapped region."]
    pub unsafe fn protect_s48b3af97b45c173e(
        __this: *mut Self,
        mut len: libc::c_long,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::protect_s3cfcbcf4455cdc93(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    len,
                    flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Change the protection of the pages of the mapped region to @a prot
  /// starting at @a addr up to @a len bytes.*/
    pub unsafe fn protect_sdf95393c41ed2b14(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::protect_scc29d13ed7aaf5d9(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    addr,
                    len,
                    flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Remove any resources associated with this memory manager.
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::remove(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Sync @a len bytes of the memory region to the backing store\n   * starting at @c this->base_addr_.  If @a len == -1 then sync the\n   * whole region."]
    pub unsafe fn sync_s48b3af97b45c173e(
        __this: *mut Self,
        mut len: libc::c_long,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::sync_s3cfcbcf4455cdc93(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    len,
                    flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Sync @a len bytes of the memory region to the backing store
  /// starting at @c addr_.*/
    pub unsafe fn sync_sdf95393c41ed2b14(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::sync_scc29d13ed7aaf5d9(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    addr,
                    len,
                    flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Associate @a name with @a pointer.  Does not allow duplicate\n   * name/pointer associations.  Returns 0 if successfully binds\n   * (1) a previously unbound @a name, 1 if trying to bind a previously\n   * bound @a name, or returns -1 if a resource failure occurs.  When\n   * this call returns, @a pointer's value will always reference the\n   * void * that @a name is associated with.  Thus, if the caller needs\n   * to use @a pointer (e.g., to free it) a copy must be maintained by\n   * the caller."]
    pub unsafe fn trybind(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::trybind(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                    ::core::ptr::addr_of_mut!((* pointer)),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Unbind (remove) the name from the map.  Don't return the pointer
  /// to the caller*/
    pub unsafe fn unbind_se7e55738a6d934ec(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::unbind_s07b337cff4ad6a1f(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Break any association of name.  Returns the value of pointer in
  /// case the caller needs to deallocate memory.*/
    pub unsafe fn unbind_sd1d97c7eb0448d41(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::unbind_sb79c8f0521a6ddd8(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* (::core::ptr::addr_of_mut!((*
                        ::core::ptr::addr_of!((* __this).allocator_) .cast:: <
                        ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex_ > ()
                        .cast_mut())))).__base_0) .cast:: <
                        ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_
                        > ().cast_mut())
                    ))
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                    ::core::ptr::addr_of_mut!((* pointer)),
                );
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
impl ACE_New_Allocator {
    ///These methods are defined.
    pub unsafe fn malloc(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator6mallocEm"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                nbytes: libc::c_ulong,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_New_Allocator, nbytes)
    }
    pub unsafe fn calloc(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
        mut initial_value: libc::c_char,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator6callocEmc"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                nbytes: libc::c_ulong,
                initial_value: libc::c_char,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_New_Allocator, nbytes, initial_value)
    }
    pub unsafe fn calloc_ub14cd9e591a2d69d(
        __this: *mut Self,
        mut n_elem: libc::c_ulong,
        mut elem_size: libc::c_ulong,
        mut initial_value: libc::c_char,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator6callocEmmc"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                n_elem: libc::c_ulong,
                elem_size: libc::c_ulong,
                initial_value: libc::c_char,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_New_Allocator, n_elem, elem_size, initial_value)
    }
    pub unsafe fn free(__this: *mut Self, mut ptr: *mut libc::c_void) {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator4freeEPv"]
            fn __ext(__this: *mut ACE_New_Allocator, ptr: *mut libc::c_void);
        }
        __ext(__this as *mut ACE_New_Allocator, ptr)
    }
    ///These methods are no-ops.
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator6removeEv"]
            fn __ext(__this: *mut ACE_New_Allocator) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator)
    }
    pub unsafe fn bind(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut libc::c_void,
        mut duplicates: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator4bindEPKcPvi"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                name: *const libc::c_char,
                pointer: *mut libc::c_void,
                duplicates: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, name, pointer, duplicates)
    }
    pub unsafe fn trybind(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator7trybindEPKcRPv"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                name: *const libc::c_char,
                pointer: *mut *mut libc::c_void,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, name, pointer)
    }
    pub unsafe fn find(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator4findEPKcRPv"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                name: *const libc::c_char,
                pointer: *mut *mut libc::c_void,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, name, pointer)
    }
    pub unsafe fn find_ud72abe9cd5ce064f(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator4findEPKc"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, name)
    }
    pub unsafe fn unbind(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator6unbindEPKc"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, name)
    }
    pub unsafe fn unbind_u9886c474d772c149(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator6unbindEPKcRPv"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                name: *const libc::c_char,
                pointer: *mut *mut libc::c_void,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, name, pointer)
    }
    pub unsafe fn sync(
        __this: *mut Self,
        mut len: libc::c_long,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator4syncEli"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                len: libc::c_long,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, len, flags)
    }
    pub unsafe fn sync_u882390cc14c47b90(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator4syncEPvmi"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                addr: *mut libc::c_void,
                len: libc::c_ulong,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, addr, len, flags)
    }
    pub unsafe fn protect(
        __this: *mut Self,
        mut len: libc::c_long,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator7protectEli"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                len: libc::c_long,
                prot: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, len, prot)
    }
    pub unsafe fn protect_ucd829d73a2707ee4(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN17ACE_New_Allocator7protectEPvmi"]
            fn __ext(
                __this: *mut ACE_New_Allocator,
                addr: *mut libc::c_void,
                len: libc::c_ulong,
                prot: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_New_Allocator, addr, len, prot)
    }
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK17ACE_New_Allocator4dumpEv"]
            fn __ext(__this: *const ACE_New_Allocator);
        }
        __ext(__this as *const ACE_New_Allocator)
    }
}
impl ACE_Static_Allocator_Base {
    pub unsafe fn new_at(
        __this: *mut Self,
        mut buffer: *mut libc::c_char,
        mut size: libc::c_ulong,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            <ACE_Allocator>::new_at(
                ::core::ptr::addr_of_mut!((* __this).__base_0).cast::<ACE_Allocator>(),
            );
            *(__this as *mut *const ()) = &__VTBL_uee81b96054c3f1a9
                as *const __Vtbl_uee81b96054c3f1a9 as *const ();
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).buffer_), buffer);
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).size_), size);
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).offset_),
                ((0) as libc::c_ulong),
            );
            {}
            ()
        }
    }
    pub unsafe fn new(mut __a0: *mut libc::c_char, mut __a1: libc::c_ulong) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at(::core::ptr::addr_of_mut!(__obj), __a0, __a1);
        __obj
    }
    pub unsafe fn malloc(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base6mallocEm"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                nbytes: libc::c_ulong,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, nbytes)
    }
    pub unsafe fn calloc(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
        mut initial_value: libc::c_char,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base6callocEmc"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                nbytes: libc::c_ulong,
                initial_value: libc::c_char,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, nbytes, initial_value)
    }
    pub unsafe fn calloc_u816b60613ed1d3f9(
        __this: *mut Self,
        mut n_elem: libc::c_ulong,
        mut elem_size: libc::c_ulong,
        mut initial_value: libc::c_char,
    ) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base6callocEmmc"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                n_elem: libc::c_ulong,
                elem_size: libc::c_ulong,
                initial_value: libc::c_char,
            ) -> *mut libc::c_void;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, n_elem, elem_size, initial_value)
    }
    pub unsafe fn free(__this: *mut Self, mut ptr: *mut libc::c_void) {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base4freeEPv"]
            fn __ext(__this: *mut ACE_Static_Allocator_Base, ptr: *mut libc::c_void);
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, ptr)
    }
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base6removeEv"]
            fn __ext(__this: *mut ACE_Static_Allocator_Base) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base)
    }
    pub unsafe fn bind(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut libc::c_void,
        mut duplicates: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base4bindEPKcPvi"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                name: *const libc::c_char,
                pointer: *mut libc::c_void,
                duplicates: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, name, pointer, duplicates)
    }
    pub unsafe fn trybind(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base7trybindEPKcRPv"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                name: *const libc::c_char,
                pointer: *mut *mut libc::c_void,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, name, pointer)
    }
    pub unsafe fn find(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base4findEPKcRPv"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                name: *const libc::c_char,
                pointer: *mut *mut libc::c_void,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, name, pointer)
    }
    pub unsafe fn find_ueb49d4df81e0a60b(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base4findEPKc"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, name)
    }
    pub unsafe fn unbind(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base6unbindEPKc"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                name: *const libc::c_char,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, name)
    }
    pub unsafe fn unbind_ubd49d95e20a7b2cd(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base6unbindEPKcRPv"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                name: *const libc::c_char,
                pointer: *mut *mut libc::c_void,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, name, pointer)
    }
    pub unsafe fn sync(
        __this: *mut Self,
        mut len: libc::c_long,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base4syncEli"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                len: libc::c_long,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, len, flags)
    }
    pub unsafe fn sync_u3c039f9aead40514(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base4syncEPvmi"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                addr: *mut libc::c_void,
                len: libc::c_ulong,
                flags: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, addr, len, flags)
    }
    pub unsafe fn protect(
        __this: *mut Self,
        mut len: libc::c_long,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base7protectEli"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                len: libc::c_long,
                prot: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, len, prot)
    }
    pub unsafe fn protect_u20867242cdfcbb98(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut prot: libc::c_int,
    ) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_Base7protectEPvmi"]
            fn __ext(
                __this: *mut ACE_Static_Allocator_Base,
                addr: *mut libc::c_void,
                len: libc::c_ulong,
                prot: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(__this as *mut ACE_Static_Allocator_Base, addr, len, prot)
    }
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK25ACE_Static_Allocator_Base4dumpEv"]
            fn __ext(__this: *const ACE_Static_Allocator_Base);
        }
        __ext(__this as *const ACE_Static_Allocator_Base)
    }
    ///Don't allow direct instantiations of this class.
    pub unsafe fn new_at_u772cfbf04a27e185(__this: *mut Self) {
        extern "C-unwind" {
            #[link_name = "_ZN25ACE_Static_Allocator_BaseC1Ev"]
            fn __ext(__this: *mut ACE_Static_Allocator_Base);
        }
        __ext(__this as *mut ACE_Static_Allocator_Base)
    }
    pub unsafe fn new_u772cfbf04a27e185() -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_u772cfbf04a27e185(::core::ptr::addr_of_mut!(__obj));
        __obj
    }
}
impl ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_ {
    #[doc = "* Initialize ACE_Malloc.  This constructor passes @a pool_name to\n   * initialize the memory pool, and uses @a lock_name to automatically\n   * extract out the name used for the underlying lock name (if\n   * necessary).  In addition, @a options is passed through to\n   * initialize the underlying memory pool.\n   *\n   * Note that @a pool_name should be located in\n   * a directory with the appropriate visibility and protection so\n   * that all processes that need to access it can do so."]
    pub unsafe fn new_at_sa6f84b6421492009(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
        mut __a2: *const ACE_MMAP_Memory_Pool_Options,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockEC1EPKcS5_PK28ACE_MMAP_Memory_Pool_Options"]
            fn __ext(
                __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                __a0: *const libc::c_char,
                __a1: *const libc::c_char,
                __a2: *const ACE_MMAP_Memory_Pool_Options,
            );
        }
        __ext(
            __this
                as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            __a0,
            __a1,
            __a2,
        )
    }
    pub unsafe fn new_sa6f84b6421492009(
        mut __a0: *const libc::c_char,
        mut __a1: *const libc::c_char,
        mut __a2: *const ACE_MMAP_Memory_Pool_Options,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sa6f84b6421492009(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    #[doc = "* Initialize an ACE_Malloc with an external ACE_LOCK.\n   * This constructor passes @a pool_name and @a options to initialize\n   * the memory pool. @a lock is used as the pool lock, and must be\n   * properly set up and ready for use before being passed to this method."]
    pub unsafe fn new_at_sd8dd94a8768bef96(
        __this: *mut Self,
        mut __a0: *const libc::c_char,
        mut __a1: *const ACE_MMAP_Memory_Pool_Options,
        mut __a2: *mut ACE_Null_Mutex,
    ) {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockEC1EPKcPK28ACE_MMAP_Memory_Pool_OptionsPS1_"]
            fn __ext(
                __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                __a0: *const libc::c_char,
                __a1: *const ACE_MMAP_Memory_Pool_Options,
                __a2: *mut ACE_Null_Mutex,
            );
        }
        __ext(
            __this
                as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            __a0,
            __a1,
            __a2,
        )
    }
    pub unsafe fn new_sd8dd94a8768bef96(
        mut __a0: *const libc::c_char,
        mut __a1: *const ACE_MMAP_Memory_Pool_Options,
        mut __a2: *mut ACE_Null_Mutex,
    ) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_sd8dd94a8768bef96(
            ::core::ptr::addr_of_mut!(__obj),
            __a0,
            __a1,
            __a2,
        );
        __obj
    }
    ///Get Reference counter.
    pub unsafe fn ref_counter(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE11ref_counterEv"]
            fn __ext(
                __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            ) -> libc::c_int;
        }
        __ext(
            __this
                as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
        )
    }
    /**Release ref counter.
  /// @retval 0 Success
  /// @retval -1 Failure due to missing control block
  /// @retval >0 Memory not release because refcount is not zero*/
    pub unsafe fn release(__this: *mut Self, mut close: libc::c_int) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE7releaseEi"]
            fn __ext(
                __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                close: libc::c_int,
            ) -> libc::c_int;
        }
        __ext(
            __this
                as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            close,
        )
    }
    ///Returns a reference to the underlying memory pool.
    pub unsafe fn memory_pool(__this: *mut Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE11memory_poolEv"]
            fn __ext(
                __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            ) -> *mut libc::c_void;
        }
        __ext(
            __this
                as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
        )
    }
    #[doc = "* Returns a count of the number of available chunks that can hold\n   * @a size byte allocations.  Function can be used to determine if you\n   * have reached a water mark. This implies a fixed amount of allocated\n   * memory.\n   *\n   * @param size  The chunk size of that you would like a count of\n   * @return Function returns the number of chunks of the given size\n   *          that would fit in the currently allocated memory."]
    pub unsafe fn avail_chunks(
        __this: *const Self,
        mut size: libc::c_ulong,
    ) -> libc::c_long {
        extern "C-unwind" {
            #[link_name = "_ZNK12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE12avail_chunksEm"]
            fn __ext(
                __this: *const ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                size: libc::c_ulong,
            ) -> libc::c_long;
        }
        __ext(
            __this
                as *const ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            size,
        )
    }
    /**Returns a pointer to the lock used to provide mutual exclusion to
  /// an ACE_Malloc allocator.*/
    pub unsafe fn mutex(__this: *mut Self) -> *mut ACE_Null_Mutex {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE5mutexEv"]
            fn __ext(
                __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            ) -> *mut ACE_Null_Mutex;
        }
        __ext(
            __this
                as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
        )
    }
    ///Dump the state of an object.
    pub unsafe fn dump(__this: *const Self) {
        extern "C-unwind" {
            #[link_name = "_ZNK12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE4dumpEv"]
            fn __ext(
                __this: *const ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            );
        }
        __ext(
            __this
                as *const ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
        )
    }
    ///Return cb_ptr value.
    pub unsafe fn base_addr(__this: *mut Self) -> *mut libc::c_void {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE9base_addrEv"]
            fn __ext(
                __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            ) -> *mut libc::c_void;
        }
        __ext(
            __this
                as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
        )
    }
    #[doc = "* Bad flag.  This operation should be called immediately after the\n   * construction of the Malloc object to query whether the object was\n   * constructed successfully.  If not, the user should invoke @c\n   * remove and release the object (it is not usable.)\n   * @retval 0 if all is fine.  non-zero if this malloc object is\n   *         unuable."]
    pub unsafe fn bad(__this: *mut Self) -> libc::c_int {
        extern "C-unwind" {
            #[link_name = "_ZN12ACE_Malloc_TI20ACE_MMAP_Memory_Pool14ACE_Null_Mutex17ACE_Control_BlockE3badEv"]
            fn __ext(
                __this: *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
            ) -> libc::c_int;
        }
        __ext(
            __this
                as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
        )
    }
    ///Deallocate memory.  Assumes that locks are held by callers.
    pub unsafe fn shared_free(__this: *mut Self, mut ap: *mut libc::c_void) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((((((((ap).is_null()) as libc::c_int) as libc::c_int) != 0)
                    || ((((((*__this).cb_ptr_).is_null()) as libc::c_int) as libc::c_int)
                        != 0)) as libc::c_int) as libc::c_int) != 0)
                {
                    return;
                }
                let mut blockp: *mut ACE_Control_Block_ACE_Malloc_Header = (((ap
                    as *mut ACE_Control_Block_ACE_Malloc_Header)))
                    .wrapping_offset(-((1) as isize));
                let mut currp: *mut ACE_Control_Block_ACE_Malloc_Header = (((*(*__this)
                    .cb_ptr_)
                    .freep_) as *mut ACE_Control_Block_ACE_Malloc_Header);
                if ((1) != 0) {
                    {
                        'for_0: loop {
                            if !(((((((((((blockp) as *const u8))
                                <= (((currp) as *const u8))) as libc::c_int as libc::c_int)
                                != 0)
                                || ((((((blockp) as *const u8))
                                    >= (((((*currp).next_block_
                                        as *mut ACE_Control_Block_ACE_Malloc_Header))
                                        as *const u8))) as libc::c_int as libc::c_int) != 0))
                                as libc::c_int) as libc::c_int) != 0))
                            {
                                break;
                            }
                            'cont_0: loop {
                                {
                                    {
                                        if ((((((((((currp) as *const u8))
                                            >= (((((*currp).next_block_
                                                as *mut ACE_Control_Block_ACE_Malloc_Header))
                                                as *const u8))) as libc::c_int as libc::c_int) != 0)
                                            && (((((((((((blockp) as *const u8))
                                                > (((currp) as *const u8))) as libc::c_int as libc::c_int)
                                                != 0)
                                                || ((((((blockp) as *const u8))
                                                    < (((((*currp).next_block_
                                                        as *mut ACE_Control_Block_ACE_Malloc_Header))
                                                        as *const u8))) as libc::c_int as libc::c_int) != 0))
                                                as libc::c_int)) as libc::c_int) != 0)) as libc::c_int)
                                            as libc::c_int) != 0)
                                        {
                                            break 'for_0;
                                        }
                                    }
                                }
                                #[allow(unreachable_code)] break 'cont_0;
                            }
                            {
                                let __v = (((*currp).next_block_)
                                    as *mut ACE_Control_Block_ACE_Malloc_Header);
                                currp = __v;
                                __v
                            };
                        }
                    }
                    if (((((((blockp).wrapping_offset(((*blockp).size_) as isize))
                        as *const u8))
                        == (((((*currp).next_block_
                            as *mut ACE_Control_Block_ACE_Malloc_Header)) as *const u8)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        {
                            let __lv = ::core::ptr::addr_of_mut!((* blockp).size_);
                            unsafe {
                                *__lv = ((((*__lv)) as libc::c_ulong))
                                    .wrapping_add(
                                        ((*(*currp).next_block_).size_) as libc::c_ulong,
                                    );
                                *__lv
                            }
                        };
                        (*blockp).next_block_ = (*(*currp).next_block_).next_block_;
                    } else {
                        (*blockp).next_block_ = (*currp).next_block_;
                    }
                    if ((((((((currp).wrapping_offset(((*currp).size_) as isize)))
                        as *const u8)) == (((blockp) as *const u8))) as libc::c_int
                        as libc::c_int) != 0)
                    {
                        {
                            let __lv = ::core::ptr::addr_of_mut!((* currp).size_);
                            unsafe {
                                *__lv = ((((*__lv)) as libc::c_ulong))
                                    .wrapping_add(((*blockp).size_) as libc::c_ulong);
                                *__lv
                            }
                        };
                        (*currp).next_block_ = (*blockp).next_block_;
                    } else {
                        (*currp).next_block_ = ((blockp)
                            as *mut ACE_Control_Block_ACE_Malloc_Header);
                    };
                    (*(*__this).cb_ptr_).freep_ = ((currp)
                        as *mut ACE_Control_Block_ACE_Malloc_Header);
                }
                'while_1: loop {
                    if !(((0) != 0)) {
                        break 'while_1;
                    }
                    'cont_1: loop {
                        { {} }
                        #[allow(unreachable_code)] break 'cont_1;
                    }
                }
            }
            ()
        }
    }
    ///Initialize the Malloc pool.
    pub unsafe fn open(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Null_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Null_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Null_Mutex_>::new_at(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Null_Mutex_,
                    (*__this).lock_,
                );
                if (((((<ACE_Guard_ACE_Null_Mutex_>::locked(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Null_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return (-((1) as libc::c_int));
                };
                let mut rounded_bytes: libc::c_ulong = ((0) as libc::c_ulong);
                let mut first_time: libc::c_int = 0;
                (*__this).cb_ptr_ = (<ACE_MMAP_Memory_Pool>::init_acquire(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).memory_pool_) .cast:: <
                        ACE_MMAP_Memory_Pool > ().cast_mut())
                    )) as *mut ACE_MMAP_Memory_Pool,
                    ((::core::mem::size_of::<ACE_Control_Block>() as libc::c_ulong)
                        as libc::c_ulong),
                    ::core::ptr::addr_of_mut!(rounded_bytes),
                    ::core::ptr::addr_of_mut!(first_time),
                ) as *mut ACE_Control_Block);
                if ((((((*__this).cb_ptr_).is_null()) as libc::c_int) as libc::c_int)
                    != 0)
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
                                        return (-((1) as libc::c_int));
                                    }
                                    <ACE_Log_Category_TSS>::conditional_set(
                                        (ace___) as *mut ACE_Log_Category_TSS,
                                        ((b"/build/ace_full/src/ACE/ace/Malloc_T.cpp\0".as_ptr()
                                            as *const libc::c_char) as *const libc::c_char),
                                        449,
                                        (-((1) as libc::c_int)),
                                        ((__ace_error) as libc::c_int),
                                    );
                                    <ACE_Log_Category_TSS>::log(
                                        (ace___) as *mut ACE_Log_Category_TSS,
                                        LM_ERROR,
                                        ((b"(%P|%t) %p\n\0".as_ptr() as *const libc::c_char)
                                            as *const libc::c_char),
                                        b"init_acquire failed\0".as_ptr() as *const libc::c_char,
                                    );
                                    return (-((1) as libc::c_int));
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                        if !(((0) != 0)) {
                            break 'dowhile_0;
                        }
                    }
                } else {
                    if ((first_time) != 0) {
                        <ACE_Control_Block_ACE_Malloc_Header>::init_ptr(
                            ::core::ptr::addr_of_mut!((* (* __this).cb_ptr_).freep_)
                                as *mut *mut ACE_Control_Block_ACE_Malloc_Header,
                            ::core::ptr::addr_of_mut!((* (* __this).cb_ptr_).base_)
                                as *mut ACE_Control_Block_ACE_Malloc_Header,
                            (((*__this).cb_ptr_) as *mut libc::c_void),
                        );
                        <ACE_Control_Block_ACE_Malloc_Header>::init_ptr(
                            ::core::ptr::addr_of_mut!(
                                (* (* (* __this).cb_ptr_).freep_).next_block_
                            ) as *mut *mut ACE_Control_Block_ACE_Malloc_Header,
                            (*(*__this).cb_ptr_).freep_,
                            (((*__this).cb_ptr_) as *mut libc::c_void),
                        );
                        <ACE_Control_Block_ACE_Name_Node>::init_ptr(
                            ::core::ptr::addr_of_mut!((* (* __this).cb_ptr_).name_head_)
                                as *mut *mut ACE_Control_Block_ACE_Name_Node,
                            ((0) as *mut ACE_Control_Block_ACE_Name_Node),
                            (((*__this).cb_ptr_) as *mut libc::c_void),
                        );
                        (*(*(*__this).cb_ptr_).freep_).size_ = ((0) as libc::c_ulong);
                        (*(*__this).cb_ptr_).ref_counter_ = 1;
                        if (((((rounded_bytes as libc::c_ulong))
                            > (((((((::core::mem::size_of::<ACE_Control_Block>()
                                as libc::c_ulong) as libc::c_ulong))
                                .wrapping_add((24) as libc::c_ulong))) as libc::c_ulong)))
                            as libc::c_int as libc::c_int) != 0)
                        {
                            let mut p: *mut ACE_Control_Block_ACE_Malloc_Header = (((((*(*__this)
                                .cb_ptr_)
                                .freep_) as *mut ACE_Control_Block_ACE_Malloc_Header)))
                                .wrapping_offset((1) as isize);
                            <ACE_Control_Block_ACE_Malloc_Header>::init_ptr(
                                ::core::ptr::addr_of_mut!((* p).next_block_)
                                    as *mut *mut ACE_Control_Block_ACE_Malloc_Header,
                                ((0) as *mut ACE_Control_Block_ACE_Malloc_Header),
                                (((*__this).cb_ptr_) as *mut libc::c_void),
                            );
                            (*p).size_ = (((((((((rounded_bytes) as libc::c_ulong))
                                .wrapping_sub(
                                    (::core::mem::size_of::<ACE_Control_Block>()
                                        as libc::c_ulong) as libc::c_ulong,
                                ))) as libc::c_ulong)) / ((24) as libc::c_ulong))
                                as libc::c_ulong);
                            <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_free(
                                (__this)
                                    as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                                (((p).wrapping_offset((1) as isize)) as *mut libc::c_void),
                            );
                        }
                    } else {
                        {
                            let __lv = &mut ((*(*__this).cb_ptr_).ref_counter_);
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
    #[doc = "* Initialize ACE_Malloc.  This constructor passes @a pool_name to\n   * initialize the memory pool, and uses ACE::basename() to\n   * automatically extract out the name used for the underlying lock\n   * name (if necessary).\n   *\n   * Note that @a pool_name should be located in\n   * a directory with the appropriate visibility and protection so\n   * that all processes that need to access it can do so."]
    pub unsafe fn new_at_s64e2e5cf64745a9b(
        __this: *mut Self,
        mut pool_name: *const libc::c_char,
    ) {
        unsafe {
            ::core::ptr::write(
                __this,
                ::core::mem::MaybeUninit::<Self>::zeroed().assume_init(),
            );
            ::core::ptr::write(
                ::core::ptr::addr_of_mut!((* __this).cb_ptr_),
                ((0) as *mut ACE_Control_Block),
            );
            <ACE_MMAP_Memory_Pool>::new_at(
                (::core::ptr::addr_of_mut!((* __this).memory_pool_)
                    .cast::<ACE_MMAP_Memory_Pool>()) as *mut ACE_MMAP_Memory_Pool,
                pool_name,
                ((0) as *const ACE_MMAP_Memory_Pool_Options),
            );
            ::core::ptr::write(::core::ptr::addr_of_mut!((* __this).bad_flag_), 0);
            {
                (*__this).lock_ = <ACE_Malloc_Lock_Adapter_T_ACE_Null_Mutex_>::operator_call(
                    (((&(<ACE_Malloc_Lock_Adapter_T_ACE_Null_Mutex_>::new()))
                        as *const ACE_Malloc_Lock_Adapter_T_ACE_Null_Mutex_))
                        as *const ACE_Malloc_Lock_Adapter_T_ACE_Null_Mutex_,
                    pool_name,
                );
                if ((((!((*__this).lock_).is_null()) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    (*__this).delete_lock_ = true;
                    (*__this).bad_flag_ = <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::open(
                        (__this)
                            as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    );
                    if ((((((*__this).bad_flag_ as libc::c_int))
                        == ((((-((1) as libc::c_int))) as libc::c_int))) as libc::c_int
                        as libc::c_int) != 0)
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
                                            ((b"/build/ace_full/src/ACE/ace/Malloc_T.cpp\0".as_ptr()
                                                as *const libc::c_char) as *const libc::c_char),
                                            524,
                                            (-((1) as libc::c_int)),
                                            ((__ace_error) as libc::c_int),
                                        );
                                        <ACE_Log_Category_TSS>::log(
                                            (ace___) as *mut ACE_Log_Category_TSS,
                                            LM_ERROR,
                                            ((b"%p\n\0".as_ptr() as *const libc::c_char)
                                                as *const libc::c_char),
                                            b"ACE_Malloc_T<ACE_MEM_POOL_2, ACE_LOCK, ACE_CB>::ACE_Malloc_T\0"
                                                .as_ptr() as *const libc::c_char,
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
            }
            ()
        }
    }
    pub unsafe fn new_s64e2e5cf64745a9b(mut __a0: *const libc::c_char) -> Self {
        let mut __obj: Self = ::core::mem::MaybeUninit::<Self>::zeroed().assume_init();
        Self::new_at_s64e2e5cf64745a9b(::core::ptr::addr_of_mut!(__obj), __a0);
        __obj
    }
    #[doc = "* Try to locate @a name.  If found, return the associated\n   * ACE_Name_Node, else returns 0 if can't find the @a name.\n   * Assumes that locks are held by callers.  Remember to cast the\n   * return value to ACE_CB::ACE_Name_Node*."]
    pub unsafe fn shared_find(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).cb_ptr_).is_null()) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    return ((0) as *mut libc::c_void);
                }
                if ((1) != 0) {
                    {
                        let mut node: *mut ACE_Control_Block_ACE_Name_Node = (((*(*__this)
                            .cb_ptr_)
                            .name_head_) as *mut ACE_Control_Block_ACE_Name_Node);
                        'for_0: loop {
                            if !(((((!(node).is_null()) as libc::c_int) as libc::c_int)
                                != 0))
                            {
                                break;
                            }
                            'cont_0: loop {
                                {
                                    if (((((strcmp(
                                        <ACE_Control_Block_ACE_Name_Node>::name(
                                            (node) as *const ACE_Control_Block_ACE_Name_Node,
                                        ),
                                        name,
                                    ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                        as libc::c_int) != 0)
                                    {
                                        return ((node) as *mut libc::c_void);
                                    }
                                }
                                #[allow(unreachable_code)] break 'cont_0;
                            }
                            {
                                let __v = (((*node).next_)
                                    as *mut ACE_Control_Block_ACE_Name_Node);
                                node = __v;
                                __v
                            };
                        }
                    }
                }
                'while_1: loop {
                    if !(((0) != 0)) {
                        break 'while_1;
                    }
                    'cont_1: loop {
                        { {} }
                        #[allow(unreachable_code)] break 'cont_1;
                    }
                }
                return ((0) as *mut libc::c_void);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Locate @a name and pass out parameter via @a pointer.  If found,
  /// return 0, returns -1 if failure occurs.*/
    pub unsafe fn find_sb79c8f0521a6ddd8(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Read_Guard_ACE_Null_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Read_Guard_ACE_Null_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Read_Guard_ACE_Null_Mutex_>::new_at(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Read_Guard_ACE_Null_Mutex_,
                    (*__this).lock_,
                );
                if (((((<ACE_Guard_ACE_Null_Mutex_>::locked(
                    (::core::ptr::addr_of_mut!(
                        (* (::core::ptr::addr_of_mut!(ace_mon))).__base_0
                    )) as *mut ACE_Guard_ACE_Null_Mutex_,
                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                let mut node: *mut ACE_Control_Block_ACE_Name_Node = (<ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_find(
                    (__this)
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                ) as *mut ACE_Control_Block_ACE_Name_Node);
                if (((((node).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return (-((1) as libc::c_int));
                }
                (*pointer) = ((((*node).pointer_ as *mut libc::c_char))
                    as *mut libc::c_void);
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Allocate memory.  Assumes that locks are held by callers.
    pub unsafe fn shared_malloc(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
    ) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if ((((((*__this).cb_ptr_).is_null()) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    return ((0) as *mut libc::c_void);
                }
                let mut nunits: libc::c_ulong = (((((((((((((((nbytes) as libc::c_ulong))
                    .wrapping_add((24) as libc::c_ulong)) as libc::c_ulong))
                    .wrapping_sub((1) as libc::c_ulong))) as libc::c_ulong))
                    / ((24) as libc::c_ulong)) as libc::c_ulong))
                    .wrapping_add((1) as libc::c_ulong)) as libc::c_ulong);
                let mut prevp: *mut ACE_Control_Block_ACE_Malloc_Header = ((0)
                    as *mut ACE_Control_Block_ACE_Malloc_Header);
                let mut currp: *mut ACE_Control_Block_ACE_Malloc_Header = ((0)
                    as *mut ACE_Control_Block_ACE_Malloc_Header);
                if ((1) != 0) {
                    prevp = (((*(*__this).cb_ptr_).freep_)
                        as *mut ACE_Control_Block_ACE_Malloc_Header);
                    currp = (((*prevp).next_block_)
                        as *mut ACE_Control_Block_ACE_Malloc_Header);
                }
                'while_0: loop {
                    if !(((1) != 0)) {
                        break 'while_0;
                    }
                    'cont_0: loop {
                        {
                            {
                                if ((1) != 0) {
                                    if ((((((*currp).size_ as libc::c_ulong))
                                        >= (((nunits) as libc::c_ulong))) as libc::c_int
                                        as libc::c_int) != 0)
                                    {
                                        if ((((((*currp).size_ as libc::c_ulong))
                                            == (((nunits) as libc::c_ulong))) as libc::c_int
                                            as libc::c_int) != 0)
                                        {
                                            (*prevp).next_block_ = (*currp).next_block_;
                                        } else {
                                            {
                                                let __lv = ::core::ptr::addr_of_mut!((* currp).size_);
                                                unsafe {
                                                    *__lv = ((((*__lv)) as libc::c_ulong))
                                                        .wrapping_sub((nunits) as libc::c_ulong);
                                                    *__lv
                                                }
                                            };
                                            {
                                                currp = (currp).wrapping_offset(((*currp).size_) as isize);
                                                currp
                                            };
                                            <ACE_Control_Block_ACE_Malloc_Header>::init_ptr(
                                                ::core::ptr::addr_of_mut!((* currp).next_block_)
                                                    as *mut *mut ACE_Control_Block_ACE_Malloc_Header,
                                                ((0) as *mut ACE_Control_Block_ACE_Malloc_Header),
                                                (((*__this).cb_ptr_) as *mut libc::c_void),
                                            );
                                            (*currp).size_ = ((nunits) as libc::c_ulong);
                                        }
                                        (*(*__this).cb_ptr_).freep_ = ((prevp)
                                            as *mut ACE_Control_Block_ACE_Malloc_Header);
                                        return (((currp).wrapping_offset((1) as isize))
                                            as *mut libc::c_void);
                                    } else {
                                        if ((((((currp) as *const u8))
                                            == (((((*(*__this).cb_ptr_).freep_
                                                as *mut ACE_Control_Block_ACE_Malloc_Header))
                                                as *const u8))) as libc::c_int as libc::c_int) != 0)
                                        {
                                            let mut chunk_bytes: libc::c_ulong = ((0) as libc::c_ulong);
                                            currp = (<ACE_MMAP_Memory_Pool>::acquire(
                                                (::core::ptr::addr_of_mut!(
                                                    (* ::core::ptr::addr_of!((* __this).memory_pool_) .cast:: <
                                                    ACE_MMAP_Memory_Pool > ().cast_mut())
                                                )) as *mut ACE_MMAP_Memory_Pool,
                                                (((((nunits) as libc::c_ulong))
                                                    .wrapping_mul((24) as libc::c_ulong)) as libc::c_ulong),
                                                ::core::ptr::addr_of_mut!(chunk_bytes),
                                            ) as *mut ACE_Control_Block_ACE_Malloc_Header);
                                            let mut remap_addr: *mut libc::c_void = <ACE_MMAP_Memory_Pool>::base_addr(
                                                (::core::ptr::addr_of!(
                                                    (* ::core::ptr::addr_of!((* __this).memory_pool_) .cast:: <
                                                    ACE_MMAP_Memory_Pool > ().cast_mut())
                                                )) as *const ACE_MMAP_Memory_Pool,
                                            );
                                            if ((((!(remap_addr).is_null()) as libc::c_int)
                                                as libc::c_int) != 0)
                                            {
                                                (*__this).cb_ptr_ = (remap_addr as *mut ACE_Control_Block);
                                            }
                                            if ((((!(currp).is_null()) as libc::c_int) as libc::c_int)
                                                != 0)
                                            {
                                                <ACE_Control_Block_ACE_Malloc_Header>::init_ptr(
                                                    ::core::ptr::addr_of_mut!((* currp).next_block_)
                                                        as *mut *mut ACE_Control_Block_ACE_Malloc_Header,
                                                    ((0) as *mut ACE_Control_Block_ACE_Malloc_Header),
                                                    (((*__this).cb_ptr_) as *mut libc::c_void),
                                                );
                                                (*currp).size_ = (((((chunk_bytes) as libc::c_ulong))
                                                    / ((24) as libc::c_ulong)) as libc::c_ulong);
                                                <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_free(
                                                    (__this)
                                                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                                                    (((currp).wrapping_offset((1) as isize))
                                                        as *mut libc::c_void),
                                                );
                                                currp = (((*(*__this).cb_ptr_).freep_)
                                                    as *mut ACE_Control_Block_ACE_Malloc_Header);
                                            } else {
                                                return ((0) as *mut libc::c_void);
                                            }
                                        }
                                    }
                                    prevp = currp;
                                    currp = (((*currp).next_block_)
                                        as *mut ACE_Control_Block_ACE_Malloc_Header);
                                }
                                'while_1: loop {
                                    if !(((0) != 0)) {
                                        break 'while_1;
                                    }
                                    'cont_1: loop {
                                        { {} }
                                        #[allow(unreachable_code)] break 'cont_1;
                                    }
                                }
                            }
                        }
                        #[allow(unreachable_code)] break 'cont_0;
                    }
                }
                return ((0) as *mut libc::c_void);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Allocate @a nbytes, but don't give them any initial value.
    pub unsafe fn malloc(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
    ) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Null_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Null_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Null_Mutex_>::new_at(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Null_Mutex_,
                    (*__this).lock_,
                );
                if (((((<ACE_Guard_ACE_Null_Mutex_>::locked(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Null_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return ((0) as *mut libc::c_void);
                };
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_malloc(
                    (__this)
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    nbytes,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Associate @a name with @a pointer.  Assumes that locks are held by
  /// callers.*/
    pub unsafe fn shared_bind(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((!((!((*__this).cb_ptr_).is_null())) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    return (-((1) as libc::c_int));
                }
                let mut new_node: *mut ACE_Control_Block_ACE_Name_Node = ((0)
                    as *mut ACE_Control_Block_ACE_Name_Node);
                'dowhile_0: loop {
                    'cont_0: loop {
                        {
                            {
                                new_node = (<ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_malloc(
                                    (__this)
                                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                                    ((((((((32) as libc::c_ulong))
                                        .wrapping_add(
                                            (ACE_OS::strlen_u07dd12a225364fa6(name)) as libc::c_ulong,
                                        )) as libc::c_ulong))
                                        .wrapping_add((1) as libc::c_ulong)) as libc::c_ulong),
                                ) as *mut ACE_Control_Block_ACE_Name_Node);
                                if (((((new_node).is_null()) as libc::c_int) as libc::c_int)
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
                let mut name_ptr: *mut libc::c_char = (((new_node)
                    .wrapping_offset((1) as isize)) as *mut libc::c_char);
                let mut result: *mut ACE_Control_Block_ACE_Name_Node = {
                    let __place: *mut ACE_Control_Block_ACE_Name_Node = (new_node)
                        as *mut ACE_Control_Block_ACE_Name_Node;
                    <ACE_Control_Block_ACE_Name_Node>::new_at(
                        (__place) as *mut ACE_Control_Block_ACE_Name_Node,
                        name,
                        name_ptr,
                        (pointer as *mut libc::c_char),
                        (*(*__this).cb_ptr_).name_head_,
                    );
                    __place
                };
                (*(*__this).cb_ptr_).name_head_ = ((result)
                    as *mut ACE_Control_Block_ACE_Name_Node);
                return 0;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Associate @a name with @a pointer.  If @a duplicates == 0 then do\n   * not allow duplicate name/pointer associations, else if\n   * @a duplicates != 0 then allow duplicate name/pointer\n   * associations.  Returns 0 if successfully binds (1) a previously\n   * unbound @a name or (2) @a duplicates != 0, returns 1 if trying to\n   * bind a previously bound @a name and @a duplicates == 0, else\n   * returns -1 if a resource failure occurs."]
    pub unsafe fn bind(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut libc::c_void,
        mut duplicates: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Write_Guard_ACE_Null_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Write_Guard_ACE_Null_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Write_Guard_ACE_Null_Mutex_>::new_at(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Write_Guard_ACE_Null_Mutex_,
                    (*__this).lock_,
                );
                if (((((<ACE_Guard_ACE_Null_Mutex_>::locked(
                    (::core::ptr::addr_of_mut!(
                        (* (::core::ptr::addr_of_mut!(ace_mon))).__base_0
                    )) as *mut ACE_Guard_ACE_Null_Mutex_,
                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                if (((((((((duplicates as libc::c_int)) == (((0) as libc::c_int)))
                    as libc::c_int as libc::c_int) != 0)
                    && ((((!(<ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_find(
                        (__this)
                            as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                        name,
                    ))
                        .is_null()) as libc::c_int) as libc::c_int) != 0))
                    as libc::c_int) as libc::c_int) != 0)
                {
                    return 1;
                }
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_bind(
                    (__this)
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                    pointer,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Allocate @a nbytes, giving them @a initial_value.
    pub unsafe fn calloc_s6ac7ac984caf1001(
        __this: *mut Self,
        mut nbytes: libc::c_ulong,
        mut initial_value: libc::c_char,
    ) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ptr: *mut libc::c_void = <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::malloc(
                    (__this)
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    nbytes,
                );
                if ((((!(ptr).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    ACE_OS::memset_u2b5dfc47d301370a(
                        ptr,
                        ((initial_value) as libc::c_int),
                        nbytes,
                    );
                }
                return ptr;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Allocate @a n_elem each of size @a elem_size, giving them
  /// @a initial_value.*/
    pub unsafe fn calloc_s7fc4b13687d88c4a(
        __this: *mut Self,
        mut n_elem: libc::c_ulong,
        mut elem_size: libc::c_ulong,
        mut initial_value: libc::c_char,
    ) -> *mut libc::c_void {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::calloc_s6ac7ac984caf1001(
                    (__this)
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    (((n_elem) as libc::c_ulong))
                        .wrapping_mul((elem_size) as libc::c_ulong),
                    initial_value,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Returns 0 if @a name is in the mapping. -1, otherwise.
    pub unsafe fn find_s07b337cff4ad6a1f(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Read_Guard_ACE_Null_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Read_Guard_ACE_Null_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Read_Guard_ACE_Null_Mutex_>::new_at(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Read_Guard_ACE_Null_Mutex_,
                    (*__this).lock_,
                );
                if (((((<ACE_Guard_ACE_Null_Mutex_>::locked(
                    (::core::ptr::addr_of_mut!(
                        (* (::core::ptr::addr_of_mut!(ace_mon))).__base_0
                    )) as *mut ACE_Guard_ACE_Null_Mutex_,
                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                return if (((((<ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_find(
                    (__this)
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                ))
                    .is_null()) as libc::c_int) as libc::c_int) != 0)
                {
                    (-((1) as libc::c_int))
                } else {
                    0
                };
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Deallocate memory pointed to by @a ptr, which must have been
  /// allocated previously by malloc().*/
    pub unsafe fn free(__this: *mut Self, mut ptr: *mut libc::c_void) {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Guard_ACE_Null_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Guard_ACE_Null_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Guard_ACE_Null_Mutex_>::new_at(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Null_Mutex_,
                    (*__this).lock_,
                );
                if (((((<ACE_Guard_ACE_Null_Mutex_>::locked(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Guard_ACE_Null_Mutex_,
                ) as libc::c_int)) != (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {} else {
                    return;
                };
                <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_free(
                    (__this)
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    ptr,
                );
            }
            ()
        }
    }
    #[doc = "* Change the protection of the pages of the mapped region to @a prot\n   * starting at @c this->base_addr_ up to @a len bytes.  If @a len == -1\n   * then change protection of all pages in the mapped region."]
    pub unsafe fn protect_s3cfcbcf4455cdc93(
        __this: *mut Self,
        mut len: libc::c_long,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_MMAP_Memory_Pool>::protect(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).memory_pool_) .cast:: <
                        ACE_MMAP_Memory_Pool > ().cast_mut())
                    )) as *mut ACE_MMAP_Memory_Pool,
                    (len as libc::c_ulong),
                    flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Change the protection of the pages of the mapped region to @a prot
  /// starting at @a addr up to @a len bytes.*/
    pub unsafe fn protect_scc29d13ed7aaf5d9(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_MMAP_Memory_Pool>::protect_u1b7d5a46d0b852fe(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).memory_pool_) .cast:: <
                        ACE_MMAP_Memory_Pool > ().cast_mut())
                    )) as *mut ACE_MMAP_Memory_Pool,
                    addr,
                    len,
                    flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///Releases resources allocated by this object.
    pub unsafe fn remove(__this: *mut Self) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                if (((*__this).delete_lock_ as libc::c_int) != 0) {
                    <ACE_Null_Mutex>::remove(((*__this).lock_) as *mut ACE_Null_Mutex);
                }
                let mut result: libc::c_int = ((<ACE_MMAP_Memory_Pool>::release(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).memory_pool_) .cast:: <
                        ACE_MMAP_Memory_Pool > ().cast_mut())
                    )) as *mut ACE_MMAP_Memory_Pool,
                    1,
                )) as libc::c_int);
                (*__this).cb_ptr_ = ((0) as *mut ACE_Control_Block);
                return ((result) as libc::c_int);
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Sync @a len bytes of the memory region to the backing store\n   * starting at @c this->base_addr_.  If @a len == -1 then sync the\n   * whole region."]
    pub unsafe fn sync_s3cfcbcf4455cdc93(
        __this: *mut Self,
        mut len: libc::c_long,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_MMAP_Memory_Pool>::sync(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).memory_pool_) .cast:: <
                        ACE_MMAP_Memory_Pool > ().cast_mut())
                    )) as *mut ACE_MMAP_Memory_Pool,
                    (len as libc::c_ulong),
                    flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    /**Sync @a len bytes of the memory region to the backing store
  /// starting at @c addr_.*/
    pub unsafe fn sync_scc29d13ed7aaf5d9(
        __this: *mut Self,
        mut addr: *mut libc::c_void,
        mut len: libc::c_ulong,
        mut flags: libc::c_int,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                return <ACE_MMAP_Memory_Pool>::sync_u8f2f28f4f83a3cf6(
                    (::core::ptr::addr_of_mut!(
                        (* ::core::ptr::addr_of!((* __this).memory_pool_) .cast:: <
                        ACE_MMAP_Memory_Pool > ().cast_mut())
                    )) as *mut ACE_MMAP_Memory_Pool,
                    addr,
                    len,
                    flags,
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Associate @a name with @a pointer.  Does not allow duplicate\n   * name/pointer associations.  Returns 0 if successfully binds\n   * (1) a previously unbound @a name, 1 if trying to bind a previously\n   * bound @a name, or returns -1 if a resource failure occurs.  When\n   * this call returns @a pointer's value will always reference the\n   * void * that @a name is associated with.  Thus, if the caller needs\n   * to use @a pointer (e.g., to free it) a copy must be maintained by\n   * the caller."]
    pub unsafe fn trybind(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Write_Guard_ACE_Null_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Write_Guard_ACE_Null_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Write_Guard_ACE_Null_Mutex_>::new_at(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Write_Guard_ACE_Null_Mutex_,
                    (*__this).lock_,
                );
                if (((((<ACE_Guard_ACE_Null_Mutex_>::locked(
                    (::core::ptr::addr_of_mut!(
                        (* (::core::ptr::addr_of_mut!(ace_mon))).__base_0
                    )) as *mut ACE_Guard_ACE_Null_Mutex_,
                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                let mut node: *mut ACE_Control_Block_ACE_Name_Node = (<ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_find(
                    (__this)
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                ) as *mut ACE_Control_Block_ACE_Name_Node);
                if (((((node).is_null()) as libc::c_int) as libc::c_int) != 0) {
                    return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_bind(
                        (__this)
                            as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                        name,
                        (*pointer),
                    );
                }
                (*pointer) = ((((*node).pointer_ as *mut libc::c_char))
                    as *mut libc::c_void);
                return 1;
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Unbind (remove) one association of @a name to @a pointer.  Returns\n   * the value of pointer in case the caller needs to deallocate\n   * memory.  If you want to remove all occurrences of @a name you'll\n   * need to call this method multiple times until it fails..."]
    pub unsafe fn unbind_sb79c8f0521a6ddd8(
        __this: *mut Self,
        mut name: *const libc::c_char,
        mut pointer: *mut *mut libc::c_void,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut ace_mon: ACE_Write_Guard_ACE_Null_Mutex_ = ::core::mem::MaybeUninit::<
                    ACE_Write_Guard_ACE_Null_Mutex_,
                >::zeroed()
                    .assume_init();
                <ACE_Write_Guard_ACE_Null_Mutex_>::new_at(
                    (::core::ptr::addr_of_mut!(ace_mon))
                        as *mut ACE_Write_Guard_ACE_Null_Mutex_,
                    (*__this).lock_,
                );
                if (((((<ACE_Guard_ACE_Null_Mutex_>::locked(
                    (::core::ptr::addr_of_mut!(
                        (* (::core::ptr::addr_of_mut!(ace_mon))).__base_0
                    )) as *mut ACE_Guard_ACE_Null_Mutex_,
                ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                    as libc::c_int) != 0)
                {
                    return (-((1) as libc::c_int));
                }
                if ((((((*__this).cb_ptr_).is_null()) as libc::c_int) as libc::c_int)
                    != 0)
                {
                    return (-((1) as libc::c_int));
                }
                let mut prev: *mut ACE_Control_Block_ACE_Name_Node = ((0)
                    as *mut ACE_Control_Block_ACE_Name_Node);
                {
                    let mut curr: *mut ACE_Control_Block_ACE_Name_Node = (((*(*__this)
                        .cb_ptr_)
                        .name_head_) as *mut ACE_Control_Block_ACE_Name_Node);
                    'for_0: loop {
                        if !(((((!(curr).is_null()) as libc::c_int) as libc::c_int)
                            != 0))
                        {
                            break;
                        }
                        'cont_0: loop {
                            {
                                {
                                    if (((((strcmp(
                                        <ACE_Control_Block_ACE_Name_Node>::name(
                                            (curr) as *const ACE_Control_Block_ACE_Name_Node,
                                        ),
                                        name,
                                    ) as libc::c_int)) == (((0) as libc::c_int))) as libc::c_int
                                        as libc::c_int) != 0)
                                    {
                                        (*pointer) = ((((*curr).pointer_ as *mut libc::c_char))
                                            as *mut libc::c_void);
                                        if (((((prev).is_null()) as libc::c_int) as libc::c_int)
                                            != 0)
                                        {
                                            (*(*__this).cb_ptr_).name_head_ = (*curr).next_;
                                        } else {
                                            (*prev).next_ = (*curr).next_;
                                        }
                                        if (!((*curr).next_).is_null()) {
                                            (*(*curr).next_).prev_ = ((prev)
                                                as *mut ACE_Control_Block_ACE_Name_Node);
                                        }
                                        <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::shared_free(
                                            (__this)
                                                as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                                            ((curr) as *mut libc::c_void),
                                        );
                                        return 0;
                                    }
                                    prev = curr;
                                }
                            }
                            #[allow(unreachable_code)] break 'cont_0;
                        }
                        {
                            let __v = (((*curr).next_)
                                as *mut ACE_Control_Block_ACE_Name_Node);
                            curr = __v;
                            __v
                        };
                    }
                }
                return (-((1) as libc::c_int));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    #[doc = "* Unbind (remove) the name from the map.  Don't return the pointer\n   * to the caller.  If you want to remove all occurrences of @a name\n   * you'll need to call this method multiple times until it fails..."]
    pub unsafe fn unbind_s07b337cff4ad6a1f(
        __this: *mut Self,
        mut name: *const libc::c_char,
    ) -> libc::c_int {
        unsafe {
            let __this: *mut Self = __this as *mut Self;
            {
                let mut temp: *mut libc::c_void = ((0) as *mut libc::c_void);
                return <ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_>::unbind_sb79c8f0521a6ddd8(
                    (__this)
                        as *mut ACE_Malloc_T_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__ACE_Control_Block_,
                    name,
                    ::core::ptr::addr_of_mut!(temp),
                );
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
}
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
pub unsafe fn __vdtor_ue6180d8ab1473747(__this: *mut ACE_Sig_Handler) {
    let _ = Box::from_raw(__this as *mut ACE_Sig_Handler);
}
pub unsafe fn __vthunk_ou539f07b05227de6a_iu539f07b05227de6a(
    __this: *mut ACE_Sig_Handler,
    p0: libc::c_int,
    p1: *mut ACE_Event_Handler,
    p2: *mut ACE_Sig_Action,
    p3: *mut *mut ACE_Event_Handler,
    p4: *mut ACE_Sig_Action,
) -> libc::c_int {
    <ACE_Sig_Handler>::register_handler(
        (__this as *mut ACE_Sig_Handler),
        p0,
        p1,
        p2,
        p3,
        p4,
    )
}
pub unsafe fn __vthunk_oue9b6bb8f2d4f5657_iue9b6bb8f2d4f5657(
    __this: *mut ACE_Sig_Handler,
    p0: libc::c_int,
    p1: *mut ACE_Sig_Action,
    p2: *mut ACE_Sig_Action,
    p3: libc::c_int,
) -> libc::c_int {
    <ACE_Sig_Handler>::remove_handler((__this as *mut ACE_Sig_Handler), p0, p1, p2, p3)
}
pub unsafe fn __vthunk_ouf42b9415c0084b09_iuf42b9415c0084b09(
    __this: *mut ACE_Sig_Handler,
    p0: libc::c_int,
) -> *mut ACE_Event_Handler {
    <ACE_Sig_Handler>::handler((__this as *mut ACE_Sig_Handler), p0)
}
pub unsafe fn __vthunk_ouc91b6b15865c04fe_iuc91b6b15865c04fe(
    __this: *mut ACE_Sig_Handler,
    p0: libc::c_int,
    p1: *mut ACE_Event_Handler,
) -> *mut ACE_Event_Handler {
    <ACE_Sig_Handler>::handler_uc91b6b15865c04fe(
        (__this as *mut ACE_Sig_Handler),
        p0,
        p1,
    )
}
#[repr(C)]
pub struct __Vtbl_ue6180d8ab1473747 {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Sig_Handler),
    pub vfn_u539f07b05227de6a: unsafe fn(
        *mut ACE_Sig_Handler,
        libc::c_int,
        *mut ACE_Event_Handler,
        *mut ACE_Sig_Action,
        *mut *mut ACE_Event_Handler,
        *mut ACE_Sig_Action,
    ) -> libc::c_int,
    pub vfn_ue9b6bb8f2d4f5657: unsafe fn(
        *mut ACE_Sig_Handler,
        libc::c_int,
        *mut ACE_Sig_Action,
        *mut ACE_Sig_Action,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uf42b9415c0084b09: unsafe fn(
        *mut ACE_Sig_Handler,
        libc::c_int,
    ) -> *mut ACE_Event_Handler,
    pub vfn_uc91b6b15865c04fe: unsafe fn(
        *mut ACE_Sig_Handler,
        libc::c_int,
        *mut ACE_Event_Handler,
    ) -> *mut ACE_Event_Handler,
}
pub static __VTBL_ue6180d8ab1473747: __Vtbl_ue6180d8ab1473747 = __Vtbl_ue6180d8ab1473747 {
    __type_info: &__TYPEINFO_15ACE_Sig_Handler,
    __vdtor: __vdtor_ue6180d8ab1473747,
    vfn_u539f07b05227de6a: __vthunk_ou539f07b05227de6a_iu539f07b05227de6a,
    vfn_ue9b6bb8f2d4f5657: __vthunk_oue9b6bb8f2d4f5657_iue9b6bb8f2d4f5657,
    vfn_uf42b9415c0084b09: __vthunk_ouf42b9415c0084b09_iuf42b9415c0084b09,
    vfn_uc91b6b15865c04fe: __vthunk_ouc91b6b15865c04fe_iuc91b6b15865c04fe,
};
pub unsafe fn __vdtor_u421636ab3c02cd5c(__this: *mut ACE_Sig_Handler) {
    let _ = Box::from_raw(__this as *mut ACE_Sig_Handlers);
}
pub unsafe fn __vthunk_oub83015d30a4c3558_iu539f07b05227de6a(
    __this: *mut ACE_Sig_Handler,
    p0: libc::c_int,
    p1: *mut ACE_Event_Handler,
    p2: *mut ACE_Sig_Action,
    p3: *mut *mut ACE_Event_Handler,
    p4: *mut ACE_Sig_Action,
) -> libc::c_int {
    <ACE_Sig_Handlers>::register_handler(
        (__this as *mut ACE_Sig_Handlers),
        p0,
        p1,
        p2,
        p3,
        p4,
    )
}
pub unsafe fn __vthunk_ouc6f94673b1dd5829_iue9b6bb8f2d4f5657(
    __this: *mut ACE_Sig_Handler,
    p0: libc::c_int,
    p1: *mut ACE_Sig_Action,
    p2: *mut ACE_Sig_Action,
    p3: libc::c_int,
) -> libc::c_int {
    <ACE_Sig_Handlers>::remove_handler((__this as *mut ACE_Sig_Handlers), p0, p1, p2, p3)
}
pub unsafe fn __vthunk_ou12fcd1409ea38bab_iuf42b9415c0084b09(
    __this: *mut ACE_Sig_Handler,
    p0: libc::c_int,
) -> *mut ACE_Event_Handler {
    <ACE_Sig_Handlers>::handler((__this as *mut ACE_Sig_Handlers), p0)
}
pub unsafe fn __vthunk_ou4822a5c3ba6d369c_iuc91b6b15865c04fe(
    __this: *mut ACE_Sig_Handler,
    p0: libc::c_int,
    p1: *mut ACE_Event_Handler,
) -> *mut ACE_Event_Handler {
    <ACE_Sig_Handlers>::handler_u4822a5c3ba6d369c(
        (__this as *mut ACE_Sig_Handlers),
        p0,
        p1,
    )
}
#[repr(C)]
pub struct __Vtbl_u421636ab3c02cd5c {
    pub __type_info: &'static crate::__cxx_std::TypeInfo,
    pub __vdtor: unsafe fn(*mut ACE_Sig_Handler),
    pub vfn_u539f07b05227de6a: unsafe fn(
        *mut ACE_Sig_Handler,
        libc::c_int,
        *mut ACE_Event_Handler,
        *mut ACE_Sig_Action,
        *mut *mut ACE_Event_Handler,
        *mut ACE_Sig_Action,
    ) -> libc::c_int,
    pub vfn_ue9b6bb8f2d4f5657: unsafe fn(
        *mut ACE_Sig_Handler,
        libc::c_int,
        *mut ACE_Sig_Action,
        *mut ACE_Sig_Action,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uf42b9415c0084b09: unsafe fn(
        *mut ACE_Sig_Handler,
        libc::c_int,
    ) -> *mut ACE_Event_Handler,
    pub vfn_uc91b6b15865c04fe: unsafe fn(
        *mut ACE_Sig_Handler,
        libc::c_int,
        *mut ACE_Event_Handler,
    ) -> *mut ACE_Event_Handler,
}
pub static __VTBL_u421636ab3c02cd5c: __Vtbl_u421636ab3c02cd5c = __Vtbl_u421636ab3c02cd5c {
    __type_info: &__TYPEINFO_16ACE_Sig_Handlers,
    __vdtor: __vdtor_u421636ab3c02cd5c,
    vfn_u539f07b05227de6a: __vthunk_oub83015d30a4c3558_iu539f07b05227de6a,
    vfn_ue9b6bb8f2d4f5657: __vthunk_ouc6f94673b1dd5829_iue9b6bb8f2d4f5657,
    vfn_uf42b9415c0084b09: __vthunk_ou12fcd1409ea38bab_iuf42b9415c0084b09,
    vfn_uc91b6b15865c04fe: __vthunk_ou4822a5c3ba6d369c_iuc91b6b15865c04fe,
};
pub unsafe fn __vdtor_uba5fe4b11c1b37b5(__this: *mut ACE_Event_Handler) {
    let _ = Box::from_raw(__this as *mut ACE_MMAP_Memory_Pool);
}
pub unsafe fn __vthunk_ou49fba8633748ad3e_iu3f872401b3199422(
    __this: *mut ACE_Event_Handler,
    p0: libc::c_int,
    p1: *mut siginfo_t,
    p2: *mut ucontext_t,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::handle_signal(
        (__this as *mut ACE_MMAP_Memory_Pool),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou885510de379ba0ae_iu885510de379ba0ae(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_ulong,
    p1: *mut libc::c_ulong,
    p2: *mut libc::c_int,
) -> *mut libc::c_void {
    <ACE_MMAP_Memory_Pool>::init_acquire(
        (__this as *mut ACE_MMAP_Memory_Pool),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou9e3b67a324526ce5_iu9e3b67a324526ce5(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_ulong,
    p1: *mut libc::c_ulong,
) -> *mut libc::c_void {
    <ACE_MMAP_Memory_Pool>::acquire((__this as *mut ACE_MMAP_Memory_Pool), p0, p1)
}
pub unsafe fn __vthunk_ou5f619f2d8864069c_iu5f619f2d8864069c(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::release((__this as *mut ACE_MMAP_Memory_Pool), p0)
}
pub unsafe fn __vthunk_ou0d148cc732a7616f_iu0d148cc732a7616f(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_ulong,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::sync((__this as *mut ACE_MMAP_Memory_Pool), p0, p1)
}
pub unsafe fn __vthunk_oua77e6bd2e6007bc6_iua77e6bd2e6007bc6(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::sync_ua77e6bd2e6007bc6(
        (__this as *mut ACE_MMAP_Memory_Pool),
        p0,
    )
}
pub unsafe fn __vthunk_ou8f2f28f4f83a3cf6_iu8f2f28f4f83a3cf6(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::sync_u8f2f28f4f83a3cf6(
        (__this as *mut ACE_MMAP_Memory_Pool),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_oua4c3b772c5471bb7_iua4c3b772c5471bb7(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_ulong,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::protect((__this as *mut ACE_MMAP_Memory_Pool), p0, p1)
}
pub unsafe fn __vthunk_ou540e04f76dbb68be_iu540e04f76dbb68be(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::protect_u540e04f76dbb68be(
        (__this as *mut ACE_MMAP_Memory_Pool),
        p0,
    )
}
pub unsafe fn __vthunk_ou1b7d5a46d0b852fe_iu1b7d5a46d0b852fe(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::protect_u1b7d5a46d0b852fe(
        (__this as *mut ACE_MMAP_Memory_Pool),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ouaaac20440c36bfb9_iuaaac20440c36bfb9(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: *mut libc::c_void,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::remap((__this as *mut ACE_MMAP_Memory_Pool), p0)
}
pub unsafe fn __vthunk_ou5bebae1215b9786f_iu5bebae1215b9786f(
    __this: *mut ACE_MMAP_Memory_Pool,
) -> *mut libc::c_void {
    <ACE_MMAP_Memory_Pool>::base_addr((__this as *mut ACE_MMAP_Memory_Pool))
}
pub unsafe fn __vthunk_oua9dd1ff659afcbcc_iua9dd1ff659afcbcc(
    __this: *mut ACE_MMAP_Memory_Pool,
) {
    <ACE_MMAP_Memory_Pool>::dump((__this as *mut ACE_MMAP_Memory_Pool))
}
pub unsafe fn __vthunk_ou4d0589be66e92dfe_iu4d0589be66e92dfe(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_ulong,
) -> libc::c_ulong {
    <ACE_MMAP_Memory_Pool>::round_up((__this as *mut ACE_MMAP_Memory_Pool), p0)
}
pub unsafe fn __vthunk_ouc4c50a602485e978_iuc4c50a602485e978(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_ulong,
    p1: *mut libc::c_ulong,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::commit_backing_store_name(
        (__this as *mut ACE_MMAP_Memory_Pool),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ou40093adb9e70cb2d_iu40093adb9e70cb2d(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_ulong,
) -> libc::c_int {
    <ACE_MMAP_Memory_Pool>::map_file((__this as *mut ACE_MMAP_Memory_Pool), p0)
}
#[repr(C)]
pub struct __Vtbl_uba5fe4b11c1b37b5 {
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
    pub vfn_u885510de379ba0ae: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        *mut libc::c_ulong,
        *mut libc::c_int,
    ) -> *mut libc::c_void,
    pub vfn_u9e3b67a324526ce5: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        *mut libc::c_ulong,
    ) -> *mut libc::c_void,
    pub vfn_u5f619f2d8864069c: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u0d148cc732a7616f: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_ua77e6bd2e6007bc6: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u8f2f28f4f83a3cf6: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        *mut libc::c_void,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_ua4c3b772c5471bb7: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u540e04f76dbb68be: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u1b7d5a46d0b852fe: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        *mut libc::c_void,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uaaac20440c36bfb9: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        *mut libc::c_void,
    ) -> libc::c_int,
    pub vfn_u5bebae1215b9786f: unsafe fn(*mut ACE_MMAP_Memory_Pool) -> *mut libc::c_void,
    pub vfn_ua9dd1ff659afcbcc: unsafe fn(*mut ACE_MMAP_Memory_Pool),
    pub vfn_u4d0589be66e92dfe: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
    ) -> libc::c_ulong,
    pub vfn_uc4c50a602485e978: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        *mut libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u40093adb9e70cb2d: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
    ) -> libc::c_int,
}
pub static __VTBL_uba5fe4b11c1b37b5: __Vtbl_uba5fe4b11c1b37b5 = __Vtbl_uba5fe4b11c1b37b5 {
    __type_info: &__TYPEINFO_20ACE_MMAP_Memory_Pool,
    __vdtor: __vdtor_uba5fe4b11c1b37b5,
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
    vfn_u3f872401b3199422: __vthunk_ou49fba8633748ad3e_iu3f872401b3199422,
    vfn_uc909131ec77881c7: __vthunk_ouc909131ec77881c7_iuc909131ec77881c7,
    vfn_uf69856450f20fddf: __vthunk_ouf69856450f20fddf_iuf69856450f20fddf,
    vfn_u366f25d08aaa768b: __vthunk_ou366f25d08aaa768b_iu366f25d08aaa768b,
    vfn_u5cb133098a439c05: __vthunk_ou5cb133098a439c05_iu5cb133098a439c05,
    vfn_ub75675325861507c: __vthunk_oub75675325861507c_iub75675325861507c,
    vfn_u8ad488637ad422a6: __vthunk_ou8ad488637ad422a6_iu8ad488637ad422a6,
    vfn_u1c00fc150b810f94: __vthunk_ou1c00fc150b810f94_iu1c00fc150b810f94,
    vfn_u590ad03b86fd5891: __vthunk_ou590ad03b86fd5891_iu590ad03b86fd5891,
    vfn_u885510de379ba0ae: __vthunk_ou885510de379ba0ae_iu885510de379ba0ae,
    vfn_u9e3b67a324526ce5: __vthunk_ou9e3b67a324526ce5_iu9e3b67a324526ce5,
    vfn_u5f619f2d8864069c: __vthunk_ou5f619f2d8864069c_iu5f619f2d8864069c,
    vfn_u0d148cc732a7616f: __vthunk_ou0d148cc732a7616f_iu0d148cc732a7616f,
    vfn_ua77e6bd2e6007bc6: __vthunk_oua77e6bd2e6007bc6_iua77e6bd2e6007bc6,
    vfn_u8f2f28f4f83a3cf6: __vthunk_ou8f2f28f4f83a3cf6_iu8f2f28f4f83a3cf6,
    vfn_ua4c3b772c5471bb7: __vthunk_oua4c3b772c5471bb7_iua4c3b772c5471bb7,
    vfn_u540e04f76dbb68be: __vthunk_ou540e04f76dbb68be_iu540e04f76dbb68be,
    vfn_u1b7d5a46d0b852fe: __vthunk_ou1b7d5a46d0b852fe_iu1b7d5a46d0b852fe,
    vfn_uaaac20440c36bfb9: __vthunk_ouaaac20440c36bfb9_iuaaac20440c36bfb9,
    vfn_u5bebae1215b9786f: __vthunk_ou5bebae1215b9786f_iu5bebae1215b9786f,
    vfn_ua9dd1ff659afcbcc: __vthunk_oua9dd1ff659afcbcc_iua9dd1ff659afcbcc,
    vfn_u4d0589be66e92dfe: __vthunk_ou4d0589be66e92dfe_iu4d0589be66e92dfe,
    vfn_uc4c50a602485e978: __vthunk_ouc4c50a602485e978_iuc4c50a602485e978,
    vfn_u40093adb9e70cb2d: __vthunk_ou40093adb9e70cb2d_iu40093adb9e70cb2d,
};
pub unsafe fn __vdtor_uea71ce087f45c45a(__this: *mut ACE_Event_Handler) {
    let _ = Box::from_raw(__this as *mut ACE_Lite_MMAP_Memory_Pool);
}
pub unsafe fn __vthunk_oud22207bce87a31f3_iu0d148cc732a7616f(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_ulong,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_Lite_MMAP_Memory_Pool>::sync((__this as *mut ACE_Lite_MMAP_Memory_Pool), p0, p1)
}
pub unsafe fn __vthunk_ou5c8be02cdb2a7232_iua77e6bd2e6007bc6(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: libc::c_int,
) -> libc::c_int {
    <ACE_Lite_MMAP_Memory_Pool>::sync_u5c8be02cdb2a7232(
        (__this as *mut ACE_Lite_MMAP_Memory_Pool),
        p0,
    )
}
pub unsafe fn __vthunk_oub3caaa2d6b80b7fa_iu8f2f28f4f83a3cf6(
    __this: *mut ACE_MMAP_Memory_Pool,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_Lite_MMAP_Memory_Pool>::sync_ub3caaa2d6b80b7fa(
        (__this as *mut ACE_Lite_MMAP_Memory_Pool),
        p0,
        p1,
        p2,
    )
}
#[repr(C)]
pub struct __Vtbl_uea71ce087f45c45a {
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
    pub vfn_u885510de379ba0ae: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        *mut libc::c_ulong,
        *mut libc::c_int,
    ) -> *mut libc::c_void,
    pub vfn_u9e3b67a324526ce5: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        *mut libc::c_ulong,
    ) -> *mut libc::c_void,
    pub vfn_u5f619f2d8864069c: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u0d148cc732a7616f: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_ua77e6bd2e6007bc6: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u8f2f28f4f83a3cf6: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        *mut libc::c_void,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_ua4c3b772c5471bb7: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u540e04f76dbb68be: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_u1b7d5a46d0b852fe: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        *mut libc::c_void,
        libc::c_ulong,
        libc::c_int,
    ) -> libc::c_int,
    pub vfn_uaaac20440c36bfb9: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        *mut libc::c_void,
    ) -> libc::c_int,
    pub vfn_u5bebae1215b9786f: unsafe fn(*mut ACE_MMAP_Memory_Pool) -> *mut libc::c_void,
    pub vfn_ua9dd1ff659afcbcc: unsafe fn(*mut ACE_MMAP_Memory_Pool),
    pub vfn_u4d0589be66e92dfe: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
    ) -> libc::c_ulong,
    pub vfn_uc4c50a602485e978: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
        *mut libc::c_ulong,
    ) -> libc::c_int,
    pub vfn_u40093adb9e70cb2d: unsafe fn(
        *mut ACE_MMAP_Memory_Pool,
        libc::c_ulong,
    ) -> libc::c_int,
}
pub static __VTBL_uea71ce087f45c45a: __Vtbl_uea71ce087f45c45a = __Vtbl_uea71ce087f45c45a {
    __type_info: &__TYPEINFO_25ACE_Lite_MMAP_Memory_Pool,
    __vdtor: __vdtor_uea71ce087f45c45a,
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
    vfn_u3f872401b3199422: __vthunk_ou49fba8633748ad3e_iu3f872401b3199422,
    vfn_uc909131ec77881c7: __vthunk_ouc909131ec77881c7_iuc909131ec77881c7,
    vfn_uf69856450f20fddf: __vthunk_ouf69856450f20fddf_iuf69856450f20fddf,
    vfn_u366f25d08aaa768b: __vthunk_ou366f25d08aaa768b_iu366f25d08aaa768b,
    vfn_u5cb133098a439c05: __vthunk_ou5cb133098a439c05_iu5cb133098a439c05,
    vfn_ub75675325861507c: __vthunk_oub75675325861507c_iub75675325861507c,
    vfn_u8ad488637ad422a6: __vthunk_ou8ad488637ad422a6_iu8ad488637ad422a6,
    vfn_u1c00fc150b810f94: __vthunk_ou1c00fc150b810f94_iu1c00fc150b810f94,
    vfn_u590ad03b86fd5891: __vthunk_ou590ad03b86fd5891_iu590ad03b86fd5891,
    vfn_u885510de379ba0ae: __vthunk_ou885510de379ba0ae_iu885510de379ba0ae,
    vfn_u9e3b67a324526ce5: __vthunk_ou9e3b67a324526ce5_iu9e3b67a324526ce5,
    vfn_u5f619f2d8864069c: __vthunk_ou5f619f2d8864069c_iu5f619f2d8864069c,
    vfn_u0d148cc732a7616f: __vthunk_oud22207bce87a31f3_iu0d148cc732a7616f,
    vfn_ua77e6bd2e6007bc6: __vthunk_ou5c8be02cdb2a7232_iua77e6bd2e6007bc6,
    vfn_u8f2f28f4f83a3cf6: __vthunk_oub3caaa2d6b80b7fa_iu8f2f28f4f83a3cf6,
    vfn_ua4c3b772c5471bb7: __vthunk_oua4c3b772c5471bb7_iua4c3b772c5471bb7,
    vfn_u540e04f76dbb68be: __vthunk_ou540e04f76dbb68be_iu540e04f76dbb68be,
    vfn_u1b7d5a46d0b852fe: __vthunk_ou1b7d5a46d0b852fe_iu1b7d5a46d0b852fe,
    vfn_uaaac20440c36bfb9: __vthunk_ouaaac20440c36bfb9_iuaaac20440c36bfb9,
    vfn_u5bebae1215b9786f: __vthunk_ou5bebae1215b9786f_iu5bebae1215b9786f,
    vfn_ua9dd1ff659afcbcc: __vthunk_oua9dd1ff659afcbcc_iua9dd1ff659afcbcc,
    vfn_u4d0589be66e92dfe: __vthunk_ou4d0589be66e92dfe_iu4d0589be66e92dfe,
    vfn_uc4c50a602485e978: __vthunk_ouc4c50a602485e978_iuc4c50a602485e978,
    vfn_u40093adb9e70cb2d: __vthunk_ou40093adb9e70cb2d_iu40093adb9e70cb2d,
};
pub unsafe fn __vdtor_u36615f686fb5613d(__this: *mut ACE_Allocator) {
    let _ = Box::from_raw(
        __this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__,
    );
}
pub unsafe fn __vthunk_ouc1b24dd723b8d076_iu685215409e23bf32(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
) -> *mut libc::c_void {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::malloc(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
    )
}
pub unsafe fn __vthunk_ou1518de55eae3ac9a_iu0a9372cacdda8cbe(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
    p1: libc::c_char,
) -> *mut libc::c_void {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::calloc_s1ae7c325fc123850(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ou98d8624a0ef3def7_iued53ccfa62009d93(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
    p1: libc::c_ulong,
    p2: libc::c_char,
) -> *mut libc::c_void {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::calloc_s20377fc4621028db(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_oufe1e66791cf97842_iucc7a27ee055bb87e(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
) {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::free(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
    )
}
pub unsafe fn __vthunk_oudc8c4b3183aa906f_iuaeedb459d846087b(
    __this: *mut ACE_Allocator,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::remove(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
    )
}
pub unsafe fn __vthunk_ouccc466403919513a_iu7bff1870c893b3fe(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut libc::c_void,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::bind(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou25c488609ed5a22d_iu381e2dddd3465a71(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::trybind(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_oue5d4b423e85492f6_iu41a4de2216226892(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::find_sd1d97c7eb0448d41(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ou64d39da9dce82ac1_iu22342900ef7c0f5d(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::find_se7e55738a6d934ec(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
    )
}
pub unsafe fn __vthunk_ouc180d43ab800ddd4_iuce55bcfdd7d4af38(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::unbind_se7e55738a6d934ec(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
    )
}
pub unsafe fn __vthunk_ou6117f3af9be6fbdf_iu18ccae12f60528e3(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::unbind_sd1d97c7eb0448d41(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ou3a0b146e7ecabee3_iu62da3684ac8c6bf7(
    __this: *mut ACE_Allocator,
    p0: libc::c_long,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::sync_s48b3af97b45c173e(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ouc466738d37f0cc0a_iu7db10b00cce8fa5e(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::sync_sdf95393c41ed2b14(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou0901991088fc06fb_iuada926f987c0415f(
    __this: *mut ACE_Allocator,
    p0: libc::c_long,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::protect_s48b3af97b45c173e(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_oua75a7d1542c7f982_iuf36ac9d4f584a786(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::protect_sdf95393c41ed2b14(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou1158bf75b031b5a0_iuac8ca8237e7c8154(
    __this: *mut ACE_Allocator,
) {
    <ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__>::dump(
        (__this
            as *mut ACE_Allocator_Adapter_ACE_Malloc_ACE_MMAP_Memory_Pool__ACE_Null_Mutex__),
    )
}
#[repr(C)]
pub struct __Vtbl_u36615f686fb5613d {
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
pub static __VTBL_u36615f686fb5613d: __Vtbl_u36615f686fb5613d = __Vtbl_u36615f686fb5613d {
    __type_info: &__TYPEINFO_21ACE_Allocator_Adapter,
    __vdtor: __vdtor_u36615f686fb5613d,
    vfn_u685215409e23bf32: __vthunk_ouc1b24dd723b8d076_iu685215409e23bf32,
    vfn_u0a9372cacdda8cbe: __vthunk_ou1518de55eae3ac9a_iu0a9372cacdda8cbe,
    vfn_ued53ccfa62009d93: __vthunk_ou98d8624a0ef3def7_iued53ccfa62009d93,
    vfn_ucc7a27ee055bb87e: __vthunk_oufe1e66791cf97842_iucc7a27ee055bb87e,
    vfn_uaeedb459d846087b: __vthunk_oudc8c4b3183aa906f_iuaeedb459d846087b,
    vfn_u7bff1870c893b3fe: __vthunk_ouccc466403919513a_iu7bff1870c893b3fe,
    vfn_u381e2dddd3465a71: __vthunk_ou25c488609ed5a22d_iu381e2dddd3465a71,
    vfn_u41a4de2216226892: __vthunk_oue5d4b423e85492f6_iu41a4de2216226892,
    vfn_u22342900ef7c0f5d: __vthunk_ou64d39da9dce82ac1_iu22342900ef7c0f5d,
    vfn_uce55bcfdd7d4af38: __vthunk_ouc180d43ab800ddd4_iuce55bcfdd7d4af38,
    vfn_u18ccae12f60528e3: __vthunk_ou6117f3af9be6fbdf_iu18ccae12f60528e3,
    vfn_u62da3684ac8c6bf7: __vthunk_ou3a0b146e7ecabee3_iu62da3684ac8c6bf7,
    vfn_u7db10b00cce8fa5e: __vthunk_ouc466738d37f0cc0a_iu7db10b00cce8fa5e,
    vfn_uada926f987c0415f: __vthunk_ou0901991088fc06fb_iuada926f987c0415f,
    vfn_uf36ac9d4f584a786: __vthunk_oua75a7d1542c7f982_iuf36ac9d4f584a786,
    vfn_uac8ca8237e7c8154: __vthunk_ou1158bf75b031b5a0_iuac8ca8237e7c8154,
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
pub unsafe fn __vdtor_u8a5e72ffd9043ac9(__this: *mut ACE_Allocator) {
    let _ = Box::from_raw(__this as *mut ACE_New_Allocator);
}
pub unsafe fn __vthunk_oua823c7d088ad569c_iu685215409e23bf32(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
) -> *mut libc::c_void {
    <ACE_New_Allocator>::malloc((__this as *mut ACE_New_Allocator), p0)
}
pub unsafe fn __vthunk_ou757910826b014974_iu0a9372cacdda8cbe(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
    p1: libc::c_char,
) -> *mut libc::c_void {
    <ACE_New_Allocator>::calloc((__this as *mut ACE_New_Allocator), p0, p1)
}
pub unsafe fn __vthunk_oub14cd9e591a2d69d_iued53ccfa62009d93(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
    p1: libc::c_ulong,
    p2: libc::c_char,
) -> *mut libc::c_void {
    <ACE_New_Allocator>::calloc_ub14cd9e591a2d69d(
        (__this as *mut ACE_New_Allocator),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou6e02a1c7f32efb3c_iucc7a27ee055bb87e(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
) {
    <ACE_New_Allocator>::free((__this as *mut ACE_New_Allocator), p0)
}
pub unsafe fn __vthunk_ou5a1829294f889bed_iuaeedb459d846087b(
    __this: *mut ACE_Allocator,
) -> libc::c_int {
    <ACE_New_Allocator>::remove((__this as *mut ACE_New_Allocator))
}
pub unsafe fn __vthunk_ou619e4e89eda90c3c_iu7bff1870c893b3fe(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut libc::c_void,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_New_Allocator>::bind((__this as *mut ACE_New_Allocator), p0, p1, p2)
}
pub unsafe fn __vthunk_oub7fe9fe6792a7efb_iu381e2dddd3465a71(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    <ACE_New_Allocator>::trybind((__this as *mut ACE_New_Allocator), p0, p1)
}
pub unsafe fn __vthunk_ou908137ee30d61e54_iu41a4de2216226892(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    <ACE_New_Allocator>::find((__this as *mut ACE_New_Allocator), p0, p1)
}
pub unsafe fn __vthunk_oud72abe9cd5ce064f_iu22342900ef7c0f5d(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
) -> libc::c_int {
    <ACE_New_Allocator>::find_ud72abe9cd5ce064f((__this as *mut ACE_New_Allocator), p0)
}
pub unsafe fn __vthunk_ouc8c7ce2ab1a4d01e_iuce55bcfdd7d4af38(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
) -> libc::c_int {
    <ACE_New_Allocator>::unbind((__this as *mut ACE_New_Allocator), p0)
}
pub unsafe fn __vthunk_ou9886c474d772c149_iu18ccae12f60528e3(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    <ACE_New_Allocator>::unbind_u9886c474d772c149(
        (__this as *mut ACE_New_Allocator),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ou7257c0b68a6f44a1_iu62da3684ac8c6bf7(
    __this: *mut ACE_Allocator,
    p0: libc::c_long,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_New_Allocator>::sync((__this as *mut ACE_New_Allocator), p0, p1)
}
pub unsafe fn __vthunk_ou882390cc14c47b90_iu7db10b00cce8fa5e(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_New_Allocator>::sync_u882390cc14c47b90(
        (__this as *mut ACE_New_Allocator),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou016c2ae7b468d245_iuada926f987c0415f(
    __this: *mut ACE_Allocator,
    p0: libc::c_long,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_New_Allocator>::protect((__this as *mut ACE_New_Allocator), p0, p1)
}
pub unsafe fn __vthunk_oucd829d73a2707ee4_iuf36ac9d4f584a786(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_New_Allocator>::protect_ucd829d73a2707ee4(
        (__this as *mut ACE_New_Allocator),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_oue242c36824ff7e5e_iuac8ca8237e7c8154(
    __this: *mut ACE_Allocator,
) {
    <ACE_New_Allocator>::dump((__this as *mut ACE_New_Allocator))
}
#[repr(C)]
pub struct __Vtbl_u8a5e72ffd9043ac9 {
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
pub static __VTBL_u8a5e72ffd9043ac9: __Vtbl_u8a5e72ffd9043ac9 = __Vtbl_u8a5e72ffd9043ac9 {
    __type_info: &__TYPEINFO_17ACE_New_Allocator,
    __vdtor: __vdtor_u8a5e72ffd9043ac9,
    vfn_u685215409e23bf32: __vthunk_oua823c7d088ad569c_iu685215409e23bf32,
    vfn_u0a9372cacdda8cbe: __vthunk_ou757910826b014974_iu0a9372cacdda8cbe,
    vfn_ued53ccfa62009d93: __vthunk_oub14cd9e591a2d69d_iued53ccfa62009d93,
    vfn_ucc7a27ee055bb87e: __vthunk_ou6e02a1c7f32efb3c_iucc7a27ee055bb87e,
    vfn_uaeedb459d846087b: __vthunk_ou5a1829294f889bed_iuaeedb459d846087b,
    vfn_u7bff1870c893b3fe: __vthunk_ou619e4e89eda90c3c_iu7bff1870c893b3fe,
    vfn_u381e2dddd3465a71: __vthunk_oub7fe9fe6792a7efb_iu381e2dddd3465a71,
    vfn_u41a4de2216226892: __vthunk_ou908137ee30d61e54_iu41a4de2216226892,
    vfn_u22342900ef7c0f5d: __vthunk_oud72abe9cd5ce064f_iu22342900ef7c0f5d,
    vfn_uce55bcfdd7d4af38: __vthunk_ouc8c7ce2ab1a4d01e_iuce55bcfdd7d4af38,
    vfn_u18ccae12f60528e3: __vthunk_ou9886c474d772c149_iu18ccae12f60528e3,
    vfn_u62da3684ac8c6bf7: __vthunk_ou7257c0b68a6f44a1_iu62da3684ac8c6bf7,
    vfn_u7db10b00cce8fa5e: __vthunk_ou882390cc14c47b90_iu7db10b00cce8fa5e,
    vfn_uada926f987c0415f: __vthunk_ou016c2ae7b468d245_iuada926f987c0415f,
    vfn_uf36ac9d4f584a786: __vthunk_oucd829d73a2707ee4_iuf36ac9d4f584a786,
    vfn_uac8ca8237e7c8154: __vthunk_oue242c36824ff7e5e_iuac8ca8237e7c8154,
};
pub unsafe fn __vdtor_uee81b96054c3f1a9(__this: *mut ACE_Allocator) {
    let _ = Box::from_raw(__this as *mut ACE_Static_Allocator_Base);
}
pub unsafe fn __vthunk_ouff88b63dcf892638_iu685215409e23bf32(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
) -> *mut libc::c_void {
    <ACE_Static_Allocator_Base>::malloc((__this as *mut ACE_Static_Allocator_Base), p0)
}
pub unsafe fn __vthunk_ou338bdc520fb0a5b8_iu0a9372cacdda8cbe(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
    p1: libc::c_char,
) -> *mut libc::c_void {
    <ACE_Static_Allocator_Base>::calloc(
        (__this as *mut ACE_Static_Allocator_Base),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ou816b60613ed1d3f9_iued53ccfa62009d93(
    __this: *mut ACE_Allocator,
    p0: libc::c_ulong,
    p1: libc::c_ulong,
    p2: libc::c_char,
) -> *mut libc::c_void {
    <ACE_Static_Allocator_Base>::calloc_u816b60613ed1d3f9(
        (__this as *mut ACE_Static_Allocator_Base),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_oue4852fddc171f668_iucc7a27ee055bb87e(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
) {
    <ACE_Static_Allocator_Base>::free((__this as *mut ACE_Static_Allocator_Base), p0)
}
pub unsafe fn __vthunk_ou53ade39d9a23f8d9_iuaeedb459d846087b(
    __this: *mut ACE_Allocator,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::remove((__this as *mut ACE_Static_Allocator_Base))
}
pub unsafe fn __vthunk_ou230553ec4974ded0_iu7bff1870c893b3fe(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut libc::c_void,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::bind(
        (__this as *mut ACE_Static_Allocator_Base),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou2d475b0d9af43e0f_iu381e2dddd3465a71(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::trybind(
        (__this as *mut ACE_Static_Allocator_Base),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_oufedc59b6913a8e60_iu41a4de2216226892(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::find((__this as *mut ACE_Static_Allocator_Base), p0, p1)
}
pub unsafe fn __vthunk_oueb49d4df81e0a60b_iu22342900ef7c0f5d(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::find_ueb49d4df81e0a60b(
        (__this as *mut ACE_Static_Allocator_Base),
        p0,
    )
}
pub unsafe fn __vthunk_ou8e03c1b37cd98cd2_iuce55bcfdd7d4af38(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::unbind((__this as *mut ACE_Static_Allocator_Base), p0)
}
pub unsafe fn __vthunk_oubd49d95e20a7b2cd_iu18ccae12f60528e3(
    __this: *mut ACE_Allocator,
    p0: *const libc::c_char,
    p1: *mut *mut libc::c_void,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::unbind_ubd49d95e20a7b2cd(
        (__this as *mut ACE_Static_Allocator_Base),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ou9995621f72a61935_iu62da3684ac8c6bf7(
    __this: *mut ACE_Allocator,
    p0: libc::c_long,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::sync((__this as *mut ACE_Static_Allocator_Base), p0, p1)
}
pub unsafe fn __vthunk_ou3c039f9aead40514_iu7db10b00cce8fa5e(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::sync_u3c039f9aead40514(
        (__this as *mut ACE_Static_Allocator_Base),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_oue9fd38d42042cfc9_iuada926f987c0415f(
    __this: *mut ACE_Allocator,
    p0: libc::c_long,
    p1: libc::c_int,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::protect(
        (__this as *mut ACE_Static_Allocator_Base),
        p0,
        p1,
    )
}
pub unsafe fn __vthunk_ou20867242cdfcbb98_iuf36ac9d4f584a786(
    __this: *mut ACE_Allocator,
    p0: *mut libc::c_void,
    p1: libc::c_ulong,
    p2: libc::c_int,
) -> libc::c_int {
    <ACE_Static_Allocator_Base>::protect_u20867242cdfcbb98(
        (__this as *mut ACE_Static_Allocator_Base),
        p0,
        p1,
        p2,
    )
}
pub unsafe fn __vthunk_ou36aed3e541c9cb42_iuac8ca8237e7c8154(
    __this: *mut ACE_Allocator,
) {
    <ACE_Static_Allocator_Base>::dump((__this as *mut ACE_Static_Allocator_Base))
}
#[repr(C)]
pub struct __Vtbl_uee81b96054c3f1a9 {
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
pub static __VTBL_uee81b96054c3f1a9: __Vtbl_uee81b96054c3f1a9 = __Vtbl_uee81b96054c3f1a9 {
    __type_info: &__TYPEINFO_25ACE_Static_Allocator_Base,
    __vdtor: __vdtor_uee81b96054c3f1a9,
    vfn_u685215409e23bf32: __vthunk_ouff88b63dcf892638_iu685215409e23bf32,
    vfn_u0a9372cacdda8cbe: __vthunk_ou338bdc520fb0a5b8_iu0a9372cacdda8cbe,
    vfn_ued53ccfa62009d93: __vthunk_ou816b60613ed1d3f9_iued53ccfa62009d93,
    vfn_ucc7a27ee055bb87e: __vthunk_oue4852fddc171f668_iucc7a27ee055bb87e,
    vfn_uaeedb459d846087b: __vthunk_ou53ade39d9a23f8d9_iuaeedb459d846087b,
    vfn_u7bff1870c893b3fe: __vthunk_ou230553ec4974ded0_iu7bff1870c893b3fe,
    vfn_u381e2dddd3465a71: __vthunk_ou2d475b0d9af43e0f_iu381e2dddd3465a71,
    vfn_u41a4de2216226892: __vthunk_oufedc59b6913a8e60_iu41a4de2216226892,
    vfn_u22342900ef7c0f5d: __vthunk_oueb49d4df81e0a60b_iu22342900ef7c0f5d,
    vfn_uce55bcfdd7d4af38: __vthunk_ou8e03c1b37cd98cd2_iuce55bcfdd7d4af38,
    vfn_u18ccae12f60528e3: __vthunk_oubd49d95e20a7b2cd_iu18ccae12f60528e3,
    vfn_u62da3684ac8c6bf7: __vthunk_ou9995621f72a61935_iu62da3684ac8c6bf7,
    vfn_u7db10b00cce8fa5e: __vthunk_ou3c039f9aead40514_iu7db10b00cce8fa5e,
    vfn_uada926f987c0415f: __vthunk_oue9fd38d42042cfc9_iuada926f987c0415f,
    vfn_uf36ac9d4f584a786: __vthunk_ou20867242cdfcbb98_iuf36ac9d4f584a786,
    vfn_uac8ca8237e7c8154: __vthunk_ou36aed3e541c9cb42_iuac8ca8237e7c8154,
};
