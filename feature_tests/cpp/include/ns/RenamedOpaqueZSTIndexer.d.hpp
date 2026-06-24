#ifndef SOMELIB_ns_RenamedOpaqueZSTIndexer_D_HPP
#define SOMELIB_ns_RenamedOpaqueZSTIndexer_D_HPP

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
namespace capi { struct RenamedOpaqueZSTIndexer; }
class RenamedOpaqueZSTIndexer;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedOpaqueZSTIndexer;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedOpaqueZSTIndexer {
public:

  inline static std::unique_ptr<somelib::ns::RenamedOpaqueZSTIndexer> new_();

  inline std::unique_ptr<somelib::ns::RenamedOpaqueZSTIndexer> operator[](size_t idx) const;

    inline const somelib::ns::capi::RenamedOpaqueZSTIndexer* AsFFI() const;
    inline somelib::ns::capi::RenamedOpaqueZSTIndexer* AsFFI();
    inline static const somelib::ns::RenamedOpaqueZSTIndexer* FromFFI(const somelib::ns::capi::RenamedOpaqueZSTIndexer* ptr);
    inline static somelib::ns::RenamedOpaqueZSTIndexer* FromFFI(somelib::ns::capi::RenamedOpaqueZSTIndexer* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueZSTIndexer() = delete;
    RenamedOpaqueZSTIndexer(const somelib::ns::RenamedOpaqueZSTIndexer&) = delete;
    RenamedOpaqueZSTIndexer(somelib::ns::RenamedOpaqueZSTIndexer&&) noexcept = delete;
    RenamedOpaqueZSTIndexer operator=(const somelib::ns::RenamedOpaqueZSTIndexer&) = delete;
    RenamedOpaqueZSTIndexer operator=(somelib::ns::RenamedOpaqueZSTIndexer&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_ns_RenamedOpaqueZSTIndexer_D_HPP
