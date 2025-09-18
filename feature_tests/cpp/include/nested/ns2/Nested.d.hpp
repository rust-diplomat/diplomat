#ifndef SOMELIB_nested_ns2_Nested_D_HPP
#define SOMELIB_nested_ns2_Nested_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../../diplomat_runtime.hpp"


namespace somelib::nested::ns2 {
namespace capi {
    struct Nested;
} // namespace capi
} // namespace

namespace somelib::nested::ns2 {
class Nested {
public:

    inline const somelib::nested::ns2::capi::Nested* AsFFI() const;
    inline somelib::nested::ns2::capi::Nested* AsFFI();
    inline static const somelib::nested::ns2::Nested* FromFFI(const somelib::nested::ns2::capi::Nested* ptr);
    inline static somelib::nested::ns2::Nested* FromFFI(somelib::nested::ns2::capi::Nested* ptr);
    inline static void operator delete(void* ptr);
private:
    Nested() = delete;
    Nested(const somelib::nested::ns2::Nested&) = delete;
    Nested(somelib::nested::ns2::Nested&&) noexcept = delete;
    Nested operator=(const somelib::nested::ns2::Nested&) = delete;
    Nested operator=(somelib::nested::ns2::Nested&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_nested_ns2_Nested_D_HPP
