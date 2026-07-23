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
    extern "C" {
    void Foo_destroy(Foo* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class Foo;
using FooRef = somelib::diplomat::Ref<Foo, const somelib::capi::Foo>;
using FooRefMut = somelib::diplomat::Ref<Foo, somelib::capi::Foo>;

class Foo : public somelib::diplomat::OpaquePointer<Foo, somelib::capi::Foo, somelib::capi::Foo_destroy> {
public:

  inline static somelib::Foo new_(std::string_view x DIPLOMAT_LIFETIME_BOUND);

  inline somelib::Bar get_bar() const DIPLOMAT_LIFETIME_BOUND;

  inline static somelib::Foo new_static(std::string_view x);

  inline somelib::BorrowedFieldsReturning as_returning() const DIPLOMAT_LIFETIME_BOUND;

  inline static somelib::Foo extract_from_fields(somelib::BorrowedFields fields DIPLOMAT_LIFETIME_BOUND);

  /**
   * Test that the extraction logic correctly pins the right fields
   */
  inline static somelib::Foo extract_from_bounds(somelib::BorrowedFieldsWithBounds bounds DIPLOMAT_LIFETIME_BOUND, std::string_view another_string DIPLOMAT_LIFETIME_BOUND);

};

} // namespace
#endif // SOMELIB_Foo_D_HPP
