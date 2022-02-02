#ifndef OwnershipEater_H
#define OwnershipEater_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct OwnershipEater OwnershipEater;

OwnershipEater* OwnershipEater_new();
void OwnershipEater_destroy(OwnershipEater* self);

#ifdef __cplusplus
}
#endif
#endif
