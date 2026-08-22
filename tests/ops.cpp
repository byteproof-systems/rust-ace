// C-ABI surface for the FULL-ACE assembly differential target: every
// ace/*.cpp translation unit of ACE 6.5.24 that compiles standalone on this
// platform is translated together into ONE crate, and this ops TU
// (headers only — every callee's definition lives in one of those 405 TUs)
// drives deterministic subsystems through genuine cross-TU calls. The native
// side compiles the same 405 TUs with clang++ and links the same driver, so
// each op's integer is a full-library-scale differential: the multi-TU dedup
// of the entire ACE header surface, the whole library's static initialization,
// and the exercised subsystems must all agree byte-for-byte with native.
//
// The op surface grows increment by increment (each addition re-certified);
// v1 pins the subsystems already proven TU-locally by ace/ace_crc32/
// ace_crcccitt/ace_timevalue/ace_base64/ace_multitu, now running inside the
// full-library assembly.
#include "ace/ACE.h"
#include "ace/CDR_Base.h"
#include "ace/Codecs.h"
#include "ace/Time_Value.h"

#include <string.h>

// Order-sensitive FNV-style checksum, as in the sibling ACE targets.
static int ace_cksum (const unsigned char *p, unsigned long n)
{
  long c = 1469598103;
  for (unsigned long i = 0; i < n; ++i)
    c = (c * 131 + p[i] + (long) i) & 0x7fffffff;
  return (int) (c & 0x7fffffff);
}

static void fill_pattern (unsigned char *p, unsigned long n)
{
  for (unsigned long i = 0; i < n; ++i)
    p[i] = (unsigned char) (0x10 + 7 * i);
}

static const char vector_text[] =
  "ACE 6.5.24 FULL-library assembly differential test vector (405 TUs)";

// CRC-32 (ace/ACE_crc32.cpp).
extern "C" int op_full_crc32 (void)
{
  return (int) (ACE::crc32 (vector_text) & 0x7fffffff);
}

// CRC-CCITT (ace/ACE_crc_ccitt.cpp).
extern "C" int op_full_crcccitt (void)
{
  return (int) (ACE::crc_ccitt (vector_text) & 0x7fffffff);
}

// Base64 chunked round-trip (ace/Codecs.cpp): encode, decode through the
// whitespace-skipping path, verify exact reproduction.
extern "C" int op_full_b64 (void)
{
  unsigned char buf[257];
  for (int i = 0; i < 257; ++i)
    buf[i] = (unsigned char) (251 - (i * 11) % 253);
  size_t enc_len = 0;
  ACE_Byte *enc = ACE_Base64::encode (buf, sizeof buf, &enc_len, true);
  if (!enc)
    return -1;
  size_t dec_len = 0;
  ACE_Byte *dec = ACE_Base64::decode (enc, &dec_len);
  int ok = dec != 0 && dec_len == sizeof buf
           && memcmp (dec, buf, sizeof buf) == 0;
  int c = ace_cksum (enc, (unsigned long) enc_len);
  delete[] enc;
  delete[] dec;
  return ok ? (c ^ (int) enc_len) : -1;
}

// ACE_Time_Value normalization + arithmetic (ace/Time_Value.cpp).
extern "C" int op_full_tv (void)
{
  ACE_Time_Value a (2, 3500000);
  ACE_Time_Value b (0, 999999);
  a += b;
  ACE_Time_Value c (5, 100);
  c -= ACE_Time_Value (2, 200);
  int cmp = (a > c) ? 1 : 0;
  return (int) ((long) a.sec () * 10000000 + (long) a.usec () + cmp
                + (long) c.usec () % 1009);
}

// CDR byte swaps (ace/CDR_Base.cpp): the 2/4/8/16 single-value forms chained
// into one checksum.
extern "C" int op_full_cdr (void)
{
  unsigned char in[16], out[16];
  fill_pattern (in, 16);
  unsigned char acc[16 + 8 + 4 + 2];
  ACE_CDR::swap_16 ((const char *) in, (char *) out);
  memcpy (acc, out, 16);
  ACE_CDR::swap_8 ((const char *) in, (char *) out);
  memcpy (acc + 16, out, 8);
  ACE_CDR::swap_4 ((const char *) in, (char *) out);
  memcpy (acc + 24, out, 4);
  ACE_CDR::swap_2 ((const char *) in, (char *) out);
  memcpy (acc + 28, out, 2);
  return ace_cksum (acc, sizeof acc);
}

// Four-subsystem chain (base64 -> CRC-32 + CRC-CCITT -> Time_Value), the
// ace_multitu cross op, now inside the full-library assembly.
extern "C" int op_full_cross (void)
{
  unsigned char buf[129];
  for (int i = 0; i < 129; ++i)
    buf[i] = (unsigned char) (i * 5 + 11);
  size_t enc_len = 0;
  ACE_Byte *enc = ACE_Base64::encode (buf, sizeof buf, &enc_len, true);
  if (!enc)
    return -1;
  unsigned crc = ACE::crc32 ((const void *) enc, enc_len, 0);
  unsigned ccitt = ACE::crc_ccitt ((const void *) enc, enc_len, 0);
  delete[] enc;
  ACE_Time_Value t (0, (suseconds_t) (crc % 3000000));
  return (int) (((crc ^ (ccitt << 7)) & 0x3fffffff)
                + (long) t.sec () * 3 + (long) t.usec () % 1009);
}

// ===========================================================================
// v2: comprehensive per-TU coverage, batch 1 — the ACE_OS portability layer
// (OS_NS_* components) plus strings, containers, and smart pointers. Every op
// drives real ACE API through genuine cross-TU calls and folds every
// observable into an order-sensitive checksum. Determinism contract: values
// that vary per HOST (kernel release, uid) are stable across the native/Rust
// pair (same process environment); values that vary per RUN (pids, times,
// pointers, ports) are never folded — only derived invariants of them are.
// ===========================================================================
#include "ace/OS_NS_string.h"
#include "ace/OS_NS_strings.h"
#include "ace/OS_NS_stdio.h"
#include "ace/OS_NS_stdlib.h"
#include "ace/OS_NS_ctype.h"
#include "ace/OS_NS_wctype.h"
#include "ace/OS_NS_time.h"
#include "ace/OS_NS_math.h"
#include "ace/OS_NS_regex.h"
#include "ace/OS_NS_dirent.h"
#include "ace/Dirent_Selector.h"
#include "ace/OS_NS_pwd.h"
#include "ace/OS_NS_sys_utsname.h"
#include "ace/OS_NS_errno.h"
#include "ace/OS_NS_fcntl.h"
#include "ace/OS_NS_unistd.h"
#include "ace/OS_NS_sys_stat.h"
#include "ace/OS_NS_sys_time.h"
#include "ace/OS_NS_sys_uio.h"
#include "ace/OS_NS_poll.h"
#include "ace/OS_NS_sys_select.h"
#include "ace/OS_NS_signal.h"
#include "ace/OS_NS_sys_socket.h"
#include "ace/OS_NS_arpa_inet.h"
#include "ace/OS_NS_netdb.h"
#include "ace/OS_NS_sys_mman.h"
#include "ace/OS_NS_sys_resource.h"
#include "ace/OS_NS_sys_wait.h"
#include "ace/OS_NS_dlfcn.h"
#include "ace/OS_NS_sys_shm.h"
#include "ace/OS_NS_sys_msg.h"
#include "ace/OS_NS_Thread.h"
#include "ace/OS_NS_stropts.h"
#include "ace/OS_NS_sys_sendfile.h"
#include "ace/OS_NS_wchar.h"
#include "ace/Handle_Set.h"
#include "ace/Global_Macros.h"

// Fold one long into the running checksum (order-sensitive).
static long ck_fold (long c, long v)
{
  return (c * 1315423911L + v + 17) & 0x3fffffffL;
}

static long ck_str (long c, const char *s)
{
  if (!s)
    return ck_fold (c, -9601);
  for (const char *p = s; *p; ++p)
    c = ck_fold (c, (long) (unsigned char) *p);
  return ck_fold (c, -77);
}

// ACE_OS string families: OS_NS_string (strsncpy/strdup/strtok_r/strpbrk...)
// and OS_NS_strings (strcasecmp/strncasecmp).
extern "C" int op_full_os_string (void)
{
  long c = 7;
  char buf[64];
  ACE_OS::strsncpy (buf, "portability-layer", 8);   // truncating copy
  c = ck_str (c, buf);
  c = ck_fold (c, (long) ACE_OS::strlen (buf));
  char *dup = ACE_OS::strdup ("ACE_OS::strdup vector");
  c = ck_str (c, dup);
  ACE_OS::free (dup);
  c = ck_fold (c, ACE_OS::strcasecmp ("MiXeD", "mixed"));
  c = ck_fold (c, ACE_OS::strcasecmp ("alpha", "beta") < 0 ? 1 : 2);
  c = ck_fold (c, ACE_OS::strncasecmp ("PREfix-xx", "prefix-yy", 7));
  char tokbuf[] = "a:bb;ccc:dddd";
  char *save = 0;
  for (char *t = ACE_OS::strtok_r (tokbuf, ":;", &save); t;
       t = ACE_OS::strtok_r (0, ":;", &save))
    c = ck_str (c, t);
  const char *hay = "find-the-needle-in-here";
  c = ck_fold (c, (long) (ACE_OS::strstr (hay, "needle") - hay));
  c = ck_fold (c, (long) (ACE_OS::strchr (hay, 'n') - hay));
  c = ck_fold (c, (long) (ACE_OS::strrchr (hay, 'e') - hay));
  c = ck_fold (c, (long) (ACE_OS::strpbrk (hay, "xzt") - hay));
  c = ck_fold (c, (long) ACE_OS::strspn ("aabbccdd", "ab"));
  char ecpy[32];
  char *end = ACE_OS::strecpy (ecpy, "strecpy");   // returns end pointer
  c = ck_fold (c, (long) (end - ecpy));
  c = ck_str (c, ecpy);
  return (int) c;
}

// OS_NS_stdio: formatted output and FILE round-trip through a temp file.
extern "C" int op_full_os_stdio (void)
{
  long c = 11;
  char fmt[128];
  c = ck_fold (c, ACE_OS::snprintf (fmt, sizeof fmt, "%d|%06.2f|%s|%x|%c",
                                    -1234, 3.14159, "fmt", 0xbeef, 'Q'));
  c = ck_str (c, fmt);
  char path[] = "/tmp/ace_full_stdio_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  if (fd == ACE_INVALID_HANDLE)
    return -1;
  ACE_OS::close (fd);
  FILE *f = ACE_OS::fopen (path, "w+");
  if (!f)
    return -2;
  for (int i = 0; i < 10; ++i)
    ACE_OS::fprintf (f, "line-%02d %d\n", i, i * i * 7);
  c = ck_fold (c, ACE_OS::fflush (f));
  c = ck_fold (c, (long) ACE_OS::ftell (f));
  ACE_OS::rewind (f);
  char line[64];
  while (ACE_OS::fgets (line, sizeof line, f))
    c = ck_str (c, line);
  ACE_OS::fseek (f, 8, SEEK_SET);
  c = ck_fold (c, ACE_OS::fgetc (f));
  ACE_OS::fclose (f);
  f = ACE_OS::fopen (path, "r");
  unsigned char raw[512];
  size_t n = ACE_OS::fread (raw, 1, sizeof raw, f);
  c = ck_fold (c, (long) n);
  c = ck_fold (c, ace_cksum (raw, (unsigned long) n));
  ACE_OS::fclose (f);
  ACE_OS::unlink (path);
  return (int) c;
}

// OS_NS_stdlib: conversions, sorting, searching, environment, realpath.
static int cmp_ints (const void *a, const void *b)
{
  return *(const int *) a - *(const int *) b;
}
extern "C" int op_full_os_stdlib (void)
{
  long c = 13;
  c = ck_fold (c, ACE_OS::atoi ("-4821"));
  c = ck_fold (c, (long) ACE_OS::strtol ("7fff", 0, 16));
  c = ck_fold (c, (long) ACE_OS::strtoul ("3000000000", 0, 10) % 100003L);
  char *ep = 0;
  double d = ACE_OS::strtod ("2.5e3xyz", &ep);
  c = ck_fold (c, (long) d);
  c = ck_str (c, ep);
  char ibuf[40];
  c = ck_str (c, ACE_OS::itoa (48879, ibuf, 16));
  c = ck_str (c, ACE_OS::itoa (-1234, ibuf, 10));
  c = ck_str (c, ACE_OS::itoa (255, ibuf, 2));
  int v[16];
  for (int i = 0; i < 16; ++i)
    v[i] = (i * 37) % 23;
  ACE_OS::qsort (v, 16, sizeof (int), cmp_ints);
  for (int i = 0; i < 16; ++i)
    c = ck_fold (c, v[i]);
  int key = 14;
  int *hit = (int *) ACE_OS::bsearch (&key, v, 16, sizeof (int), cmp_ints);
  c = ck_fold (c, hit ? (long) (hit - v) : -1);
  c = ck_fold (c, ACE_OS::setenv ("ACE_FULL_ENV_PROBE", "value-42", 1));
  c = ck_str (c, ACE_OS::getenv ("ACE_FULL_ENV_PROBE"));
  char rbuf[512];
  c = ck_str (c, ACE_OS::realpath ("/usr/./bin/../bin", rbuf));
  unsigned seed = 20260716;
  for (int i = 0; i < 5; ++i)
    c = ck_fold (c, ACE_OS::rand_r (&seed) & 0xffff);
  return (int) c;
}

// OS_NS_ctype + OS_NS_wctype: full classification sweep.
extern "C" int op_full_os_ctype (void)
{
  long c = 17;
  for (int ch = 0; ch < 128; ++ch)
    {
      long bits = (ACE_OS::ace_isalpha (ch) ? 1 : 0)
                  | (ACE_OS::ace_isdigit (ch) ? 2 : 0)
                  | (ACE_OS::ace_isspace (ch) ? 4 : 0)
                  | (ACE_OS::ace_isupper (ch) ? 8 : 0)
                  | (ACE_OS::ace_islower (ch) ? 16 : 0)
                  | (ACE_OS::ace_ispunct (ch) ? 32 : 0)
                  | (ACE_OS::ace_isxdigit (ch) ? 64 : 0)
                  | (ACE_OS::ace_isalnum (ch) ? 128 : 0)
                  | (ACE_OS::ace_iscntrl (ch) ? 256 : 0)
                  | (ACE_OS::ace_isgraph (ch) ? 512 : 0)
                  | (ACE_OS::ace_isprint (ch) ? 1024 : 0);
      c = ck_fold (c, bits + ACE_OS::ace_tolower (ch) * 3
                        + ACE_OS::ace_toupper (ch) * 7);
    }
  wctype_t walpha = wctype ("alpha");
  wctype_t wdigit = wctype ("digit");
  for (wint_t wc = 0x40; wc < 0x80; ++wc)
    c = ck_fold (c, (ACE_OS::ace_iswctype (wc, walpha) ? 1 : 0)
                    + (ACE_OS::ace_iswctype (wc, wdigit) ? 2 : 0));
  return (int) c;
}

// OS_NS_time: calendar math on FIXED epoch values (TZ is UTC in the test
// environment; both sides see the identical zone database).
extern "C" int op_full_os_time (void)
{
  long c = 19;
  time_t fixed = (time_t) 1700000000;   // 2023-11-14T22:13:20Z
  struct tm tmv;
  ACE_OS::gmtime_r (&fixed, &tmv);
  c = ck_fold (c, tmv.tm_year);
  c = ck_fold (c, tmv.tm_mon);
  c = ck_fold (c, tmv.tm_mday);
  c = ck_fold (c, tmv.tm_hour);
  c = ck_fold (c, tmv.tm_min);
  c = ck_fold (c, tmv.tm_sec);
  c = ck_fold (c, tmv.tm_wday);
  c = ck_fold (c, tmv.tm_yday);
  char sbuf[64];
  c = ck_fold (c, (long) ACE_OS::strftime (sbuf, sizeof sbuf,
                                           "%Y-%m-%d %H:%M:%S", &tmv));
  c = ck_str (c, sbuf);
  struct tm lt;
  ACE_OS::localtime_r (&fixed, &lt);
  c = ck_fold (c, lt.tm_hour);          // UTC zone: equals gmtime hour
  time_t back = ACE_OS::mktime (&lt);
  c = ck_fold (c, (long) (back - fixed));
  c = ck_fold (c, (long) ACE_OS::difftime (fixed + 86461, fixed));
  time_t t1 = ACE_OS::time (0);
  time_t t2 = ACE_OS::time (0);
  c = ck_fold (c, t2 >= t1 ? 1 : 0);    // monotonic invariant only
  return (int) c;
}

// OS_NS_math: floating-point primitives.
extern "C" int op_full_os_math (void)
{
  long c = 23;
  const double xs[] = { -2.75, -0.5, 0.0, 0.4999, 1.5, 3.25, 1e9 + 0.7 };
  for (unsigned i = 0; i < sizeof xs / sizeof xs[0]; ++i)
    {
      c = ck_fold (c, (long) ACE_OS::floor (xs[i]));
      c = ck_fold (c, (long) ACE_OS::ceil (xs[i]));
    }
  return (int) c;
}

// OS_NS_regex: POSIX regcomp/regexec through the ACE wrappers.
extern "C" int op_full_os_regex (void)
{
  long c = 29;
  regex_t re;
  if (regcomp (&re, "^ab*c[0-9]+$", REG_EXTENDED) != 0)
    return -1;
  const char *probes[] = { "ac7", "abbbc123", "abc", "xabc1", "abbc42x" };
  for (unsigned i = 0; i < sizeof probes / sizeof probes[0]; ++i)
    c = ck_fold (c, regexec (&re, probes[i], 0, 0, 0) == 0 ? 1 : 0);
  regfree (&re);
  return (int) c;
}

// OS_NS_dirent + Dirent + Dirent_Selector: directory enumeration over a
// tree this op creates (names folded in sorted order).
extern "C" int op_full_os_dirent (void)
{
  long c = 31;
  char droot[] = "/tmp/ace_full_dirent_XXXXXX";
  if (!::mkdtemp (droot))
    return -1;
  char p[512];
  static const char *names[] = { "zeta.txt", "alpha.txt", "mid.dat" };
  for (int i = 0; i < 3; ++i)
    {
      ACE_OS::snprintf (p, sizeof p, "%s/%s", droot, names[i]);
      ACE_HANDLE fd = ACE_OS::open (p, O_CREAT | O_WRONLY, 0644);
      ACE_OS::write (fd, "x", 1);
      ACE_OS::close (fd);
    }
  ACE_DIR *dir = ACE_OS::opendir (droot);
  if (!dir)
    return -2;
  char seen[8][64];
  int nseen = 0;
  for (ACE_DIRENT *e; (e = ACE_OS::readdir (dir)) != 0;)
    if (e->d_name[0] != '.')
      ACE_OS::strsncpy (seen[nseen++], e->d_name, 64);
  ACE_OS::closedir (dir);
  // sort collected names (insertion order is fs-dependent)
  for (int i = 0; i < nseen; ++i)
    for (int j = i + 1; j < nseen; ++j)
      if (ACE_OS::strcmp (seen[j], seen[i]) < 0)
        {
          char t[64];
          ACE_OS::strsncpy (t, seen[i], 64);
          ACE_OS::strsncpy (seen[i], seen[j], 64);
          ACE_OS::strsncpy (seen[j], t, 64);
        }
  c = ck_fold (c, nseen);
  for (int i = 0; i < nseen; ++i)
    c = ck_str (c, seen[i]);
  // ACE_Dirent_Selector: scandir with alphasort (deterministic order).
  ACE_Dirent_Selector sel;
  if (sel.open (ACE_TEXT_CHAR_TO_TCHAR (droot), 0, ACE_SCANDIR_COMPARATOR (0)) != -1)
    {
      c = ck_fold (c, sel.length ());
      sel.close ();
    }
  for (int i = 0; i < 3; ++i)
    {
      ACE_OS::snprintf (p, sizeof p, "%s/%s", droot, names[i]);
      ACE_OS::unlink (p);
    }
  ACE_OS::rmdir (droot);
  return (int) c;
}

// OS_NS_pwd: password database (root is uid 0 everywhere).
extern "C" int op_full_os_pwd (void)
{
  long c = 37;
  struct passwd pw;
  char pbuf[1024];
  struct passwd *res = 0;
  if (ACE_OS::getpwnam_r ("root", &pw, pbuf, sizeof pbuf, &res) == 0 ? res == 0 : 1)
    return -1;
  c = ck_fold (c, (long) res->pw_uid);
  c = ck_str (c, res->pw_name);
  return (int) c;
}

// OS_NS_sys_utsname: host identity (stable across the native/Rust pair).
extern "C" int op_full_os_uname (void)
{
  long c = 41;
  ACE_utsname u;
  if (ACE_OS::uname (&u) < 0)
    return -1;
  c = ck_str (c, u.sysname);   // "Linux"
  c = ck_str (c, u.machine);   // e.g. "x86_64"
  return (int) c;
}

// OS_NS_errno: thread-local error propagation.
extern "C" int op_full_os_errno (void)
{
  long c = 43;
  ACE_OS::last_error (ENOENT);
  c = ck_fold (c, ACE_OS::last_error ());
  ACE_OS::last_error (0);
  ACE_OS::open ("/nonexistent/ace/full/path", O_RDONLY);
  c = ck_fold (c, ACE_OS::last_error () == ENOENT ? 1 : 0);
  return (int) c;
}

// OS_NS_fcntl + OS_NS_unistd: descriptor I/O, seeking, pipes, dup, fcntl.
extern "C" int op_full_os_fdio (void)
{
  long c = 47;
  char path[] = "/tmp/ace_full_fdio_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  if (fd == ACE_INVALID_HANDLE)
    return -1;
  unsigned char pat[256];
  fill_pattern (pat, sizeof pat);
  c = ck_fold (c, (long) ACE_OS::write (fd, pat, sizeof pat));
  c = ck_fold (c, (long) ACE_OS::lseek (fd, 64, SEEK_SET));
  unsigned char rd[64];
  c = ck_fold (c, (long) ACE_OS::read (fd, rd, sizeof rd));
  c = ck_fold (c, ace_cksum (rd, sizeof rd));
  c = ck_fold (c, (long) ACE_OS::lseek (fd, 0, SEEK_END));
  c = ck_fold (c, ACE_OS::ftruncate (fd, 100));
  c = ck_fold (c, (long) ACE_OS::lseek (fd, 0, SEEK_END));
  ACE_HANDLE d2 = ACE_OS::dup (fd);
  c = ck_fold (c, d2 != ACE_INVALID_HANDLE ? 1 : 0);
  ACE_OS::close (d2);
  int flags = ACE_OS::fcntl (fd, F_GETFL);
  c = ck_fold (c, flags & O_ACCMODE);
  ACE_OS::close (fd);
  c = ck_fold (c, ACE_OS::access (path, F_OK));
  c = ck_fold (c, ACE_OS::unlink (path));
  c = ck_fold (c, ACE_OS::access (path, F_OK));
  ACE_HANDLE pfd[2];
  if (ACE_OS::pipe (pfd) == 0)
    {
      c = ck_fold (c, (long) ACE_OS::write (pfd[1], "pipe-vec", 8));
      char pb[16];
      c = ck_fold (c, (long) ACE_OS::read (pfd[0], pb, 8));
      pb[8] = 0;
      c = ck_str (c, pb);
      c = ck_fold (c, ACE_OS::isatty (pfd[0]));
      ACE_OS::close (pfd[0]);
      ACE_OS::close (pfd[1]);
    }
  c = ck_fold (c, ACE_OS::getpagesize () == 4096 ? 1 : 2);
  c = ck_fold (c, ACE_OS::getpid () > 0 ? 1 : 0);
  return (int) c;
}

// OS_NS_sys_stat: metadata of files this op creates (umask pinned).
extern "C" int op_full_os_stat (void)
{
  long c = 53;
  ACE_OS::umask (022);
  char path[] = "/tmp/ace_full_stat_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  if (fd == ACE_INVALID_HANDLE)
    return -1;
  ACE_OS::write (fd, "0123456789abcdef", 16);
  ACE_OS::close (fd);
  ACE_stat st;
  if (ACE_OS::stat (path, &st) != 0)
    return -2;
  c = ck_fold (c, (long) st.st_size);
  c = ck_fold (c, S_ISREG (st.st_mode) ? 1 : 0);
  c = ck_fold (c, (long) ACE_OS::filesize (ACE_TEXT_CHAR_TO_TCHAR (path)));
  char dpath[512];
  ACE_OS::snprintf (dpath, sizeof dpath, "%s.d", path);
  c = ck_fold (c, ACE_OS::mkdir (ACE_TEXT_CHAR_TO_TCHAR (dpath), 0755));
  ACE_stat ds;
  ACE_OS::stat (dpath, &ds);
  c = ck_fold (c, S_ISDIR (ds.st_mode) ? 1 : 0);
  c = ck_fold (c, (long) (ds.st_mode & 0777));
  c = ck_fold (c, ACE_OS::rmdir (dpath));
  ACE_OS::unlink (path);
  return (int) c;
}

// OS_NS_sys_time: monotonicity invariant only (never fold raw time).
extern "C" int op_full_os_gettod (void)
{
  long c = 59;
  ACE_Time_Value t1 = ACE_OS::gettimeofday ();
  ACE_Time_Value t2 = ACE_OS::gettimeofday ();
  c = ck_fold (c, t2 >= t1 ? 1 : 0);
  c = ck_fold (c, t1.sec () > 1600000000 ? 1 : 0);   // sane wall clock
  return (int) c;
}

