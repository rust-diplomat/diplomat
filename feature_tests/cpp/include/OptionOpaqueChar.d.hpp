#ifndef SOMELIB_OptionOpaqueChar_D_HPP
#define SOMELIB_OptionOpaqueChar_D_HPP

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
namespace capi {
    struct OptionOpaqueChar;
} // namespace capi
} // namespace

namespace somelib {
class OptionOpaqueChar {
public:

  inline void assert_char(char32_t ch) const;

    inline const somelib::capi::OptionOpaqueChar* AsFFI() const;
    inline somelib::capi::OptionOpaqueChar* AsFFI();
    inline static const somelib::OptionOpaqueChar* FromFFI(const somelib::capi::OptionOpaqueChar* ptr);
    inline static somelib::OptionOpaqueChar* FromFFI(somelib::capi::OptionOpaqueChar* ptr);
    inline static void operator delete(void* ptr);
private:
    OptionOpaqueChar() = delete;
    OptionOpaqueChar(const somelib::OptionOpaqueChar&) = delete;
    OptionOpaqueChar(somelib::OptionOpaqueChar&&) noexcept = delete;
    OptionOpaqueChar operator=(const somelib::OptionOpaqueChar&) = delete;
    OptionOpaqueChar operator=(somelib::OptionOpaqueChar&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OptionOpaqueChar_D_HPP
