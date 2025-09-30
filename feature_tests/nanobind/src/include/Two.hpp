#ifndef SOMELIB_Two_HPP
#define SOMELIB_Two_HPP

#include "Two.d.hpp"

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

    void Two_destroy(Two* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::capi::Two* somelib::Two::AsFFI() const {
    return reinterpret_cast<const somelib::capi::Two*>(this);
}

inline somelib::capi::Two* somelib::Two::AsFFI() {
    return reinterpret_cast<somelib::capi::Two*>(this);
}

inline const somelib::Two* somelib::Two::FromFFI(const somelib::capi::Two* ptr) {
    return reinterpret_cast<const somelib::Two*>(ptr);
}

inline somelib::Two* somelib::Two::FromFFI(somelib::capi::Two* ptr) {
    return reinterpret_cast<somelib::Two*>(ptr);
}

inline void somelib::Two::operator delete(void* ptr) {
    somelib::capi::Two_destroy(reinterpret_cast<somelib::capi::Two*>(ptr));
}


#endif // SOMELIB_Two_HPP
