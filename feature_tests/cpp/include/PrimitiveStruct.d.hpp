#ifndef SOMELIB_PrimitiveStruct_D_HPP
#define SOMELIB_PrimitiveStruct_D_HPP

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
struct PrimitiveStruct;
} // namespace somelib



namespace somelib {
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


namespace somelib {
struct PrimitiveStruct {
    float x;
    bool a;
    char32_t b;
    int64_t c;
    intptr_t d;
    uint8_t e;

  inline static void mutable_slice(somelib::diplomat::span<somelib::PrimitiveStruct> a);

  inline void mutable_ref(somelib::PrimitiveStruct& a);

    inline somelib::capi::PrimitiveStruct AsFFI() const;
    inline static somelib::PrimitiveStruct FromFFI(somelib::capi::PrimitiveStruct c_struct);
};

} // namespace
namespace somelib::diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const somelib::PrimitiveStruct>>>> {
        using type = somelib::capi::DiplomatPrimitiveStructView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<somelib::PrimitiveStruct>>>> {
        using type = somelib::capi::DiplomatPrimitiveStructViewMut;
};
}
#endif // SOMELIB_PrimitiveStruct_D_HPP
