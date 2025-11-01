#ifndef SOMELIB_CyclicStructA_D_HPP
#define SOMELIB_CyclicStructA_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "CyclicStructB.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
struct CyclicStructA;
struct CyclicStructB;
} // namespace somelib



namespace somelib {
namespace capi {
    struct CyclicStructA {
      somelib::capi::CyclicStructB a;
    };

    typedef struct CyclicStructA_option {union { CyclicStructA ok; }; bool is_ok; } CyclicStructA_option;
    typedef struct DiplomatCyclicStructAView {
      const CyclicStructA* data;
      size_t len;
    } DiplomatCyclicStructAView;

    typedef struct DiplomatCyclicStructAViewMut {
      CyclicStructA* data;
      size_t len;
    } DiplomatCyclicStructAViewMut;
} // namespace capi
} // namespace


namespace somelib {
struct CyclicStructA {
    somelib::CyclicStructB a;

  inline static somelib::CyclicStructB get_b();

  inline std::string cyclic_out() const;
  template<typename W>
  inline void cyclic_out_write(W& writeable_output) const;

  inline static uint8_t nested_slice(somelib::diplomat::span<const somelib::CyclicStructA> sl);

  inline std::string double_cyclic_out(somelib::CyclicStructA cyclic_struct_a) const;
  template<typename W>
  inline void double_cyclic_out_write(somelib::CyclicStructA cyclic_struct_a, W& writeable_output) const;

  inline std::string getter_out() const;
  template<typename W>
  inline void getter_out_write(W& writeable_output) const;

    inline somelib::capi::CyclicStructA AsFFI() const;
    inline static somelib::CyclicStructA FromFFI(somelib::capi::CyclicStructA c_struct);
};

} // namespace
namespace somelib::diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const somelib::CyclicStructA>>>> {
        using type = somelib::capi::DiplomatCyclicStructAView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<somelib::CyclicStructA>>>> {
        using type = somelib::capi::DiplomatCyclicStructAViewMut;
};
}
#endif // SOMELIB_CyclicStructA_D_HPP
