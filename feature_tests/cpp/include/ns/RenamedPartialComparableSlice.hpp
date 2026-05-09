#ifndef SOMELIB_ns_RenamedPartialComparableSlice_HPP
#define SOMELIB_ns_RenamedPartialComparableSlice_HPP

#include "RenamedPartialComparableSlice.d.hpp"

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

    typedef struct namespace_PartialComparableSlice_partial_cmp_result {union {int8_t ok; }; bool is_ok;} namespace_PartialComparableSlice_partial_cmp_result;
    namespace_PartialComparableSlice_partial_cmp_result namespace_PartialComparableSlice_partial_cmp(const somelib::ns::capi::RenamedPartialComparableSlice* self, const somelib::ns::capi::RenamedPartialComparableSlice* other);

    } // extern "C"
} // namespace capi
} // namespace

inline std::optional<int8_t> somelib::ns::RenamedPartialComparableSlice::partial_cmp(const somelib::ns::RenamedPartialComparableSlice& other) const {
    auto result = somelib::ns::capi::namespace_PartialComparableSlice_partial_cmp(reinterpret_cast<const somelib::ns::capi::RenamedPartialComparableSlice*>(this),
        reinterpret_cast<const somelib::ns::capi::RenamedPartialComparableSlice*>(&other));
    return result.is_ok ? std::optional<int8_t>(result.ok) : std::nullopt;
}
inline std::optional<bool> somelib::ns::RenamedPartialComparableSlice::operator==(const somelib::ns::RenamedPartialComparableSlice& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() == 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparableSlice::operator!=(const somelib::ns::RenamedPartialComparableSlice& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() != 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparableSlice::operator<=(const somelib::ns::RenamedPartialComparableSlice& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() <= 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparableSlice::operator>=(const somelib::ns::RenamedPartialComparableSlice& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() >= 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparableSlice::operator<(const somelib::ns::RenamedPartialComparableSlice& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() < 0;
}

inline std::optional<bool> somelib::ns::RenamedPartialComparableSlice::operator>(const somelib::ns::RenamedPartialComparableSlice& other) const {
    auto val = this->partial_cmp(other);
    if (!val.has_value()) { return std::nullopt; } return val.value() > 0;
}


inline somelib::ns::capi::RenamedPartialComparableSlice somelib::ns::RenamedPartialComparableSlice::AsFFI() const {
    return somelib::ns::capi::RenamedPartialComparableSlice {
        /* .f = */ f,
    };
}

inline somelib::ns::RenamedPartialComparableSlice somelib::ns::RenamedPartialComparableSlice::FromFFI(somelib::ns::capi::RenamedPartialComparableSlice c_struct) {
    return somelib::ns::RenamedPartialComparableSlice {
        /* .f = */ c_struct.f,
    };
}


#endif // SOMELIB_ns_RenamedPartialComparableSlice_HPP
