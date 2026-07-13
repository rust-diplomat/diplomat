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
#include "ns/RenamedAttrEnum.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    somelib::capi::Unnamespaced* namespace_Unnamespaced_make(somelib::ns::capi::RenamedAttrEnum _e);

    void namespace_Unnamespaced_use_namespaced(const somelib::capi::Unnamespaced* self, const somelib::ns::capi::AttrOpaque1Renamed* _n);

    void namespace_Unnamespaced_destroy(Unnamespaced* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::Unnamespaced somelib::Unnamespaced::make(somelib::ns::RenamedAttrEnum _e) {
    auto result = somelib::capi::namespace_Unnamespaced_make(_e.AsFFI());
    return somelib::Unnamespaced::FromFFI(result);
}

inline void somelib::Unnamespaced::use_namespaced(const somelib::ns::AttrOpaque1Renamed& _n) const {
    somelib::capi::namespace_Unnamespaced_use_namespaced(this->AsFFI(),
        _n.AsFFI());
}


#endif // SOMELIB_Unnamespaced_HPP
