#ifndef Utf16Wrap_HPP
#define Utf16Wrap_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "Utf16Wrap.h"


/**
 * A destruction policy for using Utf16Wrap with std::unique_ptr.
 */
struct Utf16WrapDeleter {
  void operator()(capi::Utf16Wrap* l) const noexcept {
    capi::Utf16Wrap_destroy(l);
  }
};
class Utf16Wrap {
 public:

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  const std::u16string_view borrow_cont() const;
  const std::u16string_view owned() const;
  inline const capi::Utf16Wrap* AsFFI() const { return this->inner.get(); }
  inline capi::Utf16Wrap* AsFFIMut() { return this->inner.get(); }
  inline explicit Utf16Wrap(capi::Utf16Wrap* i) : inner(i) {}
  Utf16Wrap() = default;
  Utf16Wrap(Utf16Wrap&&) noexcept = default;
  Utf16Wrap& operator=(Utf16Wrap&& other) noexcept = default;
 private:
  std::unique_ptr<capi::Utf16Wrap, Utf16WrapDeleter> inner;
};


inline const std::u16string_view Utf16Wrap::borrow_cont() const {
  capi::DiplomatString16View diplomat_slice_raw_out_value = capi::Utf16Wrap_borrow_cont(this->inner.get());
  diplomat::span<const char16_t> slice(diplomat_slice_raw_out_value.data, diplomat_slice_raw_out_value.len);
  return slice;
}
inline const std::u16string_view Utf16Wrap::owned() const {
  capi::DiplomatString16View diplomat_slice_raw_out_value = capi::Utf16Wrap_owned(this->inner.get());
  diplomat::span<const char16_t> slice(diplomat_slice_raw_out_value.data, diplomat_slice_raw_out_value.len);
  return slice;
}
#endif
