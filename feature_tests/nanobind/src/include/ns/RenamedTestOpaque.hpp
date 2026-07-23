#ifndef SOMELIB_ns_RenamedTestOpaque_HPP
#define SOMELIB_ns_RenamedTestOpaque_HPP

#include "RenamedTestOpaque.d.hpp"

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

    void namespace_TestOpaque_destroy(RenamedTestOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace


#endif // SOMELIB_ns_RenamedTestOpaque_HPP
