#ifndef ErrorEnum_H
#define ErrorEnum_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ErrorEnum_type.h"
#include "diplomat_result_ErrorEnum_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_ErrorEnum_void ErrorEnum_make_errorenum_for_string(const char* s_data, size_t s_len);
void ErrorEnum_destroy(ErrorEnum* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ErrorEnum_H
