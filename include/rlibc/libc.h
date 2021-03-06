/// Avoid needless header complexity.
/// #include "libc.h"

#pragma once

#include <ctype.h>
#include <errno.h>
#include <fcntl.h>
#include <inttypes.h>
#include <math.h>
#include <signal.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/types.h>
#include <unistd.h>
#include <utime.h>

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

/* System calls */
int sys_exit(int);
int sys_write(int, const char*, unsigned long);
int sys_read(int, char*, unsigned long);
int sys_fork(void);
