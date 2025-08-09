#ifndef PrimitiveStruct_D_HPP
#define PrimitiveStruct_D_HPP

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
    struct PrimitiveStruct {
      float x;
      bool a;
      char32_t b;
      int64_t c;
      intptr_t d;
      uint8_t e;
    };

    typedef struct PrimitiveStruct_option {union { PrimitiveStruct ok; }; bool is_ok; } PrimitiveStruct_option;
    typedef struct DiplomatPrimitiveStructView {
      const PrimitiveStruct* data;
      size_t len;
    } DiplomatPrimitiveStructView;

    typedef struct DiplomatPrimitiveStructViewMut {
      PrimitiveStruct* data;
      size_t len;
    } DiplomatPrimitiveStructViewMut;
} // namespace capi
} // namespace


struct PrimitiveStruct {
  float x;
  bool a;
  char32_t b;
  int64_t c;
  intptr_t d;
  uint8_t e;

  inline static void mutable_slice(diplomat::span<PrimitiveStruct> a);

  inline void mutable_ref(PrimitiveStruct& a);

  inline diplomat::capi::PrimitiveStruct AsFFI() const;
  inline static PrimitiveStruct FromFFI(diplomat::capi::PrimitiveStruct c_struct);
};


namespace diplomat {
  template<typename T>
  struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const PrimitiveStruct>>>> {
    using type = capi::DiplomatPrimitiveStructView;
  };

  template<typename T>
  struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<PrimitiveStruct>>>> {
    using type = capi::DiplomatPrimitiveStructViewMut;
};
}
#endif // PrimitiveStruct_D_HPP
