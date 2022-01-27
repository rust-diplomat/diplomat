#ifndef OwnershipEater_HPP
#define OwnershipEater_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "OwnershipEater.h"
}

class OwnershipEater;

/**
 * A destruction policy for using OwnershipEater with std::unique_ptr.
 */
struct OwnershipEaterDeleter {
  void operator()(capi::OwnershipEater* l) const noexcept {
    capi::OwnershipEater_destroy(l);
  }
};
class OwnershipEater {
 public:
  static OwnershipEater new_();
  inline const capi::OwnershipEater* AsFFI() const { return this->inner.get(); }
  inline capi::OwnershipEater* AsFFIMut() { return this->inner.get(); }
  inline OwnershipEater(capi::OwnershipEater* i) : inner(i) {}
  OwnershipEater() = default;
  OwnershipEater(OwnershipEater&&) noexcept = default;
  OwnershipEater& operator=(OwnershipEater&& other) noexcept = default;
 private:
  std::unique_ptr<capi::OwnershipEater, OwnershipEaterDeleter> inner;
};


inline OwnershipEater OwnershipEater::new_() {
  return OwnershipEater(capi::OwnershipEater_new());
}
#endif
