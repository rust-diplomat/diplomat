#ifndef OpaqueThinIter_HPP
#define OpaqueThinIter_HPP

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


namespace diplomat {
namespace capi {
    extern "C" {
    const diplomat::capi::OpaqueThin* OpaqueThinIter_next(diplomat::capi::OpaqueThinIter* self);

    void OpaqueThinIter_destroy(OpaqueThinIter* self);

    } // extern "C"

} // namespace capi
} // namespace

inline const OpaqueThin* OpaqueThinIter::next() {
  auto result = diplomat::capi::OpaqueThinIter_next(this->AsFFI());
  return OpaqueThin::FromFFI(result);
}

inline const diplomat::capi::OpaqueThinIter* OpaqueThinIter::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::OpaqueThinIter*>(this);
}

inline diplomat::capi::OpaqueThinIter* OpaqueThinIter::AsFFI() {
  return reinterpret_cast<diplomat::capi::OpaqueThinIter*>(this);
}

inline const OpaqueThinIter* OpaqueThinIter::FromFFI(const diplomat::capi::OpaqueThinIter* ptr) {
  return reinterpret_cast<const OpaqueThinIter*>(ptr);
}

inline OpaqueThinIter* OpaqueThinIter::FromFFI(diplomat::capi::OpaqueThinIter* ptr) {
  return reinterpret_cast<OpaqueThinIter*>(ptr);
}

inline void OpaqueThinIter::operator delete(void* ptr) {
  diplomat::capi::OpaqueThinIter_destroy(reinterpret_cast<diplomat::capi::OpaqueThinIter*>(ptr));
}


#endif // OpaqueThinIter_HPP
