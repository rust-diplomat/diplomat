#ifndef SOMELIB_ns_RenamedDeprecatedOpaque_HPP
#define SOMELIB_ns_RenamedDeprecatedOpaque_HPP

#include "RenamedDeprecatedOpaque.d.hpp"

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

    void namespace_DeprecatedOpaque_destroy(RenamedDeprecatedOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::ns::capi::RenamedDeprecatedOpaque* somelib::ns::RenamedDeprecatedOpaque::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedDeprecatedOpaque*>(this);
}

inline somelib::ns::capi::RenamedDeprecatedOpaque* somelib::ns::RenamedDeprecatedOpaque::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedDeprecatedOpaque*>(this);
}

inline const somelib::ns::RenamedDeprecatedOpaque* somelib::ns::RenamedDeprecatedOpaque::FromFFI(const somelib::ns::capi::RenamedDeprecatedOpaque* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedDeprecatedOpaque*>(ptr);
}

inline somelib::ns::RenamedDeprecatedOpaque* somelib::ns::RenamedDeprecatedOpaque::FromFFI(somelib::ns::capi::RenamedDeprecatedOpaque* ptr) {
    return reinterpret_cast<somelib::ns::RenamedDeprecatedOpaque*>(ptr);
}

inline void somelib::ns::RenamedDeprecatedOpaque::operator delete(void* ptr) {
    somelib::ns::capi::namespace_DeprecatedOpaque_destroy(reinterpret_cast<somelib::ns::capi::RenamedDeprecatedOpaque*>(ptr));
}


#endif // SOMELIB_ns_RenamedDeprecatedOpaque_HPP
