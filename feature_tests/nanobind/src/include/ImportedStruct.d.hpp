#ifndef SOMELIB_ImportedStruct_D_HPP
#define SOMELIB_ImportedStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "UnimportedEnum.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
class UnimportedEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    struct ImportedStruct {
      somelib::capi::UnimportedEnum foo;
      uint8_t count;
    };

    typedef struct ImportedStruct_option {union { ImportedStruct ok; }; bool is_ok; } ImportedStruct_option;
} // namespace capi
} // namespace


namespace somelib {
struct ImportedStruct {
    somelib::UnimportedEnum foo;
    uint8_t count;

    inline somelib::capi::ImportedStruct AsFFI() const;
    inline static somelib::ImportedStruct FromFFI(somelib::capi::ImportedStruct c_struct);
};

} // namespace
#endif // SOMELIB_ImportedStruct_D_HPP
