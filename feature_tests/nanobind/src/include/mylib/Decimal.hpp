#ifndef SOMELIB_mylib_Decimal_HPP
#define SOMELIB_mylib_Decimal_HPP

#include "Decimal.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::mylib {
namespace capi {
    extern "C" {

    somelib::mylib::capi::Decimal* Decimal_from_int32(int32_t _v);

    somelib::mylib::capi::Decimal* Decimal_from_int64(int64_t _v);

    somelib::mylib::capi::Decimal* Decimal_from_uint32(uint32_t _v);

    void Decimal_destroy(Decimal* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::mylib::Decimal> somelib::mylib::Decimal::from(int32_t _v) {
    auto result = somelib::mylib::capi::Decimal_from_int32(_v);
    return std::unique_ptr<somelib::mylib::Decimal>(somelib::mylib::Decimal::FromFFI(result));
}

inline std::unique_ptr<somelib::mylib::Decimal> somelib::mylib::Decimal::from(int64_t _v) {
    auto result = somelib::mylib::capi::Decimal_from_int64(_v);
    return std::unique_ptr<somelib::mylib::Decimal>(somelib::mylib::Decimal::FromFFI(result));
}

inline std::unique_ptr<somelib::mylib::Decimal> somelib::mylib::Decimal::from(uint32_t _v) {
    auto result = somelib::mylib::capi::Decimal_from_uint32(_v);
    return std::unique_ptr<somelib::mylib::Decimal>(somelib::mylib::Decimal::FromFFI(result));
}

inline const somelib::mylib::capi::Decimal* somelib::mylib::Decimal::AsFFI() const {
    return reinterpret_cast<const somelib::mylib::capi::Decimal*>(this);
}

inline somelib::mylib::capi::Decimal* somelib::mylib::Decimal::AsFFI() {
    return reinterpret_cast<somelib::mylib::capi::Decimal*>(this);
}

inline const somelib::mylib::Decimal* somelib::mylib::Decimal::FromFFI(const somelib::mylib::capi::Decimal* ptr) {
    return reinterpret_cast<const somelib::mylib::Decimal*>(ptr);
}

inline somelib::mylib::Decimal* somelib::mylib::Decimal::FromFFI(somelib::mylib::capi::Decimal* ptr) {
    return reinterpret_cast<somelib::mylib::Decimal*>(ptr);
}

inline void somelib::mylib::Decimal::operator delete(void* ptr) {
    somelib::mylib::capi::Decimal_destroy(reinterpret_cast<somelib::mylib::capi::Decimal*>(ptr));
}


#endif // SOMELIB_mylib_Decimal_HPP
