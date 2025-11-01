#ifndef SOMELIB_NestedBorrowedFields_HPP
#define SOMELIB_NestedBorrowedFields_HPP

#include "NestedBorrowedFields.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Bar.hpp"
#include "BorrowedFields.hpp"
#include "BorrowedFieldsWithBounds.hpp"
#include "Foo.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::NestedBorrowedFields NestedBorrowedFields_from_bar_and_foo_and_strings(const somelib::capi::Bar* bar, const somelib::capi::Foo* foo, somelib::diplomat::capi::DiplomatString16View dstr16_x, somelib::diplomat::capi::DiplomatString16View dstr16_z, somelib::diplomat::capi::DiplomatStringView utf8_str_y, somelib::diplomat::capi::DiplomatStringView utf8_str_z);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::result<somelib::NestedBorrowedFields, somelib::diplomat::Utf8Error> somelib::NestedBorrowedFields::from_bar_and_foo_and_strings(const somelib::Bar& bar, const somelib::Foo& foo, std::u16string_view dstr16_x, std::u16string_view dstr16_z, std::string_view utf8_str_y, std::string_view utf8_str_z) {
    if (!somelib::diplomat::capi::diplomat_is_str(utf8_str_y.data(), utf8_str_y.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    if (!somelib::diplomat::capi::diplomat_is_str(utf8_str_z.data(), utf8_str_z.size())) {
    return somelib::diplomat::Err<somelib::diplomat::Utf8Error>();
  }
    auto result = somelib::capi::NestedBorrowedFields_from_bar_and_foo_and_strings(bar.AsFFI(),
        foo.AsFFI(),
        {dstr16_x.data(), dstr16_x.size()},
        {dstr16_z.data(), dstr16_z.size()},
        {utf8_str_y.data(), utf8_str_y.size()},
        {utf8_str_z.data(), utf8_str_z.size()});
    return somelib::diplomat::Ok<somelib::NestedBorrowedFields>(somelib::NestedBorrowedFields::FromFFI(result));
}


inline somelib::capi::NestedBorrowedFields somelib::NestedBorrowedFields::AsFFI() const {
    return somelib::capi::NestedBorrowedFields {
        /* .fields = */ fields.AsFFI(),
        /* .bounds = */ bounds.AsFFI(),
        /* .bounds2 = */ bounds2.AsFFI(),
    };
}

inline somelib::NestedBorrowedFields somelib::NestedBorrowedFields::FromFFI(somelib::capi::NestedBorrowedFields c_struct) {
    return somelib::NestedBorrowedFields {
        /* .fields = */ somelib::BorrowedFields::FromFFI(c_struct.fields),
        /* .bounds = */ somelib::BorrowedFieldsWithBounds::FromFFI(c_struct.bounds),
        /* .bounds2 = */ somelib::BorrowedFieldsWithBounds::FromFFI(c_struct.bounds2),
    };
}


#endif // SOMELIB_NestedBorrowedFields_HPP
