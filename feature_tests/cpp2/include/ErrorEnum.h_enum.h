#ifndef ErrorEnum_H_enum
#define ErrorEnum_H_enum


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



typedef enum ErrorEnum {
	ErrorEnum_Foo = 0,
	ErrorEnum_Bar = 1,
} ErrorEnum;


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // ErrorEnum_H_enum
