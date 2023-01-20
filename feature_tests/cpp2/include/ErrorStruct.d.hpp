#ifndef ErrorStruct_D_HPP
#define ErrorStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


struct ErrorStruct {
  int32_t i;
  int32_t j;

  inline capi::ErrorStruct AsFFI() const;
  inline static ErrorStruct FromFFI(capi::ErrorStruct ptr);
};


#endif // ErrorStruct_D_HPP
