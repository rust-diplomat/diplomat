#ifndef ContiguousEnum_HPP
#define ContiguousEnum_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ContiguousEnum.h"


enum struct ContiguousEnum {
  C = 0,
  D = 1,
  E = 2,
  F = 3,
};

#endif
