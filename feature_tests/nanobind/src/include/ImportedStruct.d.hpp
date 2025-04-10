#ifndef ImportedStruct_D_HPP
#define ImportedStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "UnimportedEnum.d.hpp"
#include "diplomat_runtime.hpp"

class UnimportedEnum;


namespace diplomat {
namespace capi {
    struct ImportedStruct {
      diplomat::capi::UnimportedEnum foo;
      uint8_t count;
    };
    
    typedef struct ImportedStruct_option {union { ImportedStruct ok; }; bool is_ok; } ImportedStruct_option;
} // namespace capi
} // namespace


struct ImportedStruct {
  UnimportedEnum foo;
  uint8_t count;

  inline diplomat::capi::ImportedStruct AsFFI() const;
  inline static ImportedStruct FromFFI(diplomat::capi::ImportedStruct c_struct);
};


#endif // ImportedStruct_D_HPP
