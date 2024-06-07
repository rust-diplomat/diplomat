#ifndef BorrowedFields_H
#define BorrowedFields_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Bar.d.h"
#include "Bar.h"

#include "BorrowedFields.d.h"






BorrowedFields BorrowedFields_from_bar_and_strings(const Bar* bar, const char16_t* dstr16_data, size_t dstr16_len, const char* utf8_str_data, size_t utf8_str_len);






#endif // BorrowedFields_H
