#ifndef Comparable_HPP
#define Comparable_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "Comparable.h"

class Comparable;

/**
 * A destruction policy for using Comparable with std::unique_ptr.
 */
struct ComparableDeleter {
  void operator()(capi::Comparable* l) const noexcept {
    capi::namespace_Comparable_destroy(l);
  }
};
class Comparable {
 public:
  static Comparable new_(uint8_t int);
  int8_t cmp(const Comparable& other) const;
  inline const capi::Comparable* AsFFI() const { return this->inner.get(); }
  inline capi::Comparable* AsFFIMut() { return this->inner.get(); }
  inline explicit Comparable(capi::Comparable* i) : inner(i) {}
  Comparable() = default;
  Comparable(Comparable&&) noexcept = default;
  Comparable& operator=(Comparable&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Comparable, ComparableDeleter> inner;
};


inline Comparable Comparable::new_(uint8_t int) {
  return Comparable(capi::namespace_Comparable_new(int));
}
inline int8_t Comparable::cmp(const Comparable& other) const {
  return capi::namespace_Comparable_cmp(this->inner.get(), other.AsFFI());
}
#endif
