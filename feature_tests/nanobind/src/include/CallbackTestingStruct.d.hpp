#ifndef CallbackTestingStruct_D_HPP
#define CallbackTestingStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct CallbackTestingStruct {
      int32_t x;
      int32_t y;
    };
    
    typedef struct CallbackTestingStruct_option {union { CallbackTestingStruct ok; }; bool is_ok; } CallbackTestingStruct_option;
} // namespace capi
} // namespace


struct CallbackTestingStruct {
  int32_t x;
  int32_t y;

  inline diplomat::capi::CallbackTestingStruct AsFFI() const;
  inline static CallbackTestingStruct FromFFI(diplomat::capi::CallbackTestingStruct c_struct);
};


#endif // CallbackTestingStruct_D_HPP
