#ifndef OptionString_D_HPP
#define OptionString_D_HPP

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
    struct OptionString;
} // namespace capi
} // namespace

class OptionString {
public:

  inline static std::unique_ptr<OptionString> new_(std::string_view diplomat_str);

  inline diplomat::result<std::string, std::monostate> write() const;

  inline std::optional<std::string_view> borrow() const;

  inline const diplomat::capi::OptionString* AsFFI() const;
  inline diplomat::capi::OptionString* AsFFI();
  inline static const OptionString* FromFFI(const diplomat::capi::OptionString* ptr);
  inline static OptionString* FromFFI(diplomat::capi::OptionString* ptr);
  inline static void operator delete(void* ptr);
private:
  OptionString() = delete;
  OptionString(const OptionString&) = delete;
  OptionString(OptionString&&) noexcept = delete;
  OptionString operator=(const OptionString&) = delete;
  OptionString operator=(OptionString&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // OptionString_D_HPP
