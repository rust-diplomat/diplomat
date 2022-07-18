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
  static Foo new_(const std::string_view x);
  Bar get_bar() const;
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

inline Foo Foo::new_(const std::string_view x) {
  return Foo(capi::Foo_new(x.data(), x.size()));
}
inline Bar Foo::get_bar() const {
  return Bar(capi::Foo_get_bar(this->inner.get()));
}
#endif
