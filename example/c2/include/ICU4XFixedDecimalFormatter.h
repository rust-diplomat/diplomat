#ifndef ICU4XFixedDecimalFormatter_H
#define ICU4XFixedDecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XFixedDecimal.d.h"
#include "ICU4XFixedDecimal.h"
#include "ICU4XFixedDecimalFormatterOptions.d.h"
#include "ICU4XFixedDecimalFormatterOptions.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XFixedDecimalFormatter_void.d.h"

#include "ICU4XFixedDecimalFormatter.d.h"






diplomat_result_box_ICU4XFixedDecimalFormatter_void ICU4XFixedDecimalFormatter_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XFixedDecimalFormatterOptions options);

void ICU4XFixedDecimalFormatter_format_write(const ICU4XFixedDecimalFormatter* self, const ICU4XFixedDecimal* value, DiplomatWrite* write);


void ICU4XFixedDecimalFormatter_destroy(ICU4XFixedDecimalFormatter* self);





#endif // ICU4XFixedDecimalFormatter_H
