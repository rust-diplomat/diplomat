#ifndef FixedDecimalFormatter_H
#define FixedDecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "FixedDecimal.d.h"
#include "FixedDecimalFormatterOptions.d.h"
#include "Locale.d.h"

#include "FixedDecimalFormatter.d.h"






typedef struct icu4x_FixedDecimalFormatter_try_new_mv1_result {union {FixedDecimalFormatter* ok; }; bool is_ok;} icu4x_FixedDecimalFormatter_try_new_mv1_result;
icu4x_FixedDecimalFormatter_try_new_mv1_result icu4x_FixedDecimalFormatter_try_new_mv1(const Locale* locale, const DataProvider* provider, FixedDecimalFormatterOptions options);

void icu4x_FixedDecimalFormatter_format_write_mv1(const FixedDecimalFormatter* self, const FixedDecimal* value, DiplomatWrite* write);


void icu4x_FixedDecimalFormatter_destroy_mv1(FixedDecimalFormatter* self);





#endif // FixedDecimalFormatter_H
