#ifndef SOMELIB_ns_RenamedAttrOpaque2_D_HPP
#define SOMELIB_ns_RenamedAttrOpaque2_D_HPP

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
    struct RenamedAttrOpaque2;
    extern "C" {
    void namespace_AttrOpaque2_destroy(RenamedAttrOpaque2* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedAttrOpaque2;
using RenamedAttrOpaque2Ref = somelib::diplomat::Ref<RenamedAttrOpaque2, const somelib::ns::capi::RenamedAttrOpaque2>;
using RenamedAttrOpaque2RefMut = somelib::diplomat::Ref<RenamedAttrOpaque2, somelib::ns::capi::RenamedAttrOpaque2>;

class RenamedAttrOpaque2 : public somelib::diplomat::OpaquePointer<RenamedAttrOpaque2, somelib::ns::capi::RenamedAttrOpaque2, somelib::ns::capi::namespace_AttrOpaque2_destroy> {
public:

};

} // namespace
#endif // SOMELIB_ns_RenamedAttrOpaque2_D_HPP
