#ifndef SOMELIB_CallbackTestingStruct_D_HPP
#define SOMELIB_CallbackTestingStruct_D_HPP

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
    struct CallbackTestingStruct {
      int32_t x;
      int32_t y;
    };

    typedef struct CallbackTestingStruct_option {union { CallbackTestingStruct ok; }; bool is_ok; } CallbackTestingStruct_option;
} // namespace capi
} // namespace


namespace somelib {
struct CallbackTestingStruct {
    int32_t x;
    int32_t y;

    inline somelib::capi::CallbackTestingStruct AsFFI() const;
    inline static somelib::CallbackTestingStruct FromFFI(somelib::capi::CallbackTestingStruct c_struct);
};

} // namespace
#endif // SOMELIB_CallbackTestingStruct_D_HPP
