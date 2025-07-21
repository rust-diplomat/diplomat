#ifndef ns_RenamedVectorTest_D_HPP
#define ns_RenamedVectorTest_D_HPP

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
struct RenamedVectorTest;
}


namespace ns {
namespace capi {
    struct RenamedVectorTest {
      double test;
    };

    typedef struct RenamedVectorTest_option {union { RenamedVectorTest ok; }; bool is_ok; } RenamedVectorTest_option;
} // namespace capi
} // namespace


namespace ns {
struct RenamedVectorTest {
  double test;

  inline static ns::RenamedVectorTest new_();

  inline ns::capi::RenamedVectorTest AsFFI() const;
  inline static ns::RenamedVectorTest FromFFI(ns::capi::RenamedVectorTest c_struct);
};

} // namespace
#endif // ns_RenamedVectorTest_D_HPP
