#ifndef SOMELIB_BorrowingOptionStruct_HPP
#define SOMELIB_BorrowingOptionStruct_HPP

#include "BorrowingOptionStruct.d.hpp"

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


inline somelib::capi::BorrowingOptionStruct somelib::BorrowingOptionStruct::AsFFI() const {
    return somelib::capi::BorrowingOptionStruct {
        /* .a = */ a.has_value() ? (somelib::diplomat::capi::OptionStringView{ { {const_cast<somelib::diplomat::Optional<std::string_view>&>(a).value().data(), const_cast<somelib::diplomat::Optional<std::string_view>&>(a).value().size()} }, true }) : (somelib::diplomat::capi::OptionStringView{ {}, false }),
    };
}

inline somelib::BorrowingOptionStruct somelib::BorrowingOptionStruct::FromFFI(somelib::capi::BorrowingOptionStruct c_struct) {
    return somelib::BorrowingOptionStruct {
        /* .a = */ c_struct.a.is_ok ? somelib::diplomat::Optional(std::string_view(c_struct.a.ok.data, c_struct.a.ok.len)) : decltype(somelib::diplomat::Optional(std::string_view(c_struct.a.ok.data, c_struct.a.ok.len)))(std::nullopt),
    };
}


#endif // SOMELIB_BorrowingOptionStruct_HPP
