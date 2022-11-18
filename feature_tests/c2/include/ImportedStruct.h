#ifndef ImportedStruct_H
#define ImportedStruct_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "UnimportedEnum.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



typedef struct ImportedStruct {
	UnimportedEnum foo;
	uint8_t count;
} ImportedStruct;





#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // ImportedStruct_H
