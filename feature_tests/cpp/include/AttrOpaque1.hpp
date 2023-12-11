#ifndef AttrOpaque1_HPP
#define AttrOpaque1_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "AttrOpaque1.h"


/**
 * A destruction policy for using AttrOpaque1 with std::unique_ptr.
 */
struct AttrOpaque1Deleter {
  void operator()(capi::AttrOpaque1* l) const noexcept {
    capi::AttrOpaque1_destroy(l);
  }
};
class AttrOpaque1 {
 public:
  void method() const;
  void method_disabledcpp() const;
  inline const capi::AttrOpaque1* AsFFI() const { return this->inner.get(); }
  inline capi::AttrOpaque1* AsFFIMut() { return this->inner.get(); }
  inline explicit AttrOpaque1(capi::AttrOpaque1* i) : inner(i) {}
  AttrOpaque1() = default;
  AttrOpaque1(AttrOpaque1&&) noexcept = default;
  AttrOpaque1& operator=(AttrOpaque1&& other) noexcept = default;
 private:
  std::unique_ptr<capi::AttrOpaque1, AttrOpaque1Deleter> inner;
};


inline void AttrOpaque1::method() const {
  capi::AttrOpaque1_method(this->inner.get());
}
inline void AttrOpaque1::method_disabledcpp() const {
  capi::AttrOpaque1_method_disabledcpp(this->inner.get());
}
#endif
