/// Avoid needless header complexity.
/// #include "libc.h"

#pragma once

#include <ctype.h>
#include <errno.h>
#include <inttypes.h>
#include <math.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/types.h>

typedef unsigned long clock_t;
typedef long time_t;
typedef uint32_t wchar_t;
typedef uint32_t wint_t;

#define NULL (0)

/* Types */

int iswalnum(wint_t);
int iswalpha(wint_t);
int iswblank(wint_t);
int iswcntrl(wint_t);
int iswdigit(wint_t);
int iswgraph(wint_t);
int iswlower(wint_t);
int iswprint(wint_t);
int iswpunct(wint_t);
int iswspace(wint_t);
int iswupper(wint_t);
int iswxdigit(wint_t);
wint_t towlower(wint_t);
wint_t towupper(wint_t);

/* Filesystem */
typedef struct FILE FILE;

extern FILE __stdin;
extern FILE __stdout;
extern FILE __stderr;

#define stdin (&__stdin)
#define stdout (&__stdout)
#define stderr (&__stderr)

/* I/O */
int puts(const char*);
int fputs(const char*, FILE*);
int fputc(int, FILE*);
#define putc(c, f) (fputc((c), (f)))

int printf(const char* fmt, ...);
int fprintf(FILE*, const char* fmt, ...);
int sprintf(char*, const char* fmt, ...);
int snprintf(char*, size_t, const char* fmt, ...);

int vprintf(const char* fmt, va_list);
int vfprintf(FILE*, const char* fmt, va_list);
int vsprintf(char*, const char* fmt, va_list);
int vsnprintf(char*, size_t, const char* fmt, va_list);

#define _IOFBF (0)
#define _IOLBF (1)
#define _IONBF (2)

#define BUFSIZ (8192)

#define EOF (-1)

#define FOPEN_MAX (16)
#define FILENAME_MAX (4096)
#define L_tmpnam (20)

#define SEEK_SET (0)
#define SEEK_CUR (1)
#define SEEK_END (2)

#define TMP_MAX (238328)

int close(int);
ssize_t read(int, void*, size_t);
ssize_t write(int, const void*, size_t);
ssize_t pread(int, void*, size_t, off_t);
ssize_t pwrite(int, const void*, size_t, off_t);
off_t lseek(int, off_t, int);
int remove(const char*);
int rename(const char*, const char*);
int rmdir(const char*);
int unlink(const char*);
int utime(const char*, void*);

/* Memory Management */
int brk(void*);
void* sbrk(intptr_t);

/* Environment */
pid_t getpid(void);
uid_t getuid(void);
uid_t geteuid(void);
pid_t setsid(void);
int setgid(gid_t);
int setuid(uid_t);

/* Time */
struct tm
{
    int tm_sec; /* seconds after the minute [0-60] */
    int tm_min; /* minutes after the hour [0-59] */
    int tm_hour; /* hours since midnight [0-23] */
    int tm_mday; /* day of the month [1-31] */
    int tm_mon; /* months since January [0-11] */
    int tm_year; /* years since 1900 */
    int tm_wday; /* days since Sunday [0-6] */
    int tm_yday; /* days since January 1 [0-365] */
    int tm_isdst; /* Daylight Savings Time flag */
    long tm_gmtoff; /* offset from CUT in seconds */
    char* tm_zone; /* timezone abbreviation */
};
time_t time(time_t*);
struct tm* gmtime(const time_t*);
struct tm* gmtime_r(const time_t*, struct tm*);
struct tm* localtime(const time_t*);
struct tm* localtime_r(const time_t*, struct tm*);
time_t timegm(struct tm*);
time_t timelocal(struct tm*);
time_t mktime(struct tm*);

/* Dynamic loading */
void* dlopen(const char* filename, int flag);
char* dlerror(void);
void* dlsym(void* handle, const char* symbol);
int dlclose(void* handle);
int dladdr(void* addr, void* info);

/* Signals */
typedef void (*sighandler_t)(int);
int raise(int);
sighandler_t signal(int, sighandler_t);

/* System calls */
int sys_exit(int);
int sys_write(int, const char*, unsigned long);
int sys_read(int, char*, unsigned long);
int sys_fork(void);
