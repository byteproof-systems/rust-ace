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
    pub fn alloca(__size: libc::c_ulong) -> *mut libc::c_void;
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
pub mod __gnu_debug {}
pub mod ACE_OS {
    pub use crate::full_ops_0::ACE_OS::log2_u389382349d30b71a;
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
