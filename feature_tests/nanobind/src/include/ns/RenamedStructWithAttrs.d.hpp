#ifndef ns_RenamedStructWithAttrs_D_HPP
#define ns_RenamedStructWithAttrs_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace ns {
struct RenamedStructWithAttrs;
} // namespace ns




namespace ns {
namespace capi {
    struct RenamedStructWithAttrs {
      bool a;
      uint32_t b;
    };

    typedef struct RenamedStructWithAttrs_option {union { RenamedStructWithAttrs ok; }; bool is_ok; } RenamedStructWithAttrs_option;
    typedef struct DiplomatRenamedStructWithAttrsView {
      const RenamedStructWithAttrs* data;
      size_t len;
    } DiplomatRenamedStructWithAttrsView;

    typedef struct DiplomatRenamedStructWithAttrsViewMut {
      RenamedStructWithAttrs* data;
      size_t len;
    } DiplomatRenamedStructWithAttrsViewMut;
} // namespace capi
} // namespace


namespace ns {
struct RenamedStructWithAttrs {
    bool a;
    uint32_t b;

  inline static diplomat::result<ns::RenamedStructWithAttrs, std::monostate> new_fallible(bool a, uint32_t b);

  inline uint32_t c() const;

  /**
   * \deprecated use Foo
   */
  [[deprecated("use Foo")]]
  inline void deprecated() const;

    inline ns::capi::RenamedStructWithAttrs AsFFI() const;
    inline static ns::RenamedStructWithAttrs FromFFI(ns::capi::RenamedStructWithAttrs c_struct);
};

} // namespace
namespace diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const ns::RenamedStructWithAttrs>>>> {
        using type = ns::capi::DiplomatRenamedStructWithAttrsView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<ns::RenamedStructWithAttrs>>>> {
        using type = ns::capi::DiplomatRenamedStructWithAttrsViewMut;
};
}
#endif // ns_RenamedStructWithAttrs_D_HPP
