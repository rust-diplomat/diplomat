#ifndef SOMELIB_BorrowedFields_HPP
#define SOMELIB_BorrowedFields_HPP

#include "BorrowedFields.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Bar.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::BorrowedFields BorrowedFields_from_bar_and_strings(const somelib::capi::Bar* bar, somelib::diplomat::capi::DiplomatString16View dstr16, somelib::diplomat::capi::DiplomatStringView utf8_str);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::result<somelib::BorrowedFields, somelib::diplomat::Utf8Error> somelib::BorrowedFields::from_bar_and_strings(const somelib::Bar& bar, std::u16string_view dstr16, std::string_view utf8_str) {
    if (!somelib::diplomat::capi::diplomat_is_str(utf8_str.data(), utf8_str.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    auto result = somelib::capi::BorrowedFields_from_bar_and_strings(bar.AsFFI(),
        {dstr16.data(), dstr16.size()},
        {utf8_str.data(), utf8_str.size()});
    return somelib::diplomat::Ok<somelib::BorrowedFields>(somelib::BorrowedFields::FromFFI(result));
}


inline somelib::capi::BorrowedFields somelib::BorrowedFields::AsFFI() const {
    return somelib::capi::BorrowedFields {
        /* .a = */ {a.data(), a.size()},
        /* .b = */ {b.data(), b.size()},
        /* .c = */ {c.data(), c.size()},
    };
}

inline somelib::BorrowedFields somelib::BorrowedFields::FromFFI(somelib::capi::BorrowedFields c_struct) {
    return somelib::BorrowedFields {
        /* .a = */ std::u16string_view(c_struct.a.data, c_struct.a.len),
        /* .b = */ std::string_view(c_struct.b.data, c_struct.b.len),
        /* .c = */ std::string_view(c_struct.c.data, c_struct.c.len),
    };
}


#endif // SOMELIB_BorrowedFields_HPP
