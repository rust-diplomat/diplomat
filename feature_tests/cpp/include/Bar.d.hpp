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
#include "Foo.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct Foo; }
class Foo;
} // namespace somelib



namespace somelib {
namespace capi {
    struct Bar;
    extern "C" {
    void Bar_destroy(Bar* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class Bar;
using BarRef = somelib::diplomat::Ref<Bar, const somelib::capi::Bar>;
using BarRefMut = somelib::diplomat::Ref<Bar, somelib::capi::Bar>;

class Bar : public somelib::diplomat::OpaquePointer<Bar, somelib::capi::Bar, somelib::capi::Bar_destroy> {
public:

  inline somelib::FooRef foo() const DIPLOMAT_LIFETIME_BOUND;

};

} // namespace
#endif // SOMELIB_Bar_D_HPP
