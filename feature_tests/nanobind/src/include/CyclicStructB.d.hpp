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
    typedef struct DiplomatCyclicStructBView {
      const CyclicStructB* data;
      size_t len;
    } DiplomatCyclicStructBView;

    typedef struct DiplomatCyclicStructBViewMut {
      CyclicStructB* data;
      size_t len;
    } DiplomatCyclicStructBViewMut;
} // namespace capi
} // namespace


namespace somelib {
struct CyclicStructB {
    uint8_t field;

  inline static somelib::CyclicStructA get_a();

  inline static std::optional<somelib::CyclicStructA> get_a_option();

    inline somelib::capi::CyclicStructB AsFFI() const;
    inline static somelib::CyclicStructB FromFFI(somelib::capi::CyclicStructB c_struct);
};

} // namespace
namespace somelib::diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const somelib::CyclicStructB>>>> {
        using type = somelib::capi::DiplomatCyclicStructBView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<somelib::CyclicStructB>>>> {
        using type = somelib::capi::DiplomatCyclicStructBViewMut;
};
}
#endif // SOMELIB_CyclicStructB_D_HPP
