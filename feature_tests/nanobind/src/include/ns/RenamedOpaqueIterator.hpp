#ifndef ns_RenamedOpaqueIterator_HPP
#define ns_RenamedOpaqueIterator_HPP

#include "RenamedOpaqueIterator.d.hpp"

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

    ns::capi::AttrOpaque1Renamed* namespace_OpaqueIterator_next(ns::capi::RenamedOpaqueIterator* self);

    void namespace_OpaqueIterator_destroy(RenamedOpaqueIterator* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<ns::AttrOpaque1Renamed> ns::RenamedOpaqueIterator::next() {
    auto result = ns::capi::namespace_OpaqueIterator_next(this->AsFFI());
    return std::unique_ptr<ns::AttrOpaque1Renamed>(ns::AttrOpaque1Renamed::FromFFI(result));
}

inline const ns::capi::RenamedOpaqueIterator* ns::RenamedOpaqueIterator::AsFFI() const {
    return reinterpret_cast<const ns::capi::RenamedOpaqueIterator*>(this);
}

inline ns::capi::RenamedOpaqueIterator* ns::RenamedOpaqueIterator::AsFFI() {
    return reinterpret_cast<ns::capi::RenamedOpaqueIterator*>(this);
}

inline const ns::RenamedOpaqueIterator* ns::RenamedOpaqueIterator::FromFFI(const ns::capi::RenamedOpaqueIterator* ptr) {
    return reinterpret_cast<const ns::RenamedOpaqueIterator*>(ptr);
}

inline ns::RenamedOpaqueIterator* ns::RenamedOpaqueIterator::FromFFI(ns::capi::RenamedOpaqueIterator* ptr) {
    return reinterpret_cast<ns::RenamedOpaqueIterator*>(ptr);
}

inline void ns::RenamedOpaqueIterator::operator delete(void* ptr) {
    ns::capi::namespace_OpaqueIterator_destroy(reinterpret_cast<ns::capi::RenamedOpaqueIterator*>(ptr));
}


#endif // ns_RenamedOpaqueIterator_HPP
