#ifndef Foo_H
#define Foo_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Bar.d.h"
#include "BorrowedFields.d.h"
#include "BorrowedFieldsReturning.d.h"
#include "BorrowedFieldsWithBounds.d.h"

#include "Foo.d.h"






Foo* Foo_new(DiplomatStringView x);

Bar* Foo_get_bar(const Foo* self);

Foo* Foo_new_static(DiplomatStringView x);

BorrowedFieldsReturning Foo_as_returning(const Foo* self);

Foo* Foo_extract_from_fields(BorrowedFields fields);

Foo* Foo_extract_from_bounds(BorrowedFieldsWithBounds bounds, DiplomatStringView another_string);

void Foo_destroy(Foo* self);





#endif // Foo_H
