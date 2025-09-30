#ifndef SOMELIB_BorrowedFieldsReturning_HPP
#define SOMELIB_BorrowedFieldsReturning_HPP

#include "BorrowedFieldsReturning.d.hpp"

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

} // namespace capi
} // namespace


inline somelib::capi::BorrowedFieldsReturning somelib::BorrowedFieldsReturning::AsFFI() const {
    return somelib::capi::BorrowedFieldsReturning {
        /* .bytes = */ {bytes.data(), bytes.size()},
    };
}

inline somelib::BorrowedFieldsReturning somelib::BorrowedFieldsReturning::FromFFI(somelib::capi::BorrowedFieldsReturning c_struct) {
    return somelib::BorrowedFieldsReturning {
        /* .bytes = */ std::string_view(c_struct.bytes.data, c_struct.bytes.len),
    };
}


#endif // SOMELIB_BorrowedFieldsReturning_HPP
