#ifndef SOMELIB_ContiguousEnum_D_HPP
#define SOMELIB_ContiguousEnum_D_HPP

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
namespace capi {
    enum ContiguousEnum {
      ContiguousEnum_C = 0,
      ContiguousEnum_D = 1,
      ContiguousEnum_E = 2,
      ContiguousEnum_F = 3,
    };

    typedef struct ContiguousEnum_option {union { ContiguousEnum ok; }; bool is_ok; } ContiguousEnum_option;
    typedef struct DiplomatContiguousEnumView {
      const ContiguousEnum* data;
      size_t len;
    } DiplomatContiguousEnumView;

    typedef struct DiplomatContiguousEnumViewMut {
      ContiguousEnum* data;
      size_t len;
    } DiplomatContiguousEnumViewMut;
} // namespace capi
} // namespace

namespace somelib {
class ContiguousEnum {
public:
    enum Value {
        C = 0,
        D = 1,
        E = 2,
        F = 3,
    };

    ContiguousEnum(): value(Value::E) {}

    // Implicit conversions between enum and ::Value
    constexpr ContiguousEnum(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline somelib::capi::ContiguousEnum AsFFI() const;
    inline static somelib::ContiguousEnum FromFFI(somelib::capi::ContiguousEnum c_enum);
private:
    Value value;
};

} // namespace
namespace somelib::diplomat {
    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<const somelib::ContiguousEnum>>>> {
        using type = somelib::capi::DiplomatContiguousEnumView;
    };

    template<typename T>
    struct diplomat_c_span_convert<T, std::enable_if_t<std::is_same_v<T, span<somelib::ContiguousEnum>>>> {
        using type = somelib::capi::DiplomatContiguousEnumViewMut;
    };
}
#endif // SOMELIB_ContiguousEnum_D_HPP
