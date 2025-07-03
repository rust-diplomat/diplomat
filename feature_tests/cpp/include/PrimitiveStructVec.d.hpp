#ifndef PrimitiveStructVec_D_HPP
#define PrimitiveStructVec_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"

struct PrimitiveStruct;


namespace diplomat {
namespace capi {
    struct PrimitiveStructVec;
} // namespace capi
} // namespace

class PrimitiveStructVec {
public:

  inline static std::unique_ptr<PrimitiveStructVec> new_();

  inline void push(PrimitiveStruct value);

  inline size_t len() const;

  inline diplomat::span<const PrimitiveStruct> as_slice() const;

  inline diplomat::span<PrimitiveStruct> as_slice_mut();

  inline PrimitiveStruct get(size_t idx) const;

  inline const diplomat::capi::PrimitiveStructVec* AsFFI() const;
  inline diplomat::capi::PrimitiveStructVec* AsFFI();
  inline static const PrimitiveStructVec* FromFFI(const diplomat::capi::PrimitiveStructVec* ptr);
  inline static PrimitiveStructVec* FromFFI(diplomat::capi::PrimitiveStructVec* ptr);
  inline static void operator delete(void* ptr);
private:
  PrimitiveStructVec() = delete;
  PrimitiveStructVec(const PrimitiveStructVec&) = delete;
  PrimitiveStructVec(PrimitiveStructVec&&) noexcept = delete;
  PrimitiveStructVec operator=(const PrimitiveStructVec&) = delete;
  PrimitiveStructVec operator=(PrimitiveStructVec&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // PrimitiveStructVec_D_HPP
