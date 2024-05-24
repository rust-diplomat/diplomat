#ifndef diplomat_result_box_MyString_Utf8Error_D_H
#define diplomat_result_box_MyString_Utf8Error_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "MyString.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct diplomat_result_box_MyString_Utf8Error {
  union {
    MyString* ok;
    Utf8Error err;
  };
  bool is_ok;
} diplomat_result_box_MyString_Utf8Error;

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // diplomat_result_box_MyString_Utf8Error_D_H
