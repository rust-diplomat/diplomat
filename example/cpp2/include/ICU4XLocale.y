#ifndef ICU4XLocale_H
#define ICU4XLocale_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




typedef struct ICU4XLocale ICU4XLocale;



std::unique_ptr<ICU4XLocale> ICU4XLocale_new(std::string_view name);
std::unique_ptr<ICU4XLocale> ICU4XLocale_new_from_bytes(const std::span<uint8_t> bytes);
void ICU4XLocale_destroy(ICU4XLocale* self);


#endif // ICU4XLocale_HPP
