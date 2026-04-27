#ifndef SOMELIB_ContainingTuple_HPP
#define SOMELIB_ContainingTuple_HPP

#include "ContainingTuple.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "TupleStruct.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {

} // namespace capi
} // namespace


inline somelib::capi::ContainingTuple somelib::ContainingTuple::AsFFI() const {
    return somelib::capi::ContainingTuple {
        /* .inner = */ inner.AsFFI(),
    };
}

inline somelib::ContainingTuple somelib::ContainingTuple::FromFFI(somelib::capi::ContainingTuple c_struct) {
    return somelib::ContainingTuple {
        /* .inner = */ somelib::TupleStruct::FromFFI(c_struct.inner),
    };
}


#endif // SOMELIB_ContainingTuple_HPP
