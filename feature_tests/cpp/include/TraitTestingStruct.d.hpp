#ifndef SOMELIB_TraitTestingStruct_D_HPP
#define SOMELIB_TraitTestingStruct_D_HPP

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
    struct TraitTestingStruct {
      int32_t x;
      int32_t y;
    };

    typedef struct TraitTestingStruct_option {union { TraitTestingStruct ok; }; bool is_ok; } TraitTestingStruct_option;
} // namespace capi
} // namespace


namespace somelib {
struct TraitTestingStruct {
    int32_t x;
    int32_t y;

    inline somelib::capi::TraitTestingStruct AsFFI() const;
    inline static somelib::TraitTestingStruct FromFFI(somelib::capi::TraitTestingStruct c_struct);
};

} // namespace
#endif // SOMELIB_TraitTestingStruct_D_HPP
