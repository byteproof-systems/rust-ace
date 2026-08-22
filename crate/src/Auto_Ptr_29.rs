#[allow(unused_imports)]
use crate::__common::*;
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
pub mod __gnu_debug {}
pub mod __cxxabiv1 {}
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
    pub fn __sysconf(__name: libc::c_int) -> libc::c_long;
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
pub mod __pstl {}
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
