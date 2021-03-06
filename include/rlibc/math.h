#pragma once

#include <stdint.h>

double acos(double x);
float acosf(float x);
double acosh(double x);
float acoshf(float x);

double asin(double x);
float asinf(float x);
double asinh(double x);
float asinhf(float x);

double atan(double x);
double atan2(double x, double y);
float atan2f(float x, float y);
float atanf(float x);
double atanh(double x);
float atanhf(float x);

double cbrt(double x);
float cbrtf(float x);

double ceil(double x);
float ceilf(float x);

double copysign(double x, double y);
float copysignf(float x, float y);

double cos(double x);
float cosf(float x);
double cosh(double x);
float coshf(float x);

double erf(double x);
double erfc(double x);
float erfcf(float x);
float erff(float x);

double exp(double x);
double exp2(double x);
float exp2f(float x);
double exp10(double x);
float exp10f(float x);
float expf(float x);
double expm1(double x);
float expm1f(float x);

double fabs(double x);
float fabsf(float x);

double fdim(double x, double y);
float fdimf(float x, float y);

double floor(double x);
float floorf(float x);

double fma(double x, double y, double z);
float fmaf(float x, float y, float z);

double fmax(double x, double y);
float fmaxf(float x, float y);

double fmin(double x, double y);
float fminf(float x, float y);

double fmod(double x, double y);
float fmodf(float x, float y);

double frexp(double x, int* p);
float frexpf(float x, int* p);

double hypot(double x, double y);
float hypotf(float x, float y);

int ilogb(double x);
int ilogbf(float x);

double j0(double x);
float j0f(float x);
double j1(double x);
float j1f(float x);
double jn(int n, double x);
float jnf(int n, float x);

double ldexp(double x, int exp);
float ldexpf(float x, int exp);

double lgamma(double x);
float lgammaf(float x);
double lgamma_r(double x, int* p);
float lgammaf_r(float x, int* p);

double log(double x);
double log1p(double x);
float log1pf(float x);
double log2(double x);
float log2f(float x);
double log10(double x);
float log10f(float x);
float logf(float x);

double modf(double x, double* p);
float modff(float x, float* p);

double nextafter(double x, double y);
float nextafterf(float x, float y);

double pow(double mant, double expo);
float powf(float mant, float expo);

double remainder(double x, double y);
float remainderf(float x, float y);

double remquo(double x, double y, int* p);
float remquof(float x, float y, int* p);

double round(double x);
float roundf(float x);

double scalbn(double x, int n);
float scalbnf(float x, int n);

double sin(double x);
float sinf(float x);
double sinh(double x);
float sinhf(float x);

void sincos(double x, double* ps, double* pc);
void sincosf(float x, float* ps, float* pc);

double sqrt(double x);
float sqrtf(float x);

double tan(double x);
float tanf(float x);
double tanh(double x);
float tanhf(float x);

double trunc(double x);
float truncf(float x);

double y0(double x);
float y0f(float x);
double y1(double x);
float y1f(float x);
double yn(int n, double x);
float ynf(int n, float x);
