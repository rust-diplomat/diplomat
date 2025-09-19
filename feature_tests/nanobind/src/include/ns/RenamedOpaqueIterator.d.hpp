#ifndef ns_RenamedOpaqueIterator_D_HPP
#define ns_RenamedOpaqueIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace ns {
namespace capi { struct AttrOpaque1Renamed; }
class AttrOpaque1Renamed;
} // namespace ns




namespace ns {
namespace capi {
    struct RenamedOpaqueIterator;
} // namespace capi
} // namespace

namespace ns {
class RenamedOpaqueIterator {
public:

  inline std::unique_ptr<ns::AttrOpaque1Renamed> next();

    inline const ns::capi::RenamedOpaqueIterator* AsFFI() const;
    inline ns::capi::RenamedOpaqueIterator* AsFFI();
    inline static const ns::RenamedOpaqueIterator* FromFFI(const ns::capi::RenamedOpaqueIterator* ptr);
    inline static ns::RenamedOpaqueIterator* FromFFI(ns::capi::RenamedOpaqueIterator* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueIterator() = delete;
    RenamedOpaqueIterator(const ns::RenamedOpaqueIterator&) = delete;
    RenamedOpaqueIterator(ns::RenamedOpaqueIterator&&) noexcept = delete;
    RenamedOpaqueIterator operator=(const ns::RenamedOpaqueIterator&) = delete;
    RenamedOpaqueIterator operator=(ns::RenamedOpaqueIterator&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedOpaqueIterator_D_HPP
