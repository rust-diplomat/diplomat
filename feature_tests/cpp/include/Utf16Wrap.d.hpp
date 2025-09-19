#ifndef SOMELIB_Utf16Wrap_D_HPP
#define SOMELIB_Utf16Wrap_D_HPP

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
namespace capi { struct Utf16Wrap; }
class Utf16Wrap;
} // namespace somelib



namespace somelib {
namespace capi {
    struct Utf16Wrap;
} // namespace capi
} // namespace

namespace somelib {
class Utf16Wrap {
public:

  inline static std::unique_ptr<somelib::Utf16Wrap> from_utf16(std::u16string_view input);

  inline std::string get_debug_str() const;
  template<typename W>
  inline void get_debug_str_write(W& writeable_output) const;

  inline std::u16string_view borrow_cont() const;

    inline const somelib::capi::Utf16Wrap* AsFFI() const;
    inline somelib::capi::Utf16Wrap* AsFFI();
    inline static const somelib::Utf16Wrap* FromFFI(const somelib::capi::Utf16Wrap* ptr);
    inline static somelib::Utf16Wrap* FromFFI(somelib::capi::Utf16Wrap* ptr);
    inline static void operator delete(void* ptr);
private:
    Utf16Wrap() = delete;
    Utf16Wrap(const somelib::Utf16Wrap&) = delete;
    Utf16Wrap(somelib::Utf16Wrap&&) noexcept = delete;
    Utf16Wrap operator=(const somelib::Utf16Wrap&) = delete;
    Utf16Wrap operator=(somelib::Utf16Wrap&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_Utf16Wrap_D_HPP
