#ifndef ICU4XFixedDecimalFormat_D_HPP
#define ICU4XFixedDecimalFormat_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalFormat.d.h"
#include "ICU4XFixedDecimalFormatOptions.d.hpp"
#include "ICU4XFixedDecimalFormatResult.d.hpp"

class ICU4XDataProvider;
class ICU4XFixedDecimal;
class ICU4XLocale;
struct ICU4XFixedDecimalFormatOptions;
struct ICU4XFixedDecimalFormatResult;


class ICU4XFixedDecimalFormat {
public:

  inline static ICU4XFixedDecimalFormatResult try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options);

  inline std::string format_write(const ICU4XFixedDecimal& value) const;

  inline const capi::ICU4XFixedDecimalFormat* AsFFI() const;
  inline capi::ICU4XFixedDecimalFormat* AsFFI();
  inline static const ICU4XFixedDecimalFormat* FromFFI(const capi::ICU4XFixedDecimalFormat* ptr);
  inline static ICU4XFixedDecimalFormat* FromFFI(capi::ICU4XFixedDecimalFormat* ptr);
  inline ~ICU4XFixedDecimalFormat();
private:
  ICU4XFixedDecimalFormat() = delete;
};


#endif // ICU4XFixedDecimalFormat_D_HPP
