#ifndef SOMELIB_ns_RenamedOpaqueIterable_HPP
#define SOMELIB_ns_RenamedOpaqueIterable_HPP

#include "RenamedOpaqueIterable.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "RenamedOpaqueIterator.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {

    somelib::ns::capi::RenamedOpaqueIterable* namespace_OpaqueIterable_new(size_t size);

    somelib::ns::capi::RenamedOpaqueIterator* namespace_OpaqueIterable_iter(const somelib::ns::capi::RenamedOpaqueIterable* self);

    void namespace_OpaqueIterable_destroy(RenamedOpaqueIterable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::ns::RenamedOpaqueIterable somelib::ns::RenamedOpaqueIterable::new_(size_t size) {
    auto result = somelib::ns::capi::namespace_OpaqueIterable_new(size);
    return somelib::ns::RenamedOpaqueIterable::FromFFI(result);
}

inline somelib::ns::RenamedOpaqueIterator somelib::ns::RenamedOpaqueIterable::iter() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::ns::capi::namespace_OpaqueIterable_iter(this->AsFFI());
    return somelib::ns::RenamedOpaqueIterator::FromFFI(result);
}


inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueIterator> somelib::ns::RenamedOpaqueIterable::begin() const {
    return iter();
}


#endif // SOMELIB_ns_RenamedOpaqueIterable_HPP
