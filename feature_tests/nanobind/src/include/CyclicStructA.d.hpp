#ifndef CyclicStructA_D_HPP
#define CyclicStructA_D_HPP

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

struct CyclicStructB;




namespace diplomat {
namespace capi {
    struct CyclicStructA {
      diplomat::capi::CyclicStructB a;
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


struct CyclicStructA {
    CyclicStructB a;

  inline static CyclicStructB get_b();

  inline std::string cyclic_out() const;
  template<typename W>
  inline void cyclic_out_write(W& writeable_output) const;

  inline static uint8_t nested_slice(diplomat::span<const CyclicStructA> sl);

  inline std::string double_cyclic_out(CyclicStructA cyclic_struct_a) const;
  template<typename W>
  inline void double_cyclic_out_write(CyclicStructA cyclic_struct_a, W& writeable_output) const;

  inline std::string getter_out() const;
  template<typename W>
  inline void getter_out_write(W& writeable_output) const;

    inline diplomat::capi::CyclicStructA AsFFI() const;
    inline static CyclicStructA FromFFI(diplomat::capi::CyclicStructA c_struct);
};


namespace diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const CyclicStructA>>>> {
        using type = capi::DiplomatCyclicStructAView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<CyclicStructA>>>> {
        using type = capi::DiplomatCyclicStructAViewMut;
};
}
#endif // CyclicStructA_D_HPP
