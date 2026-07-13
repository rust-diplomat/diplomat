#ifndef SOMELIB_ns_RenamedDeprecatedOpaque_D_HPP
#define SOMELIB_ns_RenamedDeprecatedOpaque_D_HPP

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
    struct RenamedDeprecatedOpaque;
    extern "C" {
    void namespace_DeprecatedOpaque_destroy(RenamedDeprecatedOpaque* self);
    }
} // namespace capi
} // namespace

namespace somelib::ns {
class RenamedDeprecatedOpaque;
using RenamedDeprecatedOpaqueRef = somelib::diplomat::Ref<RenamedDeprecatedOpaque, const somelib::ns::capi::RenamedDeprecatedOpaque>;
using RenamedDeprecatedOpaqueRefMut = somelib::diplomat::Ref<RenamedDeprecatedOpaque, somelib::ns::capi::RenamedDeprecatedOpaque>;

/**
 * \deprecated use Foo
 */
class [[deprecated("use Foo")]] RenamedDeprecatedOpaque : public somelib::diplomat::OpaquePointer<RenamedDeprecatedOpaque, somelib::ns::capi::RenamedDeprecatedOpaque, somelib::ns::capi::namespace_DeprecatedOpaque_destroy> {
public:

};

} // namespace
#endif // SOMELIB_ns_RenamedDeprecatedOpaque_D_HPP
