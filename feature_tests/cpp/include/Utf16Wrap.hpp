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

class Utf16Wrap;

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
  static Utf16Wrap from_utf16(const std::u16string_view input);
  template<typename W> void get_debug_str_to_write(W& write) const;
  std::string get_debug_str() const;

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


inline Utf16Wrap Utf16Wrap::from_utf16(const std::u16string_view input) {
  return Utf16Wrap(capi::Utf16Wrap_from_utf16(input.data(), input.size()));
}
template<typename W> inline void Utf16Wrap::get_debug_str_to_write(W& write) const {
  capi::DiplomatWrite write_writer = diplomat::WriteTrait<W>::Construct(write);
  capi::Utf16Wrap_get_debug_str(this->inner.get(), &write_writer);
}
inline std::string Utf16Wrap::get_debug_str() const {
  std::string diplomat_write_string;
  capi::DiplomatWrite diplomat_write_out = diplomat::WriteFromString(diplomat_write_string);
  capi::Utf16Wrap_get_debug_str(this->inner.get(), &diplomat_write_out);
  return diplomat_write_string;
}
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
