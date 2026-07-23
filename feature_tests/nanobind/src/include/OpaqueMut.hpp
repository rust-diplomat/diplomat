#ifndef SOMELIB_OpaqueMut_HPP
#define SOMELIB_OpaqueMut_HPP

#include "OpaqueMut.d.hpp"

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

    somelib::capi::OpaqueMut* OpaqueMut_new(void);

    void OpaqueMut_destroy(OpaqueMut* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::OpaqueMut somelib::OpaqueMut::new_() {
    auto result = somelib::capi::OpaqueMut_new();
    return somelib::OpaqueMut::FromFFI(result);
}


#endif // SOMELIB_OpaqueMut_HPP
