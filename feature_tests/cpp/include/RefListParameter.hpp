#ifndef SOMELIB_RefListParameter_HPP
#define SOMELIB_RefListParameter_HPP

#include "RefListParameter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    void RefListParameter_destroy(RefListParameter* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::capi::RefListParameter* somelib::RefListParameter::AsFFI() const {
    return reinterpret_cast<const somelib::capi::RefListParameter*>(this);
}

inline somelib::capi::RefListParameter* somelib::RefListParameter::AsFFI() {
    return reinterpret_cast<somelib::capi::RefListParameter*>(this);
}

inline const somelib::RefListParameter* somelib::RefListParameter::FromFFI(const somelib::capi::RefListParameter* ptr) {
    return reinterpret_cast<const somelib::RefListParameter*>(ptr);
}

inline somelib::RefListParameter* somelib::RefListParameter::FromFFI(somelib::capi::RefListParameter* ptr) {
    return reinterpret_cast<somelib::RefListParameter*>(ptr);
}

inline void somelib::RefListParameter::operator delete(void* ptr) {
    somelib::capi::RefListParameter_destroy(reinterpret_cast<somelib::capi::RefListParameter*>(ptr));
}


#endif // SOMELIB_RefListParameter_HPP