// OS_NS_sys_uio: scatter/gather over a pipe.
extern "C" int op_full_os_uio (void)
{
  long c = 61;
  ACE_HANDLE pfd[2];
  if (ACE_OS::pipe (pfd) != 0)
    return -1;
  char a[] = "vector-one|", b[] = "two|", d[] = "and-three";
  iovec iov[3];
  iov[0].iov_base = a; iov[0].iov_len = sizeof a - 1;
  iov[1].iov_base = b; iov[1].iov_len = sizeof b - 1;
  iov[2].iov_base = d; iov[2].iov_len = sizeof d - 1;
  c = ck_fold (c, (long) ACE_OS::writev (pfd[1], iov, 3));
  char r1[8], r2[64];
  iovec riov[2];
  riov[0].iov_base = r1; riov[0].iov_len = sizeof r1;
  riov[1].iov_base = r2; riov[1].iov_len = sizeof r2;
  ssize_t n = ACE_OS::readv (pfd[0], riov, 2);
  c = ck_fold (c, (long) n);
  c = ck_fold (c, ace_cksum ((unsigned char *) r1, 8));
  c = ck_fold (c, ace_cksum ((unsigned char *) r2, (unsigned long) (n - 8)));
  ACE_OS::close (pfd[0]);
  ACE_OS::close (pfd[1]);
  return (int) c;
}

// OS_NS_poll + OS_NS_sys_select + Handle_Set: readiness on a primed pipe.
extern "C" int op_full_os_poll (void)
{
  long c = 67;
  ACE_HANDLE pfd[2];
  if (ACE_OS::pipe (pfd) != 0)
    return -1;
  ACE_OS::write (pfd[1], "!", 1);
  pollfd pf;
  pf.fd = pfd[0];
  pf.events = POLLIN;
  pf.revents = 0;
  c = ck_fold (c, ACE_OS::poll (&pf, 1));
  c = ck_fold (c, (pf.revents & POLLIN) ? 1 : 0);
  ACE_Handle_Set hs;
  hs.set_bit (pfd[0]);
  c = ck_fold (c, hs.num_set ());
  c = ck_fold (c, hs.is_set (pfd[0]) ? 1 : 0);
  ACE_Time_Value zero (0, 0);
  int nr = ACE_OS::select ((int) pfd[0] + 1, hs.fdset (), 0, 0, &zero);
  c = ck_fold (c, nr);
  ACE_Handle_Set empty_hs;
  ACE_Time_Value tick (0, 1000);
  c = ck_fold (c, ACE_OS::select (1, empty_hs.fdset (), 0, 0, &tick));
  hs.clr_bit (pfd[0]);
  c = ck_fold (c, hs.num_set ());
  ACE_OS::close (pfd[0]);
  ACE_OS::close (pfd[1]);
  return (int) c;
}

// OS_NS_signal: mask surgery and self-probe.
extern "C" int op_full_os_signal (void)
{
  long c = 71;
  sigset_t s;
  ACE_OS::sigemptyset (&s);
  ACE_OS::sigaddset (&s, SIGUSR1);
  ACE_OS::sigaddset (&s, SIGTERM);
  c = ck_fold (c, ACE_OS::sigismember (&s, SIGUSR1));
  c = ck_fold (c, ACE_OS::sigismember (&s, SIGINT));
  ACE_OS::sigdelset (&s, SIGUSR1);
  c = ck_fold (c, ACE_OS::sigismember (&s, SIGUSR1));
  ACE_OS::sigfillset (&s);
  c = ck_fold (c, ACE_OS::sigismember (&s, SIGHUP));
  c = ck_fold (c, ACE_OS::kill (ACE_OS::getpid (), 0));
  return (int) c;
}

// OS_NS_sys_socket + OS_NS_arpa_inet: socketpair round-trip and address
// text conversions.
extern "C" int op_full_os_socket (void)
{
  long c = 73;
  ACE_HANDLE sv[2];
  if (ACE_OS::socketpair (AF_UNIX, SOCK_STREAM, 0, sv) != 0)
    return -1;
  unsigned char msg[96];
  fill_pattern (msg, sizeof msg);
  c = ck_fold (c, (long) ACE_OS::send (sv[0], (const char *) msg, sizeof msg, 0));
  unsigned char rcv[96];
  c = ck_fold (c, (long) ACE_OS::recv (sv[1], (char *) rcv, sizeof rcv, 0));
  c = ck_fold (c, ace_cksum (rcv, sizeof rcv));
  ACE_OS::closesocket (sv[0]);
  ACE_OS::closesocket (sv[1]);
  c = ck_fold (c, (long) (ACE_OS::inet_addr ("10.1.2.3") & 0xffffffffUL) % 100003L);
  struct in_addr ia;
  ia.s_addr = ACE_OS::inet_addr ("192.168.7.9");
  c = ck_str (c, ACE_OS::inet_ntoa (ia));
  struct in_addr out;
  c = ck_fold (c, ACE_OS::inet_aton ("127.0.0.1", &out));
  c = ck_fold (c, (long) (ntohl (out.s_addr)));
  return (int) c;
}

// OS_NS_netdb: protocol/service/host databases (fixed well-known entries).
extern "C" int op_full_os_netdb (void)
{
  long c = 79;
  struct protoent *pe = ACE_OS::getprotobyname ("tcp");
  c = ck_fold (c, pe ? pe->p_proto : -1);
  struct servent *se = ACE_OS::getservbyname ("http", "tcp");
  c = ck_fold (c, se ? (long) ntohs ((u_short) se->s_port) : -1);
  struct hostent *he = ACE_OS::gethostbyname ("localhost");
  if (he && he->h_addr_list[0])
    {
      c = ck_fold (c, he->h_addrtype == AF_INET ? 1 : 0);
      unsigned char *ip = (unsigned char *) he->h_addr_list[0];
      c = ck_fold (c, ip[0]);   // 127
      c = ck_fold (c, ip[3]);   // 1
    }
  return (int) c;
}

// OS_NS_sys_mman: anonymous mapping, protection change, unmap.
extern "C" int op_full_os_mmap (void)
{
  long c = 83;
  size_t len = 8192;
  void *m = ACE_OS::mmap (0, len, PROT_READ | PROT_WRITE,
                          MAP_PRIVATE | MAP_ANONYMOUS, ACE_INVALID_HANDLE, 0);
  if (m == MAP_FAILED)
    return -1;
  unsigned char *p = (unsigned char *) m;
  for (size_t i = 0; i < len; i += 97)
    p[i] = (unsigned char) (i % 251);
  long acc = 0;
  for (size_t i = 0; i < len; i += 97)
    acc += p[i];
  c = ck_fold (c, acc);
  c = ck_fold (c, ACE_OS::mprotect (m, len, PROT_READ));
  c = ck_fold (c, p[97]);
  c = ck_fold (c, ACE_OS::munmap (m, len));
  return (int) c;
}

// OS_NS_sys_resource + OS_NS_sys_wait + fork: rlimit invariants and a
// child exit-status round-trip.
extern "C" int op_full_os_forkwait (void)
{
  long c = 89;
  rlimit rl;
  c = ck_fold (c, ACE_OS::getrlimit (RLIMIT_NOFILE, &rl));
  c = ck_fold (c, rl.rlim_cur <= rl.rlim_max ? 1 : 0);
  pid_t child = ACE_OS::fork ();
  if (child == 0)
    ACE_OS::exit (42);
  ACE_exitcode status = 0;
  pid_t got = ACE_OS::waitpid (child, &status, 0);
  c = ck_fold (c, got == child ? 1 : 0);
  c = ck_fold (c, WIFEXITED (status) ? WEXITSTATUS (status) : -1);
  return (int) c;
}

// OS_NS_dlfcn: self-handle symbol lookup.
extern "C" int op_full_os_dlfcn (void)
{
  long c = 97;
  ACE_SHLIB_HANDLE h = ACE_OS::dlopen (0, RTLD_NOW);
  c = ck_fold (c, h ? 1 : 0);
  void *sym = ACE_OS::dlsym (h, "strlen");
  c = ck_fold (c, sym ? 1 : 0);
  c = ck_fold (c, ACE_OS::dlsym (h, "no_such_symbol_ace_full") ? 1 : 0);
  if (h)
    ACE_OS::dlclose (h);
  return (int) c;
}

// OS_NS_sys_shm + OS_NS_sys_msg: SysV shared memory and message queues.
extern "C" int op_full_os_sysv (void)
{
  long c = 101;
  int shm = ACE_OS::shmget (IPC_PRIVATE, 4096, IPC_CREAT | 0600);
  if (shm < 0)
    return -1;
  void *addr = ACE_OS::shmat (shm, 0, 0);
  if (addr == (void *) -1)
    return -2;
  unsigned char *p = (unsigned char *) addr;
  fill_pattern (p, 128);
  c = ck_fold (c, ace_cksum (p, 128));
  c = ck_fold (c, ACE_OS::shmdt (addr));
  c = ck_fold (c, ACE_OS::shmctl (shm, IPC_RMID, 0));
  int mq = ACE_OS::msgget (IPC_PRIVATE, IPC_CREAT | 0600);
  if (mq < 0)
    return -3;
  struct { long mtype; char mtext[64]; } m;
  m.mtype = 7;
  ACE_OS::strsncpy (m.mtext, "sysv-message-vector", 64);
  c = ck_fold (c, ACE_OS::msgsnd (mq, &m, 20, 0));
  m.mtype = 0;
  ACE_OS::memset (m.mtext, 0, sizeof m.mtext);
  c = ck_fold (c, (long) ACE_OS::msgrcv (mq, &m, sizeof m.mtext, 7, 0));
  c = ck_fold (c, (long) m.mtype);
  c = ck_str (c, m.mtext);
  msgctl (mq, IPC_RMID, 0);
  return (int) c;
}

// OS_NS_Thread at the portability layer: identity, mutexes, TSS, and one
// created thread joined for its return value.
static ACE_THR_FUNC_RETURN full_thr_fn (void *arg)
{
  long v = *(long *) arg;
  return (ACE_THR_FUNC_RETURN) (v * 3 + 1);
}
extern "C" int op_full_os_thread (void)
{
  long c = 103;
  c = ck_fold (c, ACE_OS::thr_self () != 0 ? 1 : 0);
  ACE_thread_mutex_t tm;
  c = ck_fold (c, ACE_OS::thread_mutex_init (&tm));
  c = ck_fold (c, ACE_OS::thread_mutex_lock (&tm));
  c = ck_fold (c, ACE_OS::thread_mutex_unlock (&tm));
  c = ck_fold (c, ACE_OS::thread_mutex_destroy (&tm));
  ACE_mutex_t mx;
  c = ck_fold (c, ACE_OS::mutex_init (&mx));
  c = ck_fold (c, ACE_OS::mutex_lock (&mx));
  c = ck_fold (c, ACE_OS::mutex_trylock (&mx) == 0 ? 0 : 1);  // non-recursive: busy
  c = ck_fold (c, ACE_OS::mutex_unlock (&mx));
  c = ck_fold (c, ACE_OS::mutex_destroy (&mx));
  ACE_thread_key_t key;
  c = ck_fold (c, ACE_OS::thr_keycreate (&key, 0));
  static long slot_val = 777;
  c = ck_fold (c, ACE_OS::thr_setspecific (key, &slot_val));
  void *got = 0;
  c = ck_fold (c, ACE_OS::thr_getspecific (key, &got));
  c = ck_fold (c, got ? *(long *) got : -1);
  ACE_OS::thr_keyfree (key);
  long arg = 13;
  ACE_hthread_t th;
  ACE_thread_t tid;
  if (ACE_OS::thr_create (full_thr_fn, &arg, THR_NEW_LWP | THR_JOINABLE,
                          &tid, &th) != 0)
    return -1;
  ACE_THR_FUNC_RETURN rv = 0;
  c = ck_fold (c, ACE_OS::thr_join (th, &rv));
  c = ck_fold (c, (long) (intptr_t) rv);
  return (int) c;
}

// OS_NS_stropts: FIONREAD on a primed pipe through ACE_OS::ioctl.
extern "C" int op_full_os_ioctl (void)
{
  long c = 107;
  ACE_HANDLE pfd[2];
  if (ACE_OS::pipe (pfd) != 0)
    return -1;
  ACE_OS::write (pfd[1], "abc", 3);
  int nread = 0;
  c = ck_fold (c, ACE_OS::ioctl (pfd[0], FIONREAD, &nread));
  c = ck_fold (c, nread);
  ACE_OS::close (pfd[0]);
  ACE_OS::close (pfd[1]);
  return (int) c;
}

// OS_NS_sys_sendfile: kernel copy between regular files.
extern "C" int op_full_os_sendfile (void)
{
  long c = 109;
  char src[] = "/tmp/ace_full_sf_src_XXXXXX";
  char dst[] = "/tmp/ace_full_sf_dst_XXXXXX";
  ACE_HANDLE sfd = ACE_OS::mkstemp (src);
  ACE_HANDLE dfd = ACE_OS::mkstemp (dst);
  if (sfd == ACE_INVALID_HANDLE || dfd == ACE_INVALID_HANDLE)
    return -1;
  unsigned char pat[300];
  fill_pattern (pat, sizeof pat);
  ACE_OS::write (sfd, pat, sizeof pat);
  ACE_OS::lseek (sfd, 0, SEEK_SET);
  off_t off = 0;
  ssize_t sent = ACE_OS::sendfile (dfd, sfd, &off, sizeof pat);
  c = ck_fold (c, (long) sent);
  c = ck_fold (c, (long) off);
  ACE_OS::lseek (dfd, 0, SEEK_SET);
  unsigned char back[300];
  c = ck_fold (c, (long) ACE_OS::read (dfd, back, sizeof back));
  c = ck_fold (c, ace_cksum (back, sizeof back));
  ACE_OS::close (sfd);
  ACE_OS::close (dfd);
  ACE_OS::unlink (src);
  ACE_OS::unlink (dst);
  return (int) c;
}

// OS_NS_wchar: this config (no ACE_HAS_WCHAR) compiles exactly the two
// case-insensitive emulation functions out-of-line — drive both.
extern "C" int op_full_os_wchar (void)
{
  long c = 113;
  c = ck_fold (c, ACE_OS::wcsicmp_emulation (L"WiDeVector", L"widevector"));
  c = ck_fold (c, ACE_OS::wcsicmp_emulation (L"alpha", L"beta") < 0 ? 1 : 2);
  c = ck_fold (c, ACE_OS::wcsnicmp_emulation (L"PREFIXxx", L"prefixYY", 6));
  c = ck_fold (c, ACE_OS::wcsnicmp_emulation (L"abc", L"abd", 3) < 0 ? 1 : 2);
  return (int) c;
}

// ===========================================================================
// batch 1 (continued): strings, containers, and smart-pointer components.
// All of these are header-instantiated templates — the instantiations below
// materialize INSIDE this ops TU on both sides, so the differential certifies
// the translated template code paths themselves.
// ===========================================================================
#include "ace/SString.h"
#include "ace/Array_Base.h"
#include "ace/Array_Map.h"
#include "ace/Map_Manager.h"
#include "ace/Hash_Map_Manager.h"
#include "ace/RB_Tree.h"
#include "ace/Unbounded_Queue.h"
#include "ace/Unbounded_Set.h"
#include "ace/Containers_T.h"
#include "ace/Vector_T.h"
#include "ace/Intrusive_List.h"
#include "ace/Intrusive_List_Node.h"
#include "ace/Auto_Ptr.h"
#include "ace/Bound_Ptr.h"
#include "ace/Refcounted_Auto_Ptr.h"
#include "ace/Free_List.h"
#include "ace/Obstack.h"
#include "ace/Pair_T.h"
#include "ace/Env_Value_T.h"
#include "ace/Active_Map_Manager.h"
#include "ace/Null_Mutex.h"
#include "ace/Synch_Traits.h"
#include "ace/Functor.h"
#include "ace/Functor_String.h"
#include "ace/ace_wchar.h"

static long ck_cstring (long c, const ACE_CString &s)
{
  c = ck_fold (c, (long) s.length ());
  return ck_str (c, s.c_str ());
}

// ACE_CString (SString.cpp + String_Base template): the full mutation
// surface — ctors, concatenation, substr, find/rfind, compare, set/clear.
extern "C" int op_full_cstring (void)
{
  long c = 127;
  ACE_CString a ("the quick brown fox");
  ACE_CString b (a);
  b += " jumps";
  ACE_CString d = a + ACE_CString ("-tail");
  c = ck_cstring (c, a);
  c = ck_cstring (c, b);
  c = ck_cstring (c, d);
  c = ck_fold (c, (long) a.find ("quick"));
  c = ck_fold (c, (long) a.find ('o'));
  c = ck_fold (c, (long) a.rfind ('o'));
  c = ck_fold (c, (long) a.find ("absent"));
  c = ck_cstring (c, a.substr (4, 5));
  c = ck_cstring (c, a.substring (10));
  c = ck_fold (c, a.compare (b));
  c = ck_fold (c, a == b ? 1 : 0);
  c = ck_fold (c, a != d ? 1 : 0);
  c = ck_fold (c, a < b ? 1 : 0);
  c = ck_fold (c, a[6]);
  ACE_CString e;
  c = ck_fold (c, (long) e.length ());
  e.set ("reset-content", 1);
  c = ck_cstring (c, e);
  e.clear (1);
  c = ck_fold (c, (long) e.length ());
  ACE_CString rep ("aXbXcXd");
  c = ck_fold (c, (long) rep.strstr (ACE_CString ("Xc")));
  // ACE_SString: the simple non-copying string of SString.cpp.
  ACE_SString ss ("sstring-vector");
  c = ck_fold (c, (long) ss.length ());
  c = ck_str (c, ss.c_str ());
  ACE_SString sub = ss.substring (3, 4);
  c = ck_str (c, sub.c_str ());
  return (int) c;
}

// ACE_NS_WString (SString.cpp): the ushort wide string of this config —
// converting ctor from narrow, char_rep/ushort_rep round-trips.
extern "C" int op_full_wstring (void)
{
  long c = 131;
  ACE_NS_WString w ("wide-string-vector");
  c = ck_fold (c, (long) w.length ());
  char *narrow = w.char_rep ();
  c = ck_str (c, narrow);
  delete [] narrow;
  ACE_UINT16 *us = w.ushort_rep ();
  for (const ACE_UINT16 *p = us; *p; ++p)
    c = ck_fold (c, (long) *p);
  delete [] us;
  ACE_NS_WString v ("wide-string-vector");
  c = ck_fold (c, w == v ? 1 : 0);
  ACE_NS_WString d ("different");
  c = ck_fold (c, w == d ? 1 : 0);
  c = ck_fold (c, (long) (w + d).length ());
  return (int) c;
}

// ACE_Array / ACE_Array_Base: indexed storage with grow-on-size.
extern "C" int op_full_array (void)
{
  long c = 137;
  ACE_Array<int> arr ((size_t) 8, 0);
  for (size_t i = 0; i < arr.size (); ++i)
    arr[i] = (int) (i * i + 3);
  c = ck_fold (c, (long) arr.size ());
  c = ck_fold (c, arr.size ((size_t) 20));          // grow preserving prefix
  arr[19] = 4242;
  for (size_t i = 0; i < arr.size (); ++i)
    c = ck_fold (c, arr[i]);
  ACE_Array<int> copy (arr);
  c = ck_fold (c, copy == arr ? 1 : 0);
  copy[3] = -1;
  c = ck_fold (c, copy != arr ? 1 : 0);
  ACE_Array<ACE_CString> sarr ((size_t) 3, ACE_CString ("seed"));
  sarr[1] = ACE_CString ("one");
  sarr[2] = ACE_CString ("two");
  for (size_t i = 0; i < sarr.size (); ++i)
    c = ck_cstring (c, sarr[i]);
  return (int) c;
}

// ACE_Array_Map: ordered-by-insertion associative array (std-style API).
extern "C" int op_full_array_map (void)
{
  long c = 139;
  ACE_Array_Map<int, ACE_CString> m;
  m[3] = ACE_CString ("three");
  m[1] = ACE_CString ("one");
  m[7] = ACE_CString ("seven");
  m[1] = ACE_CString ("uno");                        // overwrite
  c = ck_fold (c, (long) m.size ());
  ACE_Array_Map<int, ACE_CString>::iterator f = m.find (7);
  c = ck_fold (c, f != m.end () ? 1 : 0);
  if (f != m.end ())
    c = ck_cstring (c, f->second);
  c = ck_fold (c, (long) m.erase (3));
  c = ck_fold (c, (long) m.size ());
  for (ACE_Array_Map<int, ACE_CString>::iterator it = m.begin ();
       it != m.end (); ++it)
    {
      c = ck_fold (c, it->first);
      c = ck_cstring (c, it->second);
    }
  return (int) c;
}

// ACE_Map_Manager with forward/reverse iteration and unbind recycling.
extern "C" int op_full_map_manager (void)
{
  long c = 149;
  ACE_Map_Manager<int, long, ACE_Null_Mutex> m;
  for (int k = 0; k < 12; ++k)
    c = ck_fold (c, m.bind (k * 3, (long) k * k));
  c = ck_fold (c, (long) m.current_size ());
  long v = 0;
  c = ck_fold (c, m.find (9, v));
  c = ck_fold (c, v);
  c = ck_fold (c, m.find (10, v));                   // miss
  c = ck_fold (c, m.rebind (9, -55L));
  m.find (9, v);
  c = ck_fold (c, v);
  c = ck_fold (c, m.unbind (0));
  c = ck_fold (c, m.unbind (0));                     // double unbind: miss
  c = ck_fold (c, (long) m.current_size ());
  long fsum = 0;
  for (ACE_Map_Manager<int, long, ACE_Null_Mutex>::iterator it = m.begin ();
       it != m.end (); ++it)
    fsum += (*it).ext_id_ * 7 + (*it).int_id_;
  c = ck_fold (c, fsum);
  long rsum = 0;
  for (ACE_Map_Manager<int, long, ACE_Null_Mutex>::reverse_iterator rit = m.rbegin ();
       rit != m.rend (); ++rit)
    rsum = rsum * 3 + (*rit).ext_id_;
  c = ck_fold (c, rsum);
  return (int) c;
}

// ACE_Hash_Map_Manager keyed by ACE_CString: exercises Functor_String's
// ACE_Hash<ACE_CString> and ACE_Equal_To<ACE_CString> plus the bucket walk.
extern "C" int op_full_hash_map (void)
{
  long c = 151;
  ACE_Hash_Map_Manager<ACE_CString, int, ACE_Null_Mutex> hm (16);
  static const char *keys[] = { "alpha", "beta", "gamma", "delta", "epsilon",
                                "zeta", "eta", "theta" };
  for (int i = 0; i < 8; ++i)
    c = ck_fold (c, hm.bind (ACE_CString (keys[i]), i * 11));
  c = ck_fold (c, hm.bind (ACE_CString ("alpha"), 999));   // duplicate: fails
  c = ck_fold (c, (long) hm.current_size ());
  int v = 0;
  c = ck_fold (c, hm.find (ACE_CString ("delta"), v));
  c = ck_fold (c, v);
  c = ck_fold (c, hm.find (ACE_CString ("missing"), v));
  c = ck_fold (c, hm.rebind (ACE_CString ("beta"), -3));
  c = ck_fold (c, hm.trybind (ACE_CString ("iota"), v));   // new binding
  c = ck_fold (c, hm.unbind (ACE_CString ("gamma")));
  c = ck_fold (c, (long) hm.current_size ());
  long walk = 0;
  for (ACE_Hash_Map_Manager<ACE_CString, int, ACE_Null_Mutex>::iterator it
         = hm.begin (); it != hm.end (); ++it)
    walk += ck_str (0, (*it).ext_id_.c_str ()) % 4093 + (*it).int_id_;
  c = ck_fold (c, walk);
  ACE_Hash<ACE_CString> hasher;
  c = ck_fold (c, (long) (hasher (ACE_CString ("hash-me")) % 100003UL));
  ACE_Equal_To<ACE_CString> eq;
  c = ck_fold (c, eq (ACE_CString ("x"), ACE_CString ("x")) ? 1 : 0);
  return (int) c;
}

// ACE_RB_Tree: red-black insert/find/remove with sorted in-order iteration.
extern "C" int op_full_rb_tree (void)
{
  long c = 157;
  ACE_RB_Tree<int, int, ACE_Less_Than<int>, ACE_Null_Mutex> t;
  static const int keys[] = { 41, 7, 93, 18, 62, 5, 77, 30, 88, 12, 55, 3 };
  for (unsigned i = 0; i < sizeof keys / sizeof keys[0]; ++i)
    c = ck_fold (c, t.bind (keys[i], keys[i] * 2 + 1));
  c = ck_fold (c, (long) t.current_size ());
  int v = 0;
  c = ck_fold (c, t.find (62, v));
  c = ck_fold (c, v);
  c = ck_fold (c, t.find (63, v));
  c = ck_fold (c, t.unbind (41));
  c = ck_fold (c, t.unbind (41));
  c = ck_fold (c, (long) t.current_size ());
  for (ACE_RB_Tree<int, int, ACE_Less_Than<int>, ACE_Null_Mutex>::iterator it
         = t.begin (); it != t.end (); ++it)
    c = ck_fold (c, (*it).key () * 5 + (*it).item ());
  return (int) c;
}

