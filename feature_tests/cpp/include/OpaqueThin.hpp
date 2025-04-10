#ifndef OpaqueThin_HPP
#define OpaqueThin_HPP

#include "OpaqueThin.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    int32_t OpaqueThin_a(const diplomat::capi::OpaqueThin* self);
    float OpaqueThin_b(const diplomat::capi::OpaqueThin* self);

    void OpaqueThin_destroy(OpaqueThin* self);

    } // extern "C"

} // namespace capi
} // namespace

inline int32_t OpaqueThin::a() const {
  auto result = diplomat::capi::OpaqueThin_a(this->AsFFI());
  return result;
}

inline float OpaqueThin::b() const {
  auto result = diplomat::capi::OpaqueThin_b(this->AsFFI());
  return result;
}

inline const diplomat::capi::OpaqueThin* OpaqueThin::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::OpaqueThin*>(this);
}

inline diplomat::capi::OpaqueThin* OpaqueThin::AsFFI() {
  return reinterpret_cast<diplomat::capi::OpaqueThin*>(this);
}

inline const OpaqueThin* OpaqueThin::FromFFI(const diplomat::capi::OpaqueThin* ptr) {
  return reinterpret_cast<const OpaqueThin*>(ptr);
}

inline OpaqueThin* OpaqueThin::FromFFI(diplomat::capi::OpaqueThin* ptr) {
  return reinterpret_cast<OpaqueThin*>(ptr);
}

inline void OpaqueThin::operator delete(void* ptr) {
  diplomat::capi::OpaqueThin_destroy(reinterpret_cast<diplomat::capi::OpaqueThin*>(ptr));
}


#endif // OpaqueThin_HPP
