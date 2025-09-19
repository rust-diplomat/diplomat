#ifndef SOMELIB_MyStructContainingAnOption_HPP
#define SOMELIB_MyStructContainingAnOption_HPP

#include "MyStructContainingAnOption.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DefaultEnum.hpp"
#include "MyStruct.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::MyStructContainingAnOption MyStructContainingAnOption_new(void);

    somelib::capi::MyStructContainingAnOption MyStructContainingAnOption_filled(void);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::MyStructContainingAnOption somelib::MyStructContainingAnOption::new_() {
    auto result = somelib::capi::MyStructContainingAnOption_new();
    return somelib::MyStructContainingAnOption::FromFFI(result);
}

inline somelib::MyStructContainingAnOption somelib::MyStructContainingAnOption::filled() {
    auto result = somelib::capi::MyStructContainingAnOption_filled();
    return somelib::MyStructContainingAnOption::FromFFI(result);
}


inline somelib::capi::MyStructContainingAnOption somelib::MyStructContainingAnOption::AsFFI() const {
    return somelib::capi::MyStructContainingAnOption {
        /* .a = */ a.has_value() ? (somelib::capi::MyStruct_option{ { a.value().AsFFI() }, true }) : (somelib::capi::MyStruct_option{ {}, false }),
        /* .b = */ b.has_value() ? (somelib::capi::DefaultEnum_option{ { b.value().AsFFI() }, true }) : (somelib::capi::DefaultEnum_option{ {}, false }),
    };
}

inline somelib::MyStructContainingAnOption somelib::MyStructContainingAnOption::FromFFI(somelib::capi::MyStructContainingAnOption c_struct) {
    return somelib::MyStructContainingAnOption {
        /* .a = */ c_struct.a.is_ok ? std::optional(somelib::MyStruct::FromFFI(c_struct.a.ok)) : std::nullopt,
        /* .b = */ c_struct.b.is_ok ? std::optional(somelib::DefaultEnum::FromFFI(c_struct.b.ok)) : std::nullopt,
    };
}


#endif // SOMELIB_MyStructContainingAnOption_HPP
