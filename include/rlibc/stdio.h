#pragma once

#include <stdarg.h>
#include <stdint.h>
#include <sys/types.h>

typedef struct FILE FILE;

extern FILE __stdin;
extern FILE __stdout;
extern FILE __stderr;

#define stdin (&__stdin)
#define stdout (&__stdout)
#define stderr (&__stderr)

// TODO: f*

int puts(const char*);
int fputs(const char* restrict, FILE* restrict);
int fputc(int, FILE*);
#define putc(c, f) (fputc((c), (f)))
int putchar(int);

char* gets(char*);
char* fgets(char* restrict, int, FILE* restrict);
int fgetc(FILE*);
#define getc(f) (fgetc(f))
int getchar(void);

int fileno(FILE*);
void setbuf(FILE* restrict stream, char* restrict buffer);
int setvbuf(FILE* restrict stream, char* restrict buffer, int mode, size_t size);

int fflush(FILE*);

int printf(const char* fmt, ...);
int fprintf(FILE*, const char* fmt, ...);
int sprintf(char*, const char* fmt, ...);
int snprintf(char*, size_t, const char* fmt, ...);

int vprintf(const char* fmt, va_list);
int vfprintf(FILE*, const char* fmt, va_list);
int vsprintf(char*, const char* fmt, va_list);
int vsnprintf(char*, size_t, const char* fmt, va_list);

// TODO: scanf

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

int remove(const char*);
int rename(const char*, const char*);
