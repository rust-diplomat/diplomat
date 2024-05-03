#ifndef NestedBorrowedFields_D_HPP
#define NestedBorrowedFields_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFields.d.hpp"
#include "BorrowedFieldsWithBounds.d.hpp"
#include "NestedBorrowedFields.d.h"

class Bar;
class Foo;
struct BorrowedFields;
struct BorrowedFieldsWithBounds;


struct NestedBorrowedFields {
  BorrowedFields fields;
  BorrowedFieldsWithBounds bounds;
  BorrowedFieldsWithBounds bounds2;

  inline static NestedBorrowedFields from_bar_and_foo_and_strings(const Bar& bar, const Foo& foo, std::u16string_view dstr16_x, std::u16string_view dstr16_z, std::string_view utf8_str_y, std::string_view utf8_str_z);

  inline capi::NestedBorrowedFields AsFFI() const;
  inline static NestedBorrowedFields FromFFI(capi::NestedBorrowedFields c_struct);
};


#endif // NestedBorrowedFields_D_HPP
