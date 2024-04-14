#ifndef MyIterable_HPP
#define MyIterable_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "MyIterable.h"

class MyIterable;
class MyIterator;

/**
 * A destruction policy for using MyIterable with std::unique_ptr.
 */
struct MyIterableDeleter {
  void operator()(capi::MyIterable* l) const noexcept {
    capi::namespace_MyIterable_destroy(l);
  }
};
class MyIterable {
 public:
  static MyIterable new_(const diplomat::span<const uint8_t> x);

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  MyIterator iter() const;
  inline const capi::MyIterable* AsFFI() const { return this->inner.get(); }
  inline capi::MyIterable* AsFFIMut() { return this->inner.get(); }
  inline explicit MyIterable(capi::MyIterable* i) : inner(i) {}
  MyIterable() = default;
  MyIterable(MyIterable&&) noexcept = default;
  MyIterable& operator=(MyIterable&& other) noexcept = default;
 private:
  std::unique_ptr<capi::MyIterable, MyIterableDeleter> inner;
};

#include "MyIterator.hpp"

inline MyIterable MyIterable::new_(const diplomat::span<const uint8_t> x) {
  return MyIterable(capi::namespace_MyIterable_new(x.data(), x.size()));
}
inline MyIterator MyIterable::iter() const {
  return MyIterator(capi::namespace_MyIterable_iter(this->inner.get()));
}
#endif
