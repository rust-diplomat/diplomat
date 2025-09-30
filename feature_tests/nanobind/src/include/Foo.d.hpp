#ifndef SOMELIB_Foo_D_HPP
#define SOMELIB_Foo_D_HPP

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
namespace capi { struct Bar; }
class Bar;
namespace capi { struct Foo; }
class Foo;
struct BorrowedFields;
struct BorrowedFieldsReturning;
struct BorrowedFieldsWithBounds;
} // namespace somelib



namespace somelib {
namespace capi {
    struct Foo;
} // namespace capi
} // namespace

namespace somelib {
class Foo {
public:

  inline static std::unique_ptr<somelib::Foo> new_(std::string_view x);

  inline std::unique_ptr<somelib::Bar> get_bar() const;

  inline static std::unique_ptr<somelib::Foo> new_static(std::string_view x);

  inline somelib::BorrowedFieldsReturning as_returning() const;

  inline static std::unique_ptr<somelib::Foo> extract_from_fields(somelib::BorrowedFields fields);

  /**
   * Test that the extraction logic correctly pins the right fields
   */
  inline static std::unique_ptr<somelib::Foo> extract_from_bounds(somelib::BorrowedFieldsWithBounds bounds, std::string_view another_string);

    inline const somelib::capi::Foo* AsFFI() const;
    inline somelib::capi::Foo* AsFFI();
    inline static const somelib::Foo* FromFFI(const somelib::capi::Foo* ptr);
    inline static somelib::Foo* FromFFI(somelib::capi::Foo* ptr);
    inline static void operator delete(void* ptr);
private:
    Foo() = delete;
    Foo(const somelib::Foo&) = delete;
    Foo(somelib::Foo&&) noexcept = delete;
    Foo operator=(const somelib::Foo&) = delete;
    Foo operator=(somelib::Foo&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_Foo_D_HPP
