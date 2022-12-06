#ifndef ICU4XFixedDecimalFormat_H
#define ICU4XFixedDecimalFormat_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.h"
#include "ICU4XFixedDecimal.h"
#include "ICU4XFixedDecimalFormatOptions.h"
#include "ICU4XFixedDecimalFormatResult.h"
#include "ICU4XLocale.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct ICU4XDataProvider ICU4XDataProvider;
typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;
typedef struct ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions;
typedef struct ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormatResult;
typedef struct ICU4XLocale ICU4XLocale;


typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;



ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XFixedDecimalFormatOptions options);
void ICU4XFixedDecimalFormat_format_write(const ICU4XFixedDecimalFormat* self, const ICU4XFixedDecimal* value, DiplomatWriteable* writeable);
void ICU4XFixedDecimalFormat_destroy(ICU4XFixedDecimalFormat* self);


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // ICU4XFixedDecimalFormat_H
