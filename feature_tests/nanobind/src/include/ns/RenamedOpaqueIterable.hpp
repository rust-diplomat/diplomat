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


namespace somelib::ns {
namespace capi {
    extern "C" {

    somelib::ns::capi::RenamedOpaqueIterable* namespace_OpaqueIterable_new(size_t size);

    void namespace_OpaqueIterable_destroy(RenamedOpaqueIterable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::ns::RenamedOpaqueIterable> somelib::ns::RenamedOpaqueIterable::new_(size_t size) {
    auto result = somelib::ns::capi::namespace_OpaqueIterable_new(size);
    return std::unique_ptr<somelib::ns::RenamedOpaqueIterable>(somelib::ns::RenamedOpaqueIterable::FromFFI(result));
}

inline const somelib::ns::capi::RenamedOpaqueIterable* somelib::ns::RenamedOpaqueIterable::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedOpaqueIterable*>(this);
}

inline somelib::ns::capi::RenamedOpaqueIterable* somelib::ns::RenamedOpaqueIterable::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedOpaqueIterable*>(this);
}

inline const somelib::ns::RenamedOpaqueIterable* somelib::ns::RenamedOpaqueIterable::FromFFI(const somelib::ns::capi::RenamedOpaqueIterable* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedOpaqueIterable*>(ptr);
}

inline somelib::ns::RenamedOpaqueIterable* somelib::ns::RenamedOpaqueIterable::FromFFI(somelib::ns::capi::RenamedOpaqueIterable* ptr) {
    return reinterpret_cast<somelib::ns::RenamedOpaqueIterable*>(ptr);
}

inline void somelib::ns::RenamedOpaqueIterable::operator delete(void* ptr) {
    somelib::ns::capi::namespace_OpaqueIterable_destroy(reinterpret_cast<somelib::ns::capi::RenamedOpaqueIterable*>(ptr));
}


#endif // SOMELIB_ns_RenamedOpaqueIterable_HPP
