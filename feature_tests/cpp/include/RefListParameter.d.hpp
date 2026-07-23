#ifndef SOMELIB_RefListParameter_D_HPP
#define SOMELIB_RefListParameter_D_HPP

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
namespace capi {
    struct RefListParameter;
    extern "C" {
    void RefListParameter_destroy(RefListParameter* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class RefListParameter;
using RefListParameterRef = somelib::diplomat::Ref<RefListParameter, const somelib::capi::RefListParameter>;
using RefListParameterRefMut = somelib::diplomat::Ref<RefListParameter, somelib::capi::RefListParameter>;

class RefListParameter : public somelib::diplomat::OpaquePointer<RefListParameter, somelib::capi::RefListParameter, somelib::capi::RefListParameter_destroy> {
public:

};

} // namespace
#endif // SOMELIB_RefListParameter_D_HPP
