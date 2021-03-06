#pragma once

#include <stdint.h>

// TODO: ato*
// TODO: strto*

void* malloc(size_t);
void* calloc(size_t num, size_t size);
void* realloc(void* ptr, size_t new_size);
void free(void*);
