#ifndef CallbackHolder_D_HPP
#define CallbackHolder_D_HPP

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
    struct CallbackHolder;
} // namespace capi
} // namespace

class CallbackHolder {
public:

  inline static std::unique_ptr<CallbackHolder> new_(std::function<int32_t(int32_t)> func);

  inline int32_t call(int32_t a) const;

  inline const diplomat::capi::CallbackHolder* AsFFI() const;
  inline diplomat::capi::CallbackHolder* AsFFI();
  inline static const CallbackHolder* FromFFI(const diplomat::capi::CallbackHolder* ptr);
  inline static CallbackHolder* FromFFI(diplomat::capi::CallbackHolder* ptr);
  inline static void operator delete(void* ptr);
private:
  CallbackHolder() = delete;
  CallbackHolder(const CallbackHolder&) = delete;
  CallbackHolder(CallbackHolder&&) noexcept = delete;
  CallbackHolder operator=(const CallbackHolder&) = delete;
  CallbackHolder operator=(CallbackHolder&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CallbackHolder_D_HPP
