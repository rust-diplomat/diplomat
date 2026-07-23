#ifndef SOMELIB_Two_D_HPP
#define SOMELIB_Two_D_HPP

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
    struct Two;
    extern "C" {
    void Two_destroy(Two* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class Two;
using TwoRef = somelib::diplomat::Ref<Two, const somelib::capi::Two>;
using TwoRefMut = somelib::diplomat::Ref<Two, somelib::capi::Two>;

class Two : public somelib::diplomat::OpaquePointer<Two, somelib::capi::Two, somelib::capi::Two_destroy> {
public:

};

} // namespace
#endif // SOMELIB_Two_D_HPP
