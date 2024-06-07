#ifndef Foo_H
#define Foo_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Bar.d.h"
#include "Bar.h"
#include "BorrowedFields.d.h"
#include "BorrowedFields.h"
#include "BorrowedFieldsReturning.d.h"
#include "BorrowedFieldsReturning.h"
#include "BorrowedFieldsWithBounds.d.h"
#include "BorrowedFieldsWithBounds.h"

#include "Foo.d.h"






Foo* Foo_new(const char* x_data, size_t x_len);

Bar* Foo_get_bar(const Foo* self);

Foo* Foo_new_static(const char* x_data, size_t x_len);

BorrowedFieldsReturning Foo_as_returning(const Foo* self);

Foo* Foo_extract_from_fields(BorrowedFields fields);

Foo* Foo_extract_from_bounds(BorrowedFieldsWithBounds bounds, const char* another_string_data, size_t another_string_len);


void Foo_destroy(Foo* self);





#endif // Foo_H
