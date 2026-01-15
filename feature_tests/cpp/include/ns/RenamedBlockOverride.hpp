#ifndef SOMELIB_ns_RenamedBlockOverride_HPP
#define SOMELIB_ns_RenamedBlockOverride_HPP

#include "RenamedBlockOverride.d.hpp"

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

    void namespace_BlockOverride_destroy(RenamedBlockOverride* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const somelib::ns::capi::RenamedBlockOverride* somelib::ns::RenamedBlockOverride::AsFFI() const {
    return reinterpret_cast<const somelib::ns::capi::RenamedBlockOverride*>(this);
}

inline somelib::ns::capi::RenamedBlockOverride* somelib::ns::RenamedBlockOverride::AsFFI() {
    return reinterpret_cast<somelib::ns::capi::RenamedBlockOverride*>(this);
}

inline const somelib::ns::RenamedBlockOverride* somelib::ns::RenamedBlockOverride::FromFFI(const somelib::ns::capi::RenamedBlockOverride* ptr) {
    return reinterpret_cast<const somelib::ns::RenamedBlockOverride*>(ptr);
}

inline somelib::ns::RenamedBlockOverride* somelib::ns::RenamedBlockOverride::FromFFI(somelib::ns::capi::RenamedBlockOverride* ptr) {
    return reinterpret_cast<somelib::ns::RenamedBlockOverride*>(ptr);
}

inline void somelib::ns::RenamedBlockOverride::operator delete(void* ptr) {
    somelib::ns::capi::namespace_BlockOverride_destroy(reinterpret_cast<somelib::ns::capi::RenamedBlockOverride*>(ptr));
}

std::string somelib::ns::RenamedBlockOverride::special_function() {
    return "This is a custom binding.";
}

#endif // SOMELIB_ns_RenamedBlockOverride_HPP
