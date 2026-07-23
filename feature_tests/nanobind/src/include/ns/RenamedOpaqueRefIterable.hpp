#ifndef SOMELIB_ns_RenamedOpaqueRefIterable_HPP
#define SOMELIB_ns_RenamedOpaqueRefIterable_HPP

#include "RenamedOpaqueRefIterable.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "RenamedOpaqueRefIterator.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {

    somelib::ns::capi::RenamedOpaqueRefIterable* namespace_OpaqueRefIterable_new(size_t size);

    somelib::ns::capi::RenamedOpaqueRefIterator* namespace_OpaqueRefIterable_iter(const somelib::ns::capi::RenamedOpaqueRefIterable* self);

    void namespace_OpaqueRefIterable_destroy(RenamedOpaqueRefIterable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::ns::RenamedOpaqueRefIterable somelib::ns::RenamedOpaqueRefIterable::new_(size_t size) {
    auto result = somelib::ns::capi::namespace_OpaqueRefIterable_new(size);
    return somelib::ns::RenamedOpaqueRefIterable::FromFFI(result);
}

inline somelib::ns::RenamedOpaqueRefIterator somelib::ns::RenamedOpaqueRefIterable::iter() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::ns::capi::namespace_OpaqueRefIterable_iter(this->AsFFI());
    return somelib::ns::RenamedOpaqueRefIterator::FromFFI(result);
}


inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueRefIterator> somelib::ns::RenamedOpaqueRefIterable::begin() const {
    return iter();
}


#endif // SOMELIB_ns_RenamedOpaqueRefIterable_HPP
