#ifndef ICU4XDataProvider_H
#define ICU4XDataProvider_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ICU4XDataProvider.d.h"






ICU4XDataProvider* ICU4XDataProvider_new_static();

typedef struct ICU4XDataProvider_returns_result_result { bool is_ok;} ICU4XDataProvider_returns_result_result;
ICU4XDataProvider_returns_result_result ICU4XDataProvider_returns_result();


void ICU4XDataProvider_destroy(ICU4XDataProvider* self);





#endif // ICU4XDataProvider_H
