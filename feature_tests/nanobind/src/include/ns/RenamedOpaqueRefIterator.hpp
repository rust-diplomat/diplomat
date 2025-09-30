#ifndef SOMELIB_ns_RenamedOpaqueRefIterator_HPP
#define SOMELIB_ns_RenamedOpaqueRefIterator_HPP

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


namespace somelib::ns {
namespace capi {
    extern "C" {

    const somelib::ns::capi::AttrOpaque1Renamed* namespace_OpaqueRefIterator_next(somelib::ns::capi::RenamedOpaqueRefIterator* self);

    void namespace_OpaqueRefIterator_destroy(RenamedOpaqueRefIterator* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::ns::AttrOpaque1Renamed* somelib::ns::RenamedOpaqueRefIterator::next() {
    auto result = somelib::ns::capi::namespace_OpaqueRefIterator_next(this->AsFFI());
    return somelib::ns::AttrOpaque1Renamed::FromFFI(result);
}

inline const somelib::ns::capi::RenamedOpaqueRefIterator* somelib::ns::RenamedOpaqueRefIterator::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedOpaqueRefIterator*>(this);
}

inline somelib::ns::capi::RenamedOpaqueRefIterator* somelib::ns::RenamedOpaqueRefIterator::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedOpaqueRefIterator*>(this);
}

inline const somelib::ns::RenamedOpaqueRefIterator* somelib::ns::RenamedOpaqueRefIterator::FromFFI(const somelib::ns::capi::RenamedOpaqueRefIterator* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedOpaqueRefIterator*>(ptr);
}

inline somelib::ns::RenamedOpaqueRefIterator* somelib::ns::RenamedOpaqueRefIterator::FromFFI(somelib::ns::capi::RenamedOpaqueRefIterator* ptr) {
    return reinterpret_cast<somelib::ns::RenamedOpaqueRefIterator*>(ptr);
}

inline void somelib::ns::RenamedOpaqueRefIterator::operator delete(void* ptr) {
    somelib::ns::capi::namespace_OpaqueRefIterator_destroy(reinterpret_cast<somelib::ns::capi::RenamedOpaqueRefIterator*>(ptr));
}


#endif // SOMELIB_ns_RenamedOpaqueRefIterator_HPP
