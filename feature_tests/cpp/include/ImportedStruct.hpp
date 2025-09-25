#ifndef SOMELIB_ImportedStruct_HPP
#define SOMELIB_ImportedStruct_HPP

#include "ImportedStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "UnimportedEnum.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {

} // namespace capi
} // namespace


inline somelib::capi::ImportedStruct somelib::ImportedStruct::AsFFI() const {
    return somelib::capi::ImportedStruct {
        /* .foo = */ foo.AsFFI(),
        /* .count = */ count,
    };
}

inline somelib::ImportedStruct somelib::ImportedStruct::FromFFI(somelib::capi::ImportedStruct c_struct) {
    return somelib::ImportedStruct {
        /* .foo = */ somelib::UnimportedEnum::FromFFI(c_struct.foo),
        /* .count = */ c_struct.count,
    };
}


#endif // SOMELIB_ImportedStruct_HPP
