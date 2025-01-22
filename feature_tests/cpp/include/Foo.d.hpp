#ifndef Foo_D_HPP
#define Foo_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct Bar; }
class Bar;
struct BorrowedFields;
struct BorrowedFieldsReturning;
struct BorrowedFieldsWithBounds;


namespace diplomat {
namespace capi {
    struct Foo;
} // namespace capi
} // namespace

class Foo {
public:

  inline static std::unique_ptr<Foo> new_(std::string_view x);

  inline std::unique_ptr<Bar> get_bar() const;

  inline static std::unique_ptr<Foo> new_static(std::string_view x);

  inline BorrowedFieldsReturning as_returning() const;

  inline static std::unique_ptr<Foo> extract_from_fields(BorrowedFields fields);

  inline static std::unique_ptr<Foo> extract_from_bounds(BorrowedFieldsWithBounds bounds, std::string_view another_string);

  inline const diplomat::capi::Foo* AsFFI() const;
  inline diplomat::capi::Foo* AsFFI();
  inline static const Foo* FromFFI(const diplomat::capi::Foo* ptr);
  inline static Foo* FromFFI(diplomat::capi::Foo* ptr);
  inline static void operator delete(void* ptr);
private:
  Foo() = delete;
  Foo(const Foo&) = delete;
  Foo(Foo&&) noexcept = delete;
  Foo operator=(const Foo&) = delete;
  Foo operator=(Foo&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Foo_D_HPP
