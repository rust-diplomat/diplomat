#ifndef OpaqueThin_D_HPP
#define OpaqueThin_D_HPP

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
    struct OpaqueThin;

} // namespace capi
} // namespace

class OpaqueThin {
public:

  inline int32_t a() const;

  inline float b() const;

  inline const diplomat::capi::OpaqueThin* AsFFI() const;
  inline diplomat::capi::OpaqueThin* AsFFI();
  inline static const OpaqueThin* FromFFI(const diplomat::capi::OpaqueThin* ptr);
  inline static OpaqueThin* FromFFI(diplomat::capi::OpaqueThin* ptr);
  inline static void operator delete(void* ptr);
private:
  OpaqueThin() = delete;
  OpaqueThin(const OpaqueThin&) = delete;
  OpaqueThin(OpaqueThin&&) noexcept = delete;
  OpaqueThin operator=(const OpaqueThin&) = delete;
  OpaqueThin operator=(OpaqueThin&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // OpaqueThin_D_HPP
