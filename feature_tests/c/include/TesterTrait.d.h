#ifndef TesterTrait_D_H
#define TesterTrait_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "TraitTestingStruct.d.h"





typedef struct test_result_output_result {union {uint32_t ok; }; bool is_ok;} test_result_output_result;
typedef struct TesterTrait_VTable {
    void (*destructor)(const void*);
    size_t SIZE; size_t ALIGNMENT;
    uint32_t (*run_test_trait_fn_callback)(void*, uint32_t);
    void (*run_test_void_trait_fn_callback)(void*);
    int32_t (*run_test_struct_trait_fn_callback)(void*, TraitTestingStruct);
    test_result_output_result (*run_test_result_output_callback)(void*);
} TesterTrait_VTable;

typedef struct DiplomatTraitStruct_TesterTrait {
    void (*destructor)(const void*);
    TesterTrait_VTable vtable;
} DiplomatTraitStruct_TesterTrait;

static void general_destructor(const void* data) {
    // TODO
}

const size_t TesterTrait_DATA_SIZE = 0;
const size_t TesterTrait_DATA_ALIGNMENT = 0;




#endif // TesterTrait_D_H
