#ifndef ErrorStruct_HPP
#define ErrorStruct_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ErrorStruct.h"


struct ErrorStruct {
 public:
  int32_t i;
  int32_t j;
};


#endif
