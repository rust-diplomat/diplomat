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

inline somelib::FooRef somelib::Bar::foo() const DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::Bar_foo(this->AsFFI());
    return somelib::FooRef::FromFFI(result);
}


#endif // SOMELIB_Bar_HPP
