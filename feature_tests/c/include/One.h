#ifndef One_H
#define One_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Two.d.h"

#include "One.d.h"






One* One_transitivity(const One* hold, const One* nohold);

One* One_cycle(const Two* hold, const One* nohold);

One* One_many_dependents(const One* a, const One* b, const Two* c, const Two* d, const Two* nohold);

One* One_return_outlives_param(const Two* hold, const One* nohold);

One* One_diamond_top(const One* top, const One* left, const One* right, const One* bottom);

One* One_diamond_left(const One* top, const One* left, const One* right, const One* bottom);

One* One_diamond_right(const One* top, const One* left, const One* right, const One* bottom);

One* One_diamond_bottom(const One* top, const One* left, const One* right, const One* bottom);

One* One_diamond_and_nested_types(const One* a, const One* b, const One* c, const One* d, const One* nohold);

One* One_implicit_bounds(const One* explicit_hold, const One* implicit_hold, const One* nohold);

One* One_implicit_bounds_deep(const One* explicit_, const One* implicit_1, const One* implicit_2, const One* nohold);

void One_destroy(One* self);





#endif // One_H
