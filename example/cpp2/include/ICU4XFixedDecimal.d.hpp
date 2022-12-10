#ifndef ICU4XFixedDecimal_D_HPP
#define ICU4XFixedDecimal_D_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimal.d.h"




class ICU4XFixedDecimal {
public:

  inline static std::unique_ptr<ICU4XFixedDecimal> new_(int32_t v);

  inline void multiply_pow10(int16_t power);

  inline void negate();

  inline DiplomatResult<std::string, std::monostate> to_string() const;

  inline const capi::ICU4XFixedDecimal* AsFFI() const;
  inline capi::ICU4XFixedDecimal* AsFFI();
  inline static const ICU4XFixedDecimal* FromFFI(const capi::ICU4XFixedDecimal* ptr);
  inline static ICU4XFixedDecimal* FromFFI(capi::ICU4XFixedDecimal* ptr);
  inline ~ICU4XFixedDecimal();
private:
  ICU4XFixedDecimal() = delete;
};





#endif // ICU4XFixedDecimal_D_HPP
