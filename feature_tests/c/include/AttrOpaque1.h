#ifndef AttrOpaque1_H
#define AttrOpaque1_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AttrEnum.d.h"
#include "Unnamespaced.d.h"

#include "AttrOpaque1.d.h"






AttrOpaque1* namespace_AttrOpaque1_new();

uint8_t namespace_AttrOpaque1_method(const AttrOpaque1* self);

uint8_t renamed_on_abi_only(const AttrOpaque1* self);

void namespace_AttrOpaque1_use_unnamespaced(const AttrOpaque1* self, const Unnamespaced* _un);

void namespace_AttrOpaque1_use_namespaced(const AttrOpaque1* self, AttrEnum _n);


void namespace_AttrOpaque1_destroy(AttrOpaque1* self);





#endif // AttrOpaque1_H
