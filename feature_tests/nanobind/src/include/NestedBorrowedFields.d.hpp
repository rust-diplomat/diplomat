#ifndef NestedBorrowedFields_D_HPP
#define NestedBorrowedFields_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "BorrowedFields.d.hpp"
#include "BorrowedFieldsWithBounds.d.hpp"
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct Bar; }
class Bar;
namespace diplomat::capi { struct Foo; }
class Foo;
struct BorrowedFields;
struct BorrowedFieldsWithBounds;


namespace diplomat {
namespace capi {
    struct NestedBorrowedFields {
      diplomat::capi::BorrowedFields fields;
      diplomat::capi::BorrowedFieldsWithBounds bounds;
      diplomat::capi::BorrowedFieldsWithBounds bounds2;
    };
    
    typedef struct NestedBorrowedFields_option {union { NestedBorrowedFields ok; }; bool is_ok; } NestedBorrowedFields_option;
} // namespace capi
} // namespace


struct NestedBorrowedFields {
  BorrowedFields fields;
  BorrowedFieldsWithBounds bounds;
  BorrowedFieldsWithBounds bounds2;

  inline static diplomat::result<NestedBorrowedFields, diplomat::Utf8Error> from_bar_and_foo_and_strings(const Bar& bar, const Foo& foo, std::u16string_view dstr16_x, std::u16string_view dstr16_z, std::string_view utf8_str_y, std::string_view utf8_str_z);

  inline diplomat::capi::NestedBorrowedFields AsFFI() const;
  inline static NestedBorrowedFields FromFFI(diplomat::capi::NestedBorrowedFields c_struct);
};


#endif // NestedBorrowedFields_D_HPP
