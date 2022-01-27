#ifndef CountedOpaque_HPP
#define CountedOpaque_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "CountedOpaque.h"
}

class Counter;
class CountedOpaque;

/**
 * A destruction policy for using CountedOpaque with std::unique_ptr.
 */
struct CountedOpaqueDeleter {
  void operator()(capi::CountedOpaque* l) const noexcept {
    capi::CountedOpaque_destroy(l);
  }
};
class CountedOpaque {
 public:
  static CountedOpaque new_(const Counter& counter);
  inline const capi::CountedOpaque* AsFFI() const { return this->inner.get(); }
  inline capi::CountedOpaque* AsFFIMut() { return this->inner.get(); }
  inline CountedOpaque(capi::CountedOpaque* i) : inner(i) {}
  CountedOpaque() = default;
  CountedOpaque(CountedOpaque&&) noexcept = default;
  CountedOpaque& operator=(CountedOpaque&& other) noexcept = default;
 private:
  std::unique_ptr<capi::CountedOpaque, CountedOpaqueDeleter> inner;
};

#include "Counter.hpp"

inline CountedOpaque CountedOpaque::new_(const Counter& counter) {
  return CountedOpaque(capi::CountedOpaque_new(counter.AsFFI()));
}
#endif
