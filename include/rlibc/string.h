#pragma once

#include <stdint.h>

char* strcpy(char* restrict dst, const char* restrict src);
char* strncpy(char* restrict dst, const char* restrict src, size_t n);
char* strcat(char* restrict dst, const char* restrict src);
char* strncat(char* restrict dst, const char* restrict src, size_t n);
size_t strxfrm(char* restrict dst, const char* restrict src, size_t n);

size_t strlen(const char*);
size_t strnlen(const char*, size_t);
int strcmp(const char* m1, const char* m2);
int strncmp(const char* m1, const char* m2, size_t n);
int strcoll(const char* lhs, const char* rhs);
char* strchr(const char* s, int c);
char* strrchr(const char* str, int c);
size_t strspn(const char* dst, const char* src);
size_t strcspn(const char* dst, const char* src);
char* strpbrk(const char* dst, const char* breakset);
char* strstr(const char* str, const char* substr);
char* strtok(char* restrict str, const char* restrict delim);

void* memchr(const void*, int, size_t);
int memcmp(const void*, const void*, size_t);
void* memset(void*, int, size_t);
void* memcpy(void* restrict, const void* restrict, size_t);
void* memmove(void*, const void*, size_t);

char* strerror(int);
