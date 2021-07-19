#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XDataProvider ICU4XDataProvider;

typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;

typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;

typedef struct ICU4XFixedDecimalFormatOptions {
    uint8_t grouping_strategy;
    uint8_t sign_display;
} ICU4XFixedDecimalFormatOptions;

typedef struct ICU4XFixedDecimalFormatResult {
    ICU4XFixedDecimalFormat* fdf;
    bool success;
} ICU4XFixedDecimalFormatResult;

typedef struct ICU4XLocale ICU4XLocale;

ICU4XDataProvider* ICU4XDataProvider_new_static();

ICU4XFixedDecimal* ICU4XFixedDecimal_new(int32_t v);

void ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal* self, int16_t power);

void ICU4XFixedDecimal_negate(ICU4XFixedDecimal* self);

void ICU4XFixedDecimal_to_string(ICU4XFixedDecimal* self, DiplomatWriteable* to);

ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat_try_new(ICU4XLocale* locale, ICU4XDataProvider* provider, ICU4XFixedDecimalFormatOptions options);

void ICU4XFixedDecimalFormat_format_write(ICU4XFixedDecimalFormat* self, ICU4XFixedDecimal* value, DiplomatWriteable* write);

ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions_default();

ICU4XLocale* ICU4XLocale_new(const char* name_data, size_t name_len);
#ifdef __cplusplus
}
#endif
