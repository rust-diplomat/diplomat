#ifndef OptionStruct_H
#define OptionStruct_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct OptionOpaque OptionOpaque;
typedef struct OptionOpaqueChar OptionOpaqueChar;

typedef struct OptionStruct {
    OptionOpaque* a;
    OptionOpaqueChar* b;
    uint32_t c;
    OptionOpaque* d;
} OptionStruct;

void OptionStruct_destroy(OptionStruct* self);

#ifdef __cplusplus
}
#endif
#endif
