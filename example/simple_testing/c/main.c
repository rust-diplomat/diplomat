#include <stdio.h>
#include <stdarg.h>
#include <stdint.h>


// -------------------------------------------------------------------------- generated code

typedef struct TesterTraitFunctionPointers {
    int32_t (*run_test_trait_fn_callback)(int32_t);
    void (*run_test_void_trait_fn_callback)();
} TesterTraitFunctionPointers;

typedef struct TesterTrait_VTable {
    int32_t (*run_test_trait_fn_callback)(TesterTraitFunctionPointers*, int32_t);
    void (*run_test_void_trait_fn_callback)(TesterTraitFunctionPointers*);
    void (*destructor)(const void*);
} TesterTrait_VTable;

typedef struct DiplomatTraitStruct_TesterTrait {
    TesterTraitFunctionPointers* data;
    TesterTrait_VTable* vtable;
} DiplomatTraitStruct_TesterTrait;


int32_t run_create_DiplomatTraitStruct_TesterTrait_test_trait_fn(TesterTraitFunctionPointers* cb_ptrs, int32_t arg0) {
    return cb_ptrs->run_test_trait_fn_callback(arg0);
}

void run_create_DiplomatTraitStruct_TesterTrait_test_void_trait_fn(TesterTraitFunctionPointers* cb_ptrs) {
    cb_ptrs->run_test_void_trait_fn_callback();
}

static void general_destructor(const void* data) {
    // TODO
}


TesterTrait_VTable tester_trait_vtable = {
    run_create_DiplomatTraitStruct_TesterTrait_test_trait_fn,
    run_create_DiplomatTraitStruct_TesterTrait_test_void_trait_fn,
    general_destructor,
};


int32_t Wrapper_test_with_trait(DiplomatTraitStruct_TesterTrait tt, int32_t x);

// ------------------------------------------------------- callbacks that will be called by the trait fcts
int32_t callback(int32_t x) {
    return x + 1;
}

void void_callback() {
    printf("CALLING CALLING CALLING CALLING from void callback\n");
}

// -------------------------------------------------------------------------------------- main
int main() {
    TesterTraitFunctionPointers fct_ptrs = {
        callback,
        void_callback,
    };

    DiplomatTraitStruct_TesterTrait tester_trait_struct = {
        &fct_ptrs,
        &tester_trait_vtable,
    };
    int32_t res = Wrapper_test_with_trait(tester_trait_struct, 5);
    printf("omg is it working: %d\n", res); 
}