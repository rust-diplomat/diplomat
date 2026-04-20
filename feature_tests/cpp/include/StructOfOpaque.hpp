#ifndef SOMELIB_StructOfOpaque_HPP
#define SOMELIB_StructOfOpaque_HPP

#include "StructOfOpaque.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Opaque.hpp"
#include "OpaqueMut.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    void StructOfOpaque_take_in(somelib::capi::StructOfOpaque* self, const somelib::capi::Opaque* other);

    } // extern "C"
} // namespace capi
} // namespace

inline void somelib::StructOfOpaque::take_in(const somelib::Opaque& other) {
    auto thisDiplomatRefClone = this->AsFFI();
    somelib::capi::StructOfOpaque_take_in(&thisDiplomatRefClone,
        other.AsFFI());
    *this = somelib::StructOfOpaque::FromFFI(thisDiplomatRefClone);
}


inline somelib::capi::StructOfOpaque somelib::StructOfOpaque::AsFFI() const {
    return somelib::capi::StructOfOpaque {
        /* .i = */ i->AsFFI(),
        /* .j = */ j->AsFFI(),
    };
}

inline somelib::StructOfOpaque somelib::StructOfOpaque::FromFFI(somelib::capi::StructOfOpaque c_struct) {
    return somelib::StructOfOpaque {
        /* .i = */ (somelib::Opaque*)somelib::Opaque::FromFFI(c_struct.i),
        /* .j = */ somelib::OpaqueMut::FromFFI(c_struct.j),
    };
}


#endif // SOMELIB_StructOfOpaque_HPP
