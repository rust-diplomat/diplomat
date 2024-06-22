#ifndef ICU4XFixedDecimal_D_HPP
#define ICU4XFixedDecimal_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;
}

class ICU4XFixedDecimal {
public:

  inline static std::unique_ptr<ICU4XFixedDecimal> new_(int32_t v);

  inline void multiply_pow10(int16_t power);

  inline diplomat::result<std::string, std::monostate> to_string() const;

  inline const capi::ICU4XFixedDecimal* AsFFI() const;
  inline capi::ICU4XFixedDecimal* AsFFI();
  inline static const ICU4XFixedDecimal* FromFFI(const capi::ICU4XFixedDecimal* ptr);
  inline static ICU4XFixedDecimal* FromFFI(capi::ICU4XFixedDecimal* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XFixedDecimal() = delete;
  ICU4XFixedDecimal(const ICU4XFixedDecimal&) = delete;
  ICU4XFixedDecimal(ICU4XFixedDecimal&&) noexcept = delete;
  ICU4XFixedDecimal operator=(const ICU4XFixedDecimal&) = delete;
  ICU4XFixedDecimal operator=(ICU4XFixedDecimal&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XFixedDecimal_D_HPP
