#ifndef ICU4XFixedDecimalFormatter_H
#define ICU4XFixedDecimalFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XFixedDecimalFormatter ICU4XFixedDecimalFormatter;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XLocale.h"
#include "ICU4XDataProvider.h"
#include "ICU4XFixedDecimalFormatterOptions.h"
#include "diplomat_result_box_ICU4XFixedDecimalFormatter_void.h"
#include "ICU4XFixedDecimal.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XFixedDecimalFormatter_void icu4x_ICU4XFixedDecimalFormatter_try_new_mv1(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XFixedDecimalFormatterOptions options);

void icu4x_ICU4XFixedDecimalFormatter_format_write_mv1(const ICU4XFixedDecimalFormatter* self, const ICU4XFixedDecimal* value, DiplomatWrite* write);
void icu4x_ICU4XFixedDecimalFormatter_destroy_mv1(ICU4XFixedDecimalFormatter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
