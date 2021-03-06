#pragma once

#include <sys/types.h>

ssize_t read(int, void*, size_t);
ssize_t write(int, const void*, size_t);
ssize_t pread(int, void*, size_t, off_t);
ssize_t pwrite(int, const void*, size_t, off_t);
off_t lseek(int, off_t, int);

int rmdir(const char*);
int unlink(const char*);
