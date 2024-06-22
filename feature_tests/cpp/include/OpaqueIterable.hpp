#ifndef OpaqueIterable_HPP
#define OpaqueIterable_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "OpaqueIterable.h"

class OpaqueIterator;

/**
 * A destruction policy for using OpaqueIterable with std::unique_ptr.
 */
struct OpaqueIterableDeleter {
  void operator()(capi::OpaqueIterable* l) const noexcept {
    capi::namespace_OpaqueIterable_destroy(l);
  }
};
class OpaqueIterable {
 public:

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  OpaqueIterator iter() const;
  inline const capi::OpaqueIterable* AsFFI() const { return this->inner.get(); }
  inline capi::OpaqueIterable* AsFFIMut() { return this->inner.get(); }
  inline explicit OpaqueIterable(capi::OpaqueIterable* i) : inner(i) {}
  OpaqueIterable() = default;
  OpaqueIterable(OpaqueIterable&&) noexcept = default;
  OpaqueIterable& operator=(OpaqueIterable&& other) noexcept = default;
 private:
  std::unique_ptr<capi::OpaqueIterable, OpaqueIterableDeleter> inner;
};

#include "OpaqueIterator.hpp"

inline OpaqueIterator OpaqueIterable::iter() const {
  return OpaqueIterator(capi::namespace_OpaqueIterable_iter(this->inner.get()));
}
#endif
