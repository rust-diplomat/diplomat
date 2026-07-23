#ifndef SOMELIB_RefListParameter_HPP
#define SOMELIB_RefListParameter_HPP

#include "RefListParameter.d.hpp"

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

    void RefListParameter_destroy(RefListParameter* self);

    } // extern "C"
} // namespace capi
} // namespace


#endif // SOMELIB_RefListParameter_HPP
