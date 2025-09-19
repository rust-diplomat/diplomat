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

inline std::unique_ptr<somelib::ns::RenamedComparable> somelib::ns::RenamedComparable::new_(uint8_t int_) {
    auto result = somelib::ns::capi::namespace_Comparable_new(int_);
    return std::unique_ptr<somelib::ns::RenamedComparable>(somelib::ns::RenamedComparable::FromFFI(result));
}

inline int8_t somelib::ns::RenamedComparable::cmp(const somelib::ns::RenamedComparable& other) const {
    auto result = somelib::ns::capi::namespace_Comparable_cmp(this->AsFFI(),
        other.AsFFI());
    return result;
}
inline bool somelib::ns::RenamedComparable::operator==(const somelib::ns::RenamedComparable& other) const {
    return this->cmp(other) == 0;
}

inline bool somelib::ns::RenamedComparable::operator!=(const somelib::ns::RenamedComparable& other) const {
    return this->cmp(other) != 0;
}

inline bool somelib::ns::RenamedComparable::operator<=(const somelib::ns::RenamedComparable& other) const {
    return this->cmp(other) <= 0;
}

inline bool somelib::ns::RenamedComparable::operator>=(const somelib::ns::RenamedComparable& other) const {
    return this->cmp(other) >= 0;
}

inline bool somelib::ns::RenamedComparable::operator<(const somelib::ns::RenamedComparable& other) const {
    return this->cmp(other) < 0;
}

inline bool somelib::ns::RenamedComparable::operator>(const somelib::ns::RenamedComparable& other) const {
    return this->cmp(other) > 0;
}

inline const somelib::ns::capi::RenamedComparable* somelib::ns::RenamedComparable::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedComparable*>(this);
}

inline somelib::ns::capi::RenamedComparable* somelib::ns::RenamedComparable::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedComparable*>(this);
}

inline const somelib::ns::RenamedComparable* somelib::ns::RenamedComparable::FromFFI(const somelib::ns::capi::RenamedComparable* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedComparable*>(ptr);
}

inline somelib::ns::RenamedComparable* somelib::ns::RenamedComparable::FromFFI(somelib::ns::capi::RenamedComparable* ptr) {
    return reinterpret_cast<somelib::ns::RenamedComparable*>(ptr);
}

inline void somelib::ns::RenamedComparable::operator delete(void* ptr) {
    somelib::ns::capi::namespace_Comparable_destroy(reinterpret_cast<somelib::ns::capi::RenamedComparable*>(ptr));
}


#endif // SOMELIB_ns_RenamedComparable_HPP
