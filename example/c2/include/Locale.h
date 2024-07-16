#ifndef Locale_H
#define Locale_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Locale.d.h"






Locale* Locale_new(const char* name_data, size_t name_len) {
    Locale* icu4x_Locale_new_mv1(const char* name_data, size_t name_len);
    return icu4x_Locale_new_mv1(name_data, name_len);
}


void icu4x_Locale_destroy_mv1(Locale* self);





#endif // Locale_H
