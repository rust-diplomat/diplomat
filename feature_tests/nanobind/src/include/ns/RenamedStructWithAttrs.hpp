#ifndef SOMELIB_ns_RenamedStructWithAttrs_HPP
#define SOMELIB_ns_RenamedStructWithAttrs_HPP

#include "RenamedStructWithAttrs.d.hpp"

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

    typedef struct namespace_StructWithAttrs_new_fallible_result {union {somelib::ns::capi::RenamedStructWithAttrs ok; }; bool is_ok;} namespace_StructWithAttrs_new_fallible_result;
    namespace_StructWithAttrs_new_fallible_result namespace_StructWithAttrs_new_fallible(bool a, uint32_t b);

    uint32_t namespace_StructWithAttrs_c(somelib::ns::capi::RenamedStructWithAttrs self);

    void namespace_StructWithAttrs_deprecated(somelib::ns::capi::RenamedStructWithAttrs self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::result<somelib::ns::RenamedStructWithAttrs, std::monostate> somelib::ns::RenamedStructWithAttrs::new_fallible(bool a, uint32_t b) {
    auto result = somelib::ns::capi::namespace_StructWithAttrs_new_fallible(a,
        b);
    return result.is_ok ? somelib::diplomat::result<somelib::ns::RenamedStructWithAttrs, std::monostate>(somelib::diplomat::Ok<somelib::ns::RenamedStructWithAttrs>(somelib::ns::RenamedStructWithAttrs::FromFFI(result.ok))) : somelib::diplomat::result<somelib::ns::RenamedStructWithAttrs, std::monostate>(somelib::diplomat::Err<std::monostate>());
}

inline uint32_t somelib::ns::RenamedStructWithAttrs::c() const {
    auto result = somelib::ns::capi::namespace_StructWithAttrs_c(this->AsFFI());
    return result;
}

inline void somelib::ns::RenamedStructWithAttrs::deprecated() const {
    somelib::ns::capi::namespace_StructWithAttrs_deprecated(this->AsFFI());
}


inline somelib::ns::capi::RenamedStructWithAttrs somelib::ns::RenamedStructWithAttrs::AsFFI() const {
    return somelib::ns::capi::RenamedStructWithAttrs {
        /* .a = */ a,
        /* .b = */ b,
    };
}

inline somelib::ns::RenamedStructWithAttrs somelib::ns::RenamedStructWithAttrs::FromFFI(somelib::ns::capi::RenamedStructWithAttrs c_struct) {
    return somelib::ns::RenamedStructWithAttrs {
        /* .a = */ c_struct.a,
        /* .b = */ c_struct.b,
    };
}


#endif // SOMELIB_ns_RenamedStructWithAttrs_HPP
