#ifndef SOMELIB_NestedBorrowedFields_D_HPP
#define SOMELIB_NestedBorrowedFields_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "BorrowedFields.d.hpp"
#include "BorrowedFieldsWithBounds.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct Bar; }
class Bar;
namespace capi { struct Foo; }
class Foo;
struct BorrowedFields;
struct BorrowedFieldsWithBounds;
struct NestedBorrowedFields;
} // namespace somelib



namespace somelib {
namespace capi {
    struct NestedBorrowedFields {
      somelib::capi::BorrowedFields fields;
      somelib::capi::BorrowedFieldsWithBounds bounds;
      somelib::capi::BorrowedFieldsWithBounds bounds2;
    };

    typedef struct NestedBorrowedFields_option {union { NestedBorrowedFields ok; }; bool is_ok; } NestedBorrowedFields_option;
} // namespace capi
} // namespace


namespace somelib {
struct NestedBorrowedFields {
    somelib::BorrowedFields fields;
    somelib::BorrowedFieldsWithBounds bounds;
    somelib::BorrowedFieldsWithBounds bounds2;

  inline static somelib::diplomat::result<somelib::NestedBorrowedFields, somelib::diplomat::Utf8Error> from_bar_and_foo_and_strings(const somelib::Bar& bar, const somelib::Foo& foo, std::u16string_view dstr16_x, std::u16string_view dstr16_z, std::string_view utf8_str_y, std::string_view utf8_str_z);

    inline somelib::capi::NestedBorrowedFields AsFFI() const;
    inline static somelib::NestedBorrowedFields FromFFI(somelib::capi::NestedBorrowedFields c_struct);
};

} // namespace
#endif // SOMELIB_NestedBorrowedFields_D_HPP
