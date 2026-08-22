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
    ///This method computes the largest integral value not greater than x.
    #[export_name = "_ZN6ACE_OS5floorIeEET_S1_"]
    pub unsafe extern "C-unwind" fn floor___long_double_(
        mut x: crate::__f80::F80,
    ) -> crate::__f80::F80 {
        unsafe {
            {
                return super::floorl((x));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    ///This method computes the smallest integral value not less than x.
    #[export_name = "_ZN6ACE_OS4ceilIeEET_S1_"]
    pub unsafe extern "C-unwind" fn ceil___long_double_(
        mut x: crate::__f80::F80,
    ) -> crate::__f80::F80 {
        unsafe {
            {
                return super::ceill((x));
            }
            #[allow(unreachable_code)] { ::core::unreachable!() }
        }
    }
    pub use crate::full_ops_0::ACE_OS::log2_u389382349d30b71a;
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
    pub use crate::full_ops_0::ACE_OS::gettimeofday_u3220bcbbceb90f45;
    pub use crate::full_ops_0::ACE_OS::gettimeofday_;
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
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut ACE_Time_Value_zero: ACE_Time_Value = unsafe {
    ::core::mem::MaybeUninit::<ACE_Time_Value>::zeroed().assume_init()
};
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut ACE_Time_Value_max_time: ACE_Time_Value = unsafe {
    ::core::mem::MaybeUninit::<ACE_Time_Value>::zeroed().assume_init()
};
pub type float_type = crate::__f80::F80;
#[export_name = "_ZN14ACE_Time_ValuemLEd"]
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_Time_ValuemLEd(
    __this: *mut ACE_Time_Value,
    d: libc::c_double,
) -> *mut ACE_Time_Value {
    unsafe { ACE_Time_Value::operator_mul_assign(__this, d) }
}
#[export_name = "_ZN14ACE_Time_ValueppEi"]
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_Time_ValueppEi(
    __this: *mut ACE_Time_Value,
    _anon_0: libc::c_int,
) -> ACE_Time_Value {
    unsafe { ACE_Time_Value::operator_inc(__this, _anon_0) }
}
#[export_name = "_ZN14ACE_Time_ValueppEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_Time_ValueppEv(
    __this: *mut ACE_Time_Value,
) -> *mut ACE_Time_Value {
    unsafe { ACE_Time_Value::operator_inc_uc33e9bd9a97b376d(__this) }
}
#[export_name = "_ZN14ACE_Time_ValuemmEi"]
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_Time_ValuemmEi(
    __this: *mut ACE_Time_Value,
    _anon_0: libc::c_int,
) -> ACE_Time_Value {
    unsafe { ACE_Time_Value::operator_dec(__this, _anon_0) }
}
#[export_name = "_ZN14ACE_Time_ValuemmEv"]
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_Time_ValuemmEv(
    __this: *mut ACE_Time_Value,
) -> *mut ACE_Time_Value {
    unsafe { ACE_Time_Value::operator_dec_ub2d9c9d9a075c40d(__this) }
}
#[export_name = "_ZNK14ACE_Time_Value3nowEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK14ACE_Time_Value3nowEv(
    __this: *const ACE_Time_Value,
) -> ACE_Time_Value {
    unsafe { ACE_Time_Value::now(__this) }
}
#[export_name = "_ZNK14ACE_Time_Value16to_relative_timeEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK14ACE_Time_Value16to_relative_timeEv(
    __this: *const ACE_Time_Value,
) -> ACE_Time_Value {
    unsafe { ACE_Time_Value::to_relative_time(__this) }
}
#[export_name = "_ZNK14ACE_Time_Value16to_absolute_timeEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK14ACE_Time_Value16to_absolute_timeEv(
    __this: *const ACE_Time_Value,
) -> ACE_Time_Value {
    unsafe { ACE_Time_Value::to_absolute_time(__this) }
}
#[export_name = "_ZNK14ACE_Time_Value9duplicateEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK14ACE_Time_Value9duplicateEv(
    __this: *const ACE_Time_Value,
) -> *mut ACE_Time_Value {
    unsafe { ACE_Time_Value::duplicate(__this) }
}
#[export_name = "_ZNK14ACE_Time_Value4dumpEv"]
pub unsafe extern "C-unwind" fn __xtu__ZNK14ACE_Time_Value4dumpEv(
    __this: *const ACE_Time_Value,
) {
    unsafe { ACE_Time_Value::dump(__this) }
}
#[export_name = "_ZN14ACE_Time_Value9normalizeEb"]
pub unsafe extern "C-unwind" fn __xtu__ZN14ACE_Time_Value9normalizeEb(
    __this: *mut ACE_Time_Value,
    saturate: bool,
) {
    unsafe { ACE_Time_Value::normalize(__this, saturate) }
}
#[export_name = "_ZlsRSoRK14ACE_Time_Value"]
pub unsafe extern "C-unwind" fn operator_shl_u87aa6c49d2c7f15d(
    mut o: *mut crate::__cxx_std::Ostream,
    mut v: *const ACE_Time_Value,
) -> *mut crate::__cxx_std::Ostream {
    unsafe {
        {
            let mut oldFiller: libc::c_char = (((((*o)).fill())) as libc::c_char);
            ((*o)).set_fill((48 as libc::c_char) as libc::c_char);
            let mut tv: *const timeval = <ACE_Time_Value>::operator_const_timeval__(
                (::core::ptr::addr_of!((* v))) as *const ACE_Time_Value,
            );
            if (((*tv).tv_sec) != 0) {
                ((*o)).put_long(((*tv).tv_sec) as libc::c_long);
                if (((*tv).tv_usec) != 0) {
                    ((((*o)).put_char((46 as libc::c_char) as libc::c_char))
                        .setw((6) as libc::c_int))
                        .put_long(
                            (labs((((*tv).tv_usec) as libc::c_long))) as libc::c_long,
                        );
                }
            } else {
                if ((((((*tv).tv_usec as libc::c_long)) < (((0) as libc::c_long)))
                    as libc::c_int as libc::c_int) != 0)
                {
                    ((((*o))
                        .put_cstr(
                            (b"-0.\0".as_ptr() as *const libc::c_char)
                                as *const libc::c_char,
                        ))
                        .setw((6) as libc::c_int))
                        .put_long(((-((*tv).tv_usec))) as libc::c_long);
                } else {
                    ((*o)).put_char((48 as libc::c_char) as libc::c_char);
                    if ((((((*tv).tv_usec as libc::c_long)) > (((0) as libc::c_long)))
                        as libc::c_int as libc::c_int) != 0)
                    {
                        ((((*o)).put_char((46 as libc::c_char) as libc::c_char))
                            .setw((6) as libc::c_int))
                            .put_long(((*tv).tv_usec) as libc::c_long);
                    }
                }
            }
            ((*o)).set_fill((oldFiller) as libc::c_char);
            return ::core::ptr::addr_of_mut!((* o));
        }
        #[allow(unreachable_code)] { ::core::unreachable!() }
    }
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
pub mod __gnu_debug {}
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
#[doc = "* @class ACE_HR_Time_Policy\n *\n * @brief Implement a time policy based on the ACE Highres timer."]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACE_HR_Time_Policy {}
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
pub mod __cxxabiv1 {}
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
    pub fn __builtin_strlen(_anon_0: *const libc::c_char) -> libc::c_ulong;
}
extern "C-unwind" {
    pub fn __builtin_strcmp(
        _anon_0: *const libc::c_char,
        _anon_1: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C-unwind" {
    pub fn __builtin_memcmp(
        _anon_0: *const libc::c_void,
        _anon_1: *const libc::c_void,
        _anon_2: libc::c_ulong,
    ) -> libc::c_int;
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
    pub fn __builtin_expect(
        _anon_0: libc::c_long,
        _anon_1: libc::c_long,
    ) -> libc::c_long;
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
#[allow(non_snake_case)]
unsafe extern "C-unwind" fn __rt_static_init() {
    ::core::ptr::write(
        ::core::ptr::addr_of_mut!(ACE_Time_Value_zero),
        <ACE_Time_Value>::new(),
    );
    ::core::ptr::write(
        ::core::ptr::addr_of_mut!(ACE_Time_Value_max_time),
        <ACE_Time_Value>::new_ub59bcc88eaedf2a6(
            ((<ACE_Numeric_Limits_long_>::max()) as libc::c_long),
            (((ACE_ONE_SECOND_IN_USECS) as libc::c_long))
                .wrapping_sub((1) as libc::c_long),
        ),
    );
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    link_section = "__DATA,__mod_init_func"
)]
static __RT_STATIC_CTOR: [unsafe extern "C-unwind" fn(); 1] = [__rt_static_init];
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
