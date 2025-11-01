#ifndef SOMELIB_ns_RenamedAttrEnum_D_HPP
#define SOMELIB_ns_RenamedAttrEnum_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace somelib::ns {
namespace capi {
    enum RenamedAttrEnum {
      RenamedAttrEnum_A = 0,
      RenamedAttrEnum_B = 1,
      RenamedAttrEnum_C = 2,
    };

    typedef struct RenamedAttrEnum_option {union { RenamedAttrEnum ok; }; bool is_ok; } RenamedAttrEnum_option;
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedAttrEnum {
public:
    enum Value {
        A = 0,
        B = 1,
        Renamed = 2,
    };

    RenamedAttrEnum(): value(Value::A) {}

    // Implicit conversions between enum and ::Value
    constexpr RenamedAttrEnum(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline somelib::ns::capi::RenamedAttrEnum AsFFI() const;
    inline static somelib::ns::RenamedAttrEnum FromFFI(somelib::ns::capi::RenamedAttrEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // SOMELIB_ns_RenamedAttrEnum_D_HPP
