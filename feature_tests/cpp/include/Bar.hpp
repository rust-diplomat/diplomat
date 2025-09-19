#ifndef SOMELIB_Bar_HPP
#define SOMELIB_Bar_HPP

#include "Bar.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "Foo.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    const somelib::capi::Foo* Bar_foo(const somelib::capi::Bar* self);

    void Bar_destroy(Bar* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::Foo& somelib::Bar::foo() const {
    auto result = somelib::capi::Bar_foo(this->AsFFI());
    return *somelib::Foo::FromFFI(result);
}

inline const somelib::capi::Bar* somelib::Bar::AsFFI() const {
    return reinterpret_cast<const somelib::capi::Bar*>(this);
}

inline somelib::capi::Bar* somelib::Bar::AsFFI() {
    return reinterpret_cast<somelib::capi::Bar*>(this);
}

inline const somelib::Bar* somelib::Bar::FromFFI(const somelib::capi::Bar* ptr) {
    return reinterpret_cast<const somelib::Bar*>(ptr);
}

inline somelib::Bar* somelib::Bar::FromFFI(somelib::capi::Bar* ptr) {
    return reinterpret_cast<somelib::Bar*>(ptr);
}

inline void somelib::Bar::operator delete(void* ptr) {
    somelib::capi::Bar_destroy(reinterpret_cast<somelib::capi::Bar*>(ptr));
}


#endif // SOMELIB_Bar_HPP
