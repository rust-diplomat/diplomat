#include<iostream>
#include<cstdlib>

#define assert_eq(name, lhs, rhs) \
    if (lhs != rhs) { \
        std::cout << __FILE__ ":" << __LINE__ << ": " << name << ": Expected " #lhs " == " #rhs ", found " << lhs << std::endl; \
        exit(1); \
    }

#define assert(name, condition) \
    if (!condition) { \
        std::cout << __FILE__ ":" << __LINE__ << ": " << name << ": " #condition " failed" << std::endl; \
        exit(1); \
    }