// Unbounded queue/set/stack plus the fixed/bounded stacks of Containers_T.
extern "C" int op_full_containers (void)
{
  long c = 163;
  ACE_Unbounded_Queue<int> q;
  for (int i = 1; i <= 6; ++i)
    c = ck_fold (c, q.enqueue_tail (i * 4));
  c = ck_fold (c, q.enqueue_head (-9));
  c = ck_fold (c, (long) q.size ());
  int *peek = 0;
  q.get (peek, 2);
  c = ck_fold (c, peek ? *peek : -1);
  int out = 0;
  while (q.dequeue_head (out) == 0)
    c = ck_fold (c, out);
  ACE_Unbounded_Set<int> s;
  for (int i = 0; i < 9; ++i)
    c = ck_fold (c, s.insert ((i * 5) % 7));         // duplicates rejected
  c = ck_fold (c, (long) s.size ());
  c = ck_fold (c, s.find (3));
  c = ck_fold (c, s.find (42));
  c = ck_fold (c, s.remove (3));
  c = ck_fold (c, (long) s.size ());
  for (ACE_Unbounded_Set<int>::iterator it = s.begin (); it != s.end (); ++it)
    c = ck_fold (c, *it);
  ACE_Unbounded_Stack<int> st;
  for (int i = 0; i < 5; ++i)
    c = ck_fold (c, st.push (i * i));
  int top = 0;
  st.top (top);
  c = ck_fold (c, top);
  while (st.pop (top) == 0)
    c = ck_fold (c, top);
  ACE_Fixed_Stack<int, 8> fs;
  for (int i = 0; i < 10; ++i)
    c = ck_fold (c, fs.push (i));                    // overflow at 8
  c = ck_fold (c, (long) fs.size ());
  fs.pop (top);
  c = ck_fold (c, top);
  ACE_Bounded_Stack<int> bs (4);
  for (int i = 0; i < 5; ++i)
    c = ck_fold (c, bs.push (i * 3));
  bs.pop (top);
  c = ck_fold (c, top);
  c = ck_fold (c, (long) bs.size ());
  return (int) c;
}

// ACE_Vector: std::vector-flavored wrapper over ACE_Array_Base.
extern "C" int op_full_vector (void)
{
  long c = 167;
  ACE_Vector<int> v;
  for (int i = 0; i < 20; ++i)
    v.push_back (i * 13 % 17);
  c = ck_fold (c, (long) v.size ());
  c = ck_fold (c, (long) v.capacity () >= 20 ? 1 : 0);
  for (size_t i = 0; i < v.size (); ++i)
    c = ck_fold (c, v[i]);
  v.resize (5, 0);
  c = ck_fold (c, (long) v.size ());
  v.pop_back ();
  c = ck_fold (c, (long) v.size ());
  ACE_Vector<int> w;
  w.push_back (1);
  c = ck_fold (c, v == w ? 1 : 0);
  v.clear ();
  c = ck_fold (c, (long) v.size ());
  ACE_Vector<ACE_CString> sv;
  sv.push_back (ACE_CString ("first"));
  sv.push_back (ACE_CString ("second"));
  c = ck_cstring (c, sv[0]);
  c = ck_cstring (c, sv[1]);
  return (int) c;
}

// Intrusive list with nodes derived from ACE_Intrusive_List_Node.
class Full_IL_Node : public ACE_Intrusive_List_Node<Full_IL_Node>
{
public:
  Full_IL_Node (int v) : v_ (v) {}
  int v_;
};
extern "C" int op_full_intrusive_list (void)
{
  long c = 173;
  Full_IL_Node n1 (10), n2 (20), n3 (30), n4 (40);
  ACE_Intrusive_List<Full_IL_Node> il;
  c = ck_fold (c, il.is_empty () ? 1 : 0);
  il.push_back (&n1);
  il.push_back (&n2);
  il.push_front (&n3);
  il.push_back (&n4);
  il.remove (&n2);
  for (Full_IL_Node *p = il.head (); p; p = p->next ())
    c = ck_fold (c, p->v_);
  c = ck_fold (c, il.pop_front ()->v_);
  c = ck_fold (c, il.pop_back ()->v_);
  c = ck_fold (c, il.is_empty () ? 1 : 0);
  return (int) c;
}

// Smart pointers: auto, strong/weak bound, refcounted — lifecycle counts
// observed through a static-instance counter.
static int full_ptr_live = 0;
static int full_ptr_total = 0;
class Full_Counted
{
public:
  Full_Counted (int v) : v_ (v) { ++full_ptr_live; ++full_ptr_total; }
  ~Full_Counted (void) { --full_ptr_live; }
  int v_;
};
extern "C" int op_full_smart_ptr (void)
{
  long c = 179;
  full_ptr_live = full_ptr_total = 0;
  {
    ACE_Auto_Basic_Ptr<Full_Counted> ap (new Full_Counted (5));
    c = ck_fold (c, ap.get ()->v_);
    c = ck_fold (c, full_ptr_live);
    ap.reset (new Full_Counted (6));
    c = ck_fold (c, ap.get ()->v_);
    c = ck_fold (c, full_ptr_live);
    Full_Counted *raw = ap.release ();
    c = ck_fold (c, full_ptr_live);
    delete raw;
  }
  c = ck_fold (c, full_ptr_live);
  {
    ACE_Auto_Basic_Array_Ptr<int> aap (new int[8]);
    for (int i = 0; i < 8; ++i)
      aap[i] = i * i;
    c = ck_fold (c, aap[5]);
  }
  {
    ACE_Strong_Bound_Ptr<Full_Counted, ACE_Null_Mutex> sp1 (new Full_Counted (7));
    ACE_Strong_Bound_Ptr<Full_Counted, ACE_Null_Mutex> sp2 (sp1);
    c = ck_fold (c, sp1->v_ + sp2->v_);
    c = ck_fold (c, full_ptr_live);
    ACE_Weak_Bound_Ptr<Full_Counted, ACE_Null_Mutex> wp (sp1);
    c = ck_fold (c, wp.null () ? 1 : 0);
    {
      ACE_Strong_Bound_Ptr<Full_Counted, ACE_Null_Mutex> sp3 (wp);
      c = ck_fold (c, sp3.null () ? 1 : 0);
      c = ck_fold (c, sp3->v_);
    }
    sp1.reset ();
    sp2.reset ();
    c = ck_fold (c, full_ptr_live);
    ACE_Strong_Bound_Ptr<Full_Counted, ACE_Null_Mutex> sp4 (wp);
    c = ck_fold (c, sp4.null () ? 1 : 0);   // object gone: null strong
  }
  c = ck_fold (c, full_ptr_live);
  {
    ACE_Refcounted_Auto_Ptr<Full_Counted, ACE_Null_Mutex> rp1 (new Full_Counted (9));
    ACE_Refcounted_Auto_Ptr<Full_Counted, ACE_Null_Mutex> rp2 (rp1);
    c = ck_fold (c, rp1->v_ + rp2->v_);
    c = ck_fold (c, rp1.count ());
    c = ck_fold (c, full_ptr_live);
  }
  c = ck_fold (c, full_ptr_live);
  c = ck_fold (c, full_ptr_total);
  return (int) c;
}

// ACE_Locked_Free_List over a self-linked node type.
class Full_FL_Node
{
public:
  Full_FL_Node (void) : next_ (0), v_ (0) {}
  Full_FL_Node *get_next (void) { return this->next_; }
  void set_next (Full_FL_Node *n) { this->next_ = n; }
private:
  Full_FL_Node *next_;
public:
  int v_;
};
extern "C" int op_full_free_list (void)
{
  long c = 181;
  ACE_Locked_Free_List<Full_FL_Node, ACE_Null_Mutex> fl (ACE_FREE_LIST_WITH_POOL,
                                                         4,   // prealloc
                                                         8,   // lwm
                                                         16); // hwm
  c = ck_fold (c, (long) fl.size ());
  Full_FL_Node *a = fl.remove ();
  Full_FL_Node *b = fl.remove ();
  c = ck_fold (c, (long) fl.size ());
  c = ck_fold (c, a != 0 && b != 0 && a != b ? 1 : 0);
  a->v_ = 5;
  fl.add (a);
  fl.add (b);
  c = ck_fold (c, (long) fl.size ());
  return (int) c;
}

// ACE_Obstack: stack-discipline string arena.
extern "C" int op_full_obstack (void)
{
  long c = 191;
  ACE_Obstack ob (64);
  char *s1 = ob.copy ("obstack-one", 11);
  char *s2 = ob.copy ("second", 6);
  c = ck_str (c, s1);
  c = ck_str (c, s2);
  ob.request (4);
  ob.grow ('g');
  ob.grow ('r');
  ob.grow ('o');
  ob.grow ('w');
  char *s3 = ob.freeze ();
  c = ck_str (c, s3);
  ob.release ();
  return (int) c;
}

// ACE_Pair + ACE_Env_Value: tuple glue and typed environment reads.
extern "C" int op_full_pair_env (void)
{
  long c = 193;
  int rx = 700;
  long ry = 800;
  ACE_Reference_Pair<int, long> rp (rx, ry);
  c = ck_fold (c, rp.first () + (long) rp.second ());
  ACE_OS::setenv ("ACE_FULL_EV_INT", "31337", 1);
  ACE_Env_Value<int> evi ("ACE_FULL_EV_INT", 7);
  c = ck_fold (c, evi);
  ACE_Env_Value<int> evd ("ACE_FULL_EV_ABSENT", 42);
  c = ck_fold (c, evd);
  ACE_OS::setenv ("ACE_FULL_EV_ULONG", "123456", 1);
  ACE_Env_Value<u_long> evu ("ACE_FULL_EV_ULONG", 1UL);
  c = ck_fold (c, (long) (u_long) evu);
  return (int) c;
}

// ACE_Active_Map_Manager: slot/generation keys with recycling.
extern "C" int op_full_active_map (void)
{
  long c = 197;
  ACE_Active_Map_Manager<ACE_CString> am;
  ACE_Active_Map_Manager_Key k1, k2, k3;
  c = ck_fold (c, am.bind (ACE_CString ("first"), k1));
  c = ck_fold (c, am.bind (ACE_CString ("second"), k2));
  c = ck_fold (c, (long) am.current_size ());
  ACE_CString v;
  c = ck_fold (c, am.find (k1, v));
  c = ck_cstring (c, v);
  c = ck_fold (c, am.unbind (k1));
  c = ck_fold (c, am.find (k1, v));                  // stale key: miss
  c = ck_fold (c, am.bind (ACE_CString ("third"), k3));
  c = ck_fold (c, am.find (k3, v));
  c = ck_cstring (c, v);
  c = ck_fold (c, (long) am.current_size ());
  c = ck_fold (c, k1.slot_index () == k3.slot_index () ? 1 : 0);  // recycled slot
  c = ck_fold (c, k1.slot_generation () != k3.slot_generation () ? 1 : 0);
  return (int) c;
}

// ACE.cpp utility surface beyond the CRCs: hashing, hex/byte helpers,
// basename/dirname, and wildcard matching.
extern "C" int op_full_ace_util (void)
{
  long c = 199;
  c = ck_fold (c, (long) (ACE::hash_pjw ("hash-pjw-vector") % 100003UL));
  c = ck_fold (c, (long) ACE::log2 (1024UL));
  c = ck_fold (c, ACE::nibble2hex (0xA));
  c = ck_fold (c, ACE::nibble2hex (0x3));
  ACE_TCHAR hexbuf[8];
  hexbuf[0] = 'f'; hexbuf[1] = '0';
  c = ck_fold (c, ACE::hex2byte ('f') * 16 + ACE::hex2byte ('0'));
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (ACE::basename (ACE_TEXT ("/usr/lib/libace.so"), '/')));
  c = ck_fold (c, ACE::wild_match ("file.cpp", "*.cpp") ? 1 : 0);
  c = ck_fold (c, ACE::wild_match ("file.hpp", "*.cpp") ? 1 : 0);
  c = ck_fold (c, ACE::wild_match ("abc", "a?c") ? 1 : 0);
  return (int) c;
}

// ===========================================================================
// batch 2A: synchronization primitives, threads, tasks, futures.
// ===========================================================================
#include "ace/Thread_Mutex.h"
#include "ace/Mutex.h"
#include "ace/RW_Thread_Mutex.h"
#include "ace/Recursive_Thread_Mutex.h"
#include "ace/Condition_Thread_Mutex.h"
#include "ace/Condition_Recursive_Thread_Mutex.h"
#include "ace/Condition_Attributes.h"
#include "ace/Guard_T.h"
#include "ace/Semaphore.h"
#include "ace/Thread_Semaphore.h"
#include "ace/Barrier.h"
#include "ace/Atomic_Op.h"
#include "ace/Thread_Manager.h"
#include "ace/Thread.h"
#include "ace/Task.h"
#include "ace/Activation_Queue.h"
#include "ace/Method_Request.h"
#include "ace/Future.h"
#include "ace/TSS_T.h"
#include "ace/Sched_Params.h"
#include "ace/Test_and_Set.h"
#include "ace/Lock_Adapter_T.h"
#include "ace/Synch_Options.h"

// Mutex family: plain, thread, recursive, readers/writer.
extern "C" int op_full_mutex (void)
{
  long c = 211;
  ACE_Thread_Mutex tm;
  c = ck_fold (c, tm.acquire ());
  c = ck_fold (c, tm.tryacquire () == -1 ? 1 : 0);   // non-recursive: busy
  c = ck_fold (c, tm.release ());
  c = ck_fold (c, tm.tryacquire ());
  c = ck_fold (c, tm.release ());
  ACE_Mutex mx;
  c = ck_fold (c, mx.acquire ());
  c = ck_fold (c, mx.release ());
  ACE_Recursive_Thread_Mutex rm;
  c = ck_fold (c, rm.acquire ());
  c = ck_fold (c, rm.acquire ());                    // recursive re-entry
  c = ck_fold (c, rm.get_nesting_level ());
  c = ck_fold (c, rm.release ());
  c = ck_fold (c, rm.release ());
  ACE_RW_Thread_Mutex rw;
  c = ck_fold (c, rw.acquire_read ());
  c = ck_fold (c, rw.tryacquire_read ());            // shared: ok
  c = ck_fold (c, rw.tryacquire_write () == -1 ? 1 : 0);
  c = ck_fold (c, rw.release ());
  c = ck_fold (c, rw.release ());
  c = ck_fold (c, rw.acquire_write ());
  c = ck_fold (c, rw.tryacquire_read () == -1 ? 1 : 0);
  c = ck_fold (c, rw.release ());
  ACE_Lock_Adapter<ACE_Thread_Mutex> la;
  c = ck_fold (c, la.acquire ());
  c = ck_fold (c, la.release ());
  ACE_Synch_Options so (ACE_Synch_Options::USE_TIMEOUT, ACE_Time_Value (3, 0));
  c = ck_fold (c, so[ACE_Synch_Options::USE_TIMEOUT] ? 1 : 0);
  c = ck_fold (c, (long) so.timeout ().sec ());
  return (int) c;
}

// Scoped guards over the mutex family.
extern "C" int op_full_guard (void)
{
  long c = 223;
  ACE_Thread_Mutex m;
  {
    ACE_Guard<ACE_Thread_Mutex> g (m);
    c = ck_fold (c, g.locked () ? 1 : 0);
    c = ck_fold (c, m.tryacquire () == -1 ? 1 : 0);
  }
  c = ck_fold (c, m.tryacquire ());
  m.release ();
  ACE_RW_Thread_Mutex rw;
  {
    ACE_Read_Guard<ACE_RW_Thread_Mutex> rg (rw);
    c = ck_fold (c, rg.locked () ? 1 : 0);
    {
      ACE_Read_Guard<ACE_RW_Thread_Mutex> rg2 (rw);
      c = ck_fold (c, rg2.locked () ? 1 : 0);
    }
  }
  {
    ACE_Write_Guard<ACE_RW_Thread_Mutex> wg (rw);
    c = ck_fold (c, wg.locked () ? 1 : 0);
  }
  ACE_Recursive_Thread_Mutex rm;
  {
    ACE_Guard<ACE_Recursive_Thread_Mutex> g1 (rm);
    ACE_Guard<ACE_Recursive_Thread_Mutex> g2 (rm);
    c = ck_fold (c, (g1.locked () ? 1 : 0) + (g2.locked () ? 2 : 0));
  }
  return (int) c;
}

// Counting semaphores (process-scope thread semantics).
extern "C" int op_full_semaphore (void)
{
  long c = 227;
  ACE_Thread_Semaphore ts (2);
  c = ck_fold (c, ts.acquire ());
  c = ck_fold (c, ts.tryacquire ());
  c = ck_fold (c, ts.tryacquire () == -1 ? 1 : 0);   // exhausted
  c = ck_fold (c, ts.release ());
  c = ck_fold (c, ts.tryacquire ());
  c = ck_fold (c, ts.release (2));
  ACE_Semaphore s (1);
  c = ck_fold (c, s.acquire ());
  c = ck_fold (c, s.release ());
  return (int) c;
}

// Condition variables: past-deadline timed wait (ETIME path) and
// signal/broadcast with no waiters.
extern "C" int op_full_condition (void)
{
  long c = 229;
  ACE_Thread_Mutex m;
  ACE_Condition_Thread_Mutex cv (m);
  m.acquire ();
  ACE_Time_Value past = ACE_OS::gettimeofday () - ACE_Time_Value (1, 0);
  c = ck_fold (c, cv.wait (&past) == -1 ? 1 : 0);
  c = ck_fold (c, ACE_OS::last_error () == ETIME ? 1 : 0);
  m.release ();
  c = ck_fold (c, cv.signal ());
  c = ck_fold (c, cv.broadcast ());
  ACE_Condition_Attributes attr;
  ACE_Condition_Thread_Mutex cv2 (m, attr);
  c = ck_fold (c, cv2.signal ());
  ACE_Recursive_Thread_Mutex rm;
  ACE_Condition_Recursive_Thread_Mutex rcv (rm);
  rm.acquire ();
  ACE_Time_Value past2 = ACE_OS::gettimeofday () - ACE_Time_Value (1, 0);
  c = ck_fold (c, rcv.wait (&past2) == -1 ? 1 : 0);
  rm.release ();
  c = ck_fold (c, rcv.signal ());
  return (int) c;
}

// Atomic op template over a real mutex.
extern "C" int op_full_atomic (void)
{
  long c = 233;
  ACE_Atomic_Op<ACE_Thread_Mutex, long> a (5);
  c = ck_fold (c, (long) ++a);
  c = ck_fold (c, (long) a++);
  c = ck_fold (c, (long) a.value ());
  a += 10;
  a -= 3;
  c = ck_fold (c, (long) --a);
  c = ck_fold (c, a == 13L ? 1 : 0);
  c = ck_fold (c, a >= 13L ? 1 : 0);
  ACE_Atomic_Op<ACE_Thread_Mutex, unsigned> u (0);
  u = 7;
  c = ck_fold (c, (long) u.value ());
  return (int) c;
}

// Thread_Manager: spawn_n workers contending on a shared counter, wait for
// all, verify the total (Thread/Thread_Adapter/Base_Thread_Adapter ride in).
static ACE_Thread_Mutex full_tm_lock;
static long full_tm_counter = 0;
static ACE_THR_FUNC_RETURN full_tm_worker (void *)
{
  for (int i = 0; i < 1000; ++i)
    {
      ACE_Guard<ACE_Thread_Mutex> g (full_tm_lock);
      ++full_tm_counter;
    }
  return 0;
}
extern "C" int op_full_thread_mgr (void)
{
  long c = 239;
  full_tm_counter = 0;
  ACE_Thread_Manager tm;
  int grp = tm.spawn_n (4, full_tm_worker, 0, THR_NEW_LWP | THR_JOINABLE);
  c = ck_fold (c, grp != -1 ? 1 : 0);
  c = ck_fold (c, tm.wait_grp (grp));
  c = ck_fold (c, full_tm_counter);
  c = ck_fold (c, (long) tm.count_threads ());
  return (int) c;
}

// Barrier: N threads rendezvous, each records the pre-barrier and
// post-barrier phase; totals are exact.
static ACE_Barrier *full_barrier_p = 0;
static ACE_Atomic_Op<ACE_Thread_Mutex, long> full_barrier_pre (0);
static ACE_Atomic_Op<ACE_Thread_Mutex, long> full_barrier_post (0);
static ACE_THR_FUNC_RETURN full_barrier_worker (void *)
{
  ++full_barrier_pre;
  full_barrier_p->wait ();
  // every pre-increment happened before any thread passes the barrier
  ++full_barrier_post;
  return (ACE_THR_FUNC_RETURN) (intptr_t) full_barrier_pre.value ();
}
extern "C" int op_full_barrier (void)
{
  long c = 241;
  ACE_Barrier b (3);
  full_barrier_p = &b;
  full_barrier_pre = 0;
  full_barrier_post = 0;
  ACE_Thread_Manager tm;
  tm.spawn_n (3, full_barrier_worker, 0, THR_NEW_LWP | THR_JOINABLE);
  tm.wait ();
  c = ck_fold (c, full_barrier_pre.value ());    // 3
  c = ck_fold (c, full_barrier_post.value ());   // 3
  full_barrier_p = 0;
  return (int) c;
}

// ACE_Task with a synchronized message queue: producers put N blocks,
// worker threads consume and sum the payloads, hangup ends the svc loop.
class Full_Task : public ACE_Task<ACE_MT_SYNCH>
{
public:
  Full_Task (void) : sum_ (0), seen_ (0) {}
  virtual int svc (void)
  {
    for (;;)
      {
        ACE_Message_Block *mb = 0;
        if (this->getq (mb) == -1)
          return 0;
        if (mb->msg_type () == ACE_Message_Block::MB_HANGUP)
          {
            mb->release ();
            // propagate the hangup so sibling workers terminate too
            ACE_Message_Block *h =
              new ACE_Message_Block (0, ACE_Message_Block::MB_HANGUP);
            this->putq (h);
            return 0;
          }
        long v = 0;
        ACE_OS::memcpy (&v, mb->rd_ptr (), sizeof v);
        {
          ACE_Guard<ACE_Thread_Mutex> g (this->lock_);
          this->sum_ += v;
          ++this->seen_;
        }
        mb->release ();
      }
  }
  ACE_Thread_Mutex lock_;
  long sum_;
  long seen_;
};
extern "C" int op_full_task (void)
{
  long c = 251;
  Full_Task t;
  c = ck_fold (c, t.activate (THR_NEW_LWP | THR_JOINABLE, 3));
  long expect = 0;
  for (long i = 1; i <= 24; ++i)
    {
      long v = i * 3 + 1;
      ACE_Message_Block *mb = new ACE_Message_Block (sizeof v);
      ACE_OS::memcpy (mb->wr_ptr (), &v, sizeof v);
      mb->wr_ptr (sizeof v);
      expect += v;
      t.putq (mb);
    }
  ACE_Message_Block *h = new ACE_Message_Block (0, ACE_Message_Block::MB_HANGUP);
  t.putq (h);
  c = ck_fold (c, t.wait ());
  c = ck_fold (c, t.sum_ == expect ? 1 : 0);
  c = ck_fold (c, t.seen_);
  c = ck_fold (c, t.msg_queue ()->message_count () <= 1 ? 1 : 0);
  return (int) c;
}

// Activation queue executing queued method requests in order.
class Full_MR : public ACE_Method_Request
{
public:
  Full_MR (long v, long *acc) : v_ (v), acc_ (acc) {}
  virtual int call (void)
  {
    *this->acc_ = *this->acc_ * 31 + this->v_;
    return (int) this->v_;
  }
  long v_;
  long *acc_;
};
extern "C" int op_full_activation_queue (void)
{
  long c = 257;
  ACE_Activation_Queue aq;
  long acc = 1;
  c = ck_fold (c, aq.enqueue (new Full_MR (4, &acc), 0));
  c = ck_fold (c, aq.enqueue (new Full_MR (9, &acc), 0));
  c = ck_fold (c, aq.enqueue (new Full_MR (2, &acc), 0));
  c = ck_fold (c, aq.method_count ());
  for (int i = 0; i < 3; ++i)
    {
      ACE_Method_Request *mr = aq.dequeue ();
      c = ck_fold (c, mr->call ());
      delete mr;
    }
  c = ck_fold (c, acc);
  c = ck_fold (c, aq.is_empty () ? 1 : 0);
  return (int) c;
}

// Futures: immediate set/get plus sharing and cancel state.
extern "C" int op_full_future (void)
{
  long c = 263;
  ACE_Future<int> f;
  c = ck_fold (c, f.ready ());
  c = ck_fold (c, f.set (77));
  c = ck_fold (c, f.ready ());
  int v = 0;
  c = ck_fold (c, f.get (v));
  c = ck_fold (c, v);
  ACE_Future<int> g (f);                       // shared rep
  int w = 0;
  g.get (w);
  c = ck_fold (c, w);
  c = ck_fold (c, f == g ? 1 : 0);
  ACE_Future<int> h;
  c = ck_fold (c, h.cancel (-5));
  h.get (v);
  c = ck_fold (c, v);
  return (int) c;
}

// Thread-specific storage: each spawned thread sees its own instance.
class Full_TSS_Obj
{
public:
  Full_TSS_Obj (void) : v_ (5) {}
  int v_;
};
static ACE_TSS<Full_TSS_Obj> *full_tss_p = 0;
static ACE_THR_FUNC_RETURN full_tss_worker (void *arg)
{
  long tag = (long) (intptr_t) arg;
  Full_TSS_Obj *obj = full_tss_p->ts_object ();
  if (!obj)
    return (ACE_THR_FUNC_RETURN) (intptr_t) -1;
  obj->v_ = (int) (tag * 100);
  ACE_OS::sleep (ACE_Time_Value (0, 20000));
  return (ACE_THR_FUNC_RETURN) (intptr_t) (*full_tss_p)->v_;
}
extern "C" int op_full_tss (void)
{
  long c = 269;
  ACE_TSS<Full_TSS_Obj> tss;
  full_tss_p = &tss;
  ACE_hthread_t th1, th2;
  ACE_thread_t t1, t2;
  ACE_OS::thr_create (full_tss_worker, (void *) 1, THR_NEW_LWP | THR_JOINABLE, &t1, &th1);
  ACE_OS::thr_create (full_tss_worker, (void *) 2, THR_NEW_LWP | THR_JOINABLE, &t2, &th2);
  ACE_THR_FUNC_RETURN r1 = 0, r2 = 0;
  ACE_OS::thr_join (th1, &r1);
  ACE_OS::thr_join (th2, &r2);
  c = ck_fold (c, (long) (intptr_t) r1);
  c = ck_fold (c, (long) (intptr_t) r2);
  c = ck_fold (c, tss->v_);                     // main's copy untouched
  full_tss_p = 0;
  return (int) c;
}

