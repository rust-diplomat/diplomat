#ifndef ns_RenamedVectorTest_HPP
#define ns_RenamedVectorTest_HPP

#include "RenamedVectorTest.d.hpp"

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

    ns::capi::RenamedVectorTest namespace_VectorTest_new(void);

    } // extern "C"
} // namespace capi
} // namespace

inline ns::RenamedVectorTest ns::RenamedVectorTest::new_() {
  auto result = ns::capi::namespace_VectorTest_new();
  return ns::RenamedVectorTest::FromFFI(result);
}


inline ns::capi::RenamedVectorTest ns::RenamedVectorTest::AsFFI() const {
  return ns::capi::RenamedVectorTest {
    /* .test = */ test,
  };
}

inline ns::RenamedVectorTest ns::RenamedVectorTest::FromFFI(ns::capi::RenamedVectorTest c_struct) {
  return ns::RenamedVectorTest {
    /* .test = */ c_struct.test,
  };
}


#endif // ns_RenamedVectorTest_HPP
