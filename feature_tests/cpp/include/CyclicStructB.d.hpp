#ifndef CyclicStructB_D_HPP
#define CyclicStructB_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

struct CyclicStructA;


namespace diplomat {
namespace capi {
    struct CyclicStructB {
      uint8_t field;
    };
    
    typedef struct CyclicStructB_option {union { CyclicStructB ok; }; bool is_ok; } CyclicStructB_option;
} // namespace capi
} // namespace


struct CyclicStructB {
  uint8_t field;

  inline static CyclicStructA get_a();

  inline diplomat::capi::CyclicStructB AsFFI() const;
  inline static CyclicStructB FromFFI(diplomat::capi::CyclicStructB c_struct);
};


#endif // CyclicStructB_D_HPP
