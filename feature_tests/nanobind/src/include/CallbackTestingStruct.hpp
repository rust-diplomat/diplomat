#ifndef SOMELIB_CallbackTestingStruct_HPP
#define SOMELIB_CallbackTestingStruct_HPP

#include "CallbackTestingStruct.d.hpp"

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

} // namespace capi
} // namespace


inline somelib::capi::CallbackTestingStruct somelib::CallbackTestingStruct::AsFFI() const {
    return somelib::capi::CallbackTestingStruct {
        /* .x = */ x,
        /* .y = */ y,
    };
}

inline somelib::CallbackTestingStruct somelib::CallbackTestingStruct::FromFFI(somelib::capi::CallbackTestingStruct c_struct) {
    return somelib::CallbackTestingStruct {
        /* .x = */ c_struct.x,
        /* .y = */ c_struct.y,
    };
}


#endif // SOMELIB_CallbackTestingStruct_HPP
