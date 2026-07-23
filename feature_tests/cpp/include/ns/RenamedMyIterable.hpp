#ifndef SOMELIB_ns_RenamedMyIterable_HPP
#define SOMELIB_ns_RenamedMyIterable_HPP

#include "RenamedMyIterable.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "RenamedMyIterator.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {

    somelib::ns::capi::RenamedMyIterable* namespace_MyIterable_new(somelib::diplomat::capi::DiplomatU8View x);

    somelib::ns::capi::RenamedMyIterator* namespace_MyIterable_iter(const somelib::ns::capi::RenamedMyIterable* self);

    void namespace_MyIterable_destroy(RenamedMyIterable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::ns::RenamedMyIterable somelib::ns::RenamedMyIterable::new_(somelib::diplomat::span<const uint8_t> x) {
    auto result = somelib::ns::capi::namespace_MyIterable_new({x.data(), x.size()});
    return somelib::ns::RenamedMyIterable::FromFFI(result);
}

inline somelib::ns::RenamedMyIterator somelib::ns::RenamedMyIterable::iter() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::ns::capi::namespace_MyIterable_iter(this->AsFFI());
    return somelib::ns::RenamedMyIterator::FromFFI(result);
}


inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedMyIterator> somelib::ns::RenamedMyIterable::begin() const {
    return iter();
}


#endif // SOMELIB_ns_RenamedMyIterable_HPP
