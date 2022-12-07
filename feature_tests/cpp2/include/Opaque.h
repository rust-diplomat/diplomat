#ifndef Opaque_H
#define Opaque_H


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ImportedStruct.h"
#include "MyStruct.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus

typedef struct ImportedStruct ImportedStruct;
typedef struct MyStruct MyStruct;


typedef struct Opaque Opaque;



Opaque* Opaque_new();
void Opaque_assert_struct(const Opaque* self, MyStruct s);
size_t Opaque_returns_usize();
ImportedStruct Opaque_returns_imported();
void Opaque_destroy(Opaque* self);


#ifdef __cplusplus
} // namespace capi
} // extern "C"
#endif // __cplusplus

#endif // Opaque_H
