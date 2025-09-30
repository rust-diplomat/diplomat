#ifndef SOMELIB_OptionInputStruct_D_HPP
#define SOMELIB_OptionInputStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "OptionEnum.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
class OptionEnum;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OptionInputStruct {
      somelib::diplomat::capi::OptionU8 a;
      somelib::diplomat::capi::OptionChar b;
      somelib::capi::OptionEnum_option c;
    };

    typedef struct OptionInputStruct_option {union { OptionInputStruct ok; }; bool is_ok; } OptionInputStruct_option;
} // namespace capi
} // namespace


namespace somelib {
struct OptionInputStruct {
    std::optional<uint8_t> a;
    std::optional<char32_t> b;
    std::optional<somelib::OptionEnum> c;

    inline somelib::capi::OptionInputStruct AsFFI() const;
    inline static somelib::OptionInputStruct FromFFI(somelib::capi::OptionInputStruct c_struct);
};

} // namespace
#endif // SOMELIB_OptionInputStruct_D_HPP
