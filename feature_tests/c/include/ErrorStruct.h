#ifndef ErrorStruct_H
#define ErrorStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ErrorStruct.d.h"






typedef struct ErrorStruct_returns_result_option_result {union {ErrorStruct_option ok; }; bool is_ok;} ErrorStruct_returns_result_option_result;
ErrorStruct_returns_result_option_result ErrorStruct_returns_result_option(bool is_some);





#endif // ErrorStruct_H
