#ifndef ns_RenamedTestMacroStruct_HPP
#define ns_RenamedTestMacroStruct_HPP

#include "RenamedTestMacroStruct.d.hpp"

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
namespace capi {
    extern "C" {

    size_t namespace_TestMacroStruct_test_func(void);

    ns::capi::RenamedTestMacroStruct namespace_TestMacroStruct_test_meta(void);

    } // extern "C"
} // namespace capi
} // namespace

inline size_t ns::RenamedTestMacroStruct::test_func() {
  auto result = ns::capi::namespace_TestMacroStruct_test_func();
  return result;
}

inline ns::RenamedTestMacroStruct ns::RenamedTestMacroStruct::test_meta() {
  auto result = ns::capi::namespace_TestMacroStruct_test_meta();
  return ns::RenamedTestMacroStruct::FromFFI(result);
}


inline ns::capi::RenamedTestMacroStruct ns::RenamedTestMacroStruct::AsFFI() const {
  return ns::capi::RenamedTestMacroStruct {
    /* .a = */ a,
  };
}

inline ns::RenamedTestMacroStruct ns::RenamedTestMacroStruct::FromFFI(ns::capi::RenamedTestMacroStruct c_struct) {
  return ns::RenamedTestMacroStruct {
    /* .a = */ c_struct.a,
  };
}


#endif // ns_RenamedTestMacroStruct_HPP
