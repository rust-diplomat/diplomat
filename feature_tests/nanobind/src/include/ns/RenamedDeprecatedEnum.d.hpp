#ifndef ns_RenamedDeprecatedEnum_D_HPP
#define ns_RenamedDeprecatedEnum_D_HPP

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
namespace capi {
    enum RenamedDeprecatedEnum {
      RenamedDeprecatedEnum_A = 0,
    };

    typedef struct RenamedDeprecatedEnum_option {union { RenamedDeprecatedEnum ok; }; bool is_ok; } RenamedDeprecatedEnum_option;
} // namespace capi
} // namespace

namespace ns {
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

    inline ns::capi::RenamedDeprecatedEnum AsFFI() const;
    inline static ns::RenamedDeprecatedEnum FromFFI(ns::capi::RenamedDeprecatedEnum c_enum);
private:
    Value value;
};

} // namespace
#endif // ns_RenamedDeprecatedEnum_D_HPP
