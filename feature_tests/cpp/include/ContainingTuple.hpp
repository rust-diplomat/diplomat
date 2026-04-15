#ifndef SOMELIB_ContainingTuple_HPP
#define SOMELIB_ContainingTuple_HPP

#include "ContainingTuple.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "MyStruct.hpp"
#include "Opaque.hpp"
#include "TupleStruct.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {

} // namespace capi
} // namespace


inline somelib::capi::ContainingTuple somelib::ContainingTuple::AsFFI() const {
    return somelib::capi::ContainingTuple {
        /* .inner = */ somelib::TupleStruct::AsTupleFFI(inner),
    };
}

inline somelib::ContainingTuple somelib::ContainingTuple::FromFFI(somelib::capi::ContainingTuple c_struct) {
    return somelib::ContainingTuple {
        /* .inner = */ somelib::TupleStruct::TupleFromFFI(c_struct.inner),
    };
}
inline somelib::capi::ContainingTuple somelib::ContainingTuple::AsTupleFFI(std::tuple<std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&>> tuple) {
    return somelib::capi::ContainingTuple {
        /* .inner = */ somelib::TupleStruct::AsTupleFFI(std::get<0>(tuple)),
    };
}

inline std::tuple<std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&>> somelib::ContainingTuple::TupleFromFFI(somelib::capi::ContainingTuple c_struct) {
    return std::tuple<std::tuple<int32_t,int32_t,somelib::MyStruct,const somelib::Opaque&>>{
        /* .inner = */ somelib::TupleStruct::TupleFromFFI(c_struct.inner),
    };
}


#endif // SOMELIB_ContainingTuple_HPP
