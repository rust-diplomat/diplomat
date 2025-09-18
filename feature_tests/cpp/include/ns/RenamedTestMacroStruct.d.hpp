#ifndef SOMELIB_ns_RenamedTestMacroStruct_D_HPP
#define SOMELIB_ns_RenamedTestMacroStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
namespace somelib {
namespace ns {
struct RenamedTestMacroStruct;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedTestMacroStruct {
      size_t a;
    };

    typedef struct RenamedTestMacroStruct_option {union { RenamedTestMacroStruct ok; }; bool is_ok; } RenamedTestMacroStruct_option;
} // namespace capi
} // namespace


namespace somelib::ns {
struct RenamedTestMacroStruct {
    size_t a;

  inline static size_t test_func();

  inline static somelib::ns::RenamedTestMacroStruct test_meta();

    inline somelib::ns::capi::RenamedTestMacroStruct AsFFI() const;
    inline static somelib::ns::RenamedTestMacroStruct FromFFI(somelib::ns::capi::RenamedTestMacroStruct c_struct);
};

} // namespace
#endif // SOMELIB_ns_RenamedTestMacroStruct_D_HPP
