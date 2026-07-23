#ifndef SOMELIB_ns_RenamedBlockOverride_HPP
#define SOMELIB_ns_RenamedBlockOverride_HPP

#include "RenamedBlockOverride.d.hpp"

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

    void namespace_BlockOverride_destroy(RenamedBlockOverride* self);

    } // extern "C"
} // namespace capi
} // namespace

//Test!
std::string somelib::ns::RenamedBlockOverride::special_function() {
    return "This is a custom binding.";
}

#endif // SOMELIB_ns_RenamedBlockOverride_HPP
