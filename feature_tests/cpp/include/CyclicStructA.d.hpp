#ifndef CyclicStructA_D_HPP
#define CyclicStructA_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CyclicStructB.d.hpp"

struct CyclicStructB;


namespace diplomat {
namespace capi {
    struct CyclicStructA {
      diplomat::capi::CyclicStructB a;
    };
} // namespace capi
} // namespace


struct CyclicStructA {
  CyclicStructB a;

  inline static CyclicStructB get_b();

  inline diplomat::capi::CyclicStructA AsFFI() const;
  inline static CyclicStructA FromFFI(diplomat::capi::CyclicStructA c_struct);
};


#endif // CyclicStructA_D_HPP
