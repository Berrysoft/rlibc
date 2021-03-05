#include "libc.h"

#define ASSERT(pred, name) \
    if (!(pred)) { puts("Assertion error in: " name); };

/* Type tests */

void test_isalnum()
{
    for (char c = 'A'; c <= 'Z'; ++c)
    {
        ASSERT(isalnum(c), "isalnum");
    }
    for (char c = 'a'; c <= 'z'; ++c)
    {
        ASSERT(isalnum(c), "isalnum");
    }
    for (char c = '0'; c <= '9'; ++c)
    {
        ASSERT(isalnum(c), "isalnum");
    }
    ASSERT(!isalnum('$'), "isalnum");
}

void test_isalpha()
{
    for (char c = 'A'; c <= 'Z'; ++c)
    {
        ASSERT(isalpha(c), "isalpha");
    }
    for (char c = 'a'; c <= 'z'; ++c)
    {
        ASSERT(isalpha(c), "isalpha");
    }
    for (char c = '0'; c <= '9'; ++c)
    {
        ASSERT(!isalpha(c), "isalpha");
    }
    ASSERT(!isalpha('$'), "isalpha");
}

void test_isblank()
{
    ASSERT(isblank(' '), "isblank");
    ASSERT(isblank('\t'), "isblank");
    ASSERT(!isblank('1'), "isblank");
    ASSERT(!isblank('$'), "isblank");
}

void test_iscntrl()
{
    for (char c = 0; c <= 0x19; ++c)
    {
        ASSERT(iscntrl(c), "iscntrl");
    }
    for (char c = '0'; c <= 'z'; ++c)
    {
        ASSERT(!iscntrl(c), "iscntrl");
    }
}

void test_isdigit()
{
    for (char c = '0'; c <= '9'; ++c)
    {
        ASSERT(isdigit(c), "isdigit");
    }
    for (char c = 'A'; c <= 'Z'; ++c)
    {
        ASSERT(!isdigit(c), "isdigit");
    }
    for (char c = 'a'; c <= 'z'; ++c)
    {
        ASSERT(!isdigit(c), "isdigit");
    }
}

void test_isgraph()
{
    for (char c = 0; c <= 0x19; ++c)
    {
        ASSERT(!isgraph(c), "isgraph");
    }
    ASSERT(!isgraph(' '), "isgraph");
    for (char c = '!'; c <= '~'; ++c)
    {
        ASSERT(isgraph(c), "isgraph");
    }
    ASSERT(!isgraph(0x7f), "isgraph");
}

void test_islower()
{
    for (char c = 'a'; c <= 'z'; ++c)
    {
        ASSERT(islower(c), "islower");
    }
    ASSERT(!islower('A'), "islower");
}

void test_isprint()
{
    for (char c = 0; c <= 0x19; ++c)
    {
        ASSERT(!isprint(c), "isprint");
    }
    for (char c = ' '; c <= '~'; ++c)
    {
        ASSERT(isprint(c), "isprint");
    }
    ASSERT(!isprint(0x7f), "isprint");
}

void test_ispunct()
{
    ASSERT(ispunct(','), "ispunct");
    ASSERT(!ispunct('a'), "ispunct");
}

void test_isspace()
{
    ASSERT(isspace(' '), "isspace");
    ASSERT(isspace('\t'), "isspace");
    ASSERT(isspace('\n'), "isspace");
    ASSERT(isspace('\v'), "isspace");
    ASSERT(isspace('\f'), "isspace");
    ASSERT(isspace('\r'), "isspace");
    for (char c = '!'; c <= '~'; ++c)
    {
        ASSERT(isspace(c), "isspace");
    }
}

void test_isupper()
{
    for (char c = 'A'; c <= 'Z'; ++c)
    {
        ASSERT(isupper(c), "isupper");
    }
    ASSERT(!isupper('a'), "isupper");
}

void test_isxdigit()
{
    for (char c = '0'; c <= '9'; ++c)
    {
        ASSERT(isxdigit(c), "isxdigit");
    }
    for (char c = 'A'; c <= 'F'; ++c)
    {
        ASSERT(isxdigit(c), "isxdigit");
    }
    for (char c = 'a'; c <= 'f'; ++c)
    {
        ASSERT(isxdigit(c), "isxdigit");
    }
    ASSERT(!isxdigit('j'), "isxdigit");
    ASSERT(!isxdigit('Z'), "isxdigit");
    ASSERT(!isxdigit('&'), "isxdigit");
}

void test_tolower()
{
    ASSERT(tolower('A') == 'a', "tolower");
}

void test_toupper()
{
    ASSERT(toupper('a') == 'A', "toupper");
}

void test_ctypes()
{
    test_isalnum();
    test_isalpha();
    test_isblank();
    test_iscntrl();
    test_isdigit();
    test_isgraph();
    test_islower();
    test_isprint();
    test_ispunct();
    test_isupper();
    test_isxdigit();
    test_tolower();
    test_toupper();
}

void test_pow()
{
    ASSERT(powf(2, 2) == 4.0f, "powf");
    ASSERT(pow(2, 2) == 4.0, "pow");
}

void test_math()
{
    test_pow();
}

void test_string()
{
    puts("Hello, world!");
    puts(getenv("HOME"));

    if (memcmp("bbb", "aaa", 3) > 0)
    {
        puts("1");
    }
    char buffer[4];
    memcpy(buffer, "abc", 4);
    puts(buffer);
    memmove(buffer + 1, buffer, 2);
    puts(buffer);

    char str[] = "- This, a sample string. ";
    puts(str);
    char* pch = strtok(str, " ,.-");
    while (pch)
    {
        puts(pch);
        pch = strtok(NULL, " ,.-");
    }
    printf("abc%c%lc%%%s\n", '%', L'💯', "abc💯");
    printf("%lc == %d\n", L'💯', 100);
}

void test_alloc()
{
    char* buffer = malloc(4);
    strcpy(buffer, "123");
    puts(buffer);
    free(buffer);
}

void test_all()
{
    test_ctypes();
    test_math();
    test_string();
    test_alloc();
}

int main(int argc, char const* argv[])
{
    test_all();

    return 0;
}
