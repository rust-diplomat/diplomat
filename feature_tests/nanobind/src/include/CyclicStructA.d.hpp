#ifndef CyclicStructA_D_HPP
#define CyclicStructA_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "CyclicStructB.d.hpp"
#include "diplomat_runtime.hpp"

struct CyclicStructB;


namespace diplomat {
namespace capi {
    struct CyclicStructA {
      diplomat::capi::CyclicStructB a;
    };
    
    typedef struct CyclicStructA_option {union { CyclicStructA ok; }; bool is_ok; } CyclicStructA_option;
} // namespace capi
} // namespace


struct CyclicStructA {
  CyclicStructB a;

  inline static CyclicStructB get_b();

  inline std::string cyclic_out();

  inline std::string double_cyclic_out(CyclicStructA cyclic_struct_a);

  inline std::string getter_out();

  inline diplomat::capi::CyclicStructA AsFFI() const;
  inline static CyclicStructA FromFFI(diplomat::capi::CyclicStructA c_struct);
};


#endif // CyclicStructA_D_HPP
