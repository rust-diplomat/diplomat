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
#include "MyStruct.d.hpp"
#include "TupleStruct.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct Opaque; }
class Opaque;
struct MyStruct;
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
    std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&> inner;

    inline somelib::capi::ContainingTuple AsFFI() const;
    inline static somelib::ContainingTuple FromFFI(somelib::capi::ContainingTuple c_struct);
    inline static somelib::capi::ContainingTuple AsTupleFFI(std::tuple<std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&>> tuple);
    inline static std::tuple<std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&>> TupleFromFFI(somelib::capi::ContainingTuple c_struct);
};

} // namespace
#endif // SOMELIB_ContainingTuple_D_HPP
