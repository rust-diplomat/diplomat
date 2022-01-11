#ifndef ErrorEnum_H
#define ErrorEnum_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ErrorEnum {
  ErrorEnum_Foo = 0,
  ErrorEnum_Bar = 1,
} ErrorEnum;

void ErrorEnum_destroy(ErrorEnum* self);

#ifdef __cplusplus
}
#endif
#endif
