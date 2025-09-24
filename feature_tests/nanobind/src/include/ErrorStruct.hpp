#ifndef ErrorStruct_HPP
#define ErrorStruct_HPP

#include "ErrorStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {

} // namespace capi
} // namespace


inline diplomat::capi::ErrorStruct ErrorStruct::AsFFI() const {
    return diplomat::capi::ErrorStruct {
        /* .i = */ i,
        /* .j = */ j,
    };
}

inline ErrorStruct ErrorStruct::FromFFI(diplomat::capi::ErrorStruct c_struct) {
    return ErrorStruct {
        /* .i = */ c_struct.i,
        /* .j = */ c_struct.j,
    };
}


#endif // ErrorStruct_HPP
