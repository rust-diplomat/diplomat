#ifndef SOMELIB_Two_HPP
#define SOMELIB_Two_HPP

#include "Two.d.hpp"

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

    void Two_destroy(Two* self);

    } // extern "C"
} // namespace capi
} // namespace


#endif // SOMELIB_Two_HPP
