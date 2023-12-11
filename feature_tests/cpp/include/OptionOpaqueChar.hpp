#ifndef OptionOpaqueChar_HPP
#define OptionOpaqueChar_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "OptionOpaqueChar.h"


/**
 * A destruction policy for using OptionOpaqueChar with std::unique_ptr.
 */
struct OptionOpaqueCharDeleter {
  void operator()(capi::OptionOpaqueChar* l) const noexcept {
    capi::OptionOpaqueChar_destroy(l);
  }
};
class OptionOpaqueChar {
 public:
  void assert_char(char32_t ch) const;
  inline const capi::OptionOpaqueChar* AsFFI() const { return this->inner.get(); }
  inline capi::OptionOpaqueChar* AsFFIMut() { return this->inner.get(); }
  inline explicit OptionOpaqueChar(capi::OptionOpaqueChar* i) : inner(i) {}
  OptionOpaqueChar() = default;
  OptionOpaqueChar(OptionOpaqueChar&&) noexcept = default;
  OptionOpaqueChar& operator=(OptionOpaqueChar&& other) noexcept = default;
 private:
  std::unique_ptr<capi::OptionOpaqueChar, OptionOpaqueCharDeleter> inner;
};


inline void OptionOpaqueChar::assert_char(char32_t ch) const {
  capi::OptionOpaqueChar_assert_char(this->inner.get(), ch);
}
#endif
