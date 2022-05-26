#ifndef Foo_H
#define Foo_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Foo Foo;
#include "Bar.h"

Foo* Foo_new(const char* x_data, size_t x_len);

Bar* Foo_get_bar(const Foo* self);
void Foo_destroy(Foo* self);

#ifdef __cplusplus
}
#endif
#endif
