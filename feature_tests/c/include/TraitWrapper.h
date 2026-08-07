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

void TraitWrapper_test_optional_output(DiplomatTraitStruct_TesterTrait t_trait_wrap, uint32_t x);

typedef struct TraitWrapper_test_result_of_optional_result {union {uint32_t ok; OptionU32 err;}; bool is_ok;} TraitWrapper_test_result_of_optional_result;
TraitWrapper_test_result_of_optional_result TraitWrapper_test_result_of_optional(DiplomatTraitStruct_TesterTrait t_trait_wrap, bool is_ok);





#endif // TraitWrapper_H
