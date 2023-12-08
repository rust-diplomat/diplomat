#ifndef RefListParameter_HPP
#define RefListParameter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "RefListParameter.h"


/**
 * A destruction policy for using RefListParameter with std::unique_ptr.
 */
struct RefListParameterDeleter {
  void operator()(capi::RefListParameter* l) const noexcept {
    capi::RefListParameter_destroy(l);
  }
};
class RefListParameter {
 public:
  inline const capi::RefListParameter* AsFFI() const { return this->inner.get(); }
  inline capi::RefListParameter* AsFFIMut() { return this->inner.get(); }
  inline explicit RefListParameter(capi::RefListParameter* i) : inner(i) {}
  RefListParameter() = default;
  RefListParameter(RefListParameter&&) noexcept = default;
  RefListParameter& operator=(RefListParameter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::RefListParameter, RefListParameterDeleter> inner;
};


#endif
