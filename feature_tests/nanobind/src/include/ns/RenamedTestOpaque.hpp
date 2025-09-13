#ifndef ns_RenamedTestOpaque_HPP
#define ns_RenamedTestOpaque_HPP

#include "RenamedTestOpaque.d.hpp"

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
    extern "C" {

    void namespace_TestOpaque_destroy(RenamedTestOpaque* self);

    } // extern "C"
} // namespace capi
} // namespace

inline const ns::capi::RenamedTestOpaque* ns::RenamedTestOpaque::AsFFI() const {
    return reinterpret_cast<const ns::capi::RenamedTestOpaque*>(this);
}

inline ns::capi::RenamedTestOpaque* ns::RenamedTestOpaque::AsFFI() {
    return reinterpret_cast<ns::capi::RenamedTestOpaque*>(this);
}

inline const ns::RenamedTestOpaque* ns::RenamedTestOpaque::FromFFI(const ns::capi::RenamedTestOpaque* ptr) {
    return reinterpret_cast<const ns::RenamedTestOpaque*>(ptr);
}

inline ns::RenamedTestOpaque* ns::RenamedTestOpaque::FromFFI(ns::capi::RenamedTestOpaque* ptr) {
    return reinterpret_cast<ns::RenamedTestOpaque*>(ptr);
}

inline void ns::RenamedTestOpaque::operator delete(void* ptr) {
    ns::capi::namespace_TestOpaque_destroy(reinterpret_cast<ns::capi::RenamedTestOpaque*>(ptr));
}


#endif // ns_RenamedTestOpaque_HPP
