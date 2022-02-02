#ifndef Counter_HPP
#define Counter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "Counter.h"
}

class Counter;

/**
 * A destruction policy for using Counter with std::unique_ptr.
 */
struct CounterDeleter {
  void operator()(capi::Counter* l) const noexcept {
    capi::Counter_destroy(l);
  }
};
class Counter {
 public:
  static Counter new_();
  size_t count() const;
  inline const capi::Counter* AsFFI() const { return this->inner.get(); }
  inline capi::Counter* AsFFIMut() { return this->inner.get(); }
  inline Counter(capi::Counter* i) : inner(i) {}
  Counter() = default;
  Counter(Counter&&) noexcept = default;
  Counter& operator=(Counter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Counter, CounterDeleter> inner;
};


inline Counter Counter::new_() {
  return Counter(capi::Counter_new());
}
inline size_t Counter::count() const {
  return capi::Counter_count(this->inner.get());
}
#endif
