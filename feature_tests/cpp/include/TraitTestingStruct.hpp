#ifndef SOMELIB_TraitTestingStruct_HPP
#define SOMELIB_TraitTestingStruct_HPP

#include "TraitTestingStruct.d.hpp"

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


inline somelib::capi::TraitTestingStruct somelib::TraitTestingStruct::AsFFI() const {
    return somelib::capi::TraitTestingStruct {
        /* .x = */ x,
        /* .y = */ y,
    };
}

inline somelib::TraitTestingStruct somelib::TraitTestingStruct::FromFFI(somelib::capi::TraitTestingStruct c_struct) {
    return somelib::TraitTestingStruct {
        /* .x = */ c_struct.x,
        /* .y = */ c_struct.y,
    };
}


#endif // SOMELIB_TraitTestingStruct_HPP
