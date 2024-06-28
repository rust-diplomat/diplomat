#ifndef ImportedStruct_HPP
#define ImportedStruct_HPP

#include "ImportedStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "UnimportedEnum.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}

inline ::capi::ImportedStruct ImportedStruct::AsFFI() const {
  return ::capi::ImportedStruct {
    .foo = foo.AsFFI(),
    .count = count,
  };
}

inline ImportedStruct ImportedStruct::FromFFI(::capi::ImportedStruct c_struct) {
  return ImportedStruct {
    .foo = UnimportedEnum::FromFFI(c_struct.foo),
    .count = c_struct.count,
  };
}


#endif // ImportedStruct_HPP
