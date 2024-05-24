#ifndef Bar_HPP
#define Bar_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "Bar.h"


/**
 * A destruction policy for using Bar with std::unique_ptr.
 */
struct BarDeleter {
  void operator()(capi::Bar* l) const noexcept {
    capi::Bar_destroy(l);
  }
};
class Bar {
 public:
  inline const capi::Bar* AsFFI() const { return this->inner.get(); }
  inline capi::Bar* AsFFIMut() { return this->inner.get(); }
  inline explicit Bar(capi::Bar* i) : inner(i) {}
  Bar() = default;
  Bar(Bar&&) noexcept = default;
  Bar& operator=(Bar&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Bar, BarDeleter> inner;
};


#endif
