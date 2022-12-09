#ifndef BorrowedFieldsReturning_H
#define BorrowedFieldsReturning_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



typedef struct BorrowedFieldsReturning {
	const uint8_t* bytes_data;
	size_t bytes_len;
} BorrowedFieldsReturning;





#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // BorrowedFieldsReturning_H
