#ifndef MyZst_HPP
#define MyZst_HPP

#include "MyZst.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    uint32_t MyZst_method(uint8_t foo);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline uint32_t MyZst::method(uint8_t foo) {
  auto result = diplomat::capi::MyZst_method(foo);
  return result;
}




#endif // MyZst_HPP
