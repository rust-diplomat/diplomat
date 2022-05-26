#ifndef RefList_H
#define RefList_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RefList RefList;

RefList* RefList_node(const int32_t* data);
void RefList_destroy(RefList* self);

#ifdef __cplusplus
}
#endif
#endif
