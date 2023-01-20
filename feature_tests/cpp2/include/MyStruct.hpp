#ifndef MyStruct_HPP
#define MyStruct_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "MyStruct.h"

#include "MyStruct.d.hpp"


inline MyStruct MyStruct::new_() {
  auto result = capi::MyStruct_new();
  return MyStruct::FromFFI(result);
}


#endif // MyStruct_HPP
