#ifndef OptionOpaque_H
#define OptionOpaque_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct OptionOpaque OptionOpaque;
#ifdef __cplusplus
} // namespace capi
#endif
#include "diplomat_result_OptionStruct_void.h"
#include "OptionStruct.h"
#include "diplomat_result_intptr_t_void.h"
#include "diplomat_result_size_t_void.h"
#include "diplomat_result_int32_t_void.h"
#include "diplomat_result_uint32_t_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

OptionOpaque* OptionOpaque_new(int32_t i);

OptionOpaque* OptionOpaque_new_none();

diplomat_result_OptionStruct_void OptionOpaque_returns();

diplomat_result_intptr_t_void OptionOpaque_option_isize(const OptionOpaque* self);

diplomat_result_size_t_void OptionOpaque_option_usize(const OptionOpaque* self);

diplomat_result_int32_t_void OptionOpaque_option_i32(const OptionOpaque* self);

diplomat_result_uint32_t_void OptionOpaque_option_u32(const OptionOpaque* self);

OptionStruct OptionOpaque_new_struct();

OptionStruct OptionOpaque_new_struct_nones();

void OptionOpaque_assert_integer(const OptionOpaque* self, int32_t i);

bool OptionOpaque_option_opaque_argument(const OptionOpaque* arg);
void OptionOpaque_destroy(OptionOpaque* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
