#ifndef SOMELIB_Unnamespaced_HPP
#define SOMELIB_Unnamespaced_HPP

#include "Unnamespaced.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
#include "ns/AttrOpaque1Renamed.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    void namespace_Unnamespaced_use_namespaced(const somelib::capi::Unnamespaced* self, const somelib::ns::capi::AttrOpaque1Renamed* _n);

    void namespace_Unnamespaced_destroy(Unnamespaced* self);

    } // extern "C"
} // namespace capi
} // namespace

inline void somelib::Unnamespaced::use_namespaced(const somelib::ns::AttrOpaque1Renamed& _n) const {
    somelib::capi::namespace_Unnamespaced_use_namespaced(this->AsFFI(),
        _n.AsFFI());
}

inline const somelib::capi::Unnamespaced* somelib::Unnamespaced::AsFFI() const {
    return reinterpret_cast<const somelib::capi::Unnamespaced*>(this);
}

inline somelib::capi::Unnamespaced* somelib::Unnamespaced::AsFFI() {
    return reinterpret_cast<somelib::capi::Unnamespaced*>(this);
}

inline const somelib::Unnamespaced* somelib::Unnamespaced::FromFFI(const somelib::capi::Unnamespaced* ptr) {
    return reinterpret_cast<const somelib::Unnamespaced*>(ptr);
}

inline somelib::Unnamespaced* somelib::Unnamespaced::FromFFI(somelib::capi::Unnamespaced* ptr) {
    return reinterpret_cast<somelib::Unnamespaced*>(ptr);
}

inline void somelib::Unnamespaced::operator delete(void* ptr) {
    somelib::capi::namespace_Unnamespaced_destroy(reinterpret_cast<somelib::capi::Unnamespaced*>(ptr));
}


#endif // SOMELIB_Unnamespaced_HPP
