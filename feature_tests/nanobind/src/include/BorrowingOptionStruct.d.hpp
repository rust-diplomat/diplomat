#ifndef SOMELIB_BorrowingOptionStruct_D_HPP
#define SOMELIB_BorrowingOptionStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    struct BorrowingOptionStruct {
      somelib::diplomat::capi::OptionStringView a;
    };

    typedef struct BorrowingOptionStruct_option {union { BorrowingOptionStruct ok; }; bool is_ok; } BorrowingOptionStruct_option;
} // namespace capi
} // namespace


namespace somelib {
struct BorrowingOptionStruct {
    std::optional<std::string_view> a;

    inline somelib::capi::BorrowingOptionStruct AsFFI() const;
    inline static somelib::BorrowingOptionStruct FromFFI(somelib::capi::BorrowingOptionStruct c_struct);
};

} // namespace
#endif // SOMELIB_BorrowingOptionStruct_D_HPP
