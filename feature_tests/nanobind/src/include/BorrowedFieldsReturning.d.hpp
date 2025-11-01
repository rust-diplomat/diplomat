#ifndef SOMELIB_BorrowedFieldsReturning_D_HPP
#define SOMELIB_BorrowedFieldsReturning_D_HPP

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
    struct BorrowedFieldsReturning {
      somelib::diplomat::capi::DiplomatStringView bytes;
    };

    typedef struct BorrowedFieldsReturning_option {union { BorrowedFieldsReturning ok; }; bool is_ok; } BorrowedFieldsReturning_option;
} // namespace capi
} // namespace


namespace somelib {
struct BorrowedFieldsReturning {
    std::string_view bytes;

    inline somelib::capi::BorrowedFieldsReturning AsFFI() const;
    inline static somelib::BorrowedFieldsReturning FromFFI(somelib::capi::BorrowedFieldsReturning c_struct);
};

} // namespace
#endif // SOMELIB_BorrowedFieldsReturning_D_HPP
