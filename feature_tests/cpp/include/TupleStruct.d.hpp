#ifndef SOMELIB_TupleStruct_D_HPP
#define SOMELIB_TupleStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "MyStruct.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct Opaque; }
class Opaque;
struct ContainingTuple;
struct MyStruct;
struct TupleStruct;
} // namespace somelib



namespace somelib {
namespace capi {
    struct TupleStruct {
      int32_t x;
      int32_t y;
      somelib::capi::MyStruct st;
      const somelib::capi::Opaque* op;
    };

    typedef struct TupleStruct_option {union { TupleStruct ok; }; bool is_ok; } TupleStruct_option;
} // namespace capi
} // namespace


namespace somelib {
struct TupleStruct {
    int32_t x;
    int32_t y;
    somelib::MyStruct st;
    const somelib::Opaque& op;

  inline static int32_t takes_st_as_tuple(std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&> a);

  inline static char32_t takes_containing(std::tuple<std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&>> c);

    inline somelib::capi::TupleStruct AsFFI() const;
    inline static somelib::TupleStruct FromFFI(somelib::capi::TupleStruct c_struct);
    inline static somelib::capi::TupleStruct AsTupleFFI(std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&> tuple);
    inline static std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&> TupleFromFFI(somelib::capi::TupleStruct c_struct);
};

} // namespace
#endif // SOMELIB_TupleStruct_D_HPP