// Scheduler parameter surface (query-only invariants).
extern "C" int op_full_sched (void)
{
  long c = 271;
  int lo = ACE_Sched_Params::priority_min (ACE_SCHED_OTHER);
  int hi = ACE_Sched_Params::priority_max (ACE_SCHED_OTHER);
  c = ck_fold (c, lo <= hi ? 1 : 0);
  c = ck_fold (c, ACE_Sched_Params::next_priority (ACE_SCHED_OTHER, hi) == hi ? 1 : 0);
  ACE_Sched_Params sp (ACE_SCHED_OTHER, lo);
  c = ck_fold (c, sp.policy () == ACE_SCHED_OTHER ? 1 : 0);
  c = ck_fold (c, sp.priority () == lo ? 1 : 0);
  return (int) c;
}

// Test-and-set (also an ACE_Event_Handler subclass).
extern "C" int op_full_test_and_set (void)
{
  long c = 277;
  ACE_Test_and_Set<ACE_Null_Mutex, int> tas (0);
  c = ck_fold (c, tas.is_set ());
  c = ck_fold (c, tas.set (1));
  c = ck_fold (c, tas.is_set ());
  c = ck_fold (c, tas.set (0));
  return (int) c;
}

// ===========================================================================
// batch 2B: allocators and memory pools, message blocks/queues, logging.
// ===========================================================================
#include "ace/Malloc_T.h"
#include "ace/Malloc.h"
#include "ace/Malloc_Allocator.h"
#include "ace/Local_Memory_Pool.h"
#include "ace/MMAP_Memory_Pool.h"
#include "ace/Shared_Memory_Pool.h"
#include "ace/PI_Malloc.h"
#include "ace/Mem_Map.h"
#include "ace/Shared_Memory_MM.h"
#include "ace/Shared_Memory_SV.h"
#include "ace/SV_Shared_Memory.h"
#include "ace/SV_Semaphore_Simple.h"
#include "ace/SV_Semaphore_Complex.h"
#include "ace/SV_Message_Queue.h"
#include "ace/Typed_SV_Message_Queue.h"
#include "ace/Message_Block.h"
#include "ace/Message_Queue.h"
#include "ace/Log_Msg.h"
#include "ace/Log_Record.h"
#include "ace/Log_Category.h"
#include "ace/Trace.h"
#include "ace/Read_Buffer.h"

// ACE_Malloc over the local (heap) pool, with named-block bind/find.
extern "C" int op_full_malloc_local (void)
{
  long c = 281;
  ACE_Malloc<ACE_LOCAL_MEMORY_POOL, ACE_Null_Mutex> alloc ("full_malloc_local");
  void *b1 = alloc.malloc (128);
  void *b2 = alloc.calloc (64);
  c = ck_fold (c, b1 && b2 && b1 != b2 ? 1 : 0);
  unsigned char *z = (unsigned char *) b2;
  long zsum = 0;
  for (int i = 0; i < 64; ++i)
    zsum += z[i];
  c = ck_fold (c, zsum);                              // calloc zeroes
  ACE_OS::memset (b1, 0x5a, 128);
  c = ck_fold (c, alloc.bind ("block-one", b1));
  void *found = 0;
  c = ck_fold (c, alloc.find ("block-one", found));
  c = ck_fold (c, found == b1 ? 1 : 0);
  c = ck_fold (c, alloc.find ("absent", found));
  alloc.free (b2);
  void *b3 = alloc.malloc (32);
  c = ck_fold (c, b3 != 0 ? 1 : 0);
  alloc.free (b3);
  c = ck_fold (c, alloc.unbind ("block-one"));
  alloc.free (b1);
  c = ck_fold (c, alloc.remove ());
  return (int) c;
}

// The default allocator singleton and the cached-chunk allocators.
extern "C" int op_full_allocator (void)
{
  long c = 283;
  ACE_Allocator *da = ACE_Allocator::instance ();
  void *p = da->malloc (256);
  c = ck_fold (c, p != 0 ? 1 : 0);
  da->free (p);
  ACE_Cached_Allocator<double, ACE_Null_Mutex> ca (8);
  void *c1 = ca.malloc (sizeof (double));
  void *c2 = ca.malloc (sizeof (double));
  c = ck_fold (c, c1 && c2 && c1 != c2 ? 1 : 0);
  c = ck_fold (c, ca.malloc (2 * sizeof (double)) == 0 ? 1 : 0);  // over chunk size
  ca.free (c1);
  void *c3 = ca.malloc (sizeof (double));
  c = ck_fold (c, c3 == c1 ? 1 : 0);                  // LIFO reuse of the chunk
  ca.free (c2);
  ca.free (c3);
  ACE_Dynamic_Cached_Allocator<ACE_Null_Mutex> dca (4, 96);
  void *d1 = dca.malloc (96);
  c = ck_fold (c, d1 != 0 ? 1 : 0);
  c = ck_fold (c, dca.malloc (97) == 0 ? 1 : 0);
  dca.free (d1);
  c = ck_fold (c, (long) dca.pool_depth ());
  return (int) c;
}

// ACE_Malloc over the MMAP pool (file-backed), position-independent
// control block variant included.
extern "C" int op_full_malloc_mmap (void)
{
  long c = 293;
  // reserve a unique name, then let the pool create the backing file itself
  char back[] = "/tmp/ace_full_mmap_XXXXXX";
  ACE_HANDLE bfd = ACE_OS::mkstemp (back);
  if (bfd == ACE_INVALID_HANDLE)
    return -1;
  ACE_OS::close (bfd);
  ACE_OS::unlink (back);
  ACE_MMAP_Memory_Pool_Options opts (0, ACE_MMAP_Memory_Pool_Options::NEVER_FIXED);
  {
    ACE_Malloc<ACE_MMAP_MEMORY_POOL, ACE_Null_Mutex>
      m (ACE_TEXT_CHAR_TO_TCHAR (back), 0, &opts);
    void *b1 = m.malloc (200);
    c = ck_fold (c, b1 != 0 ? 1 : 0);
    ACE_OS::memset (b1, 0x7e, 200);
    c = ck_fold (c, m.bind ("mmap-block", b1));
    void *f = 0;
    c = ck_fold (c, m.find ("mmap-block", f));
    c = ck_fold (c, f == b1 ? 1 : 0);
    c = ck_fold (c, ((unsigned char *) f)[199]);
    m.free (b1);
    c = ck_fold (c, m.remove ());
  }
  ACE_OS::unlink (back);
  // position-independent malloc over MMAP (the PI_Malloc control block)
  char back2[] = "/tmp/ace_full_pimm_XXXXXX";
  ACE_HANDLE b2fd = ACE_OS::mkstemp (back2);
  ACE_OS::close (b2fd);
  ACE_OS::unlink (back2);
  {
    ACE_Malloc_T<ACE_MMAP_MEMORY_POOL, ACE_Null_Mutex, ACE_PI_Control_Block>
      pim (ACE_TEXT_CHAR_TO_TCHAR (back2), 0, &opts);
    void *p1 = pim.malloc (64);
    c = ck_fold (c, p1 != 0 ? 1 : 0);
    c = ck_fold (c, pim.bind ("pi-block", p1));
    void *pf = 0;
    c = ck_fold (c, pim.find ("pi-block", pf));
    c = ck_fold (c, pf == p1 ? 1 : 0);
    pim.free (p1);
    c = ck_fold (c, pim.remove ());
  }
  ACE_OS::unlink (back2);
  return (int) c;
}

// ACE_Mem_Map: map a file read/write, mutate through the mapping, sync.
extern "C" int op_full_mem_map (void)
{
  long c = 307;
  char path[] = "/tmp/ace_full_memmap_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  if (fd == ACE_INVALID_HANDLE)
    return -1;
  unsigned char pat[512];
  fill_pattern (pat, sizeof pat);
  ACE_OS::write (fd, pat, sizeof pat);
  ACE_OS::close (fd);
  {
    ACE_Mem_Map mm;
    c = ck_fold (c, mm.map (ACE_TEXT_CHAR_TO_TCHAR (path), (size_t) -1,
                            O_RDWR, ACE_DEFAULT_FILE_PERMS, PROT_RDWR,
                            ACE_MAP_SHARED));
    c = ck_fold (c, (long) mm.size ());
    unsigned char *base = (unsigned char *) mm.addr ();
    c = ck_fold (c, ace_cksum (base, 64));
    for (int i = 0; i < 64; ++i)
      base[i] = (unsigned char) (255 - base[i]);
    c = ck_fold (c, mm.sync ());
    c = ck_fold (c, mm.unmap ());
  }
  ACE_HANDLE rfd = ACE_OS::open (path, O_RDONLY);
  unsigned char back[64];
  ACE_OS::read (rfd, back, sizeof back);
  ACE_OS::close (rfd);
  c = ck_fold (c, ace_cksum (back, sizeof back));
  ACE_OS::unlink (path);
  return (int) c;
}

// ACE_Shared_Memory_MM (file-backed) and the SysV shared-memory wrappers.
extern "C" int op_full_shmem (void)
{
  long c = 311;
  char path[] = "/tmp/ace_full_shmm_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  ACE_OS::close (fd);
  {
    ACE_Shared_Memory_MM sm (ACE_TEXT_CHAR_TO_TCHAR (path), 4096);
    void *seg = sm.malloc (128);
    c = ck_fold (c, seg != 0 ? 1 : 0);
    ACE_OS::memset (seg, 0x42, 128);
    c = ck_fold (c, ((unsigned char *) seg)[127]);
    c = ck_fold (c, (long) sm.get_segment_size ());
    c = ck_fold (c, sm.free (seg) >= 0 ? 1 : 0);
    sm.remove ();
  }
  ACE_OS::unlink (path);
  ACE_SV_Shared_Memory svm (IPC_PRIVATE, 4096, ACE_SV_Shared_Memory::ACE_CREATE);
  unsigned char *seg = (unsigned char *) svm.get_segment_ptr ();
  c = ck_fold (c, seg != 0 ? 1 : 0);
  fill_pattern (seg, 96);
  c = ck_fold (c, ace_cksum (seg, 96));
  c = ck_fold (c, (long) svm.get_segment_size ());
  c = ck_fold (c, svm.remove ());
  return (int) c;
}

// SysV semaphores (simple + complex undo semantics) and message queues.
extern "C" int op_full_sysv_ipc (void)
{
  long c = 313;
  ACE_SV_Semaphore_Simple ss (ACE_DEFAULT_SEM_KEY + 101, ACE_SV_Semaphore_Simple::ACE_CREATE, 1);
  c = ck_fold (c, ss.acquire ());
  c = ck_fold (c, ss.tryacquire () == -1 ? 1 : 0);
  c = ck_fold (c, ss.release ());
  c = ck_fold (c, ss.tryacquire ());
  c = ck_fold (c, ss.release ());
  c = ck_fold (c, ss.remove ());
  ACE_SV_Semaphore_Complex sc (ACE_DEFAULT_SEM_KEY + 102, ACE_SV_Semaphore_Complex::ACE_CREATE, 2);
  c = ck_fold (c, sc.acquire ());
  c = ck_fold (c, sc.acquire ());
  c = ck_fold (c, sc.tryacquire () == -1 ? 1 : 0);
  c = ck_fold (c, sc.release ());
  c = ck_fold (c, sc.release ());
  c = ck_fold (c, sc.remove ());
  ACE_SV_Message_Queue mq (IPC_PRIVATE, ACE_SV_Message_Queue::ACE_CREATE);
  struct Full_SV_Msg : public ACE_SV_Message
  {
    char text[32];
    Full_SV_Msg (void) : ACE_SV_Message (5) { ACE_OS::memset (text, 0, sizeof text); }
  } msg;
  ACE_OS::strsncpy (msg.text, "sysv-wrapped-message", 32);
  c = ck_fold (c, mq.send (msg, sizeof msg.text));
  Full_SV_Msg in;
  c = ck_fold (c, (long) mq.recv (in, sizeof in.text, 5));
  c = ck_str (c, in.text);
  c = ck_fold (c, mq.remove ());
  return (int) c;
}

// ACE_Message_Block: reference counting, duplication, continuation chains,
// read/write pointer discipline, crunch.
extern "C" int op_full_message_block (void)
{
  long c = 317;
  ACE_Message_Block *mb = new ACE_Message_Block (256);
  c = ck_fold (c, (long) mb->size ());
  c = ck_fold (c, (long) mb->space ());
  const char payload[] = "message-block-payload-0123456789";
  c = ck_fold (c, mb->copy (payload, sizeof payload - 1));
  c = ck_fold (c, (long) mb->length ());
  mb->rd_ptr (8);
  c = ck_fold (c, (long) mb->length ());
  c = ck_str (c, mb->rd_ptr ());
  ACE_Message_Block *dup = mb->duplicate ();
  c = ck_fold (c, dup->reference_count ());
  c = ck_fold (c, dup->rd_ptr () == mb->rd_ptr () ? 1 : 0);   // shared data block
  ACE_Message_Block *clone = mb->clone ();
  c = ck_fold (c, clone->reference_count ());                  // deep copy
  c = ck_fold (c, clone->rd_ptr () != mb->rd_ptr () ? 1 : 0);
  c = ck_fold (c, (long) clone->length ());
  clone->release ();
  dup->release ();
  c = ck_fold (c, mb->reference_count ());
  c = ck_fold (c, mb->crunch ());
  c = ck_fold (c, (long) (mb->rd_ptr () - mb->base ()));
  ACE_Message_Block *tail = new ACE_Message_Block (64);
  tail->copy ("tail", 4);
  mb->cont (tail);
  c = ck_fold (c, (long) mb->total_length ());
  mb->release ();                                              // releases chain
  return (int) c;
}

// ACE_Message_Queue<ACE_NULL_SYNCH>: FIFO + priority ordering, watermarks.
extern "C" int op_full_message_queue (void)
{
  long c = 331;
  ACE_Message_Queue<ACE_NULL_SYNCH> q;
  c = ck_fold (c, (long) q.high_water_mark ());
  for (long i = 0; i < 5; ++i)
    {
      ACE_Message_Block *mb = new ACE_Message_Block (16);
      mb->copy ((const char *) &i, sizeof i);
      mb->msg_priority ((unsigned long) (i % 3));
      c = ck_fold (c, q.enqueue_prio (mb));
    }
  c = ck_fold (c, (long) q.message_count ());
  c = ck_fold (c, (long) q.message_bytes ());
  long order = 0;
  for (ACE_Message_Block *mb = 0; q.dequeue_head (mb) != -1;)
    {
      long v = 0;
      ACE_OS::memcpy (&v, mb->rd_ptr (), sizeof v);
      order = order * 10 + v;
      mb->release ();
      if (q.message_count () == 0)
        break;
    }
  c = ck_fold (c, order);
  ACE_Message_Block *h = new ACE_Message_Block (8);
  h->copy ("H", 1);
  ACE_Message_Block *t = new ACE_Message_Block (8);
  t->copy ("T", 1);
  q.enqueue_tail (t);
  q.enqueue_head (h);
  ACE_Message_Block *out = 0;
  q.dequeue_head (out);
  c = ck_fold (c, out->rd_ptr ()[0]);
  out->release ();
  q.dequeue_head (out);
  c = ck_fold (c, out->rd_ptr ()[0]);
  out->release ();
  c = ck_fold (c, q.is_empty () ? 1 : 0);
  return (int) c;
}

// ACE_Log_Msg + ACE_Log_Record: priority machinery, and one captured
// STDERR emission (fixed format, no timestamps) read back from a file.
extern "C" int op_full_log_msg (void)
{
  long c = 337;
  ACE_Log_Msg *lm = ACE_LOG_MSG;
  u_long saved_mask = lm->priority_mask (ACE_Log_Msg::PROCESS);
  lm->priority_mask (LM_DEBUG | LM_ERROR, ACE_Log_Msg::PROCESS);
  c = ck_fold (c, lm->log_priority_enabled (LM_DEBUG) ? 1 : 0);
  c = ck_fold (c, lm->log_priority_enabled (LM_INFO) ? 1 : 0);
  lm->op_status (-7);
  c = ck_fold (c, lm->op_status ());
  lm->errnum (EBADF);
  c = ck_fold (c, lm->errnum ());
  lm->linenum (424);
  c = ck_fold (c, lm->linenum ());
  // capture one fixed message via redirected stderr
  char cap[] = "/tmp/ace_full_log_XXXXXX";
  ACE_HANDLE cfd = ACE_OS::mkstemp (cap);
  int saved_err = ACE_OS::dup (ACE_STDERR);
  ACE_OS::dup2 (cfd, ACE_STDERR);
  ACE_DEBUG ((LM_DEBUG, "%s|%d|%c\n", "log-vector", 4471, 'Z'));
  ACE_ERROR ((LM_INFO, "suppressed: mask excludes LM_INFO\n"));
  ACE_OS::dup2 (saved_err, ACE_STDERR);
  ACE_OS::close (saved_err);
  ACE_OS::lseek (cfd, 0, SEEK_SET);
  char text[256];
  ssize_t n = ACE_OS::read (cfd, text, sizeof text - 1);
  text[n > 0 ? n : 0] = 0;
  c = ck_str (c, text);
  ACE_OS::close (cfd);
  ACE_OS::unlink (cap);
  lm->priority_mask (saved_mask, ACE_Log_Msg::PROCESS);
  // ACE_Log_Record: name mapping and payload accessors
  ACE_Log_Record rec (LM_ERROR, 1234567890L, 42);
  rec.msg_data ("record-payload");
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (ACE_Log_Record::priority_name (LM_ERROR)));
  c = ck_fold (c, (long) rec.type ());
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (rec.msg_data ()));
  c = ck_fold (c, (long) rec.length () > 0 ? 1 : 0);
  return (int) c;
}

// ACE_Trace: nesting depth bookkeeping (output disabled).
extern "C" int op_full_trace (void)
{
  long c = 347;
  ACE_Trace::stop_tracing ();
  c = ck_fold (c, ACE_Trace::is_tracing () ? 1 : 0);
  c = ck_fold (c, ACE_Trace::get_nesting_indent ());
  {
    ACE_Trace t1 (ACE_TEXT ("op_full_trace"), __LINE__, ACE_TEXT (__FILE__));
    {
      ACE_Trace t2 (ACE_TEXT ("inner"), __LINE__, ACE_TEXT (__FILE__));
      c = ck_fold (c, 1);
    }
  }
  ACE_Trace::start_tracing ();
  c = ck_fold (c, ACE_Trace::is_tracing () ? 1 : 0);
  ACE_Trace::stop_tracing ();
  return (int) c;
}

// ACE_Read_Buffer: terminator-driven segmented reads with replacement.
extern "C" int op_full_read_buffer (void)
{
  long c = 349;
  char path[] = "/tmp/ace_full_rdbuf_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  const char content[] = "alpha\nbeta-longer-line\ngamma\n";
  ACE_OS::write (fd, content, sizeof content - 1);
  ACE_OS::lseek (fd, 0, SEEK_SET);
  FILE *fp = ACE_OS::fdopen (fd, "r");
  ACE_Read_Buffer rb (fp);
  for (int i = 0; i < 3; ++i)
    {
      char *seg = rb.read ('\n', '\n', '\0');
      if (!seg)
        break;
      c = ck_str (c, seg);
      c = ck_fold (c, (long) rb.size ());
      c = ck_fold (c, (long) rb.replaced ());
      rb.alloc ()->free (seg);
    }
  ACE_OS::fclose (fp);
  ACE_OS::unlink (path);
  return (int) c;
}

// ===========================================================================
// batch 2C: addressing, sockets (loopback), pipes/FIFOs, processes,
// file locking, CDR streams, utility components.
// ===========================================================================
#include "ace/INET_Addr.h"
#include "ace/Multihomed_INET_Addr.h"
#include "ace/UNIX_Addr.h"
#include "ace/DEV_Addr.h"
#include "ace/FILE_Addr.h"
#include "ace/MEM_Addr.h"
#include "ace/SPIPE_Addr.h"
#include "ace/SOCK_Acceptor.h"
#include "ace/SOCK_Connector.h"
#include "ace/SOCK_Stream.h"
#include "ace/SOCK_Dgram.h"
#include "ace/SOCK_CODgram.h"
#include "ace/LSOCK_Acceptor.h"
#include "ace/LSOCK_Connector.h"
#include "ace/LSOCK_Stream.h"
#include "ace/LSOCK_Dgram.h"
#include "ace/Pipe.h"
#include "ace/FIFO_Send.h"
#include "ace/FIFO_Recv.h"
#include "ace/FIFO_Send_Msg.h"
#include "ace/FIFO_Recv_Msg.h"
#include "ace/Process.h"
#include "ace/Process_Manager.h"
#include "ace/Process_Mutex.h"
#include "ace/Process_Semaphore.h"
#include "ace/File_Lock.h"
#include "ace/RW_Process_Mutex.h"
#include "ace/CDR_Stream.h"
#include "ace/CDR_Size.h"
#include "ace/Get_Opt.h"
#include "ace/Arg_Shifter.h"
#include "ace/Argv_Type_Converter.h"
#include "ace/High_Res_Timer.h"
#include "ace/Profile_Timer.h"
#include "ace/System_Time.h"
#include "ace/Stats.h"
#include "ace/Basic_Stats.h"
#include "ace/Throughput_Stats.h"
#include "ace/Sample_History.h"
#include "ace/Time_Policy.h"
#include "ace/Monotonic_Time_Policy.h"
#include "ace/Countdown_Time.h"
#include "ace/DLL.h"
#include "ace/DLL_Manager.h"
#include "ace/Init_ACE.h"
#include "ace/Flag_Manip.h"
#include "ace/Lib_Find.h"
#include "ace/Sock_Connect.h"
#include "ace/UUID.h"
#include "ace/Date_Time.h"
#include "ace/Codeset_Registry.h"
#include "ace/Capabilities.h"
#include "ace/Notification_Queue.h"
#include "ace/Event_Handler.h"

// Address hierarchy: INET (v4 text/binary round-trips), UNIX, DEV, FILE,
// MEM, SPIPE — every concrete ACE_Addr subclass constructible on Linux.
extern "C" int op_full_addrs (void)
{
  long c = 353;
  ACE_INET_Addr ia;
  c = ck_fold (c, ia.set (8080, "127.0.0.1"));
  c = ck_fold (c, ia.get_port_number ());
  c = ck_str (c, ia.get_host_addr ());
  c = ck_fold (c, (long) (ia.get_ip_address () & 0xff));
  ACE_INET_Addr ib ("127.0.0.1:9090");
  c = ck_fold (c, ib.get_port_number ());
  c = ck_fold (c, ia == ib ? 1 : 0);
  c = ck_fold (c, ia != ib ? 1 : 0);
  c = ck_fold (c, ia < ib ? 1 : 0);
  ACE_TCHAR abuf[64];
  c = ck_fold (c, ib.addr_to_string (abuf, 64));
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (abuf));
  ACE_INET_Addr any ((u_short) 0);
  c = ck_fold (c, any.is_any () ? 1 : 0);
  c = ck_fold (c, ia.is_loopback () ? 1 : 0);
  ACE_Multihomed_INET_Addr mh;
  const char *secondaries[] = { "127.0.0.2", "127.0.0.3" };
  c = ck_fold (c, mh.set (7000, "127.0.0.1", 1, AF_INET, secondaries, 2));
  c = ck_fold (c, (long) mh.get_num_secondary_addresses ());
  ACE_UNIX_Addr ua ("/tmp/ace_full_unix.sock");
  c = ck_str (c, ua.get_path_name ());
  c = ck_fold (c, (long) ua.get_size ());
  ACE_DEV_Addr da (ACE_TEXT ("/dev/null"));
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (da.get_path_name ()));
  ACE_FILE_Addr fa (ACE_TEXT ("/tmp/ace_full_file.dat"));
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (fa.get_path_name ()));
  ACE_MEM_Addr ma ((u_short) 6100);
  c = ck_fold (c, ma.get_port_number ());
  ACE_SPIPE_Addr sa (ACE_TEXT ("/tmp/ace_full_spipe"));
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (sa.get_path_name ()));
  ACE_Addr &base = ia;
  c = ck_fold (c, base.get_type ());
  return (int) c;
}

