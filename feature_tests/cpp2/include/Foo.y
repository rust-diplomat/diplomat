#ifndef Foo_H
#define Foo_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


class Bar;


class Foo;



std::unique_ptr<Foo> Foo_new(std::string_view x);
std::unique_ptr<Bar> Foo_get_bar(const Foo& self);
std::unique_ptr<Foo> Foo_new_static(std::string_view x);
void Foo_destroy(Foo* self);


#endif // Foo_HPP
