rlibc
=====

An implementation of libc and POSIX in Rust.

# Compiling

To build, you'll need make, clang, lld and cargo.
* Optionally, create a `config.mk` file to specify custom tools and targets.
* Run `make`

# Coverage
rlibc currently supports part of C99 and POSIX, as well as some OS-specific functions.

* mem - mostly done
* strings - mostly done
* math - done
* printing - partial
* time - partial
* fs - partial
* mm - partial
* environment - partial
* dl - none and WON'T
* signals - almost none
* pthreads - NONE
* thread-local - partial
* net - NONE
* atomics - NONE

# Targets

rlibc currently supports Linux on x86-64.
