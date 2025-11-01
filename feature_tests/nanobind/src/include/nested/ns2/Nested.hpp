#ifndef SOMELIB_nested_ns2_Nested_HPP
#define SOMELIB_nested_ns2_Nested_HPP

#include "Nested.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../../diplomat_runtime.hpp"


namespace somelib::nested::ns2 {
namespace capi {
    extern "C" {

    void namespace_Nested2_destroy(Nested* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::nested::ns2::capi::Nested* somelib::nested::ns2::Nested::AsFFI() const {
    return reinterpret_cast<const somelib::nested::ns2::capi::Nested*>(this);
}

inline somelib::nested::ns2::capi::Nested* somelib::nested::ns2::Nested::AsFFI() {
    return reinterpret_cast<somelib::nested::ns2::capi::Nested*>(this);
}

inline const somelib::nested::ns2::Nested* somelib::nested::ns2::Nested::FromFFI(const somelib::nested::ns2::capi::Nested* ptr) {
    return reinterpret_cast<const somelib::nested::ns2::Nested*>(ptr);
}

inline somelib::nested::ns2::Nested* somelib::nested::ns2::Nested::FromFFI(somelib::nested::ns2::capi::Nested* ptr) {
    return reinterpret_cast<somelib::nested::ns2::Nested*>(ptr);
}

inline void somelib::nested::ns2::Nested::operator delete(void* ptr) {
    somelib::nested::ns2::capi::namespace_Nested2_destroy(reinterpret_cast<somelib::nested::ns2::capi::Nested*>(ptr));
}


#endif // SOMELIB_nested_ns2_Nested_HPP
