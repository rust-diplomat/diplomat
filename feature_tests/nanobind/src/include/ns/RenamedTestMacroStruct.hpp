#ifndef SOMELIB_ns_RenamedTestMacroStruct_HPP
#define SOMELIB_ns_RenamedTestMacroStruct_HPP

#include "RenamedTestMacroStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    extern "C" {

    size_t namespace_TestMacroStruct_test_func(void);

    somelib::ns::capi::RenamedTestMacroStruct namespace_TestMacroStruct_test_meta(void);

    } // extern "C"
} // namespace capi
} // namespace

inline size_t somelib::ns::RenamedTestMacroStruct::test_func() {
    auto result = somelib::ns::capi::namespace_TestMacroStruct_test_func();
    return result;
}

inline somelib::ns::RenamedTestMacroStruct somelib::ns::RenamedTestMacroStruct::test_meta() {
    auto result = somelib::ns::capi::namespace_TestMacroStruct_test_meta();
    return somelib::ns::RenamedTestMacroStruct::FromFFI(result);
}


inline somelib::ns::capi::RenamedTestMacroStruct somelib::ns::RenamedTestMacroStruct::AsFFI() const {
    return somelib::ns::capi::RenamedTestMacroStruct {
        /* .a = */ a,
    };
}

inline somelib::ns::RenamedTestMacroStruct somelib::ns::RenamedTestMacroStruct::FromFFI(somelib::ns::capi::RenamedTestMacroStruct c_struct) {
    return somelib::ns::RenamedTestMacroStruct {
        /* .a = */ c_struct.a,
    };
}


#endif // SOMELIB_ns_RenamedTestMacroStruct_HPP
