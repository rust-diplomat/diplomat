#ifndef Utf16Wrap_D_HPP
#define Utf16Wrap_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct Utf16Wrap;
} // namespace capi
} // namespace

class Utf16Wrap {
public:

  inline static std::unique_ptr<Utf16Wrap> from_utf16(std::u16string_view input);

  inline std::string get_debug_str() const;

  inline std::u16string_view borrow_cont() const;

  inline const diplomat::capi::Utf16Wrap* AsFFI() const;
  inline diplomat::capi::Utf16Wrap* AsFFI();
  inline static const Utf16Wrap* FromFFI(const diplomat::capi::Utf16Wrap* ptr);
  inline static Utf16Wrap* FromFFI(diplomat::capi::Utf16Wrap* ptr);
  inline static void operator delete(void* ptr);
private:
  Utf16Wrap() = delete;
  Utf16Wrap(const Utf16Wrap&) = delete;
  Utf16Wrap(Utf16Wrap&&) noexcept = delete;
  Utf16Wrap operator=(const Utf16Wrap&) = delete;
  Utf16Wrap operator=(Utf16Wrap&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Utf16Wrap_D_HPP
