#ifndef ErrorEnum_D_H
#define ErrorEnum_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ErrorEnum {
  ErrorEnum_Foo = 0,
  ErrorEnum_Bar = 1,
} ErrorEnum;



#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ErrorEnum_D_H
