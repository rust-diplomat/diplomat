#ifndef SOMELIB_CyclicStructB_D_HPP
#define SOMELIB_CyclicStructB_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace somelib {
struct CyclicStructA;
} // namespace somelib



namespace somelib {
namespace capi {
    struct CyclicStructB {
      uint8_t field;
    };

    typedef struct CyclicStructB_option {union { CyclicStructB ok; }; bool is_ok; } CyclicStructB_option;
} // namespace capi
} // namespace


namespace somelib {
struct CyclicStructB {
    uint8_t field;

  inline static somelib::CyclicStructA get_a();

  inline static somelib::diplomat::Optional<somelib::CyclicStructA> get_a_option();

    inline somelib::capi::CyclicStructB AsFFI() const;
    inline static somelib::CyclicStructB FromFFI(somelib::capi::CyclicStructB c_struct);
};

} // namespace
#endif // SOMELIB_CyclicStructB_D_HPP
