#ifndef SOMELIB_ns_RenamedDeprecatedEnum_D_HPP
#define SOMELIB_ns_RenamedDeprecatedEnum_D_HPP

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
    enum RenamedDeprecatedEnum {
      RenamedDeprecatedEnum_A = 0,
    };

    typedef struct RenamedDeprecatedEnum_option {union { RenamedDeprecatedEnum ok; }; bool is_ok; } RenamedDeprecatedEnum_option;
} // namespace capi
} // namespace

namespace somelib::ns {
/**
 * \deprecated use Foo
 */
class [[deprecated("use Foo")]] RenamedDeprecatedEnum {
public:
    enum Value {
        A = 0,
    };

    RenamedDeprecatedEnum(): value(Value::A) {}

    // Implicit conversions between enum and ::Value
    constexpr RenamedDeprecatedEnum(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline somelib::ns::capi::RenamedDeprecatedEnum AsFFI() const;
    inline static somelib::ns::RenamedDeprecatedEnum FromFFI(somelib::ns::capi::RenamedDeprecatedEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // SOMELIB_ns_RenamedDeprecatedEnum_D_HPP