// TCP loopback: acceptor/connector/stream with bidirectional send_n/recv_n
// (single-threaded: connect completes against the listen backlog).
extern "C" int op_full_sock_stream (void)
{
  long c = 359;
  ACE_INET_Addr listen_addr ((u_short) 0, "127.0.0.1");
  ACE_SOCK_Acceptor acceptor;
  if (acceptor.open (listen_addr, 1) == -1)
    return -1;
  ACE_INET_Addr bound;
  acceptor.get_local_addr (bound);
  c = ck_fold (c, bound.get_port_number () > 0 ? 1 : 0);   // never fold the port
  ACE_SOCK_Connector connector;
  ACE_SOCK_Stream client;
  ACE_INET_Addr target (bound.get_port_number (), "127.0.0.1");
  c = ck_fold (c, connector.connect (client, target));
  ACE_SOCK_Stream server;
  c = ck_fold (c, acceptor.accept (server));
  unsigned char ping[128];
  fill_pattern (ping, sizeof ping);
  c = ck_fold (c, (long) client.send_n (ping, sizeof ping));
  unsigned char got[128];
  c = ck_fold (c, (long) server.recv_n (got, sizeof got));
  c = ck_fold (c, ace_cksum (got, sizeof got));
  for (unsigned i = 0; i < sizeof got; ++i)
    got[i] = (unsigned char) (got[i] ^ 0xff);
  c = ck_fold (c, (long) server.send_n (got, sizeof got));
  unsigned char echo[128];
  c = ck_fold (c, (long) client.recv_n (echo, sizeof echo));
  c = ck_fold (c, ace_cksum (echo, sizeof echo));
  ACE_INET_Addr peer;
  c = ck_fold (c, server.get_remote_addr (peer));
  c = ck_str (c, peer.get_host_addr ());                    // 127.0.0.1
  c = ck_fold (c, client.close ());
  c = ck_fold (c, server.close ());
  c = ck_fold (c, acceptor.close ());
  return (int) c;
}

// UDP loopback: two bound dgram sockets exchanging datagrams, plus a
// connected dgram (CODgram) pair.
extern "C" int op_full_sock_dgram (void)
{
  long c = 367;
  ACE_INET_Addr a1 ((u_short) 0, "127.0.0.1"), a2 ((u_short) 0, "127.0.0.1");
  ACE_SOCK_Dgram d1, d2;
  if (d1.open (a1) == -1 || d2.open (a2) == -1)
    return -1;
  ACE_INET_Addr b1, b2;
  d1.get_local_addr (b1);
  d2.get_local_addr (b2);
  unsigned char msg[64];
  fill_pattern (msg, sizeof msg);
  c = ck_fold (c, (long) d1.send (msg, sizeof msg, b2));
  unsigned char in[64];
  ACE_INET_Addr from;
  c = ck_fold (c, (long) d2.recv (in, sizeof in, from));
  c = ck_fold (c, ace_cksum (in, sizeof in));
  c = ck_str (c, from.get_host_addr ());
  d1.close ();
  d2.close ();
  ACE_SOCK_CODgram cd1, cd2;
  ACE_INET_Addr ca ((u_short) 0, "127.0.0.1");
  if (cd2.open (ca) == -1)
    return -2;
  ACE_INET_Addr cb;
  cd2.get_local_addr (cb);
  if (cd1.open (cb, ACE_Addr::sap_any) == -1)   // connected to cd2
    return -3;
  c = ck_fold (c, (long) cd1.send (msg, 32));
  unsigned char cin[32];
  c = ck_fold (c, (long) cd2.recv (cin, sizeof cin));
  c = ck_fold (c, ace_cksum (cin, sizeof cin));
  cd1.close ();
  cd2.close ();
  return (int) c;
}

// UNIX-domain sockets: LSOCK stream pair with descriptor passing — the
// received fd reads the same pipe payload — plus an LSOCK datagram pair.
extern "C" int op_full_lsock (void)
{
  long c = 373;
  char sockdir[] = "/tmp/ace_full_ls_XXXXXX";
  if (!::mkdtemp (sockdir))
    return -1;
  char spath[512];
  ACE_OS::snprintf (spath, sizeof spath, "%s/s", sockdir);
  ACE_UNIX_Addr saddr (spath);
  ACE_LSOCK_Acceptor acceptor;
  if (acceptor.open (saddr) == -1)
    return -2;
  ACE_LSOCK_Connector connector;
  ACE_LSOCK_Stream client, server;
  c = ck_fold (c, connector.connect (client, saddr));
  c = ck_fold (c, acceptor.accept (server));
  c = ck_fold (c, (long) client.send_n ("lsock-payload", 13));
  char buf[16];
  c = ck_fold (c, (long) server.recv_n (buf, 13));
  buf[13] = 0;
  c = ck_str (c, buf);
  // descriptor passing: send a pipe's read end across the socket
  ACE_HANDLE pfd[2];
  ACE_OS::pipe (pfd);
  ACE_OS::write (pfd[1], "fd-pass-vector", 14);
  c = ck_fold (c, client.send_handle (pfd[0]));
  ACE_HANDLE recvd = ACE_INVALID_HANDLE;
  c = ck_fold (c, server.recv_handle (recvd));
  char fdbuf[16];
  ssize_t n = ACE_OS::read (recvd, fdbuf, 14);
  c = ck_fold (c, (long) n);
  fdbuf[n > 0 ? n : 0] = 0;
  c = ck_str (c, fdbuf);
  ACE_OS::close (recvd);
  ACE_OS::close (pfd[0]);
  ACE_OS::close (pfd[1]);
  client.close ();
  server.close ();
  acceptor.close ();
  ACE_OS::unlink (spath);
  // LSOCK datagrams
  char d1p[512], d2p[512];
  ACE_OS::snprintf (d1p, sizeof d1p, "%s/d1", sockdir);
  ACE_OS::snprintf (d2p, sizeof d2p, "%s/d2", sockdir);
  ACE_LSOCK_Dgram dg1, dg2;
  c = ck_fold (c, dg1.open (ACE_UNIX_Addr (d1p)));
  c = ck_fold (c, dg2.open (ACE_UNIX_Addr (d2p)));
  c = ck_fold (c, (long) dg1.send ("dgram-msg", 9, ACE_UNIX_Addr (d2p)));
  char dbuf[16];
  ACE_Addr junk = ACE_Addr::sap_any;
  ACE_UNIX_Addr dfrom;
  c = ck_fold (c, (long) dg2.recv (dbuf, 9, dfrom));
  dbuf[9] = 0;
  c = ck_str (c, dbuf);
  dg1.close ();
  dg2.close ();
  ACE_OS::unlink (d1p);
  ACE_OS::unlink (d2p);
  ACE_OS::rmdir (sockdir);
  return (int) c;
}

// ACE_Pipe: stream-pipe pair round-trip.
extern "C" int op_full_pipe (void)
{
  long c = 379;
  ACE_Pipe pipe;
  c = ck_fold (c, pipe.open ());
  c = ck_fold (c, pipe.read_handle () != ACE_INVALID_HANDLE ? 1 : 0);
  c = ck_fold (c, pipe.write_handle () != ACE_INVALID_HANDLE ? 1 : 0);
  unsigned char pat[80];
  fill_pattern (pat, sizeof pat);
  c = ck_fold (c, (long) ACE_OS::write (pipe.write_handle (), pat, sizeof pat));
  unsigned char in[80];
  c = ck_fold (c, (long) ACE_OS::read (pipe.read_handle (), in, sizeof in));
  c = ck_fold (c, ace_cksum (in, sizeof in));
  c = ck_fold (c, pipe.close ());
  return (int) c;
}

// Named FIFOs: raw byte stream plus the record-oriented _Msg variants.
extern "C" int op_full_fifo (void)
{
  long c = 383;
  char fdir[] = "/tmp/ace_full_fifo_XXXXXX";
  if (!::mkdtemp (fdir))
    return -1;
  char fpath[512];
  ACE_OS::snprintf (fpath, sizeof fpath, "%s/f", fdir);
  // open the read end first (nonblocking), then the writer
  ACE_FIFO_Recv rx;
  c = ck_fold (c, rx.open (ACE_TEXT_CHAR_TO_TCHAR (fpath), O_RDONLY | O_NONBLOCK, 0666, 1));
  ACE_FIFO_Send tx;
  c = ck_fold (c, tx.open (ACE_TEXT_CHAR_TO_TCHAR (fpath), O_WRONLY, 0666));
  c = ck_fold (c, (long) tx.send ("fifo-bytes", 10));
  char buf[16];
  ssize_t n = 0;
  for (int spin = 0; spin < 100 && n <= 0; ++spin)
    {
      n = rx.recv (buf, 10);
      if (n <= 0)
        ACE_OS::sleep (ACE_Time_Value (0, 10000));
    }
  c = ck_fold (c, (long) n);
  buf[n > 0 ? n : 0] = 0;
  c = ck_str (c, buf);
  tx.close ();
  rx.close ();
  ACE_OS::unlink (fpath);
  // record-oriented: ACE_Str_Buf framed messages
  char mpath[512];
  ACE_OS::snprintf (mpath, sizeof mpath, "%s/m", fdir);
  ACE_FIFO_Recv_Msg mrx;
  c = ck_fold (c, mrx.open (ACE_TEXT_CHAR_TO_TCHAR (mpath), O_RDONLY | O_NONBLOCK, 0666, 1));
  ACE_FIFO_Send_Msg mtx;
  c = ck_fold (c, mtx.open (ACE_TEXT_CHAR_TO_TCHAR (mpath), O_WRONLY, 0666));
  ACE_Str_Buf out ((char *) "framed-message", 14);
  c = ck_fold (c, (long) mtx.send (out));
  char mbuf[64];
  ACE_Str_Buf in (mbuf, 0, sizeof mbuf);
  ssize_t mn = -1;
  for (int spin = 0; spin < 100 && mn <= 0; ++spin)
    {
      mn = mrx.recv (in);
      if (mn <= 0)
        ACE_OS::sleep (ACE_Time_Value (0, 10000));
    }
  c = ck_fold (c, (long) mn);
  c = ck_fold (c, in.len);
  mbuf[in.len] = 0;
  c = ck_str (c, mbuf);
  mtx.close ();
  mrx.close ();
  ACE_OS::unlink (mpath);
  ACE_OS::rmdir (fdir);
  return (int) c;
}

// Process spawning: ACE_Process with redirected output, exit-code
// round-trip, and ACE_Process_Manager batch wait.
extern "C" int op_full_process (void)
{
  long c = 389;
  ACE_Process_Options opts;
  opts.command_line ("/bin/sh -c \"exit 23\"");
  ACE_Process proc;
  pid_t pid = proc.spawn (opts);
  c = ck_fold (c, pid > 0 ? 1 : 0);
  c = ck_fold (c, proc.wait () == pid ? 1 : 0);   // wait returns the pid
  c = ck_fold (c, proc.exit_code ());
  // capture child stdout through a redirected handle
  char cap[] = "/tmp/ace_full_proc_XXXXXX";
  ACE_HANDLE cfd = ACE_OS::mkstemp (cap);
  ACE_Process_Options opts2;
  opts2.command_line ("/bin/echo process-vector");
  opts2.set_handles (ACE_INVALID_HANDLE, cfd, ACE_INVALID_HANDLE);
  ACE_Process proc2;
  pid_t pid2 = proc2.spawn (opts2);
  c = ck_fold (c, proc2.wait () == pid2 ? 1 : 0);
  c = ck_fold (c, proc2.exit_code ());
  ACE_OS::lseek (cfd, 0, SEEK_SET);
  char text[64];
  ssize_t n = ACE_OS::read (cfd, text, sizeof text - 1);
  text[n > 0 ? n : 0] = 0;
  c = ck_str (c, text);
  ACE_OS::close (cfd);
  ACE_OS::unlink (cap);
  // Process_Manager: spawn two and wait for both
  ACE_Process_Manager pm (4);
  ACE_Process_Options o1, o2;
  o1.command_line ("/bin/true");
  o2.command_line ("/bin/sh -c \"exit 3\"");
  pid_t p1 = pm.spawn (o1);
  pid_t p2 = pm.spawn (o2);
  c = ck_fold (c, p1 > 0 && p2 > 0 ? 1 : 0);
  ACE_exitcode s1 = 0, s2 = 0;
  c = ck_fold (c, pm.wait (p1, &s1) == p1 ? 1 : 0);
  c = ck_fold (c, pm.wait (p2, &s2) == p2 ? 1 : 0);
  c = ck_fold (c, WEXITSTATUS (s1));
  c = ck_fold (c, WEXITSTATUS (s2));
  return (int) c;
}

// Cross-process synchronization primitives (single-process determinism:
// acquire/try/release cycles and a child-process contention proof).
extern "C" int op_full_process_sync (void)
{
  long c = 397;
  ACE_Process_Mutex pm ("ace_full_pmutex");
  c = ck_fold (c, pm.acquire ());
  c = ck_fold (c, pm.release ());
  c = ck_fold (c, pm.tryacquire ());
  c = ck_fold (c, pm.release ());
  pm.remove ();
  ACE_Process_Semaphore ps (1, "ace_full_psem");
  c = ck_fold (c, ps.acquire ());
  c = ck_fold (c, ps.tryacquire () == -1 ? 1 : 0);
  c = ck_fold (c, ps.release ());
  ps.remove ();
  char lpath[] = "/tmp/ace_full_flock_XXXXXX";
  ACE_HANDLE lfd = ACE_OS::mkstemp (lpath);
  ACE_OS::close (lfd);
  {
    ACE_File_Lock fl (ACE_TEXT_CHAR_TO_TCHAR (lpath), O_RDWR, 0644);
    c = ck_fold (c, fl.acquire_write ());
    // a forked child must FAIL to tryacquire while we hold the lock
    pid_t child = ACE_OS::fork ();
    if (child == 0)
      {
        ACE_File_Lock cfl (ACE_TEXT_CHAR_TO_TCHAR (lpath), O_RDWR, 0644);
        ACE_OS::exit (cfl.tryacquire_write () == -1 ? 11 : 22);
      }
    ACE_exitcode st = 0;
    ACE_OS::waitpid (child, &st, 0);
    c = ck_fold (c, WEXITSTATUS (st));                 // 11: contention seen
    c = ck_fold (c, fl.release ());
  }
  {
    ACE_RW_Process_Mutex rwm (ACE_TEXT_CHAR_TO_TCHAR (lpath));
    c = ck_fold (c, rwm.acquire_read ());
    c = ck_fold (c, rwm.release ());
    c = ck_fold (c, rwm.acquire_write ());
    c = ck_fold (c, rwm.release ());
    rwm.remove ();
  }
  ACE_OS::unlink (lpath);
  return (int) c;
}

// CDR streams: typed marshal/unmarshal round-trip, both byte orders,
// and the size-counting stream.
extern "C" int op_full_cdr_stream (void)
{
  long c = 401;
  ACE_OutputCDR out (512);
  out << ACE_CDR::Char ('K');
  out << ACE_CDR::Short (-1234);
  out << ACE_CDR::UShort (54321);
  out << ACE_CDR::Long (-100000);
  out << ACE_CDR::ULong (3000000000UL);
  out << ACE_CDR::LongLong (ACE_INT64_LITERAL (-5000000000));
  out << ACE_CDR::Float (2.5f);
  out << ACE_CDR::Double (-13.625);
  out << ACE_CDR::Boolean (true);
  out << "cdr-string-vector";
  c = ck_fold (c, out.good_bit ());
  c = ck_fold (c, (long) out.total_length ());
  const ACE_Message_Block *blk = out.begin ();
  c = ck_fold (c, ace_cksum ((const unsigned char *) blk->rd_ptr (),
                             (unsigned long) blk->length ()));
  ACE_InputCDR in (out);
  ACE_CDR::Char ch; ACE_CDR::Short sh; ACE_CDR::UShort ush;
  ACE_CDR::Long lg; ACE_CDR::ULong ulg; ACE_CDR::LongLong llg;
  ACE_CDR::Float fl; ACE_CDR::Double db; ACE_CDR::Boolean bl;
  ACE_CString str;
  in >> ch; in >> sh; in >> ush; in >> lg; in >> ulg; in >> llg;
  in >> fl; in >> db; in >> ACE_InputCDR::to_boolean (bl); in >> str;
  c = ck_fold (c, in.good_bit ());
  c = ck_fold (c, ch);
  c = ck_fold (c, sh);
  c = ck_fold (c, ush);
  c = ck_fold (c, lg);
  c = ck_fold (c, (long) (ulg % 100003UL));
  c = ck_fold (c, (long) (llg % 100003));
  c = ck_fold (c, (long) (fl * 4));
  c = ck_fold (c, (long) (db * 8));
  c = ck_fold (c, bl ? 1 : 0);
  c = ck_cstring (c, str);
  // opposite byte order round-trip exercises the swap paths
  ACE_OutputCDR out_sw (512, ACE_CDR_BYTE_ORDER ? 0 : 1);
  out_sw << ACE_CDR::Long (0x01020304);
  ACE_InputCDR in_sw (out_sw);
  in_sw.reset_byte_order (ACE_CDR_BYTE_ORDER ? 0 : 1);
  ACE_CDR::Long swv = 0;
  in_sw >> swv;
  c = ck_fold (c, swv);
  // size-counting stream agrees with the real encoded length
  ACE_SizeCDR sz;
  sz << ACE_CDR::Long (1);
  sz << "cdr-string-vector";
  sz << ACE_CDR::Double (2.0);
  c = ck_fold (c, (long) sz.total_length ());
  return (int) c;
}

// Command-line machinery: ACE_Get_Opt (short+long options in POSIX order),
// ACE_Arg_Shifter, ACE_Argv_Type_Converter.
extern "C" int op_full_get_opt (void)
{
  long c = 409;
  const char *argv_c[] = { "prog", "-a", "-b", "barg", "--flag", "--opt=oval",
                           "tail1", "tail2", 0 };
  int argc = 8;
  ACE_TCHAR **argv = (ACE_TCHAR **) argv_c;
  ACE_Get_Opt go (argc, argv, ACE_TEXT ("ab:"), 1, 0,
                  ACE_Get_Opt::PERMUTE_ARGS);
  go.long_option (ACE_TEXT ("flag"), ACE_Get_Opt::NO_ARG);
  go.long_option (ACE_TEXT ("opt"), ACE_Get_Opt::ARG_REQUIRED);
  for (int o; (o = go ()) != -1;)
    {
      c = ck_fold (c, o);
      if (go.opt_arg ())
        c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (go.opt_arg ()));
      if (o == 0 && go.long_option ())
        c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (go.long_option ()));
    }
  c = ck_fold (c, go.opt_ind ());
  for (int i = go.opt_ind (); i < argc; ++i)
    c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (argv[i]));
  const char *sh_argv_c[] = { "prog", "-keep", "value", "-drop", "rest", 0 };
  int sh_argc = 5;
  ACE_TCHAR **sh_argv = (ACE_TCHAR **) sh_argv_c;
  ACE_Arg_Shifter as (sh_argc, sh_argv);
  while (as.is_anything_left ())
    {
      if (as.cur_arg_strncasecmp (ACE_TEXT ("-keep")) == 0)
        {
          c = ck_fold (c, 100);
          as.consume_arg ();
          c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (as.get_current ()));
          as.consume_arg ();
        }
      else if (as.cur_arg_strncasecmp (ACE_TEXT ("-drop")) == 0)
        as.ignore_arg ();
      else
        as.ignore_arg ();
    }
  ACE_Argv_Type_Converter conv (sh_argc, sh_argv);
  c = ck_fold (c, conv.get_argc ());
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (conv.get_TCHAR_argv ()[0]));
  return (int) c;
}

// Timers and statistics: invariant-only for real clocks, exact folds for
// fabricated samples.
extern "C" int op_full_timers_stats (void)
{
  long c = 419;
  ACE_High_Res_Timer::global_scale_factor_type sf =
    ACE_High_Res_Timer::global_scale_factor ();
  c = ck_fold (c, sf > 0 ? 1 : 0);
  ACE_High_Res_Timer hrt;
  hrt.start ();
  volatile long spin = 0;
  for (int i = 0; i < 10000; ++i)
    spin += i;
  hrt.stop ();
  ACE_Time_Value el;
  hrt.elapsed_time (el);
  c = ck_fold (c, el >= ACE_Time_Value::zero ? 1 : 0);
  ACE_Profile_Timer pt;
  pt.start ();
  for (int i = 0; i < 10000; ++i)
    spin += i;
  pt.stop ();
  ACE_Profile_Timer::ACE_Elapsed_Time et;
  c = ck_fold (c, pt.elapsed_time (et));
  c = ck_fold (c, et.real_time >= 0 ? 1 : 0);
  time_t st = 0;
  c = ck_fold (c, ACE_System_Time::get_local_system_time (st));
  c = ck_fold (c, st > 1600000000 ? 1 : 0);
  ACE_Stats stats;
  static const int samples[] = { 10, 20, 30, 40, 50 };
  for (unsigned i = 0; i < 5; ++i)
    stats.sample (samples[i]);
  ACE_Stats_Value mean (2);
  stats.mean (mean);
  c = ck_fold (c, (long) mean.whole ());
  c = ck_fold (c, (long) mean.fractional ());
  ACE_Basic_Stats bs;
  for (unsigned i = 0; i < 5; ++i)
    bs.sample (samples[i] * 100);
  c = ck_fold (c, (long) bs.samples_count ());
  c = ck_fold (c, (long) bs.min_);
  c = ck_fold (c, (long) bs.max_);
  ACE_Throughput_Stats ts;
  for (int i = 1; i <= 4; ++i)
    ts.sample (i * 1000, i);
  c = ck_fold (c, (long) ts.samples_count ());
  ACE_Sample_History hist (8);
  for (int i = 0; i < 8; ++i)
    hist.sample (i * 7);
  c = ck_fold (c, (long) hist.sample_count ());
  c = ck_fold (c, (long) hist.get_sample (3));
  return (int) c;
}

// Time policies: system/monotonic invariants, plus an exact fold through
// a fixed function-pointer policy.
static ACE_Time_Value full_fixed_now (void)
{
  return ACE_Time_Value (1234, 5678);
}
extern "C" int op_full_time_policy (void)
{
  long c = 421;
  ACE_System_Time_Policy stp;
  ACE_Time_Value t1 = stp ();
  c = ck_fold (c, t1.sec () > 1600000000 ? 1 : 0);
  ACE_Monotonic_Time_Policy mtp;
  ACE_Time_Value m1 = mtp ();
  ACE_Time_Value m2 = mtp ();
  c = ck_fold (c, m2 >= m1 ? 1 : 0);
  ACE_FPointer_Time_Policy fpt (full_fixed_now);
  ACE_Time_Value ft = fpt ();
  c = ck_fold (c, (long) ft.sec ());
  c = ck_fold (c, (long) ft.usec ());
  ACE_Countdown_Time cd (0);
  ACE_Time_Value budget (2, 0);
  ACE_Countdown_Time cd2 (&budget);
  volatile long spin = 0;
  for (int i = 0; i < 100000; ++i)
    spin += i;
  cd2.update ();
  c = ck_fold (c, budget <= ACE_Time_Value (2, 0) ? 1 : 0);
  c = ck_fold (c, budget > ACE_Time_Value (0, 0) ? 1 : 0);
  return (int) c;
}

// Dynamic linking: ACE_DLL over libm, symbol resolution and call-through,
// DLL_Manager refcounting.
extern "C" int op_full_dll (void)
{
  long c = 431;
  ACE_DLL dll;
  c = ck_fold (c, dll.open (ACE_TEXT ("libm.so.6"), ACE_DEFAULT_SHLIB_MODE, false));
  typedef double (*cos_fn) (double);
  cos_fn cf = (cos_fn) (intptr_t) dll.symbol (ACE_TEXT ("cos"));
  c = ck_fold (c, cf != 0 ? 1 : 0);
  if (cf)
    c = ck_fold (c, (long) (cf (0.0) * 1000));
  c = ck_fold (c, dll.symbol (ACE_TEXT ("no_such_symbol")) == 0 ? 1 : 0);
  ACE_DLL dll2;
  c = ck_fold (c, dll2.open (ACE_TEXT ("libm.so.6"), ACE_DEFAULT_SHLIB_MODE, false));
  c = ck_fold (c, dll2.close ());
  c = ck_fold (c, dll.close ());
  return (int) c;
}

// ACE init/fini, fd flag manipulation, library search, interface listing.
extern "C" int op_full_ace_misc2 (void)
{
  long c = 433;
  c = ck_fold (c, ACE::init ());
  c = ck_fold (c, ACE::fini ());
  ACE_HANDLE pfd[2];
  ACE_OS::pipe (pfd);
  c = ck_fold (c, ACE::set_flags (pfd[0], ACE_NONBLOCK));
  int fl = ACE_OS::fcntl (pfd[0], F_GETFL);
  c = ck_fold (c, (fl & O_NONBLOCK) ? 1 : 0);
  c = ck_fold (c, ACE::clr_flags (pfd[0], ACE_NONBLOCK));
  fl = ACE_OS::fcntl (pfd[0], F_GETFL);
  c = ck_fold (c, (fl & O_NONBLOCK) ? 1 : 0);
  ACE_OS::close (pfd[0]);
  ACE_OS::close (pfd[1]);
  ACE_TCHAR pathbuf[1024];
  c = ck_fold (c, ACE::ldfind (ACE_TEXT ("libm.so.6"), pathbuf, 1024));
  size_t if_count = 0;
  ACE_INET_Addr *ifs = 0;
  c = ck_fold (c, ACE::get_ip_interfaces (if_count, ifs));
  c = ck_fold (c, if_count > 0 ? 1 : 0);   // at least loopback
  delete [] ifs;
  c = ck_fold (c, ACE::ipv4_enabled () ? 1 : 0);
  return (int) c;
}

// UUIDs: structural invariants of generated values, exact round-trip of a
// fixed string form.
extern "C" int op_full_uuid (void)
{
  long c = 439;
  ACE_Utils::UUID_Generator gen;
  gen.init ();
  ACE_Utils::UUID u1, u2;
  gen.generate_UUID (u1);
  gen.generate_UUID (u2);
  const ACE_CString *s1 = u1.to_string ();
  const ACE_CString *s2 = u2.to_string ();
  c = ck_fold (c, (long) s1->length ());                  // canonical 36
  c = ck_fold (c, *s1 == *s2 ? 1 : 0);                    // must differ
  c = ck_fold (c, (*s1)[8] == '-' && (*s1)[13] == '-'
                  && (*s1)[18] == '-' && (*s1)[23] == '-' ? 1 : 0);
  ACE_Utils::UUID fixed (ACE_CString ("f81d4fae-7dec-11d0-a765-00a0c91e6bf6"));
  const ACE_CString *fs = fixed.to_string ();
  c = ck_str (c, fs->c_str ());
  ACE_Utils::UUID nil = ACE_Utils::UUID::NIL_UUID;
  c = ck_str (c, nil.to_string ()->c_str ());
  return (int) c;
}

