#ifndef SOMELIB_Bar_D_HPP
#define SOMELIB_Bar_D_HPP

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
namespace capi { struct Foo; }
class Foo;
} // namespace somelib



namespace somelib {
namespace capi {
    struct Bar;
} // namespace capi
} // namespace

namespace somelib {
class Bar {
public:

  inline const somelib::Foo& foo() const;

    inline const somelib::capi::Bar* AsFFI() const;
    inline somelib::capi::Bar* AsFFI();
    inline static const somelib::Bar* FromFFI(const somelib::capi::Bar* ptr);
    inline static somelib::Bar* FromFFI(somelib::capi::Bar* ptr);
    inline static void operator delete(void* ptr);
private:
    Bar() = delete;
    Bar(const somelib::Bar&) = delete;
    Bar(somelib::Bar&&) noexcept = delete;
    Bar operator=(const somelib::Bar&) = delete;
    Bar operator=(somelib::Bar&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // SOMELIB_Bar_D_HPP
