#ifndef SOMELIB_OpaqueMutexedString_D_HPP
#define SOMELIB_OpaqueMutexedString_D_HPP

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
namespace capi { struct OpaqueMutexedString; }
class OpaqueMutexedString;
namespace capi { struct Utf16Wrap; }
class Utf16Wrap;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OpaqueMutexedString;
} // namespace capi
} // namespace

namespace somelib {
class OpaqueMutexedString {
public:

  inline static std::unique_ptr<somelib::OpaqueMutexedString> from_usize(size_t number);

  inline void change(size_t number) const;

  inline const somelib::OpaqueMutexedString& borrow() const;

  inline static const somelib::OpaqueMutexedString& borrow_other(const somelib::OpaqueMutexedString& other);

  inline const somelib::OpaqueMutexedString& borrow_self_or_other(const somelib::OpaqueMutexedString& other) const;

  inline size_t get_len_and_add(size_t other) const;

  inline std::string_view dummy_str() const;

  inline std::unique_ptr<somelib::Utf16Wrap> wrapper() const;

  inline uint16_t to_unsigned_from_unsigned(uint16_t input) const;

    inline const somelib::capi::OpaqueMutexedString* AsFFI() const;
    inline somelib::capi::OpaqueMutexedString* AsFFI();
    inline static const somelib::OpaqueMutexedString* FromFFI(const somelib::capi::OpaqueMutexedString* ptr);
    inline static somelib::OpaqueMutexedString* FromFFI(somelib::capi::OpaqueMutexedString* ptr);
    inline static void operator delete(void* ptr);
private:
    OpaqueMutexedString() = delete;
    OpaqueMutexedString(const somelib::OpaqueMutexedString&) = delete;
    OpaqueMutexedString(somelib::OpaqueMutexedString&&) noexcept = delete;
    OpaqueMutexedString operator=(const somelib::OpaqueMutexedString&) = delete;
    OpaqueMutexedString operator=(somelib::OpaqueMutexedString&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_OpaqueMutexedString_D_HPP