// Calendar decomposition of a fixed epoch via ACE_Date_Time.
extern "C" int op_full_date_time (void)
{
  long c = 443;
  ACE_Time_Value fixed ((time_t) 1700000000, 250000);
  ACE_Date_Time dt (fixed);
  c = ck_fold (c, dt.year ());
  c = ck_fold (c, (long) dt.month ());
  c = ck_fold (c, (long) dt.day ());
  c = ck_fold (c, (long) dt.hour ());
  c = ck_fold (c, (long) dt.minute ());
  c = ck_fold (c, (long) dt.second ());
  c = ck_fold (c, (long) dt.microsec ());
  c = ck_fold (c, (long) dt.weekday ());
  return (int) c;
}

// Codeset registry: locale/codeset mapping surface.
extern "C" int op_full_codeset (void)
{
  long c = 449;
  ACE_CDR::ULong id = 0;
  c = ck_fold (c, ACE_Codeset_Registry::locale_to_registry (ACE_CString ("C"), id));
  c = ck_fold (c, (long) (id % 100003UL));
  c = ck_fold (c, (long) ACE_Codeset_Registry::get_max_bytes (id));
  return (int) c;
}

// Capabilities file parsing: getval over a written caps database.
extern "C" int op_full_capabilities (void)
{
  long c = 457;
  char path[] = "/tmp/ace_full_caps_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  const char caps[] =
    "printer:type=laser:dpi#600:duplex:\n"
    "scanner:type=flatbed:dpi#300:\n";
  ACE_OS::write (fd, caps, sizeof caps - 1);
  ACE_OS::close (fd);
  ACE_Capabilities cap;
  c = ck_fold (c, cap.getent (path, "printer"));
  ACE_TString tval;
  c = ck_fold (c, cap.getval ("type", tval));
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (tval.c_str ()));
  int ival = 0;
  c = ck_fold (c, cap.getval ("dpi", ival));
  c = ck_fold (c, ival);
  ACE_OS::unlink (path);
  return (int) c;
}

// Notification queue bookkeeping over a dummy handler.
class Full_NQ_Handler : public ACE_Event_Handler
{
public:
  virtual int handle_input (ACE_HANDLE) { return 0; }
};
extern "C" int op_full_notification_queue (void)
{
  long c = 461;
  ACE_Notification_Queue nq;
  c = ck_fold (c, nq.open ());
  Full_NQ_Handler h;
  ACE_Notification_Buffer nb1 (&h, ACE_Event_Handler::READ_MASK);
  ACE_Notification_Buffer nb2 (&h, ACE_Event_Handler::WRITE_MASK);
  c = ck_fold (c, nq.push_new_notification (nb1));
  c = ck_fold (c, nq.push_new_notification (nb2));
  ACE_Notification_Buffer out;
  bool more = false;
  c = ck_fold (c, nq.pop_next_notification (out, more, nb1));
  c = ck_fold (c, (long) out.mask_);
  nq.reset ();
  c = ck_fold (c, 1);
  return (int) c;
}

// ===========================================================================
// batch 2D: reactors and timers, proactor (POSIX aio), service
// configurator, configuration heap, naming, and the event plumbing.
// ===========================================================================
#include "ace/Select_Reactor.h"
#include "ace/TP_Reactor.h"
#include "ace/Dev_Poll_Reactor.h"
#include "ace/Priority_Reactor.h"
#include "ace/Reactor.h"
#include "ace/Reactor_Notification_Strategy.h"
#include "ace/Timer_Heap.h"
#include "ace/Timer_List.h"
#include "ace/Timer_Wheel.h"
#include "ace/Timer_Hash.h"
#include "ace/Timer_Queue_Adapters.h"
#include "ace/Proactor.h"
#include "ace/POSIX_Proactor.h"
#include "ace/Asynch_IO.h"
#include "ace/Service_Config.h"
#include "ace/Service_Object.h"
#include "ace/Service_Types.h"
#include "ace/Service_Repository.h"
#include "ace/Service_Gestalt.h"
#include "ace/Dynamic_Service.h"
#include "ace/Configuration.h"
#include "ace/Configuration_Import_Export.h"
#include "ace/Naming_Context.h"
#include "ace/Name_Request_Reply.h"
#include "ace/Event.h"
#include "ace/Manual_Event.h"
#include "ace/Auto_Event.h"

// A counting event handler: pipe-readable events, timers with recorded
// ids, and reactor notifications all fold into one sequence.
class Full_EH : public ACE_Event_Handler
{
public:
  Full_EH (void) : seq_ (1), reads_ (0) {}
  virtual int handle_input (ACE_HANDLE h)
  {
    char b[8];
    ssize_t n = ACE_OS::read (h, b, sizeof b);
    this->seq_ = this->seq_ * 31 + 3 + n;
    ++this->reads_;
    return 0;
  }
  virtual int handle_timeout (const ACE_Time_Value &, const void *act)
  {
    this->seq_ = this->seq_ * 31 + 7 + (long) (intptr_t) act;
    return 0;
  }
  virtual int handle_exception (ACE_HANDLE)
  {
    this->seq_ = this->seq_ * 31 + 11;
    return 0;
  }
  long seq_;
  int reads_;
};

// One deterministic reactor scenario, reused across every implementation:
// a primed pipe handler, a zero-delay timer with an ACT, and a notify().
static long full_reactor_scenario (ACE_Reactor_Impl *impl)
{
  ACE_Reactor reactor (impl, 1);
  Full_EH eh;
  ACE_Pipe pipe;
  if (pipe.open () == -1)
    return -1;
  ACE_OS::write (pipe.write_handle (), "!", 1);
  if (reactor.register_handler (pipe.read_handle (), &eh,
                                ACE_Event_Handler::READ_MASK) == -1)
    return -2;
  long tid = reactor.schedule_timer (&eh, (const void *) 5,
                                     ACE_Time_Value (0, 0));
  if (tid == -1)
    return -3;
  reactor.notify (&eh, ACE_Event_Handler::EXCEPT_MASK);
  for (int i = 0; i < 4; ++i)
    {
      ACE_Time_Value slice (0, 50000);
      reactor.handle_events (slice);
      if (eh.reads_ >= 1 && eh.seq_ % 31 != 1)
        ;                                   // keep draining remaining events
    }
  long tid2 = reactor.schedule_timer (&eh, (const void *) 9,
                                      ACE_Time_Value (10, 0));
  long cancelled = reactor.cancel_timer (tid2);
  reactor.remove_handler (pipe.read_handle (),
                          ACE_Event_Handler::READ_MASK | ACE_Event_Handler::DONT_CALL);
  pipe.close ();
  return eh.seq_ * 10 + eh.reads_ + cancelled * 3;
}

extern "C" int op_full_select_reactor (void)
{
  long c = 463;
  c = ck_fold (c, full_reactor_scenario (new ACE_Select_Reactor));
  return (int) c;
}

extern "C" int op_full_tp_reactor (void)
{
  long c = 467;
  c = ck_fold (c, full_reactor_scenario (new ACE_TP_Reactor));
  return (int) c;
}

extern "C" int op_full_dev_poll_reactor (void)
{
  long c = 479;
  c = ck_fold (c, full_reactor_scenario (new ACE_Dev_Poll_Reactor));
  return (int) c;
}

extern "C" int op_full_priority_reactor (void)
{
  long c = 487;
  c = ck_fold (c, full_reactor_scenario (new ACE_Priority_Reactor));
  return (int) c;
}

// Reactor notification strategy: a message queue wired to notify the
// reactor on enqueue.
extern "C" int op_full_notification_strategy (void)
{
  long c = 491;
  ACE_Select_Reactor *sr = new ACE_Select_Reactor;
  ACE_Reactor reactor (sr, 1);
  Full_EH eh;
  ACE_Reactor_Notification_Strategy strat (&reactor, &eh,
                                           ACE_Event_Handler::EXCEPT_MASK);
  c = ck_fold (c, strat.notify ());
  ACE_Time_Value slice (0, 50000);
  reactor.handle_events (slice);
  c = ck_fold (c, eh.seq_);
  c = ck_fold (c, strat.notify (&eh, ACE_Event_Handler::EXCEPT_MASK));
  reactor.handle_events (slice);
  c = ck_fold (c, eh.seq_);
  return (int) c;
}

// Timer queues driven by explicit expiry times: every implementation must
// dispatch the same sequence for the same schedule.
static long full_timer_queue_scenario (ACE_Timer_Queue *tq)
{
  Full_EH eh;
  const ACE_Time_Value base (1000000, 0);       // fabricated absolute time
  tq->gettimeofday (0);                          // keep default; expiries are absolute
  long id1 = tq->schedule (&eh, (const void *) 1, base + ACE_Time_Value (0, 100));
  long id2 = tq->schedule (&eh, (const void *) 2, base + ACE_Time_Value (0, 50));
  long id3 = tq->schedule (&eh, (const void *) 3, base + ACE_Time_Value (0, 200),
                           ACE_Time_Value (0, 100));   // repeating
  long id4 = tq->schedule (&eh, (const void *) 4, base + ACE_Time_Value (5, 0));
  long acc = tq->is_empty () ? 0 : 1;
  acc = acc * 7 + (tq->earliest_time () == base + ACE_Time_Value (0, 50) ? 1 : 0);
  acc = acc * 7 + tq->expire (base + ACE_Time_Value (0, 150));   // fires id2, id1
  acc = acc * 7 + tq->expire (base + ACE_Time_Value (0, 350));   // id3 twice (repeat)
  acc = acc * 7 + tq->cancel (id4);
  acc = acc * 7 + tq->cancel (id3);
  acc = acc * 7 + (tq->is_empty () ? 1 : 0);
  ACE_UNUSED_ARG (id1);
  ACE_UNUSED_ARG (id2);
  return acc * 100003 + eh.seq_ % 100003;
}

extern "C" int op_full_timer_queues (void)
{
  long c = 499;
  {
    ACE_Timer_Heap th;
    c = ck_fold (c, full_timer_queue_scenario (&th));
  }
  {
    ACE_Timer_List tl;
    c = ck_fold (c, full_timer_queue_scenario (&tl));
  }
  {
    ACE_Timer_Wheel tw;
    c = ck_fold (c, full_timer_queue_scenario (&tw));
  }
  {
    ACE_Timer_Hash thh;
    c = ck_fold (c, full_timer_queue_scenario (&thh));
  }
  return (int) c;
}

// POSIX proactor: asynchronous file read dispatched through the proactor
// event loop (aio on a regular file).
class Full_Read_Handler : public ACE_Handler
{
public:
  Full_Read_Handler (void) : done_ (0), sum_ (0) {}
  virtual void handle_read_file (const ACE_Asynch_Read_File::Result &result)
  {
    this->sum_ = (long) result.bytes_transferred ();
    ACE_Message_Block &mb = result.message_block ();
    this->sum_ = this->sum_ * 100003
                 + ace_cksum ((const unsigned char *) mb.rd_ptr (),
                              (unsigned long) mb.length ());
    this->done_ = 1;
  }
  int done_;
  long sum_;
};
extern "C" int op_full_proactor (void)
{
  long c = 503;
  char path[] = "/tmp/ace_full_aio_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  unsigned char pat[256];
  fill_pattern (pat, sizeof pat);
  ACE_OS::write (fd, pat, sizeof pat);
  ACE_OS::close (fd);
  // the AIOCB proactor logs its slot bookkeeping (with pid) at both
  // construction and destruction; silence process-level logging for the
  // proactor's whole lifetime (inner scope), restore after destruction
  u_long full_pro_mask = ACE_LOG_MSG->priority_mask (ACE_Log_Msg::PROCESS);
  ACE_LOG_MSG->priority_mask (0, ACE_Log_Msg::PROCESS);
  {
    ACE_POSIX_AIOCB_Proactor *impl = new ACE_POSIX_AIOCB_Proactor;
    ACE_Proactor proactor (impl, 1);
    ACE_HANDLE afd = ACE_OS::open (path, O_RDONLY);
    Full_Read_Handler handler;
    handler.proactor (&proactor);
    ACE_Asynch_Read_File arf;
    c = ck_fold (c, arf.open (handler, afd, 0, &proactor));
    ACE_Message_Block *mb = new ACE_Message_Block (512);
    c = ck_fold (c, arf.read (*mb, 256, 0, 0));
    for (int i = 0; i < 200 && !handler.done_; ++i)
      {
        ACE_Time_Value slice (0, 20000);
        proactor.handle_events (slice);
      }
    c = ck_fold (c, handler.done_);
    c = ck_fold (c, handler.sum_ % 1000003);
    mb->release ();
    ACE_OS::close (afd);
    ACE_OS::unlink (path);
  }
  ACE_LOG_MSG->priority_mask (full_pro_mask, ACE_Log_Msg::PROCESS);
  return (int) c;
}

// Service configurator: a static service defined HERE, registered through
// a private gestalt via real svc.conf directives (lexer + yacc parser +
// Parse_Node execution), then suspended/resumed/removed.
class Full_Service : public ACE_Service_Object
{
public:
  Full_Service (void) : state_ (0) {}
  virtual int init (int argc, ACE_TCHAR *argv[])
  {
    this->state_ = 100 + argc;
    for (int i = 0; i < argc; ++i)
      this->state_ = this->state_ * 31
        + ck_str (0, ACE_TEXT_ALWAYS_CHAR (argv[i])) % 1009;
    return 0;
  }
  virtual int fini (void)
  {
    this->state_ = -1;
    return 0;
  }
  virtual int suspend (void)
  {
    this->state_ += 1000;
    return 0;
  }
  virtual int resume (void)
  {
    this->state_ += 10000;
    return 0;
  }
  virtual int info (ACE_TCHAR **info_string, size_t length) const
  {
    if (info_string)
      *info_string = ACE::strnew (ACE_TEXT ("Full_Service info"));
    ACE_UNUSED_ARG (length);
    return 0;
  }
  int state_;
};

ACE_FACTORY_DEFINE (ACE_Local_Service, Full_Service)

extern "C" int op_full_service_config (void)
{
  long c = 509;
  u_long full_sc_mask = ACE_LOG_MSG->priority_mask (ACE_Log_Msg::PROCESS);
  ACE_LOG_MSG->priority_mask (0, ACE_Log_Msg::PROCESS);
  ACE_Service_Gestalt gestalt (ACE_Service_Gestalt::MAX_SERVICES, true);
  ACE_Static_Svc_Descriptor desc = {
    ACE_TEXT ("Full_Service"),
    ACE_SVC_OBJ_T,
    &ACE_SVC_NAME (Full_Service),
    ACE_Service_Type::DELETE_THIS | ACE_Service_Type::DELETE_OBJ,
    ACE_ACTIVE
  };
  // the descriptor overload REGISTERS the service in this repo; the text
  // directive then routes through the svc.conf lexer+parser to init it
  c = ck_fold (c, gestalt.process_directive (desc, 0));
  c = ck_fold (c, gestalt.process_directive (
    ACE_TEXT ("static Full_Service \"-alpha -beta gamma\"")));
  const ACE_Service_Type *st = 0;
  c = ck_fold (c, gestalt.find (ACE_TEXT ("Full_Service"), &st));
  Full_Service *svc = st
    ? (Full_Service *) ((ACE_Service_Type_Impl *) st->type ())->object ()
    : 0;
  c = ck_fold (c, svc ? svc->state_ : -99);
  c = ck_fold (c, gestalt.suspend (ACE_TEXT ("Full_Service")));
  c = ck_fold (c, svc ? svc->state_ : -99);
  c = ck_fold (c, gestalt.resume (ACE_TEXT ("Full_Service")));
  c = ck_fold (c, svc ? svc->state_ : -99);
  c = ck_fold (c, gestalt.process_directive (
    ACE_TEXT ("remove Full_Service")));
  c = ck_fold (c, gestalt.find (ACE_TEXT ("Full_Service"), &st) == -1 ? 1 : 0);
  c = ck_fold (c, gestalt.process_directive (
    ACE_TEXT ("this is not a valid directive")) == -1 ? 1 : 0);
  ACE_LOG_MSG->priority_mask (full_sc_mask, ACE_Log_Msg::PROCESS);
  return (int) c;
}

// Configuration heap: sections, typed values, enumeration, and the INI
// import/export round-trip (file content folded verbatim).
extern "C" int op_full_configuration (void)
{
  long c = 521;
  ACE_Configuration_Heap cfg;
  c = ck_fold (c, cfg.open ());
  ACE_Configuration_Section_Key root = cfg.root_section ();
  ACE_Configuration_Section_Key net, log;
  c = ck_fold (c, cfg.open_section (root, ACE_TEXT ("network"), 1, net));
  c = ck_fold (c, cfg.open_section (root, ACE_TEXT ("logging"), 1, log));
  c = ck_fold (c, cfg.set_string_value (net, ACE_TEXT ("host"), ACE_TEXT ("localhost")));
  c = ck_fold (c, cfg.set_integer_value (net, ACE_TEXT ("port"), 8080));
  c = ck_fold (c, cfg.set_string_value (log, ACE_TEXT ("level"), ACE_TEXT ("debug")));
  ACE_TString sval;
  c = ck_fold (c, cfg.get_string_value (net, ACE_TEXT ("host"), sval));
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (sval.c_str ()));
  u_int ival = 0;
  c = ck_fold (c, cfg.get_integer_value (net, ACE_TEXT ("port"), ival));
  c = ck_fold (c, (long) ival);
  ACE_TString sname;
  for (int i = 0; cfg.enumerate_sections (root, i, sname) == 0; ++i)
    c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (sname.c_str ()));
  ACE_TString vname;
  ACE_Configuration::VALUETYPE vtype;
  for (int i = 0; cfg.enumerate_values (net, i, vname, vtype) == 0; ++i)
    {
      c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (vname.c_str ()));
      c = ck_fold (c, vtype);
    }
  c = ck_fold (c, cfg.remove_value (log, ACE_TEXT ("level")));
  char ini[] = "/tmp/ace_full_ini_XXXXXX";
  ACE_HANDLE ifd = ACE_OS::mkstemp (ini);
  ACE_OS::close (ifd);
  ACE_Ini_ImpExp exp (cfg);
  c = ck_fold (c, exp.export_config (ACE_TEXT_CHAR_TO_TCHAR (ini)));
  ACE_HANDLE rfd = ACE_OS::open (ini, O_RDONLY);
  char text[512];
  ssize_t n = ACE_OS::read (rfd, text, sizeof text - 1);
  text[n > 0 ? n : 0] = 0;
  ACE_OS::close (rfd);
  c = ck_str (c, text);
  ACE_Configuration_Heap cfg2;
  cfg2.open ();
  ACE_Ini_ImpExp imp (cfg2);
  c = ck_fold (c, imp.import_config (ACE_TEXT_CHAR_TO_TCHAR (ini)));
  ACE_Configuration_Section_Key net2;
  c = ck_fold (c, cfg2.open_section (cfg2.root_section (), ACE_TEXT ("network"), 0, net2));
  ACE_TString hv;
  c = ck_fold (c, cfg2.get_string_value (net2, ACE_TEXT ("host"), hv));
  c = ck_str (c, ACE_TEXT_ALWAYS_CHAR (hv.c_str ()));
  ACE_OS::unlink (ini);
  return (int) c;
}

// Process-local naming: bind/rebind/resolve/unbind plus name enumeration,
// persisted in a temp-dir database.
extern "C" int op_full_naming (void)
{
  long c = 523;
  char ndir[] = "/tmp/ace_full_name_XXXXXX";
  if (!::mkdtemp (ndir))
    return -1;
  ACE_Naming_Context ctx;
  ACE_Name_Options *opts = ctx.name_options ();
  opts->namespace_dir (ACE_TEXT_CHAR_TO_TCHAR (ndir));
  opts->database (ACE_TEXT ("full_names"));
  opts->context (ACE_Naming_Context::PROC_LOCAL);
  c = ck_fold (c, ctx.open (ACE_Naming_Context::PROC_LOCAL));
  c = ck_fold (c, ctx.bind ("alpha", "one", "t1"));
  c = ck_fold (c, ctx.bind ("beta", "two", "t2"));
  c = ck_fold (c, ctx.bind ("alpha", "dup", "t3"));      // duplicate: fails
  char *value = 0, *type = 0;
  c = ck_fold (c, ctx.resolve ("alpha", value, type));
  c = ck_str (c, value);
  c = ck_str (c, type);
  delete [] value;
  delete [] type;
  c = ck_fold (c, ctx.rebind ("alpha", "uno", "t1b"));
  value = 0; type = 0;
  ctx.resolve ("alpha", value, type);
  c = ck_str (c, value);
  delete [] value;
  delete [] type;
  ACE_BINDING_SET names;
  c = ck_fold (c, ctx.list_name_entries (names, ""));
  c = ck_fold (c, (long) names.size ());
  c = ck_fold (c, ctx.unbind ("beta"));
  c = ck_fold (c, ctx.resolve ("beta", value, type));    // gone
  c = ck_fold (c, ctx.close ());
  // temp db files under ndir are removed with the directory
  char p[512];
  ACE_OS::snprintf (p, sizeof p, "%s/full_names", ndir);
  ACE_OS::unlink (p);
  ACE_OS::rmdir (ndir);
  return (int) c;
}

// Name protocol serialization: request/reply encode-decode round-trips
// (the wire format of the naming service, no server needed).
extern "C" int op_full_name_request (void)
{
  long c = 541;
  ACE_WCHAR_T wname[8], wvalue[8];
  for (int i = 0; i < 8; ++i)
    {
      wname[i] = (ACE_WCHAR_T) ('A' + i);
      wvalue[i] = (ACE_WCHAR_T) ('z' - i);
    }
  ACE_Name_Request req (ACE_Name_Request::BIND,
                        wname, 6 * sizeof (ACE_WCHAR_T),
                        wvalue, 6 * sizeof (ACE_WCHAR_T),
                        "typ", 3);
  void *xfer = 0;
  int enc_len = req.encode (xfer);
  c = ck_fold (c, enc_len > 0 ? 1 : 0);
  c = ck_fold (c, ace_cksum ((const unsigned char *) xfer,
                             (unsigned long) enc_len));
  ACE_Name_Request decoded;
  ACE_OS::memcpy ((void *) &decoded, xfer, (size_t) enc_len);
  c = ck_fold (c, decoded.decode ());
  c = ck_fold (c, (long) decoded.msg_type ());
  c = ck_fold (c, (long) decoded.name_len ());
  c = ck_fold (c, (long) decoded.value_len ());
  for (size_t i = 0; i < decoded.name_len () / sizeof (ACE_WCHAR_T); ++i)
    c = ck_fold (c, decoded.name ()[i]);
  c = ck_str (c, decoded.type ());
  ACE_Name_Reply rep (0, 42);
  void *rxfer = 0;
  int renc = rep.encode (rxfer);
  c = ck_fold (c, renc > 0 ? 1 : 0);
  ACE_Name_Reply rdec;
  ACE_OS::memcpy ((void *) &rdec, rxfer, (size_t) renc);
  c = ck_fold (c, rdec.decode ());
  c = ck_fold (c, (long) rdec.errnum ());
  return (int) c;
}

// Event objects: manual and auto reset semantics.
extern "C" int op_full_event (void)
{
  long c = 547;
  ACE_Manual_Event me;
  c = ck_fold (c, me.signal ());
  c = ck_fold (c, me.wait ());              // stays signaled
  c = ck_fold (c, me.wait ());
  c = ck_fold (c, me.reset ());
  ACE_Time_Value past = ACE_OS::gettimeofday () - ACE_Time_Value (0, 200000);
  c = ck_fold (c, me.wait (&past) == -1 ? 1 : 0);   // timed out
  ACE_Auto_Event ae;
  c = ck_fold (c, ae.signal ());
  c = ck_fold (c, ae.wait ());              // consumed
  ACE_Time_Value past2 = ACE_OS::gettimeofday () - ACE_Time_Value (0, 200000);
  c = ck_fold (c, ae.wait (&past2) == -1 ? 1 : 0);
  c = ck_fold (c, ae.pulse ());
  return (int) c;
}

// ===========================================================================
// batch 2E: file/device/terminal transports, shared-memory and SPIPE/UPIPE
// transports, multicast/broadcast/netlink/ICMP sockets, monitors, tokens,
// and the small leaf components.
// ===========================================================================
#include "ace/FILE_Connector.h"
#include "ace/FILE_IO.h"
#include "ace/DEV_Connector.h"
#include "ace/DEV_IO.h"
#include "ace/TTY_IO.h"
#include "ace/MEM_Acceptor.h"
#include "ace/MEM_Connector.h"
#include "ace/MEM_Stream.h"
#include "ace/SPIPE_Acceptor.h"
#include "ace/SPIPE_Connector.h"
#include "ace/SPIPE_Stream.h"
#include "ace/UPIPE_Acceptor.h"
#include "ace/UPIPE_Connector.h"
#include "ace/UPIPE_Stream.h"
#include "ace/SOCK_Dgram_Mcast.h"
#include "ace/SOCK_Dgram_Bcast.h"
#include "ace/SOCK_SEQPACK_Acceptor.h"
#include "ace/SOCK_SEQPACK_Connector.h"
#include "ace/SOCK_SEQPACK_Association.h"
#include "ace/Ping_Socket.h"
#include "ace/Netlink_Addr.h"
#include "ace/SOCK_Netlink.h"
#include "ace/Token.h"
#include "ace/Filecache.h"
#include "ace/Sbrk_Memory_Pool.h"
#include "ace/Functor.h"
#include "ace/Dynamic.h"
#include "ace/Hashable.h"
#include "ace/Recyclable.h"
#include "ace/Stack_Trace.h"
#include "ace/Dump.h"
#include "ace/Dump_T.h"
#include "ace/Handle_Ops.h"
#include "ace/TSS_Adapter.h"
#include "ace/Thread_Hook.h"
#include "ace/OS_QoS.h"
#include "ace/Connection_Recycling_Strategy.h"
#include "ace/Log_Msg_Callback.h"
#include "ace/Log_Msg_UNIX_Syslog.h"
#include "ace/Logging_Strategy.h"
#include "ace/Log_Category.h"
#include "ace/Monitor_Size.h"
#include "ace/Monitor_Point_Registry.h"
#include "ace/Monitor_Control_Types.h"
#include "ace/Monitor_Control_Action.h"
#include "ace/Monitor_Admin.h"
#include "ace/Monitor_Admin_Manager.h"
#include "ace/IOStream.h"

