#ifndef OptionOpaque_H
#define OptionOpaque_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "OptionStruct.hpp"


struct OptionStruct;


class OptionOpaque;



std::unique_ptr<OptionOpaque> OptionOpaque_new(int32_t i);
std::unique_ptr<OptionOpaque> OptionOpaque_new_none();
OptionStruct OptionOpaque_new_struct();
OptionStruct OptionOpaque_new_struct_nones();
void OptionOpaque_assert_integer(const OptionOpaque& self, int32_t i);
bool OptionOpaque_option_opaque_argument(const std::optional<OptionOpaque&> arg);
void OptionOpaque_destroy(OptionOpaque* self);


#endif // OptionOpaque_HPP
