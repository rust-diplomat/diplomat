#ifndef SOMELIB_TupleStruct_HPP
#define SOMELIB_TupleStruct_HPP

#include "TupleStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "ContainingTuple.hpp"
#include "MyStruct.hpp"
#include "Opaque.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    int32_t TupleStruct_takes_st_as_tuple(somelib::capi::TupleStruct a);

    char32_t TupleStruct_takes_containing(somelib::capi::ContainingTuple c);

    } // extern "C"
} // namespace capi
} // namespace

inline int32_t somelib::TupleStruct::takes_st_as_tuple(somelib::TupleStruct a) {
    auto result = somelib::capi::TupleStruct_takes_st_as_tuple(a.AsFFI());
    return result;
}

inline char32_t somelib::TupleStruct::takes_containing(somelib::ContainingTuple c) {
    auto result = somelib::capi::TupleStruct_takes_containing(c.AsFFI());
    return result;
}


inline somelib::capi::TupleStruct somelib::TupleStruct::AsFFI() const {
    return somelib::capi::TupleStruct {
        /* .x = */ x,
        /* .y = */ y,
        /* .st = */ st.AsFFI(),
        /* .op = */ op->AsFFI(),
    };
}

inline somelib::TupleStruct somelib::TupleStruct::FromFFI(somelib::capi::TupleStruct c_struct) {
    return somelib::TupleStruct {
        /* .x = */ c_struct.x,
        /* .y = */ c_struct.y,
        /* .st = */ somelib::MyStruct::FromFFI(c_struct.st),
        /* .op = */ (somelib::Opaque*)somelib::Opaque::FromFFI(c_struct.op),
    };
}


#endif // SOMELIB_TupleStruct_HPP
