#ifndef Opaque_H
#define Opaque_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Opaque Opaque;
#include "MyStruct.h"

Opaque* Opaque_new();

void Opaque_assert_struct(const Opaque* self, MyStruct s);
void Opaque_destroy(Opaque* self);

#ifdef __cplusplus
}
#endif
#endif
