#ifndef SOMELIB_ContainingTuple_D_HPP
#define SOMELIB_ContainingTuple_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "TupleStruct.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
struct TupleStruct;
} // namespace somelib



namespace somelib {
namespace capi {
    struct ContainingTuple {
      somelib::capi::TupleStruct inner;
    };

    typedef struct ContainingTuple_option {union { ContainingTuple ok; }; bool is_ok; } ContainingTuple_option;
} // namespace capi
} // namespace


namespace somelib {
struct ContainingTuple {
    somelib::TupleStruct inner;

    inline somelib::capi::ContainingTuple AsFFI() const;
    inline static somelib::ContainingTuple FromFFI(somelib::capi::ContainingTuple c_struct);
};

} // namespace
#endif // SOMELIB_ContainingTuple_D_HPP
