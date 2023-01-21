#ifndef ICU4XFixedDecimalFormatter_D_HPP
#define ICU4XFixedDecimalFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalFormatter.d.h"
#include "ICU4XFixedDecimalFormatterOptions.d.hpp"

class ICU4XDataProvider;
class ICU4XFixedDecimal;
class ICU4XLocale;
struct ICU4XFixedDecimalFormatterOptions;


class ICU4XFixedDecimalFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, std::monostate> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatterOptions options);

  inline std::string format_write(const ICU4XFixedDecimal& value) const;

  inline const capi::ICU4XFixedDecimalFormatter* AsFFI() const;
  inline capi::ICU4XFixedDecimalFormatter* AsFFI();
  inline static const ICU4XFixedDecimalFormatter* FromFFI(const capi::ICU4XFixedDecimalFormatter* ptr);
  inline static ICU4XFixedDecimalFormatter* FromFFI(capi::ICU4XFixedDecimalFormatter* ptr);
  inline ~ICU4XFixedDecimalFormatter();
private:
  ICU4XFixedDecimalFormatter() = delete;
};


#endif // ICU4XFixedDecimalFormatter_D_HPP
