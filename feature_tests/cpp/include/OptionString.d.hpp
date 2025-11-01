#ifndef SOMELIB_OptionString_D_HPP
#define SOMELIB_OptionString_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct OptionString; }
class OptionString;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OptionString;
} // namespace capi
} // namespace

namespace somelib {
class OptionString {
public:

  inline static std::unique_ptr<somelib::OptionString> new_(std::string_view diplomat_str);

  inline somelib::diplomat::result<std::string, std::monostate> write() const;
  template<typename W>
  inline somelib::diplomat::result<std::monostate, std::monostate> write_write(W& writeable_output) const;

  inline std::optional<std::string_view> borrow() const;

    inline const somelib::capi::OptionString* AsFFI() const;
    inline somelib::capi::OptionString* AsFFI();
    inline static const somelib::OptionString* FromFFI(const somelib::capi::OptionString* ptr);
    inline static somelib::OptionString* FromFFI(somelib::capi::OptionString* ptr);
    inline static void operator delete(void* ptr);
private:
    OptionString() = delete;
    OptionString(const somelib::OptionString&) = delete;
    OptionString(somelib::OptionString&&) noexcept = delete;
    OptionString operator=(const somelib::OptionString&) = delete;
    OptionString operator=(somelib::OptionString&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OptionString_D_HPP
