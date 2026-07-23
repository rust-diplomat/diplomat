#ifndef SOMELIB_OpaqueThinIter_D_HPP
#define SOMELIB_OpaqueThinIter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "OpaqueThin.d.hpp"
#include "diplomat_runtime.hpp"
namespace somelib {
namespace capi { struct OpaqueThin; }
class OpaqueThin;
} // namespace somelib



namespace somelib {
namespace capi {
    struct OpaqueThinIter;
    extern "C" {
    void OpaqueThinIter_destroy(OpaqueThinIter* self);
    }
} // namespace capi
} // namespace

namespace somelib {
class OpaqueThinIter;
using OpaqueThinIterRef = somelib::diplomat::Ref<OpaqueThinIter, const somelib::capi::OpaqueThinIter>;
using OpaqueThinIterRefMut = somelib::diplomat::Ref<OpaqueThinIter, somelib::capi::OpaqueThinIter>;

class OpaqueThinIter : public somelib::diplomat::OpaquePointer<OpaqueThinIter, somelib::capi::OpaqueThinIter, somelib::capi::OpaqueThinIter_destroy> {
public:

  inline somelib::diplomat::Optional<somelib::OpaqueThinRef> next() DIPLOMAT_LIFETIME_BOUND;

};

} // namespace
#endif // SOMELIB_OpaqueThinIter_D_HPP
