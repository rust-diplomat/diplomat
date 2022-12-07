#ifndef Foo_H
#define Foo_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "Bar.h"


class Bar;


typedef struct Foo Foo;



Foo* Foo_new(const char* x_data, size_t x_len);
Bar* Foo_get_bar(const Foo* self);
Foo* Foo_new_static(const char* x_data, size_t x_len);
void Foo_destroy(Foo* self);


#endif // Foo_HPP
