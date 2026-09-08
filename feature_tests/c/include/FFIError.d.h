#ifndef FFIError_D_H
#define FFIError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum FFIError {
  FFIError_FFI = 0,
  FFIError_User = 1,
} FFIError;

typedef struct FFIError_option {union { FFIError ok; }; bool is_ok; } FFIError_option;



#endif // FFIError_D_H
