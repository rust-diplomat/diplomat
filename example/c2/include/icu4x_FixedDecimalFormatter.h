#ifndef icu4x_FixedDecimalFormatter_H
#define icu4x_FixedDecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "icu4x_DataProvider.d.h"
#include "icu4x_FixedDecimal.d.h"
#include "icu4x_FixedDecimalFormatterOptions.d.h"
#include "icu4x_Locale.d.h"

#include "icu4x_FixedDecimalFormatter.d.h"






typedef struct icu4x_FixedDecimalFormatter_try_new_result {union {icu4x_FixedDecimalFormatter* ok; }; bool is_ok;} icu4x_FixedDecimalFormatter_try_new_result;
icu4x_FixedDecimalFormatter_try_new_result icu4x_FixedDecimalFormatter_try_new(const icu4x_Locale* locale, const icu4x_DataProvider* provider, icu4x_FixedDecimalFormatterOptions options) {
    icu4x_FixedDecimalFormatter_try_new_result icu4x_FixedDecimalFormatter_try_new_mv1(const icu4x_Locale* locale, const icu4x_DataProvider* provider, icu4x_FixedDecimalFormatterOptions options);
    return icu4x_FixedDecimalFormatter_try_new_mv1(locale, provider, options);
}

void icu4x_FixedDecimalFormatter_format_write(const icu4x_FixedDecimalFormatter* self, const icu4x_FixedDecimal* value, DiplomatWrite* write) {
    void icu4x_FixedDecimalFormatter_format_write_mv1(const icu4x_FixedDecimalFormatter* self, const icu4x_FixedDecimal* value, DiplomatWrite* write);
    return icu4x_FixedDecimalFormatter_format_write_mv1(self, value, write);
}


void icu4x_FixedDecimalFormatter_destroy(icu4x_FixedDecimalFormatter* self) {
    void icu4x_FixedDecimalFormatter_destroy_mv1(icu4x_FixedDecimalFormatter* self);
    icu4x_FixedDecimalFormatter_destroy_mv1(self);
}





#endif // icu4x_FixedDecimalFormatter_H
