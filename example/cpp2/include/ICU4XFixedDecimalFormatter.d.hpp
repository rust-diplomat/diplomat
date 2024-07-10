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

namespace icu4x {
namespace capi {typedef struct ICU4XDataProvider ICU4XDataProvider; }
class ICU4XDataProvider;
namespace capi {typedef struct ICU4XFixedDecimal ICU4XFixedDecimal; }
class ICU4XFixedDecimal;
namespace capi {typedef struct ICU4XFixedDecimalFormatter ICU4XFixedDecimalFormatter; }
class ICU4XFixedDecimalFormatter;
namespace capi {typedef struct ICU4XLocale ICU4XLocale; }
class ICU4XLocale;
struct ICU4XFixedDecimalFormatterOptions;
}


namespace icu4x {
namespace capi {
    typedef struct ICU4XFixedDecimalFormatter ICU4XFixedDecimalFormatter;
}

class ICU4XFixedDecimalFormatter {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::ICU4XFixedDecimalFormatter>, std::monostate> try_new(const icu4x::ICU4XLocale& locale, const icu4x::ICU4XDataProvider& provider, icu4x::ICU4XFixedDecimalFormatterOptions options);

  inline std::string format_write(const icu4x::ICU4XFixedDecimal& value) const;

  inline const icu4x::capi::ICU4XFixedDecimalFormatter* AsFFI() const;
  inline icu4x::capi::ICU4XFixedDecimalFormatter* AsFFI();
  inline static const icu4x::ICU4XFixedDecimalFormatter* FromFFI(const icu4x::capi::ICU4XFixedDecimalFormatter* ptr);
  inline static icu4x::ICU4XFixedDecimalFormatter* FromFFI(icu4x::capi::ICU4XFixedDecimalFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XFixedDecimalFormatter() = delete;
  ICU4XFixedDecimalFormatter(const icu4x::ICU4XFixedDecimalFormatter&) = delete;
  ICU4XFixedDecimalFormatter(icu4x::ICU4XFixedDecimalFormatter&&) noexcept = delete;
  ICU4XFixedDecimalFormatter operator=(const icu4x::ICU4XFixedDecimalFormatter&) = delete;
  ICU4XFixedDecimalFormatter operator=(icu4x::ICU4XFixedDecimalFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

}
#endif // ICU4XFixedDecimalFormatter_D_HPP
