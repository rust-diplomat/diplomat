#ifndef SOMELIB_ns_RenamedTestOpaque_D_HPP
#define SOMELIB_ns_RenamedTestOpaque_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    struct RenamedTestOpaque;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedTestOpaque {
public:

    inline const somelib::ns::capi::RenamedTestOpaque* AsFFI() const;
    inline somelib::ns::capi::RenamedTestOpaque* AsFFI();
    inline static const somelib::ns::RenamedTestOpaque* FromFFI(const somelib::ns::capi::RenamedTestOpaque* ptr);
    inline static somelib::ns::RenamedTestOpaque* FromFFI(somelib::ns::capi::RenamedTestOpaque* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedTestOpaque() = delete;
    RenamedTestOpaque(const somelib::ns::RenamedTestOpaque&) = delete;
    RenamedTestOpaque(somelib::ns::RenamedTestOpaque&&) noexcept = delete;
    RenamedTestOpaque operator=(const somelib::ns::RenamedTestOpaque&) = delete;
    RenamedTestOpaque operator=(somelib::ns::RenamedTestOpaque&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedTestOpaque_D_HPP
