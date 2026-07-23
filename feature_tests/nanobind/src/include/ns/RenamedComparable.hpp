#ifndef SOMELIB_ns_RenamedComparable_HPP
#define SOMELIB_ns_RenamedComparable_HPP

#include "RenamedComparable.d.hpp"

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

    somelib::ns::capi::RenamedComparable* namespace_Comparable_new(uint8_t int_);

    int8_t namespace_Comparable_cmp(const somelib::ns::capi::RenamedComparable* self, const somelib::ns::capi::RenamedComparable* other);

    void namespace_Comparable_destroy(RenamedComparable* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::ns::RenamedComparable somelib::ns::RenamedComparable::new_(uint8_t int_) {
    auto result = somelib::ns::capi::namespace_Comparable_new(int_);
    return somelib::ns::RenamedComparable::FromFFI(result);
}

inline int8_t somelib::ns::RenamedComparable::cmp(const somelib::ns::RenamedComparable& other) const {
    auto result = somelib::ns::capi::namespace_Comparable_cmp(this->AsFFI(),
        other.AsFFI());
    return result;
}
inline bool somelib::ns::RenamedComparable::operator==(const somelib::ns::RenamedComparable& other) const {
    auto val = this->cmp(other);
    return val == 0;
}

inline bool somelib::ns::RenamedComparable::operator!=(const somelib::ns::RenamedComparable& other) const {
    auto val = this->cmp(other);
    return val != 0;
}

inline bool somelib::ns::RenamedComparable::operator<=(const somelib::ns::RenamedComparable& other) const {
    auto val = this->cmp(other);
    return val <= 0;
}

inline bool somelib::ns::RenamedComparable::operator>=(const somelib::ns::RenamedComparable& other) const {
    auto val = this->cmp(other);
    return val >= 0;
}

inline bool somelib::ns::RenamedComparable::operator<(const somelib::ns::RenamedComparable& other) const {
    auto val = this->cmp(other);
    return val < 0;
}

inline bool somelib::ns::RenamedComparable::operator>(const somelib::ns::RenamedComparable& other) const {
    auto val = this->cmp(other);
    return val > 0;
}


#endif // SOMELIB_ns_RenamedComparable_HPP
