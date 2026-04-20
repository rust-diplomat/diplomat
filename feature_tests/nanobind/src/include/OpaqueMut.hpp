#ifndef SOMELIB_OpaqueMut_HPP
#define SOMELIB_OpaqueMut_HPP

#include "OpaqueMut.d.hpp"

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

    somelib::capi::OpaqueMut* OpaqueMut_new(void);

    void OpaqueMut_destroy(OpaqueMut* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::OpaqueMut> somelib::OpaqueMut::new_() {
    auto result = somelib::capi::OpaqueMut_new();
    return std::unique_ptr<somelib::OpaqueMut>(somelib::OpaqueMut::FromFFI(result));
}

inline const somelib::capi::OpaqueMut* somelib::OpaqueMut::AsFFI() const {
    return reinterpret_cast<const somelib::capi::OpaqueMut*>(this);
}

inline somelib::capi::OpaqueMut* somelib::OpaqueMut::AsFFI() {
    return reinterpret_cast<somelib::capi::OpaqueMut*>(this);
}

inline const somelib::OpaqueMut* somelib::OpaqueMut::FromFFI(const somelib::capi::OpaqueMut* ptr) {
    return reinterpret_cast<const somelib::OpaqueMut*>(ptr);
}

inline somelib::OpaqueMut* somelib::OpaqueMut::FromFFI(somelib::capi::OpaqueMut* ptr) {
    return reinterpret_cast<somelib::OpaqueMut*>(ptr);
}

inline void somelib::OpaqueMut::operator delete(void* ptr) {
    somelib::capi::OpaqueMut_destroy(reinterpret_cast<somelib::capi::OpaqueMut*>(ptr));
}


#endif // SOMELIB_OpaqueMut_HPP
