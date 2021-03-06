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
#include <time.h>
#include <unistd.h>
#include <utime.h>
#include <wchar.h>
#include <wctype.h>

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
