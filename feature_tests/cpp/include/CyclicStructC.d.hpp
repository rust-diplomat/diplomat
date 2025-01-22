#ifndef CyclicStructC_D_HPP
#define CyclicStructC_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "CyclicStructA.d.hpp"
#include "diplomat_runtime.hpp"

struct CyclicStructA;


namespace diplomat {
namespace capi {
    struct CyclicStructC {
      diplomat::capi::CyclicStructA a;
    };
    
    typedef struct CyclicStructC_option {union { CyclicStructC ok; }; bool is_ok; } CyclicStructC_option;
} // namespace capi
} // namespace


struct CyclicStructC {
  CyclicStructA a;

  inline static CyclicStructC takes_nested_parameters(CyclicStructC c);

  inline std::string cyclic_out();

  inline diplomat::capi::CyclicStructC AsFFI() const;
  inline static CyclicStructC FromFFI(diplomat::capi::CyclicStructC c_struct);
};


#endif // CyclicStructC_D_HPP
