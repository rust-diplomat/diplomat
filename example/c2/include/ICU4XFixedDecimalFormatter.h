#ifndef ICU4XFixedDecimalFormatter_H
#define ICU4XFixedDecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataProvider.d.h"
#include "ICU4XFixedDecimal.d.h"
#include "ICU4XFixedDecimalFormatterOptions.d.h"
#include "ICU4XLocale.d.h"

#include "ICU4XFixedDecimalFormatter.d.h"






typedef struct icu4x_ICU4XFixedDecimalFormatter_try_new_mv1_result {union {ICU4XFixedDecimalFormatter* ok; }; bool is_ok;} icu4x_ICU4XFixedDecimalFormatter_try_new_mv1_result;
icu4x_ICU4XFixedDecimalFormatter_try_new_mv1_result icu4x_ICU4XFixedDecimalFormatter_try_new_mv1(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XFixedDecimalFormatterOptions options);

void icu4x_ICU4XFixedDecimalFormatter_format_write_mv1(const ICU4XFixedDecimalFormatter* self, const ICU4XFixedDecimal* value, DiplomatWrite* write);


void icu4x_ICU4XFixedDecimalFormatter_mv1_destroy(ICU4XFixedDecimalFormatter* self);





#endif // ICU4XFixedDecimalFormatter_H
