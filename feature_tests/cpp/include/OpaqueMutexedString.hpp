#ifndef OpaqueMutexedString_HPP
#define OpaqueMutexedString_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "OpaqueMutexedString.h"

class OpaqueMutexedString;
class Utf16Wrap;

/**
 * A destruction policy for using OpaqueMutexedString with std::unique_ptr.
 */
struct OpaqueMutexedStringDeleter {
  void operator()(capi::OpaqueMutexedString* l) const noexcept {
    capi::OpaqueMutexedString_destroy(l);
  }
};
class OpaqueMutexedString {
 public:
  static OpaqueMutexedString from_usize(size_t number);
  void change(size_t number) const;
  size_t get_len_and_add(size_t other) const;

  /**
   * Lifetimes: `this` must live at least as long as the output.
   */
  const std::string_view dummy_str() const;
  Utf16Wrap wrapper() const;
  inline const capi::OpaqueMutexedString* AsFFI() const { return this->inner.get(); }
  inline capi::OpaqueMutexedString* AsFFIMut() { return this->inner.get(); }
  inline explicit OpaqueMutexedString(capi::OpaqueMutexedString* i) : inner(i) {}
  OpaqueMutexedString() = default;
  OpaqueMutexedString(OpaqueMutexedString&&) noexcept = default;
  OpaqueMutexedString& operator=(OpaqueMutexedString&& other) noexcept = default;
 private:
  std::unique_ptr<capi::OpaqueMutexedString, OpaqueMutexedStringDeleter> inner;
};

#include "Utf16Wrap.hpp"

inline OpaqueMutexedString OpaqueMutexedString::from_usize(size_t number) {
  return OpaqueMutexedString(capi::OpaqueMutexedString_from_usize(number));
}
inline void OpaqueMutexedString::change(size_t number) const {
  capi::OpaqueMutexedString_change(this->inner.get(), number);
}
inline size_t OpaqueMutexedString::get_len_and_add(size_t other) const {
  return capi::OpaqueMutexedString_get_len_and_add(this->inner.get(), other);
}
inline const std::string_view OpaqueMutexedString::dummy_str() const {
  capi::DiplomatStringView diplomat_str_raw_out_value = capi::OpaqueMutexedString_dummy_str(this->inner.get());
  std::string_view str(diplomat_str_raw_out_value.data, diplomat_str_raw_out_value.len);
  return str;
}
inline Utf16Wrap OpaqueMutexedString::wrapper() const {
  return Utf16Wrap(capi::OpaqueMutexedString_wrapper(this->inner.get()));
}
#endif
