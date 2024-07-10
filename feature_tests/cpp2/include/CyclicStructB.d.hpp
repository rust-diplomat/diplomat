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


namespace capi {
    typedef struct CyclicStructB {
      uint8_t field;
    } CyclicStructB;
}

struct CyclicStructB {
  uint8_t field;

  inline static CyclicStructA get_a();

  inline ::capi::CyclicStructB AsFFI() const;
  inline static CyclicStructB FromFFI(::capi::CyclicStructB c_struct);
};


#endif // CyclicStructB_D_HPP
