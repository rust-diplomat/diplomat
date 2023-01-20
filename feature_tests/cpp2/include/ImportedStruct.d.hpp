#ifndef ImportedStruct_D_HPP
#define ImportedStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "UnimportedEnum.d.hpp"

enum struct UnimportedEnum;


struct ImportedStruct {
  UnimportedEnum foo;
  uint8_t count;

  inline capi::ImportedStruct AsFFI() const;
  inline static ImportedStruct FromFFI(capi::ImportedStruct c_struct);
};


#endif // ImportedStruct_D_HPP
