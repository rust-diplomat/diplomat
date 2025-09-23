#ifndef ns_RenamedOpaqueRefIterator_HPP
#define ns_RenamedOpaqueRefIterator_HPP

#include "RenamedOpaqueRefIterator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "AttrOpaque1Renamed.hpp"


namespace ns {
namespace capi {
    extern "C" {

    const ns::capi::AttrOpaque1Renamed* namespace_OpaqueRefIterator_next(ns::capi::RenamedOpaqueRefIterator* self);

    void namespace_OpaqueRefIterator_destroy(RenamedOpaqueRefIterator* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const ns::AttrOpaque1Renamed* ns::RenamedOpaqueRefIterator::next() {
    auto result = ns::capi::namespace_OpaqueRefIterator_next(this->AsFFI());
    return ns::AttrOpaque1Renamed::FromFFI(result);
}

inline const ns::capi::RenamedOpaqueRefIterator* ns::RenamedOpaqueRefIterator::AsFFI() const {
    return reinterpret_cast<const ns::capi::RenamedOpaqueRefIterator*>(this);
}

inline ns::capi::RenamedOpaqueRefIterator* ns::RenamedOpaqueRefIterator::AsFFI() {
    return reinterpret_cast<ns::capi::RenamedOpaqueRefIterator*>(this);
}

inline const ns::RenamedOpaqueRefIterator* ns::RenamedOpaqueRefIterator::FromFFI(const ns::capi::RenamedOpaqueRefIterator* ptr) {
    return reinterpret_cast<const ns::RenamedOpaqueRefIterator*>(ptr);
}

inline ns::RenamedOpaqueRefIterator* ns::RenamedOpaqueRefIterator::FromFFI(ns::capi::RenamedOpaqueRefIterator* ptr) {
    return reinterpret_cast<ns::RenamedOpaqueRefIterator*>(ptr);
}

inline void ns::RenamedOpaqueRefIterator::operator delete(void* ptr) {
    ns::capi::namespace_OpaqueRefIterator_destroy(reinterpret_cast<ns::capi::RenamedOpaqueRefIterator*>(ptr));
}


#endif // ns_RenamedOpaqueRefIterator_HPP
