#ifndef SOMELIB_ns_RenamedTestOpaque_D_HPP
#define SOMELIB_ns_RenamedTestOpaque_D_HPP

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
    struct RenamedTestOpaque;
    extern "C" {
    void namespace_TestOpaque_destroy(RenamedTestOpaque* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedTestOpaque;
using RenamedTestOpaqueRef = somelib::diplomat::Ref<RenamedTestOpaque, const somelib::ns::capi::RenamedTestOpaque>;
using RenamedTestOpaqueRefMut = somelib::diplomat::Ref<RenamedTestOpaque, somelib::ns::capi::RenamedTestOpaque>;

class RenamedTestOpaque : public somelib::diplomat::OpaquePointer<RenamedTestOpaque, somelib::ns::capi::RenamedTestOpaque, somelib::ns::capi::namespace_TestOpaque_destroy> {
public:

};

} // namespace
#endif // SOMELIB_ns_RenamedTestOpaque_D_HPP
