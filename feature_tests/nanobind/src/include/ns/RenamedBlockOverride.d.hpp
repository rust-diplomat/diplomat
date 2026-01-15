#ifndef SOMELIB_ns_RenamedBlockOverride_D_HPP
#define SOMELIB_ns_RenamedBlockOverride_D_HPP

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
    struct RenamedBlockOverride;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedBlockOverride {
public:

    inline const somelib::ns::capi::RenamedBlockOverride* AsFFI() const;
    inline somelib::ns::capi::RenamedBlockOverride* AsFFI();
    inline static const somelib::ns::RenamedBlockOverride* FromFFI(const somelib::ns::capi::RenamedBlockOverride* ptr);
    inline static somelib::ns::RenamedBlockOverride* FromFFI(somelib::ns::capi::RenamedBlockOverride* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedBlockOverride() = delete;
    RenamedBlockOverride(const somelib::ns::RenamedBlockOverride&) = delete;
    RenamedBlockOverride(somelib::ns::RenamedBlockOverride&&) noexcept = delete;
    RenamedBlockOverride operator=(const somelib::ns::RenamedBlockOverride&) = delete;
    RenamedBlockOverride operator=(somelib::ns::RenamedBlockOverride&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;

public:
    const static bool custom_bool = false;
    static std::string special_function();
};

} // namespace
#endif // SOMELIB_ns_RenamedBlockOverride_D_HPP
