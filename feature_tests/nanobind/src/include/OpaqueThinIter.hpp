#ifndef SOMELIB_OpaqueThinIter_HPP
#define SOMELIB_OpaqueThinIter_HPP

#include "OpaqueThinIter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "OpaqueThin.hpp"
#include "diplomat_runtime.hpp"


namespace somelib {
namespace capi {
    extern "C" {

    const somelib::capi::OpaqueThin* OpaqueThinIter_next(somelib::capi::OpaqueThinIter* self);

    void OpaqueThinIter_destroy(OpaqueThinIter* self);

    } // extern "C"
} // namespace capi
} // namespace

inline somelib::diplomat::Optional<somelib::OpaqueThinRef> somelib::OpaqueThinIter::next() DIPLOMAT_LIFETIME_BOUND {
    auto result = somelib::capi::OpaqueThinIter_next(this->AsFFI());
    return somelib::diplomat::Optional<somelib::OpaqueThinRef>::FromFFI(result);
}


#endif // SOMELIB_OpaqueThinIter_HPP
