#pragma once

#include <stdint.h>

typedef int errno_t;

errno_t* __p_errno(void);
#define errno (*__p_errno())
