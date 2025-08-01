#ifndef TraitWrapper_H
#define TraitWrapper_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "TesterTrait.d.h"

#include "TraitWrapper.d.h"






int32_t TraitWrapper_test_with_trait(DiplomatTraitStruct_TesterTrait t_trait_wrap, int32_t x);

int32_t TraitWrapper_test_trait_with_struct(DiplomatTraitStruct_TesterTrait t_trait_wrap);

void TraitWrapper_test_result_output(DiplomatTraitStruct_TesterTrait t_trait_wrap);





#endif // TraitWrapper_H
