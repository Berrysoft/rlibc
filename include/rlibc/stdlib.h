#pragma once

#include <stdint.h>

// TODO: ato*
// TODO: strto*

void* malloc(size_t);
void* calloc(size_t num, size_t size);
void* realloc(void* ptr, size_t new_size);
void free(void*);

void abort(void);
void exit(int);
void _exit(int);
void _Exit(int);
int atexit(void (*)(void));

#define EXIT_SUCCESS (0)
#define EXIT_FAILURE (1)

char* getenv(const char*);
int setenv(const char* key, const char* value, int overwrite);
int unsetenv(const char* key);

int mkstemp(char* template);

// TODO: rand
// TODO: qsort
// TODO: bsearch
