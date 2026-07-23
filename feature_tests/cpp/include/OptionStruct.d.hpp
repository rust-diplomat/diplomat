#ifndef SOMELIB_OptionStruct_D_HPP
#define SOMELIB_OptionStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "OptionOpaque.d.hpp"
#include "OptionOpaqueChar.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct OptionOpaque; }
class OptionOpaque;
namespace capi { struct OptionOpaqueChar; }
class OptionOpaqueChar;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OptionStruct {
      somelib::capi::OptionOpaque* a;
      somelib::capi::OptionOpaqueChar* b;
      uint32_t c;
      somelib::capi::OptionOpaque* d;
    };

    typedef struct OptionStruct_option {union { OptionStruct ok; }; bool is_ok; } OptionStruct_option;
} // namespace capi
} // namespace


namespace somelib {
struct OptionStruct {
    somelib::diplomat::Optional<somelib::OptionOpaque> a;
    somelib::diplomat::Optional<somelib::OptionOpaqueChar> b;
    uint32_t c;
    somelib::OptionOpaque d;

    inline somelib::capi::OptionStruct AsFFI() const;
    inline static somelib::OptionStruct FromFFI(somelib::capi::OptionStruct c_struct);
};

} // namespace
#endif // SOMELIB_OptionStruct_D_HPP
