#ifndef MyEnum_HPP
#define MyEnum_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "MyEnum.h"


enum struct MyEnum {
  A = -2,
  B = -1,
  C = 0,
  D = 1,
  E = 2,
  F = 3,
};

#endif
