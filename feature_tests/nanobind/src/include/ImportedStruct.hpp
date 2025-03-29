#ifndef ImportedStruct_HPP
#define ImportedStruct_HPP

#include "ImportedStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "UnimportedEnum.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::ImportedStruct ImportedStruct::AsFFI() const {
  return diplomat::capi::ImportedStruct {
    /* .foo = */ foo.AsFFI(),
    /* .count = */ count,
  };
}

inline ImportedStruct ImportedStruct::FromFFI(diplomat::capi::ImportedStruct c_struct) {
  return ImportedStruct {
    /* .foo = */ UnimportedEnum::FromFFI(c_struct.foo),
    /* .count = */ c_struct.count,
  };
}


#endif // ImportedStruct_HPP
