#ifndef SOMELIB_OptionStruct_HPP
#define SOMELIB_OptionStruct_HPP

#include "OptionStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "OptionOpaque.hpp"
#include "OptionOpaqueChar.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {

} // namespace capi
} // namespace


inline somelib::capi::OptionStruct somelib::OptionStruct::AsFFI() const {
    return somelib::capi::OptionStruct {
        /* .a = */ a ? a->AsFFI() : nullptr,
        /* .b = */ b ? b->AsFFI() : nullptr,
        /* .c = */ c,
        /* .d = */ d->AsFFI(),
    };
}

inline somelib::OptionStruct somelib::OptionStruct::FromFFI(somelib::capi::OptionStruct c_struct) {
    return somelib::OptionStruct {
        /* .a = */ std::unique_ptr<somelib::OptionOpaque>(somelib::OptionOpaque::FromFFI(c_struct.a)),
        /* .b = */ std::unique_ptr<somelib::OptionOpaqueChar>(somelib::OptionOpaqueChar::FromFFI(c_struct.b)),
        /* .c = */ c_struct.c,
        /* .d = */ std::unique_ptr<somelib::OptionOpaque>(somelib::OptionOpaque::FromFFI(c_struct.d)),
    };
}


#endif // SOMELIB_OptionStruct_HPP
