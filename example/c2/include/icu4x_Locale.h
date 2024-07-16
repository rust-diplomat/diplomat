#ifndef icu4x_Locale_H
#define icu4x_Locale_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "icu4x_Locale.d.h"






icu4x_Locale* icu4x_Locale_new(const char* name_data, size_t name_len) {
    icu4x_Locale* icu4x_Locale_new_mv1(const char* name_data, size_t name_len);
    return icu4x_Locale_new_mv1(name_data, name_len);
}


void icu4x_Locale_destroy(icu4x_Locale* self) {
    void icu4x_Locale_destroy_mv1(icu4x_Locale* self);
    icu4x_Locale_destroy_mv1(self);
}





#endif // icu4x_Locale_H
