#ifndef BorrowedFields_H
#define BorrowedFields_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Bar.d.h"

#include "BorrowedFields.d.h"






BorrowedFields BorrowedFields_from_bar_and_strings(const Bar* bar, DiplomatString16View dstr16, DiplomatStringView utf8_str);





#endif // BorrowedFields_H
