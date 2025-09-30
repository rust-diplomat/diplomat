#ifndef SOMELIB_ns_RenamedOpaqueRefIterator_D_HPP
#define SOMELIB_ns_RenamedOpaqueRefIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
namespace somelib {
namespace ns {
namespace capi { struct AttrOpaque1Renamed; }
class AttrOpaque1Renamed;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueRefIterator;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueRefIterator {
public:

  inline const somelib::ns::AttrOpaque1Renamed* next();

    inline const somelib::ns::capi::RenamedOpaqueRefIterator* AsFFI() const;
    inline somelib::ns::capi::RenamedOpaqueRefIterator* AsFFI();
    inline static const somelib::ns::RenamedOpaqueRefIterator* FromFFI(const somelib::ns::capi::RenamedOpaqueRefIterator* ptr);
    inline static somelib::ns::RenamedOpaqueRefIterator* FromFFI(somelib::ns::capi::RenamedOpaqueRefIterator* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueRefIterator() = delete;
    RenamedOpaqueRefIterator(const somelib::ns::RenamedOpaqueRefIterator&) = delete;
    RenamedOpaqueRefIterator(somelib::ns::RenamedOpaqueRefIterator&&) noexcept = delete;
    RenamedOpaqueRefIterator operator=(const somelib::ns::RenamedOpaqueRefIterator&) = delete;
    RenamedOpaqueRefIterator operator=(somelib::ns::RenamedOpaqueRefIterator&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueRefIterator_D_HPP
