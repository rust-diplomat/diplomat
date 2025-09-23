#ifndef ns_RenamedOpaqueRefIterator_D_HPP
#define ns_RenamedOpaqueRefIterator_D_HPP

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
    struct RenamedOpaqueRefIterator;
} // namespace capi
} // namespace

namespace ns {
class RenamedOpaqueRefIterator {
public:

  inline const ns::AttrOpaque1Renamed* next();

    inline const ns::capi::RenamedOpaqueRefIterator* AsFFI() const;
    inline ns::capi::RenamedOpaqueRefIterator* AsFFI();
    inline static const ns::RenamedOpaqueRefIterator* FromFFI(const ns::capi::RenamedOpaqueRefIterator* ptr);
    inline static ns::RenamedOpaqueRefIterator* FromFFI(ns::capi::RenamedOpaqueRefIterator* ptr);
    inline static void operator delete(void* ptr);
private:
    RenamedOpaqueRefIterator() = delete;
    RenamedOpaqueRefIterator(const ns::RenamedOpaqueRefIterator&) = delete;
    RenamedOpaqueRefIterator(ns::RenamedOpaqueRefIterator&&) noexcept = delete;
    RenamedOpaqueRefIterator operator=(const ns::RenamedOpaqueRefIterator&) = delete;
    RenamedOpaqueRefIterator operator=(ns::RenamedOpaqueRefIterator&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ns_RenamedOpaqueRefIterator_D_HPP
