#ifndef SOMELIB_ErrorStruct_D_HPP
#define SOMELIB_ErrorStruct_D_HPP

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
    struct ErrorStruct {
      int32_t i;
      int32_t j;
    };

    typedef struct ErrorStruct_option {union { ErrorStruct ok; }; bool is_ok; } ErrorStruct_option;
} // namespace capi
} // namespace


namespace somelib {
struct ErrorStruct {
    int32_t i;
    int32_t j;

    inline somelib::capi::ErrorStruct AsFFI() const;
    inline static somelib::ErrorStruct FromFFI(somelib::capi::ErrorStruct c_struct);
};

} // namespace
#endif // SOMELIB_ErrorStruct_D_HPP
