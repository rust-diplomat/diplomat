#ifndef SOMELIB_CyclicStructB_HPP
#define SOMELIB_CyclicStructB_HPP

#include "CyclicStructB.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "CyclicStructA.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::CyclicStructA CyclicStructB_get_a(void);

    typedef struct CyclicStructB_get_a_option_result {union {somelib::capi::CyclicStructA ok; }; bool is_ok;} CyclicStructB_get_a_option_result;
    CyclicStructB_get_a_option_result CyclicStructB_get_a_option(void);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::CyclicStructA somelib::CyclicStructB::get_a() {
    auto result = somelib::capi::CyclicStructB_get_a();
    return somelib::CyclicStructA::FromFFI(result);
}

inline std::optional<somelib::CyclicStructA> somelib::CyclicStructB::get_a_option() {
    auto result = somelib::capi::CyclicStructB_get_a_option();
    return result.is_ok ? std::optional<somelib::CyclicStructA>(somelib::CyclicStructA::FromFFI(result.ok)) : std::nullopt;
}


inline somelib::capi::CyclicStructB somelib::CyclicStructB::AsFFI() const {
    return somelib::capi::CyclicStructB {
        /* .field = */ field,
    };
}

inline somelib::CyclicStructB somelib::CyclicStructB::FromFFI(somelib::capi::CyclicStructB c_struct) {
    return somelib::CyclicStructB {
        /* .field = */ c_struct.field,
    };
}


#endif // SOMELIB_CyclicStructB_HPP
