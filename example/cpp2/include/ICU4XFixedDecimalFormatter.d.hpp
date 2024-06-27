#ifndef ICU4XFixedDecimalFormatter_D_HPP
#define ICU4XFixedDecimalFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalFormatterOptions.d.hpp"

namespace capi {typedef struct ICU4XDataProvider ICU4XDataProvider; }
class ICU4XDataProvider;
namespace capi {typedef struct ICU4XFixedDecimal ICU4XFixedDecimal; }
class ICU4XFixedDecimal;
namespace capi {typedef struct ICU4XLocale ICU4XLocale; }
class ICU4XLocale;
struct ICU4XFixedDecimalFormatterOptions;


namespace capi {
    typedef struct ICU4XFixedDecimalFormatter ICU4XFixedDecimalFormatter;
}

class ICU4XFixedDecimalFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, std::monostate> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatterOptions options);

  inline std::string format_write(const ICU4XFixedDecimal& value) const;

  inline const ::capi::ICU4XFixedDecimalFormatter* AsFFI() const;
  inline ::capi::ICU4XFixedDecimalFormatter* AsFFI();
  inline static const ICU4XFixedDecimalFormatter* FromFFI(const ::capi::ICU4XFixedDecimalFormatter* ptr);
  inline static ICU4XFixedDecimalFormatter* FromFFI(::capi::ICU4XFixedDecimalFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XFixedDecimalFormatter() = delete;
  ICU4XFixedDecimalFormatter(const ICU4XFixedDecimalFormatter&) = delete;
  ICU4XFixedDecimalFormatter(ICU4XFixedDecimalFormatter&&) noexcept = delete;
  ICU4XFixedDecimalFormatter operator=(const ICU4XFixedDecimalFormatter&) = delete;
  ICU4XFixedDecimalFormatter operator=(ICU4XFixedDecimalFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XFixedDecimalFormatter_D_HPP
