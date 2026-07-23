#ifndef SOMELIB_ns_RenamedAttrOpaque2_HPP
#define SOMELIB_ns_RenamedAttrOpaque2_HPP

#include "RenamedAttrOpaque2.d.hpp"

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

    void namespace_AttrOpaque2_destroy(RenamedAttrOpaque2* self);

    } // extern "C"
} // namespace capi
} // namespace


#endif // SOMELIB_ns_RenamedAttrOpaque2_HPP
