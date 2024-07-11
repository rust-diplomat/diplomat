#ifndef ErrorStruct_D_HPP
#define ErrorStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef struct ErrorStruct {
      int32_t i;
      int32_t j;
    } ErrorStruct;
} // namespace capi
} // namespace


struct ErrorStruct {
  int32_t i;
  int32_t j;

  inline diplomat::capi::ErrorStruct AsFFI() const;
  inline static ErrorStruct FromFFI(diplomat::capi::ErrorStruct c_struct);
};


#endif // ErrorStruct_D_HPP
