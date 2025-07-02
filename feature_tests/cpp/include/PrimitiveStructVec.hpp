#ifndef PrimitiveStructVec_HPP
#define PrimitiveStructVec_HPP

#include "PrimitiveStructVec.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "PrimitiveStruct.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {

    diplomat::capi::PrimitiveStructVec* PrimitiveStructVec_new(void);

    void PrimitiveStructVec_push(diplomat::capi::PrimitiveStructVec* self, diplomat::capi::PrimitiveStruct value);

    size_t PrimitiveStructVec_len(const diplomat::capi::PrimitiveStructVec* self);

    diplomat::capi::DiplomatPrimitiveStructView PrimitiveStructVec_as_slice(const diplomat::capi::PrimitiveStructVec* self);

    void PrimitiveStructVec_destroy(PrimitiveStructVec* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<PrimitiveStructVec> PrimitiveStructVec::new_() {
  auto result = diplomat::capi::PrimitiveStructVec_new();
  return std::unique_ptr<PrimitiveStructVec>(PrimitiveStructVec::FromFFI(result));
}

inline void PrimitiveStructVec::push(PrimitiveStruct value) {
  diplomat::capi::PrimitiveStructVec_push(this->AsFFI(),
    value.AsFFI());
}

inline size_t PrimitiveStructVec::len() const {
  auto result = diplomat::capi::PrimitiveStructVec_len(this->AsFFI());
  return result;
}

inline diplomat::span<const PrimitiveStruct> PrimitiveStructVec::as_slice() const {
  auto result = diplomat::capi::PrimitiveStructVec_as_slice(this->AsFFI());
  return diplomat::span<const PrimitiveStruct>(reinterpret_cast<const PrimitiveStruct*>(result.data), result.len);
}

inline const diplomat::capi::PrimitiveStructVec* PrimitiveStructVec::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::PrimitiveStructVec*>(this);
}

inline diplomat::capi::PrimitiveStructVec* PrimitiveStructVec::AsFFI() {
  return reinterpret_cast<diplomat::capi::PrimitiveStructVec*>(this);
}

inline const PrimitiveStructVec* PrimitiveStructVec::FromFFI(const diplomat::capi::PrimitiveStructVec* ptr) {
  return reinterpret_cast<const PrimitiveStructVec*>(ptr);
}

inline PrimitiveStructVec* PrimitiveStructVec::FromFFI(diplomat::capi::PrimitiveStructVec* ptr) {
  return reinterpret_cast<PrimitiveStructVec*>(ptr);
}

inline void PrimitiveStructVec::operator delete(void* ptr) {
  diplomat::capi::PrimitiveStructVec_destroy(reinterpret_cast<diplomat::capi::PrimitiveStructVec*>(ptr));
}


#endif // PrimitiveStructVec_HPP
