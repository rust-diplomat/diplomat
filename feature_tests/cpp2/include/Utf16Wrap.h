#ifndef Utf16Wrap_H
#define Utf16Wrap_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Utf16Wrap.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus



DiplomatString16View Utf16Wrap_borrow_cont(const Utf16Wrap* self);

DiplomatString16View Utf16Wrap_owned(const Utf16Wrap* self);

void Utf16Wrap_destroy(Utf16Wrap* self);



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // Utf16Wrap_H
