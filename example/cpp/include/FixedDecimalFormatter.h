#ifndef FixedDecimalFormatter_H
#define FixedDecimalFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct FixedDecimalFormatter FixedDecimalFormatter;
#ifdef __cplusplus
} // namespace capi
#endif
#include "Locale.h"
#include "DataProvider.h"
#include "FixedDecimalFormatterOptions.h"
#include "diplomat_result_box_FixedDecimalFormatter_void.h"
#include "FixedDecimal.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_FixedDecimalFormatter_void icu4x_FixedDecimalFormatter_try_new_mv1(const Locale* locale, const DataProvider* provider, FixedDecimalFormatterOptions options);

void icu4x_FixedDecimalFormatter_format_write_mv1(const FixedDecimalFormatter* self, const FixedDecimal* value, DiplomatWrite* write);
void icu4x_FixedDecimalFormatter_destroy_mv1(FixedDecimalFormatter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
