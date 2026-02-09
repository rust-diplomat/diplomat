#ifndef SOMELIB_Unnamespaced_D_HPP
#define SOMELIB_Unnamespaced_D_HPP

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
namespace ns {
namespace capi { struct AttrOpaque1Renamed; }
class AttrOpaque1Renamed;
} // namespace ns
} // namespace somelib



namespace somelib {
namespace capi {
    struct Unnamespaced;
} // namespace capi
} // namespace

namespace somelib {
class Unnamespaced {
public:

  inline void use_namespaced(const somelib::ns::AttrOpaque1Renamed& _n) const;

    inline const somelib::capi::Unnamespaced* AsFFI() const;
    inline somelib::capi::Unnamespaced* AsFFI();
    inline static const somelib::Unnamespaced* FromFFI(const somelib::capi::Unnamespaced* ptr);
    inline static somelib::Unnamespaced* FromFFI(somelib::capi::Unnamespaced* ptr);
    inline static void operator delete(void* ptr);
private:
    Unnamespaced() = delete;
    Unnamespaced(const somelib::Unnamespaced&) = delete;
    Unnamespaced(somelib::Unnamespaced&&) noexcept = delete;
    Unnamespaced operator=(const somelib::Unnamespaced&) = delete;
    Unnamespaced operator=(somelib::Unnamespaced&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_Unnamespaced_D_HPP
