#ifndef SOMELIB_FallibleOpaqueConstructor_D_HPP
#define SOMELIB_FallibleOpaqueConstructor_D_HPP

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
namespace capi { struct ResultOpaque; }
class ResultOpaque;
struct FallibleOpaqueConstructor;
} // namespace somelib



namespace somelib {
namespace capi {
    struct FallibleOpaqueConstructor {
      int32_t x;
    };

    typedef struct FallibleOpaqueConstructor_option {union { FallibleOpaqueConstructor ok; }; bool is_ok; } FallibleOpaqueConstructor_option;
} // namespace capi
} // namespace


namespace somelib {
struct FallibleOpaqueConstructor {
    int32_t x;

  inline static somelib::diplomat::result<somelib::FallibleOpaqueConstructor, std::unique_ptr<somelib::ResultOpaque>> ctor();

    inline somelib::capi::FallibleOpaqueConstructor AsFFI() const;
    inline static somelib::FallibleOpaqueConstructor FromFFI(somelib::capi::FallibleOpaqueConstructor c_struct);
};

} // namespace
#endif // SOMELIB_FallibleOpaqueConstructor_D_HPP
