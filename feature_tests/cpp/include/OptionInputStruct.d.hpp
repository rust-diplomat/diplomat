#ifndef OptionInputStruct_D_HPP
#define OptionInputStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "OptionEnum.d.hpp"
#include "diplomat_runtime.hpp"

class OptionEnum;


namespace diplomat {
namespace capi {
    struct OptionInputStruct {
      diplomat::capi::OptionU8 a;
      diplomat::capi::OptionChar b;
      diplomat::capi::OptionEnum_option c;
    };
    
    typedef struct OptionInputStruct_option {union { OptionInputStruct ok; }; bool is_ok; } OptionInputStruct_option;
} // namespace capi
} // namespace


struct OptionInputStruct {
  std::optional<uint8_t> a;
  std::optional<char32_t> b;
  std::optional<OptionEnum> c;

  inline diplomat::capi::OptionInputStruct AsFFI() const;
  inline static OptionInputStruct FromFFI(diplomat::capi::OptionInputStruct c_struct);
};


#endif // OptionInputStruct_D_HPP
