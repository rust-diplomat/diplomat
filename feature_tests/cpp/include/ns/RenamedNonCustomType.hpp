#ifndef SOMELIB_ns_RenamedNonCustomType_HPP
#define SOMELIB_ns_RenamedNonCustomType_HPP

#include "RenamedNonCustomType.d.hpp"

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

    void namespace_NonCustomType_destroy(RenamedNonCustomType* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::ns::capi::RenamedNonCustomType* somelib::ns::RenamedNonCustomType::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedNonCustomType*>(this);
}

inline somelib::ns::capi::RenamedNonCustomType* somelib::ns::RenamedNonCustomType::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedNonCustomType*>(this);
}

inline const somelib::ns::RenamedNonCustomType* somelib::ns::RenamedNonCustomType::FromFFI(const somelib::ns::capi::RenamedNonCustomType* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedNonCustomType*>(ptr);
}

inline somelib::ns::RenamedNonCustomType* somelib::ns::RenamedNonCustomType::FromFFI(somelib::ns::capi::RenamedNonCustomType* ptr) {
    return reinterpret_cast<somelib::ns::RenamedNonCustomType*>(ptr);
}

inline void somelib::ns::RenamedNonCustomType::operator delete(void* ptr) {
    somelib::ns::capi::namespace_NonCustomType_destroy(reinterpret_cast<somelib::ns::capi::RenamedNonCustomType*>(ptr));
}


#endif // SOMELIB_ns_RenamedNonCustomType_HPP
