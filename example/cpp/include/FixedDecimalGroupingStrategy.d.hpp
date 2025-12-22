#ifndef ICU4X_FixedDecimalGroupingStrategy_D_HPP
#define ICU4X_FixedDecimalGroupingStrategy_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum FixedDecimalGroupingStrategy {
      FixedDecimalGroupingStrategy_Auto = 0,
      FixedDecimalGroupingStrategy_Never = 1,
      FixedDecimalGroupingStrategy_Always = 2,
      FixedDecimalGroupingStrategy_Min2 = 3,
    };

    typedef struct FixedDecimalGroupingStrategy_option {union { FixedDecimalGroupingStrategy ok; }; bool is_ok; } FixedDecimalGroupingStrategy_option;
} // namespace capi
} // namespace

namespace icu4x {
class FixedDecimalGroupingStrategy {
public:
    enum Value {
        /**
         * Auto grouping
         */
        Auto = 0,
        /**
         * No grouping
         */
        Never = 1,
        /**
         * Always group
         */
        Always = 2,
        /**
         * At least 2 groups
         */
        Min2 = 3,
    };

    FixedDecimalGroupingStrategy(): value(Value::Auto) {}

    // Implicit conversions between enum and ::Value
    constexpr FixedDecimalGroupingStrategy(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::FixedDecimalGroupingStrategy AsFFI() const;
    inline static icu4x::FixedDecimalGroupingStrategy FromFFI(icu4x::capi::FixedDecimalGroupingStrategy c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_FixedDecimalGroupingStrategy_D_HPP
