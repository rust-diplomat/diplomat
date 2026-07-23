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

inline somelib::diplomat::Optional<somelib::ns::AttrOpaque1RenamedRef> somelib::ns::RenamedOpaqueRefIterator::next() DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::ns::capi::namespace_OpaqueRefIterator_next(this->AsFFI());
    return somelib::diplomat::Optional<somelib::ns::AttrOpaque1RenamedRef>::FromFFI(result);
}


#endif // SOMELIB_ns_RenamedOpaqueRefIterator_HPP
