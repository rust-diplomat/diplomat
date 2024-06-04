#ifndef ICU4XLocale_H
#define ICU4XLocale_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "ICU4XLocale.d.h"






ICU4XLocale* ICU4XLocale_new(const char* name_data, size_t name_len);


void ICU4XLocale_destroy(ICU4XLocale* self);





#endif // ICU4XLocale_H