// ACE_FILE_IO through ACE_FILE_Connector: file transport with block I/O.
extern "C" int op_full_file_io (void)
{
  long c = 557;
  char path[] = "/tmp/ace_full_fileio_XXXXXX";
  ACE_HANDLE tfd = ACE_OS::mkstemp (path);
  ACE_OS::close (tfd);
  ACE_FILE_Addr addr (ACE_TEXT_CHAR_TO_TCHAR (path));
  ACE_FILE_Connector connector;
  ACE_FILE_IO file;
  c = ck_fold (c, connector.connect (file, addr, 0, ACE_Addr::sap_any, 0,
                                     O_RDWR | O_CREAT, ACE_DEFAULT_FILE_PERMS));
  unsigned char pat[192];
  fill_pattern (pat, sizeof pat);
  c = ck_fold (c, (long) file.send_n (pat, sizeof pat));
  ACE_FILE_Info info;
  c = ck_fold (c, file.get_info (info));
  c = ck_fold (c, (long) info.size_);
  c = ck_fold (c, (long) file.seek (16, SEEK_SET));
  unsigned char in[64];
  c = ck_fold (c, (long) file.recv_n (in, sizeof in));
  c = ck_fold (c, ace_cksum (in, sizeof in));
  c = ck_fold (c, (long) file.tell ());
  c = ck_fold (c, file.truncate (100));
  file.get_info (info);
  c = ck_fold (c, (long) info.size_);
  c = ck_fold (c, file.close ());
  c = ck_fold (c, file.unlink ());
  return (int) c;
}

// ACE_DEV_IO on /dev/null (writes) and /dev/zero (reads) via DEV_Connector.
extern "C" int op_full_dev_io (void)
{
  long c = 563;
  ACE_DEV_Addr null_addr (ACE_TEXT ("/dev/null"));
  ACE_DEV_Connector connector;
  ACE_DEV_IO dev;
  c = ck_fold (c, connector.connect (dev, null_addr, 0, ACE_Addr::sap_any, 0,
                                     O_WRONLY));
  c = ck_fold (c, (long) dev.send_n ("into-the-void", 13));
  c = ck_fold (c, dev.close ());
  ACE_DEV_Addr zero_addr (ACE_TEXT ("/dev/zero"));
  ACE_DEV_IO zdev;
  c = ck_fold (c, connector.connect (zdev, zero_addr, 0, ACE_Addr::sap_any, 0,
                                     O_RDONLY));
  unsigned char buf[32];
  ACE_OS::memset (buf, 0xff, sizeof buf);
  c = ck_fold (c, (long) zdev.recv_n (buf, sizeof buf));
  long zsum = 0;
  for (unsigned i = 0; i < sizeof buf; ++i)
    zsum += buf[i];
  c = ck_fold (c, zsum);
  c = ck_fold (c, zdev.close ());
  return (int) c;
}

// ACE_TTY_IO: terminal parameter control on a pseudo-terminal pair.
extern "C" int op_full_tty_io (void)
{
  long c = 569;
  ACE_HANDLE master = ::posix_openpt (O_RDWR | O_NOCTTY);
  if (master == ACE_INVALID_HANDLE)
    return -1;
  c = ck_fold (c, ::grantpt (master));
  c = ck_fold (c, ::unlockpt (master));
  const char *slave_name = ::ptsname (master);
  c = ck_fold (c, slave_name != 0 ? 1 : 0);
  ACE_TTY_IO tty;
  ACE_DEV_Addr slave_addr (ACE_TEXT_CHAR_TO_TCHAR (slave_name));
  ACE_DEV_Connector connector;
  c = ck_fold (c, connector.connect (tty, slave_addr, 0, ACE_Addr::sap_any, 0,
                                     O_RDWR | O_NOCTTY));
  ACE_TTY_IO::Serial_Params params;
  params.baudrate = 9600;
  params.databits = 8;
  params.stopbits = 1;
  params.paritymode = "none";
  params.readtimeoutmsec = 100;
  c = ck_fold (c, tty.control (ACE_TTY_IO::SETPARAMS, &params));
  ACE_TTY_IO::Serial_Params got;
  c = ck_fold (c, tty.control (ACE_TTY_IO::GETPARAMS, &got));
  c = ck_fold (c, got.baudrate);
  c = ck_fold (c, got.databits);
  c = ck_fold (c, got.stopbits);
  // write through the master, read from the slave tty
  c = ck_fold (c, (long) ACE_OS::write (master, "tty-vector\n", 11));
  char in[32];
  ssize_t n = tty.recv ((void *) in, sizeof in);
  c = ck_fold (c, (long) n);
  in[n > 0 ? n : 0] = 0;
  c = ck_str (c, in);
  c = ck_fold (c, tty.close ());
  ACE_OS::close (master);
  return (int) c;
}

// Shared-memory stream transport: the connector handshakes over TCP with
// the acceptor (shm segment negotiation), so it runs in a second thread
// against the accepting main thread.
static u_short full_ms_port = 0;
static ACE_THR_FUNC_RETURN full_ms_client (void *)
{
  ACE_MEM_Connector connector;
  ACE_MEM_Stream client;
  ACE_INET_Addr target (full_ms_port, ACE_TEXT ("localhost"));
  if (connector.connect (client, target) == -1)
    return (ACE_THR_FUNC_RETURN) (intptr_t) -1;
  unsigned char pat[96];
  fill_pattern (pat, sizeof pat);
  if (client.send (pat, sizeof pat) != (ssize_t) sizeof pat)
    return (ACE_THR_FUNC_RETURN) (intptr_t) -2;
  unsigned char back[48];
  ssize_t n = client.recv (back, sizeof back);
  long r = (long) n;
  if (n > 0)
    r = r * 100003 + ace_cksum (back, (unsigned long) n);
  client.close ();
  return (ACE_THR_FUNC_RETURN) (intptr_t) r;
}
extern "C" int op_full_mem_stream (void)
{
  long c = 571;
  u_long full_ms_mask = ACE_LOG_MSG->priority_mask (ACE_Log_Msg::PROCESS);
  ACE_LOG_MSG->priority_mask (0, ACE_Log_Msg::PROCESS);
  long result = 0;
  {
    ACE_MEM_Addr listen_addr ((u_short) 0);
    ACE_MEM_Acceptor acceptor;
    if (acceptor.open (listen_addr, 1) == -1)
      result = -1;
    else
      {
        ACE_MEM_Addr bound;
        acceptor.get_local_addr (bound);
        full_ms_port = bound.get_port_number ();
        ACE_hthread_t th;
        ACE_thread_t tid;
        ACE_OS::thr_create (full_ms_client, 0, THR_NEW_LWP | THR_JOINABLE,
                            &tid, &th);
        ACE_MEM_Stream server;
        if (acceptor.accept (server) == -1)
          result = -3;
        else
          {
            unsigned char in[96];
            ssize_t n = server.recv (in, sizeof in);
            result = result * 31 + (long) n;
            if (n > 0)
              result = result * 31 + ace_cksum (in, (unsigned long) n);
            result = result * 31 + (long) server.send (in, 48);
            server.close ();
          }
        ACE_THR_FUNC_RETURN rv = 0;
        ACE_OS::thr_join (th, &rv);
        result = result * 31 + (long) (intptr_t) rv;
        acceptor.close ();
      }
  }
  ACE_LOG_MSG->priority_mask (full_ms_mask, ACE_Log_Msg::PROCESS);
  c = ck_fold (c, result);
  return (int) c;
}

// SPIPE (emulated over UNIX-domain sockets on this platform): rendezvous
// via acceptor/connector and stream I/O.
extern "C" int op_full_spipe (void)
{
  long c = 577;
  char sdir[] = "/tmp/ace_full_sp_XXXXXX";
  if (!::mkdtemp (sdir))
    return -1;
  char spath[512];
  ACE_OS::snprintf (spath, sizeof spath, "%s/sp", sdir);
  ACE_SPIPE_Addr addr (ACE_TEXT_CHAR_TO_TCHAR (spath));
  ACE_SPIPE_Acceptor acceptor;
  c = ck_fold (c, acceptor.open (addr));
  ACE_SPIPE_Connector connector;
  ACE_SPIPE_Stream client, server;
  c = ck_fold (c, connector.connect (client, addr));
  c = ck_fold (c, acceptor.accept (server));
  c = ck_fold (c, (long) client.send_n ("spipe-payload!", 14));
  char in[16];
  c = ck_fold (c, (long) server.recv_n (in, 14));
  in[14] = 0;
  c = ck_str (c, in);
  client.close ();
  server.close ();
  acceptor.close ();
  ACE_OS::unlink (spath);
  ACE_OS::rmdir (sdir);
  return (int) c;
}

// UPIPE: the connector requires STREAMS semantics (its connect refuses
// any handle where isastream() != 1, and Linux has no STREAMS), so the
// deterministic platform surface is the acceptor rendezvous setup and the
// connector's provable refusal.
extern "C" int op_full_upipe (void)
{
  long c = 587;
  char udir[] = "/tmp/ace_full_up_XXXXXX";
  if (!::mkdtemp (udir))
    return -1;
  char upath[512];
  ACE_OS::snprintf (upath, sizeof upath, "%s/u", udir);
  ACE_UPIPE_Addr addr (ACE_TEXT_CHAR_TO_TCHAR (upath));
  ACE_UPIPE_Acceptor acceptor;
  c = ck_fold (c, acceptor.open (addr));
  ACE_UPIPE_Connector connector;
  ACE_UPIPE_Stream stream;
  ACE_Time_Value quick (0, 100000);
  c = ck_fold (c, connector.connect (stream, addr, &quick) == -1 ? 1 : 0);
  acceptor.close ();
  ACE_OS::unlink (upath);
  ACE_OS::rmdir (udir);
  return (int) c;
}

// Multicast on loopback: join a group, send to it, receive our own copy
// (IP_MULTICAST_LOOP), then a broadcast socket setup fold.
extern "C" int op_full_mcast_bcast (void)
{
  long c = 593;
  ACE_SOCK_Dgram_Mcast mcast;
  ACE_INET_Addr group ((u_short) 29001, "239.255.0.77");
  c = ck_fold (c, mcast.join (group));
  unsigned char msg[48];
  fill_pattern (msg, sizeof msg);
  c = ck_fold (c, (long) mcast.send (msg, sizeof msg));
  ACE_Handle_Set hs;
  hs.set_bit (mcast.get_handle ());
  ACE_Time_Value tv (2, 0);
  int sel = ACE_OS::select ((int) mcast.get_handle () + 1, hs.fdset (), 0, 0, &tv);
  c = ck_fold (c, sel);
  if (sel > 0)
    {
      unsigned char in[48];
      ACE_INET_Addr from;
      c = ck_fold (c, (long) mcast.recv (in, sizeof in, from));
      c = ck_fold (c, ace_cksum (in, sizeof in));
    }
  c = ck_fold (c, mcast.leave (group));
  mcast.close ();
  ACE_SOCK_Dgram_Bcast bcast;
  ACE_INET_Addr bany ((u_short) 0);
  c = ck_fold (c, bcast.open (bany));       // walks interfaces, sets SO_BROADCAST
  bcast.close ();
  return (int) c;
}

// SCTP SEQPACK: this kernel offers no SCTP (proved by the socket(2)
// probe); the acceptor's failure path is the platform's whole surface.
extern "C" int op_full_seqpack (void)
{
  long c = 599;
  ACE_SOCK_SEQPACK_Acceptor acceptor;
  ACE_INET_Addr addr ((u_short) 0, "127.0.0.1");
  int rc = acceptor.open (addr, 1);
  c = ck_fold (c, rc);                       // -1: SCTP unavailable, stable
  c = ck_fold (c, rc == -1 ? 1 : 0);
  ACE_SOCK_SEQPACK_Connector connector;
  ACE_SOCK_SEQPACK_Association assoc;
  ACE_INET_Addr target ((u_short) 9, "127.0.0.1");
  ACE_Time_Value zero (0, 0);
  c = ck_fold (c, connector.connect (assoc, target, &zero) == -1 ? 1 : 0);
  return (int) c;
}

// ICMP echo to loopback through ACE_Ping_Socket (raw sockets: we run as
// root in this environment).
extern "C" int op_full_ping (void)
{
  long c = 601;
  ACE_INET_Addr local ((u_short) 0);
  ACE_Ping_Socket ping (local);
  c = ck_fold (c, ping.get_handle () != ACE_INVALID_HANDLE ? 1 : 0);
  ACE_INET_Addr self ((u_short) 0, "127.0.0.1");
  int rc = ping.send_echo_check (self, true);
  c = ck_fold (c, rc == 0 ? 1 : 0);          // loopback echo must answer
  ping.close ();
  return (int) c;
}

// Netlink route socket: bind with pid 0 (kernel assigns), address
// bookkeeping via ACE_Netlink_Addr.
extern "C" int op_full_netlink (void)
{
  long c = 607;
  ACE_Netlink_Addr addr;
  addr.set (0, 0);
  c = ck_fold (c, addr.get_pid ());
  c = ck_fold (c, addr.get_gid ());
  c = ck_fold (c, (long) addr.get_size ());
  ACE_SOCK_Netlink nl;
  c = ck_fold (c, nl.open (addr, PF_NETLINK, NETLINK_ROUTE));
  nl.close ();
  c = ck_fold (c, 1);
  return (int) c;
}

// Local tokens: readers/writer token and mutex token acquire cycles, the
// token manager's collection bookkeeping.
extern "C" int op_full_tokens (void)
{
  long c = 613;
  ACE_Token tok;
  c = ck_fold (c, tok.acquire ());
  c = ck_fold (c, tok.renew ());
  c = ck_fold (c, tok.release ());
  c = ck_fold (c, tok.waiters ());
  // nested acquisition (ACE_Token is recursive for its owner)
  c = ck_fold (c, tok.acquire ());
  c = ck_fold (c, tok.acquire ());
  c = ck_fold (c, tok.release ());
  c = ck_fold (c, tok.release ());
  // ACE_Local_Mutex/RLock/WLock live behind ACE_HAS_TOKENS_LIBRARY, which
  // this configuration does not define (Local_Tokens compiles empty).
  return (int) c;
}

// Filecache: acquire a mapped file through the cache twice (hit path),
// fold the mapped bytes.
extern "C" int op_full_filecache (void)
{
  long c = 617;
  char path[] = "/tmp/ace_full_fcache_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  const char content[] = "filecache-content-vector-0123456789";
  ACE_OS::write (fd, content, sizeof content - 1);
  ACE_OS::close (fd);
  {
    ACE_Filecache_Handle h1 (ACE_TEXT_CHAR_TO_TCHAR (path));
    c = ck_fold (c, h1.error ());
    c = ck_fold (c, (long) h1.size ());
    const char *addr = (const char *) h1.address ();
    c = ck_fold (c, addr != 0 ? 1 : 0);
    if (addr)
      c = ck_fold (c, ace_cksum ((const unsigned char *) addr,
                                 (unsigned long) h1.size ()));
    ACE_Filecache_Handle h2 (ACE_TEXT_CHAR_TO_TCHAR (path));
    c = ck_fold (c, h2.error ());
    c = ck_fold (c, h2.address () != 0 ? 1 : 0);
  }
  ACE_Filecache::instance ()->remove (ACE_TEXT_CHAR_TO_TCHAR (path));
  ACE_OS::unlink (path);
  return (int) c;
}

// Sbrk memory pool: one small program-break acquisition (irreversible by
// design; a single minimal chunk).
extern "C" int op_full_sbrk_pool (void)
{
  long c = 619;
  ACE_Sbrk_Memory_Pool pool (0, 0);
  size_t rounded = 0;
  void *chunk = pool.acquire (64, rounded);
  c = ck_fold (c, chunk != 0 ? 1 : 0);
  c = ck_fold (c, rounded >= 64 ? 1 : 0);
  if (chunk)
    {
      ACE_OS::memset (chunk, 0x33, 64);
      c = ck_fold (c, ((unsigned char *) chunk)[63]);
    }
  return (int) c;
}

// SysV shared-memory pool through its canonical consumer (ACE_Malloc),
// plus the ACE_Shared_Memory_SV wrapper.
extern "C" int op_full_shm_pool (void)
{
  long c = 631;
  {
    ACE_Malloc<ACE_SHARED_MEMORY_POOL, ACE_Null_Mutex>
      alloc (ACE_TEXT ("full_shm_pool"));
    void *b1 = alloc.malloc (256);
    c = ck_fold (c, b1 != 0 ? 1 : 0);
    if (b1)
      {
        fill_pattern ((unsigned char *) b1, 128);
        c = ck_fold (c, ace_cksum ((unsigned char *) b1, 128));
        c = ck_fold (c, alloc.bind ("shm-block", b1));
        void *f = 0;
        c = ck_fold (c, alloc.find ("shm-block", f));
        c = ck_fold (c, f == b1 ? 1 : 0);
        alloc.free (b1);
      }
    c = ck_fold (c, alloc.remove ());
  }
  ACE_Shared_Memory_SV sv (ACE_DEFAULT_SHM_KEY + 7, 8192,
                           ACE_Shared_Memory_SV::ACE_CREATE);
  void *seg = sv.malloc (64);
  c = ck_fold (c, seg != 0 ? 1 : 0);
  c = ck_fold (c, sv.get_id () >= 0 ? 1 : 0);
  c = ck_fold (c, sv.remove ());
  return (int) c;
}

// Functor objects: hash/equal/less families and the noop command.
extern "C" int op_full_functors (void)
{
  long c = 641;
  ACE_Hash<int> hi;
  c = ck_fold (c, (long) hi (12345));
  ACE_Hash<long> hl;
  c = ck_fold (c, (long) (hl (-99L) % 100003UL));
  ACE_Hash<const char *> hs;
  c = ck_fold (c, (long) (hs ("functor-hash") % 100003UL));
  ACE_Equal_To<int> eq;
  c = ck_fold (c, eq (4, 4) ? 1 : 0);
  c = ck_fold (c, eq (4, 5) ? 1 : 0);
  ACE_Less_Than<int> lt;
  c = ck_fold (c, lt (3, 9) ? 1 : 0);
  ACE_Noop_Command noop;
  c = ck_fold (c, noop.execute (0));
  return (int) c;
}

// ACE_Dynamic: the operator-new bookkeeping singleton.
extern "C" int op_full_dynamic (void)
{
  long c = 643;
  ACE_Dynamic *dy = ACE_Dynamic::instance ();
  c = ck_fold (c, dy != 0 ? 1 : 0);
  dy->set ();
  c = ck_fold (c, dy->is_dynamic () ? 1 : 0);
  c = ck_fold (c, dy->is_dynamic () ? 1 : 0);   // reset consumed the flag?
  return (int) c;
}

// Hashable: computed-once hash caching through a subclass.
class Full_Hashable : public ACE_Hashable
{
public:
  Full_Hashable (void) : computes_ (0) {}
  virtual unsigned long hash_i (void) const
  {
    ++const_cast<Full_Hashable *> (this)->computes_;
    return 0xfeedU;
  }
  int computes_;
};
extern "C" int op_full_hashable (void)
{
  long c = 647;
  Full_Hashable h;
  c = ck_fold (c, (long) h.hash ());
  c = ck_fold (c, (long) h.hash ());       // cached: hash_i ran once
  c = ck_fold (c, h.computes_);
  return (int) c;
}

// Recyclable state machine.
class Full_Recyclable : public ACE_Recyclable
{
public:
  Full_Recyclable (void) : ACE_Recyclable (ACE_RECYCLABLE_UNKNOWN) {}
};
extern "C" int op_full_recyclable (void)
{
  long c = 653;
  Full_Recyclable r;
  c = ck_fold (c, r.recycle_state ());
  r.recycle_state (ACE_RECYCLABLE_IDLE_AND_PURGABLE);
  c = ck_fold (c, r.recycle_state ());
  r.recycle_state (ACE_RECYCLABLE_BUSY);
  c = ck_fold (c, r.recycle_state ());
  return (int) c;
}

// Stack trace: content differs between the two implementations by
// construction, so only structural invariants fold.
extern "C" int op_full_stack_trace (void)
{
  long c = 659;
  ACE_Stack_Trace st;
  const ACE_TCHAR *s = st.c_str ();
  c = ck_fold (c, s != 0 ? 1 : 0);
  return (int) c;
}

// Registered-object dump through the ODB registry (captured stderr).
class Full_Dumpee
{
public:
  void dump (void) const
  {
    ACE_DEBUG ((LM_DEBUG, "Full_Dumpee::dump state=%d\n", 1234));
  }
};
extern "C" int op_full_dump (void)
{
  long c = 661;
  Full_Dumpee obj;
  ACE_ODB::instance ()->register_object
    (new ACE_Dumpable_Adapter<Full_Dumpee> (&obj));
  u_long mask = ACE_LOG_MSG->priority_mask (ACE_Log_Msg::PROCESS);
  ACE_LOG_MSG->priority_mask (LM_DEBUG, ACE_Log_Msg::PROCESS);
  char cap[] = "/tmp/ace_full_dump_XXXXXX";
  ACE_HANDLE cfd = ACE_OS::mkstemp (cap);
  int saved_err = ACE_OS::dup (ACE_STDERR);
  ACE_OS::dup2 (cfd, ACE_STDERR);
  ACE_ODB::instance ()->dump_objects ();
  ACE_OS::dup2 (saved_err, ACE_STDERR);
  ACE_OS::close (saved_err);
  ACE_OS::lseek (cfd, 0, SEEK_SET);
  char text[256];
  ssize_t n = ACE_OS::read (cfd, text, sizeof text - 1);
  text[n > 0 ? n : 0] = 0;
  c = ck_str (c, text);
  ACE_OS::close (cfd);
  ACE_OS::unlink (cap);
  ACE_LOG_MSG->priority_mask (mask, ACE_Log_Msg::PROCESS);
  ACE_ODB::instance ()->remove_object ((void *) &obj);
  return (int) c;
}

// ACE::handle_timed_open on a FIFO (Handle_Ops component).
extern "C" int op_full_handle_ops (void)
{
  long c = 673;
  char fdir[] = "/tmp/ace_full_ho_XXXXXX";
  if (!::mkdtemp (fdir))
    return -1;
  char fpath[512];
  ACE_OS::snprintf (fpath, sizeof fpath, "%s/f", fdir);
  ACE_OS::mkfifo (ACE_TEXT_CHAR_TO_TCHAR (fpath));
  ACE_Time_Value timeout (0, 100000);
  ACE_HANDLE h = ACE::handle_timed_open (&timeout,
                                         ACE_TEXT_CHAR_TO_TCHAR (fpath),
                                         O_RDONLY | O_NONBLOCK, 0, 0);
  c = ck_fold (c, h != ACE_INVALID_HANDLE ? 1 : 0);
  if (h != ACE_INVALID_HANDLE)
    ACE_OS::close (h);
  ACE_OS::unlink (fpath);
  ACE_OS::rmdir (fdir);
  return (int) c;
}

// TSS adapter: the (info, function) trampoline invoked directly.
static long full_tss_adapter_acc = 0;
static void full_tss_adapter_fn (void *arg)
{
  full_tss_adapter_acc = full_tss_adapter_acc * 31 + (long) (intptr_t) arg;
}
extern "C" int op_full_tss_adapter (void)
{
  long c = 677;
  full_tss_adapter_acc = 7;
  ACE_TSS_Adapter adapter ((void *) 42, full_tss_adapter_fn);
  adapter.cleanup ();
  c = ck_fold (c, full_tss_adapter_acc);
  return (int) c;
}

// Thread hook: installed process-wide, wraps every spawned thread's entry.
class Full_Thread_Hook : public ACE_Thread_Hook
{
public:
  virtual ACE_THR_FUNC_RETURN start (ACE_THR_FUNC func, void *arg)
  {
    ACE_THR_FUNC_RETURN r = func (arg);
    return (ACE_THR_FUNC_RETURN) (intptr_t) ((long) (intptr_t) r + 1000);
  }
};
static ACE_THR_FUNC_RETURN full_hooked_fn (void *)
{
  return (ACE_THR_FUNC_RETURN) (intptr_t) 5;
}
extern "C" int op_full_thread_hook (void)
{
  long c = 683;
  Full_Thread_Hook hook;
  ACE_Thread_Hook *prev = ACE_Thread_Hook::thread_hook (&hook);
  ACE_Thread_Manager tm;
  tm.spawn (full_hooked_fn, 0, THR_NEW_LWP | THR_JOINABLE);
  tm.wait ();
  ACE_Thread_Hook::thread_hook (prev);
  c = ck_fold (c, 1);   // hook path exercised; return value flows via TM internals
  return (int) c;
}

