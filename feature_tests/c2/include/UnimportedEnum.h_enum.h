#ifndef UnimportedEnum_H_enum
#define UnimportedEnum_H_enum


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



typedef enum UnimportedEnum {
	UnimportedEnum_A = 0,
	UnimportedEnum_B = 1,
	UnimportedEnum_C = 2,
} UnimportedEnum;


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // UnimportedEnum_H_enum
