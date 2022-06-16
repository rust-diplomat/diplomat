#ifndef StrRef_H
#define StrRef_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct StrRef StrRef;

void StrRef_destroy(StrRef* self);

#ifdef __cplusplus
}
#endif
#endif
