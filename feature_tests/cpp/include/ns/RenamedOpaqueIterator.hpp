#ifndef SOMELIB_ns_RenamedOpaqueIterator_HPP
#define SOMELIB_ns_RenamedOpaqueIterator_HPP

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


namespace somelib::ns {
namespace capi {
    extern "C" {

    somelib::ns::capi::AttrOpaque1Renamed* namespace_OpaqueIterator_next(somelib::ns::capi::RenamedOpaqueIterator* self);

    void namespace_OpaqueIterator_destroy(RenamedOpaqueIterator* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::ns::AttrOpaque1Renamed> somelib::ns::RenamedOpaqueIterator::next() {
    auto result = somelib::ns::capi::namespace_OpaqueIterator_next(this->AsFFI());
    return std::unique_ptr<somelib::ns::AttrOpaque1Renamed>(somelib::ns::AttrOpaque1Renamed::FromFFI(result));
}

inline const somelib::ns::capi::RenamedOpaqueIterator* somelib::ns::RenamedOpaqueIterator::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedOpaqueIterator*>(this);
}

inline somelib::ns::capi::RenamedOpaqueIterator* somelib::ns::RenamedOpaqueIterator::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedOpaqueIterator*>(this);
}

inline const somelib::ns::RenamedOpaqueIterator* somelib::ns::RenamedOpaqueIterator::FromFFI(const somelib::ns::capi::RenamedOpaqueIterator* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedOpaqueIterator*>(ptr);
}

inline somelib::ns::RenamedOpaqueIterator* somelib::ns::RenamedOpaqueIterator::FromFFI(somelib::ns::capi::RenamedOpaqueIterator* ptr) {
    return reinterpret_cast<somelib::ns::RenamedOpaqueIterator*>(ptr);
}

inline void somelib::ns::RenamedOpaqueIterator::operator delete(void* ptr) {
    somelib::ns::capi::namespace_OpaqueIterator_destroy(reinterpret_cast<somelib::ns::capi::RenamedOpaqueIterator*>(ptr));
}


#endif // SOMELIB_ns_RenamedOpaqueIterator_HPP
