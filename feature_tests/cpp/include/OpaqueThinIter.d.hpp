#ifndef OpaqueThinIter_D_HPP
#define OpaqueThinIter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct OpaqueThin; }
class OpaqueThin;


namespace diplomat {
namespace capi {
    struct OpaqueThinIter;

} // namespace capi
} // namespace

class OpaqueThinIter {
public:

  inline const OpaqueThin* next();

  inline const diplomat::capi::OpaqueThinIter* AsFFI() const;
  inline diplomat::capi::OpaqueThinIter* AsFFI();
  inline static const OpaqueThinIter* FromFFI(const diplomat::capi::OpaqueThinIter* ptr);
  inline static OpaqueThinIter* FromFFI(diplomat::capi::OpaqueThinIter* ptr);
  inline static void operator delete(void* ptr);
private:
  OpaqueThinIter() = delete;
  OpaqueThinIter(const OpaqueThinIter&) = delete;
  OpaqueThinIter(OpaqueThinIter&&) noexcept = delete;
  OpaqueThinIter operator=(const OpaqueThinIter&) = delete;
  OpaqueThinIter operator=(OpaqueThinIter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // OpaqueThinIter_D_HPP
