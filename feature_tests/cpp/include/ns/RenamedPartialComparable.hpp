#ifndef SOMELIB_ns_RenamedPartialComparable_HPP
#define SOMELIB_ns_RenamedPartialComparable_HPP

#include "RenamedPartialComparable.d.hpp"

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
    extern "C" {

    somelib::ns::capi::RenamedPartialComparable* namespace_PartialComparable_new(float float_);

    typedef struct namespace_PartialComparable_partial_cmp_result {union {int8_t ok; }; bool is_ok;} namespace_PartialComparable_partial_cmp_result;
    namespace_PartialComparable_partial_cmp_result namespace_PartialComparable_partial_cmp(const somelib::ns::capi::RenamedPartialComparable* self, const somelib::ns::capi::RenamedPartialComparable* other);

    typedef struct namespace_PartialComparable_test_nonstd_result {union {int8_t ok; }; bool is_ok;} namespace_PartialComparable_test_nonstd_result;
    namespace_PartialComparable_test_nonstd_result namespace_PartialComparable_test_nonstd(const somelib::ns::capi::RenamedPartialComparable* self, const somelib::ns::capi::RenamedPartialComparable* other);

    void namespace_PartialComparable_destroy(RenamedPartialComparable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::ns::RenamedPartialComparable somelib::ns::RenamedPartialComparable::new_(float float_) {
    auto result = somelib::ns::capi::namespace_PartialComparable_new(float_);
    return somelib::ns::RenamedPartialComparable::FromFFI(result);
}

inline somelib::diplomat::Optional<int8_t> somelib::ns::RenamedPartialComparable::partial_cmp(const somelib::ns::RenamedPartialComparable& other) const {
    auto result = somelib::ns::capi::namespace_PartialComparable_partial_cmp(this->AsFFI(),
        other.AsFFI());
    return result.is_ok ? somelib::diplomat::Optional<int8_t>(result.ok) : somelib::diplomat::Optional<int8_t>(std::nullopt);
}
inline std::optional<bool> somelib::ns::RenamedPartialComparable::operator==(const somelib::ns::RenamedPartialComparable& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() == 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparable::operator!=(const somelib::ns::RenamedPartialComparable& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() != 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparable::operator<=(const somelib::ns::RenamedPartialComparable& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() <= 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparable::operator>=(const somelib::ns::RenamedPartialComparable& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() >= 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparable::operator<(const somelib::ns::RenamedPartialComparable& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() < 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparable::operator>(const somelib::ns::RenamedPartialComparable& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() > 0;
}

inline somelib::diplomat::Optional<int8_t> somelib::ns::RenamedPartialComparable::test_nonstd(const somelib::ns::RenamedPartialComparable& other) const {
    auto result = somelib::ns::capi::namespace_PartialComparable_test_nonstd(this->AsFFI(),
        other.AsFFI());
    return result.is_ok ? somelib::diplomat::Optional<int8_t>(result.ok) : somelib::diplomat::Optional<int8_t>(std::nullopt);
}


#endif // SOMELIB_ns_RenamedPartialComparable_HPP
