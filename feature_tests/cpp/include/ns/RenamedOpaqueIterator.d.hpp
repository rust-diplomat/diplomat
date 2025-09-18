#ifndef SOMELIB_ns_RenamedOpaqueIterator_D_HPP
#define SOMELIB_ns_RenamedOpaqueIterator_D_HPP

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
    struct RenamedOpaqueIterator;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueIterator {
public:

  inline std::unique_ptr<somelib::ns::AttrOpaque1Renamed> next();

    inline const somelib::ns::capi::RenamedOpaqueIterator* AsFFI() const;
    inline somelib::ns::capi::RenamedOpaqueIterator* AsFFI();
    inline static const somelib::ns::RenamedOpaqueIterator* FromFFI(const somelib::ns::capi::RenamedOpaqueIterator* ptr);
    inline static somelib::ns::RenamedOpaqueIterator* FromFFI(somelib::ns::capi::RenamedOpaqueIterator* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueIterator() = delete;
    RenamedOpaqueIterator(const somelib::ns::RenamedOpaqueIterator&) = delete;
    RenamedOpaqueIterator(somelib::ns::RenamedOpaqueIterator&&) noexcept = delete;
    RenamedOpaqueIterator operator=(const somelib::ns::RenamedOpaqueIterator&) = delete;
    RenamedOpaqueIterator operator=(somelib::ns::RenamedOpaqueIterator&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueIterator_D_HPP