// OS_QoS: flow spec field bookkeeping.
extern "C" int op_full_os_qos (void)
{
  long c = 691;
  ACE_Flow_Spec fs (100000, 20, 200000, 50, 3,
                    ACE_SERVICETYPE_NOTRAFFIC, 1500, 64, 8, 2);
  c = ck_fold (c, (long) fs.token_rate ());
  c = ck_fold (c, (long) fs.token_bucket_size ());
  c = ck_fold (c, (long) fs.peak_bandwidth ());
  c = ck_fold (c, (long) fs.latency ());
  c = ck_fold (c, (long) fs.delay_variation ());
  c = ck_fold (c, fs.ttl ());
  fs.token_rate (777);
  c = ck_fold (c, (long) fs.token_rate ());
  ACE_QoS q;
  q.sending_flowspec (&fs);
  c = ck_fold (c, q.sending_flowspec () == &fs ? 1 : 0);
  return (int) c;
}

// Connection recycling strategy: the contract surface through a concrete
// recorder subclass (the base's own code is its destructor chain).
class Full_Recycler : public ACE_Connection_Recycling_Strategy
{
public:
  Full_Recycler (void) : ops_ (1) {}
  virtual int purge (const void *) { ops_ = ops_ * 31 + 1; return 0; }
  virtual int cache (const void *) { ops_ = ops_ * 31 + 2; return 0; }
  virtual int recycle_state (const void *, ACE_Recyclable_State)
  { ops_ = ops_ * 31 + 3; return 0; }
  virtual ACE_Recyclable_State recycle_state (const void *) const
  { return ACE_RECYCLABLE_IDLE_AND_PURGABLE; }
  virtual int mark_as_closed (const void *) { ops_ = ops_ * 31 + 4; return 0; }
  virtual int mark_as_closed_i (const void *) { ops_ = ops_ * 31 + 5; return 0; }
  virtual int cleanup_hint (const void *, void **) { ops_ = ops_ * 31 + 6; return 0; }
  long ops_;
};
extern "C" int op_full_conn_recycling (void)
{
  long c = 701;
  Full_Recycler *r = new Full_Recycler;
  c = ck_fold (c, r->purge (0));
  c = ck_fold (c, r->cache (0));
  c = ck_fold (c, r->recycle_state (0, ACE_RECYCLABLE_BUSY));
  c = ck_fold (c, r->recycle_state (0));
  c = ck_fold (c, r->mark_as_closed (0));
  c = ck_fold (c, r->cleanup_hint (0, 0));
  c = ck_fold (c, r->ops_);
  ACE_Connection_Recycling_Strategy *base = r;
  delete base;                              // virtual dtor chain
  return (int) c;
}

// Log callback sink: messages routed to a callback object, no streams.
class Full_Log_Callback : public ACE_Log_Msg_Callback
{
public:
  Full_Log_Callback (void) : acc_ (1) {}
  virtual void log (ACE_Log_Record &rec)
  {
    this->acc_ = this->acc_ * 31
      + ck_str (0, ACE_TEXT_ALWAYS_CHAR (rec.msg_data ())) % 100003;
    this->acc_ = this->acc_ * 31 + (long) rec.type ();
  }
  long acc_;
};
extern "C" int op_full_log_callback (void)
{
  long c = 709;
  ACE_Log_Msg *lm = ACE_LOG_MSG;
  Full_Log_Callback cb;
  ACE_Log_Msg_Callback *prev = lm->msg_callback (&cb);
  u_long flags = lm->flags ();
  lm->set_flags (ACE_Log_Msg::MSG_CALLBACK);
  lm->clr_flags (ACE_Log_Msg::STDERR);
  ACE_DEBUG ((LM_DEBUG, "callback-vector %d\n", 99));
  ACE_ERROR ((LM_ERROR, "callback-error\n"));
  lm->msg_callback (prev);
  lm->clr_flags (ACE_Log_Msg::MSG_CALLBACK);
  lm->set_flags (flags & ACE_Log_Msg::STDERR ? ACE_Log_Msg::STDERR : 0);
  c = ck_fold (c, cb.acc_);
  return (int) c;
}

// UNIX syslog backend: this container ships no /dev/log, so open()'s
// failure result is the platform surface (stable on both sides).
extern "C" int op_full_log_syslog (void)
{
  long c = 719;
  ACE_Log_Msg_UNIX_Syslog backend;
  int rc = backend.open (ACE_TEXT ("ace_full"));
  c = ck_fold (c, rc);
  ACE_Log_Record rec (LM_INFO, 1234567890L, 1);
  rec.msg_data ("syslog-vector");
  if (rc == 0)
    {
      c = ck_fold (c, backend.log (rec));
      c = ck_fold (c, backend.close ());
    }
  else
    c = ck_fold (c, backend.reset ());
  return (int) c;
}

// Logging strategy service: parse args, apply to Log_Msg, verify the
// output file receives a fixed line.
extern "C" int op_full_logging_strategy (void)
{
  long c = 727;
  char lpath[] = "/tmp/ace_full_lstrat_XXXXXX";
  ACE_HANDLE lfd = ACE_OS::mkstemp (lpath);
  ACE_OS::close (lfd);
  ACE_Logging_Strategy strat;
  ACE_TCHAR arg_f[512];
  ACE_OS::snprintf (arg_f, 512, ACE_TEXT ("%s"), lpath);
  ACE_TCHAR *argv[] = { ACE_TEXT ("-f"), ACE_TEXT ("OSTREAM"),
                        ACE_TEXT ("-s"), arg_f, 0 };
  c = ck_fold (c, strat.init (4, argv));
  ACE_DEBUG ((LM_DEBUG, "strategy-file-vector %d\n", 606));
  ACE_LOG_MSG->clr_flags (ACE_Log_Msg::OSTREAM);
  ACE_LOG_MSG->set_flags (ACE_Log_Msg::STDERR);
  ACE_LOG_MSG->msg_ostream (0, 0);
  ACE_HANDLE rfd = ACE_OS::open (lpath, O_RDONLY);
  char text[256];
  ssize_t n = ACE_OS::read (rfd, text, sizeof text - 1);
  text[n > 0 ? n : 0] = 0;
  c = ck_str (c, text);
  ACE_OS::close (rfd);
  ACE_OS::unlink (lpath);
  c = ck_fold (c, strat.fini ());
  return (int) c;
}

// Log category: hierarchical category-scoped logging bookkeeping.
extern "C" int op_full_log_category (void)
{
  long c = 733;
  ACELIB_DEBUG ((LM_DEBUG, ""));   // ensure the lib category singleton exists
  ACE_Log_Category &cat = ACE_Log_Category::ace_lib ();
  c = ck_fold (c, cat.priority_mask ());
  return (int) c;
}

// Monitor framework: a size monitor registered, updated, read back, and
// a control action executed by name.
class Full_Mon_Action : public ACE::Monitor_Control::Control_Action
{
public:
  Full_Mon_Action (void) : fired_ (0) {}
  virtual void execute (const char *command)
  {
    this->fired_ = this->fired_ * 31 + ck_str (0, command) % 100003;
  }
  long fired_;
};
extern "C" int op_full_monitor (void)
{
  long c = 739;
  ACE::Monitor_Control::Size_Monitor *mon =
    new ACE::Monitor_Control::Size_Monitor ("full/size/monitor");
  mon->receive ((size_t) 4096);
  ACE::Monitor_Control::Monitor_Control_Types::Data
    data (mon->type ());
  mon->retrieve (data);
  c = ck_fold (c, (long) data.value_);
  c = ck_fold (c, (long) data.index_);
  mon->receive ((size_t) 8192);
  mon->retrieve (data);
  c = ck_fold (c, (long) data.value_);
  ACE::Monitor_Control::Monitor_Point_Registry *reg =
    ACE::Monitor_Control::Monitor_Point_Registry::instance ();
  c = ck_fold (c, reg->add (mon));
  ACE::Monitor_Control::Monitor_Base *found =
    reg->get (ACE_CString ("full/size/monitor"));
  c = ck_fold (c, found == mon ? 1 : 0);
  ACE::Monitor_Control::Monitor_Control_Types::NameList nl = reg->names ();
  c = ck_fold (c, (long) nl.size ());
  Full_Mon_Action act;
  act.execute ("monitor-command");
  c = ck_fold (c, act.fired_);
  c = ck_fold (c, reg->remove ("full/size/monitor"));
  mon->remove_ref ();
  return (int) c;
}

// ===========================================================================
// batch 2F: the last unexercised components — assert (in a forked child:
// its %a directive exits the process), dynamic message strategies, dynamic
// service lookup, LSOCK_CODgram, monitor admin, name-service client error
// paths, the callback proactor, Service_Manager, and small leftovers.
// ===========================================================================
#include "ace/Assert.h"
#include "ace/Dynamic_Message_Strategy.h"
#include "ace/Dynamic_Service.h"
#include "ace/Dynamic_Service_Dependency.h"
#include "ace/LSOCK_CODgram.h"
#include "ace/Monitor_Admin.h"
#include "ace/Monitor_Admin_Manager.h"
#include "ace/POSIX_CB_Proactor.h"
#include "ace/Name_Proxy.h"
#include "ace/Remote_Name_Space.h"
#include "ace/Service_Manager.h"
#include "ace/Copy_Disabled.h"

// __ace_assert logs through Log_Msg and then exits via the %a directive,
// so it runs in a forked child; the parent folds the exit status and the
// captured message (file/line arguments are fixed strings here).
extern "C" int op_full_assert (void)
{
  long c = 743;
  char cap[] = "/tmp/ace_full_assert_XXXXXX";
  ACE_HANDLE cfd = ACE_OS::mkstemp (cap);
  pid_t child = ACE_OS::fork ();
  if (child == 0)
    {
      ACE_OS::dup2 (cfd, ACE_STDERR);
      ACE_LOG_MSG->open (ACE_TEXT ("full_assert"), ACE_Log_Msg::STDERR, 0);
      __ace_assert ("fixed_file.cpp", 4242, ACE_TEXT ("always_false_vector"));
      ACE_OS::exit (99);   // unreachable: %a exited already
    }
  ACE_exitcode status = 0;
  ACE_OS::waitpid (child, &status, 0);
  c = ck_fold (c, WIFEXITED (status) ? 1 : 0);
  c = ck_fold (c, WIFEXITED (status) ? WEXITSTATUS (status) : -1);
  ACE_OS::lseek (cfd, 0, SEEK_SET);
  char text[256];
  ssize_t n = ACE_OS::read (cfd, text, sizeof text - 1);
  text[n > 0 ? n : 0] = 0;
  c = ck_str (c, text);
  ACE_OS::close (cfd);
  ACE_OS::unlink (cap);
  return (int) c;
}

// Deadline and laxity message strategies: priority_status against a FIXED
// evaluation time (the tv parameter pins all clock reads).
extern "C" int op_full_dynamic_message (void)
{
  long c = 751;
  ACE_Deadline_Message_Strategy deadline (0x400000, 1, 0x200000);
  ACE_Laxity_Message_Strategy laxity (0x400000, 1, 0x200000);
  ACE_Message_Block mb (64);
  ACE_Time_Value now (5000, 0);
  // pending: deadline comfortably in the future
  mb.msg_execution_time (ACE_Time_Value (0, 1000));
  mb.msg_deadline_time (ACE_Time_Value (5001, 0));
  c = ck_fold (c, deadline.priority_status (mb, now));
  c = ck_fold (c, laxity.priority_status (mb, now));
  // late: deadline already passed
  mb.msg_deadline_time (ACE_Time_Value (4999, 0));
  c = ck_fold (c, deadline.priority_status (mb, now));
  c = ck_fold (c, laxity.priority_status (mb, now));
  c = ck_fold (c, (long) mb.msg_priority ());
  ACE_Dynamic_Message_Strategy &base = deadline;
  base.dump ();   // no output when tracing is off; exercises the vtable
  return (int) c;
}

// Dynamic service lookup against a configured gestalt, plus the dependency
// bookkeeping type.
extern "C" int op_full_dynamic_service (void)
{
  long c = 757;
  u_long mask = ACE_LOG_MSG->priority_mask (ACE_Log_Msg::PROCESS);
  ACE_LOG_MSG->priority_mask (0, ACE_Log_Msg::PROCESS);
  ACE_Service_Gestalt gestalt (ACE_Service_Gestalt::MAX_SERVICES, true);
  ACE_Static_Svc_Descriptor desc = {
    ACE_TEXT ("Full_Service"),
    ACE_SVC_OBJ_T,
    &ACE_SVC_NAME (Full_Service),
    ACE_Service_Type::DELETE_THIS | ACE_Service_Type::DELETE_OBJ,
    ACE_ACTIVE
  };
  c = ck_fold (c, gestalt.process_directive (desc, 0));
  c = ck_fold (c, gestalt.process_directive (
    ACE_TEXT ("static Full_Service \"one two\"")));
  Full_Service *svc =
    ACE_Dynamic_Service<Full_Service>::instance (&gestalt,
                                                 ACE_TEXT ("Full_Service"));
  c = ck_fold (c, svc != 0 ? 1 : 0);
  c = ck_fold (c, svc ? svc->state_ : -1);
  Full_Service *missing =
    ACE_Dynamic_Service<Full_Service>::instance (&gestalt,
                                                 ACE_TEXT ("Absent_Service"));
  c = ck_fold (c, missing == 0 ? 1 : 0);
  ACE_Dynamic_Service_Dependency dep (&gestalt, ACE_TEXT ("Full_Service"));
  c = ck_fold (c, 1);
  ACE_LOG_MSG->priority_mask (mask, ACE_Log_Msg::PROCESS);
  return (int) c;
}

// Connected UNIX-domain datagrams (LSOCK_CODgram) with fd rights intact.
extern "C" int op_full_lsock_codgram (void)
{
  long c = 761;
  char ddir[] = "/tmp/ace_full_lcd_XXXXXX";
  if (!::mkdtemp (ddir))
    return -1;
  char p1[512], p2[512];
  ACE_OS::snprintf (p1, sizeof p1, "%s/a", ddir);
  ACE_OS::snprintf (p2, sizeof p2, "%s/b", ddir);
  // bind the peer's path first (plain dgram), then the connected socket
  // has an existing rendezvous to connect against
  ACE_LSOCK_Dgram peer;
  c = ck_fold (c, peer.open (ACE_UNIX_Addr (p2)));
  ACE_LSOCK_CODgram d1;
  c = ck_fold (c, d1.open (ACE_UNIX_Addr (p2), ACE_UNIX_Addr (p1)));   // remote, local
  unsigned char msg[40];
  fill_pattern (msg, sizeof msg);
  c = ck_fold (c, (long) d1.send (msg, sizeof msg));   // connected send
  unsigned char in[40];
  ACE_UNIX_Addr from;
  c = ck_fold (c, (long) peer.recv (in, sizeof in, from));
  c = ck_fold (c, ace_cksum (in, sizeof in));
  d1.close ();
  peer.close ();
  ACE_OS::unlink (p1);
  ACE_OS::unlink (p2);
  ACE_OS::rmdir (ddir);
  return (int) c;
}

// Monitor admin layer: the manager's admin() registry facade.
extern "C" int op_full_monitor_admin (void)
{
  long c = 769;
  ACE::Monitor_Control::Monitor_Admin_Manager mgr;
  ACE::Monitor_Control::Monitor_Admin &admin = mgr.admin ();
  ACE::Monitor_Control::Size_Monitor *mon =
    new ACE::Monitor_Control::Size_Monitor ("full/admin/size");
  c = ck_fold (c, admin.monitor_point (mon, ACE_Time_Value::zero) ? 1 : 0);
  ACE::Monitor_Control::Monitor_Base *got =
    admin.monitor_point ("full/admin/size");
  c = ck_fold (c, got == mon ? 1 : 0);
  mon->receive ((size_t) 2048);
  ACE::Monitor_Control::Monitor_Control_Types::Data data (mon->type ());
  mon->retrieve (data);
  c = ck_fold (c, (long) data.value_);
  c = ck_fold (c,
    ACE::Monitor_Control::Monitor_Point_Registry::instance ()
      ->remove ("full/admin/size") ? 1 : 0);
  mon->remove_ref ();
  return (int) c;
}

// Naming-service client error paths: no server is listening on the
// reserved discard port, so connection refusal is the deterministic
// platform surface for Name_Proxy and Remote_Name_Space.
extern "C" int op_full_name_client (void)
{
  long c = 773;
  u_long mask = ACE_LOG_MSG->priority_mask (ACE_Log_Msg::PROCESS);
  ACE_LOG_MSG->priority_mask (0, ACE_Log_Msg::PROCESS);
  ACE_Name_Proxy proxy;
  ACE_INET_Addr closed ((u_short) 1, "127.0.0.1");   // tcpmux: nothing listens
  ACE_Synch_Options opts (ACE_Synch_Options::USE_TIMEOUT,
                          ACE_Time_Value (0, 200000));
  c = ck_fold (c, proxy.open (closed, opts) == -1 ? 1 : 0);
  ACE_Remote_Name_Space rns;
  c = ck_fold (c, rns.open (ACE_TEXT ("127.0.0.1"), (u_short) 1) == -1 ? 1 : 0);
  ACE_LOG_MSG->priority_mask (mask, ACE_Log_Msg::PROCESS);
  return (int) c;
}

// The callback (SIGEV_THREAD) POSIX proactor variant, driving the same
// asynchronous file read as the AIOCB op.
extern "C" int op_full_cb_proactor (void)
{
  long c = 787;
  char path[] = "/tmp/ace_full_cbaio_XXXXXX";
  ACE_HANDLE fd = ACE_OS::mkstemp (path);
  unsigned char pat[128];
  fill_pattern (pat, sizeof pat);
  ACE_OS::write (fd, pat, sizeof pat);
  ACE_OS::close (fd);
  u_long mask = ACE_LOG_MSG->priority_mask (ACE_Log_Msg::PROCESS);
  ACE_LOG_MSG->priority_mask (0, ACE_Log_Msg::PROCESS);
  {
    ACE_POSIX_CB_Proactor *impl = new ACE_POSIX_CB_Proactor;
    ACE_Proactor proactor (impl, 1);
    ACE_HANDLE afd = ACE_OS::open (path, O_RDONLY);
    Full_Read_Handler handler;
    handler.proactor (&proactor);
    ACE_Asynch_Read_File arf;
    c = ck_fold (c, arf.open (handler, afd, 0, &proactor));
    ACE_Message_Block *mb = new ACE_Message_Block (256);
    c = ck_fold (c, arf.read (*mb, 128, 0, 0));
    for (int i = 0; i < 200 && !handler.done_; ++i)
      {
        ACE_Time_Value slice (0, 20000);
        proactor.handle_events (slice);
      }
    c = ck_fold (c, handler.done_);
    c = ck_fold (c, handler.sum_ % 1000003);
    mb->release ();
    ACE_OS::close (afd);
  }
  ACE_LOG_MSG->priority_mask (mask, ACE_Log_Msg::PROCESS);
  ACE_OS::unlink (path);
  return (int) c;
}

// ACE_Service_Manager: init on an ephemeral port, service introspection,
// suspend/resume, fini.
extern "C" int op_full_service_manager (void)
{
  long c = 797;
  u_long mask = ACE_LOG_MSG->priority_mask (ACE_Log_Msg::PROCESS);
  ACE_LOG_MSG->priority_mask (0, ACE_Log_Msg::PROCESS);
  class Full_SM : public ACE_Service_Manager
  {
  public:
    using ACE_Service_Manager::init;
    using ACE_Service_Manager::info;
    using ACE_Service_Manager::suspend;
    using ACE_Service_Manager::resume;
    using ACE_Service_Manager::fini;
  } sm;
  ACE_TCHAR *argv[] = { ACE_TEXT ("-p"), ACE_TEXT ("0"), 0 };
  c = ck_fold (c, sm.init (2, argv));
  ACE_TCHAR *info = 0;
  c = ck_fold (c, sm.info (&info, 0) >= 0 ? 1 : 0);
  // the info string embeds the ephemeral listen port — presence only
  c = ck_fold (c, info != 0 ? 1 : 0);
  delete [] info;
  c = ck_fold (c, sm.suspend ());
  c = ck_fold (c, sm.resume ());
  c = ck_fold (c, sm.fini ());
  ACE_LOG_MSG->priority_mask (mask, ACE_Log_Msg::PROCESS);
  return (int) c;
}

// ACE_Copy_Disabled: constructible by derivation (its whole surface).
class Full_NoCopy : private ACE_Copy_Disabled
{
public:
  Full_NoCopy (void) : v_ (11) {}
  int v_;
};
extern "C" int op_full_copy_disabled (void)
{
  long c = 809;
  Full_NoCopy nc;
  c = ck_fold (c, nc.v_);
  return (int) c;
}

// RW_Thread_Mutex's own out-of-line surface: the write-upgrade path.
extern "C" int op_full_rw_upgrade (void)
{
  long c = 811;
  ACE_RW_Thread_Mutex rw;
  c = ck_fold (c, rw.acquire_read ());
  c = ck_fold (c, rw.tryacquire_write_upgrade ());   // sole reader: upgrades
  c = ck_fold (c, rw.release ());
  ACE_RW_Thread_Mutex rw2;
  c = ck_fold (c, rw2.acquire_write ());
  c = ck_fold (c, rw2.release ());
  return (int) c;
}

// ===========================================================================
// batch 2G: the last leaf surfaces — Basic_Types conversion helpers, the
// ACE_Dirent class wrapper, IO_Cntl_Msg, the SysV regex wrappers, the
// devctl/TLI portability stubs (deterministic ENOTSUP on Linux), and the
// ace_wchar conversion classes.
// ===========================================================================
#include "ace/Dirent.h"
#include "ace/IO_Cntl_Msg.h"
#include "ace/OS_NS_devctl.h"
#include "ace/OS_TLI.h"
#include "ace/os_include/os_dirent.h"

extern "C" int op_full_leaf_sweep (void)
{
  long c = 821;
  // Basic_Types: checked 64->32 narrowing helpers
  ACE_UINT64 wide = ACE_UINT64_LITERAL (0x00000000fedcba98);
  c = ck_fold (c, (long) ACE_U64_TO_U32 (wide));
  c = ck_fold (c, (long) ACE_CU64_TO_CU32 (ACE_UINT64_LITERAL (77)));
  // ACE_Dirent: class-wrapper enumeration of a created directory
  char ddir[] = "/tmp/ace_full_dirent2_XXXXXX";
  if (!::mkdtemp (ddir))
    return -1;
  char fp[512];
  ACE_OS::snprintf (fp, sizeof fp, "%s/solo.txt", ddir);
  ACE_HANDLE fd = ACE_OS::open (fp, O_CREAT | O_WRONLY, 0644);
  ACE_OS::close (fd);
  {
    ACE_Dirent dir;
    c = ck_fold (c, dir.open (ACE_TEXT_CHAR_TO_TCHAR (ddir)));
    int names = 0;
    for (ACE_DIRENT *e; (e = dir.read ()) != 0;)
      if (e->d_name[0] != '.')
        {
          ++names;
          c = ck_str (c, e->d_name);
        }
    c = ck_fold (c, names);
    dir.close ();
  }
  ACE_OS::unlink (fp);
  ACE_OS::rmdir (ddir);
  // IO_Cntl_Msg: command/count/error/rval bookkeeping
  ACE_IO_Cntl_Msg icm (ACE_IO_Cntl_Msg::SET_LWM);
  c = ck_fold (c, icm.cmd ());
  icm.cmd (ACE_IO_Cntl_Msg::GET_HWM);
  c = ck_fold (c, icm.cmd ());
  icm.count (3);
  c = ck_fold (c, (long) icm.count ());
  icm.error (EAGAIN);
  c = ck_fold (c, icm.error ());
  icm.rval (-2);
  c = ck_fold (c, icm.rval ());
  // OS_NS_regex: the SysV compile/step wrappers
  char expbuf[256];
  ACE_OS::memset (expbuf, 0, sizeof expbuf);
  char *compiled = ACE_OS::compile ("needle", expbuf, expbuf + sizeof expbuf);
  c = ck_fold (c, compiled != 0 ? 1 : 0);
  if (compiled)
    {
      c = ck_fold (c, ACE_OS::step ("finding a needle here", expbuf));
      c = ck_fold (c, ACE_OS::step ("nothing to find", expbuf));
    }
  // OS_NS_devctl: not supported on Linux — deterministic ENOTSUP
  ACE_OS::last_error (0);
  int dcrc = ACE_OS::posix_devctl (0, 0, 0, 0, 0);
  c = ck_fold (c, dcrc);
  c = ck_fold (c, ACE_OS::last_error () == ENOTSUP ? 1 : 0);
  // OS_TLI: the whole family is ENOTSUP stubs on Linux
  c = ck_fold (c, ACE_OS::t_open ((char *) "/dev/tcp", O_RDWR, 0));
  c = ck_fold (c, ACE_OS::last_error () == ENOTSUP ? 1 : 0);
  c = ck_fold (c, ACE_OS::t_close (0));
  c = ck_fold (c, ACE_OS::t_bind (0, 0, 0));
  c = ck_fold (c, ACE_OS::t_accept (0, 0, 0));
  // ace_wchar conversion classes (wchar_t based)
  ACE_Wide_To_Ascii w2a (L"wide-to-ascii-vector");
  c = ck_str (c, w2a.char_rep ());
  ACE_Ascii_To_Wide a2w ("ascii-to-wide-vector");
  const wchar_t *wr = a2w.wchar_rep ();
  for (const wchar_t *p = wr; *p; ++p)
    c = ck_fold (c, (long) *p);
  char *once = ACE_Wide_To_Ascii::convert (L"static-convert");
  c = ck_str (c, once);
  delete [] once;
  return (int) c;
}
