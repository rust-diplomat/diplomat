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






typedef struct FixedDecimalFormatter_try_new_result {union {FixedDecimalFormatter* ok; }; bool is_ok;} FixedDecimalFormatter_try_new_result;
FixedDecimalFormatter_try_new_result FixedDecimalFormatter_try_new(const Locale* locale, const DataProvider* provider, FixedDecimalFormatterOptions options) {
    FixedDecimalFormatter_try_new_result icu4x_FixedDecimalFormatter_try_new_mv1(const Locale* locale, const DataProvider* provider, FixedDecimalFormatterOptions options);
    return icu4x_FixedDecimalFormatter_try_new_mv1(locale, provider, options);
}

void FixedDecimalFormatter_format_write(const FixedDecimalFormatter* self, const FixedDecimal* value, DiplomatWrite* write) {
    void icu4x_FixedDecimalFormatter_format_write_mv1(const FixedDecimalFormatter* self, const FixedDecimal* value, DiplomatWrite* write);
    return icu4x_FixedDecimalFormatter_format_write_mv1(self, value, write);
}


void icu4x_FixedDecimalFormatter_destroy_mv1(FixedDecimalFormatter* self);





#endif // FixedDecimalFormatter_H
