#ifndef ICU4XFixedDecimalFormat_H
#define ICU4XFixedDecimalFormat_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimalFormat_type.h"
#include "ICU4XLocale_type.h"
#include "ICU4XDataProvider_type.h"
#include "ICU4XFixedDecimalFormatOptions_type.h"
#include "ICU4XFixedDecimalFormatResult_type.h"
#include "ICU4XFixedDecimal_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XFixedDecimalFormatOptions options);

void ICU4XFixedDecimalFormat_format_write(const ICU4XFixedDecimalFormat* self, const ICU4XFixedDecimal* value, DiplomatWriteable* write);
void ICU4XFixedDecimalFormat_destroy(ICU4XFixedDecimalFormat* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XFixedDecimalFormat_H
