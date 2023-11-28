#ifndef Foo_HPP
#define Foo_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "Foo.h"

class Foo;
class Bar;
struct BorrowedFieldsReturning;
struct BorrowedFields;

/**
 * A destruction policy for using Foo with std::unique_ptr.
 */
struct FooDeleter {
  void operator()(capi::Foo* l) const noexcept {
    capi::Foo_destroy(l);
  }
};
class Foo {
 public:

  /**
   * Lifetimes: `x` must live at least as long as the output.
   */
  static Foo new_(const std::string_view x);

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  Bar get_bar() const;

  /**
   * Lifetimes: `x` must live for the duration of the program.
   */
  static Foo new_static(const std::string_view x);

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  BorrowedFieldsReturning as_returning() const;

  /**
   * Lifetimes: `fields` must live at least as long as the output.
   */
  static Foo extract_from_fields(BorrowedFields fields);
  inline const capi::Foo* AsFFI() const { return this->inner.get(); }
  inline capi::Foo* AsFFIMut() { return this->inner.get(); }
  inline Foo(capi::Foo* i) : inner(i) {}
  Foo() = default;
  Foo(Foo&&) noexcept = default;
  Foo& operator=(Foo&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Foo, FooDeleter> inner;
};

#include "Bar.hpp"
#include "BorrowedFieldsReturning.hpp"
#include "BorrowedFields.hpp"

inline Foo Foo::new_(const std::string_view x) {
  return Foo(capi::Foo_new(x.data(), x.size()));
}
inline Bar Foo::get_bar() const {
  return Bar(capi::Foo_get_bar(this->inner.get()));
}
inline Foo Foo::new_static(const std::string_view x) {
  return Foo(capi::Foo_new_static(x.data(), x.size()));
}
inline BorrowedFieldsReturning Foo::as_returning() const {
  capi::BorrowedFieldsReturning diplomat_raw_struct_out_value = capi::Foo_as_returning(this->inner.get());
  capi::DiplomatStringView diplomat_str_raw_out_value_bytes = diplomat_raw_struct_out_value.bytes;
  std::string_view str(diplomat_str_raw_out_value_bytes.data, diplomat_str_raw_out_value_bytes.len);
  return BorrowedFieldsReturning{ .bytes = std::move(str) };
}
inline Foo Foo::extract_from_fields(BorrowedFields fields) {
  BorrowedFields diplomat_wrapped_struct_fields = fields;
  return Foo(capi::Foo_extract_from_fields(capi::BorrowedFields{ .a = { diplomat_wrapped_struct_fields.a.data(), diplomat_wrapped_struct_fields.a.size() }, .b = { diplomat_wrapped_struct_fields.b.data(), diplomat_wrapped_struct_fields.b.size() }, .c = { diplomat_wrapped_struct_fields.c.data(), diplomat_wrapped_struct_fields.c.size() } }));
}
#endif
