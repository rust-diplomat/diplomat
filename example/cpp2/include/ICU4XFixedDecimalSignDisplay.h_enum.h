#ifndef ICU4XFixedDecimalSignDisplay_H_enum
#define ICU4XFixedDecimalSignDisplay_H_enum


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



typedef enum ICU4XFixedDecimalSignDisplay {
	ICU4XFixedDecimalSignDisplay_Auto = 0,
	ICU4XFixedDecimalSignDisplay_Never = 1,
	ICU4XFixedDecimalSignDisplay_Always = 2,
	ICU4XFixedDecimalSignDisplay_ExceptZero = 3,
	ICU4XFixedDecimalSignDisplay_Negative = 4,
} ICU4XFixedDecimalSignDisplay;


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // ICU4XFixedDecimalSignDisplay_H_enum
