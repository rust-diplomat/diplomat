#ifndef OpaqueThinVec_HPP
#define OpaqueThinVec_HPP

#include "OpaqueThinVec.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "OpaqueThin.hpp"
#include "OpaqueThinIter.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    diplomat::capi::OpaqueThinVec* OpaqueThinVec_create(diplomat::capi::DiplomatI32View a, diplomat::capi::DiplomatF32View b);
    diplomat::capi::OpaqueThinIter* OpaqueThinVec_iter(const diplomat::capi::OpaqueThinVec* self);
    size_t OpaqueThinVec_len(const diplomat::capi::OpaqueThinVec* self);
    const diplomat::capi::OpaqueThin* OpaqueThinVec_get(const diplomat::capi::OpaqueThinVec* self, size_t idx);
    const diplomat::capi::OpaqueThin* OpaqueThinVec_first(const diplomat::capi::OpaqueThinVec* self);

    void OpaqueThinVec_destroy(OpaqueThinVec* self);

    } // extern "C"

} // namespace capi
} // namespace

inline std::unique_ptr<OpaqueThinVec> OpaqueThinVec::create(diplomat::span<const int32_t> a, diplomat::span<const float> b) {
  auto result = diplomat::capi::OpaqueThinVec_create({a.data(), a.size()},
    {b.data(), b.size()});
  return std::unique_ptr<OpaqueThinVec>(OpaqueThinVec::FromFFI(result));
}

inline std::unique_ptr<OpaqueThinIter> OpaqueThinVec::iter() const {
  auto result = diplomat::capi::OpaqueThinVec_iter(this->AsFFI());
  return std::unique_ptr<OpaqueThinIter>(OpaqueThinIter::FromFFI(result));
}

inline diplomat::next_to_iter_helper<OpaqueThinIter>OpaqueThinVec::begin() const {
  return iter();
}

inline size_t OpaqueThinVec::len() const {
  auto result = diplomat::capi::OpaqueThinVec_len(this->AsFFI());
  return result;
}

inline const OpaqueThin* OpaqueThinVec::operator[](size_t idx) const {
  auto result = diplomat::capi::OpaqueThinVec_get(this->AsFFI(),
    idx);
  return OpaqueThin::FromFFI(result);
}

inline const OpaqueThin* OpaqueThinVec::first() const {
  auto result = diplomat::capi::OpaqueThinVec_first(this->AsFFI());
  return OpaqueThin::FromFFI(result);
}

inline const diplomat::capi::OpaqueThinVec* OpaqueThinVec::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::OpaqueThinVec*>(this);
}

inline diplomat::capi::OpaqueThinVec* OpaqueThinVec::AsFFI() {
  return reinterpret_cast<diplomat::capi::OpaqueThinVec*>(this);
}

inline const OpaqueThinVec* OpaqueThinVec::FromFFI(const diplomat::capi::OpaqueThinVec* ptr) {
  return reinterpret_cast<const OpaqueThinVec*>(ptr);
}

inline OpaqueThinVec* OpaqueThinVec::FromFFI(diplomat::capi::OpaqueThinVec* ptr) {
  return reinterpret_cast<OpaqueThinVec*>(ptr);
}

inline void OpaqueThinVec::operator delete(void* ptr) {
  diplomat::capi::OpaqueThinVec_destroy(reinterpret_cast<diplomat::capi::OpaqueThinVec*>(ptr));
}


#endif // OpaqueThinVec_HPP
