#ifndef Two_HPP
#define Two_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "Two.h"


/**
 * A destruction policy for using Two with std::unique_ptr.
 */
struct TwoDeleter {
  void operator()(capi::Two* l) const noexcept {
    capi::Two_destroy(l);
  }
};
class Two {
 public:
  inline const capi::Two* AsFFI() const { return this->inner.get(); }
  inline capi::Two* AsFFIMut() { return this->inner.get(); }
  inline explicit Two(capi::Two* i) : inner(i) {}
  Two() = default;
  Two(Two&&) noexcept = default;
  Two& operator=(Two&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Two, TwoDeleter> inner;
};


#endif
