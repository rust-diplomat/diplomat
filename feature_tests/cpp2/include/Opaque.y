#ifndef Opaque_H
#define Opaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ImportedStruct.hpp"
#include "MyStruct.hpp"


struct ImportedStruct;
struct MyStruct;


class Opaque;



std::unique_ptr<Opaque> Opaque_new();
void Opaque_assert_struct(const Opaque& self, MyStruct s);
size_t Opaque_returns_usize();
ImportedStruct Opaque_returns_imported();
void Opaque_destroy(Opaque* self);


#endif // Opaque_HPP
