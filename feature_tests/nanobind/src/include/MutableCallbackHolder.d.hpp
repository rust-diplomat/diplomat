#ifndef MutableCallbackHolder_D_HPP
#define MutableCallbackHolder_D_HPP

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
    struct MutableCallbackHolder;
} // namespace capi
} // namespace

class MutableCallbackHolder {
public:

  inline static std::unique_ptr<MutableCallbackHolder> new_(std::function<int32_t(int32_t)> func);

  inline int32_t call(int32_t a);

  inline const diplomat::capi::MutableCallbackHolder* AsFFI() const;
  inline diplomat::capi::MutableCallbackHolder* AsFFI();
  inline static const MutableCallbackHolder* FromFFI(const diplomat::capi::MutableCallbackHolder* ptr);
  inline static MutableCallbackHolder* FromFFI(diplomat::capi::MutableCallbackHolder* ptr);
  inline static void operator delete(void* ptr);
private:
  MutableCallbackHolder() = delete;
  MutableCallbackHolder(const MutableCallbackHolder&) = delete;
  MutableCallbackHolder(MutableCallbackHolder&&) noexcept = delete;
  MutableCallbackHolder operator=(const MutableCallbackHolder&) = delete;
  MutableCallbackHolder operator=(MutableCallbackHolder&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // MutableCallbackHolder_D_HPP
