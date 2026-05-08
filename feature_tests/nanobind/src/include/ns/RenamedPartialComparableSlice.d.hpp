#ifndef SOMELIB_ns_RenamedPartialComparableSlice_D_HPP
#define SOMELIB_ns_RenamedPartialComparableSlice_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
namespace somelib {
namespace ns {
struct RenamedPartialComparableSlice;
} // namespace ns
} // namespace somelib



namespace somelib::ns {
namespace capi {
    struct RenamedPartialComparableSlice {
      float f;
    };

    typedef struct RenamedPartialComparableSlice_option {union { RenamedPartialComparableSlice ok; }; bool is_ok; } RenamedPartialComparableSlice_option;
    typedef struct DiplomatRenamedPartialComparableSliceView {
      const RenamedPartialComparableSlice* data;
      size_t len;
    } DiplomatRenamedPartialComparableSliceView;

    typedef struct DiplomatRenamedPartialComparableSliceViewMut {
      RenamedPartialComparableSlice* data;
      size_t len;
    } DiplomatRenamedPartialComparableSliceViewMut;
} // namespace capi
} // namespace


namespace somelib::ns {
struct RenamedPartialComparableSlice {
    float f;

  inline std::optional<int8_t> partial_cmp(const somelib::ns::RenamedPartialComparableSlice& other) const;

  inline std::optional<bool> operator==(const somelib::ns::RenamedPartialComparableSlice& other) const;
  inline std::optional<bool> operator!=(const somelib::ns::RenamedPartialComparableSlice& other) const;
  inline std::optional<bool> operator<=(const somelib::ns::RenamedPartialComparableSlice& other) const;
  inline std::optional<bool> operator>=(const somelib::ns::RenamedPartialComparableSlice& other) const;
  inline std::optional<bool> operator<(const somelib::ns::RenamedPartialComparableSlice& other) const;
  inline std::optional<bool> operator>(const somelib::ns::RenamedPartialComparableSlice& other) const;

    inline somelib::ns::capi::RenamedPartialComparableSlice AsFFI() const;
    inline static somelib::ns::RenamedPartialComparableSlice FromFFI(somelib::ns::capi::RenamedPartialComparableSlice c_struct);
};

} // namespace
namespace somelib::diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const somelib::ns::RenamedPartialComparableSlice>>>> {
        using type = somelib::ns::capi::DiplomatRenamedPartialComparableSliceView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<somelib::ns::RenamedPartialComparableSlice>>>> {
        using type = somelib::ns::capi::DiplomatRenamedPartialComparableSliceViewMut;
};
}
#endif // SOMELIB_ns_RenamedPartialComparableSlice_D_HPP
