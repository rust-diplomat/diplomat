#ifndef OptionEnum_HPP
#define OptionEnum_HPP

#include "OptionEnum.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {

} // namespace capi
} // namespace

inline diplomat::capi::OptionEnum OptionEnum::AsFFI() const {
    return static_cast<diplomat::capi::OptionEnum>(value);
}

inline OptionEnum OptionEnum::FromFFI(diplomat::capi::OptionEnum c_enum) {
    switch (c_enum) {
        case diplomat::capi::OptionEnum_Foo:
        case diplomat::capi::OptionEnum_Bar:
        case diplomat::capi::OptionEnum_Baz:
            return static_cast<OptionEnum::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // OptionEnum_HPP
