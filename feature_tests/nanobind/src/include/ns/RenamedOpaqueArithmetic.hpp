#ifndef SOMELIB_ns_RenamedOpaqueArithmetic_HPP
#define SOMELIB_ns_RenamedOpaqueArithmetic_HPP

#include "RenamedOpaqueArithmetic.d.hpp"

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

    somelib::ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_make(int32_t x, int32_t y);

    somelib::ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_make_overload(float x, float y);

    somelib::ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_make_overload_rename_arg(float x, bool z);

    int32_t namespace_OpaqueArithmetic_x(const somelib::ns::capi::RenamedOpaqueArithmetic* self);

    int32_t namespace_OpaqueArithmetic_y(const somelib::ns::capi::RenamedOpaqueArithmetic* self);

    somelib::ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_add(const somelib::ns::capi::RenamedOpaqueArithmetic* self, const somelib::ns::capi::RenamedOpaqueArithmetic* o);

    somelib::ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_sub(const somelib::ns::capi::RenamedOpaqueArithmetic* self, const somelib::ns::capi::RenamedOpaqueArithmetic* o);

    somelib::ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_mul(const somelib::ns::capi::RenamedOpaqueArithmetic* self, const somelib::ns::capi::RenamedOpaqueArithmetic* o);

    somelib::ns::capi::RenamedOpaqueArithmetic* namespace_OpaqueArithmetic_div(const somelib::ns::capi::RenamedOpaqueArithmetic* self, const somelib::ns::capi::RenamedOpaqueArithmetic* o);

    void namespace_OpaqueArithmetic_addassign(somelib::ns::capi::RenamedOpaqueArithmetic* self, const somelib::ns::capi::RenamedOpaqueArithmetic* o);

    void namespace_OpaqueArithmetic_subassign(somelib::ns::capi::RenamedOpaqueArithmetic* self, const somelib::ns::capi::RenamedOpaqueArithmetic* o);

    void namespace_OpaqueArithmetic_mulassign(somelib::ns::capi::RenamedOpaqueArithmetic* self, const somelib::ns::capi::RenamedOpaqueArithmetic* o);

    void namespace_OpaqueArithmetic_divassign(somelib::ns::capi::RenamedOpaqueArithmetic* self, const somelib::ns::capi::RenamedOpaqueArithmetic* o);

    void namespace_OpaqueArithmetic_destroy(RenamedOpaqueArithmetic* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> somelib::ns::RenamedOpaqueArithmetic::make(int32_t x, int32_t y) {
    auto result = somelib::ns::capi::namespace_OpaqueArithmetic_make(x,
        y);
    return std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic>(somelib::ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> somelib::ns::RenamedOpaqueArithmetic::make(float x, float y) {
    auto result = somelib::ns::capi::namespace_OpaqueArithmetic_make_overload(x,
        y);
    return std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic>(somelib::ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> somelib::ns::RenamedOpaqueArithmetic::make(float x, bool z) {
    auto result = somelib::ns::capi::namespace_OpaqueArithmetic_make_overload_rename_arg(x,
        z);
    return std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic>(somelib::ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline int32_t somelib::ns::RenamedOpaqueArithmetic::x() const {
    auto result = somelib::ns::capi::namespace_OpaqueArithmetic_x(this->AsFFI());
    return result;
}

inline int32_t somelib::ns::RenamedOpaqueArithmetic::y() const {
    auto result = somelib::ns::capi::namespace_OpaqueArithmetic_y(this->AsFFI());
    return result;
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> somelib::ns::RenamedOpaqueArithmetic::operator+(const somelib::ns::RenamedOpaqueArithmetic& o) const {
    auto result = somelib::ns::capi::namespace_OpaqueArithmetic_add(this->AsFFI(),
        o.AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic>(somelib::ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> somelib::ns::RenamedOpaqueArithmetic::operator-(const somelib::ns::RenamedOpaqueArithmetic& o) const {
    auto result = somelib::ns::capi::namespace_OpaqueArithmetic_sub(this->AsFFI(),
        o.AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic>(somelib::ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> somelib::ns::RenamedOpaqueArithmetic::operator*(const somelib::ns::RenamedOpaqueArithmetic& o) const {
    auto result = somelib::ns::capi::namespace_OpaqueArithmetic_mul(this->AsFFI(),
        o.AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic>(somelib::ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic> somelib::ns::RenamedOpaqueArithmetic::operator/(const somelib::ns::RenamedOpaqueArithmetic& o) const {
    auto result = somelib::ns::capi::namespace_OpaqueArithmetic_div(this->AsFFI(),
        o.AsFFI());
    return std::unique_ptr<somelib::ns::RenamedOpaqueArithmetic>(somelib::ns::RenamedOpaqueArithmetic::FromFFI(result));
}

inline void somelib::ns::RenamedOpaqueArithmetic::operator+=(const somelib::ns::RenamedOpaqueArithmetic& o) {
    somelib::ns::capi::namespace_OpaqueArithmetic_addassign(this->AsFFI(),
        o.AsFFI());
}

inline void somelib::ns::RenamedOpaqueArithmetic::operator-=(const somelib::ns::RenamedOpaqueArithmetic& o) {
    somelib::ns::capi::namespace_OpaqueArithmetic_subassign(this->AsFFI(),
        o.AsFFI());
}

inline void somelib::ns::RenamedOpaqueArithmetic::operator*=(const somelib::ns::RenamedOpaqueArithmetic& o) {
    somelib::ns::capi::namespace_OpaqueArithmetic_mulassign(this->AsFFI(),
        o.AsFFI());
}

inline void somelib::ns::RenamedOpaqueArithmetic::operator/=(const somelib::ns::RenamedOpaqueArithmetic& o) {
    somelib::ns::capi::namespace_OpaqueArithmetic_divassign(this->AsFFI(),
        o.AsFFI());
}

inline const somelib::ns::capi::RenamedOpaqueArithmetic* somelib::ns::RenamedOpaqueArithmetic::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedOpaqueArithmetic*>(this);
}

inline somelib::ns::capi::RenamedOpaqueArithmetic* somelib::ns::RenamedOpaqueArithmetic::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedOpaqueArithmetic*>(this);
}

inline const somelib::ns::RenamedOpaqueArithmetic* somelib::ns::RenamedOpaqueArithmetic::FromFFI(const somelib::ns::capi::RenamedOpaqueArithmetic* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedOpaqueArithmetic*>(ptr);
}

inline somelib::ns::RenamedOpaqueArithmetic* somelib::ns::RenamedOpaqueArithmetic::FromFFI(somelib::ns::capi::RenamedOpaqueArithmetic* ptr) {
    return reinterpret_cast<somelib::ns::RenamedOpaqueArithmetic*>(ptr);
}

inline void somelib::ns::RenamedOpaqueArithmetic::operator delete(void* ptr) {
    somelib::ns::capi::namespace_OpaqueArithmetic_destroy(reinterpret_cast<somelib::ns::capi::RenamedOpaqueArithmetic*>(ptr));
}


#endif // SOMELIB_ns_RenamedOpaqueArithmetic_HPP
