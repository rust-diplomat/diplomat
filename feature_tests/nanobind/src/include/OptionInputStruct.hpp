#ifndef SOMELIB_OptionInputStruct_HPP
#define SOMELIB_OptionInputStruct_HPP

#include "OptionInputStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "OptionEnum.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {

} // namespace capi
} // namespace


inline somelib::capi::OptionInputStruct somelib::OptionInputStruct::AsFFI() const {
    return somelib::capi::OptionInputStruct {
        /* .a = */ a.has_value() ? (somelib::diplomat::capi::OptionU8{ { a.value() }, true }) : (somelib::diplomat::capi::OptionU8{ {}, false }),
        /* .b = */ b.has_value() ? (somelib::diplomat::capi::OptionChar{ { b.value() }, true }) : (somelib::diplomat::capi::OptionChar{ {}, false }),
        /* .c = */ c.has_value() ? (somelib::capi::OptionEnum_option{ { c.value().AsFFI() }, true }) : (somelib::capi::OptionEnum_option{ {}, false }),
    };
}

inline somelib::OptionInputStruct somelib::OptionInputStruct::FromFFI(somelib::capi::OptionInputStruct c_struct) {
    return somelib::OptionInputStruct {
        /* .a = */ c_struct.a.is_ok ? std::optional(c_struct.a.ok) : std::nullopt,
        /* .b = */ c_struct.b.is_ok ? std::optional(c_struct.b.ok) : std::nullopt,
        /* .c = */ c_struct.c.is_ok ? std::optional(somelib::OptionEnum::FromFFI(c_struct.c.ok)) : std::nullopt,
    };
}


#endif // SOMELIB_OptionInputStruct_HPP
