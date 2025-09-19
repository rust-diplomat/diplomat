#ifndef SOMELIB_CyclicStructC_D_HPP
#define SOMELIB_CyclicStructC_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "CyclicStructA.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
struct CyclicStructA;
struct CyclicStructC;
} // namespace somelib



namespace somelib {
namespace capi {
    struct CyclicStructC {
      somelib::capi::CyclicStructA a;
    };

    typedef struct CyclicStructC_option {union { CyclicStructC ok; }; bool is_ok; } CyclicStructC_option;
} // namespace capi
} // namespace


namespace somelib {
struct CyclicStructC {
    somelib::CyclicStructA a;

  inline static somelib::CyclicStructC takes_nested_parameters(somelib::CyclicStructC c);

  inline std::string cyclic_out() const;
  template<typename W>
  inline void cyclic_out_write(W& writeable_output) const;

    inline somelib::capi::CyclicStructC AsFFI() const;
    inline static somelib::CyclicStructC FromFFI(somelib::capi::CyclicStructC c_struct);
};

} // namespace
#endif // SOMELIB_CyclicStructC_D_HPP
