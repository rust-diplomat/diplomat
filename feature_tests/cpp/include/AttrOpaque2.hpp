#ifndef AttrOpaque2_HPP
#define AttrOpaque2_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "AttrOpaque2.h"


/**
 * A destruction policy for using AttrOpaque2 with std::unique_ptr.
 */
struct AttrOpaque2Deleter {
  void operator()(capi::AttrOpaque2* l) const noexcept {
    capi::AttrOpaque2_destroy(l);
  }
};
class AttrOpaque2 {
 public:
  inline const capi::AttrOpaque2* AsFFI() const { return this->inner.get(); }
  inline capi::AttrOpaque2* AsFFIMut() { return this->inner.get(); }
  inline explicit AttrOpaque2(capi::AttrOpaque2* i) : inner(i) {}
  AttrOpaque2() = default;
  AttrOpaque2(AttrOpaque2&&) noexcept = default;
  AttrOpaque2& operator=(AttrOpaque2&& other) noexcept = default;
 private:
  std::unique_ptr<capi::AttrOpaque2, AttrOpaque2Deleter> inner;
};


#endif
