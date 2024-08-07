#include <stdio.h>
#include <stdarg.h>
#include <stdint.h>


// -------------------------------------------------------------------------- generated code

typedef struct TesterTrait_VTable {
    int32_t (*run_test_trait_fn_callback)(void*, int32_t);
    void (*run_test_void_trait_fn_callback)(void*);
    void (*destructor)(const void*);
} TesterTrait_VTable;

typedef struct DiplomatTraitStruct_TesterTrait {
    const void* data;
    TesterTrait_VTable* vtable;
} DiplomatTraitStruct_TesterTrait;


static void general_destructor(const void* data) {
    // TODO
}

int32_t Wrapper_test_with_trait(DiplomatTraitStruct_TesterTrait tt, int32_t x);

// ------------------------------------------------------- callbacks that will be called by the trait fcts
int32_t callback(int32_t x) {
    return x + 1;
}

void void_callback() {
    printf("CALLING CALLING CALLING CALLING from void callback\n");
}

int32_t run_create_DiplomatTraitStruct_TesterTrait_test_trait_fn(void* unused, int32_t arg0) {
    return callback(arg0);
}

void run_create_DiplomatTraitStruct_TesterTrait_test_void_trait_fn(void* unused) {
    void_callback();
}

TesterTrait_VTable tester_trait_vtable = {
    run_create_DiplomatTraitStruct_TesterTrait_test_trait_fn,
    run_create_DiplomatTraitStruct_TesterTrait_test_void_trait_fn,
    general_destructor,
};

// -------------------------------------------------------------------------------------- main
int main() {
    DiplomatTraitStruct_TesterTrait tester_trait_struct = {
        .vtable = &tester_trait_vtable,
    };
    int32_t res = Wrapper_test_with_trait(tester_trait_struct, 5);
    printf("omg is it working: %d\n", res); 
}