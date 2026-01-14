#ifndef SOMELIB_ns_RenamedNonCustomType_D_HPP
#define SOMELIB_ns_RenamedNonCustomType_D_HPP

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
    struct RenamedNonCustomType;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedNonCustomType {
public:

    inline const somelib::ns::capi::RenamedNonCustomType* AsFFI() const;
    inline somelib::ns::capi::RenamedNonCustomType* AsFFI();
    inline static const somelib::ns::RenamedNonCustomType* FromFFI(const somelib::ns::capi::RenamedNonCustomType* ptr);
    inline static somelib::ns::RenamedNonCustomType* FromFFI(somelib::ns::capi::RenamedNonCustomType* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedNonCustomType() = delete;
    RenamedNonCustomType(const somelib::ns::RenamedNonCustomType&) = delete;
    RenamedNonCustomType(somelib::ns::RenamedNonCustomType&&) noexcept = delete;
    RenamedNonCustomType operator=(const somelib::ns::RenamedNonCustomType&) = delete;
    RenamedNonCustomType operator=(somelib::ns::RenamedNonCustomType&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedNonCustomType_D_HPP
