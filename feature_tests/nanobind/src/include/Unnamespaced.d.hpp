#ifndef SOMELIB_Unnamespaced_D_HPP
#define SOMELIB_Unnamespaced_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct Unnamespaced; }
class Unnamespaced;
} // namespace somelib
namespace somelib {
namespace ns {
namespace capi { struct AttrOpaque1Renamed; }
class AttrOpaque1Renamed;
class RenamedAttrEnum;
} // namespace ns
} // namespace somelib



namespace somelib {
namespace capi {
    struct Unnamespaced;
    extern "C" {
    void namespace_Unnamespaced_destroy(Unnamespaced* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class Unnamespaced;
using UnnamespacedRef = somelib::diplomat::Ref<Unnamespaced, const somelib::capi::Unnamespaced>;
using UnnamespacedRefMut = somelib::diplomat::Ref<Unnamespaced, somelib::capi::Unnamespaced>;

class Unnamespaced : public somelib::diplomat::OpaquePointer<Unnamespaced, somelib::capi::Unnamespaced, somelib::capi::namespace_Unnamespaced_destroy> {
public:

  inline static somelib::Unnamespaced make(somelib::ns::RenamedAttrEnum _e);

  inline void use_namespaced(const somelib::ns::AttrOpaque1Renamed& _n) const;

};

} // namespace
#endif // SOMELIB_Unnamespaced_D_HPP
