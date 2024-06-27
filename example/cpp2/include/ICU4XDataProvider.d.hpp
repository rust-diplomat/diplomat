#ifndef ICU4XDataProvider_D_HPP
#define ICU4XDataProvider_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct ICU4XDataProvider ICU4XDataProvider;
}

class ICU4XDataProvider {
public:

  inline static std::unique_ptr<ICU4XDataProvider> new_static();

  inline static diplomat::result<std::monostate, std::monostate> returns_result();

  inline const ::capi::ICU4XDataProvider* AsFFI() const;
  inline ::capi::ICU4XDataProvider* AsFFI();
  inline static const ICU4XDataProvider* FromFFI(const ::capi::ICU4XDataProvider* ptr);
  inline static ICU4XDataProvider* FromFFI(::capi::ICU4XDataProvider* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XDataProvider() = delete;
  ICU4XDataProvider(const ICU4XDataProvider&) = delete;
  ICU4XDataProvider(ICU4XDataProvider&&) noexcept = delete;
  ICU4XDataProvider operator=(const ICU4XDataProvider&) = delete;
  ICU4XDataProvider operator=(ICU4XDataProvider&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XDataProvider_D_HPP
