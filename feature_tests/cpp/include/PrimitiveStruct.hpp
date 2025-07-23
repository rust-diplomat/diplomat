#ifndef PrimitiveStruct_HPP
#define PrimitiveStruct_HPP

#include "PrimitiveStruct.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    void PrimitiveStruct_mutable_slice(diplomat::capi::DiplomatPrimitiveStructViewMut a);

    void PrimitiveStruct_mutable_ref(diplomat::capi::PrimitiveStruct* self, diplomat::capi::PrimitiveStruct* a);

    } // extern "C"
} // namespace capi
} // namespace

inline void PrimitiveStruct::mutable_slice(diplomat::span<PrimitiveStruct> a) {
  diplomat::capi::PrimitiveStruct_mutable_slice({reinterpret_cast<diplomat::capi::PrimitiveStruct*>(a.data()), a.size()});
}

inline void PrimitiveStruct::mutable_ref(PrimitiveStruct& a) const {
  diplomat::capi::PrimitiveStruct_mutable_ref(reinterpret_cast<diplomat::capi::PrimitiveStruct*>(this),
    reinterpret_cast<diplomat::capi::PrimitiveStruct*>(a));
}


inline diplomat::capi::PrimitiveStruct PrimitiveStruct::AsFFI() const {
  return diplomat::capi::PrimitiveStruct {
    /* .x = */ x,
    /* .a = */ a,
    /* .b = */ b,
    /* .c = */ c,
    /* .d = */ d,
    /* .e = */ e,
  };
}

inline PrimitiveStruct PrimitiveStruct::FromFFI(diplomat::capi::PrimitiveStruct c_struct) {
  return PrimitiveStruct {
    /* .x = */ c_struct.x,
    /* .a = */ c_struct.a,
    /* .b = */ c_struct.b,
    /* .c = */ c_struct.c,
    /* .d = */ c_struct.d,
    /* .e = */ c_struct.e,
  };
}


#endif // PrimitiveStruct_HPP
