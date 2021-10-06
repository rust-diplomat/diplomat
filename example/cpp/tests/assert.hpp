#include<iostream>
#include<cstdlib>

#define simple_assert_eq(name, lhs, rhs) \
    if (lhs != rhs) { \
        std::cout << __FILE__ ":" << __LINE__ << ": " << name << ": Expected " #lhs " == " #rhs ", found " << lhs << std::endl; \
        exit(1); \
    }

#define simple_assert(name, condition) \
    if (!condition) { \
        std::cout << __FILE__ ":" << __LINE__ << ": " << name << ": " #condition " failed" << std::endl; \
        exit(1); \
    }
