#ifndef MyEnum_H_enum
#define MyEnum_H_enum


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



typedef enum MyEnum {
	MyEnum_A = -2,
	MyEnum_B = -1,
	MyEnum_C = 0,
	MyEnum_D = 1,
	MyEnum_E = 2,
	MyEnum_F = 3,
} MyEnum;


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // MyEnum_H_enum
