#ifndef OpaqueIterator_HPP
#define OpaqueIterator_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "OpaqueIterator.h"

class AttrOpaque1;

/**
 * A destruction policy for using OpaqueIterator with std::unique_ptr.
 */
struct OpaqueIteratorDeleter {
  void operator()(capi::OpaqueIterator* l) const noexcept {
    capi::namespace_OpaqueIterator_destroy(l);
  }
};
class OpaqueIterator {
 public:
  std::optional<AttrOpaque1> next();
  inline const capi::OpaqueIterator* AsFFI() const { return this->inner.get(); }
  inline capi::OpaqueIterator* AsFFIMut() { return this->inner.get(); }
  inline explicit OpaqueIterator(capi::OpaqueIterator* i) : inner(i) {}
  OpaqueIterator() = default;
  OpaqueIterator(OpaqueIterator&&) noexcept = default;
  OpaqueIterator& operator=(OpaqueIterator&& other) noexcept = default;
 private:
  std::unique_ptr<capi::OpaqueIterator, OpaqueIteratorDeleter> inner;
};

#include "AttrOpaque1.hpp"

inline std::optional<AttrOpaque1> OpaqueIterator::next() {
  auto diplomat_optional_raw_out_value = capi::namespace_OpaqueIterator_next(this->inner.get());
  std::optional<AttrOpaque1> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = AttrOpaque1(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
#endif
