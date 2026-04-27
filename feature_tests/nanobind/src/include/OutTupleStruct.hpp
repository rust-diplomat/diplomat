#ifndef SOMELIB_OutTupleStruct_HPP
#define SOMELIB_OutTupleStruct_HPP

#include "OutTupleStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Opaque.hpp"
#include "PrimitiveStruct.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::OutTupleStruct OutTupleStruct_new(void);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::OutTupleStruct somelib::OutTupleStruct::new_() {
    auto result = somelib::capi::OutTupleStruct_new();
    return somelib::OutTupleStruct::FromFFI(result);
}


inline somelib::capi::OutTupleStruct somelib::OutTupleStruct::AsFFI() const {
    return somelib::capi::OutTupleStruct {
        /* .x = */ x,
        /* .y = */ y,
        /* .primitive = */ primitive.AsFFI(),
        /* .opaque = */ opaque->AsFFI(),
    };
}

inline somelib::OutTupleStruct somelib::OutTupleStruct::FromFFI(somelib::capi::OutTupleStruct c_struct) {
    return somelib::OutTupleStruct {
        /* .x = */ c_struct.x,
        /* .y = */ c_struct.y,
        /* .primitive = */ somelib::PrimitiveStruct::FromFFI(c_struct.primitive),
        /* .opaque = */ std::unique_ptr<somelib::Opaque>(somelib::Opaque::FromFFI(c_struct.opaque)),
    };
}


#endif // SOMELIB_OutTupleStruct_HPP
