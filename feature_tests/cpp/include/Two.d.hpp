#ifndef SOMELIB_Two_D_HPP
#define SOMELIB_Two_D_HPP

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
    struct Two;
} // namespace capi
} // namespace

namespace somelib {
class Two {
public:

    inline const somelib::capi::Two* AsFFI() const;
    inline somelib::capi::Two* AsFFI();
    inline static const somelib::Two* FromFFI(const somelib::capi::Two* ptr);
    inline static somelib::Two* FromFFI(somelib::capi::Two* ptr);
    inline static void operator delete(void* ptr);
private:
    Two() = delete;
    Two(const somelib::Two&) = delete;
    Two(somelib::Two&&) noexcept = delete;
    Two operator=(const somelib::Two&) = delete;
    Two operator=(somelib::Two&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_Two_D_HPP
