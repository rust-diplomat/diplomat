#ifndef SOMELIB_BorrowedFieldsWithBounds_HPP
#define SOMELIB_BorrowedFieldsWithBounds_HPP

#include "BorrowedFieldsWithBounds.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Foo.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::BorrowedFieldsWithBounds BorrowedFieldsWithBounds_from_foo_and_strings(const somelib::capi::Foo* foo, somelib::diplomat::capi::DiplomatString16View dstr16_x, somelib::diplomat::capi::DiplomatStringView utf8_str_z);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::result<somelib::BorrowedFieldsWithBounds, somelib::diplomat::Utf8Error> somelib::BorrowedFieldsWithBounds::from_foo_and_strings(const somelib::Foo& foo, std::u16string_view dstr16_x, std::string_view utf8_str_z) {
    if (!somelib::diplomat::capi::diplomat_is_str(utf8_str_z.data(), utf8_str_z.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    auto result = somelib::capi::BorrowedFieldsWithBounds_from_foo_and_strings(foo.AsFFI(),
        {dstr16_x.data(), dstr16_x.size()},
        {utf8_str_z.data(), utf8_str_z.size()});
    return somelib::diplomat::Ok<somelib::BorrowedFieldsWithBounds>(somelib::BorrowedFieldsWithBounds::FromFFI(result));
}


inline somelib::capi::BorrowedFieldsWithBounds somelib::BorrowedFieldsWithBounds::AsFFI() const {
    return somelib::capi::BorrowedFieldsWithBounds {
        /* .field_a = */ {field_a.data(), field_a.size()},
        /* .field_b = */ {field_b.data(), field_b.size()},
        /* .field_c = */ {field_c.data(), field_c.size()},
    };
}

inline somelib::BorrowedFieldsWithBounds somelib::BorrowedFieldsWithBounds::FromFFI(somelib::capi::BorrowedFieldsWithBounds c_struct) {
    return somelib::BorrowedFieldsWithBounds {
        /* .field_a = */ std::u16string_view(c_struct.field_a.data, c_struct.field_a.len),
        /* .field_b = */ std::string_view(c_struct.field_b.data, c_struct.field_b.len),
        /* .field_c = */ std::string_view(c_struct.field_c.data, c_struct.field_c.len),
    };
}


#endif // SOMELIB_BorrowedFieldsWithBounds_HPP
