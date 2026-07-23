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

inline somelib::diplomat::Optional<somelib::ns::AttrOpaque1Renamed> somelib::ns::RenamedOpaqueIterator::next() {
    auto result = somelib::ns::capi::namespace_OpaqueIterator_next(this->AsFFI());
    return somelib::diplomat::Optional<somelib::ns::AttrOpaque1Renamed>::FromFFI(result);
}


#endif // SOMELIB_ns_RenamedOpaqueIterator_HPP
