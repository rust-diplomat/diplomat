#ifndef SOMELIB_ns_RenamedTestOpaque_HPP
#define SOMELIB_ns_RenamedTestOpaque_HPP

#include "RenamedTestOpaque.d.hpp"

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

    void namespace_TestOpaque_destroy(RenamedTestOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::ns::capi::RenamedTestOpaque* somelib::ns::RenamedTestOpaque::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedTestOpaque*>(this);
}

inline somelib::ns::capi::RenamedTestOpaque* somelib::ns::RenamedTestOpaque::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedTestOpaque*>(this);
}

inline const somelib::ns::RenamedTestOpaque* somelib::ns::RenamedTestOpaque::FromFFI(const somelib::ns::capi::RenamedTestOpaque* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedTestOpaque*>(ptr);
}

inline somelib::ns::RenamedTestOpaque* somelib::ns::RenamedTestOpaque::FromFFI(somelib::ns::capi::RenamedTestOpaque* ptr) {
    return reinterpret_cast<somelib::ns::RenamedTestOpaque*>(ptr);
}

inline void somelib::ns::RenamedTestOpaque::operator delete(void* ptr) {
    somelib::ns::capi::namespace_TestOpaque_destroy(reinterpret_cast<somelib::ns::capi::RenamedTestOpaque*>(ptr));
}


#endif // SOMELIB_ns_RenamedTestOpaque_HPP
