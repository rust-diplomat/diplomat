#ifndef ns_RenamedTestMacroStruct_D_HPP
#define ns_RenamedTestMacroStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace ns {
struct RenamedTestMacroStruct;
}


namespace ns {
namespace capi {
    struct RenamedTestMacroStruct {
      size_t a;
    };

    typedef struct RenamedTestMacroStruct_option {union { RenamedTestMacroStruct ok; }; bool is_ok; } RenamedTestMacroStruct_option;
} // namespace capi
} // namespace


namespace ns {
struct RenamedTestMacroStruct {
  size_t a;

  inline static size_t test_func();

  inline static ns::RenamedTestMacroStruct test_meta();

  inline ns::capi::RenamedTestMacroStruct AsFFI() const;
  inline static ns::RenamedTestMacroStruct FromFFI(ns::capi::RenamedTestMacroStruct c_struct);
};

} // namespace
#endif // ns_RenamedTestMacroStruct_D_HPP
