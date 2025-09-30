#ifndef SOMELIB_nested_ns_Nested_HPP
#define SOMELIB_nested_ns_Nested_HPP

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


namespace somelib::nested::ns {
namespace capi {
    extern "C" {

    void namespace_Nested_destroy(Nested* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::nested::ns::capi::Nested* somelib::nested::ns::Nested::AsFFI() const {
    return reinterpret_cast<const somelib::nested::ns::capi::Nested*>(this);
}

inline somelib::nested::ns::capi::Nested* somelib::nested::ns::Nested::AsFFI() {
    return reinterpret_cast<somelib::nested::ns::capi::Nested*>(this);
}

inline const somelib::nested::ns::Nested* somelib::nested::ns::Nested::FromFFI(const somelib::nested::ns::capi::Nested* ptr) {
    return reinterpret_cast<const somelib::nested::ns::Nested*>(ptr);
}

inline somelib::nested::ns::Nested* somelib::nested::ns::Nested::FromFFI(somelib::nested::ns::capi::Nested* ptr) {
    return reinterpret_cast<somelib::nested::ns::Nested*>(ptr);
}

inline void somelib::nested::ns::Nested::operator delete(void* ptr) {
    somelib::nested::ns::capi::namespace_Nested_destroy(reinterpret_cast<somelib::nested::ns::capi::Nested*>(ptr));
}


#endif // SOMELIB_nested_ns_Nested_HPP
