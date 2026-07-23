#ifndef SOMELIB_OpaqueMut_D_HPP
#define SOMELIB_OpaqueMut_D_HPP

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
namespace capi { struct OpaqueMut; }
class OpaqueMut;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OpaqueMut;
    extern "C" {
    void OpaqueMut_destroy(OpaqueMut* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class OpaqueMut;
using OpaqueMutRef = somelib::diplomat::Ref<OpaqueMut, const somelib::capi::OpaqueMut>;
using OpaqueMutRefMut = somelib::diplomat::Ref<OpaqueMut, somelib::capi::OpaqueMut>;

class OpaqueMut : public somelib::diplomat::OpaquePointer<OpaqueMut, somelib::capi::OpaqueMut, somelib::capi::OpaqueMut_destroy> {
public:

  inline static somelib::OpaqueMut new_();

};

} // namespace
#endif // SOMELIB_OpaqueMut_D_HPP
