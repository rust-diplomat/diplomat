#ifndef ErrorStruct_HPP
#define ErrorStruct_HPP

#include "ErrorStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ErrorStruct.h"



inline capi::ErrorStruct ErrorStruct::AsFFI() const {
  return capi::ErrorStruct {
    .i = i,
    .j = j,
  };
}

inline ErrorStruct ErrorStruct::FromFFI(capi::ErrorStruct c_struct) {
  return ErrorStruct {
    .i = c_struct.i,
    .j = c_struct.j,
  };
}


#endif // ErrorStruct_HPP
