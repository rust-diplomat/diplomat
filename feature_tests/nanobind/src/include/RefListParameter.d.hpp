#ifndef SOMELIB_RefListParameter_D_HPP
#define SOMELIB_RefListParameter_D_HPP

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
    struct RefListParameter;
} // namespace capi
} // namespace

namespace somelib {
class RefListParameter {
public:

    inline const somelib::capi::RefListParameter* AsFFI() const;
    inline somelib::capi::RefListParameter* AsFFI();
    inline static const somelib::RefListParameter* FromFFI(const somelib::capi::RefListParameter* ptr);
    inline static somelib::RefListParameter* FromFFI(somelib::capi::RefListParameter* ptr);
    inline static void operator delete(void* ptr);
private:
    RefListParameter() = delete;
    RefListParameter(const somelib::RefListParameter&) = delete;
    RefListParameter(somelib::RefListParameter&&) noexcept = delete;
    RefListParameter operator=(const somelib::RefListParameter&) = delete;
    RefListParameter operator=(somelib::RefListParameter&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_RefListParameter_D_HPP
