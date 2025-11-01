#ifndef SOMELIB_BorrowedFieldsWithBounds_D_HPP
#define SOMELIB_BorrowedFieldsWithBounds_D_HPP

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
namespace capi { struct Foo; }
class Foo;
struct BorrowedFieldsWithBounds;
} // namespace somelib



namespace somelib {
namespace capi {
    struct BorrowedFieldsWithBounds {
      somelib::diplomat::capi::DiplomatString16View field_a;
      somelib::diplomat::capi::DiplomatStringView field_b;
      somelib::diplomat::capi::DiplomatStringView field_c;
    };

    typedef struct BorrowedFieldsWithBounds_option {union { BorrowedFieldsWithBounds ok; }; bool is_ok; } BorrowedFieldsWithBounds_option;
} // namespace capi
} // namespace


namespace somelib {
struct BorrowedFieldsWithBounds {
    std::u16string_view field_a;
    std::string_view field_b;
    std::string_view field_c;

  inline static somelib::diplomat::result<somelib::BorrowedFieldsWithBounds, somelib::diplomat::Utf8Error> from_foo_and_strings(const somelib::Foo& foo, std::u16string_view dstr16_x, std::string_view utf8_str_z);

    inline somelib::capi::BorrowedFieldsWithBounds AsFFI() const;
    inline static somelib::BorrowedFieldsWithBounds FromFFI(somelib::capi::BorrowedFieldsWithBounds c_struct);
};

} // namespace
#endif // SOMELIB_BorrowedFieldsWithBounds_D_HPP
