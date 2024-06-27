#ifndef Foo_D_HPP
#define Foo_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "BorrowedFields.d.hpp"
#include "BorrowedFieldsReturning.d.hpp"
#include "BorrowedFieldsWithBounds.d.hpp"

namespace capi {typedef struct Bar Bar; }
class Bar;
struct BorrowedFields;
struct BorrowedFieldsReturning;
struct BorrowedFieldsWithBounds;


namespace capi {
    typedef struct Foo Foo;
}

class Foo {
public:

  inline static std::unique_ptr<Foo> new_(std::string_view x);

  inline std::unique_ptr<Bar> get_bar() const;

  inline static std::unique_ptr<Foo> new_static(std::string_view x);

  inline BorrowedFieldsReturning as_returning() const;

  inline static std::unique_ptr<Foo> extract_from_fields(BorrowedFields fields);

  inline static std::unique_ptr<Foo> extract_from_bounds(BorrowedFieldsWithBounds bounds, std::string_view another_string);

  inline const ::capi::Foo* AsFFI() const;
  inline ::capi::Foo* AsFFI();
  inline static const Foo* FromFFI(const ::capi::Foo* ptr);
  inline static Foo* FromFFI(::capi::Foo* ptr);
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
