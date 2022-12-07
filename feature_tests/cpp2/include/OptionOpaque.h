#ifndef OptionOpaque_H
#define OptionOpaque_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionStruct.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct OptionStruct OptionStruct;


typedef struct OptionOpaque OptionOpaque;



OptionOpaque* OptionOpaque_new(int32_t i);
OptionOpaque* OptionOpaque_new_none();
OptionStruct OptionOpaque_new_struct();
OptionStruct OptionOpaque_new_struct_nones();
void OptionOpaque_assert_integer(const OptionOpaque* self, int32_t i);
bool OptionOpaque_option_opaque_argument(const OptionOpaque* arg);
void OptionOpaque_destroy(OptionOpaque* self);


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // OptionOpaque_H
