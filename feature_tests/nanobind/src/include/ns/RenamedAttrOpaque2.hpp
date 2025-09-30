#ifndef SOMELIB_ns_RenamedAttrOpaque2_HPP
#define SOMELIB_ns_RenamedAttrOpaque2_HPP

#include "RenamedAttrOpaque2.d.hpp"

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

    void namespace_AttrOpaque2_destroy(RenamedAttrOpaque2* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::ns::capi::RenamedAttrOpaque2* somelib::ns::RenamedAttrOpaque2::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedAttrOpaque2*>(this);
}

inline somelib::ns::capi::RenamedAttrOpaque2* somelib::ns::RenamedAttrOpaque2::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedAttrOpaque2*>(this);
}

inline const somelib::ns::RenamedAttrOpaque2* somelib::ns::RenamedAttrOpaque2::FromFFI(const somelib::ns::capi::RenamedAttrOpaque2* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedAttrOpaque2*>(ptr);
}

inline somelib::ns::RenamedAttrOpaque2* somelib::ns::RenamedAttrOpaque2::FromFFI(somelib::ns::capi::RenamedAttrOpaque2* ptr) {
    return reinterpret_cast<somelib::ns::RenamedAttrOpaque2*>(ptr);
}

inline void somelib::ns::RenamedAttrOpaque2::operator delete(void* ptr) {
    somelib::ns::capi::namespace_AttrOpaque2_destroy(reinterpret_cast<somelib::ns::capi::RenamedAttrOpaque2*>(ptr));
}


#endif // SOMELIB_ns_RenamedAttrOpaque2_HPP
