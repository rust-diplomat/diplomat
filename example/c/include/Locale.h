#ifndef Locale_H
#define Locale_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Locale.d.h"






Locale* icu4x_Locale_new_mv1(DiplomatStringView name);


void icu4x_Locale_destroy_mv1(Locale* self);





#endif // Locale_H
