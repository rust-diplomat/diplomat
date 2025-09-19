#ifndef SOMELIB_PrimitiveStruct_HPP
#define SOMELIB_PrimitiveStruct_HPP

#include "PrimitiveStruct.d.hpp"

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

    void PrimitiveStruct_mutable_slice(somelib::capi::DiplomatPrimitiveStructViewMut a);

    void PrimitiveStruct_mutable_ref(somelib::capi::PrimitiveStruct* self, somelib::capi::PrimitiveStruct* a);

    } // extern "C"
} // namespace capi
} // namespace

inline void somelib::PrimitiveStruct::mutable_slice(somelib::diplomat::span<somelib::PrimitiveStruct> a) {
    somelib::capi::PrimitiveStruct_mutable_slice({reinterpret_cast<somelib::capi::PrimitiveStruct*>(a.data()), a.size()});
}

inline void somelib::PrimitiveStruct::mutable_ref(somelib::PrimitiveStruct& a) {
    somelib::capi::PrimitiveStruct_mutable_ref(reinterpret_cast<somelib::capi::PrimitiveStruct*>(this),
        reinterpret_cast<somelib::capi::PrimitiveStruct*>(&a));
}


inline somelib::capi::PrimitiveStruct somelib::PrimitiveStruct::AsFFI() const {
    return somelib::capi::PrimitiveStruct {
        /* .x = */ x,
        /* .a = */ a,
        /* .b = */ b,
        /* .c = */ c,
        /* .d = */ d,
        /* .e = */ e,
    };
}

inline somelib::PrimitiveStruct somelib::PrimitiveStruct::FromFFI(somelib::capi::PrimitiveStruct c_struct) {
    return somelib::PrimitiveStruct {
        /* .x = */ c_struct.x,
        /* .a = */ c_struct.a,
        /* .b = */ c_struct.b,
        /* .c = */ c_struct.c,
        /* .d = */ c_struct.d,
        /* .e = */ c_struct.e,
    };
}


#endif // SOMELIB_PrimitiveStruct_HPP
