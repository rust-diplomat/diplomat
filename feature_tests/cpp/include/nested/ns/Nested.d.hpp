#ifndef SOMELIB_nested_ns_Nested_D_HPP
#define SOMELIB_nested_ns_Nested_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../../diplomat_runtime.hpp"


namespace somelib::nested::ns {
namespace capi {
    struct Nested;
    extern "C" {
    void namespace_Nested_destroy(Nested* self);
    }
} // namespace capi
} // namespace

namespace somelib::nested::ns {
class Nested;
using NestedRef = somelib::diplomat::Ref<Nested, const somelib::nested::ns::capi::Nested>;
using NestedRefMut = somelib::diplomat::Ref<Nested, somelib::nested::ns::capi::Nested>;

class Nested : public somelib::diplomat::OpaquePointer<Nested, somelib::nested::ns::capi::Nested, somelib::nested::ns::capi::namespace_Nested_destroy> {
public:

};

} // namespace
#endif // SOMELIB_nested_ns_Nested_D_HPP
