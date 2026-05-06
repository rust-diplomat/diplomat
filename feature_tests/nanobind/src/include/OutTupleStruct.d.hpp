#ifndef SOMELIB_OutTupleStruct_D_HPP
#define SOMELIB_OutTupleStruct_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "PrimitiveStruct.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct Opaque; }
class Opaque;
struct OutTupleStruct;
struct PrimitiveStruct;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OutTupleStruct {
      int32_t x;
      int32_t y;
      somelib::capi::PrimitiveStruct primitive;
      somelib::capi::Opaque* opaque;
    };

    typedef struct OutTupleStruct_option {union { OutTupleStruct ok; }; bool is_ok; } OutTupleStruct_option;
} // namespace capi
} // namespace


namespace somelib {
struct OutTupleStruct {
    int32_t x;
    int32_t y;
    somelib::PrimitiveStruct primitive;
    std::unique_ptr<somelib::Opaque> opaque;

  inline static std::tuple<int32_t,int32_t,somelib::PrimitiveStruct,std::unique_ptr<somelib::Opaque>> new_();

    inline somelib::capi::OutTupleStruct AsFFI() const;
    inline static somelib::OutTupleStruct FromFFI(somelib::capi::OutTupleStruct c_struct);
    inline static somelib::capi::OutTupleStruct AsTupleFFI(std::tuple<int32_t,int32_t,somelib::PrimitiveStruct,std::unique_ptr<somelib::Opaque>> tuple);
    inline static std::tuple<int32_t,int32_t,somelib::PrimitiveStruct,std::unique_ptr<somelib::Opaque>> TupleFromFFI(somelib::capi::OutTupleStruct c_struct);
};

} // namespace
#endif // SOMELIB_OutTupleStruct_D_HPP
