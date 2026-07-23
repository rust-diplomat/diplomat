#ifndef SOMELIB_Float64VecError_HPP
#define SOMELIB_Float64VecError_HPP

#include "Float64VecError.d.hpp"

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
    extern "C" {

    somelib::capi::Float64VecError* Float64VecError_new(somelib::diplomat::capi::DiplomatF64View v);

    typedef struct Float64VecError_get_result {union {double ok; }; bool is_ok;} Float64VecError_get_result;
    Float64VecError_get_result Float64VecError_get(const somelib::capi::Float64VecError* self, size_t i);

    void Float64VecError_destroy(Float64VecError* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::Float64VecError somelib::Float64VecError::new_(somelib::diplomat::span<const double> v) {
    auto result = somelib::capi::Float64VecError_new({v.data(), v.size()});
    return somelib::Float64VecError::FromFFI(result);
}

inline somelib::diplomat::result<double, std::monostate> somelib::Float64VecError::operator[](size_t i) const {
    auto result = somelib::capi::Float64VecError_get(this->AsFFI(),
        i);
    return result.is_ok ? somelib::diplomat::result<double, std::monostate>(somelib::diplomat::Ok<double>(result.ok)) : somelib::diplomat::result<double, std::monostate>(somelib::diplomat::Err<std::monostate>());
}


#endif // SOMELIB_Float64VecError_HPP
