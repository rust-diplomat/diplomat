#ifndef SOMELIB_ns_RenamedAttrOpaque2_D_HPP
#define SOMELIB_ns_RenamedAttrOpaque2_D_HPP

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
    struct RenamedAttrOpaque2;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedAttrOpaque2 {
public:

    inline const somelib::ns::capi::RenamedAttrOpaque2* AsFFI() const;
    inline somelib::ns::capi::RenamedAttrOpaque2* AsFFI();
    inline static const somelib::ns::RenamedAttrOpaque2* FromFFI(const somelib::ns::capi::RenamedAttrOpaque2* ptr);
    inline static somelib::ns::RenamedAttrOpaque2* FromFFI(somelib::ns::capi::RenamedAttrOpaque2* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedAttrOpaque2() = delete;
    RenamedAttrOpaque2(const somelib::ns::RenamedAttrOpaque2&) = delete;
    RenamedAttrOpaque2(somelib::ns::RenamedAttrOpaque2&&) noexcept = delete;
    RenamedAttrOpaque2 operator=(const somelib::ns::RenamedAttrOpaque2&) = delete;
    RenamedAttrOpaque2 operator=(somelib::ns::RenamedAttrOpaque2&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedAttrOpaque2_D_HPP
