#ifndef Float64VecError_HPP
#define Float64VecError_HPP

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


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::Float64VecError* Float64VecError_new(diplomat::capi::DiplomatF64View v);

    typedef struct Float64VecError_get_result {union {double ok; }; bool is_ok;} Float64VecError_get_result;
    Float64VecError_get_result Float64VecError_get(const diplomat::capi::Float64VecError* self, size_t i);

    void Float64VecError_destroy(Float64VecError* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<Float64VecError> Float64VecError::new_(diplomat::span<const double> v) {
    auto result = diplomat::capi::Float64VecError_new({v.data(), v.size()});
    return std::unique_ptr<Float64VecError>(Float64VecError::FromFFI(result));
}

inline diplomat::result<double, std::monostate> Float64VecError::operator[](size_t i) const {
    auto result = diplomat::capi::Float64VecError_get(this->AsFFI(),
        i);
    return result.is_ok ? diplomat::result<double, std::monostate>(diplomat::Ok<double>(result.ok)) : diplomat::result<double, std::monostate>(diplomat::Err<std::monostate>());
}

inline const diplomat::capi::Float64VecError* Float64VecError::AsFFI() const {
    return reinterpret_cast<const diplomat::capi::Float64VecError*>(this);
}

inline diplomat::capi::Float64VecError* Float64VecError::AsFFI() {
    return reinterpret_cast<diplomat::capi::Float64VecError*>(this);
}

inline const Float64VecError* Float64VecError::FromFFI(const diplomat::capi::Float64VecError* ptr) {
    return reinterpret_cast<const Float64VecError*>(ptr);
}

inline Float64VecError* Float64VecError::FromFFI(diplomat::capi::Float64VecError* ptr) {
    return reinterpret_cast<Float64VecError*>(ptr);
}

inline void Float64VecError::operator delete(void* ptr) {
    diplomat::capi::Float64VecError_destroy(reinterpret_cast<diplomat::capi::Float64VecError*>(ptr));
}


#endif // Float64VecError_HPP
