#pragma once

typedef void (*sighandler_t)(int);
int raise(int);
sighandler_t signal(int, sighandler_t);
