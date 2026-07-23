#ifndef SOMELIB_nested_ns2_Nested_HPP
#define SOMELIB_nested_ns2_Nested_HPP

#include "Nested.d.hpp"

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
    extern "C" {

    void namespace_Nested2_destroy(Nested* self);

    } // extern "C"
} // namespace capi
} // namespace


#endif // SOMELIB_nested_ns2_Nested_HPP
