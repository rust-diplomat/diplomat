#ifndef ImportedStruct_HPP
#define ImportedStruct_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ImportedStruct.h"

#include "UnimportedEnum.hpp"

struct ImportedStruct {
 public:
  UnimportedEnum foo;
  uint8_t count;
};


#endif
