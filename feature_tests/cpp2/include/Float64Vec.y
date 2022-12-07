#ifndef Float64Vec_H
#define Float64Vec_HPP


#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"




class Float64Vec;



std::unique_ptr<Float64Vec> Float64Vec_new(const std::span<double> v);
void Float64Vec_fill_slice(const Float64Vec& self, std::span<double> v);
void Float64Vec_set_value(Float64Vec& self, const std::span<double> new_slice);
void Float64Vec_destroy(Float64Vec* self);


#endif // Float64Vec_HPP
