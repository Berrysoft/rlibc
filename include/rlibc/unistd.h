#pragma once

#include <sys/types.h>

ssize_t read(int, void*, size_t);
ssize_t write(int, const void*, size_t);
ssize_t pread(int, void*, size_t, off_t);
ssize_t pwrite(int, const void*, size_t, off_t);
off_t lseek(int, off_t, int);

int rmdir(const char*);
int unlink(const char*);

int brk(void*);
void* sbrk(intptr_t);

pid_t getpid(void);
uid_t getuid(void);
uid_t geteuid(void);
pid_t setsid(void);
int setgid(gid_t);
int setuid(uid_t);
