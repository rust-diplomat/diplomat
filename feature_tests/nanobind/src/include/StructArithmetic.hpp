#ifndef SOMELIB_StructArithmetic_HPP
#define SOMELIB_StructArithmetic_HPP

#include "StructArithmetic.d.hpp"

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

    somelib::capi::StructArithmetic StructArithmetic_ORIGIN(void);

    void StructArithmetic_set_origin(somelib::capi::StructArithmetic _new_origin);

    somelib::capi::StructArithmetic StructArithmetic_new(int32_t x, int32_t y);

    somelib::capi::StructArithmetic StructArithmetic_add(somelib::capi::StructArithmetic self, somelib::capi::StructArithmetic o);

    somelib::capi::StructArithmetic StructArithmetic_sub(somelib::capi::StructArithmetic self, somelib::capi::StructArithmetic o);

    somelib::capi::StructArithmetic StructArithmetic_mul(somelib::capi::StructArithmetic self, somelib::capi::StructArithmetic o);

    somelib::capi::StructArithmetic StructArithmetic_div(somelib::capi::StructArithmetic self, somelib::capi::StructArithmetic o);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::StructArithmetic somelib::StructArithmetic::ORIGIN() {
    auto result = somelib::capi::StructArithmetic_ORIGIN();
    return somelib::StructArithmetic::FromFFI(result);
}

inline void somelib::StructArithmetic::set_origin(somelib::StructArithmetic _new_origin) {
    somelib::capi::StructArithmetic_set_origin(_new_origin.AsFFI());
}

inline somelib::StructArithmetic somelib::StructArithmetic::new_(int32_t x, int32_t y) {
    auto result = somelib::capi::StructArithmetic_new(x,
        y);
    return somelib::StructArithmetic::FromFFI(result);
}

inline somelib::StructArithmetic somelib::StructArithmetic::operator+(somelib::StructArithmetic o) const {
    auto result = somelib::capi::StructArithmetic_add(this->AsFFI(),
        o.AsFFI());
    return somelib::StructArithmetic::FromFFI(result);
}
inline somelib::StructArithmetic& somelib::StructArithmetic::operator+=(somelib::StructArithmetic o) {
  *this = *this + o;
  return *this;
}

inline somelib::StructArithmetic somelib::StructArithmetic::operator-(somelib::StructArithmetic o) const {
    auto result = somelib::capi::StructArithmetic_sub(this->AsFFI(),
        o.AsFFI());
    return somelib::StructArithmetic::FromFFI(result);
}
inline somelib::StructArithmetic& somelib::StructArithmetic::operator-=(somelib::StructArithmetic o) {
  *this = *this - o;
  return *this;
}

inline somelib::StructArithmetic somelib::StructArithmetic::operator*(somelib::StructArithmetic o) const {
    auto result = somelib::capi::StructArithmetic_mul(this->AsFFI(),
        o.AsFFI());
    return somelib::StructArithmetic::FromFFI(result);
}
inline somelib::StructArithmetic& somelib::StructArithmetic::operator*=(somelib::StructArithmetic o) {
  *this = *this * o;
  return *this;
}

inline somelib::StructArithmetic somelib::StructArithmetic::operator/(somelib::StructArithmetic o) const {
    auto result = somelib::capi::StructArithmetic_div(this->AsFFI(),
        o.AsFFI());
    return somelib::StructArithmetic::FromFFI(result);
}
inline somelib::StructArithmetic& somelib::StructArithmetic::operator/=(somelib::StructArithmetic o) {
  *this = *this / o;
  return *this;
}


inline somelib::capi::StructArithmetic somelib::StructArithmetic::AsFFI() const {
    return somelib::capi::StructArithmetic {
        /* .x = */ x,
        /* .y = */ y,
    };
}

inline somelib::StructArithmetic somelib::StructArithmetic::FromFFI(somelib::capi::StructArithmetic c_struct) {
    return somelib::StructArithmetic {
        /* .x = */ c_struct.x,
        /* .y = */ c_struct.y,
    };
}


#endif // SOMELIB_StructArithmetic_HPP
