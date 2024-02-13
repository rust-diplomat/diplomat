#ifndef Unnamespaced_HPP
#define Unnamespaced_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "Unnamespaced.h"

#include "AttrEnum.hpp"
class Unnamespaced;
class AttrOpaque1;

/**
 * A destruction policy for using Unnamespaced with std::unique_ptr.
 */
struct UnnamespacedDeleter {
  void operator()(capi::Unnamespaced* l) const noexcept {
    capi::namespace_Unnamespaced_destroy(l);
  }
};
class Unnamespaced {
 public:
  static Unnamespaced make(AttrEnum e);
  void use_namespaced(const AttrOpaque1& _n) const;
  inline const capi::Unnamespaced* AsFFI() const { return this->inner.get(); }
  inline capi::Unnamespaced* AsFFIMut() { return this->inner.get(); }
  inline explicit Unnamespaced(capi::Unnamespaced* i) : inner(i) {}
  Unnamespaced() = default;
  Unnamespaced(Unnamespaced&&) noexcept = default;
  Unnamespaced& operator=(Unnamespaced&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Unnamespaced, UnnamespacedDeleter> inner;
};

#include "AttrOpaque1.hpp"

inline Unnamespaced Unnamespaced::make(AttrEnum e) {
  return Unnamespaced(capi::namespace_Unnamespaced_make(static_cast<capi::AttrEnum>(e)));
}
inline void Unnamespaced::use_namespaced(const AttrOpaque1& _n) const {
  capi::namespace_Unnamespaced_use_namespaced(this->inner.get(), _n.AsFFI());
}
#endif
