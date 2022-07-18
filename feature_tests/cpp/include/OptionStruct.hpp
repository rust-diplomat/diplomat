#ifndef OptionStruct_HPP
#define OptionStruct_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "OptionStruct.h"

class OptionOpaque;
class OptionOpaqueChar;

/**
 * A destruction policy for using OptionStruct with std::unique_ptr.
 */
struct OptionStructDeleter {
  void operator()(capi::OptionStruct* l) const noexcept {
    capi::OptionStruct_destroy(l);
  }
};
struct OptionStruct {
 public:
  std::optional<OptionOpaque> a;
  std::optional<OptionOpaqueChar> b;
  uint32_t c;
  std::optional<OptionOpaque> d;
};


#endif
