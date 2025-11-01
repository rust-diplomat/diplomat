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

inline const somelib::OpaqueThin* somelib::OpaqueThinIter::next() {
    auto result = somelib::capi::OpaqueThinIter_next(this->AsFFI());
    return somelib::OpaqueThin::FromFFI(result);
}

inline const somelib::capi::OpaqueThinIter* somelib::OpaqueThinIter::AsFFI() const {
    return reinterpret_cast<const somelib::capi::OpaqueThinIter*>(this);
}

inline somelib::capi::OpaqueThinIter* somelib::OpaqueThinIter::AsFFI() {
    return reinterpret_cast<somelib::capi::OpaqueThinIter*>(this);
}

inline const somelib::OpaqueThinIter* somelib::OpaqueThinIter::FromFFI(const somelib::capi::OpaqueThinIter* ptr) {
    return reinterpret_cast<const somelib::OpaqueThinIter*>(ptr);
}

inline somelib::OpaqueThinIter* somelib::OpaqueThinIter::FromFFI(somelib::capi::OpaqueThinIter* ptr) {
    return reinterpret_cast<somelib::OpaqueThinIter*>(ptr);
}

inline void somelib::OpaqueThinIter::operator delete(void* ptr) {
    somelib::capi::OpaqueThinIter_destroy(reinterpret_cast<somelib::capi::OpaqueThinIter*>(ptr));
}


#endif // SOMELIB_OpaqueThinIter_HPP
