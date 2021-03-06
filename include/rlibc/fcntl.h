#pragma once

#include <stdint.h>

typedef uint32_t mode_t;

int open(const char*, int, mode_t);
int close(int);
