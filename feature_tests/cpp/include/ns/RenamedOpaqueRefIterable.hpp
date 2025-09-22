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

inline std::unique_ptr<somelib::ns::RenamedOpaqueRefIterable> somelib::ns::RenamedOpaqueRefIterable::new_(size_t size) {
    auto result = somelib::ns::capi::namespace_OpaqueRefIterable_new(size);
    return std::unique_ptr<somelib::ns::RenamedOpaqueRefIterable>(somelib::ns::RenamedOpaqueRefIterable::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueRefIterator> somelib::ns::RenamedOpaqueRefIterable::iter() const {
    auto result = somelib::ns::capi::namespace_OpaqueRefIterable_iter(this->AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueRefIterator>(somelib::ns::RenamedOpaqueRefIterator::FromFFI(result));
}

inline somelib::diplomat::next_to_iter_helper<somelib::ns::RenamedOpaqueRefIterator>somelib::ns::RenamedOpaqueRefIterable::begin() const {
    return iter();
}

inline const somelib::ns::capi::RenamedOpaqueRefIterable* somelib::ns::RenamedOpaqueRefIterable::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedOpaqueRefIterable*>(this);
}

inline somelib::ns::capi::RenamedOpaqueRefIterable* somelib::ns::RenamedOpaqueRefIterable::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedOpaqueRefIterable*>(this);
}

inline const somelib::ns::RenamedOpaqueRefIterable* somelib::ns::RenamedOpaqueRefIterable::FromFFI(const somelib::ns::capi::RenamedOpaqueRefIterable* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedOpaqueRefIterable*>(ptr);
}

inline somelib::ns::RenamedOpaqueRefIterable* somelib::ns::RenamedOpaqueRefIterable::FromFFI(somelib::ns::capi::RenamedOpaqueRefIterable* ptr) {
    return reinterpret_cast<somelib::ns::RenamedOpaqueRefIterable*>(ptr);
}

inline void somelib::ns::RenamedOpaqueRefIterable::operator delete(void* ptr) {
    somelib::ns::capi::namespace_OpaqueRefIterable_destroy(reinterpret_cast<somelib::ns::capi::RenamedOpaqueRefIterable*>(ptr));
}


#endif // SOMELIB_ns_RenamedOpaqueRefIterable_HPP
